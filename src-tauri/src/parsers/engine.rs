// Parser orchestration logic
// This module coordinates parsing across different game engines

use std::path::Path;

/// Supported game engines
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEngine {
    RpgMakerMV,
    RpgMakerMZ,
    // Future engines
    // WolfRPG,
    // Baki,
}

/// Text entry extracted from game files
#[derive(Debug, Clone)]
pub struct TextEntry {
    pub id: String,
    pub source_file: String,
    pub field: String,
    pub original_text: String,
    pub context: String,
}

impl Default for TextEntry {
    fn default() -> Self {
        Self {
            id: String::new(),
            source_file: String::new(),
            field: String::new(),
            original_text: String::new(),
            context: String::new(),
        }
    }
}

/// Translation entry for injection
#[derive(Debug, Clone)]
pub struct TranslationEntry {
    pub id: String,
    pub translated_text: String,
}

/// Common trait for file parsers
pub trait FileParser {
    fn extract(&self, file_path: &Path, version: GameEngine) -> Result<Vec<TextEntry>, String>;
    fn inject(
        &self,
        file_path: &Path,
        translations: &[TranslationEntry],
        version: GameEngine,
    ) -> Result<(), String>;
}

/// Detect game engine from directory structure
pub fn detect_engine(game_path: &Path) -> Result<GameEngine, String> {
    // Check for RPG Maker MZ (package.json + data/ folder)
    let package_json = game_path.join("package.json");
    let data_folder = game_path.join("data");

    if package_json.exists() && data_folder.is_dir() {
        return Ok(GameEngine::RpgMakerMZ);
    }

    // Check for RPG Maker MV (www/data/ folder)
    let www_data_folder = game_path.join("www").join("data");
    if www_data_folder.is_dir() {
        return Ok(GameEngine::RpgMakerMV);
    }

    Err("Unknown game engine or invalid game directory".to_string())
}
