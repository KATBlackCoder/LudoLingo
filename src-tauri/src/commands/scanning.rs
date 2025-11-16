// Scanning commands for game file analysis and text extraction
// Implements the scanning workflow for game localization

use crate::parsers::engine::{detect_engine, GameEngine, TextEntry};
use crate::parsers::rpg_maker::engine::RpgMakerEngine;
use crate::parsers::wolfrpg::engine::WolfRpgEngine;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;
use tauri::State;

// Shared state for scan progress tracking
#[derive(Default)]
pub struct ScanState {
    pub current_scan: Mutex<Option<ScanProgress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub project_id: i64,
    pub folder_path: String,
    pub recursive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub scan_id: String,
    pub current_file: String,
    pub files_processed: usize,
    pub total_files: usize,
    pub entries_extracted: usize,
    pub errors: Vec<String>,
    pub status: ScanStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanStatus {
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub scan_id: String,
    pub total_files_found: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValidationResult {
    pub supported: bool,
    pub detected_engine: Option<String>,
    pub format_details: Option<String>,
}

/// Start a scan operation
#[tauri::command]
pub async fn scan_folder(
    request: ScanRequest,
    state: State<'_, ScanState>,
) -> Result<ScanResult, String> {
    let scan_id = format!("scan_{}", chrono::Utc::now().timestamp());
    let folder_path = Path::new(&request.folder_path);

    // Validate folder exists
    if !folder_path.exists() || !folder_path.is_dir() {
        return Err("Invalid folder path".to_string());
    }

    // Detect game engine
    let engine =
        detect_engine(folder_path).map_err(|e| format!("Failed to detect game engine: {}", e))?;

    // Initialize scan progress
    let progress = ScanProgress {
        scan_id: scan_id.clone(),
        current_file: String::new(),
        files_processed: 0,
        total_files: 0,
        entries_extracted: 0,
        errors: Vec::new(),
        status: ScanStatus::InProgress,
    };

    *state.current_scan.lock().unwrap() = Some(progress.clone());

    // Perform scan synchronously for now (can be made async later)
    perform_scan(scan_id.clone(), request.folder_path.clone(), engine, state);

    Ok(ScanResult {
        scan_id,
        total_files_found: 0, // Will be updated during scan
    })
}

/// Get current scan progress
#[tauri::command]
pub fn get_scan_progress(
    scan_id: String,
    state: State<'_, ScanState>,
) -> Result<ScanProgress, String> {
    if let Some(progress) = &*state.current_scan.lock().unwrap() {
        if progress.scan_id == scan_id {
            return Ok(progress.clone());
        }
    }
    Err("Scan not found".to_string())
}

/// Cancel ongoing scan
#[tauri::command]
pub fn cancel_scan(scan_id: String, state: State<'_, ScanState>) -> Result<(), String> {
    if let Some(progress) = &mut *state.current_scan.lock().unwrap() {
        if progress.scan_id == scan_id {
            // Mark as failed/cancelled
            // In a real implementation, you'd signal the background task to stop
            return Ok(());
        }
    }
    Err("Scan not found".to_string())
}

/// Extract texts from a game folder and return them
#[tauri::command]
pub fn extract_texts_from_folder(folder_path: String) -> Result<Vec<TextEntry>, String> {
    let path = Path::new(&folder_path);

    // Validate folder exists
    if !path.exists() || !path.is_dir() {
        return Err(format!(
            "Le chemin '{}' n'existe pas ou n'est pas un dossier.",
            folder_path
        ));
    }

    // Detect game engine
    let engine = detect_engine(path).map_err(|_| {
        format!(
            "Structure de projet non reconnue dans '{}'. \
            Pour RPG Maker MZ : doit contenir 'package.json' et dossier 'data/'. \
            Pour RPG Maker MV : doit contenir dossier 'www/data/'. \
            Pour Wolf RPG Editor : doit contenir dossier 'dump/' avec 'db/', 'mps/', et 'common/'.",
            folder_path
        )
    })?;

    // Extract texts
    match engine {
        GameEngine::RpgMakerMV | GameEngine::RpgMakerMZ => {
            RpgMakerEngine::extract_all(path, engine)
                .map_err(|e| format!("Erreur lors de l'extraction des textes : {}", e))
        }
        GameEngine::WolfRPG => {
            WolfRpgEngine::extract_all(path)
                .map_err(|e| format!("Erreur lors de l'extraction des textes : {}", e))
        }
    }
}

/// Validate file format compatibility
#[tauri::command]
pub fn validate_file_format(file_path: String) -> Result<FileValidationResult, String> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    // For now, only support RPG Maker formats
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let supported = matches!(extension, "json");

    let detected_engine = if supported {
        // Try to detect based on content or parent directory
        if let Some(parent) = path.parent() {
            match detect_engine(parent) {
                Ok(GameEngine::RpgMakerMZ) => Some("RPG Maker MZ".to_string()),
                Ok(GameEngine::RpgMakerMV) => Some("RPG Maker MV".to_string()),
                Ok(GameEngine::WolfRPG) => Some("Wolf RPG Editor".to_string()),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };

    Ok(FileValidationResult {
        supported,
        detected_engine,
        format_details: if supported {
            Some("JSON format supported".to_string())
        } else {
            Some(format!("Unsupported format: {}", extension))
        },
    })
}

/// Analyze scan errors and provide detailed error messages
fn analyze_scan_error(error: &str, game_path: &Path) -> String {
    // Check for common error patterns and provide user-friendly messages
    if error.contains("Data directory not found") {
        return format!(
            "Dossier de données introuvable. Vérifiez que le dossier '{}' contient bien les fichiers du jeu (data/ pour MZ, www/data/ pour MV).",
            game_path.display()
        );
    }

    if error.contains("Failed to parse JSON") {
        return format!(
            "Fichier JSON corrompu ou invalide dans '{}'. Le fichier peut être endommagé ou ne pas être un fichier de jeu valide.",
            game_path.display()
        );
    }

    if error.contains("Failed to open file") {
        return format!(
            "Impossible d'accéder aux fichiers dans '{}'. Vérifiez les permissions d'accès au dossier.",
            game_path.display()
        );
    }

    if error.contains("Unknown game engine") {
        return format!(
            "Moteur de jeu non reconnu dans '{}'. Seuls RPG Maker MV et MZ sont supportés pour le moment.",
            game_path.display()
        );
    }

    // Generic error with context
    format!(
        "Erreur lors du scan: {} (dossier: {})",
        error,
        game_path.display()
    )
}

/// Attempt to recover from common scan errors
fn attempt_error_recovery(error: &str, game_path: &Path) -> Option<String> {
    // Check if it's a missing data directory that might be in a different location
    if error.contains("Data directory not found") {
        // Try common alternative paths
        let alternatives = [
            game_path.join("www").join("data"),
            game_path.join("data"),
            game_path.join("game").join("data"),
        ];

        for alt_path in &alternatives {
            if alt_path.exists() && alt_path.is_dir() {
                return Some(format!(
                    "Suggestion: Les fichiers de données ont été trouvés dans '{}'. Essayez de scanner ce dossier directement.",
                    alt_path.display()
                ));
            }
        }
    }

    None
}

/// Perform the actual scanning operation
fn perform_scan(
    _scan_id: String,
    folder_path: String,
    engine: GameEngine,
    state: State<'_, ScanState>,
) {
    let path = Path::new(&folder_path);

    match engine {
        GameEngine::RpgMakerMV | GameEngine::RpgMakerMZ => {
            match RpgMakerEngine::extract_all(path, engine) {
                Ok(entries) => {
                    // Update progress with extracted entries count
                    let mut progress_guard = state.current_scan.lock().unwrap();
                    if let Some(progress) = &mut *progress_guard {
                        progress.entries_extracted = entries.len();
                        progress.status = ScanStatus::Completed;
                        progress.current_file = "Scan completed".to_string();
                    }
                }
                Err(e) => {
                    // Enhanced error handling for corrupted files
                    let error_details = analyze_scan_error(&e, path);
                    let mut errors = vec![error_details];

                    // Try to provide recovery suggestions
                    if let Some(recovery_suggestion) = attempt_error_recovery(&e, path) {
                        errors.push(recovery_suggestion);
                    }

                    let mut progress_guard = state.current_scan.lock().unwrap();
                    if let Some(progress) = &mut *progress_guard {
                        progress.errors = errors;
                        progress.status = ScanStatus::Failed;
                    }
                }
            }
        }
        GameEngine::WolfRPG => {
            match WolfRpgEngine::extract_all(path) {
                Ok(entries) => {
                    // Update progress with extracted entries count
                    let mut progress_guard = state.current_scan.lock().unwrap();
                    if let Some(progress) = &mut *progress_guard {
                        progress.entries_extracted = entries.len();
                        progress.status = ScanStatus::Completed;
                        progress.current_file = "Scan completed".to_string();
                    }
                }
                Err(e) => {
                    // Enhanced error handling for corrupted files
                    let error_details = analyze_scan_error(&e, path);
                    let mut errors = vec![error_details];

                    let mut progress_guard = state.current_scan.lock().unwrap();
                    if let Some(progress) = &mut *progress_guard {
                        progress.errors = errors;
                        progress.status = ScanStatus::Failed;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_file_format_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        fs::write(&file_path, "{}").unwrap();

        let result = validate_file_format(file_path.to_string_lossy().to_string()).unwrap();
        assert!(result.supported);
        assert_eq!(result.format_details.unwrap(), "JSON format supported");
    }

    #[test]
    fn test_validate_file_format_unsupported() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, "test").unwrap();

        let result = validate_file_format(file_path.to_string_lossy().to_string()).unwrap();
        assert!(!result.supported);
        assert!(result
            .format_details
            .unwrap()
            .contains("Unsupported format"));
    }

    #[test]
    fn test_validate_file_format_nonexistent() {
        let result = validate_file_format("nonexistent.json".to_string());
        assert!(result.is_err());
    }
}
