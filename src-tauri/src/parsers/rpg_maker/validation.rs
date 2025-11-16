// RPG Maker injection validation
// Validates files and permissions before injection

use crate::commands::injection::ValidationIssue;
use crate::parsers::engine::GameEngine;
use std::path::Path;

/// Validate injection for RPG Maker projects
/// Returns (files_to_process, issues)
pub fn validate_injection(
    game_path: &Path,
    engine: GameEngine,
) -> Result<(usize, Vec<ValidationIssue>), String> {
    let mut issues = Vec::new();
    let mut files_to_process = 0;

    // Get the correct data prefix based on version
    let data_prefix = match engine {
        GameEngine::RpgMakerMZ => "data/",
        GameEngine::RpgMakerMV => "www/data/",
        _ => return Err("Invalid engine for RPG Maker handler".to_string()),
    };

    let data_root = game_path.join(data_prefix);
    
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
        let full_path = data_root.join(file);
        if full_path.exists() {
            files_to_process += 1;

            // Check if file is writable
            match std::fs::OpenOptions::new()
                .write(true)
                .open(&full_path)
            {
                Ok(_) => {}
                Err(e) => {
                    issues.push(ValidationIssue {
                        file_path: full_path.display().to_string(),
                        severity: "error".to_string(),
                        message: format!("Le fichier n'est pas accessible en écriture: {}", e),
                    });
                }
            }
        } else {
            issues.push(ValidationIssue {
                file_path: full_path.display().to_string(),
                severity: "warning".to_string(),
                message: format!("Le fichier '{}' n'existe pas et sera ignoré lors de l'injection", file),
            });
        }
    }

    // Count map files
    let map_dir = data_root.join("Map");
    if map_dir.exists() {
        match std::fs::read_dir(&map_dir) {
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
                    file_path: map_dir.display().to_string(),
                    severity: "warning".to_string(),
                    message: format!("Impossible de lire le dossier Map: {}", e),
                });
            }
        }
    }

    Ok((files_to_process, issues))
}

