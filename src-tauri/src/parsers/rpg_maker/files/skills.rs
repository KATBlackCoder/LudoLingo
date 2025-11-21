// Skills.json parser for RPG Maker MV/MZ
// Extracts and injects text from skill data

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

/// Skill data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub message1: String,
    pub message2: String,
    /// All other fields preserved to avoid data loss during injection
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}

/// Skills parser implementation
pub struct SkillsParser;

impl crate::parsers::engine::FileParser for SkillsParser {
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

/// Extracts translatable text from Skills.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Skills.json file
///
/// # Returns
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    // Parse function for Skills.json
    let parse_skills = |content: &str| -> AppResult<Vec<Option<Skill>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Skills.json: {}", e)))
    };

    // Extract function for each skill
    let extract_skill_units = |skill: &Skill, index: usize, file_path: &str| -> Vec<TextUnit> {
        // Skip null skill at index 0
        if index == 0 || skill.id == 0 {
            return Vec::new();
        }

        let mut fields = Vec::new();

        // Add raw text for each field if not empty (formatting happens later in the pipeline)
        if !skill.name.trim().is_empty() {
            fields.push(("name", skill.name.as_str(), PromptType::Skill));
        }

        if !skill.description.trim().is_empty() {
            fields.push(("description", skill.description.as_str(), PromptType::Skill));
        }

        if !skill.message1.trim().is_empty() {
            fields.push(("message1", skill.message1.as_str(), PromptType::System));
        }

        if !skill.message2.trim().is_empty() {
            fields.push(("message2", skill.message2.as_str(), PromptType::System));
        }

        extract_text_units_for_object("skill", skill.id as i32, file_path, index, fields)
    };

    // Use the common function
    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "Skills.json",
        parse_skills,
        extract_skill_units,
    )
}

/// Injects translated text back into Skills.json
///
/// # Arguments
/// * `project_path` - Path to the project directory
/// * `file_path` - Relative path to the Skills.json file
/// * `text_units` - Vector of translated text units
///
/// # Returns
/// * `AppResult<()>` - Success or error
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    // Parse function for Skills.json
    let parse_skills = |content: &str| -> AppResult<Vec<Option<Skill>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse Skills.json: {}", e)))
    };

    // Update function for each skill
    let update_skill = |skill: &mut Skill, text_unit_map: &HashMap<String, &TextUnit>| {
        // Prepare mutable references for injection
        let name_ref = &mut skill.name;
        let description_ref = &mut skill.description;
        let message1_ref = &mut skill.message1;
        let message2_ref = &mut skill.message2;

        // Update each field and restore formatting
        if let Some(text_unit) = text_unit_map.get(&format!("skill_{}_name", skill.id)) {
            if !text_unit.translated_text.is_empty() {
                *name_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("skill_{}_description", skill.id)) {
            if !text_unit.translated_text.is_empty() {
                *description_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("skill_{}_message1", skill.id)) {
            if !text_unit.translated_text.is_empty() {
                *message1_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }

        if let Some(text_unit) = text_unit_map.get(&format!("skill_{}_message2", skill.id)) {
            if !text_unit.translated_text.is_empty() {
                *message2_ref =
                    RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
            }
        }
    };

    // Use the common function
    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "Skills.json",
        text_units,
        parse_skills,
        update_skill,
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
    fn test_extract_mv_skills() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Skills.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV skills: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has skills - should extract names and messages
        assert!(game_data.text_units.len() > 0, "Should extract skill texts");

        // Check that we have skill entries
        let has_skill_entry = game_data.text_units.iter().any(|e| e.id.contains("skill_"));
        assert!(has_skill_entry, "Should extract skill texts");

        // Check first skill (Attack) has name and message1
        let attack_name = game_data.text_units.iter().find(|e| e.id == "skill_1_name");
        let attack_message1 = game_data
            .text_units
            .iter()
            .find(|e| e.id == "skill_1_message1");

        assert!(attack_name.is_some(), "Should have skill 1 name");
        assert!(attack_message1.is_some(), "Should have skill 1 message1");
        assert_eq!(attack_name.unwrap().source_text, "攻撃");
        assert_eq!(attack_message1.unwrap().source_text, "の攻撃！");
    }

    #[test]
    fn test_extract_mz_skills() {
        let project_path = get_test_games_path().join("MZgame");

        let result = extract_text(&project_path, "data/Skills.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ skills: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game should have skill entries
        let has_skill_entry = game_data.text_units.iter().any(|e| e.id.contains("skill_"));
        assert!(has_skill_entry, "Should extract skill texts");
    }

    #[test]
    fn test_extract_skills_validation() {
        let project_path = get_test_games_path().join("MVgame");

        let result = extract_text(&project_path, "www/data/Skills.json");

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
    fn test_inject_mv_skills() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory structure to avoid modifying test files
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Copy the original file to temp directory
        let original_path = get_test_games_path().join("MVgame/www/data/Skills.json");
        let temp_skills_file = data_dir.join("Skills.json");
        fs::copy(&original_path, &temp_skills_file).unwrap();

        // Create translation entries
        let translations = vec![
            TranslationEntry {
                id: "skill_1_name".to_string(),
                translated_text: "Attack".to_string(),
            },
            TranslationEntry {
                id: "skill_1_message1".to_string(),
                translated_text: "'s attack!".to_string(),
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
                text_type: if t.id.contains("message") {
                    PromptType::System
                } else {
                    PromptType::Skill
                },
                location: String::new(),
                entry_type: String::new(),
                file_path: None,
            })
            .collect();

        let text_unit_refs: Vec<_> = text_units.iter().collect();

        let result = inject_translations(project_path, "www/data/Skills.json", &text_unit_refs);

        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection by re-extracting
        let verify_result = extract_text(&project_path, "www/data/Skills.json");
        assert!(verify_result.is_ok());

        let verify_data = verify_result.unwrap();
        let attack_name = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "skill_1_name")
            .unwrap();
        let attack_message1 = verify_data
            .text_units
            .iter()
            .find(|e| e.id == "skill_1_message1")
            .unwrap();

        assert_eq!(attack_name.source_text, "Attack");
        assert_eq!(attack_message1.source_text, "'s attack!");
    }
}
