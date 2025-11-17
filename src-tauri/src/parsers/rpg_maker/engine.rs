// RPG Maker MV/MZ engine implementation
// Handles version detection and orchestrates file parsing

use crate::parsers::engine::{GameEngine, TextEntry, TranslationEntry};
use crate::parsers::rpg_maker::files::handler::{extract_all_texts, inject_all_texts};
use std::path::{Path, PathBuf};

/// RPG Maker engine handler
pub struct RpgMakerEngine;

impl RpgMakerEngine {
    /// Get data root directory based on version
    pub fn get_data_root(game_path: &Path, version: GameEngine) -> PathBuf {
        match version {
            GameEngine::RpgMakerMZ => game_path.join("data"),
            GameEngine::RpgMakerMV => game_path.join("www").join("data"),
            _ => panic!("RpgMakerEngine should only be used with RPG Maker engines"),
        }
    }

    /// Validate project structure and provide detailed error messages
    pub fn validate_project_structure(game_path: &Path, version: GameEngine) -> Result<(), String> {
        let data_root = Self::get_data_root(game_path, version);

        // Check if data directory exists
        if !data_root.exists() {
            match version {
                GameEngine::RpgMakerMZ => {
                    let has_package_json = game_path.join("package.json").exists();
                    if has_package_json {
                        return Err("Dossier RPG Maker MZ détecté (package.json présent) mais dossier 'data/' manquant.".to_string());
                    } else {
                        return Err("Structure RPG Maker MZ invalide : 'package.json' et dossier 'data/' requis.".to_string());
                    }
                }
                GameEngine::RpgMakerMV => {
                    return Err(
                        "Structure RPG Maker MV invalide : dossier 'www/data/' manquant."
                            .to_string(),
                    );
                }
                _ => {
                    return Err(
                        "RpgMakerEngine should only be used with RPG Maker engines".to_string()
                    );
                }
            }
        }

        // Check for required files
        let actors_path = data_root.join("Actors.json");
        if !actors_path.exists() {
            return Err(format!(
                "Fichier Actors.json introuvable dans '{}'. \
                Vérifiez que le dossier de jeu est complet et non corrompu.",
                data_root.display()
            ));
        }

        Ok(())
    }

    /// Extract all texts from game directory
    pub fn extract_all(game_path: &Path, version: GameEngine) -> Result<Vec<TextEntry>, String> {
        // Validate project structure first
        Self::validate_project_structure(game_path, version)?;

        // Use the centralized handler to extract from all supported files
        extract_all_texts(game_path, version)
    }

    /// Inject translations back into game files
    pub fn inject_all(
        game_path: &Path,
        translations: &[TranslationEntry],
        version: GameEngine,
    ) -> Result<(), String> {
        // Use the centralized handler to inject into all supported files
        inject_all_texts(game_path, version, translations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::engine::GameEngine;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_data_root_mz() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        let data_root = RpgMakerEngine::get_data_root(game_path, GameEngine::RpgMakerMZ);
        assert_eq!(data_root, game_path.join("data"));
    }

    #[test]
    fn test_get_data_root_mv() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        let data_root = RpgMakerEngine::get_data_root(game_path, GameEngine::RpgMakerMV);
        assert_eq!(data_root, game_path.join("www").join("data"));
    }

    #[test]
    fn test_extract_all_with_test_files() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();
        let data_dir = game_path.join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create minimal test files
        let actors_data = r#"[
  null,
  {
    "id": 1,
    "name": "Hero",
    "nickname": "The Brave",
    "profile": "A young hero.",
    "classId": 1,
    "initialLevel": 1,
    "maxLevel": 99,
    "characterName": "",
    "characterIndex": 0,
    "faceName": "",
    "faceIndex": 0,
    "battlerName": "",
    "expCurve": {},
    "params": [],
    "equips": [],
    "traits": [],
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Actors.json"), actors_data).unwrap();

        let items_data = r#"[
  null,
  {
    "id": 1,
    "name": "Health Potion",
    "description": "Restores HP.",
    "itypeId": 1,
    "price": 50,
    "params": [],
    "traits": [],
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Items.json"), items_data).unwrap();

        let result = RpgMakerEngine::extract_all(&game_path, GameEngine::RpgMakerMZ);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert!(entries.len() > 0); // Should have extracted texts

        // Check that we have actor entries (only actors for now)
        let has_actor_entry = entries.iter().any(|e| e.entry_type.starts_with("actor_"));

        assert!(has_actor_entry, "Should extract actor texts");
    }

    #[test]
    fn test_extract_all_mv_with_test_files() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        // Create www/data directory structure for MV
        let data_dir = game_path.join("www").join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create a simple Actors.json for MV
        let actors_data = r#"[
  null,
  {
    "id": 1,
    "name": "Hero",
    "nickname": "The Brave",
    "profile": "A young hero.",
    "classId": 1,
    "initialLevel": 1,
    "maxLevel": 99,
    "characterName": "",
    "characterIndex": 0,
    "faceName": "",
    "faceIndex": 0,
    "battlerName": "",
    "expCurve": {},
    "params": [],
    "equips": [],
    "traits": [],
    "meta": {}
  }
]"#;
        fs::write(data_dir.join("Actors.json"), actors_data).unwrap();

        let result = RpgMakerEngine::extract_all(&game_path, GameEngine::RpgMakerMV);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert!(entries.len() > 0); // Should have extracted texts

        // Check that we have actor entries (only actors for now)
        let has_actor_entry = entries.iter().any(|e| e.entry_type == "actor_text_unit");

        assert!(has_actor_entry, "Should extract actor texts from MV game");

        // Check that the file_path is correct for MV
        let actor_entry = entries
            .iter()
            .find(|e| e.entry_type == "actor_text_unit")
            .unwrap();
        assert_eq!(
            actor_entry.file_path,
            Some("www/data/Actors.json".to_string())
        );
    }

    #[test]
    fn test_extract_all_nonexistent_data_dir() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        let result = RpgMakerEngine::extract_all(game_path, GameEngine::RpgMakerMZ);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(
            error_msg.contains("Structure RPG Maker MZ invalide")
                || error_msg.contains("dossier 'data/' manquant")
        );
    }
}
