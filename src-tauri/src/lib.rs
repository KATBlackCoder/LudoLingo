// Module declarations
pub mod commands;
pub mod core;
#[cfg(test)]
mod integration_test;
pub mod migrations;
pub mod models;
pub mod parsers;
pub mod translation;

// Re-export for use in main
pub use commands::scanning::ScanState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:ludolingo.db", migrations::get_migrations())
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .manage(ScanState::default())
        .invoke_handler(tauri::generate_handler![
            commands::validate_project_name,
            commands::validate_game_path,
            commands::scan_folder,
            commands::get_scan_progress,
            commands::cancel_scan,
            commands::validate_file_format,
            commands::validate_game_file,
            commands::validate_files_batch,
            commands::extract_texts_from_folder,
            commands::check_ollama_status,
            commands::start_sequential_translation,
            commands::get_sequential_progress,
            commands::pause_sequential_session,
            commands::resume_sequential_session,
            commands::stop_sequential_session,
            commands::get_project_sessions,
            commands::get_translation_suggestions,
            commands::update_translation_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_setup() {
        // Basic test to verify modules are accessible
        assert!(true);
    }
}
