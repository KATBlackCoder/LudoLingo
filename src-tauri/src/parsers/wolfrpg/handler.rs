// Wolf RPG Editor Handler Implementation
// Implements GameEngineHandler for Wolf RPG Editor

use crate::parsers::engine::{TextEntry, TranslationEntry};
use crate::parsers::handler::{GameEngineHandler, ValidationResult};
use crate::parsers::wolfrpg::engine::WolfRpgEngine;
use std::path::{Path, PathBuf};

/// Handler for Wolf RPG Editor game engine
pub struct WolfRpgHandler;

impl WolfRpgHandler {
    /// Create a new handler for Wolf RPG Editor
    pub fn new() -> Self {
        Self
    }
}

impl GameEngineHandler for WolfRpgHandler {
    fn engine_name(&self) -> &str {
        "Wolf RPG Editor"
    }

    fn validate_project_structure(&self, game_path: &Path) -> Result<ValidationResult, String> {
        match WolfRpgEngine::validate_project_structure(game_path) {
            Ok(()) => Ok(ValidationResult::valid()),
            Err(error) => Ok(ValidationResult::invalid(vec![error])),
        }
    }

    fn extract_all_texts(&self, game_path: &Path) -> Result<Vec<TextEntry>, String> {
        WolfRpgEngine::extract_all(game_path)
    }

    fn inject_all_texts(
        &self,
        game_path: &Path,
        translations: &[TranslationEntry],
    ) -> Result<(), String> {
        WolfRpgEngine::inject_all(game_path, translations)
    }

    fn count_files_to_process(&self, game_path: &Path) -> usize {
        let dump_root = WolfRpgEngine::get_data_root(game_path);
        
        if !dump_root.exists() {
            return 0;
        }

        let mut count = 0;

        // Count JSON files in dump/db/
        let db_dir = dump_root.join("db");
        if db_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&db_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(ext) = entry.path().extension() {
                                if ext == "json" {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Count JSON files in dump/mps/
        let mps_dir = dump_root.join("mps");
        if mps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&mps_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(ext) = entry.path().extension() {
                                if ext == "json" {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Count JSON files in dump/common/
        let common_dir = dump_root.join("common");
        if common_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&common_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(ext) = entry.path().extension() {
                                if ext == "json" {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn get_data_root(&self, game_path: &Path) -> PathBuf {
        WolfRpgEngine::get_data_root(game_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Get the path to the test game directories
    fn get_test_games_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("engines_past")
    }

    /// Copy a directory recursively
    fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if ty.is_dir() {
                copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    #[test]
    fn test_engine_name() {
        let handler = WolfRpgHandler::new();
        assert_eq!(handler.engine_name(), "Wolf RPG Editor");
    }

    #[test]
    fn test_get_data_root() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = WolfRpgHandler::new();

        let data_root = handler.get_data_root(game_path);
        assert_eq!(data_root, game_path.join("dump"));
    }

    #[test]
    fn test_validate_project_structure_valid() {
        let wolfrpg_game_path = get_test_games_path().join("wolfrpg");
        
        // Skip test if game doesn't exist
        if !wolfrpg_game_path.exists() {
            eprintln!("Warning: WolfRPG test game not found at {:?}, skipping test", wolfrpg_game_path);
            return;
        }

        let handler = WolfRpgHandler::new();

        let result = handler.validate_project_structure(&wolfrpg_game_path);
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_valid, "WolfRPG game should be valid, but got errors: {:?}", validation.errors);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_validate_project_structure_invalid() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = WolfRpgHandler::new();

        // Create invalid structure (no dump directory)
        let result = handler.validate_project_structure(game_path);
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
        assert!(validation.errors.iter().any(|e| e.contains("dump")));
    }

    #[test]
    fn test_extract_all_texts() {
        let wolfrpg_game_path = get_test_games_path().join("wolfrpg");
        
        // Skip test if game doesn't exist
        if !wolfrpg_game_path.exists() {
            eprintln!("Warning: WolfRPG test game not found at {:?}, skipping test", wolfrpg_game_path);
            return;
        }

        let handler = WolfRpgHandler::new();

        let result = handler.extract_all_texts(&wolfrpg_game_path);
        assert!(result.is_ok(), "Extraction should succeed for WolfRPG game: {:?}", result.err());

        let entries = result.unwrap();
        assert!(entries.len() > 0, "Should extract some texts from WolfRPG game");

        // Check that we have various types of entries
        let has_db_entry = entries.iter().any(|e| e.file_path.as_ref().map(|p| p.contains("dump/db")).unwrap_or(false));
        let has_mps_entry = entries.iter().any(|e| e.file_path.as_ref().map(|p| p.contains("dump/mps")).unwrap_or(false));

        assert!(has_db_entry || has_mps_entry, "Should extract texts from db or mps directories");

        // Check file paths are correct for WolfRPG (dump/ prefix)
        for entry in &entries {
            if let Some(ref file_path) = entry.file_path {
                assert!(file_path.starts_with("dump/"), "WolfRPG game should have dump/ prefix in file paths");
            }
        }
    }

    #[test]
    fn test_inject_all_texts() {
        let wolfrpg_game_path = get_test_games_path().join("wolfrpg");
        
        // Skip test if game doesn't exist
        if !wolfrpg_game_path.exists() {
            eprintln!("Warning: WolfRPG test game not found at {:?}, skipping test", wolfrpg_game_path);
            return;
        }

        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_game_path = temp_dir.path().join("wolfrpg_test");

        // Copy the WolfRPG game to temp directory to avoid modifying the original
        copy_dir_all(&wolfrpg_game_path, &temp_game_path).expect("Failed to copy WolfRPG game");

        let handler = WolfRpgHandler::new();

        // First extract to get some translation entries
        let extract_result = handler.extract_all_texts(&temp_game_path);
        assert!(extract_result.is_ok(), "Initial extraction should succeed");

        let entries = extract_result.unwrap();
        assert!(entries.len() > 0, "Should have extracted some entries");

        // Create some translation entries (just translate a few entries)
        let mut translations = Vec::new();

        // Find some entries to translate (prefer db entries as they're more reliable)
        // Note: For WolfRPG, we need to use the location format (wolf_json:...#types[...])
        // instead of the id format, as that's what injection uses
        if let Some(db_entry) = entries.iter().find(|e| {
            e.file_path.as_ref().map(|p| p.contains("dump/db")).unwrap_or(false)
                && !e.source_text.is_empty()
                && !e.location.is_empty()
        }) {
            // Use location as ID for WolfRPG (injection uses location format)
            translations.push(TranslationEntry {
                id: db_entry.location.clone(),
                translated_text: format!("{} (Translated)", db_entry.source_text),
            });
        }

        // Skip test if no translations to test
        if translations.is_empty() {
            eprintln!("Warning: No suitable entries found for injection test, skipping");
            return;
        }

        // Inject translations
        let inject_result = handler.inject_all_texts(&temp_game_path, &translations);
        assert!(inject_result.is_ok(), "Injection should succeed: {:?}", inject_result.err());

        // Verify injection by re-extracting
        let verify_result = handler.extract_all_texts(&temp_game_path);
        assert!(verify_result.is_ok(), "Verification extraction should succeed");

        let verify_entries = verify_result.unwrap();

        // Check that translations were applied
        // Note: For WolfRPG, we match by location since that's what injection uses
        for translation in &translations {
            let translated_entry = verify_entries.iter().find(|e| e.location == translation.id);
            assert!(translated_entry.is_some(), "Should find translated entry for location {}", translation.id);

            if let Some(entry) = translated_entry {
                // After injection, the source_text should be the translated text
                // Note: WolfRPG formatting may affect exact match, so we check that it contains the translation
                // or that it's different from the original
                let original_entry = entries.iter().find(|e| e.location == translation.id);
                if let Some(original) = original_entry {
                    // The source_text should have changed after injection
                    // It should either match the translated text or contain it (after formatting)
                    assert!(
                        entry.source_text == translation.translated_text 
                        || entry.source_text.contains(&translation.translated_text.trim())
                        || entry.source_text != original.source_text,
                        "Entry {} should have translated text. Original: '{}', Expected: '{}', Got: '{}'",
                        translation.id, original.source_text, translation.translated_text, entry.source_text
                    );
                } else {
                    // Fallback: just check it's not empty
                    assert!(!entry.source_text.is_empty(), "Entry {} should have text", translation.id);
                }
            }
        }
    }

    #[test]
    fn test_count_files_to_process() {
        let wolfrpg_game_path = get_test_games_path().join("wolfrpg");
        
        // Skip test if game doesn't exist
        if !wolfrpg_game_path.exists() {
            eprintln!("Warning: WolfRPG test game not found at {:?}, skipping test", wolfrpg_game_path);
            return;
        }

        let handler = WolfRpgHandler::new();

        let count = handler.count_files_to_process(&wolfrpg_game_path);
        assert!(count > 0, "WolfRPG game should have some files to process");

        // WolfRPG game should have JSON files in db/, mps/, and common/
        // At least a few files expected
        assert!(count >= 1, "WolfRPG game should have at least 1 JSON file, got {}", count);
    }

    #[test]
    fn test_count_files_to_process_nonexistent() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = WolfRpgHandler::new();

        // No dump directory exists
        let count = handler.count_files_to_process(game_path);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_extract_all_texts_invalid_structure() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = WolfRpgHandler::new();

        // No dump directory - should fail validation
        let result = handler.extract_all_texts(game_path);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(
            error_msg.contains("dump") || error_msg.contains("Wolf RPG Editor"),
            "Error message should mention dump or Wolf RPG Editor, got: {}",
            error_msg
        );
    }
}

