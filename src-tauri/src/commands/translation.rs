// Translation commands
// Tauri commands for translation operations using Ollama

use crate::translation::ollama::{
    OllamaClient, OllamaConfig,
    SequentialTranslationManager, SingleTranslationManager,
    SequentialTranslationRequest
};
use std::sync::Arc;
use once_cell::sync::Lazy;
use tauri::AppHandle;

// Global translation managers (lazy initialized)
static SEQUENTIAL_MANAGER: Lazy<Arc<SequentialTranslationManager>> = Lazy::new(|| {
    let config = OllamaConfig::default();
    let client = Arc::new(OllamaClient::new(config));
    let single_manager = Arc::new(SingleTranslationManager::new(Arc::clone(&client)));
    Arc::new(SequentialTranslationManager::new(Arc::clone(&single_manager)))
});

static SINGLE_MANAGER: Lazy<Arc<SingleTranslationManager>> = Lazy::new(|| {
    let config = OllamaConfig::default();
    let client = Arc::new(OllamaClient::new(config));
    Arc::new(SingleTranslationManager::new(Arc::clone(&client)))
});

/// Check Ollama availability and get server information
/// This command delegates to the ollama module for the actual logic
#[tauri::command]
pub async fn check_ollama_status(
    host: Option<String>,
    port: Option<u16>
) -> Result<serde_json::Value, String> {
    crate::translation::ollama::check_ollama_status(host, port).await
}

/// Start sequential translation session
#[tauri::command]
pub async fn start_sequential_translation(
    app: AppHandle,
    project_id: i64,
    texts: Vec<crate::translation::ollama::TranslationText>,
    start_from: Option<i32>,
    source_language: Option<String>,
    target_language: Option<String>,
    model: Option<String>
) -> Result<serde_json::Value, String> {
    println!("🚀 [Rust] Starting translation for project {} with {} texts", project_id, texts.len());
    println!("🔧 [Rust] Received settings - source_language: {:?}, target_language: {:?}, model: {:?}", source_language, target_language, model);
    
    let request = SequentialTranslationRequest {
        project_id,
        texts: texts.clone(),
        start_from,
        source_language,
        target_language,
        model,
    };

    println!("📝 [Rust] Request created, calling SEQUENTIAL_MANAGER.start_session()");
    match SEQUENTIAL_MANAGER.start_session(app, request).await {
        Ok(session_id) => {
            println!("✅ [Rust] Session started successfully: {}", session_id);
            Ok(serde_json::json!({
                "session_id": session_id,
                "status": "started",
                "total_entries": texts.len()
            }))
        },
        Err(e) => {
            println!("❌ [Rust] Failed to start session: {}", e);
            Err(format!("Failed to start sequential translation: {}", e))
        }
    }
}

/// Get sequential translation progress
#[tauri::command]
pub async fn get_sequential_progress(session_id: String) -> Result<serde_json::Value, String> {
    match SEQUENTIAL_MANAGER.get_progress(&session_id).await {
        Some(progress) => Ok(serde_json::json!({
            "session_id": progress.session_id,
            "current_entry": progress.current_entry,
            "processed_count": progress.processed_count,
            "total_count": progress.total_count,
            "status": match progress.status {
                crate::translation::ollama::SequentialStatus::Idle => "idle",
                crate::translation::ollama::SequentialStatus::Running => "running",
                crate::translation::ollama::SequentialStatus::Paused => "paused",
                crate::translation::ollama::SequentialStatus::Completed => "completed",
                crate::translation::ollama::SequentialStatus::Error => "error",
            },
            "estimated_time_remaining": progress.estimated_time_remaining,
            "errors": progress.errors.iter().map(|e| serde_json::json!({
                "entry_id": e.entry_id,
                "error_message": e.error_message,
                "timestamp": e.timestamp
            })).collect::<Vec<_>>(),
            "successful_translations": progress.successful_translations.iter().map(|t| serde_json::json!({
                "entry_id": t.entry_id,
                "translated_text": t.translated_text,
                "timestamp": t.timestamp
            })).collect::<Vec<_>>()
        })),
        _none => Err(format!("Session {} not found", session_id))
    }
}

/// Pause sequential translation session
#[tauri::command]
pub async fn pause_sequential_session(session_id: String) -> Result<(), String> {
    SEQUENTIAL_MANAGER.pause_session(&session_id).await
}

/// Resume sequential translation session
#[tauri::command]
pub async fn resume_sequential_session(session_id: String) -> Result<(), String> {
    SEQUENTIAL_MANAGER.resume_session(&session_id).await
}

/// Stop sequential translation session
#[tauri::command]
pub async fn stop_sequential_session(session_id: String) -> Result<(), String> {
    SEQUENTIAL_MANAGER.stop_session(&session_id).await
}

/// Get active sequential sessions for project
#[tauri::command]
pub async fn get_project_sessions(_project_id: i64) -> Result<serde_json::Value, String> {
    // For now, return empty array since we don't persist sessions by project yet
    // This prevents the "Unknown error" while we implement proper session persistence
    let project_sessions: Vec<serde_json::Value> = Vec::new();

    // TODO: Implement proper session filtering by project_id when session persistence is added
    // let sessions = SEQUENTIAL_MANAGER.get_active_sessions().await;
    // let project_sessions: Vec<_> = sessions.into_iter()
    //     .filter(|s| s.project_id == project_id) // Filter by actual project_id when available
    //     .map(|s| serde_json::json!({...}))
    //     .collect();

    Ok(serde_json::json!(project_sessions))
}


/// Get translation suggestions for text
#[tauri::command]
pub async fn get_translation_suggestions(
    app: AppHandle,
    source_text: String,
    context: Option<String>
) -> Result<serde_json::Value, String> {
    match SINGLE_MANAGER.get_suggestions(Some(&app), &source_text, context.as_deref(), 3).await {
        Ok(suggestions) => {
            let suggestions_json: Vec<_> = suggestions.into_iter()
                .map(|s| serde_json::json!({
                    "suggestion": s.suggestion,
                    "confidence": s.confidence,
                    "source": s.source
                }))
                .collect();
            Ok(serde_json::json!(suggestions_json))
        }
        Err(e) => Err(format!("Failed to get translation suggestions: {}", e))
    }
}

/// Translate a single text entry
#[tauri::command]
pub async fn translate_single_text(
    app: AppHandle,
    source_text: String,
    source_language: Option<String>,
    target_language: Option<String>,
    context: Option<String>,
    model: Option<String>
) -> Result<serde_json::Value, String> {
    use crate::translation::ollama::SingleTranslationRequest;
    
    let request = SingleTranslationRequest {
        source_text,
        source_language,
        target_language,
        context,
        model,
    };

    match SINGLE_MANAGER.translate(&app, request).await {
        Ok(result) => Ok(serde_json::json!({
            "translated_text": result.translated_text,
            "model_used": result.model_used,
            "confidence": result.confidence,
            "processing_time_ms": result.processing_time_ms
        })),
        Err(e) => Err(format!("Failed to translate text: {}", e))
    }
}

/// Update a text entry with translation
/// Note: This command is a placeholder - actual DB updates are done via tauri-plugin-sql from frontend
#[tauri::command]
pub async fn update_translation_entry(
    entry_id: i32,
    translated_text: String,
    source: String
) -> Result<(), String> {
    println!("💾 [Rust] Received request to update entry {} with source '{}'", entry_id, source);
    println!("📝 [Rust] Translation text: {}", translated_text);
    
    // The actual database update is handled by the frontend using tauri-plugin-sql
    // This command exists to maintain API consistency and for logging
    Ok(())
}


