// RPG Maker MV/MZ engine implementation
// Handles version detection and orchestrates file parsing

use crate::parsers::engine::{GameEngine, TextEntry, TranslationEntry};
use std::path::{Path, PathBuf};

/// RPG Maker engine handler
pub struct RpgMakerEngine;

impl RpgMakerEngine {
    /// Get data root directory based on version
    pub fn get_data_root(game_path: &Path, version: GameEngine) -> PathBuf {
        match version {
            GameEngine::RpgMakerMZ => game_path.join("data"),
            GameEngine::RpgMakerMV => game_path.join("www").join("data"),
        }
    }

    /// Extract all texts from game directory
    pub fn extract_all(game_path: &Path, version: GameEngine) -> Result<Vec<TextEntry>, String> {
        let data_root = Self::get_data_root(game_path, version);

        if !data_root.exists() {
            return Err(format!("Data directory not found: {:?}", data_root));
        }

        let entries = Vec::new();

        // TODO: Implement file parsing
        // - Actors.json
        // - Items.json
        // - System.json
        // - MapXXX.json
        // - CommonEvents.json
        // etc.

        Ok(entries)
    }

    /// Inject translations back into game files
    pub fn inject_all(
        game_path: &Path,
        _translations: &[TranslationEntry],
        version: GameEngine,
    ) -> Result<(), String> {
        let data_root = Self::get_data_root(game_path, version);

        if !data_root.exists() {
            return Err(format!("Data directory not found: {:?}", data_root));
        }

        // TODO: Implement file injection

        Ok(())
    }
}
