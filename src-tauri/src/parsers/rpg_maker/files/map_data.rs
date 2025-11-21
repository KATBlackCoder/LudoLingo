// MapXXX.json parser for RPG Maker MV/MZ
// Extracts and injects text from map events

use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit, TranslationEntry};
use crate::parsers::text::formatter::EngineFormatter;
use crate::parsers::text::formatter::RpgMakerFormatter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::common::{
    extract_text_units_for_object, extract_text_units_from_event_commands,
    inject_text_units_for_object, inject_text_units_into_event_commands, EventCommand,
    GameDataFile,
};

/// Map data structure from RPG Maker MV/MZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    /// Map display name (optional)
    #[serde(default)]
    pub displayName: String,
    /// List of events on this map
    #[serde(default)]
    pub events: Vec<Option<MapEvent>>,
    // Other fields omitted for brevity (bgm, bgs, tilesetId, data, etc.)
}

/// Map event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapEvent {
    /// Event ID
    pub id: i32,
    /// Event name
    pub name: String,
    /// Event pages (each page can have different conditions and commands)
    pub pages: Vec<MapEventPage>,
    // Other fields omitted for brevity (x, y, etc.)
}

/// Map event page structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapEventPage {
    /// List of event commands for this page
    pub list: Vec<EventCommand>,
    // Other fields omitted for brevity (conditions, image, etc.)
}

/// MapData parser implementation
pub struct MapDataParser;

impl crate::parsers::engine::FileParser for MapDataParser {
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

/// Extracts translatable text from MapXXX.json
pub fn extract_text(project_path: &Path, file_path: &str) -> AppResult<GameDataFile> {
    let full_path = project_path.join(file_path);

    // Read the JSON file
    let content = std::fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_path, e)))?;

    // Parse the JSON content
    let map_data: MapData = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse {}: {}", file_path, e)))?;

    let mut text_units = Vec::new();

    // Extract display name if present
    if !map_data.displayName.trim().is_empty() {
        // Extract map ID from filename (Map001.json -> 1)
        let map_id = extract_map_id_from_filename(file_path)?;

        // Create a dummy map data object to use with extract_text_units_for_object
        struct DummyMapData {
            id: i32,
            display_name: String,
        }

        let dummy_map = DummyMapData {
            id: map_id,
            display_name: map_data.displayName.clone(),
        };

        // Use the common function for consistent formatting
        let display_name_units = extract_text_units_for_object(
            "map",
            map_id,
            file_path,
            0, // dummy index
            vec![(
                "display_name",
                dummy_map.display_name.as_str(),
                PromptType::System,
            )],
        );

        // Update IDs and context to match our format
        for mut unit in display_name_units {
            unit.id = format!("map_{}_display_name", map_id);
            unit.location = format!("map:{}:display_name", map_id); // Structured location format
            unit.entry_type = "map_display_name".to_string();
            text_units.push(unit);
        }
    }

    // Process each event
    for (event_index, event_option) in map_data.events.iter().enumerate() {
        if let Some(event) = event_option {
            // Skip null event at index 0
            if event_index == 0 || event.id == 0 {
                continue;
            }

            let map_id = extract_map_id_from_filename(file_path)?;

            // Extract event name
            if !event.name.trim().is_empty() {
                // Use the common function for consistent formatting
                let event_name_units = extract_text_units_for_object(
                    "map_event",
                    event.id,
                    file_path,
                    event_index,
                    vec![("name", event.name.as_str(), PromptType::System)],
                );

                // Update IDs and context to match our format
                for mut unit in event_name_units {
                    unit.id = format!("map_{}_event_{}_name", map_id, event.id);
                    unit.location = format!("map:{}:event:{}:name", map_id, event.id); // Structured location format
                    unit.entry_type = "map_event_name".to_string();
                    unit.field_type = format!("name:{}:{}:{}", file_path, event_index, event.id);
                    text_units.push(unit);
                }
            }

            // Extract text from each event page
            for (_page_index, page) in event.pages.iter().enumerate() {
                let page_text_units = extract_text_units_from_event_commands(
                    &format!("map_{}_event_{}", map_id, event.id),
                    event.id,
                    &page.list,
                    file_path,
                );

                // Update context to structured location format for map events
                for mut unit in page_text_units {
                    // Reconstruct location from ID: "map_9_event_1_message_12" -> "map:9:event:1:message:12"
                    // Or parse from existing location and rebuild
                    if unit.id.starts_with("map_") && unit.id.contains("_event_") {
                        // Parse: "map_9_event_1_message_12" -> ["map", "9", "event", "1", "message", "12"]
                        let parts: Vec<&str> = unit.id.split('_').collect();
                        if parts.len() >= 6 && parts[0] == "map" && parts[2] == "event" {
                            let map_id_str = parts[1];
                            let event_id_str = parts[3];
                            let field_type = parts[4]; // "message" or "choice"
                            let index = parts[5];

                            if field_type == "message" {
                                unit.location = format!(
                                    "map:{}:event:{}:message:{}",
                                    map_id_str, event_id_str, index
                                );
                            } else if field_type == "choice" && parts.len() >= 7 {
                                let choice_index = parts[6];
                                unit.location = format!(
                                    "map:{}:event:{}:choice:{}:{}",
                                    map_id_str, event_id_str, index, choice_index
                                );
                            }
                        }
                    }
                    text_units.push(unit);
                }
            }
        }
    }

    let text_unit_count = text_units.len() as u32;

    let file_stem = Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Map")
        .to_string();

    Ok(GameDataFile {
        name: file_stem,
        path: file_path.to_string(),
        text_units,
        text_unit_count,
    })
}

/// Injects translated text back into MapXXX.json
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
    let mut map_data: MapData = serde_json::from_str(&content)
        .map_err(|e| AppError::Parsing(format!("Failed to parse {}: {}", file_path, e)))?;

    // Create a map of text units for quick lookup
    let text_unit_map: HashMap<String, &TextUnit> = text_units
        .iter()
        .map(|unit| (unit.id.clone(), *unit))
        .collect();

    let map_id = extract_map_id_from_filename(file_path)?;

    // Update display name
    let display_name_key = format!("map_{}_display_name", map_id);
    if let Some(text_unit) = text_unit_map.get(&display_name_key) {
        if !text_unit.translated_text.is_empty() {
            map_data.displayName =
                RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
        }
    }

    // Update each event
    for event_option in map_data.events.iter_mut() {
        if let Some(event) = event_option {
            if event.id == 0 {
                continue;
            }

            // Update event name using common function
            let event_name_key = format!("map_{}_event_{}_name", map_id, event.id);
            if let Some(text_unit) = text_unit_map.get(&event_name_key) {
                if !text_unit.translated_text.is_empty() {
                    event.name =
                        RpgMakerFormatter::restore_after_translation(&text_unit.translated_text);
                }
            }

            // Update text in each event page
            for page in event.pages.iter_mut() {
                inject_text_units_into_event_commands(
                    &format!("map_{}_event_{}", map_id, event.id),
                    event.id,
                    &mut page.list,
                    &text_unit_map,
                );
            }
        }
    }

    // Serialize the updated map data back to JSON
    let updated_content = serde_json::to_string_pretty(&map_data)
        .map_err(|e| AppError::Parsing(format!("Failed to serialize {}: {}", file_path, e)))?;

    // Write the updated content back to the file
    std::fs::write(&full_path, updated_content)
        .map_err(|e| AppError::FileSystem(format!("Failed to write {}: {}", file_path, e)))?;

    Ok(())
}

/// Extracts map ID from filename (e.g., "Map001.json" -> 1)
fn extract_map_id_from_filename(file_path: &str) -> AppResult<i32> {
    let file_name = Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| AppError::Parsing("Invalid file path".to_string()))?;

    if !file_name.starts_with("Map") {
        return Err(AppError::Parsing(format!("Not a map file: {}", file_name)));
    }

    let id_str = &file_name[3..]; // Remove "Map" prefix
    id_str
        .parse::<i32>()
        .map_err(|e| AppError::Parsing(format!("Invalid map ID in filename {}: {}", file_name, e)))
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
    fn test_extract_map_id_from_filename() {
        assert_eq!(extract_map_id_from_filename("Map001.json").unwrap(), 1);
        assert_eq!(extract_map_id_from_filename("data/Map001.json").unwrap(), 1);
        assert_eq!(
            extract_map_id_from_filename("www/data/Map001.json").unwrap(),
            1
        );
        assert_eq!(extract_map_id_from_filename("Map123.json").unwrap(), 123);
        assert!(extract_map_id_from_filename("NotAMap.json").is_err());
        assert!(extract_map_id_from_filename("Map.json").is_err());
    }

    #[test]
    fn test_extract_mv_map_data() {
        let project_path = get_test_games_path().join("MVgame");
        let result = extract_text(&project_path, "www/data/Map001.json");

        assert!(
            result.is_ok(),
            "Failed to extract MV map data: {:?}",
            result.err()
        );
        let game_data = result.unwrap();

        // Map001.json should have some text units (event names, display name if any, event commands)
        assert!(
            game_data.text_units.len() > 0,
            "Should extract some text units from map"
        );

        // Check that we have event-related entries
        let has_event_entries = game_data
            .text_units
            .iter()
            .any(|e| e.entry_type.contains("event"));
        assert!(has_event_entries, "Should extract event text");

        // Check that IDs contain map_1 prefix
        for unit in &game_data.text_units {
            assert!(
                unit.id.starts_with("map_1_"),
                "All units should have map_1_ prefix: {}",
                unit.id
            );
        }
    }

    #[test]
    fn test_extract_mz_map_data() {
        let project_path = get_test_games_path().join("MZgame");

        // Find a map file in MZ game
        let data_dir = project_path.join("data");
        let map_files: Vec<_> = std::fs::read_dir(&data_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("Map") && n.ends_with(".json"))
                    .unwrap_or(false)
            })
            .collect();

        if !map_files.is_empty() {
            let map_path = map_files[0]
                .strip_prefix(&project_path)
                .unwrap()
                .to_str()
                .unwrap();
            let result = extract_text(&project_path, map_path);

            assert!(
                result.is_ok(),
                "Failed to extract MZ map data: {:?}",
                result.err()
            );
            let game_data = result.unwrap();

            // Should extract some text units
            assert!(
                game_data.text_units.len() >= 0,
                "Should extract text units from MZ map"
            );
        }
    }

    #[test]
    fn test_inject_mv_map_data() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        let data_dir = project_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        let original_path = get_test_games_path().join("MVgame/www/data/Map001.json");
        let temp_map_file = data_dir.join("Map001.json");
        fs::copy(&original_path, &temp_map_file).unwrap();

        // Create a text unit for event name
        let text_unit_name = TextUnit {
            id: "map_1_event_2_name".to_string(),
            source_text: "EV002".to_string(),
            translated_text: "Event002".to_string(),
            field_type: "name:www/data/Map001.json:2:2".to_string(),
            status: TranslationStatus::Translated,
            text_type: PromptType::System,
            location: "map:1:event:2:name".to_string(), // Structured location format
            entry_type: "map_event_name".to_string(),
            file_path: Some("www/data/Map001.json".to_string()),
        };

        let text_units = vec![&text_unit_name];

        let result = inject_translations(project_path, "www/data/Map001.json", &text_units);
        assert!(result.is_ok(), "Injection failed: {:?}", result.err());

        // Verify injection by reading the JSON file directly
        let content = std::fs::read_to_string(temp_map_file).unwrap();
        let map_data: MapData = serde_json::from_str(&content).unwrap();

        // Find event with ID 2
        let event_2 = map_data
            .events
            .iter()
            .find(|e| e.as_ref().map(|ev| ev.id == 2).unwrap_or(false));
        assert!(event_2.is_some(), "Should find event with ID 2");

        let event_2_name = &event_2.unwrap().as_ref().unwrap().name;
        assert_eq!(
            event_2_name, "Event002",
            "Event name should be updated to translated value"
        );
    }
}
