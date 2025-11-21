// Scanning commands for game file analysis and text extraction
// Implements the scanning workflow for game localization

use crate::parsers::engine::TextEntry;
use crate::parsers::factory::EngineFactory;
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

    // Detect game engine and create handler
    let handler = EngineFactory::create_handler(folder_path)
        .map_err(|e| format!("Failed to detect game engine: {}", e))?;

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
    perform_scan(scan_id.clone(), request.folder_path.clone(), handler, state);

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

    // Detect game engine and create handler
    let handler = EngineFactory::create_handler(path).map_err(|e| {
        format!(
            "Structure de projet non reconnue dans '{}'. {}",
            folder_path, e
        )
    })?;

    // Extract texts using handler
    handler
        .extract_all_texts(path)
        .map_err(|e| format!("Erreur lors de l'extraction des textes : {}", e))
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
        // We need to find the game root directory by walking up the directory tree
        find_game_engine_from_file_path(path)
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

/// Find game engine by walking up the directory tree from a file path
/// Uses EngineFactory to detect the game engine, respecting the refactored architecture
fn find_game_engine_from_file_path(file_path: &Path) -> Option<String> {
    let mut current_dir = file_path.parent();

    // Walk up the directory tree and use EngineFactory to detect the game engine
    while let Some(dir) = current_dir {
        if let Ok(handler) = EngineFactory::create_handler(dir) {
            return Some(handler.engine_name().to_string());
        }
        current_dir = dir.parent();
    }
    None
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
    handler: Box<dyn crate::parsers::handler::GameEngineHandler>,
    state: State<'_, ScanState>,
) {
    let path = Path::new(&folder_path);

    match handler.extract_all_texts(path) {
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

    #[test]
    fn test_extract_texts_from_folder_rpg_maker_mv_real_game() {
        use std::env;

        // Test with real MV game from engines_past using absolute path
        let current_dir = env::current_dir().unwrap();
        let game_path = current_dir.join("../engines_past/MVgame");

        // Skip test if directory doesn't exist (engines_past might not be available in all environments)
        if !game_path.exists() {
            println!("Skipping test: MV game not found at {:?}", game_path);
            return;
        }

        let result = extract_texts_from_folder(game_path.to_string_lossy().to_string());
        assert!(result.is_ok(), "Should extract texts successfully from MV game");

        let entries = result.unwrap();
        assert!(!entries.is_empty(), "Should extract some text entries from MV game");
        // MV game should have many text entries
        assert!(entries.len() > 100, "MV game should have at least 100 text entries");
    }

    #[test]
    fn test_extract_texts_from_folder_rpg_maker_mz_real_game() {
        use std::env;

        // Test with real MZ game from engines_past using absolute path
        let current_dir = env::current_dir().unwrap();
        let game_path = current_dir.join("../engines_past/MZgame");

        // Skip test if directory doesn't exist (engines_past might not be available in all environments)
        if !game_path.exists() {
            println!("Skipping test: MZ game not found at {:?}", game_path);
            return;
        }

        let result = extract_texts_from_folder(game_path.to_string_lossy().to_string());
        assert!(result.is_ok(), "Should extract texts successfully from MZ game");

        let entries = result.unwrap();
        assert!(!entries.is_empty(), "Should extract some text entries from MZ game");
        // MZ game should have many text entries
        assert!(entries.len() > 500, "MZ game should have at least 500 text entries");
    }

    #[test]
    fn test_extract_texts_from_folder_wolfrpg_real_game() {
        use std::env;

        // Test with real WolfRPG game from engines_past using absolute path
        let current_dir = env::current_dir().unwrap();
        let game_path = current_dir.join("../engines_past/wolfrpg");

        // Skip test if directory doesn't exist (engines_past might not be available in all environments)
        if !game_path.exists() {
            println!("Skipping test: WolfRPG game not found at {:?}", game_path);
            return;
        }

        let result = extract_texts_from_folder(game_path.to_string_lossy().to_string());
        assert!(result.is_ok(), "Should extract texts successfully from WolfRPG game");

        let entries = result.unwrap();
        assert!(!entries.is_empty(), "Should extract some text entries from WolfRPG game");
        // WolfRPG should have some text entries
        assert!(entries.len() > 10, "WolfRPG game should have at least 10 text entries");
    }

    #[test]
    fn test_extract_texts_from_folder_invalid_path() {
        use std::env;

        // Test with definitely non-existent path using absolute path
        let current_dir = env::current_dir().unwrap();
        let invalid_path = current_dir.join("../engines_past/nonexistent");

        let result = extract_texts_from_folder(invalid_path.to_string_lossy().to_string());
        assert!(result.is_err(), "Should fail for non-existent game path");
    }

    #[test]
    fn test_validate_file_format_mv_game_file() {
        use std::env;

        // Test with a real file from MV game using absolute path
        let current_dir = env::current_dir().unwrap();
        let file_path = current_dir.join("../engines_past/MVgame/www/data/Actors.json");

        // Skip test if file doesn't exist (engines_past might not be available in all environments)
        if !file_path.exists() {
            println!("Skipping test: MV game file not found at {:?}", file_path);
            return;
        }

        let result = validate_file_format(file_path.to_string_lossy().to_string());
        assert!(result.is_ok(), "Should validate MV game file successfully");

        let validation = result.unwrap();
        assert!(validation.supported, "Actors.json should be supported");
        assert_eq!(validation.detected_engine, Some("RPG Maker MV".to_string()));
        assert_eq!(validation.format_details, Some("JSON format supported".to_string()));
    }

    #[test]
    fn test_validate_file_format_mz_game_file() {
        use std::env;

        // Test with a real file from MZ game using absolute path
        let current_dir = env::current_dir().unwrap();
        let file_path = current_dir.join("../engines_past/MZgame/data/Actors.json");

        // Skip test if file doesn't exist (engines_past might not be available in all environments)
        if !file_path.exists() {
            println!("Skipping test: MZ game file not found at {:?}", file_path);
            return;
        }

        let result = validate_file_format(file_path.to_string_lossy().to_string());
        assert!(result.is_ok(), "Should validate MZ game file successfully");

        let validation = result.unwrap();
        assert!(validation.supported, "Actors.json should be supported");
        assert_eq!(validation.detected_engine, Some("RPG Maker MZ".to_string()));
        assert_eq!(validation.format_details, Some("JSON format supported".to_string()));
    }

    #[test]
    fn test_validate_file_format_wolfrpg_game_file() {
        use std::env;

        // Test with a real file from WolfRPG game using absolute path
        let current_dir = env::current_dir().unwrap();
        let file_path = current_dir.join("../engines_past/wolfrpg/dump/db/actors.json");

        // Skip test if file doesn't exist (engines_past might not be available in all environments)
        if !file_path.exists() {
            println!("Skipping test: WolfRPG game file not found at {:?}", file_path);
            return;
        }

        let result = validate_file_format(file_path.to_string_lossy().to_string());
        assert!(result.is_ok(), "Should validate WolfRPG game file successfully");

        let validation = result.unwrap();
        assert!(validation.supported, "actors.json should be supported");
        assert_eq!(validation.detected_engine, Some("Wolf RPG Editor".to_string()));
        assert_eq!(validation.format_details, Some("JSON format supported".to_string()));
    }
}
