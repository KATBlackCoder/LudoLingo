// Text processing now handled by unified pipeline
use crate::parsers::engine::{PromptType, TextUnit, TranslationStatus};
use crate::parsers::text::formatter::EngineFormatter;
use crate::parsers::text::formatter::WolfRpgFormatter;
use crate::parsers::wolfrpg::WolfRpgTextValidator;
use serde_json::Value;
use std::collections::HashMap;

/// Extract text units from Wolf RPG Common Event files
/// Structure: id, name, description, commands[] (similar to MPS but simpler)
pub fn extract_text_units_from_common(common_data: &Value, file_path: &str) -> Vec<TextUnit> {
    let mut text_units = Vec::new();

    // Common files have a single object with id, name, description, commands[]
    if let Some(obj) = common_data.as_object() {
        // Extract from commands array (similar to MPS pages but without event/page structure)
        if let Some(commands) = obj.get("commands").and_then(|v| v.as_array()) {
            for (cmd_idx, command) in commands.iter().enumerate() {
                extract_from_wolf_command(
                    &mut text_units,
                    command,
                    file_path,
                    0, // common events don't have event_idx/page_idx structure
                    0,
                    cmd_idx,
                );
            }
        }
    }

    text_units
}

/// Extract text from Wolf RPG commands (reuse logic from mps.rs)
/// Only processes known translatable command codes: 101, 210, 122
/// Note: Code 150 (Picture) is excluded as it doesn't contain translatable text
fn extract_from_wolf_command(
    text_units: &mut Vec<TextUnit>,
    command: &Value,
    file_path: &str,
    event_idx: usize,
    page_idx: usize,
    cmd_idx: usize,
) {
    if let Some(cmd_obj) = command.as_object() {
        // Get command code (like RPG Maker's command codes)
        let code = cmd_obj.get("code").and_then(|v| v.as_i64()).unwrap_or(0);

        // Extract based on specific translatable command codes only
        match code {
            101 => {
                // Message - extract all text from stringArgs
                extract_command_strings(
                    text_units,
                    cmd_obj,
                    file_path,
                    event_idx,
                    page_idx,
                    cmd_idx,
                    code,
                    PromptType::Dialogue,
                );
            }
            210 => {
                // CommonEvent - extract text but skip files (.mp3, .png, etc)
                extract_command_strings(
                    text_units,
                    cmd_obj,
                    file_path,
                    event_idx,
                    page_idx,
                    cmd_idx,
                    code,
                    PromptType::Dialogue,
                );
            }
            122 => {
                // SetString - extract text only if not empty
                extract_command_strings(
                    text_units,
                    cmd_obj,
                    file_path,
                    event_idx,
                    page_idx,
                    cmd_idx,
                    code,
                    PromptType::Other,
                );
            }
            _ => {
                // Skip all other command codes - they don't contain translatable text
            }
        }
    }
}

/// Helper function to extract strings from command stringArgs
fn extract_command_strings(
    text_units: &mut Vec<TextUnit>,
    cmd_obj: &serde_json::Map<String, Value>,
    file_path: &str,
    event_idx: usize,
    page_idx: usize,
    cmd_idx: usize,
    code: i64,
    prompt_type: PromptType,
) {
    if let Some(string_args) = cmd_obj.get("stringArgs").and_then(|v| v.as_array()) {
        for (arg_idx, arg) in string_args.iter().enumerate() {
            if let Some(arg_text) = arg.as_str() {
                // Skip empty strings for SetString (122) and others
                if arg_text.trim().is_empty() {
                    continue;
                }

                // Apply Wolf RPG-specific validation to filter out non-translatable content
                if !WolfRpgTextValidator::validate_text(arg_text) {
                    continue;
                }

                // Apply Wolf RPG formatting to prepare text for translation
                let processed_text = WolfRpgFormatter::prepare_for_translation(arg_text);
                let normalized_path = file_path.replace('\\', "/");
                text_units.push(TextUnit {
                    id: format!(
                        "wolf_json:{}#commands[{}].stringArgs[{}]",
                        normalized_path, cmd_idx, arg_idx
                    ),
                    source_text: processed_text,
                    translated_text: String::new(),
                    field_type: format!(
                        "command_{}:{}:commands[{}]",
                        code, file_path, cmd_idx
                    ),
                    status: TranslationStatus::NotTranslated,
                    text_type: prompt_type.clone(),
                    location: format!("common:{}:command:{}", normalized_path, cmd_idx),
                    entry_type: "common_event_text_unit".to_string(),
                    file_path: Some(file_path.to_string()),
                });
            }
        }
    }
}

/// Inject translated text back into Wolf RPG Common Event structures
pub fn inject_text_units_into_common(
    common_data: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
) {
    if let Some(obj) = common_data.as_object_mut() {
        // Inject into command list
        if let Some(commands) = obj.get_mut("commands").and_then(|v| v.as_array_mut()) {
            for (cmd_idx, command) in commands.iter_mut().enumerate() {
                inject_into_wolf_command(command, text_units, file_path, cmd_idx);
            }
        }
    }
}

/// Inject translations into Wolf RPG commands
fn inject_into_wolf_command(
    command: &mut Value,
    text_units: &HashMap<String, &TextUnit>,
    file_path: &str,
    cmd_idx: usize,
) {
    if let Some(cmd_obj) = command.as_object_mut() {
        // Inject into stringArgs if present
        if let Some(string_args) = cmd_obj.get_mut("stringArgs").and_then(|v| v.as_array_mut()) {
            for (arg_idx, arg) in string_args.iter_mut().enumerate() {
                let normalized_path = file_path.replace('\\', "/");
                let unit_id = format!(
                    "wolf_json:{}#commands[{}].stringArgs[{}]",
                    normalized_path, cmd_idx, arg_idx
                );
                if let Some(text_unit) = text_units.get(&unit_id) {
                    if !text_unit.translated_text.is_empty() {
                        // Restore Wolf RPG formatting after translation
                        let restored_text = WolfRpgFormatter::restore_after_translation(&text_unit.translated_text);
                        *arg = Value::String(restored_text);
                    }
                }
            }
        }
    }
}
