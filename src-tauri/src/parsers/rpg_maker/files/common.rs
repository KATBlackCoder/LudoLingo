use crate::core::error::{AppError, AppResult};
use crate::parsers::engine::{PromptType, TextUnit, TranslationStatus};
use crate::parsers::text::formatter_trait::EngineFormatter;
use crate::parsers::text::rpg_maker_formatter::RpgMakerFormatter;
use crate::parsers::text::validation::ContentValidator;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a file containing game data with extracted text units
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameDataFile {
    /// Name of the file (without path)
    pub name: String,
    /// Relative path to the file
    pub path: String,
    /// List of text units extracted from this file
    pub text_units: Vec<TextUnit>,
    /// Total number of text units
    pub text_unit_count: u32,
}

/// Utility to generate TextUnits for RPG Maker objects (actor, item, etc.)
///
/// - object_type: e.g. "actor", "item"
/// - object_id: the numeric id of the object
/// - file_path: the file path string
/// - index: index in the file array
/// - fields: Vec of (field_name, value, PromptType)
///
/// Returns Vec<TextUnit> for all non-empty, non-technical values.
/// Automatically applies RPG Maker formatting to prepare text for translation.
pub fn extract_text_units_for_object(
    object_type: &str,
    object_id: i32,
    file_path: &str,
    index: usize,
    fields: Vec<(&str, &str, PromptType)>,
) -> Vec<TextUnit> {
    let mut units = Vec::new();
    for (field, value, prompt_type) in fields {
        // Skip empty values
        if value.is_empty() {
            continue;
        }

        // Apply universal validation to filter out non-translatable content
        // This filters placeholders-only text, punctuation-only text, etc.
        if !ContentValidator::validate_text(value) {
            log::debug!(
                "Skipping {} {} field '{}': '{}' (failed validation - placeholder/punctuation only or technical content)",
                object_type,
                object_id,
                field,
                value
            );
            continue;
        }

        // Apply RPG Maker formatting to prepare text for translation
        let prepared_text = RpgMakerFormatter::prepare_for_translation(value);

        log::debug!(
            "Extracting {} {} field '{}': '{}' -> '{}'",
            object_type,
            object_id,
            field,
            value,
            prepared_text
        );

        units.push(TextUnit {
            id: format!("{}_{}_{}", object_type, object_id, field),
            source_text: prepared_text, // Formatted text ready for translation
            translated_text: String::new(),
            field_type: format!("{}:{}:{}", field, file_path, index),
            status: TranslationStatus::NotTranslated,
            text_type: prompt_type,
            location: format!("{}:{}:{}", object_type, object_id, field), // Structured location format for parser_id reconstruction
            entry_type: format!("{}_{}", object_type, field),
            file_path: Some(file_path.to_string()),
        });
    }
    units
}

/// Common function to extract text from RPG Maker JSON files with object iteration.
///
/// This function handles the common pattern of:
/// 1. Parsing JSON into Vec<Option<T>>
/// 2. Iterating through objects
/// 3. Extracting text units for each object
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the JSON file
/// * `file_name` - Human-readable name for the file (e.g., "Actors.json")
/// * `parse_function` - Function to parse JSON content into objects
/// * `extract_function` - Function to extract text units from each object
///
/// # Returns
///
/// * `AppResult<GameDataFile>` - Game data file with extracted text units
pub fn extract_text_from_file_with_objects<T, P, E>(
    project_path: &Path,
    file_path: &str,
    file_name: &str,
    parse_function: P,
    extract_function: E,
) -> AppResult<GameDataFile>
where
    P: Fn(&str) -> AppResult<Vec<Option<T>>>,
    E: Fn(&T, usize, &str) -> Vec<TextUnit>,
{
    let full_path = project_path.join(file_path);
    log::debug!(
        "Extracting text from {} at: {}",
        file_name,
        full_path.display()
    );

    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "{} not found at {}",
            file_name,
            full_path.display()
        )));
    }

    // Read the JSON file
    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_name, e)))?;

    // Parse the JSON content
    let objects: Vec<Option<T>> = parse_function(&content)?;

    let mut text_units = Vec::new();

    // Process each object
    for (index, object_option) in objects.iter().enumerate() {
        if let Some(object) = object_option {
            let units = extract_function(object, index, file_path);
            text_units.extend(units);
        }
    }

    let text_unit_count = text_units.len() as u32;
    log::info!(
        "Extracted {} text units from {}",
        text_unit_count,
        file_name
    );

    let file_stem = Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or(file_name)
        .to_string();

    Ok(GameDataFile {
        name: file_stem,
        path: file_path.to_string(),
        text_units,
        text_unit_count,
    })
}

/// Common function to inject translations into RPG Maker JSON files with object iteration.
///
/// This function handles the common pattern of:
/// 1. Parsing JSON into Vec<Option<T>>
/// 2. Creating a HashMap for text unit lookup
/// 3. Updating each object with translated text
/// 4. Serializing back to JSON
///
/// # Arguments
///
/// * `project_path` - Path to the project root
/// * `file_path` - Relative path to the JSON file
/// * `file_name` - Human-readable name for the file (e.g., "Actors.json")
/// * `text_units` - Vector of translated text units to inject
/// * `parse_function` - Function to parse JSON content into objects
/// * `update_function` - Function to update each object with translations
///
/// # Returns
///
/// * `AppResult<()>` - Success or error
pub fn inject_translations_into_file_with_objects<T, P, U>(
    project_path: &Path,
    file_path: &str,
    file_name: &str,
    text_units: &[&TextUnit],
    parse_function: P,
    update_function: U,
) -> AppResult<()>
where
    P: Fn(&str) -> AppResult<Vec<Option<T>>>,
    U: Fn(&mut T, &HashMap<String, &TextUnit>),
    T: serde::Serialize,
{
    let full_path = project_path.join(file_path);
    log::debug!(
        "Injecting translations into {} at: {}",
        file_name,
        full_path.display()
    );

    // Check if the file exists
    if !full_path.exists() {
        return Err(AppError::FileSystem(format!(
            "{} not found at {}",
            file_name,
            full_path.display()
        )));
    }

    // Read the current JSON file
    let content = fs::read_to_string(&full_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to read {}: {}", file_name, e)))?;

    // Parse the JSON content
    let mut objects: Vec<Option<T>> = parse_function(&content)?;

    // Create a map of text units for quick lookup
    let text_unit_map: HashMap<String, &TextUnit> = text_units
        .iter()
        .map(|unit| (unit.id.clone(), *unit))
        .collect();

    // Update each object with translated text
    for object_option in objects.iter_mut() {
        if let Some(object) = object_option {
            update_function(object, &text_unit_map);
        }
    }

    // Serialize the updated objects back to JSON
    let updated_content = serde_json::to_string_pretty(&objects)
        .map_err(|e| AppError::Parsing(format!("Failed to serialize {}: {}", file_name, e)))?;

    // Write the updated content back to the file
    fs::write(&full_path, updated_content)
        .map_err(|e| AppError::FileSystem(format!("Failed to write {}: {}", file_name, e)))?;

    log::info!(
        "Successfully injected {} translations into {}",
        text_units.len(),
        file_name
    );
    Ok(())
}

/// Utility to inject translated text back into RPG Maker objects
///
/// - object_type: e.g. "actor", "item"
/// - object_id: the numeric id of the object
/// - text_units: HashMap of text units keyed by ID
/// - fields: Vec of (field_name, field_reference) to update
///
/// Updates the object fields with translated text if available.
/// Automatically applies RPG Maker formatting restoration.
pub fn inject_text_units_for_object(
    object_type: &str,
    object_id: i32,
    text_units: &HashMap<String, &TextUnit>,
    fields: Vec<(&str, &mut String)>,
) {
    for (field_name, field_ref) in fields {
        let unit_id = format!("{}_{}_{}", object_type, object_id, field_name);
        log::debug!("Looking for text unit with ID: {}", unit_id);

        if let Some(unit) = text_units.get(&unit_id) {
            log::debug!("Found text unit: {} -> '{}'", unit_id, unit.translated_text);
            if !unit.translated_text.is_empty() {
                // Apply RPG Maker formatting restoration to translated text
                let restored_text =
                    RpgMakerFormatter::restore_after_translation(&unit.translated_text);

                log::info!(
                    "Injecting translation: '{}' -> '{}' (restored: '{}')",
                    field_ref,
                    unit.translated_text,
                    restored_text
                );
                *field_ref = restored_text;
            } else {
                log::debug!("Text unit has empty translation, skipping");
            }
        } else {
            log::debug!("No text unit found for ID: {}", unit_id);
        }
    }
}

/// Represents a single event command for common event processing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventCommand {
    /// Command code (101 = Show Text, 401 = Message, 108 = Comment, etc.)
    pub code: i32,
    /// Indentation level
    pub indent: i32,
    /// Command parameters (array of values)
    pub parameters: Vec<serde_json::Value>,
}

/// Extracts text units from event commands
///
/// # Arguments
/// * `object_type` - Type of object (e.g., "common_event", "troop")
/// * `object_id` - ID of the object
/// * `commands` - Vector of event commands to process
/// * `_file_path` - File path for context (unused but kept for consistency)
///
/// # Returns
/// * `Vec<TextUnit>` - Vector of extracted text units
pub fn extract_text_units_from_event_commands(
    object_type: &str,
    object_id: i32,
    commands: &[EventCommand],
    _file_path: &str,
) -> Vec<TextUnit> {
    let mut text_units = Vec::new();

    for (command_index, command) in commands.iter().enumerate() {
        match command.code {
            101 => {
                // Show Text - Message window attributes
                // Parameters: [0] = window type, [1] = position, [2] = background, [3] = position type
                // No translatable text in this command
            }
            401 => {
                // Show Text - Message content
                if let Some(text_param) = command.parameters.get(0) {
                    if let Some(text) = text_param.as_str() {
                        if !text.is_empty() {
                            // Apply universal validation to filter out non-translatable content
                            if !ContentValidator::validate_text(text) {
                                log::debug!(
                                    "Skipping {} {} message {}: '{}' (failed validation)",
                                    object_type,
                                    object_id,
                                    command_index,
                                    text
                                );
                                continue;
                            }

                            // Apply RPG Maker formatting to prepare text for translation
                            let prepared_text = RpgMakerFormatter::prepare_for_translation(text);

                            text_units.push(TextUnit {
                                id: format!(
                                    "{}_{}_message_{}",
                                    object_type, object_id, command_index
                                ),
                                source_text: prepared_text, // Formatted text ready for translation
                                translated_text: String::new(),
                                field_type: format!(
                                    "message:{}:{}:{}",
                                    _file_path, object_id, command_index
                                ),
                                status: TranslationStatus::NotTranslated,
                                text_type: PromptType::Dialogue,
                                location: format!(
                                    "{}:{}:message:{}",
                                    object_type, object_id, command_index
                                ), // Structured location format: "object_type:object_id:message:command_index"
                                entry_type: "event_message".to_string(),
                                file_path: Some(_file_path.to_string()),
                            });
                        }
                    }
                }
            }
            102 => {
                // Show Choices - Choice menu options
                // Parameters: [0] = array of choice strings, [1] = cancel type, [2] = default choice, [3] = position type, [4] = background type
                if let Some(choices_param) = command.parameters.get(0) {
                    if let Some(choices_array) = choices_param.as_array() {
                        for (choice_index, choice_param) in choices_array.iter().enumerate() {
                            if let Some(choice_text) = choice_param.as_str() {
                                if !choice_text.is_empty() {
                                    // Apply universal validation to filter out non-translatable content
                                    if !ContentValidator::validate_text(choice_text) {
                                        log::debug!(
                                            "Skipping {} {} choice {} in command {}: '{}' (failed validation)",
                                            object_type,
                                            object_id,
                                            choice_index,
                                            command_index,
                                            choice_text
                                        );
                                        continue;
                                    }

                                    // Apply RPG Maker formatting to prepare text for translation
                                    let prepared_text =
                                        RpgMakerFormatter::prepare_for_translation(choice_text);

                                    text_units.push(TextUnit {
                                        id: format!(
                                            "{}_{}_choice_{}_{}",
                                            object_type, object_id, command_index, choice_index
                                        ),
                                        source_text: prepared_text, // Formatted text ready for translation
                                        translated_text: String::new(),
                                        field_type: format!(
                                            "choice:{}:{}:{}:{}",
                                            _file_path, object_id, command_index, choice_index
                                        ),
                                        status: TranslationStatus::NotTranslated,
                                        text_type: PromptType::Dialogue,
                                        location: format!(
                                            "{}:{}:choice:{}:{}",
                                            object_type, object_id, command_index, choice_index
                                        ), // Structured location format: "object_type:object_id:choice:command_index:choice_index"
                                        entry_type: "event_choice".to_string(),
                                        file_path: Some(_file_path.to_string()),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            // 108 => {
            //     // Comment - Developer comments, not player-facing text
            //     // Skipping these as they are internal documentation
            // }
            // Add more specific command codes here as needed
            // 356 => { /* Change Enemy Name */ }
            // 357 => { /* Change Enemy HP */ }
            // etc.
            _ => {
                // Skip all other command codes - they don't contain translatable text
                // This is much safer than trying to guess what might be translatable
            }
        }
    }

    text_units
}

/// Checks if content is technical and shouldn't be translated
///
/// # Arguments
/// * `content` - The content to check
///
/// # Returns
/// * `bool` - True if the content is technical and should be skipped

/// Injects translated text back into event commands
///
/// # Arguments
/// * `object_type` - Type of object (e.g., "common_event", "troop")
/// * `object_id` - ID of the object
/// * `commands` - Vector of event commands to update
/// * `text_unit_map` - HashMap of text units for lookup
///
/// Updates the command parameters with translated text if available.
pub fn inject_text_units_into_event_commands(
    object_type: &str,
    object_id: i32,
    commands: &mut [EventCommand],
    text_unit_map: &HashMap<String, &TextUnit>,
) {
    for (command_index, command) in commands.iter_mut().enumerate() {
        match command.code {
            401 => {
                // Show Text - Message content
                if let Some(text_param) = command.parameters.get_mut(0) {
                    if text_param.as_str().is_some() {
                        let unit_id =
                            format!("{}_{}_message_{}", object_type, object_id, command_index);
                        log::debug!("Looking for event command text unit with ID: {}", unit_id);

                        if let Some(text_unit) = text_unit_map.get(&unit_id) {
                            log::debug!(
                                "Found event command text unit: {} -> '{}'",
                                unit_id,
                                text_unit.translated_text
                            );
                            // Only update if translated text is not empty
                            if !text_unit.translated_text.is_empty() {
                                // Apply RPG Maker formatting restoration to translated text
                                let restored_text = RpgMakerFormatter::restore_after_translation(
                                    &text_unit.translated_text,
                                );

                                log::info!(
                                    "Injecting event command translation: '{}' -> '{}' (restored: '{}')",
                                    text_param.as_str().unwrap(),
                                    text_unit.translated_text,
                                    restored_text
                                );
                                *text_param = serde_json::Value::String(restored_text);
                            } else {
                                log::debug!(
                                    "Event command text unit has empty translation, skipping"
                                );
                            }
                        } else {
                            log::debug!("No event command text unit found for ID: {}", unit_id);
                        }
                    }
                }
            }
            102 => {
                // Show Choices - Choice menu options
                // Parameters: [0] = array of choice strings, [1] = cancel type, [2] = default choice, [3] = position type, [4] = background type
                if let Some(choices_param) = command.parameters.get_mut(0) {
                    if let Some(choices_array) = choices_param.as_array() {
                        let mut updated_choices = Vec::new();
                        for (choice_index, choice_param) in choices_array.iter().enumerate() {
                            if let Some(choice_text) = choice_param.as_str() {
                                let unit_id = format!(
                                    "{}_{}_choice_{}_{}",
                                    object_type, object_id, command_index, choice_index
                                );
                                log::debug!("Looking for choice text unit with ID: {}", unit_id);

                                if let Some(text_unit) = text_unit_map.get(&unit_id) {
                                    log::debug!(
                                        "Found choice text unit: {} -> '{}'",
                                        unit_id,
                                        text_unit.translated_text
                                    );
                                    // Only update if translated text is not empty
                                    if !text_unit.translated_text.is_empty() {
                                        // Apply RPG Maker formatting restoration to translated text
                                        let restored_text =
                                            RpgMakerFormatter::restore_after_translation(
                                                &text_unit.translated_text,
                                            );

                                        log::info!(
                                            "Injecting choice translation: '{}' -> '{}' (restored: '{}')",
                                            choice_text,
                                            text_unit.translated_text,
                                            restored_text
                                        );
                                        updated_choices
                                            .push(serde_json::Value::String(restored_text));
                                    } else {
                                        log::debug!("Choice text unit has empty translation, keeping original");
                                        updated_choices.push(choice_param.clone());
                                    }
                                } else {
                                    log::debug!("No choice text unit found for ID: {}", unit_id);
                                    updated_choices.push(choice_param.clone());
                                }
                            } else {
                                // Keep non-string parameters as-is
                                updated_choices.push(choice_param.clone());
                            }
                        }
                        // Update the choices array with translated text
                        *choices_param = serde_json::Value::Array(updated_choices);
                    }
                }
            }
            // 108 => {
            //     // Comment - Developer comments, not player-facing text
            //     // Skipping these as they are internal documentation
            // }
            // Add more specific command codes here as needed
            // 356 => { /* Change Enemy Name */ }
            // 357 => { /* Change Enemy HP */ }
            // etc.
            _ => {
                // Skip all other command codes - they don't contain translatable text
            }
        }
    }
}
