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

/// Translation status for text entries
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TranslationStatus {
    /// Text has not been translated yet
    NotTranslated,
    /// Text has been translated
    Translated,
    /// Text should be ignored during translation (e.g., already in target language)
    Ignored,
    /// Text is currently being translated
    InProgress,
}

/// Type of text content for AI prompting
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PromptType {
    /// Character names, actor names
    Character,
    /// Dialogue text, conversations
    Dialogue,
    /// Item names, descriptions
    Item,
    /// Skill names, descriptions
    Skill,
    /// System messages, UI text
    System,
}

/// Text unit for extraction and translation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextUnit {
    /// Unique identifier for this text unit
    pub id: String,
    /// Original source text
    pub source_text: String,
    /// Translated text (empty if not translated)
    pub translated_text: String,
    /// Field type and location information
    pub field_type: String,
    /// Current translation status
    pub status: TranslationStatus,
    /// Type of content for AI prompting
    pub prompt_type: PromptType,
    /// Context information (for backward compatibility with TextEntry)
    pub context: String,
    /// Entry type (for backward compatibility with TextEntry)
    pub entry_type: String,
    /// File path (for backward compatibility with TextEntry)
    pub file_path: Option<String>,
}

impl Default for TextUnit {
    fn default() -> Self {
        Self {
            id: String::new(),
            source_text: String::new(),
            translated_text: String::new(),
            field_type: String::new(),
            status: TranslationStatus::NotTranslated,
            prompt_type: PromptType::Character,
            context: String::new(),
            entry_type: String::new(),
            file_path: None,
        }
    }
}

/// Legacy TextEntry for backward compatibility (now alias to TextUnit)
pub type TextEntry = TextUnit;

/// Translation entry for injection (legacy, kept for compatibility)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_engine_rpg_maker_mz() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        // Create package.json and data folder for MZ
        fs::write(game_path.join("package.json"), "{}").unwrap();
        fs::create_dir(game_path.join("data")).unwrap();

        let result = detect_engine(game_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameEngine::RpgMakerMZ);
    }

    #[test]
    fn test_detect_engine_rpg_maker_mv() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        // Create www/data folder for MV
        fs::create_dir_all(game_path.join("www").join("data")).unwrap();

        let result = detect_engine(game_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameEngine::RpgMakerMV);
    }

    #[test]
    fn test_detect_engine_unknown() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        // No recognizable structure
        fs::create_dir(game_path.join("some_folder")).unwrap();

        let result = detect_engine(game_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown game engine"));
    }

    #[test]
    fn test_detect_engine_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        let result = detect_engine(game_path);
        assert!(result.is_err());
    }
}
