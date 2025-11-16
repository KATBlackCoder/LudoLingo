// Project validation commands
// Provides business logic validation for project management

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Project name validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNameValidation {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub suggestions: Vec<String>,
}

/// Game path validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePathValidation {
    pub is_valid: bool,
    pub detected_engine: Option<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Validate project name according to business rules
#[tauri::command]
pub fn validate_project_name(name: String) -> Result<ProjectNameValidation, String> {
    let mut errors = Vec::new();
    let mut suggestions = Vec::new();

    // Check if name is empty
    if name.trim().is_empty() {
        errors.push("Le nom du projet ne peut pas être vide".to_string());
        return Ok(ProjectNameValidation {
            is_valid: false,
            errors,
            suggestions,
        });
    }

    // Check length
    if name.len() < 3 {
        errors.push("Le nom du projet doit contenir au moins 3 caractères".to_string());
    }

    if name.len() > 100 {
        errors.push("Le nom du projet ne peut pas dépasser 100 caractères".to_string());
    }

    // Check for invalid characters
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        errors.push("Le nom du projet contient des caractères invalides".to_string());
        suggestions.push("Évitez les caractères spéciaux: / \\ : * ? \" < > |".to_string());
    }

    // Check for reserved names
    let reserved_names = ["CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];
    if reserved_names.contains(&name.to_uppercase().as_str()) {
        errors.push("Ce nom est réservé par le système".to_string());
    }

    // Check for potentially problematic patterns
    if name.starts_with('.') {
        errors.push("Le nom du projet ne peut pas commencer par un point".to_string());
    }

    if name.ends_with('.') {
        errors.push("Le nom du projet ne peut pas se terminer par un point".to_string());
    }

    // Generate suggestions if name is invalid
    if !errors.is_empty() {
        // Clean version of the name
        let clean_name = name
            .chars()
            .map(|c| if invalid_chars.contains(&c) { '_' } else { c })
            .collect::<String>()
            .trim()
            .to_string();

        if clean_name.len() >= 3 && clean_name != name {
            suggestions.push(format!("Suggestion: {}", clean_name));
        }

        // Add a generic suggestion
        if suggestions.is_empty() {
            suggestions.push("Utilisez uniquement des lettres, chiffres, espaces et tirets".to_string());
    }
    }

    Ok(ProjectNameValidation {
        is_valid: errors.is_empty(),
        errors,
        suggestions,
    })
}

/// Validate game path and detect engine
#[tauri::command]
pub fn validate_game_path(path: String) -> Result<GamePathValidation, String> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut detected_engine = None;

    let game_path = Path::new(&path);

    // Check if path exists
    if !game_path.exists() {
        errors.push("Le chemin spécifié n'existe pas".to_string());
        return Ok(GamePathValidation {
            is_valid: false,
            detected_engine,
            errors,
            warnings,
        });
    }

    // Check if it's a directory
    if !game_path.is_dir() {
        errors.push("Le chemin doit pointer vers un dossier de jeu".to_string());
        return Ok(GamePathValidation {
            is_valid: false,
            detected_engine,
            errors,
            warnings,
        });
    }

    // Try to detect RPG Maker engine
    detected_engine = detect_game_engine(game_path)?;

    // Engine-specific validations
    if let Some(ref engine) = detected_engine {
        match engine.as_str() {
            "RPG Maker MZ" => {
                // Check for package.json
                let package_json = game_path.join("package.json");
                if !package_json.exists() {
                    warnings.push("Fichier package.json manquant (recommandé pour RPG Maker MZ)".to_string());
                }

                // Check for data directory
                let data_dir = game_path.join("data");
                if !data_dir.exists() || !data_dir.is_dir() {
                    errors.push("Dossier 'data/' manquant pour RPG Maker MZ".to_string());
                }
            }
            "RPG Maker MV" => {
                // Check for www/data directory
                let www_dir = game_path.join("www");
                let data_dir = www_dir.join("data");

                if !www_dir.exists() || !www_dir.is_dir() {
                    errors.push("Dossier 'www/' manquant pour RPG Maker MV".to_string());
                } else if !data_dir.exists() || !data_dir.is_dir() {
                    errors.push("Dossier 'www/data/' manquant pour RPG Maker MV".to_string());
                }
            }
            "Wolf RPG Editor" => {
                // Check for dump directory with required subdirectories
                let dump_dir = game_path.join("dump");
                if !dump_dir.exists() || !dump_dir.is_dir() {
                    errors.push("Dossier 'dump/' manquant pour Wolf RPG Editor".to_string());
                } else {
                    let db_dir = dump_dir.join("db");
                    let mps_dir = dump_dir.join("mps");
                    let common_dir = dump_dir.join("common");

                    if !db_dir.exists() || !db_dir.is_dir() {
                        errors.push("Dossier 'dump/db/' manquant pour Wolf RPG Editor".to_string());
                    }
                    if !mps_dir.exists() || !mps_dir.is_dir() {
                        errors.push("Dossier 'dump/mps/' manquant pour Wolf RPG Editor".to_string());
                    }
                    if !common_dir.exists() || !common_dir.is_dir() {
                        errors.push("Dossier 'dump/common/' manquant pour Wolf RPG Editor".to_string());
                    }
                }
            }
            _ => {
                warnings.push("Moteur de jeu non reconnu. Seuls RPG Maker MV/MZ et Wolf RPG Editor sont pleinement supportés".to_string());
            }
        }
    } else {
        warnings.push("Impossible de détecter automatiquement le moteur de jeu".to_string());
        warnings.push("Assurez-vous que c'est un projet RPG Maker MV ou MZ valide".to_string());
    }

    // Check for required files in data directory
    if let Some(ref engine) = detected_engine {
        let data_root = match engine.as_str() {
            "RPG Maker MZ" => game_path.join("data"),
            "RPG Maker MV" => game_path.join("www").join("data"),
            _ => return Ok(GamePathValidation {
                is_valid: errors.is_empty(),
                detected_engine,
                errors,
                warnings,
            }),
        };

        if data_root.exists() {
            let actors_json = data_root.join("Actors.json");
            if !actors_json.exists() {
                errors.push("Fichier Actors.json manquant dans le dossier de données".to_string());
            }

            let system_json = data_root.join("System.json");
            if !system_json.exists() {
                errors.push("Fichier System.json manquant dans le dossier de données".to_string());
            }
        }
    }

    // Check write permissions (important for future saving)
    match std::fs::metadata(&path) {
        Ok(metadata) => {
            // On Unix systems, check if we can write to the directory
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                if permissions.readonly() {
                    warnings.push("Le dossier semble être en lecture seule".to_string());
                }
            }
        }
        Err(_) => {
            warnings.push("Impossible de vérifier les permissions du dossier".to_string());
        }
    }

    Ok(GamePathValidation {
        is_valid: errors.is_empty(),
        detected_engine,
        errors,
        warnings,
    })
}

/// Detect game engine from project structure
fn detect_game_engine(game_path: &Path) -> Result<Option<String>, String> {
    // Check for Wolf RPG Editor indicators
    let dump_folder = game_path.join("dump");
    if dump_folder.exists() && dump_folder.is_dir() {
        let db_dir = dump_folder.join("db");
        let mps_dir = dump_folder.join("mps");
        let common_dir = dump_folder.join("common");

        if db_dir.exists() && db_dir.is_dir()
            && mps_dir.exists() && mps_dir.is_dir()
            && common_dir.exists() && common_dir.is_dir()
        {
            return Ok(Some("Wolf RPG Editor".to_string()));
        }
    }

    // Check for RPG Maker MZ indicators
    let package_json = game_path.join("package.json");
    let mz_data_dir = game_path.join("data");

    if package_json.exists() && mz_data_dir.exists() && mz_data_dir.is_dir() {
        return Ok(Some("RPG Maker MZ".to_string()));
    }

    // Check for RPG Maker MV indicators
    let mv_data_dir = game_path.join("www").join("data");
    if mv_data_dir.exists() && mv_data_dir.is_dir() {
        return Ok(Some("RPG Maker MV".to_string()));
    }

    // Could be other engines or invalid structure
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_project_name_valid() {
        let result = validate_project_name("Mon Projet RPG".to_string()).unwrap();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_project_name_empty() {
        let result = validate_project_name("".to_string()).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_project_name_too_short() {
        let result = validate_project_name("AB".to_string()).unwrap();
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("3 caractères")));
    }

    #[test]
    fn test_validate_project_name_invalid_chars() {
        let result = validate_project_name("Projet/Test".to_string()).unwrap();
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("caractères invalides")));
    }

    #[test]
    fn test_validate_game_path_nonexistent() {
        let result = validate_game_path("/nonexistent/path".to_string()).unwrap();
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("n'existe pas")));
    }

    #[test]
    fn test_detect_game_engine_mz() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        // Create MZ structure
        fs::create_dir(game_path.join("data")).unwrap();
        fs::write(game_path.join("package.json"), "{}").unwrap();

        let result = detect_game_engine(game_path).unwrap();
        assert_eq!(result, Some("RPG Maker MZ".to_string()));
    }

    #[test]
    fn test_detect_game_engine_mv() {
        let temp_dir = TempDir::new().unwrap();
        let game_path = temp_dir.path();

        // Create MV structure
        fs::create_dir_all(game_path.join("www").join("data")).unwrap();

        let result = detect_game_engine(game_path).unwrap();
        assert_eq!(result, Some("RPG Maker MV".to_string()));
}
}