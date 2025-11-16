// Enemies.json parser for RPG Maker MV/MZ
// Extracts and injects text from enemy data

use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit, TranslationEntry};
use crate::parsers::text::formatter::EngineFormatter;
use crate::parsers::text::formatter::RpgMakerFormatter;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object,
    inject_text_units_for_object, inject_translations_into_file_with_objects, GameDataFile,
};

/// Enemy data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub id: u32,
    pub name: String,
    /// All other fields preserved to avoid data loss during injection
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}

/// Enemies parser implementation
pub struct EnemiesParser;

impl crate::parsers::engine::FileParser for EnemiesParser {
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

/// Extracts translatable text from Enemies.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Enemies.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Enemies.json
    let parse_enemies = |content: &str| -> AppResult<Vec<Option<Enemy>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Enemies.json: {}", e)))
    };

    // Extract function for each enemy
    let extract_enemy_units = |enemy: &Enemy, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip null enemy at index 0
        if index == 0 || enemy.id == 0 {
            return Vec::new();
        }

        let mut fields = Vec::new();

        // Add raw text for each field if not empty (formatting happens later in the pipeline)
        if !enemy.name.trim().is_empty() {
            fields.push(("name", enemy.name.as_str(), PromptType::Character));
        }

        extract_text_units_for_object("enemy", enemy.id as i32, file_path, index, fields)
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Enemies.json",
        parse_enemies,
        extract_enemy_units,
    )
}

/// Injects translated text back into Enemies.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Enemies.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Enemies.json
    let parse_enemies = |content: &str| -> AppResult<Vec<Option<Enemy>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Enemies.json: {}", e)))
    };

    // Update function for each enemy
    let update_enemy = |enemy: &mut Enemy, text_unit_map: &HashMap<String, &TextUnit>| {
        // Update the name field and restore formatting
        if let Some(text_unit) = text_unit_map.get(&format!("enemy_{}_name", enemy.id)) {
            if !text_unit.translated_text.is_empty() {
                enemy.name =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Enemies.json",
        text_units,
        parse_enemies,
        update_enemy,
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
    fn test_extract_mv_enemies() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Enemies.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV enemies: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has enemies - should extract names
        assert!(game_data.text_units.len() > 0, "Should extract enemy texts");

        // Check that we have enemy entries
        let has_enemy_entry = game_data.text_units.iter().any(|e| e.id.contains("enemy_"));
        assert!(has_enemy_entry, "Should extract enemy texts");

        // Check first enemy has name
        let first_enemy_name = game_data.text_units.iter().find(|e| e.id == "enemy_1_name");
        assert!(first_enemy_name.is_some(), "Should have enemy 1 name");
        assert_eq!(first_enemy_name.unwrap().source_text, "こうもり");
    }

    #[test]
    fn test_extract_mz_enemies() {
        let project_path = get_test_games_path().join("MZgame");

        let result = extract_text(&project_path, "data/Enemies.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ enemies: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game should have enemy entries
        let has_enemy_entry = game_data.text_units.iter().any(|e| e.id.contains("enemy_"));
        assert!(has_enemy_entry, "Should extract enemy texts");
    }

    #[test]
    fn test_extract_enemies_validation() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Enemies.json");

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
    fn test_inject_mv_enemies() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory structure to avoid modifying test files
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Copy the original file to temp directory
        let original_path = get_test_games_path().join("MVgame/www/data/Enemies.json");
        let temp_enemies_file = data_dir.join("Enemies.json");
        fs::copy(&original_path, &temp_enemies_file).unwrap();

        // Create translation entries
        let translations = vec![
            TranslationEntry {
                id: "enemy_1_name".to_string(),
                translated_text: "Bat".to_string(),
            },
            TranslationEntry {
                id: "enemy_2_name".to_string(),
                translated_text: "Slime".to_string(),
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
                text_type: PromptType::Character,
                location: String::new(),
                entry_type: String::new(),
                file_path: None,
            })
            .collect();

        let text_unit_refs: Vec<_> = text_units.iter().collect();

        let result = inject_translations(project_path, "www/data/Enemies.json", &text_unit_refs);

        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection by re-extracting
        let verify_result = extract_text(&project_path, "www/data/Enemies.json");
        assert!(verify_result.is_ok());

        let verify_data = verify_result.unwrap();
        let bat_entry = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "enemy_1_name")
            .unwrap();
        let slime_entry = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "enemy_2_name")
            .unwrap();

        assert_eq!(bat_entry.source_text, "Bat");
        assert_eq!(slime_entry.source_text, "Slime");
    }
}
