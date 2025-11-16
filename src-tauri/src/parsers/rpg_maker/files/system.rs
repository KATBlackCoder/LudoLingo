// System.json parser for RPG Maker MV/MZ
// Extracts and injects system settings and terminology

use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit};
use crate::parsers::text::formatter::EngineFormatter;
use crate::parsers::text::formatter::RpgMakerFormatter;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use super::common::GameDataFile;

/// System data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemData {
    /// Game title
    #[serde(default)]
    pub gameTitle: String,

    /// Currency unit (e.g., "G", "Gold")
    #[serde(default)]
    pub currencyUnit: String,

    /// Game terminology
    #[serde(default)]
    pub terms: Terms,

    /// Armor type names
    #[serde(default)]
    pub armorTypes: Vec<String>,

    /// Element names
    #[serde(default)]
    pub elements: Vec<String>,

    /// Equipment type names
    #[serde(default)]
    pub equipTypes: Vec<String>,

    /// Skill type names
    #[serde(default)]
    pub skillTypes: Vec<String>,

    /// Weapon type names
    #[serde(default)]
    pub weaponTypes: Vec<String>,

    /// All other fields
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}

/// Game terminology structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Terms {
    /// Basic terms (Level, HP, MP, etc.)
    #[serde(default)]
    pub basic: Vec<String>,

    /// Command terms (Fight, Escape, Attack, etc.) - can contain null values
    #[serde(default)]
    pub commands: Vec<Option<String>>,

    /// Parameter names (Max HP, Max MP, Attack, etc.)
    #[serde(default)]
    pub params: Vec<String>,

    /// Message texts
    #[serde(default)]
    pub messages: HashMap<String, String>,
}

/// System parser implementation
pub struct SystemParser;

impl crate::parsers::engine::FileParser for SystemParser {
    fn extract(
        &self,
        _file_path: &Path,
        _version: crate::parsers::engine::GameEngine,
    ) -> Result<Vec<crate::parsers::engine::TextEntry>, String> {
        Err("Use extract_text function directly".to_string())
    }

    fn inject(
        &self,
        _file_path: &Path,
        _translations: &[crate::parsers::engine::TranslationEntry],
        _version: crate::parsers::engine::GameEngine,
    ) -> Result<(), String> {
        Err("Use inject_translations function directly".to_string())
    }
}

/// Extracts translatable text from System.json
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    let full_path = project_path.join(file_path);

    // Read the JSON file
    let content = std::fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_path, e)))?;

    // Parse the JSON content
    let system_data: SystemData = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse {}: {}", file_path, e)))?;

    let mut text_units = Vec::new();

    // Extract game title
    if !system_data.gameTitle.trim().is_empty() {
        let prepared_text = RpgMakerFormatter::prepare_for_translation(&system_data.gameTitle);
        text_units.push(TextUnit {
            id: "system_gameTitle".to_string(),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("gameTitle:{}:0", file_path),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: "system:game_title".to_string(), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract currency unit
    if !system_data.currencyUnit.trim().is_empty() {
        let prepared_text = RpgMakerFormatter::prepare_for_translation(&system_data.currencyUnit);
        text_units.push(TextUnit {
            id: "system_currencyUnit".to_string(),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("currencyUnit:{}:0", file_path),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: "system:currency_unit".to_string(), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract armor types (skip index 0 which is usually empty)
    for (index, armor_type) in system_data.armorTypes.iter().enumerate() {
        if index == 0 || armor_type.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(armor_type);
        text_units.push(TextUnit {
            id: format!("system_armorType_{}", index),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("armorTypes:{}:{}", file_path, index),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:armor_type:{}", index), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract elements (skip index 0 which is usually empty)
    for (index, element) in system_data.elements.iter().enumerate() {
        if index == 0 || element.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(element);
        text_units.push(TextUnit {
            id: format!("system_element_{}", index),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("elements:{}:{}", file_path, index),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:element:{}", index), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract equipment types (skip index 0 which is usually empty)
    for (index, equip_type) in system_data.equipTypes.iter().enumerate() {
        if index == 0 || equip_type.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(equip_type);
        text_units.push(TextUnit {
            id: format!("system_equipType_{}", index),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("equipTypes:{}:{}", file_path, index),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:equipment_type:{}", index), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract skill types (skip index 0 which is usually empty)
    for (index, skill_type) in system_data.skillTypes.iter().enumerate() {
        if index == 0 || skill_type.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(skill_type);
        text_units.push(TextUnit {
            id: format!("system_skillType_{}", index),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("skillTypes:{}:{}", file_path, index),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:skill_type:{}", index), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract weapon types (skip index 0 which is usually empty)
    for (index, weapon_type) in system_data.weaponTypes.iter().enumerate() {
        if index == 0 || weapon_type.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(weapon_type);
        text_units.push(TextUnit {
            id: format!("system_weaponType_{}", index),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("weaponTypes:{}:{}", file_path, index),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:weapon_type:{}", index), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract terms.basic
    for (index, basic_term) in system_data.terms.basic.iter().enumerate() {
        if basic_term.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(basic_term);
        text_units.push(TextUnit {
            id: format!("system_terms_basic_{}", index),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("terms.basic:{}:{}", file_path, index),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:terms:basic:{}", index), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract terms.commands
    for (index, command_opt) in system_data.terms.commands.iter().enumerate() {
        if let Some(command) = command_opt {
            if command.trim().is_empty() {
                continue;
            }
            let prepared_text = RpgMakerFormatter::prepare_for_translation(command);
            text_units.push(TextUnit {
                id: format!("system_terms_command_{}", index),
                source_text: prepared_text,
                translated_text: String::new(),
                field_type: format!("terms.commands:{}:{}", file_path, index),
                status: crate::parsers::engine::TranslationStatus::NotTranslated,
                text_type: PromptType::System,
                location: format!("system:terms:command:{}", index), // Structured location format
                entry_type: "system_text_unit".to_string(),
                file_path: Some(file_path.to_string()),
            });
        }
    }

    // Extract terms.params
    for (index, param) in system_data.terms.params.iter().enumerate() {
        if param.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(param);
        text_units.push(TextUnit {
            id: format!("system_terms_param_{}", index),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("terms.params:{}:{}", file_path, index),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:terms:param:{}", index), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    // Extract terms.messages
    for (key, message) in system_data.terms.messages.iter() {
        if message.trim().is_empty() {
            continue;
        }
        let prepared_text = RpgMakerFormatter::prepare_for_translation(message);
        text_units.push(TextUnit {
            id: format!("system_terms_message_{}", key),
            source_text: prepared_text,
            translated_text: String::new(),
            field_type: format!("terms.messages.{}:{}:0", key, file_path),
            status: crate::parsers::engine::TranslationStatus::NotTranslated,
            text_type: PromptType::System,
            location: format!("system:terms:message:{}", key), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some(file_path.to_string()),
        });
    }

    let text_unit_count = text_units.len() as u32;

    log::info!("Extracted {} text units from System.json", text_unit_count);

    Ok(GameDataFile {
        name: "System".to_string(),
        path: file_path.to_string(),
        text_units,
        text_unit_count,
    })
}

/// Injects translated text back into System.json
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    let full_path = project_path.join(file_path);

    // Read the current JSON file
    let content = std::fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_path, e)))?;

    // Parse the JSON content
    let mut system_data: SystemData = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse {}: {}", file_path, e)))?;

    // Create a map of text units for quick lookup
    let text_unit_map: HashMap<String, &TextUnit> = text_units
        .iter()
        .map(|unit| (unit.id.clone(), *unit))
        .collect();

    // Update game title
    if let Some(unit) = text_unit_map.get("system_gameTitle") {
        if !unit.translated_text.is_empty() {
            system_data.gameTitle =
                RpgMakerFormatter::restore_after_translation(&unit.translated_text);
        }
    }

    // Update currency unit
    if let Some(unit) = text_unit_map.get("system_currencyUnit") {
        if !unit.translated_text.is_empty() {
            system_data.currencyUnit =
                RpgMakerFormatter::restore_after_translation(&unit.translated_text);
        }
    }

    // Update armor types
    for (index, armor_type) in system_data.armorTypes.iter_mut().enumerate() {
        let unit_id = format!("system_armorType_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *armor_type = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Update elements
    for (index, element) in system_data.elements.iter_mut().enumerate() {
        let unit_id = format!("system_element_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *element = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Update equipment types
    for (index, equip_type) in system_data.equipTypes.iter_mut().enumerate() {
        let unit_id = format!("system_equipType_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *equip_type = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Update skill types
    for (index, skill_type) in system_data.skillTypes.iter_mut().enumerate() {
        let unit_id = format!("system_skillType_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *skill_type = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Update weapon types
    for (index, weapon_type) in system_data.weaponTypes.iter_mut().enumerate() {
        let unit_id = format!("system_weaponType_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *weapon_type = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Update terms.basic
    for (index, basic_term) in system_data.terms.basic.iter_mut().enumerate() {
        let unit_id = format!("system_terms_basic_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *basic_term = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Update terms.commands
    for (index, command_opt) in system_data.terms.commands.iter_mut().enumerate() {
        let unit_id = format!("system_terms_command_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *command_opt = Some(RpgMakerFormatter::restore_after_translation(
                    &unit.translated_text,
                ));
            }
        }
    }

    // Update terms.params
    for (index, param) in system_data.terms.params.iter_mut().enumerate() {
        let unit_id = format!("system_terms_param_{}", index);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *param = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Update terms.messages
    for (key, message) in system_data.terms.messages.iter_mut() {
        let unit_id = format!("system_terms_message_{}", key);
        if let Some(unit) = text_unit_map.get(&unit_id) {
            if !unit.translated_text.is_empty() {
                *message = RpgMakerFormatter::restore_after_translation(&unit.translated_text);
            }
        }
    }

    // Serialize the updated system data back to JSON
    let updated_content = serde_json::to_string_pretty(&system_data)
        .map_err(|e| AppError::Parsing(format!("Failed to serialize {}: {}", file_path, e)))?;

    // Write the updated content back to the file
    std::fs::write(&full_path, updated_content)
        .map_err(|e| AppError::FileSystem(format!("Failed to write {}: {}", file_path, e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::engine::TranslationStatus;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn get_test_games_path() -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(manifest_dir)
            .parent()
            .unwrap()
            .join("engines_past")
    }

    #[test]
    fn test_extract_mv_system() {
        let project_path = get_test_games_path().join("MVgame");
        let result = extract_text(&project_path, "www/data/System.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV system: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game should have many system texts
        assert!(
            game_data.text_units.len() > 50,
            "Expected many system texts, got {}",
            game_data.text_units.len()
        );

        // Check game title
        let game_title = game_data
            .text_units
            .iter()
            .find(|e| e.id == "system_gameTitle");
        assert!(game_title.is_some(), "Should have game title");
        assert_eq!(
            game_title.unwrap().source_text,
            "Hunterになりたい！！[FWSPC_1]ver1.1"
        );

        // Check currency unit
        let currency = game_data
            .text_units
            .iter()
            .find(|e| e.id == "system_currencyUnit");
        assert!(currency.is_some(), "Should have currency unit");
    }

    #[test]
    fn test_extract_mz_system() {
        let project_path = get_test_games_path().join("MZgame");
        let result = extract_text(&project_path, "data/System.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ system: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game should have many system texts
        assert!(
            game_data.text_units.len() > 50,
            "Expected many system texts, got {}",
            game_data.text_units.len()
        );

        // Check game title
        let game_title = game_data
            .text_units
            .iter()
            .find(|e| e.id == "system_gameTitle");
        assert!(game_title.is_some(), "Should have game title");
    }

    #[test]
    fn test_inject_mv_system() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        let original_path = get_test_games_path().join("MVgame/www/data/System.json");
        let temp_system_file = data_dir.join("System.json");
        fs::copy(&original_path, &temp_system_file).unwrap();

        let text_unit_title = TextUnit {
            id: "system_gameTitle".to_string(),
            source_text: "Hunterになりたい！！　ver1.1".to_string(),
            translated_text: "I Want to Be a Hunter!! v1.1".to_string(),
            field_type: "gameTitle:www/data/System.json:0".to_string(),
            status: TranslationStatus::Translated,
            text_type: PromptType::System,
            location: "system:game_title".to_string(), // Structured location format
            entry_type: "system_text_unit".to_string(),
            file_path: Some("www/data/System.json".to_string()),
        };

        let text_units = vec![&text_unit_title];

        let result = inject_translations(project_path, "www/data/System.json", &text_units);
        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        let content: SystemData =
            serde_json::from_reader(std::fs::File::open(&temp_system_file).unwrap()).unwrap();

        assert_eq!(content.gameTitle, "I Want to Be a Hunter!! v1.1");
    }
}
