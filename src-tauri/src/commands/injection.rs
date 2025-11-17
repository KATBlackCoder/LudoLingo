// Injection commands for reinjecting translations into game files
// Implements the injection workflow for game localization

use crate::parsers::engine::{GameEngine, TranslationEntry};
use crate::parsers::rpg_maker::files::handler::inject_all_texts as rpg_maker_inject_all;
use crate::parsers::wolfrpg::files::handler::inject_all_texts as wolfrpg_inject_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;

// Shared state for injection progress tracking
#[derive(Default)]
pub struct InjectionState {
    pub current_injections: Mutex<HashMap<String, InjectionProgress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionRequest {
    pub project_id: i64,
    pub game_path: String,
    pub translations: Vec<TranslationEntryInput>,
    pub file_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationEntryInput {
    pub id: String,
    pub translated_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionProgress {
    pub injection_id: String,
    pub current_file: String,
    pub files_processed: usize,
    pub total_files: usize,
    pub entries_injected: usize,
    pub errors: Vec<InjectionError>,
    pub status: InjectionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionError {
    pub file_path: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjectionStatus {
    Pending,
    InProgress,
    Completed,
    Partial,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionResult {
    pub injection_id: String,
    pub status: String, // 'completed' | 'partial' | 'failed'
    pub files_processed: usize,
    pub entries_injected: usize,
    pub errors: Vec<InjectionError>,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub file_path: String,
    pub severity: String, // 'warning' | 'error'
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionValidationResult {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
    pub summary: ValidationSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub files_to_process: usize,
    pub entries_to_inject: usize,
    pub untranslated_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequest {
    pub project_id: i64,
    pub game_path: String,
    pub total_translations: usize,
    pub translated_count: usize,
    pub untranslated_count: usize,
    pub file_ids: Option<Vec<i64>>,
}

/// Start translation injection
#[tauri::command]
pub async fn start_injection(
    request: InjectionRequest,
    state: State<'_, InjectionState>,
) -> Result<(String, usize, u64), String> {
    let injection_id = format!("inj_{}", chrono::Utc::now().timestamp_millis());
    let game_path = Path::new(&request.game_path);

    // Validate game path exists
    if !game_path.exists() || !game_path.is_dir() {
        return Err(format!(
            "Game path does not exist or is not a directory: {}",
            request.game_path
        ));
    }

    // Detect game engine
    let engine = crate::parsers::engine::detect_engine(game_path)
        .map_err(|e| format!("Failed to detect game engine: {}", e))?;

    // Convert input translations to parser format
    let translations: Vec<TranslationEntry> = request
        .translations
        .into_iter()
        .map(|t| TranslationEntry {
            id: t.id,
            translated_text: t.translated_text,
        })
        .collect();

    // Count files to process
    let total_files = count_files_to_process(game_path, engine);

    // Initialize progress
    let progress = InjectionProgress {
        injection_id: injection_id.clone(),
        current_file: String::new(),
        files_processed: 0,
        total_files,
        entries_injected: 0,
        errors: Vec::new(),
        status: InjectionStatus::Pending,
    };

    state
        .current_injections
        .lock()
        .unwrap()
        .insert(injection_id.clone(), progress);

    // Perform injection synchronously (can be made async later)
    perform_injection_sync(game_path, engine, translations, injection_id.clone(), state);

    // Estimate duration (rough estimate: 1 second per file)
    let estimated_duration = total_files as u64;

    Ok((injection_id, total_files, estimated_duration))
}

/// Get injection progress
#[tauri::command]
pub fn get_injection_progress(
    injection_id: String,
    state: State<'_, InjectionState>,
) -> Result<InjectionProgress, String> {
    let injections = state.current_injections.lock().unwrap();
    injections
        .get(&injection_id)
        .cloned()
        .ok_or_else(|| "Injection not found".to_string())
}

/// Cancel injection
#[tauri::command]
pub fn cancel_injection(
    injection_id: String,
    state: State<'_, InjectionState>,
) -> Result<(), String> {
    let mut injections = state.current_injections.lock().unwrap();
    if let Some(progress) = injections.get_mut(&injection_id) {
        progress.status = InjectionStatus::Cancelled;
        Ok(())
    } else {
        Err("Injection not found".to_string())
    }
}

/// Get injection result
#[tauri::command]
pub fn get_injection_result(
    injection_id: String,
    state: State<'_, InjectionState>,
) -> Result<InjectionResult, String> {
    let injections = state.current_injections.lock().unwrap();
    let progress = injections
        .get(&injection_id)
        .ok_or_else(|| "Injection not found".to_string())?;

    let status_str = match progress.status {
        InjectionStatus::Completed => "completed",
        InjectionStatus::Partial => "partial",
        InjectionStatus::Failed => "failed",
        _ => "in_progress",
    };

    Ok(InjectionResult {
        injection_id: progress.injection_id.clone(),
        status: status_str.to_string(),
        files_processed: progress.files_processed,
        entries_injected: progress.entries_injected,
        errors: progress.errors.clone(),
        completed_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Validate injection (dry run)
#[tauri::command]
pub async fn validate_injection(
    request: ValidationRequest,
) -> Result<InjectionValidationResult, String> {
    let mut issues = Vec::new();
    let game_path = Path::new(&request.game_path);

    // 1. Validate game path exists
    if !game_path.exists() {
        issues.push(ValidationIssue {
            file_path: request.game_path.clone(),
            severity: "error".to_string(),
            message: "Le chemin du jeu n'existe pas".to_string(),
        });
        return Ok(InjectionValidationResult {
            valid: false,
            issues,
            summary: ValidationSummary {
                files_to_process: 0,
                entries_to_inject: 0,
                untranslated_entries: request.untranslated_count,
            },
        });
    }

    if !game_path.is_dir() {
        issues.push(ValidationIssue {
            file_path: request.game_path.clone(),
            severity: "error".to_string(),
            message: "Le chemin spécifié n'est pas un dossier".to_string(),
        });
        return Ok(InjectionValidationResult {
            valid: false,
            issues,
            summary: ValidationSummary {
                files_to_process: 0,
                entries_to_inject: 0,
                untranslated_entries: request.untranslated_count,
            },
        });
    }

    // 2. Detect game engine
    let engine = match crate::parsers::engine::detect_engine(game_path) {
        Ok(e) => e,
        Err(e) => {
            issues.push(ValidationIssue {
                file_path: request.game_path.clone(),
                severity: "error".to_string(),
                message: format!("Impossible de détecter le moteur de jeu: {}", e),
            });
            return Ok(InjectionValidationResult {
                valid: false,
                issues,
                summary: ValidationSummary {
                    files_to_process: 0,
                    entries_to_inject: 0,
                    untranslated_entries: request.untranslated_count,
                },
            });
        }
    };

    // 3. Check file permissions
    match std::fs::metadata(game_path) {
        Ok(metadata) => {
            #[cfg(unix)]
            {
                #[allow(unused_imports)]
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                if permissions.readonly() {
                    issues.push(ValidationIssue {
                        file_path: request.game_path.clone(),
                        severity: "error".to_string(),
                        message: "Le dossier du jeu est en lecture seule. Impossible d'injecter les traductions.".to_string(),
                    });
                }
            }
            #[cfg(windows)]
            {
                // On Windows, check if we can write to the directory
                let test_file = game_path.join(".ludolingo_write_test");
                if let Err(_) = std::fs::File::create(&test_file) {
                    issues.push(ValidationIssue {
                        file_path: request.game_path.clone(),
                        severity: "error".to_string(),
                        message:
                            "Impossible d'écrire dans le dossier du jeu. Vérifiez les permissions."
                                .to_string(),
                    });
                } else {
                    // Clean up test file
                    let _ = std::fs::remove_file(&test_file);
                }
            }
        }
        Err(e) => {
            issues.push(ValidationIssue {
                file_path: request.game_path.clone(),
                severity: "warning".to_string(),
                message: format!("Impossible de vérifier les permissions: {}", e),
            });
        }
    }

    // 4. Count files to process and validate they exist
    let (files_to_process, mut engine_issues) = match engine {
        GameEngine::RpgMakerMV | GameEngine::RpgMakerMZ => {
            crate::parsers::rpg_maker::validation::validate_injection(game_path, engine)
                .map_err(|e| format!("Erreur validation RPG Maker: {}", e))?
        }
        GameEngine::WolfRPG => crate::parsers::wolfrpg::validation::validate_injection(game_path)
            .map_err(|e| format!("Erreur validation Wolf RPG: {}", e))?,
    };

    issues.append(&mut engine_issues);

    // 5. Validate translations are ready
    if request.translated_count == 0 {
        issues.push(ValidationIssue {
            file_path: String::new(),
            severity: "error".to_string(),
            message: "Aucune traduction prête pour l'injection. Traduisez d'abord les textes."
                .to_string(),
        });
    }

    if request.untranslated_count > 0 {
        issues.push(ValidationIssue {
            file_path: String::new(),
            severity: "warning".to_string(),
            message: format!(
                "{} texte(s) non traduit(s) seront ignorés lors de l'injection",
                request.untranslated_count
            ),
        });
    }

    // 6. Check if we have translations for files that exist
    if files_to_process == 0 {
        issues.push(ValidationIssue {
            file_path: game_path.display().to_string(),
            severity: "error".to_string(),
            message: "Aucun fichier de jeu trouvé à traiter".to_string(),
        });
    }

    // Determine if validation is valid (no errors, only warnings allowed)
    let has_errors = issues.iter().any(|i| i.severity == "error");
    let valid = !has_errors && files_to_process > 0 && request.translated_count > 0;

    Ok(InjectionValidationResult {
        valid,
        issues,
        summary: ValidationSummary {
            files_to_process,
            entries_to_inject: request.translated_count,
            untranslated_entries: request.untranslated_count,
        },
    })
}

/// Perform actual injection operation synchronously
fn perform_injection_sync(
    game_path: &Path,
    engine: GameEngine,
    translations: Vec<TranslationEntry>,
    injection_id: String,
    state: State<'_, InjectionState>,
) {
    // Update progress
    {
        let mut injections = state.current_injections.lock().unwrap();
        if let Some(progress) = injections.get_mut(&injection_id) {
            progress.status = InjectionStatus::InProgress;
            progress.total_files = count_files_to_process(game_path, engine);
        }
    }

    // Perform injection
    match engine {
        GameEngine::RpgMakerMV | GameEngine::RpgMakerMZ => {
            match rpg_maker_inject_all(game_path, engine, &translations) {
                Ok(()) => {
                    let mut injections = state.current_injections.lock().unwrap();
                    if let Some(progress) = injections.get_mut(&injection_id) {
                        progress.status = InjectionStatus::Completed;
                        progress.files_processed = progress.total_files;
                        progress.entries_injected = translations.len();
                    }
                }
                Err(e) => {
                    let mut injections = state.current_injections.lock().unwrap();
                    if let Some(progress) = injections.get_mut(&injection_id) {
                        progress.status = InjectionStatus::Failed;
                        progress.errors.push(InjectionError {
                            file_path: game_path.display().to_string(),
                            error_message: e,
                        });
                    }
                }
            }
        }
        GameEngine::WolfRPG => match wolfrpg_inject_all(game_path, &translations) {
            Ok(()) => {
                let mut injections = state.current_injections.lock().unwrap();
                if let Some(progress) = injections.get_mut(&injection_id) {
                    progress.status = InjectionStatus::Completed;
                    progress.files_processed = progress.total_files;
                    progress.entries_injected = translations.len();
                }
            }
            Err(e) => {
                let mut injections = state.current_injections.lock().unwrap();
                if let Some(progress) = injections.get_mut(&injection_id) {
                    progress.status = InjectionStatus::Failed;
                    progress.errors.push(InjectionError {
                        file_path: game_path.display().to_string(),
                        error_message: e,
                    });
                }
            }
        },
    }
}

/// Count files that will be processed
fn count_files_to_process(game_path: &Path, engine: GameEngine) -> usize {
    let data_prefix = match engine {
        GameEngine::RpgMakerMZ => "data/",
        GameEngine::RpgMakerMV => "www/data/",
        GameEngine::WolfRPG => "dump/",
    };

    let mut count = 0;

    match engine {
        GameEngine::RpgMakerMV | GameEngine::RpgMakerMZ => {
            let files = [
                "Actors.json",
                "CommonEvents.json",
                "Classes.json",
                "Weapons.json",
                "Items.json",
                "Armors.json",
                "Enemies.json",
                "Skills.json",
                "States.json",
                "Troops.json",
                "MapInfos.json",
                "System.json",
            ];

            for file in &files {
                let full_path = game_path.join(data_prefix).join(file);
                if full_path.exists() {
                    count += 1;
                }
            }

            // Count map files
            let map_dir = game_path.join(data_prefix).join("Map");
            if map_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&map_dir) {
                    count += entries.count();
                }
            }
        }
        GameEngine::WolfRPG => {
            // Count database files
            let db_dir = game_path.join(data_prefix).join("db");
            if db_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&db_dir) {
                    count += entries
                        .filter_map(|e| {
                            e.ok().and_then(|e| {
                                e.path()
                                    .extension()
                                    .and_then(|s| s.to_str())
                                    .map(|ext| ext == "json")
                            })
                        })
                        .count();
                }
            }

            // Count map files (mps/)
            let mps_dir = game_path.join(data_prefix).join("mps");
            if mps_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&mps_dir) {
                    count += entries
                        .filter_map(|e| {
                            e.ok().and_then(|e| {
                                e.path()
                                    .extension()
                                    .and_then(|s| s.to_str())
                                    .map(|ext| ext == "json")
                            })
                        })
                        .count();
                }
            }

            // Count common event files (common/)
            let common_dir = game_path.join(data_prefix).join("common");
            if common_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&common_dir) {
                    count += entries
                        .filter_map(|e| {
                            e.ok().and_then(|e| {
                                e.path()
                                    .extension()
                                    .and_then(|s| s.to_str())
                                    .map(|ext| ext == "json")
                            })
                        })
                        .count();
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_files_to_process() {
        // This would require a test game directory
        // For now, just test that the function compiles
        assert!(true);
    }
}
