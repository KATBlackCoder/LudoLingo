// Wolf RPG Editor Handler Implementation
// Implements GameEngineHandler for Wolf RPG Editor

use crate::parsers::engine::{TextEntry, TranslationEntry};
use crate::parsers::handler::{GameEngineHandler, ValidationResult};
use crate::parsers::wolfrpg::engine::WolfRpgEngine;
use crate::parsers::wolfrpg::files::handler::{extract_all_texts, inject_all_texts};
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
        extract_all_texts(game_path)
    }

    fn inject_all_texts(
        &self,
        game_path: &Path,
        translations: &[TranslationEntry],
    ) -> Result<(), String> {
        inject_all_texts(game_path, translations)
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

