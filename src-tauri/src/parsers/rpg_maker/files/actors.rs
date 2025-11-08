// Actors.json parser for RPG Maker MV/MZ
// Extracts and injects text from actor data

use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit};
use crate::parsers::text::formatter_trait::EngineFormatter;
use crate::parsers::text::rpg_maker_formatter::RpgMakerFormatter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object,
    inject_text_units_for_object, inject_translations_into_file_with_objects, GameDataFile,
};

/// Actor data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: u32,
    pub name: String,
    pub nickname: String,
    pub profile: String,
    // Other fields omitted for brevity (battlerName, characterName, etc.)
}

/// Actors parser implementation
pub struct ActorsParser;

impl crate::parsers::engine::FileParser for ActorsParser {
    fn extract(
        &self,
        file_path: &Path,
        _version: crate::parsers::engine::GameEngine,
    ) -> Result<Vec<crate::parsers::engine::TextEntry>, String> {
        // For now, delegate to the old approach - this should be refactored
        // to use the new extract_text function directly in engine.rs
        Err("Use extract_text function directly".to_string())
    }

    fn inject(
        &self,
        _file_path: &Path,
        _translations: &[crate::parsers::engine::TranslationEntry],
        _version: crate::parsers::engine::GameEngine,
    ) -> Result<(), String> {
        // For now, delegate to the old approach - this should be refactored
        // to use the new inject_translations function directly in engine.rs
        Err("Use inject_translations function directly".to_string())
    }
}

/// Extracts translatable text from Actors.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Actors.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Actors.json
    let parse_actors = |content: &str| -> AppResult<Vec<Option<Actor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Actors.json: {}", e)))
    };

    // Extract function for each actor
    let extract_actor_units = |actor: &Actor, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip null actor at index 0
        if index == 0 || actor.id == 0 {
            return Vec::new();
        }

        let mut fields = Vec::new();

        // Add raw text for each field if not empty (formatting happens later in the pipeline)
        if !actor.name.trim().is_empty() {
            fields.push(("name", actor.name.as_str(), PromptType::Character));
        }

        if !actor.nickname.trim().is_empty() {
            fields.push(("nickname", actor.nickname.as_str(), PromptType::Character));
        }

        if !actor.profile.trim().is_empty() {
            fields.push(("profile", actor.profile.as_str(), PromptType::Character));
        }

        extract_text_units_for_object("actor", actor.id as i32, file_path, index, fields)
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Actors.json",
        parse_actors,
        extract_actor_units,
    )
}

/// Injects translated text back into Actors.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Actors.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Actors.json
    let parse_actors = |content: &str| -> AppResult<Vec<Option<Actor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Actors.json: {}", e)))
    };

    // Update function for each actor
    let update_actor = |actor: &mut Actor, text_unit_map: &HashMap<String, &TextUnit>| {
        // Prepare mutable references for injection
        let mut name_ref = &mut actor.name;
        let mut nickname_ref = &mut actor.nickname;
        let mut profile_ref = &mut actor.profile;

        // Update each field and restore formatting
        if let Some(text_unit) = text_unit_map.get(&format!("actor_{}_name", actor.id)) {
            if !text_unit.translated_text.is_empty() {
                *name_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("actor_{}_nickname", actor.id)) {
            if !text_unit.translated_text.is_empty() {
                *nickname_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("actor_{}_profile", actor.id)) {
            if !text_unit.translated_text.is_empty() {
                *profile_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Actors.json",
        text_units,
        parse_actors,
        update_actor,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::engine::TranslationStatus;
    use std::path::{Path, PathBuf};

    fn get_test_games_path() -> PathBuf {
        // Get the path to the engines_past directory from the project root
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(manifest_dir)
            .parent()
            .unwrap()
            .join("engines_past")
    }

    #[test]
    fn test_extract_mv_actors() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Actors.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV actors: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has 6 actors (index 0 is null, so 5 real actors)
        // Each actor has a name, empty nickname and profile - only names pass validation
        assert_eq!(
            game_data.text_units.len(),
            5,
            "Expected 5 entries, got {}",
            game_data.text_units.len()
        );

        // Check first actor (たえちゃん) - formatted text
        assert_eq!(game_data.text_units[0].source_text, "たえちゃん");
        assert_eq!(game_data.text_units[0].id, "actor_1_name");

        // Check second actor (お兄ちゃん) - formatted text
        assert_eq!(game_data.text_units[1].source_text, "お兄ちゃん");
        assert_eq!(game_data.text_units[1].id, "actor_2_name");

        // Check third actor (たえ) - formatted text
        assert_eq!(game_data.text_units[2].source_text, "たえ");
        assert_eq!(game_data.text_units[2].id, "actor_3_name");

        // Check fourth actor (たえちゃん　＆　お兄ちゃん) - formatted text with full-width spaces converted to placeholders
        assert_eq!(
            game_data.text_units[3].source_text,
            "たえちゃん[FWSPC_1]＆[FWSPC_1]お兄ちゃん"
        );
        assert_eq!(game_data.text_units[3].id, "actor_4_name");

        // Check fifth actor (すみれ) - formatted text
        assert_eq!(game_data.text_units[4].source_text, "すみれ");
        assert_eq!(game_data.text_units[4].id, "actor_5_name");
    }

    #[test]
    fn test_extract_mz_actors() {
        let project_path = get_test_games_path().join("MZgame");

        let result = extract_text(&project_path, "data/Actors.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ actors: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game has 8 actors, but only some have content
        // Count entries that pass validation
        let name_entries: Vec<_> = game_data
            .text_units
            .iter()
            .filter(|e| e.id.contains("_name"))
            .collect();
        let nickname_entries: Vec<_> = game_data
            .text_units
            .iter()
            .filter(|e| e.id.contains("_nickname"))
            .collect();
        let profile_entries: Vec<_> = game_data
            .text_units
            .iter()
            .filter(|e| e.id.contains("_profile"))
            .collect();

        // Names: actors 1-4 have names, actor 5-8 are empty
        assert_eq!(name_entries.len(), 4);
        // Nicknames: actors 1-4 have nicknames
        assert_eq!(nickname_entries.len(), 4);
        // Profiles: actors 1-4 have profiles
        assert_eq!(profile_entries.len(), 4);

        // Check first actor (主人公) - formatted text
        assert_eq!(name_entries[0].source_text, "主人公");
        assert_eq!(name_entries[0].id, "actor_1_name");

        assert_eq!(nickname_entries[0].source_text, "主");
        assert_eq!(nickname_entries[0].id, "actor_1_nickname");

        // Profile with newlines - formatted text with newlines converted to placeholders
        assert_eq!(profile_entries[0].source_text, "主夫。愛妻、愛娘に恵まれて幸せ家庭を築いている。[CTRL_NEWLINE]仕事は映像関係をしている。");
        assert_eq!(profile_entries[0].id, "actor_1_profile");
    }

    #[test]
    fn test_extract_actors_validation() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Actors.json");

        assert!(result.is_ok());
        let game_data = result.unwrap();

        // All entries should pass validation (non-empty and valid text)
        for unit in &game_data.text_units {
            assert!(
                !unit.source_text.trim().is_empty(),
                "Entry should not be empty: {:?}",
                unit
            );
            // Note: ContentValidator is now applied during extraction in the common functions
        }
    }

    #[test]
    fn test_inject_mv_actors() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory structure to avoid modifying test files
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Copy the original file to temp directory
        let original_path = get_test_games_path().join("MVgame/www/data/Actors.json");
        let temp_actors_file = data_dir.join("Actors.json");
        fs::copy(&original_path, &temp_actors_file).unwrap();

        // Create text units for injection
        let text_unit_1 = TextUnit {
            id: "actor_1_name".to_string(),
            source_text: "たえちゃん".to_string(),
            translated_text: "Tae-chan".to_string(),
            field_type: "name:www/data/Actors.json:1".to_string(),
            status: TranslationStatus::Translated,
            prompt_type: PromptType::Character,
            context: "Actor: actor_1_name".to_string(),
            entry_type: "actor_name".to_string(),
            file_path: Some("www/data/Actors.json".to_string()),
        };
        let text_unit_2 = TextUnit {
            id: "actor_2_name".to_string(),
            source_text: "お兄ちゃん".to_string(),
            translated_text: "Onii-chan".to_string(),
            field_type: "name:www/data/Actors.json:2".to_string(),
            status: TranslationStatus::Translated,
            prompt_type: PromptType::Character,
            context: "Actor: actor_2_name".to_string(),
            entry_type: "actor_name".to_string(),
            file_path: Some("www/data/Actors.json".to_string()),
        };
        let text_units = vec![&text_unit_1, &text_unit_2];

        let result = inject_translations(project_path, "www/data/Actors.json", &text_units);

        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection
        let content: Vec<Option<Actor>> =
            serde_json::from_reader(std::fs::File::open(&temp_actors_file).unwrap()).unwrap();

        assert_eq!(content[1].as_ref().unwrap().name, "Tae-chan");
        assert_eq!(content[2].as_ref().unwrap().name, "Onii-chan");
        // Other actors should remain unchanged
        assert_eq!(content[3].as_ref().unwrap().name, "たえ");
    }
}
