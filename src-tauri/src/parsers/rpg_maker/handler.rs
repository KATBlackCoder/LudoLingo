// RPG Maker Handler Implementation
// Implements GameEngineHandler for RPG Maker MV and MZ

use crate::parsers::engine::{GameEngine, TextEntry, TranslationEntry};
use crate::parsers::handler::{GameEngineHandler, ValidationResult};
use crate::parsers::rpg_maker::engine::RpgMakerEngine;
use crate::parsers::rpg_maker::files::handler::{extract_all_texts, inject_all_texts};
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
        extract_all_texts(game_path, self.version)
    }

    fn inject_all_texts(
        &self,
        game_path: &Path,
        translations: &[TranslationEntry],
    ) -> Result<(), String> {
        inject_all_texts(game_path, self.version, translations)
    }

    fn count_files_to_process(&self, game_path: &Path) -> usize {
        let data_root = RpgMakerEngine::get_data_root(game_path, self.version);
        
        if !data_root.exists() {
            return 0;
        }

        let mut count = 0;

        // Count JSON files in data directory
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

        // Count map files
        let maps_dir = data_root.join("Map");
        if maps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&maps_dir) {
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
        RpgMakerEngine::get_data_root(game_path, self.version)
    }
}

