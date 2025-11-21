// MapInfos.json parser for RPG Maker MV/MZ
// Extracts and injects map information (names, structure)

use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit, TranslationEntry};
use crate::parsers::text::formatter::EngineFormatter;
use crate::parsers::text::formatter::RpgMakerFormatter;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_from_file_with_objects, extract_text_units_for_object,
    inject_text_units_for_object, inject_translations_into_file_with_objects, GameDataFile,
};

/// Map info data structure from RPG Maker MV/MZ MapInfos.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapInfo {
    pub id: u32,
    pub name: Option<String>,
    /// Additional fields that might be present in the JSON
    #[serde(flatten)]
    pub extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// MapInfos parser implementation
pub struct MapInfosParser;

impl crate::parsers::engine::FileParser for MapInfosParser {
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

/// Extracts translatable text from MapInfos.json
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    let parse_map_infos = |content: &str| -> AppResult<Vec<Option<MapInfo>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse MapInfos.json: {}", e)))
    };

    let extract_map_info_units =
        |map_info: &MapInfo, index: usize, file_path: &str| -> Vec<TextUnit> {
            // Skip null map info at index 0
            if index == 0 || map_info.id == 0 {
                return Vec::new();
            }

            let mut fields = Vec::new();

            if let Some(name) = &map_info.name {
                if !name.trim().is_empty() {
                    fields.push(("name", name.as_str(), PromptType::System));
                }
            }

            extract_text_units_for_object("map_info", map_info.id as i32, file_path, index, fields)
        };

    extract_text_from_file_with_objects(
        project_path,
        file_path,
        "MapInfos.json",
        parse_map_infos,
        extract_map_info_units,
    )
}

/// Injects translated text back into MapInfos.json
pub fn inject_translations(
    project_path: &Path,
    file_path: &str,
    text_units: &[&TextUnit],
) -> AppResult<()> {
    let parse_map_infos = |content: &str| -> AppResult<Vec<Option<MapInfo>>> {
        serde_json::from_str(content)
            .map_err(|e| AppError::Parsing(format!("Failed to parse MapInfos.json: {}", e)))
    };

    let update_map_info = |map_info: &mut MapInfo, text_unit_map: &HashMap<String, &TextUnit>| {
        let unit_id = format!("map_info_{}_name", map_info.id);
        if let Some(text_unit) = text_unit_map.get(&unit_id) {
            if !text_unit.translated_text.is_empty() {
                map_info.name = Some(RpgMakerFormatter::restore_after_translation(
                    &text_unit.translated_text,
                ));
            }
        }
    };

    inject_translations_into_file_with_objects(
        project_path,
        file_path,
        "MapInfos.json",
        text_units,
        parse_map_infos,
        update_map_info,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::engine::TranslationStatus;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    fn get_test_games_path() -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(manifest_dir)
            .parent()
            .unwrap()
            .join("engines_past")
    }

    #[test]
    fn test_extract_mv_map_infos() {
        let project_path = get_test_games_path().join("MVgame");
        let result = extract_text(&project_path, "www/data/MapInfos.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV map infos: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MV game has 11 maps (index 0 is null, so 11 real maps with names)
        assert_eq!(
            game_data.text_units.len(),
            11,
            "Expected 11 map names, got {}",
            game_data.text_units.len()
        );

        // Check some map names
        let map_names: Vec<_> = game_data
            .text_units
            .iter()
            .filter(|e| e.id.contains("_name"))
            .collect();

        assert_eq!(map_names.len(), 11);

        // Check first map (山小屋)
        let first_map = map_names.iter().find(|e| e.id == "map_info_1_name");
        assert!(first_map.is_some(), "Should have map info 1 name");
        assert_eq!(first_map.unwrap().source_text, "山小屋");
    }

    #[test]
    fn test_extract_mz_map_infos() {
        let project_path = get_test_games_path().join("MZgame");
        let result = extract_text(&project_path, "data/MapInfos.json");

        assert!(
            result.is_ok(),
            "Failed to extract MZ map infos: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // MZ game has maps with names
        let map_names: Vec<_> = game_data
            .text_units
            .iter()
            .filter(|e| e.id.contains("_name"))
            .collect();

        assert!(map_names.len() > 0, "Should extract some map names");

        // Check that we have formatted text (no raw special characters)
        for map_name in &map_names {
            assert!(
                !map_name.source_text.trim().is_empty(),
                "Map name should not be empty: {:?}",
                map_name
            );
        }
    }

    #[test]
    fn test_inject_mv_map_infos() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        let original_path = get_test_games_path().join("MVgame/www/data/MapInfos.json");
        let temp_map_infos_file = data_dir.join("MapInfos.json");
        fs::copy(&original_path, &temp_map_infos_file).unwrap();

        let text_unit_name = TextUnit {
            id: "map_info_1_name".to_string(),
            source_text: "山小屋".to_string(),
            translated_text: "Mountain Cabin".to_string(),
            field_type: "name:www/data/MapInfos.json:1".to_string(),
            status: TranslationStatus::Translated,
            text_type: PromptType::System,
            location: "map_info:1:name".to_string(), // Structured location format
            entry_type: "map_info_name".to_string(),
            file_path: Some("www/data/MapInfos.json".to_string()),
        };

        let text_units = vec![&text_unit_name];

        let result = inject_translations(project_path, "www/data/MapInfos.json", &text_units);
        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        let content: Vec<Option<MapInfo>> =
            serde_json::from_reader(std::fs::File::open(&temp_map_infos_file).unwrap()).unwrap();

        assert_eq!(
            content[1].as_ref().unwrap().name.as_ref().unwrap(),
            "Mountain Cabin"
        );
    }
}
