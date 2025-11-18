// Centralized handler for Wolf RPG file extraction and injection
// Orchestrates parsing of mps/ directory only

use crate::parsers::engine::{TextEntry, TextUnit, TranslationEntry};
use crate::parsers::wolfrpg::files::{db, mps};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Extract all texts from Wolf RPG project
pub fn extract_all_texts(game_path: &Path) -> Result<Vec<TextEntry>, String> {
    let mut all_texts = Vec::new();
    let dump_root = game_path.join("dump");

    // Extract from database files (DataBase.json only for now)
    
    let db_dir = dump_root.join("db");
    if db_dir.exists() {
        for db_file in ["DataBase.json"/* , "CDataBase.json", "SysDatabase.json"*/] {
            let db_path = db_dir.join(db_file);
            if db_path.exists() {
                let content = fs::read_to_string(&db_path)
                    .map_err(|e| format!("Erreur lecture {}: {}", db_file, e))?;
                let json: serde_json::Value = serde_json::from_str(&content)
                    .map_err(|e| format!("Erreur parsing {}: {}", db_file, e))?;

                let relative_path = format!("dump/db/{}", db_file);
                let texts = db::extract_text_units_from_db(&json, &relative_path);

                // Convert TextUnit to TextEntry
                for unit in texts {
                    all_texts.push(TextEntry {
                        id: unit.id.clone(),
                        source_text: unit.source_text.clone(),
                        translated_text: unit.translated_text.clone(),
                        field_type: unit.field_type.clone(),
                        status: unit.status.clone(),
                        text_type: unit.text_type.clone(),
                        location: unit.location.clone(),
                        entry_type: unit.entry_type.clone(),
                        file_path: Some(relative_path.clone()),
                    });
                }
            }
        }
    }

    // Extract from map files (mps/)
    let mps_dir = dump_root.join("mps");
    if mps_dir.exists() {
        for entry in fs::read_dir(&mps_dir).map_err(|e| format!("Erreur lecture mps/: {}", e))? {
            let entry = entry.map_err(|e| format!("Erreur entrée mps/: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Erreur lecture {:?}: {}", path, e))?;
                let json: serde_json::Value = serde_json::from_str(&content)
                    .map_err(|e| format!("Erreur parsing {:?}: {}", path, e))?;

                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                let relative_path = format!("dump/mps/{}", file_name);
                let texts = mps::extract_text_units_from_mps(&json, &relative_path);

                // Convert TextUnit to TextEntry
                for unit in texts {
                    all_texts.push(TextEntry {
                        id: unit.id.clone(),
                        source_text: unit.source_text.clone(),
                        translated_text: unit.translated_text.clone(),
                        field_type: unit.field_type.clone(),
                        status: unit.status.clone(),
                        text_type: unit.text_type.clone(),
                        location: unit.location.clone(),
                        entry_type: unit.entry_type.clone(),
                        file_path: Some(relative_path.clone()),
                    });
                }
            }
        }
    }

    // Extract from common event files (common/)
    // COMMENTED OUT: Common events extraction temporarily disabled
    /*
    let common_dir = dump_root.join("common");
    if common_dir.exists() {
        for entry in fs::read_dir(&common_dir)
            .map_err(|e| format!("Erreur lecture common/: {}", e))? {
            let entry = entry.map_err(|e| format!("Erreur entrée common/: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Erreur lecture {:?}: {}", path, e))?;
                let json: serde_json::Value = serde_json::from_str(&content)
                    .map_err(|e| format!("Erreur parsing {:?}: {}", path, e))?;

                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                let relative_path = format!("dump/common/{}", file_name);
                let texts = common::extract_text_units_from_common(&json, &relative_path);

                // Convert TextUnit to TextEntry
                for unit in texts {
                    all_texts.push(TextEntry {
                        id: unit.id.clone(),
                        source_text: unit.source_text.clone(),
                        translated_text: unit.translated_text.clone(),
                        field_type: unit.field_type.clone(),
                        status: unit.status.clone(),
                        text_type: unit.text_type.clone(),
                        location: unit.location.clone(),
                        entry_type: unit.entry_type.clone(),
                        file_path: Some(relative_path.clone()),
                    });
                }
            }
        }
    }
    */

    Ok(all_texts)
}

/// Inject all translations into Wolf RPG project
pub fn inject_all_texts(game_path: &Path, translations: &[TranslationEntry]) -> Result<(), String> {
    // Build HashMap for quick lookup by id
    // We need owned values first, then create references
    let mut text_units_map: HashMap<String, TextUnit> = HashMap::new();
    for t in translations {
        text_units_map.insert(
            t.id.clone(),
            TextUnit {
                id: t.id.clone(),
                source_text: String::new(),
                translated_text: t.translated_text.clone(),
                field_type: String::new(),
                status: crate::parsers::engine::TranslationStatus::Translated,
                text_type: crate::parsers::engine::PromptType::Other,
                location: String::new(),
                entry_type: String::new(),
                file_path: None,
            },
        );
    }

    let text_units_refs: HashMap<String, &TextUnit> =
        text_units_map.iter().map(|(k, v)| (k.clone(), v)).collect();

    let dump_root = game_path.join("dump");
    
        // Inject into database files
        let db_dir = dump_root.join("db");
        if db_dir.exists() {
            for db_file in ["DataBase.json"/* , "CDataBase.json", "SysDatabase.json"*/] {
                let db_path = db_dir.join(db_file);
                if db_path.exists() {
                    let content = fs::read_to_string(&db_path)
                        .map_err(|e| format!("Erreur lecture {}: {}", db_file, e))?;
                    let mut json: serde_json::Value = serde_json::from_str(&content)
                        .map_err(|e| format!("Erreur parsing {}: {}", db_file, e))?;

                    let relative_path = format!("dump/db/{}", db_file);
                    db::inject_text_units_into_db(&mut json, &text_units_refs, &relative_path);

                    // Write back to file
                    let updated_content = serde_json::to_string_pretty(&json)
                        .map_err(|e| format!("Erreur sérialisation {}: {}", db_file, e))?;
                    fs::write(&db_path, updated_content)
                        .map_err(|e| format!("Erreur écriture {}: {}", db_file, e))?;
                }
            }
        }
    
    // Inject into map files (mps/)
    let mps_dir = dump_root.join("mps");
    if mps_dir.exists() {
        for entry in fs::read_dir(&mps_dir).map_err(|e| format!("Erreur lecture mps/: {}", e))? {
            let entry = entry.map_err(|e| format!("Erreur entrée mps/: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Erreur lecture {:?}: {}", path, e))?;
                let mut json: serde_json::Value = serde_json::from_str(&content)
                    .map_err(|e| format!("Erreur parsing {:?}: {}", path, e))?;

                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                let relative_path = format!("dump/mps/{}", file_name);
                mps::inject_text_units_into_mps(&mut json, &text_units_refs, &relative_path);

                // Write back to file
                let updated_content = serde_json::to_string_pretty(&json)
                    .map_err(|e| format!("Erreur sérialisation {:?}: {}", path, e))?;
                fs::write(&path, updated_content)
                    .map_err(|e| format!("Erreur écriture {:?}: {}", path, e))?;
            }
        }
    }
    /*
        // Inject into common event files (common/)
        let common_dir = dump_root.join("common");
        if common_dir.exists() {
            for entry in fs::read_dir(&common_dir)
                .map_err(|e| format!("Erreur lecture common/: {}", e))? {
                let entry = entry.map_err(|e| format!("Erreur entrée common/: {}", e))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Erreur lecture {:?}: {}", path, e))?;
                    let mut json: serde_json::Value = serde_json::from_str(&content)
                        .map_err(|e| format!("Erreur parsing {:?}: {}", path, e))?;

                    let file_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    let relative_path = format!("dump/common/{}", file_name);
                    common::inject_text_units_into_common(&mut json, &text_units_refs, &relative_path);

                    // Write back to file
                    let updated_content = serde_json::to_string_pretty(&json)
                        .map_err(|e| format!("Erreur sérialisation {:?}: {}", path, e))?;
                    fs::write(&path, updated_content)
                        .map_err(|e| format!("Erreur écriture {:?}: {}", path, e))?;
                }
            }
        }
    */
    Ok(())
}
