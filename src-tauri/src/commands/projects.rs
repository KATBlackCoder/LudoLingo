// Project validation commands
// Database operations are done in frontend via tauri-plugin-sql

/// Validates project name
#[tauri::command]
pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Project name cannot be empty".to_string());
    }
    if name.len() > 255 {
        return Err("Project name too long (max 255 characters)".to_string());
    }
    Ok(())
}

/// Validates game path
#[tauri::command]
pub fn validate_game_path(path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("Game path cannot be empty".to_string());
    }

    let path_obj = std::path::Path::new(path);
    if !path_obj.exists() {
        return Err("Path does not exist".to_string());
    }
    if !path_obj.is_dir() {
        return Err("Path must be a directory".to_string());
    }

    Ok(())
}
