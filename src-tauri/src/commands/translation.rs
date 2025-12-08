// Translation commands
// Tauri commands for translation operations using Ollama (local) or RunPod (online)

use crate::translation::common::types::{SequentialTranslationRequest as OllamaSequentialRequest, TranslationText as OllamaTranslationText};
use crate::translation::ollama::{
    OllamaClient, OllamaConfig, SequentialTranslationManager as OllamaSequentialManager,
    SingleTranslationManager as OllamaSingleManager,
};
use crate::translation::common::types::{SequentialTranslationRequest as RunPodSequentialRequest, TranslationText as RunPodTranslationText};
use crate::translation::runpod::{
    RunPodClient, RunPodConfig, SequentialTranslationManager as RunPodSequentialManager,
    SingleTranslationManager as RunPodSingleManager,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::AppHandle;

// Provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationProvider {
    Ollama,
    RunPod,
}

// Global Ollama managers (local)
static OLLAMA_SEQUENTIAL_MANAGER: Lazy<Arc<OllamaSequentialManager>> = Lazy::new(|| {
    let config = OllamaConfig::default();
    let client = Arc::new(OllamaClient::new(config));
    let single_manager = Arc::new(OllamaSingleManager::new(Arc::clone(&client)));
    Arc::new(OllamaSequentialManager::new(Arc::clone(&single_manager)))
});

static OLLAMA_SINGLE_MANAGER: Lazy<Arc<OllamaSingleManager>> = Lazy::new(|| {
    let config = OllamaConfig::default();
    let client = Arc::new(OllamaClient::new(config));
    Arc::new(OllamaSingleManager::new(Arc::clone(&client)))
});

// Global RunPod managers (online)
// Cache managers by pod_id to preserve sessions across command calls
type RunPodManagers = (Arc<RunPodSequentialManager>, Arc<RunPodSingleManager>);
static RUNPOD_MANAGERS_CACHE: Lazy<Arc<Mutex<HashMap<String, RunPodManagers>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Helper function to get or create RunPod managers for a pod_id
/// This ensures sessions persist across command calls
async fn get_runpod_managers(
    pod_id: String,
) -> (Arc<RunPodSequentialManager>, Arc<RunPodSingleManager>) {
    let cache = RUNPOD_MANAGERS_CACHE.clone();
    let mut managers = cache.lock().await;

    // Check if managers already exist for this pod_id
    if let Some(existing) = managers.get(&pod_id) {
        return (Arc::clone(&existing.0), Arc::clone(&existing.1));
    }

    // Create new managers for this pod_id
    let config = RunPodConfig {
        pod_id: pod_id.clone(),
    };
    let client = Arc::new(RunPodClient::new(config));
    let single_manager = Arc::new(RunPodSingleManager::new(Arc::clone(&client)));
    let sequential_manager = Arc::new(RunPodSequentialManager::new(Arc::clone(&single_manager)));

    let managers_tuple = (Arc::clone(&sequential_manager), Arc::clone(&single_manager));
    managers.insert(pod_id, managers_tuple.clone());

    (sequential_manager, single_manager)
}

/// Helper function to convert Ollama TranslationText to RunPod TranslationText
fn convert_texts_ollama_to_runpod(texts: Vec<OllamaTranslationText>) -> Vec<RunPodTranslationText> {
    texts
        .into_iter()
        .map(|t| RunPodTranslationText {
            id: t.id,
            source_text: t.source_text,
            context: t.context,
            text_type: t.text_type,
        })
        .collect()
}

/// Helper function to convert RunPod TranslationText to Ollama TranslationText
#[allow(dead_code)] // May be used in future features
fn convert_texts_runpod_to_ollama(texts: Vec<RunPodTranslationText>) -> Vec<OllamaTranslationText> {
    texts
        .into_iter()
        .map(|t| OllamaTranslationText {
            id: t.id,
            source_text: t.source_text,
            context: t.context,
            text_type: t.text_type,
        })
        .collect()
}

/// Check Ollama availability and get server information (local)
/// This command delegates to the ollama module for the actual logic
#[tauri::command]
pub async fn check_ollama_status(
    host: Option<String>,
    port: Option<u16>,
) -> Result<serde_json::Value, String> {
    crate::translation::ollama::check_ollama_status(host, port).await
}

/// Check RunPod availability and get server information (online)
/// This command delegates to the runpod module for the actual logic
#[tauri::command]
pub async fn check_runpod_status(pod_id: String) -> Result<serde_json::Value, String> {
    println!(
        "🔍 [Rust] check_runpod_status called with pod_id: {:?}",
        pod_id
    );
    crate::translation::runpod::check_runpod_status(Some(pod_id)).await
}

/// Start sequential translation session
/// Routes to Ollama (local) or RunPod (online) based on provider parameter
#[tauri::command]
pub async fn start_sequential_translation(
    app: AppHandle,
    provider: String, // "ollama" or "runpod"
    project_id: i64,
    texts: Vec<OllamaTranslationText>, // Common format from frontend
    start_from: Option<i32>,
    source_language: Option<String>,
    target_language: Option<String>,
    model: Option<String>,
    pod_id: Option<String>, // Required for RunPod provider
) -> Result<serde_json::Value, String> {
    println!(
        "🚀 [Rust] Starting translation for project {} with {} texts using provider: {}",
        project_id,
        texts.len(),
        provider
    );

    let provider_enum = match provider.as_str() {
        "ollama" => TranslationProvider::Ollama,
        "runpod" => TranslationProvider::RunPod,
        _ => {
            return Err(format!(
                "Invalid provider: {}. Must be 'ollama' or 'runpod'",
                provider
            ))
        }
    };

    match provider_enum {
        TranslationProvider::Ollama => {
            let request = OllamaSequentialRequest {
                project_id,
                texts: texts.clone(),
                start_from,
                source_language,
                target_language,
                model,
                pause_settings: None, // Sera configuré depuis les settings utilisateur
            };

            match OLLAMA_SEQUENTIAL_MANAGER.start_session(app, request).await {
                Ok(session_id) => Ok(serde_json::json!({
                    "session_id": session_id,
                    "status": "started",
                    "total_entries": texts.len(),
                    "provider": "ollama"
                })),
                Err(e) => Err(format!("Failed to start Ollama translation: {}", e)),
            }
        }
        TranslationProvider::RunPod => {
            let pod_id_str =
                pod_id.ok_or_else(|| "pod_id is required for RunPod provider".to_string())?;

            let runpod_texts = convert_texts_ollama_to_runpod(texts.clone());
            let request = RunPodSequentialRequest {
                project_id,
                texts: runpod_texts,
                start_from,
                source_language,
                target_language,
                model,
                pause_settings: None, // Sera configuré depuis les settings utilisateur
            };

            let (sequential_manager, _) = get_runpod_managers(pod_id_str).await;
            match sequential_manager.start_session(app, request).await {
                Ok(session_id) => Ok(serde_json::json!({
                    "session_id": session_id,
                    "status": "started",
                    "total_entries": texts.len(),
                    "provider": "runpod"
                })),
                Err(e) => Err(format!("Failed to start RunPod translation: {}", e)),
            }
        }
    }
}

/// Get sequential translation progress
/// Routes to Ollama or RunPod based on provider parameter
#[tauri::command]
pub async fn get_sequential_progress(
    session_id: String,
    provider: String,       // "ollama" or "runpod"
    pod_id: Option<String>, // Required for RunPod
) -> Result<serde_json::Value, String> {
    let provider_enum = match provider.as_str() {
        "ollama" => TranslationProvider::Ollama,
        "runpod" => TranslationProvider::RunPod,
        _ => {
            return Err(format!(
                "Invalid provider: {}. Must be 'ollama' or 'runpod'",
                provider
            ))
        }
    };

    match provider_enum {
        TranslationProvider::Ollama => {
            match OLLAMA_SEQUENTIAL_MANAGER.get_progress(&session_id).await {
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
                None => Err(format!("Session {} not found", session_id)),
            }
        }
        TranslationProvider::RunPod => {
            let pod_id_str = pod_id.ok_or_else(|| "pod_id is required for RunPod".to_string())?;
            let (sequential_manager, _) = get_runpod_managers(pod_id_str).await;
            match sequential_manager.get_progress(&session_id).await {
                Some(progress) => Ok(serde_json::json!({
                    "session_id": progress.session_id,
                    "current_entry": progress.current_entry,
                    "processed_count": progress.processed_count,
                    "total_count": progress.total_count,
                    "status": match progress.status {
                        crate::translation::runpod::SequentialStatus::Idle => "idle",
                        crate::translation::runpod::SequentialStatus::Running => "running",
                        crate::translation::runpod::SequentialStatus::Paused => "paused",
                        crate::translation::runpod::SequentialStatus::Completed => "completed",
                        crate::translation::runpod::SequentialStatus::Error => "error",
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
                None => Err(format!("Session {} not found", session_id)),
            }
        }
    }
}

/// Pause sequential translation session
/// Routes to Ollama or RunPod based on provider parameter
#[tauri::command]
pub async fn pause_sequential_session(
    session_id: String,
    provider: String,
    pod_id: Option<String>, // Required for RunPod
) -> Result<(), String> {
    let provider_enum = match provider.as_str() {
        "ollama" => TranslationProvider::Ollama,
        "runpod" => TranslationProvider::RunPod,
        _ => return Err(format!("Invalid provider: {}", provider)),
    };

    match provider_enum {
        TranslationProvider::Ollama => OLLAMA_SEQUENTIAL_MANAGER.pause_session(&session_id).await,
        TranslationProvider::RunPod => {
            let pod_id_str = pod_id.ok_or_else(|| "pod_id is required for RunPod".to_string())?;
            let (sequential_manager, _) = get_runpod_managers(pod_id_str).await;
            sequential_manager.pause_session(&session_id).await
        }
    }
}

/// Resume sequential translation session
/// Routes to Ollama or RunPod based on provider parameter
#[tauri::command]
pub async fn resume_sequential_session(
    session_id: String,
    provider: String,
    pod_id: Option<String>, // Required for RunPod
) -> Result<(), String> {
    let provider_enum = match provider.as_str() {
        "ollama" => TranslationProvider::Ollama,
        "runpod" => TranslationProvider::RunPod,
        _ => return Err(format!("Invalid provider: {}", provider)),
    };

    match provider_enum {
        TranslationProvider::Ollama => OLLAMA_SEQUENTIAL_MANAGER.resume_session(&session_id).await,
        TranslationProvider::RunPod => {
            let pod_id_str = pod_id.ok_or_else(|| "pod_id is required for RunPod".to_string())?;
            let (sequential_manager, _) = get_runpod_managers(pod_id_str).await;
            sequential_manager.resume_session(&session_id).await
        }
    }
}

/// Stop sequential translation session
/// Routes to Ollama or RunPod based on provider parameter
#[tauri::command]
pub async fn stop_sequential_session(
    session_id: String,
    provider: String,
    pod_id: Option<String>, // Required for RunPod
) -> Result<(), String> {
    let provider_enum = match provider.as_str() {
        "ollama" => TranslationProvider::Ollama,
        "runpod" => TranslationProvider::RunPod,
        _ => return Err(format!("Invalid provider: {}", provider)),
    };

    match provider_enum {
        TranslationProvider::Ollama => OLLAMA_SEQUENTIAL_MANAGER.stop_session(&session_id).await,
        TranslationProvider::RunPod => {
            let pod_id_str = pod_id.ok_or_else(|| "pod_id is required for RunPod".to_string())?;
            let (sequential_manager, _) = get_runpod_managers(pod_id_str).await;
            sequential_manager.stop_session(&session_id).await
        }
    }
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
/// Routes to Ollama or RunPod based on provider parameter
#[tauri::command]
pub async fn get_translation_suggestions(
    app: AppHandle,
    provider: String, // "ollama" or "runpod"
    source_text: String,
    context: Option<String>,
    pod_id: Option<String>, // Required for RunPod
) -> Result<serde_json::Value, String> {
    let provider_enum = match provider.as_str() {
        "ollama" => TranslationProvider::Ollama,
        "runpod" => TranslationProvider::RunPod,
        _ => {
            return Err(format!(
                "Invalid provider: {}. Must be 'ollama' or 'runpod'",
                provider
            ))
        }
    };

    match provider_enum {
        TranslationProvider::Ollama => {
            match OLLAMA_SINGLE_MANAGER
                .get_suggestions(Some(&app), &source_text, context.as_deref(), 3)
                .await
            {
                Ok(suggestions) => {
                    let suggestions_json: Vec<_> = suggestions
                        .into_iter()
                        .map(|s| {
                            serde_json::json!({
                                "suggestion": s.suggestion,
                                "confidence": s.confidence,
                                "source": s.source
                            })
                        })
                        .collect();
                    Ok(serde_json::json!(suggestions_json))
                }
                Err(e) => Err(format!("Failed to get Ollama suggestions: {}", e)),
            }
        }
        TranslationProvider::RunPod => {
            let pod_id_str =
                pod_id.ok_or_else(|| "pod_id is required for RunPod provider".to_string())?;
            let (_, single_manager) = get_runpod_managers(pod_id_str).await;
            match single_manager
                .get_suggestions(Some(&app), &source_text, context.as_deref(), 3)
                .await
            {
                Ok(suggestions) => {
                    let suggestions_json: Vec<_> = suggestions
                        .into_iter()
                        .map(|s| {
                            serde_json::json!({
                                "suggestion": s.suggestion,
                                "confidence": s.confidence,
                                "source": s.source
                            })
                        })
                        .collect();
                    Ok(serde_json::json!(suggestions_json))
                }
                Err(e) => Err(format!("Failed to get RunPod suggestions: {}", e)),
            }
        }
    }
}

/// Translate a single text entry
/// Routes to Ollama or RunPod based on provider parameter
#[tauri::command]
pub async fn translate_single_text(
    app: AppHandle,
    provider: String, // "ollama" or "runpod"
    source_text: String,
    source_language: Option<String>,
    target_language: Option<String>,
    context: Option<String>,
    model: Option<String>,
    pod_id: Option<String>, // Required for RunPod
) -> Result<serde_json::Value, String> {
    let provider_enum = match provider.as_str() {
        "ollama" => TranslationProvider::Ollama,
        "runpod" => TranslationProvider::RunPod,
        _ => {
            return Err(format!(
                "Invalid provider: {}. Must be 'ollama' or 'runpod'",
                provider
            ))
        }
    };

    match provider_enum {
        TranslationProvider::Ollama => {
            use crate::translation::ollama::SingleTranslationRequest;

            let request = SingleTranslationRequest {
                source_text,
                source_language,
                target_language,
                context,
                model,
                project_id: None,
                text_type: None,
            };

            match OLLAMA_SINGLE_MANAGER.translate(&app, request).await {
                Ok(result) => Ok(serde_json::json!({
                    "translated_text": result.translated_text,
                    "model_used": result.model_used,
                    "confidence": result.confidence,
                    "processing_time_ms": result.processing_time_ms
                })),
                Err(e) => Err(format!("Failed to translate text with Ollama: {}", e)),
            }
        }
        TranslationProvider::RunPod => {
            use crate::translation::runpod::SingleTranslationRequest;

            let pod_id_str =
                pod_id.ok_or_else(|| "pod_id is required for RunPod provider".to_string())?;

            // Log source text preview BEFORE moving source_text into request
            let text_preview = if source_text.len() > 50 {
                format!("{}...", &source_text[..50])
            } else {
                source_text.clone()
            };

            let request = SingleTranslationRequest {
                source_text,
                source_language,
                target_language,
                context,
                model,
                project_id: None,
                text_type: None,
            };

            println!(
                "🚀 [Rust] Getting RunPod managers for pod_id: {}",
                pod_id_str
            );
            let (_, single_manager) = get_runpod_managers(pod_id_str).await;
            println!(
                "🚀 [Rust] Starting RunPod translation for text: {}",
                text_preview
            );

            match single_manager.translate(&app, request).await {
                Ok(result) => {
                    println!("✅ [Rust] RunPod translation successful");
                    Ok(serde_json::json!({
                        "translated_text": result.translated_text,
                        "model_used": result.model_used,
                        "confidence": result.confidence,
                        "processing_time_ms": result.processing_time_ms
                    }))
                }
                Err(e) => {
                    println!("❌ [Rust] RunPod translation failed: {}", e);
                    Err(format!("Failed to translate text with RunPod: {}", e))
                }
            }
        }
    }
}

/// Update a text entry with translation
/// Note: This command is a placeholder - actual DB updates are done via tauri-plugin-sql from frontend
#[tauri::command]
pub async fn update_translation_entry(
    entry_id: i32,
    translated_text: String,
    source: String,
) -> Result<(), String> {
    println!(
        "💾 [Rust] Received request to update entry {} with source '{}'",
        entry_id, source
    );
    println!("📝 [Rust] Translation text: {}", translated_text);

    // The actual database update is handled by the frontend using tauri-plugin-sql
    // This command exists to maintain API consistency and for logging
    Ok(())
}
