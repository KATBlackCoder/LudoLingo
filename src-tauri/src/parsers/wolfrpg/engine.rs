// Wolf RPG Editor engine implementation
// Handles version detection and orchestrates file parsing

use crate::parsers::engine::{TextEntry, TranslationEntry};
use crate::parsers::wolfrpg::files::handler::{extract_all_texts, inject_all_texts};
use std::path::{Path, PathBuf};

/// Wolf RPG Editor engine handler
pub struct WolfRpgEngine;

impl WolfRpgEngine {
    /// Get data root directory (dump folder)
    pub fn get_data_root(game_path: &Path) -> PathBuf {
        game_path.join("dump")
    }

    /// Validate project structure
    pub fn validate_project_structure(game_path: &Path) -> Result<(), String> {
        let dump_root = Self::get_data_root(game_path);

        // Check if dump directory exists
        if !dump_root.exists() {
            return Err("Dossier 'dump/' manquant pour Wolf RPG Editor.".to_string());
        }

        // Check for required subdirectories
        let db_dir = dump_root.join("db");
        let mps_dir = dump_root.join("mps");
        let common_dir = dump_root.join("common");

        if !db_dir.exists() || !db_dir.is_dir() {
            return Err("Dossier 'dump/db/' manquant.".to_string());
        }

        if !mps_dir.exists() || !mps_dir.is_dir() {
            return Err("Dossier 'dump/mps/' manquant.".to_string());
        }

        if !common_dir.exists() || !common_dir.is_dir() {
            return Err("Dossier 'dump/common/' manquant.".to_string());
        }

        // Check for required database files
        let database_json = db_dir.join("DataBase.json");
        if !database_json.exists() {
            return Err("Fichier 'dump/db/DataBase.json' introuvable.".to_string());
        }

        Ok(())
    }

    /// Extract all texts from game directory
    pub fn extract_all(game_path: &Path) -> Result<Vec<TextEntry>, String> {
        // Validate project structure first
        Self::validate_project_structure(game_path)?;

        // Use the centralized handler to extract from all supported files
        extract_all_texts(game_path)
    }

    /// Inject translations back into game files
    pub fn inject_all(game_path: &Path, translations: &[TranslationEntry]) -> Result<(), String> {
        // Use the centralized handler to inject into all supported files
        inject_all_texts(game_path, translations)
    }
}
