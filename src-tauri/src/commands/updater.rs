// Updater commands module
// Provides Tauri commands for checking and managing updates

use tauri::AppHandle;

/// Check for available updates
/// Returns information about available update if one exists
#[tauri::command]
#[cfg(desktop)]
pub async fn check_updates(app: AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_updater::UpdaterExt;

    let updater_builder = app.updater_builder();
    let updater = updater_builder
        .build()
        .map_err(|e| format!("Failed to build updater: {}", e))?;

    let update = updater
        .check()
        .await
        .map_err(|e| format!("Failed to check for updates: {}", e))?;

    match update {
        Some(update_info) => {
            let version = update_info.version.clone();
            let body = update_info.body.clone().unwrap_or_default();
            Ok(Some(format!("Update available: {} - {}", version, body)))
        }
        None => Ok(None),
    }
}
