// RPG Maker files extraction handler
// Centralizes all file extraction and injection logic for RPG Maker projects

use crate::parsers::engine::{GameEngine, TextEntry, TextUnit, TranslationEntry};
use std::path::Path;

/// Extract all translatable texts from supported RPG Maker files
///
/// # Arguments
/// * `game_path` - Path to the game project directory
/// * `version` - RPG Maker version (MV or MZ)
///
/// # Returns
/// * `Result<Vec<TextEntry>, String>` - All extracted text entries or error
pub fn extract_all_texts(game_path: &Path, version: GameEngine) -> Result<Vec<TextEntry>, String> {
    let mut all_entries = Vec::new();

    // Get the correct data prefix based on version
    let data_prefix = match version {
        GameEngine::RpgMakerMZ => "data/",
        GameEngine::RpgMakerMV => "www/data/",
    };

    // Extract from each supported file
    // Actors.json
    extract_from_file(
        game_path,
        data_prefix,
        "Actors.json",
        "actor_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;
/* 
    // CommonEvents.json
    extract_from_file(
        game_path,
        data_prefix,
        "CommonEvents.json",
        "common_event_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // Classes.json
    extract_from_file(
        game_path,
        data_prefix,
        "Classes.json",
        "class_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // Weapons.json
    extract_from_file(
        game_path,
        data_prefix,
        "Weapons.json",
        "weapon_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // Items.json
    extract_from_file(
        game_path,
        data_prefix,
        "Items.json",
        "item_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // Armors.json
    extract_from_file(
        game_path,
        data_prefix,
        "Armors.json",
        "armor_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // Enemies.json
    extract_from_file(
        game_path,
        data_prefix,
        "Enemies.json",
        "enemy_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // Skills.json
    extract_from_file(
        game_path,
        data_prefix,
        "Skills.json",
        "skill_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // States.json
    extract_from_file(
        game_path,
        data_prefix,
        "States.json",
        "state_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // Troops.json
    extract_from_file(
        game_path,
        data_prefix,
        "Troops.json",
        "troop_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // MapInfos.json
    extract_from_file(
        game_path,
        data_prefix,
        "MapInfos.json",
        "map_info_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;

    // MapXXX.json files (automatically discover and process all map files)
    extract_map_data_files(game_path, data_prefix, &mut all_entries)?;

    // System.json
    extract_from_file(
        game_path,
        data_prefix,
        "System.json",
        "system_text_unit",
        |unit| unit.location.clone(),  // Use structured location format
        &mut all_entries,
    )?;*/

    Ok(all_entries)
}

/// Extract text from all MapXXX.json files
fn extract_map_data_files(
    game_path: &Path,
    data_prefix: &str,
    all_entries: &mut Vec<TextEntry>,
) -> Result<(), String> {
    let data_dir = game_path.join(data_prefix);

    if !data_dir.exists() {
        return Ok(());
    }

    // Find all MapXXX.json files (excluding MapInfos.json)
    let map_files: Vec<_> = std::fs::read_dir(&data_dir)
        .map_err(|e| format!("Failed to read data directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| {
                    // Must start with "Map", end with ".json", and NOT be "MapInfos.json"
                    n.starts_with("Map")
                        && n.ends_with(".json")
                        && n != "MapInfos.json"
                        && n.len() > 7 // MapXXX.json minimum (Map001.json = 11 chars)
                })
                .unwrap_or(false)
        })
        .collect();

    log::info!("Found {} map files to process", map_files.len());

    for map_path in map_files {
        let relative_path = map_path
            .strip_prefix(game_path)
            .map_err(|e| format!("Failed to get relative path: {}", e))?
            .to_str()
            .ok_or("Invalid path encoding")?;

        let game_data =
            crate::parsers::rpg_maker::files::map_data::extract_text(game_path, relative_path);
        match game_data {
            Ok(data) => {
                log::info!(
                    "Extracted {} text units from {}",
                    data.text_unit_count,
                    relative_path
                );

                // Convert TextUnit to TextEntry for compatibility
                for unit in data.text_units {
                    all_entries.push(TextEntry {
                        id: unit.id.clone(),
                        source_text: unit.source_text.clone(),
                        translated_text: unit.translated_text.clone(),
                        field_type: unit.field_type.clone(),
                        status: unit.status.clone(),
                        text_type: unit.text_type.clone(),
                        location: unit.location.clone(),
                        entry_type: "map_data_text_unit".to_string(),
                        file_path: Some(relative_path.to_string()),
                    });
                }
            }
            Err(e) => {
                log::warn!("Failed to extract from {}: {}", relative_path, e);
                // Continue with other files instead of failing completely
            }
        }
    }

    Ok(())
}

/// Inject translations into all MapXXX.json files
fn inject_map_data_files(
    game_path: &Path,
    data_prefix: &str,
    translations: &[TranslationEntry],
) -> Result<(), String> {
    let data_dir = game_path.join(data_prefix);

    if !data_dir.exists() {
        return Ok(());
    }

    // Find all MapXXX.json files (excluding MapInfos.json)
    let map_files: Vec<_> = std::fs::read_dir(&data_dir)
        .map_err(|e| format!("Failed to read data directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| {
                    // Must start with "Map", end with ".json", and NOT be "MapInfos.json"
                    n.starts_with("Map")
                        && n.ends_with(".json")
                        && n != "MapInfos.json"
                        && n.len() > 7 // MapXXX.json minimum (Map001.json = 11 chars)
                })
                .unwrap_or(false)
        })
        .collect();

    log::info!("Found {} map files for injection", map_files.len());

    for map_path in map_files {
        let relative_path = map_path
            .strip_prefix(game_path)
            .map_err(|e| format!("Failed to get relative path: {}", e))?
            .to_str()
            .ok_or("Invalid path encoding")?;

        // Extract map ID from filename
        let file_name = map_path
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?;
        let map_id_str = &file_name[3..]; // Remove "Map" prefix
        let map_id: i32 = map_id_str
            .parse()
            .map_err(|e| format!("Invalid map ID {}: {}", map_id_str, e))?;

        // Filter translations for this specific map
        let file_translations: Vec<&TranslationEntry> = translations
            .iter()
            .filter(|t| t.id.starts_with(&format!("map_{}_", map_id)))
            .collect();

        if !file_translations.is_empty() {
            log::info!(
                "Injecting {} translations into {}",
                file_translations.len(),
                relative_path
            );
            let text_units: Vec<_> = file_translations
                .into_iter()
                .map(|t| crate::parsers::engine::TextUnit {
                    id: t.id.clone(),
                    source_text: String::new(),
                    translated_text: t.translated_text.clone(),
                    field_type: String::new(),
                    status: crate::parsers::engine::TranslationStatus::Translated,
                    text_type: crate::parsers::engine::PromptType::Dialogue,
                    location: String::new(),
                    entry_type: String::new(),
                    file_path: None,
                })
                .collect();

            let text_unit_refs: Vec<_> = text_units.iter().collect();
            crate::parsers::rpg_maker::files::map_data::inject_translations(
                game_path,
                relative_path,
                &text_unit_refs,
            )
            .map_err(|e| format!("Injection failed for {}: {}", relative_path, e))?;
        }
    }

    Ok(())
}

/// Helper function to extract from a specific file
fn extract_from_file<F>(
    game_path: &Path,
    data_prefix: &str,
    file_name: &str,
    entry_type: &str,
    context_fn: F,
    all_entries: &mut Vec<TextEntry>,
) -> Result<(), String>
where
    F: Fn(&TextUnit) -> String,
{
    let relative_path = format!("{}{}", data_prefix, file_name);
    let full_path = game_path.join(&relative_path);

    if full_path.exists() {
        let game_data = match file_name {
            "Actors.json" => {
                crate::parsers::rpg_maker::files::actors::extract_text(game_path, &relative_path)
            }
            "Classes.json" => {
                crate::parsers::rpg_maker::files::classes::extract_text(game_path, &relative_path)
            }
            "Weapons.json" => {
                crate::parsers::rpg_maker::files::weapons::extract_text(game_path, &relative_path)
            }
            "Items.json" => {
                crate::parsers::rpg_maker::files::items::extract_text(game_path, &relative_path)
            }
            "Armors.json" => {
                crate::parsers::rpg_maker::files::armors::extract_text(game_path, &relative_path)
            }
            "Enemies.json" => {
                crate::parsers::rpg_maker::files::enemies::extract_text(game_path, &relative_path)
            }
            "Skills.json" => {
                crate::parsers::rpg_maker::files::skills::extract_text(game_path, &relative_path)
            }
            "States.json" => {
                crate::parsers::rpg_maker::files::states::extract_text(game_path, &relative_path)
            }
            "Troops.json" => {
                crate::parsers::rpg_maker::files::troops::extract_text(game_path, &relative_path)
            }
            "CommonEvents.json" => crate::parsers::rpg_maker::files::common_events::extract_text(
                game_path,
                &relative_path,
            ),
            "MapInfos.json" => {
                crate::parsers::rpg_maker::files::map_infos::extract_text(game_path, &relative_path)
            }
            "System.json" => {
                crate::parsers::rpg_maker::files::system::extract_text(game_path, &relative_path)
            }
            _ => return Ok(()), // Skip unknown files
        };

        match game_data {
            Ok(data) => {
                // Convert TextUnit to TextEntry for compatibility
                for unit in data.text_units {
                    all_entries.push(TextEntry {
                        id: unit.id.clone(),
                        source_text: unit.source_text.clone(),
                        translated_text: unit.translated_text.clone(),
                        field_type: unit.field_type.clone(),
                        status: unit.status.clone(),
                        text_type: unit.text_type.clone(),
                        location: context_fn(&unit),
                        entry_type: entry_type.to_string(),
                        file_path: Some(relative_path.clone()),
                    });
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to extract from {}: {}", file_name, e);
                // Continue with other files instead of failing completely
            }
        }
    } else {
        eprintln!(
            "Warning: {} not found at '{}', skipping",
            file_name,
            full_path.display()
        );
    }

    Ok(())
}

/// Inject all translated texts into supported RPG Maker files
///
/// # Arguments
/// * `game_path` - Path to the game project directory
/// * `version` - RPG Maker version (MV or MZ)
/// * `translations` - Vector of translation entries to inject
///
/// # Returns
/// * `Result<(), String>` - Success or error
pub fn inject_all_texts(
    game_path: &Path,
    version: GameEngine,
    translations: &[TranslationEntry],
) -> Result<(), String> {
    // Get the correct data prefix based on version
    let data_prefix = match version {
        GameEngine::RpgMakerMZ => "data/",
        GameEngine::RpgMakerMV => "www/data/",
    };

    // Inject into each supported file
    // Actors.json
    inject_into_file(game_path, data_prefix, "Actors.json", translations)?;

    // CommonEvents.json
    inject_into_file(game_path, data_prefix, "CommonEvents.json", translations)?;

    // Classes.json
    inject_into_file(game_path, data_prefix, "Classes.json", translations)?;

    // Weapons.json
    inject_into_file(game_path, data_prefix, "Weapons.json", translations)?;

    // Items.json
    inject_into_file(game_path, data_prefix, "Items.json", translations)?;

    // Armors.json
    inject_into_file(game_path, data_prefix, "Armors.json", translations)?;

    // Enemies.json
    inject_into_file(game_path, data_prefix, "Enemies.json", translations)?;

    // Skills.json
    inject_into_file(game_path, data_prefix, "Skills.json", translations)?;

    // States.json
    inject_into_file(game_path, data_prefix, "States.json", translations)?;

    // Troops.json
    inject_into_file(game_path, data_prefix, "Troops.json", translations)?;

    // MapInfos.json
    inject_into_file(game_path, data_prefix, "MapInfos.json", translations)?;

    // Map data files
    inject_map_data_files(game_path, data_prefix, translations)?;

    // System.json
    inject_into_file(game_path, data_prefix, "System.json", translations)?;

    Ok(())
}

/// Helper function to inject into a specific file
fn inject_into_file(
    game_path: &Path,
    data_prefix: &str,
    file_name: &str,
    translations: &[TranslationEntry],
) -> Result<(), String> {
    let relative_path = format!("{}{}", data_prefix, file_name);
    let full_path = game_path.join(&relative_path);

    if full_path.exists() {
        // Filter translations for this specific file
        let file_translations: Vec<&TranslationEntry> = translations
            .iter()
            .filter(|t| {
                // Check if translation ID matches file pattern
                match file_name {
                    "Actors.json" => t.id.starts_with("actor_"),
                    "Classes.json" => t.id.starts_with("class_"),
                    "Weapons.json" => t.id.starts_with("weapon_"),
                    "Items.json" => t.id.starts_with("item_"),
                    "Armors.json" => t.id.starts_with("armor_"),
                    "Enemies.json" => t.id.starts_with("enemy_"),
                    "Skills.json" => t.id.starts_with("skill_"),
                    "States.json" => t.id.starts_with("state_"),
                    "Troops.json" => t.id.starts_with("troop_"),
                    "System.json" => t.id.starts_with("system_"),
                    "CommonEvents.json" => t.id.starts_with("common_event_"),
                    "MapInfos.json" => t.id.starts_with("map_info_"),
                    // Add more patterns as files are implemented
                    _ => false,
                }
            })
            .collect();

        if !file_translations.is_empty() {
            let result = match file_name {
                "Classes.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::System,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::classes::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "Weapons.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::Item,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::weapons::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "Actors.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| {
                            // Convert TranslationEntry to TextUnit for injection
                            // We need to create a minimal TextUnit for injection
                            // The actual content will be handled by the formatter
                            crate::parsers::engine::TextUnit {
                                id: t.id.clone(),
                                source_text: String::new(), // Not used for injection
                                translated_text: t.translated_text.clone(),
                                field_type: String::new(), // Not used for injection
                                status: crate::parsers::engine::TranslationStatus::Translated,
                                text_type: crate::parsers::engine::PromptType::Character,
                                location: String::new(),
                                entry_type: String::new(),
                                file_path: None,
                            }
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::actors::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "Items.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::Item,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::items::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "Armors.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::Item,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::armors::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "Enemies.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::Character,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::enemies::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "Skills.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::Skill,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::skills::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "States.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::System,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::states::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "Troops.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::System,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::troops::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "CommonEvents.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::Dialogue,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::common_events::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "MapInfos.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::System,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::map_infos::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                "System.json" => {
                    let text_units: Vec<_> = file_translations
                        .into_iter()
                        .map(|t| crate::parsers::engine::TextUnit {
                            id: t.id.clone(),
                            source_text: String::new(),
                            translated_text: t.translated_text.clone(),
                            field_type: String::new(),
                            status: crate::parsers::engine::TranslationStatus::Translated,
                            text_type: crate::parsers::engine::PromptType::System,
                            location: String::new(),
                            entry_type: String::new(),
                            file_path: None,
                        })
                        .collect();

                    let text_unit_refs: Vec<_> = text_units.iter().collect();
                    crate::parsers::rpg_maker::files::system::inject_translations(
                        game_path,
                        &relative_path,
                        &text_unit_refs,
                    )
                }
                _ => return Ok(()), // Skip unknown files
            };

            result.map_err(|e| format!("Injection failed for {}: {}", file_name, e))?;
        }
    } else {
        eprintln!(
            "Warning: {} not found at '{}', skipping injection",
            file_name,
            full_path.display()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_extract_all_texts_mz() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let data_dir = game_path.join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create test Actors.json
        let actors_data = r#"[
          null,
          {
            "id": 1,
            "name": "Hero",
            "nickname": "The Brave",
            "profile": "A young hero.",
            "classId": 1,
            "initialLevel": 1,
            "maxLevel": 99,
            "characterName": "",
            "characterIndex": 0,
            "faceName": "",
            "faceIndex": 0,
            "battlerName": "",
            "expCurve": {},
            "params": [],
            "equips": [],
            "traits": [],
            "meta": {}
          }
        ]"#;
        fs::write(data_dir.join("Actors.json"), actors_data).unwrap();

        // Create test Classes.json
        let classes_data = r#"[
          null,
          {
            "id": 1,
            "name": "Warrior",
            "expParams": [30,20,30,30],
            "traits": [],
            "learnings": [],
            "params": [[],[],[],[],[],[],[],[]],
            "note": "",
            "meta": {}
          }
        ]"#;
        fs::write(data_dir.join("Classes.json"), classes_data).unwrap();

        // Create test Weapons.json
        let weapons_data = r#"[
          null,
          {
            "id": 1,
            "name": "Sword",
            "description": "A sharp blade.",
            "iconIndex": 97,
            "animationId": 6,
            "price": 500,
            "params": [0,0,10,0,0,0,0,0],
            "traits": [],
            "meta": {},
            "note": "",
            "wtypeId": 2,
            "etypeId": 1
          }
        ]"#;
        fs::write(data_dir.join("Weapons.json"), weapons_data).unwrap();

        // Create test Items.json
        let items_data = r#"[
  null,
  {
    "id": 1,
    "name": "Health Potion",
    "description": "Restores HP.",
    "itypeId": 1,
    "price": 50,
    "params": [],
    "traits": [],
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Items.json"), items_data).unwrap();

        let result = extract_all_texts(game_path, GameEngine::RpgMakerMZ);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert!(
            entries.len() > 0,
            "Should extract texts from multiple files"
        );

        // Check that we have both actor and item entries
        let has_actor_entry = entries.iter().any(|e| e.entry_type == "actor_text_unit");
        let has_item_entry = entries.iter().any(|e| e.entry_type == "item_text_unit");

        assert!(has_actor_entry, "Should extract actor texts");
        assert!(has_item_entry, "Should extract item texts");

        // Check file paths are correct
        let actor_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.entry_type == "actor_text_unit")
            .collect();
        let item_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.entry_type == "item_text_unit")
            .collect();

        for entry in actor_entries {
            assert_eq!(entry.file_path, Some("data/Actors.json".to_string()));
        }

        for entry in item_entries {
            assert_eq!(entry.file_path, Some("data/Items.json".to_string()));
        }
    }

    #[test]
    fn test_extract_all_texts_mv() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let data_dir = game_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create test Actors.json for MV
        let actors_data = r#"[
  null,
  {
    "id": 1,
    "name": "Hero",
    "nickname": "The Brave",
    "profile": "A young hero.",
    "classId": 1,
    "initialLevel": 1,
    "maxLevel": 99,
    "characterName": "",
    "characterIndex": 0,
    "faceName": "",
    "faceIndex": 0,
    "battlerName": "",
    "expCurve": {},
    "params": [],
    "equips": [],
    "traits": [],
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Actors.json"), actors_data).unwrap();

        let result = extract_all_texts(game_path, GameEngine::RpgMakerMV);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert!(entries.len() > 0);

        // Check that file paths are correct for MV
        let actor_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.entry_type == "actor_text_unit")
            .collect();

        for entry in actor_entries {
            assert_eq!(entry.file_path, Some("www/data/Actors.json".to_string()));
        }
    }

    #[test]
    fn test_extract_all_texts_missing_files() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        // Create only data directory without files
        let data_dir = game_path.join("data");
        fs::create_dir_all(&data_dir).unwrap();

        let result = extract_all_texts(game_path, GameEngine::RpgMakerMZ);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert_eq!(
            entries.len(),
            0,
            "Should return empty vec when no files exist"
        );
    }

    #[test]
    fn test_inject_all_texts_mz() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let data_dir = game_path.join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create test Actors.json, Classes.json, Weapons.json and Items.json
        let actors_data = r#"[
  null,
  {
    "id": 1,
    "name": "Hero",
    "nickname": "The Brave",
    "profile": "A young hero.",
    "classId": 1,
    "initialLevel": 1,
    "maxLevel": 99,
    "characterName": "",
    "characterIndex": 0,
    "faceName": "",
    "faceIndex": 0,
    "battlerName": "",
    "expCurve": {},
    "params": [],
    "equips": [],
    "traits": [],
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Actors.json"), actors_data).unwrap();

        let classes_data = r#"[
  null,
  {
    "id": 1,
    "name": "Warrior",
    "expParams": [30,20,30,30],
    "traits": [],
    "learnings": [],
    "params": [[],[],[],[],[],[],[],[]],
    "note": "",
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Classes.json"), classes_data).unwrap();

        let weapons_data = r#"[
  null,
  {
    "id": 1,
    "name": "Sword",
    "description": "A sharp blade.",
    "iconIndex": 97,
    "animationId": 6,
    "price": 500,
    "params": [0,0,10,0,0,0,0,0],
    "traits": [],
    "meta": {},
    "note": "",
    "wtypeId": 2,
    "etypeId": 1
  }
]"#;
        fs::write(data_dir.join("Weapons.json"), weapons_data).unwrap();

        let items_data = r#"[
  null,
  {
    "id": 1,
    "name": "Health Potion",
    "description": "Restores HP.",
    "itypeId": 1,
    "price": 50,
    "params": [],
    "traits": [],
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Items.json"), items_data).unwrap();

        // Create translation entries
        let translations = vec![
            TranslationEntry {
                id: "actor_1_name".to_string(),
                translated_text: "Héros".to_string(),
            },
            TranslationEntry {
                id: "class_1_name".to_string(),
                translated_text: "Guerrier".to_string(),
            },
            TranslationEntry {
                id: "weapon_1_name".to_string(),
                translated_text: "Épée".to_string(),
            },
            TranslationEntry {
                id: "item_1_name".to_string(),
                translated_text: "Potion de Soin".to_string(),
            },
        ];

        let result = inject_all_texts(game_path, GameEngine::RpgMakerMZ, &translations);
        assert!(
            result.is_ok(),
            "Injection should succeed: {:?}",
            result.err()
        );

        // Verify injection by re-extracting
        let extract_result = extract_all_texts(game_path, GameEngine::RpgMakerMZ);
        assert!(extract_result.is_ok());

        let entries = extract_result.unwrap();

        // Find the updated entries
        let actor_entry = entries.iter().find(|e| e.id == "actor_1_name").unwrap();
        let class_entry = entries.iter().find(|e| e.id == "class_1_name").unwrap();
        let weapon_entry = entries.iter().find(|e| e.id == "weapon_1_name").unwrap();
        let item_entry = entries.iter().find(|e| e.id == "item_1_name").unwrap();

        assert_eq!(actor_entry.source_text, "Héros");
        assert_eq!(class_entry.source_text, "Guerrier");
        assert_eq!(weapon_entry.source_text, "Épée");
        assert_eq!(item_entry.source_text, "Potion de Soin");
    }
}
