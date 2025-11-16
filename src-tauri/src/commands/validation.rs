// File validation commands for game localization
// Provides comprehensive validation for game files and project data

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Validation result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub details: ValidationDetails,
}

/// Detailed validation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDetails {
    pub file_path: String,
    pub file_size: Option<u64>,
    pub detected_format: Option<String>,
    pub checksum: Option<String>,
}

/// File format validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFormatValidation {
    pub is_supported: bool,
    pub format_type: Option<String>,
    pub compatibility_level: CompatibilityLevel,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    Full,
    Partial,
    None,
}

/// Project validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectValidation {
    pub name_valid: bool,
    pub path_valid: bool,
    pub engine_compatible: bool,
    pub existing_data_integrity: bool,
}

/// Batch validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchValidationResult {
    pub total_files: usize,
    pub valid_files: usize,
    pub invalid_files: usize,
    pub file_results: Vec<FileValidationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValidationSummary {
    pub file_path: String,
    pub is_valid: bool,
    pub error_count: usize,
    pub warning_count: usize,
}

/// Validate a single file for game localization compatibility
#[tauri::command]
pub fn validate_game_file(file_path: String) -> Result<ValidationResult, String> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Basic file checks
    let metadata = match path.metadata() {
        Ok(meta) => meta,
        Err(e) => {
            errors.push(format!("Cannot read file metadata: {}", e));
            return Ok(ValidationResult {
                is_valid: false,
                errors,
                warnings,
                details: ValidationDetails {
                    file_path: file_path.clone(),
                    file_size: None,
                    detected_format: None,
                    checksum: None,
                },
            });
        }
    };

    let file_size = metadata.len();

    // Size validation
    if file_size == 0 {
        errors.push("File is empty".to_string());
    } else if file_size > 100 * 1024 * 1024 {
        // 100MB
        warnings.push("File is very large (>100MB), scanning may be slow".to_string());
    }

    // Extension validation
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let detected_format = match extension {
        "json" => Some("JSON".to_string()),
        "rxdata" => Some("Ruby Marshal".to_string()),
        "rvdata2" => Some("Ruby Marshal v2".to_string()),
        _ => {
            errors.push(format!("Unsupported file extension: {}", extension));
            None
        }
    };

    // JSON-specific validation
    if extension == "json" {
        match validate_json_file(path) {
            Ok(json_validation) => {
                if !json_validation.is_valid {
                    errors.extend(json_validation.errors);
                }
                warnings.extend(json_validation.warnings);
            }
            Err(e) => {
                errors.push(format!("JSON validation failed: {}", e));
            }
        }
    }

    // Calculate checksum if file is reasonable size
    let checksum = if file_size > 0 && file_size < 10 * 1024 * 1024 {
        // 10MB
        match calculate_checksum(path) {
            Ok(cs) => Some(cs),
            Err(e) => {
                warnings.push(format!("Could not calculate checksum: {}", e));
                None
            }
        }
    } else {
        None
    };

    let is_valid = errors.is_empty();

    Ok(ValidationResult {
        is_valid,
        errors,
        warnings,
        details: ValidationDetails {
            file_path,
            file_size: Some(file_size),
            detected_format,
            checksum,
        },
    })
}

/// Validate JSON file structure and content
fn validate_json_file(file_path: &Path) -> Result<ValidationResult, String> {
    let content =
        std::fs::read_to_string(file_path).map_err(|e| format!("Cannot read file: {}", e))?;

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Parse JSON
    let json_value: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))?;

    // Basic structure validation
    match json_value {
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                warnings.push("JSON array is empty".to_string());
            } else {
                // Check for null first element (common in RPG Maker)
                if arr.len() > 0 && arr[0].is_null() {
                    // This is expected for RPG Maker files
                }

                // Validate array elements
                for (index, item) in arr.iter().enumerate() {
                    if index == 0 && item.is_null() {
                        continue; // Skip null first element
                    }

                    if item.is_object() {
                        // Basic object validation
                        let obj = item.as_object().unwrap();
                        if obj.is_empty() {
                            warnings.push(format!("Empty object at index {}", index));
                        }
                    } else {
                        errors.push(format!("Non-object element at index {}: {}", index, item));
                    }
                }
            }
        }
        serde_json::Value::Object(obj) => {
            if obj.is_empty() {
                warnings.push("JSON object is empty".to_string());
            }
            // Validate known RPG Maker structures
            validate_rpg_maker_structure(&obj, &mut errors, &mut warnings);
        }
        _ => {
            errors.push("JSON root must be an array or object".to_string());
        }
    }

    Ok(ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
        details: ValidationDetails {
            file_path: file_path.to_string_lossy().to_string(),
            file_size: None,
            detected_format: Some("JSON".to_string()),
            checksum: None,
        },
    })
}

/// Validate RPG Maker specific JSON structures
fn validate_rpg_maker_structure(
    obj: &serde_json::Map<String, serde_json::Value>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    // Check for common RPG Maker fields
    let has_id = obj.contains_key("id");
    let has_name = obj.contains_key("name");

    if !has_id && !has_name {
        warnings.push("Object does not have typical RPG Maker fields (id, name)".to_string());
    }

    // Validate specific file types based on structure
    if has_id && obj.get("id").unwrap().is_number() {
        // Looks like a database entry
        if let Some(name) = obj.get("name") {
            if !name.is_string() {
                errors.push("'name' field should be a string".to_string());
            }
        }
    }
}

/// Calculate SHA-256 checksum of a file
fn calculate_checksum(file_path: &Path) -> Result<String, String> {
    use sha2::{Digest, Sha256};
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_path).map_err(|e| format!("Cannot open file: {}", e))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| format!("Cannot read file: {}", e))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Validate multiple files in batch
#[tauri::command]
pub fn validate_files_batch(file_paths: Vec<String>) -> Result<BatchValidationResult, String> {
    let mut valid_files = 0;
    let mut invalid_files = 0;
    let mut file_results = Vec::new();

    for file_path in file_paths {
        match validate_game_file(file_path.clone()) {
            Ok(result) => {
                if result.is_valid {
                    valid_files += 1;
                } else {
                    invalid_files += 1;
                }

                file_results.push(FileValidationSummary {
                    file_path,
                    is_valid: result.is_valid,
                    error_count: result.errors.len(),
                    warning_count: result.warnings.len(),
                });
            }
            Err(_e) => {
                invalid_files += 1;
                file_results.push(FileValidationSummary {
                    file_path,
                    is_valid: false,
                    error_count: 1,
                    warning_count: 0,
                });
            }
        }
    }

    Ok(BatchValidationResult {
        total_files: file_results.len(),
        valid_files,
        invalid_files,
        file_results,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_game_file_nonexistent() {
        let result = validate_game_file("nonexistent.json".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_game_file_empty() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.json");
        fs::write(&file_path, "").unwrap();

        let result = validate_game_file(file_path.to_string_lossy().to_string()).unwrap();
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("empty")));
    }

    #[test]
    fn test_validate_game_file_valid_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("valid.json");
        fs::write(&file_path, r#"{"name": "Test", "id": 1}"#).unwrap();

        let result = validate_game_file(file_path.to_string_lossy().to_string()).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.detected_format, Some("JSON".to_string()));
    }

    #[test]
    fn test_validate_game_file_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.json");
        fs::write(&file_path, r#"{"name": "Test", "id": }"#).unwrap();

        let result = validate_game_file(file_path.to_string_lossy().to_string()).unwrap();
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("JSON")));
    }

    #[test]
    fn test_validate_files_batch() {
        let temp_dir = TempDir::new().unwrap();

        let valid_file = temp_dir.path().join("valid.json");
        fs::write(&valid_file, r#"{"name": "Test"}"#).unwrap();

        let invalid_file = temp_dir.path().join("invalid.json");
        fs::write(&invalid_file, r#"{"name": }"#).unwrap();

        let file_paths = vec![
            valid_file.to_string_lossy().to_string(),
            invalid_file.to_string_lossy().to_string(),
        ];

        let result = validate_files_batch(file_paths).unwrap();
        assert_eq!(result.total_files, 2);
        assert_eq!(result.valid_files, 1);
        assert_eq!(result.invalid_files, 1);
    }
}
