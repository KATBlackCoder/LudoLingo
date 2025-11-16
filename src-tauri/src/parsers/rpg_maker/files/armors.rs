// Armors.json parser for RPG Maker MV/MZ
// Extracts and injects text from armor data

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

/// Armor data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub id: u32,
    pub name: String,
    pub description: String,
    /// All other fields preserved to avoid data loss during injection
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}

/// Armors parser implementation
pub struct ArmorsParser;

impl crate::parsers::engine::FileParser for ArmorsParser {
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

/// Extracts translatable text from Armors.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Armors.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Armors.json
    let parse_armors = |content: &str| -> AppResult<Vec<Option<Armor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Armors.json: {}", e)))
    };

    // Extract function for each armor
    let extract_armor_units = |armor: &Armor, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip null armor at index 0
        if index == 0 || armor.id == 0 {
            return Vec::new();
        }

        let mut fields = Vec::new();

        // Add raw text for each field if not empty (formatting happens later in the pipeline)
        if !armor.name.trim().is_empty() {
            fields.push(("name", armor.name.as_str(), PromptType::Item));
        }

        if !armor.description.trim().is_empty() {
            fields.push(("description", armor.description.as_str(), PromptType::Item));
        }

        extract_text_units_for_object("armor", armor.id as i32, file_path, index, fields)
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Armors.json",
        parse_armors,
        extract_armor_units,
    )
}

/// Injects translated text back into Armors.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Armors.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Armors.json
    let parse_armors = |content: &str| -> AppResult<Vec<Option<Armor>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Armors.json: {}", e)))
    };

    // Update function for each armor
    let update_armor = |armor: &mut Armor, text_unit_map: &HashMap<String, &TextUnit>| {
        // Prepare mutable references for injection
        let mut name_ref = &mut armor.name;
        let mut description_ref = &mut armor.description;

        // Update each field and restore formatting
        if let Some(text_unit) = text_unit_map.get(&format!("armor_{}_name", armor.id)) {
            if !text_unit.translated_text.is_empty() {
                *name_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("armor_{}_description", armor.id)) {
            if !text_unit.translated_text.is_empty() {
                *description_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Armors.json",
        text_units,
        parse_armors,
        update_armor,
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
    fn test_extract_mv_armors() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Armors.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV armors: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has armors - should extract names (descriptions are empty)
        assert!(game_data.text_units.len() > 0, "Should extract armor texts");

        // Check that we have armor entries
        let has_armor_entry = game_data.text_units.iter().any(|e| e.id.contains("armor_"));
        assert!(has_armor_entry, "Should extract armor texts");

        // Check first armor has name
        let first_armor_name = game_data.text_units.iter().find(|e| e.id == "armor_1_name");
        assert!(first_armor_name.is_some(), "Should have armor 1 name");
        assert_eq!(first_armor_name.unwrap().source_text, "盾");
    }

    #[test]
    fn test_extract_mz_armors() {
        let project_path = get_test_games_path().join("MZgame");

        let result = extract_text(&project_path, "data/Armors.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ armors: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game should have armor entries
        let has_armor_entry = game_data.text_units.iter().any(|e| e.id.contains("armor_"));
        assert!(has_armor_entry, "Should extract armor texts");
    }

    #[test]
    fn test_extract_armors_validation() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Armors.json");

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
    fn test_inject_mv_armors() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory structure to avoid modifying test files
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Copy the original file to temp directory
        let original_path = get_test_games_path().join("MVgame/www/data/Armors.json");
        let temp_armors_file = data_dir.join("Armors.json");
        fs::copy(&original_path, &temp_armors_file).unwrap();

        // Create translation entries
        let translations = vec![
            TranslationEntry {
                id: "armor_1_name".to_string(),
                translated_text: "Shield".to_string(),
            },
            TranslationEntry {
                id: "armor_2_name".to_string(),
                translated_text: "Hat".to_string(),
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
                text_type: PromptType::Item,
                location: String::new(),
                entry_type: String::new(),
                file_path: None,
            })
            .collect();

        let text_unit_refs: Vec<_> = text_units.iter().collect();

        let result = inject_translations(project_path, "www/data/Armors.json", &text_unit_refs);

        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection by re-extracting
        let verify_result = extract_text(&project_path, "www/data/Armors.json");
        assert!(verify_result.is_ok());

        let verify_data = verify_result.unwrap();
        let shield_entry = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "armor_1_name")
            .unwrap();
        let hat_entry = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "armor_2_name")
            .unwrap();

        assert_eq!(shield_entry.source_text, "Shield");
        assert_eq!(hat_entry.source_text, "Hat");
    }
}
