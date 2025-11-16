// Items.json parser for RPG Maker MV/MZ
// Extracts and injects text from item data

use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit};
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

/// Item data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: u32,
    /// All other fields preserved to avoid data loss during injection
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}

/// Items parser implementation
pub struct ItemsParser;

impl crate::parsers::engine::FileParser for ItemsParser {
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

/// Extracts translatable text from Items.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Items.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Items.json
    let parse_items = |content: &str| -> AppResult<Vec<Option<Item>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Items.json: {}", e)))
    };

    // Extract function for each item
    let extract_item_units = |item: &Item, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip null item at index 0
        if index == 0 || item.id == 0 {
            return Vec::new();
        }

        let mut fields = Vec::new();

        // Add raw text for each field if not empty (formatting happens later in the pipeline)
        if !item.name.trim().is_empty() {
            fields.push(("name", item.name.as_str(), PromptType::Item));
        }

        if !item.description.trim().is_empty() {
            fields.push(("description", item.description.as_str(), PromptType::Item));
        }

        extract_text_units_for_object("item", item.id as i32, file_path, index, fields)
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Items.json",
        parse_items,
        extract_item_units,
    )
}

/// Injects translated text back into Items.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Items.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Items.json
    let parse_items = |content: &str| -> AppResult<Vec<Option<Item>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Items.json: {}", e)))
    };

    // Update function for each item
    let update_item = |item: &mut Item, text_unit_map: &HashMap<String, &TextUnit>| {
        // Prepare mutable references for injection
        let mut name_ref = &mut item.name;
        let mut description_ref = &mut item.description;

        // Update each field and restore formatting
        if let Some(text_unit) = text_unit_map.get(&format!("item_{}_name", item.id)) {
            if !text_unit.translated_text.is_empty() {
                *name_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("item_{}_description", item.id)) {
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
        "Items.json",
        text_units,
        parse_items,
        update_item,
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
    fn test_extract_mv_items() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Items.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV items: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has items - should extract names and descriptions
        assert!(
            game_data.text_units.len() > 0,
            "Expected some text units, got {}",
            game_data.text_units.len()
        );

        // Check that we have item entries
        let has_item_entry = game_data.text_units.iter().any(|e| e.id.contains("item_"));
        assert!(has_item_entry, "Should extract item texts");

        // Check first item has name (descriptions are empty in MV test data)
        let first_item_name = game_data.text_units.iter().find(|e| e.id == "item_1_name");

        assert!(first_item_name.is_some(), "Should have item 1 name");
        assert_eq!(first_item_name.unwrap().source_text, "ポーション");

        // Check that we have at least some items with names
        let name_count = game_data
            .text_units
            .iter()
            .filter(|e| e.id.contains("_name"))
            .count();
        assert!(name_count > 0, "Should have item names");
    }

    #[test]
    fn test_extract_mz_items() {
        let project_path = get_test_games_path().join("MZgame");

        let result = extract_text(&project_path, "data/Items.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ items: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game has items - should extract names and descriptions
        assert!(
            game_data.text_units.len() > 0,
            "Expected some text units, got {}",
            game_data.text_units.len()
        );

        // Check that we have item entries
        let has_item_entry = game_data.text_units.iter().any(|e| e.id.contains("item_"));
        assert!(has_item_entry, "Should extract item texts");
    }

    #[test]
    fn test_extract_items_validation() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Items.json");

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
    fn test_inject_mv_items() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory structure to avoid modifying test files
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Copy the original file to temp directory
        let original_path = get_test_games_path().join("MVgame/www/data/Items.json");
        let temp_items_file = data_dir.join("Items.json");
        fs::copy(&original_path, &temp_items_file).unwrap();

        // Create text units for injection - use item 1 name
        let text_unit_name = TextUnit {
            id: "item_1_name".to_string(),
            source_text: "ポーション".to_string(),
            translated_text: "Potion".to_string(),
            field_type: "name:www/data/Items.json:1".to_string(),
            status: TranslationStatus::Translated,
            text_type: PromptType::Item,
            location: "item:1:name".to_string(), // Structured location format
            entry_type: "item_name".to_string(),
            file_path: Some("www/data/Items.json".to_string()),
        };

        let text_units = vec![&text_unit_name];

        let result = inject_translations(project_path, "www/data/Items.json", &text_units);

        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection by re-extracting
        let verify_result = extract_text(&project_path, "www/data/Items.json").unwrap();
        let updated_item = verify_result
            .text_units
            .iter()
            .find(|e| e.id == "item_1_name")
            .unwrap();

        assert_eq!(updated_item.source_text, "Potion");
    }
}
