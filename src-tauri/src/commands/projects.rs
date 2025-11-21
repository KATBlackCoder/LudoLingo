// Project validation commands
// Provides business logic validation for project management

use crate::parsers::factory::EngineFactory;
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
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
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
            suggestions
                .push("Utilisez uniquement des lettres, chiffres, espaces et tirets".to_string());
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
    let game_path = Path::new(&path);

    // Check if path exists
    if !game_path.exists() {
        return Ok(GamePathValidation {
            is_valid: false,
            detected_engine: None,
            errors: vec!["Le chemin spécifié n'existe pas".to_string()],
            warnings: Vec::new(),
        });
    }

    // Check if it's a directory
    if !game_path.is_dir() {
        return Ok(GamePathValidation {
            is_valid: false,
            detected_engine: None,
            errors: vec!["Le chemin doit pointer vers un dossier de jeu".to_string()],
            warnings: Vec::new(),
        });
    }

    // Try to detect engine and create handler using factory
    let handler = match EngineFactory::create_handler(game_path) {
        Ok(handler) => handler,
        Err(e) => {
            // No engine detected - return validation result with error
            return Ok(GamePathValidation {
                is_valid: false,
                detected_engine: None,
                errors: vec![e],
                warnings: Vec::new(),
            });
        }
    };

    // Get engine name
    let detected_engine = Some(handler.engine_name().to_string());

    // Validate project structure using handler
    let validation_result = handler.validate_project_structure(game_path)?;

    // Check write permissions (important for future saving)
    let mut warnings = validation_result.warnings.clone();
    match std::fs::metadata(&path) {
        Ok(metadata) => {
            // On Unix systems, check if we can write to the directory
            #[cfg(unix)]
            {
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
        is_valid: validation_result.is_valid,
        detected_engine,
        errors: validation_result.errors,
        warnings,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("caractères invalides")));
    }

    #[test]
    fn test_validate_game_path_nonexistent() {
        let result = validate_game_path("/nonexistent/path".to_string()).unwrap();
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("n'existe pas")));
    }

    #[test]
    fn test_validate_game_path_mz() {
        // Test with real MZ game from engines_past
        let game_path = "../engines_past/MZgame";

        let result = validate_game_path(game_path.to_string()).unwrap();
        assert_eq!(result.detected_engine, Some("RPG Maker MZ".to_string()));
        assert!(result.is_valid, "MZ game should be valid");
    }

    #[test]
    fn test_validate_game_path_mv() {
        // Test with real MV game from engines_past
        let game_path = "../engines_past/MVgame";

        let result = validate_game_path(game_path.to_string()).unwrap();
        assert_eq!(result.detected_engine, Some("RPG Maker MV".to_string()));
        assert!(result.is_valid, "MV game should be valid");
    }

    #[test]
    fn test_validate_game_path_wolfrpg() {
        // Test with real WolfRPG game from engines_past
        let game_path = "../engines_past/wolfrpg";

        let result = validate_game_path(game_path.to_string()).unwrap();
        assert_eq!(result.detected_engine, Some("Wolf RPG Editor".to_string()));
        assert!(result.is_valid, "WolfRPG game should be valid");
    }
}
