// RPG Maker Handler Implementation
// Implements GameEngineHandler for RPG Maker MV and MZ

use crate::parsers::engine::{GameEngine, TextEntry, TranslationEntry};
use crate::parsers::handler::{GameEngineHandler, ValidationResult};
use crate::parsers::rpg_maker::engine::RpgMakerEngine;
use std::path::{Path, PathBuf};

/// Handler for RPG Maker MV and MZ game engines
pub struct RpgMakerHandler {
    /// The RPG Maker version (MV or MZ)
    version: GameEngine,
}

impl RpgMakerHandler {
    /// Create a new handler for RPG Maker MZ
    pub fn new_mz() -> Self {
        Self {
            version: GameEngine::RpgMakerMZ,
        }
    }

    /// Create a new handler for RPG Maker MV
    pub fn new_mv() -> Self {
        Self {
            version: GameEngine::RpgMakerMV,
        }
    }

    /// Get the RPG Maker version
    pub fn version(&self) -> GameEngine {
        self.version
    }
}

impl GameEngineHandler for RpgMakerHandler {
    fn engine_name(&self) -> &str {
        match self.version {
            GameEngine::RpgMakerMZ => "RPG Maker MZ",
            GameEngine::RpgMakerMV => "RPG Maker MV",
            _ => unreachable!("RpgMakerHandler should only be used with RPG Maker engines"),
        }
    }

    fn validate_project_structure(&self, game_path: &Path) -> Result<ValidationResult, String> {
        match RpgMakerEngine::validate_project_structure(game_path, self.version) {
            Ok(()) => Ok(ValidationResult::valid()),
            Err(error) => Ok(ValidationResult::invalid(vec![error])),
        }
    }

    fn extract_all_texts(&self, game_path: &Path) -> Result<Vec<TextEntry>, String> {
        RpgMakerEngine::extract_all(game_path, self.version)
    }

    fn inject_all_texts(
        &self,
        game_path: &Path,
        translations: &[TranslationEntry],
    ) -> Result<(), String> {
        RpgMakerEngine::inject_all(game_path, translations, self.version)
    }

    fn count_files_to_process(&self, game_path: &Path) -> usize {
        let data_root = RpgMakerEngine::get_data_root(game_path, self.version);
        
        if !data_root.exists() {
            return 0;
        }

        let mut count = 0;

        // Count JSON files in data directory (includes MapXXX.json files)
        // RPG Maker stores all JSON files including maps directly in data/
        if let Ok(entries) = std::fs::read_dir(&data_root) {
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

        count
    }

    fn get_data_root(&self, game_path: &Path) -> PathBuf {
        RpgMakerEngine::get_data_root(game_path, self.version)
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
    fn test_engine_name_mz() {
        let handler = RpgMakerHandler::new_mz();
        assert_eq!(handler.engine_name(), "RPG Maker MZ");
    }

    #[test]
    fn test_engine_name_mv() {
        let handler = RpgMakerHandler::new_mv();
        assert_eq!(handler.engine_name(), "RPG Maker MV");
    }

    #[test]
    fn test_get_data_root_mz() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = RpgMakerHandler::new_mz();

        let data_root = handler.get_data_root(game_path);
        assert_eq!(data_root, game_path.join("data"));
    }

    #[test]
    fn test_get_data_root_mv() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = RpgMakerHandler::new_mv();

        let data_root = handler.get_data_root(game_path);
        assert_eq!(data_root, game_path.join("www").join("data"));
    }

    #[test]
    fn test_validate_project_structure_mz_valid() {
        let mz_game_path = get_test_games_path().join("MZgame");
        let handler = RpgMakerHandler::new_mz();

        let result = handler.validate_project_structure(&mz_game_path);
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_valid, "MZ game should be valid, but got errors: {:?}", validation.errors);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_validate_project_structure_mv_valid() {
        let mv_game_path = get_test_games_path().join("MVgame");
        let handler = RpgMakerHandler::new_mv();

        let result = handler.validate_project_structure(&mv_game_path);
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_valid, "MV game should be valid, but got errors: {:?}", validation.errors);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_validate_project_structure_invalid() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = RpgMakerHandler::new_mz();

        // Create invalid structure (no data directory)
        let result = handler.validate_project_structure(game_path);
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_extract_all_texts_mz() {
        let mz_game_path = get_test_games_path().join("MZgame");
        let handler = RpgMakerHandler::new_mz();

        let result = handler.extract_all_texts(&mz_game_path);
        assert!(result.is_ok(), "Extraction should succeed for MZ game");

        let entries = result.unwrap();
        assert!(entries.len() > 0, "Should extract some texts from MZ game");

        // Check that we have various types of entries
        let has_actor_entry = entries.iter().any(|e| e.entry_type == "actor_text_unit");
        let has_item_entry = entries.iter().any(|e| e.entry_type == "item_text_unit");
        let has_class_entry = entries.iter().any(|e| e.entry_type == "class_text_unit");

        assert!(has_actor_entry, "Should extract actor texts from MZ game");
        assert!(has_item_entry, "Should extract item texts from MZ game");
        assert!(has_class_entry, "Should extract class texts from MZ game");

        // Check file paths are correct for MZ (no www/ prefix)
        let actor_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.entry_type == "actor_text_unit")
            .collect();

        for entry in actor_entries {
            if let Some(ref file_path) = entry.file_path {
                assert!(!file_path.starts_with("www/"), "MZ game should not have www/ prefix in file paths");
                assert!(file_path.starts_with("data/"), "MZ game should have data/ prefix in file paths");
            }
        }
    }

    #[test]
    fn test_extract_all_texts_mv() {
        let mv_game_path = get_test_games_path().join("MVgame");
        let handler = RpgMakerHandler::new_mv();

        let result = handler.extract_all_texts(&mv_game_path);
        assert!(result.is_ok(), "Extraction should succeed for MV game");

        let entries = result.unwrap();
        assert!(entries.len() > 0, "Should extract some texts from MV game");

        // Check that we have various types of entries
        let has_actor_entry = entries.iter().any(|e| e.entry_type == "actor_text_unit");
        let has_item_entry = entries.iter().any(|e| e.entry_type == "item_text_unit");

        assert!(has_actor_entry, "Should extract actor texts from MV game");
        assert!(has_item_entry, "Should extract item texts from MV game");

        // Check file paths are correct for MV (with www/ prefix)
        let actor_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.entry_type == "actor_text_unit")
            .collect();

        for entry in actor_entries {
            if let Some(ref file_path) = entry.file_path {
                assert!(file_path.starts_with("www/"), "MV game should have www/ prefix in file paths");
                assert!(file_path.starts_with("www/data/"), "MV game should have www/data/ prefix in file paths");
            }
        }
    }

    #[test]
    fn test_inject_all_texts_mz() {
        let mz_game_path = get_test_games_path().join("MZgame");
        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_game_path = temp_dir.path().join("mz_test");

        // Copy the MZ game to temp directory to avoid modifying the original
        copy_dir_all(&mz_game_path, &temp_game_path).expect("Failed to copy MZ game");

        let handler = RpgMakerHandler::new_mz();

        // First extract to get some translation entries
        let extract_result = handler.extract_all_texts(&temp_game_path);
        assert!(extract_result.is_ok(), "Initial extraction should succeed");

        let entries = extract_result.unwrap();
        assert!(entries.len() > 0, "Should have extracted some entries");

        // Create some translation entries (just translate a few entries)
        let mut translations = Vec::new();

        // Find some entries to translate
        if let Some(actor_entry) = entries.iter().find(|e| e.entry_type == "actor_text_unit" && e.id.ends_with("_name")) {
            translations.push(TranslationEntry {
                id: actor_entry.id.clone(),
                translated_text: format!("{} (Translated)", actor_entry.source_text),
            });
        }

        if let Some(item_entry) = entries.iter().find(|e| e.entry_type == "item_text_unit" && e.id.ends_with("_name")) {
            translations.push(TranslationEntry {
                id: item_entry.id.clone(),
                translated_text: format!("{} (Translated)", item_entry.source_text),
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
        for translation in &translations {
            let translated_entry = verify_entries.iter().find(|e| e.id == translation.id);
            assert!(translated_entry.is_some(), "Should find translated entry for {}", translation.id);

            if let Some(entry) = translated_entry {
                assert_eq!(entry.source_text, translation.translated_text,
                    "Entry {} should have translated text", translation.id);
            }
        }
    }

    #[test]
    fn test_count_files_to_process_mz() {
        let mz_game_path = get_test_games_path().join("MZgame");
        let handler = RpgMakerHandler::new_mz();

        let count = handler.count_files_to_process(&mz_game_path);
        assert!(count > 0, "MZ game should have some files to process");

        // MZ game should have many JSON files
        // Let's check it's reasonable (at least 20+ files expected)
        assert!(count >= 20, "MZ game should have at least 20 JSON files, got {}", count);
    }

    #[test]
    fn test_count_files_to_process_mv() {
        let mv_game_path = get_test_games_path().join("MVgame");
        let handler = RpgMakerHandler::new_mv();

        let count = handler.count_files_to_process(&mv_game_path);
        assert!(count > 0, "MV game should have some files to process");

        // MV game should have several JSON files
        assert!(count >= 10, "MV game should have at least 10 JSON files, got {}", count);
    }

    #[test]
    fn test_count_files_to_process_nonexistent() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = RpgMakerHandler::new_mz();

        // No data directory exists
        let count = handler.count_files_to_process(game_path);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_extract_all_texts_invalid_structure() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let handler = RpgMakerHandler::new_mz();

        // No data directory - should fail validation
        let result = handler.extract_all_texts(game_path);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(
            error_msg.contains("Structure RPG Maker MZ invalide")
                || error_msg.contains("dossier 'data/' manquant")
        );
    }
}

