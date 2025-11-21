// States.json parser for RPG Maker MV/MZ
// Extracts and injects text from state data

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

/// State data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: u32,
    pub name: String,
    pub message1: String,
    pub message2: String,
    pub message3: String,
    pub message4: String,
    /// All other fields preserved to avoid data loss during injection
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}

/// States parser implementation
pub struct StatesParser;

impl crate::parsers::engine::FileParser for StatesParser {
    fn extract(
        &self,
        _file_path: &Path,
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

/// Extracts translatable text from States.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the States.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for States.json
    let parse_states = |content: &str| -> AppResult<Vec<Option<State>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse States.json: {}", e)))
    };

    // Extract function for each state
    let extract_state_units = |state: &State, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip null state at index 0
        if index == 0 || state.id == 0 {
            return Vec::new();
        }

        let mut fields = Vec::new();

        // Add raw text for each field if not empty (formatting happens later in the pipeline)
        if !state.name.trim().is_empty() {
            fields.push(("name", state.name.as_str(), PromptType::System));
        }

        if !state.message1.trim().is_empty() {
            fields.push(("message1", state.message1.as_str(), PromptType::System));
        }

        if !state.message2.trim().is_empty() {
            fields.push(("message2", state.message2.as_str(), PromptType::System));
        }

        if !state.message3.trim().is_empty() {
            fields.push(("message3", state.message3.as_str(), PromptType::System));
        }

        if !state.message4.trim().is_empty() {
            fields.push(("message4", state.message4.as_str(), PromptType::System));
        }

        extract_text_units_for_object("state", state.id as i32, file_path, index, fields)
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "States.json",
        parse_states,
        extract_state_units,
    )
}

/// Injects translated text back into States.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the States.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for States.json
    let parse_states = |content: &str| -> AppResult<Vec<Option<State>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse States.json: {}", e)))
    };

    // Update function for each state
    let update_state = |state: &mut State, text_unit_map: &HashMap<String, &TextUnit>| {
        // Prepare mutable references for injection
        let name_ref = &mut state.name;
        let message1_ref = &mut state.message1;
        let message2_ref = &mut state.message2;
        let message3_ref = &mut state.message3;
        let message4_ref = &mut state.message4;

        // Update each field and restore formatting
        if let Some(text_unit) = text_unit_map.get(&format!("state_{}_name", state.id)) {
            if !text_unit.translated_text.is_empty() {
                *name_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("state_{}_message1", state.id)) {
            if !text_unit.translated_text.is_empty() {
                *message1_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("state_{}_message2", state.id)) {
            if !text_unit.translated_text.is_empty() {
                *message2_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("state_{}_message3", state.id)) {
            if !text_unit.translated_text.is_empty() {
                *message3_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("state_{}_message4", state.id)) {
            if !text_unit.translated_text.is_empty() {
                *message4_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "States.json",
        text_units,
        parse_states,
        update_state,
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
    fn test_extract_mv_states() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/States.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV states: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has states - should extract names and messages
        assert!(game_data.text_units.len() > 0, "Should extract state texts");

        // Check that we have state entries
        let has_state_entry = game_data.text_units.iter().any(|e| e.id.contains("state_"));
        assert!(has_state_entry, "Should extract state texts");

        // Check first state (Death) has name and messages
        let death_name = game_data.text_units.iter().find(|e| e.id == "state_1_name");
        let death_message1 = game_data
            .text_units
            .iter()
            .find(|e| e.id == "state_1_message1");
        let death_message2 = game_data
            .text_units
            .iter()
            .find(|e| e.id == "state_1_message2");

        assert!(death_name.is_some(), "Should have state 1 name");
        assert!(death_message1.is_some(), "Should have state 1 message1");
        assert!(death_message2.is_some(), "Should have state 1 message2");
        assert_eq!(death_name.unwrap().source_text, "戦闘不能");
        assert_eq!(death_message1.unwrap().source_text, "は倒れた！");
        assert_eq!(death_message2.unwrap().source_text, "を倒した！");
    }

    #[test]
    fn test_extract_mz_states() {
        let project_path = get_test_games_path().join("MZgame");

        let result = extract_text(&project_path, "data/States.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ states: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game should have state entries
        let has_state_entry = game_data.text_units.iter().any(|e| e.id.contains("state_"));
        assert!(has_state_entry, "Should extract state texts");
    }

    #[test]
    fn test_extract_states_validation() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/States.json");

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
    fn test_inject_mv_states() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory structure to avoid modifying test files
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Copy the original file to temp directory
        let original_path = get_test_games_path().join("MVgame/www/data/States.json");
        let temp_states_file = data_dir.join("States.json");
        fs::copy(&original_path, &temp_states_file).unwrap();

        // Create translation entries
        let translations = vec![
            TranslationEntry {
                id: "state_1_name".to_string(),
                translated_text: "Death".to_string(),
            },
            TranslationEntry {
                id: "state_1_message1".to_string(),
                translated_text: "has fallen!".to_string(),
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

        let result = inject_translations(project_path, "www/data/States.json", &text_unit_refs);

        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection by re-extracting
        let verify_result = extract_text(&project_path, "www/data/States.json");
        assert!(verify_result.is_ok());

        let verify_data = verify_result.unwrap();
        let death_name = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "state_1_name")
            .unwrap();
        let death_message1 = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "state_1_message1")
            .unwrap();

        assert_eq!(death_name.source_text, "Death");
        assert_eq!(death_message1.source_text, "has fallen!");
    }
}
