// Wolf RPG injection validation
// Validates files and permissions before injection

use crate::commands::injection::ValidationIssue;
use std::path::Path;

/// Validate injection for Wolf RPG projects
/// Returns (files_to_process, issues)
pub fn validate_injection(
    game_path: &Path,
) -> Result<(usize, Vec<ValidationIssue>), String> {
    let mut issues = Vec::new();
    let mut files_to_process = 0;
    let data_root = game_path.join("dump");

    // Count database files (db/)
    let db_dir = data_root.join("db");
    if db_dir.exists() {
        match std::fs::read_dir(&db_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                            files_to_process += 1;

                            match std::fs::OpenOptions::new()
                                .write(true)
                                .open(&path)
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    issues.push(ValidationIssue {
                                        file_path: path.display().to_string(),
                                        severity: "error".to_string(),
                                        message: format!("Le fichier n'est pas accessible en écriture: {}", e),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                issues.push(ValidationIssue {
                    file_path: db_dir.display().to_string(),
                    severity: "warning".to_string(),
                    message: format!("Impossible de lire le dossier db: {}", e),
                });
            }
        }
    } else {
        issues.push(ValidationIssue {
            file_path: db_dir.display().to_string(),
            severity: "warning".to_string(),
            message: "Le dossier 'dump/db/' n'existe pas".to_string(),
        });
    }

    // Count map files (mps/)
    let mps_dir = data_root.join("mps");
    if mps_dir.exists() {
        match std::fs::read_dir(&mps_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                            files_to_process += 1;

                            match std::fs::OpenOptions::new()
                                .write(true)
                                .open(&path)
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    issues.push(ValidationIssue {
                                        file_path: path.display().to_string(),
                                        severity: "error".to_string(),
                                        message: format!("Le fichier de carte n'est pas accessible en écriture: {}", e),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                issues.push(ValidationIssue {
                    file_path: mps_dir.display().to_string(),
                    severity: "warning".to_string(),
                    message: format!("Impossible de lire le dossier mps: {}", e),
                });
            }
        }
    } else {
        issues.push(ValidationIssue {
            file_path: mps_dir.display().to_string(),
            severity: "warning".to_string(),
            message: "Le dossier 'dump/mps/' n'existe pas".to_string(),
        });
    }

    // Count common event files (common/)
    let common_dir = data_root.join("common");
    if common_dir.exists() {
        match std::fs::read_dir(&common_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                            files_to_process += 1;

                            match std::fs::OpenOptions::new()
                                .write(true)
                                .open(&path)
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    issues.push(ValidationIssue {
                                        file_path: path.display().to_string(),
                                        severity: "error".to_string(),
                                        message: format!("Le fichier n'est pas accessible en écriture: {}", e),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                issues.push(ValidationIssue {
                    file_path: common_dir.display().to_string(),
                    severity: "warning".to_string(),
                    message: format!("Impossible de lire le dossier common: {}", e),
                });
            }
        }
    } else {
        issues.push(ValidationIssue {
            file_path: common_dir.display().to_string(),
            severity: "warning".to_string(),
            message: "Le dossier 'dump/common/' n'existe pas".to_string(),
        });
    }

    Ok((files_to_process, issues))
}

