// Troops.json parser for RPG Maker MV/MZ
// Extracts and injects text from troop data

use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit, TranslationEntry};
use crate::parsers::text::formatter_trait::EngineFormatter;
use crate::parsers::text::rpg_maker_formatter::RpgMakerFormatter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object,
    inject_text_units_for_object, inject_translations_into_file_with_objects, GameDataFile,
};

/// Troop data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Troop {
    pub id: u32,
    pub name: String,
    // Other fields omitted for brevity (members, pages, etc.)
}

/// Troops parser implementation
pub struct TroopsParser;

impl crate::parsers::engine::FileParser for TroopsParser {
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

/// Extracts translatable text from Troops.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Troops.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Troops.json
    let parse_troops = |content: &str| -> AppResult<Vec<Option<Troop>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Troops.json: {}", e)))
    };

    // Extract function for each troop
    let extract_troop_units = |troop: &Troop, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip null troop at index 0
        if index == 0 || troop.id == 0 {
            return Vec::new();
        }

        let mut fields = Vec::new();

        // Add raw text for each field if not empty (formatting happens later in the pipeline)
        if !troop.name.trim().is_empty() {
            fields.push(("name", troop.name.as_str(), PromptType::System));
        }

        extract_text_units_for_object("troop", troop.id as i32, file_path, index, fields)
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Troops.json",
        parse_troops,
        extract_troop_units,
    )
}

/// Injects translated text back into Troops.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Troops.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Troops.json
    let parse_troops = |content: &str| -> AppResult<Vec<Option<Troop>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Troops.json: {}", e)))
    };

    // Update function for each troop
    let update_troop = |troop: &mut Troop, text_unit_map: &HashMap<String, &TextUnit>| {
        // Update the name field and restore formatting
        if let Some(text_unit) = text_unit_map.get(&format!("troop_{}_name", troop.id)) {
            if !text_unit.translated_text.is_empty() {
                troop.name =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Troops.json",
        text_units,
        parse_troops,
        update_troop,
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
    fn test_extract_mv_troops() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Troops.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV troops: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has troops - should extract names
        assert!(game_data.text_units.len() > 0, "Should extract troop texts");

        // Check that we have troop entries
        let has_troop_entry = game_data.text_units.iter().any(|e| e.id.contains("troop_"));
        assert!(has_troop_entry, "Should extract troop texts");

        // Check first troop has name
        let first_troop_name = game_data.text_units.iter().find(|e| e.id == "troop_1_name");
        assert!(first_troop_name.is_some(), "Should have troop 1 name");
        assert_eq!(first_troop_name.unwrap().source_text, "こうもり*2");
    }

    #[test]
    fn test_extract_mz_troops() {
        let project_path = get_test_games_path().join("MZgame");

        let result = extract_text(&project_path, "data/Troops.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ troops: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game should have troop entries
        let has_troop_entry = game_data.text_units.iter().any(|e| e.id.contains("troop_"));
        assert!(has_troop_entry, "Should extract troop texts");
    }

    #[test]
    fn test_extract_troops_validation() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Troops.json");

        assert!(result.is_ok());
        let game_data = result.unwrap();

        // All entries should pass validation (non-empty and valid text)
        for unit in &game_data.text_units {
            assert!(
                !unit.source_text.trim().is_empty(),
                "Entry should not be empty: {:?}",
                unit
            );
        }
    }

    #[test]
    fn test_inject_mv_troops() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory structure to avoid modifying test files
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Copy the original file to temp directory
        let original_path = get_test_games_path().join("MVgame/www/data/Troops.json");
        let temp_troops_file = data_dir.join("Troops.json");
        fs::copy(&original_path, &temp_troops_file).unwrap();

        // Create translation entries
        let translations = vec![
            TranslationEntry {
                id: "troop_1_name".to_string(),
                translated_text: "Bats*2".to_string(),
            },
            TranslationEntry {
                id: "troop_2_name".to_string(),
                translated_text: "Slimes*2".to_string(),
            },
        ];

        let text_units: Vec<_> = translations
            .iter()
            .map(|t| TextUnit {
                id: t.id.clone(),
                source_text: String::new(),
                translated_text: t.translated_text.clone(),
                field_type: String::new(),
                status: TranslationStatus::Translated,
                text_type: PromptType::System,
                location: String::new(),
                entry_type: String::new(),
                file_path: None,
            })
            .collect();

        let text_unit_refs: Vec<_> = text_units.iter().collect();

        let result = inject_translations(project_path, "www/data/Troops.json", &text_unit_refs);

        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection by re-extracting
        let verify_result = extract_text(&project_path, "www/data/Troops.json");
        assert!(verify_result.is_ok());

        let verify_data = verify_result.unwrap();
        let bats_troop = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "troop_1_name")
            .unwrap();
        let slimes_troop = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "troop_2_name")
            .unwrap();

        assert_eq!(bats_troop.source_text, "Bats*2");
        assert_eq!(slimes_troop.source_text, "Slimes*2");
    }
}
