// Sequential translation logic for RunPod
// Handles translation of text entries one by one using RunPod Ollama via HTTP

use crate::translation::runpod::{
    get_default_model, get_default_source_language, get_default_target_language,
    SingleTranslationManager,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use tauri::AppHandle;

/// Translation text with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationText {
    pub id: i32,
    pub source_text: String,
    pub context: Option<String>,
    pub text_type: Option<String>,  // Text type for category filtering: 'dialogue', 'system', 'item', 'skill', 'other'
}

/// Sequential translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequentialTranslationRequest {
    pub project_id: i64,
    pub texts: Vec<TranslationText>,
    pub start_from: Option<i32>, // Resume from specific entry
    pub source_language: Option<String>, // Override project default
    pub target_language: Option<String>, // Override project default
    pub model: Option<String>, // Override default model
}

/// Sequential translation progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequentialProgress {
    pub session_id: String,
    pub current_entry: Option<i32>,
    pub processed_count: i32,
    pub total_count: i32,
    pub status: SequentialStatus,
    pub estimated_time_remaining: Option<i64>, // seconds
    pub errors: Vec<SequentialError>,
    pub successful_translations: Vec<SuccessfulTranslation>,
}

/// Sequential status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SequentialStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Error,
}

/// Sequential error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequentialError {
    pub entry_id: i32,
    pub error_message: String,
    pub timestamp: String,
}

/// Successful translation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessfulTranslation {
    pub entry_id: i32,
    pub translated_text: String,
    pub timestamp: String,
}

/// Translation settings for a session
#[derive(Debug, Clone)]
pub struct TranslationSettings {
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub model: Option<String>,
}

/// Sequential translation session
#[derive(Debug)]
pub struct SequentialSession {
    pub session_id: String,
    pub project_id: i64,
    pub texts: Vec<TranslationText>,
    pub current_index: usize,
    pub processed_entries: HashMap<i32, bool>, // entry_id -> success
    pub errors: Vec<SequentialError>,
    pub successful_translations: Vec<SuccessfulTranslation>,
    pub status: SequentialStatus,
    pub start_time: std::time::Instant,
    pub translation_settings: TranslationSettings, // Translation parameters
    pub app_handle: AppHandle, // Required for glossary lookup
}

/// Sequential translation manager for RunPod
pub struct SequentialTranslationManager {
    client: Arc<SingleTranslationManager>,
    active_sessions: Arc<Mutex<HashMap<String, SequentialSession>>>,
    session_counter: Arc<Mutex<u64>>,
}

impl SequentialTranslationManager {
    /// Create new sequential translation manager
    pub fn new(single_manager: Arc<SingleTranslationManager>) -> Self {
        Self {
            client: single_manager,
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
            session_counter: Arc::new(Mutex::new(1)),
        }
    }

    /// Start a new sequential translation session
    pub async fn start_session(
        &self,
        app_handle: AppHandle,
        request: SequentialTranslationRequest,
    ) -> Result<String, String> {
        println!("🔧 [RunPod Sequential] start_session called with {} texts", request.texts.len());
        let session_id = self.generate_session_id().await;

        let session = SequentialSession {
            session_id: session_id.clone(),
            project_id: request.project_id,
            texts: request.texts.clone(),
            current_index: request.start_from.map(|id| {
                request.texts.iter().position(|text| text.id == id).unwrap_or(0)
            }).unwrap_or(0),
            processed_entries: HashMap::new(),
            errors: Vec::new(),
            successful_translations: Vec::new(),
            status: SequentialStatus::Running,
            start_time: std::time::Instant::now(),
            translation_settings: TranslationSettings {
                source_language: request.source_language,
                target_language: request.target_language,
                model: request.model,
            },
            app_handle,
        };

        {
            let mut sessions = self.active_sessions.lock().await;
            sessions.insert(session_id.clone(), session);
        }

        let manager = Arc::new(self.clone());
        let session_id_clone = session_id.clone();
        tokio::spawn(async move {
            manager.process_session(session_id_clone).await;
        });

        Ok(session_id)
    }

    /// Get session progress
    pub async fn get_progress(&self, session_id: &str) -> Option<SequentialProgress> {
        let mut sessions = self.active_sessions.lock().await;
        sessions.get_mut(session_id).map(|session| {
            let total_count = session.texts.len() as i32;
            let processed_count = session.processed_entries.len() as i32;
            let current_entry = session.texts.get(session.current_index).map(|text| text.id);

            let avg_time_per_entry = 3.0; // seconds
            let remaining_entries = total_count - processed_count;
            let estimated_time_remaining = if processed_count > 0 {
                Some((remaining_entries as f64 * avg_time_per_entry) as i64)
            } else {
                None
            };

            let successful_translations = session.successful_translations.drain(..).collect::<Vec<_>>();

            SequentialProgress {
                session_id: session.session_id.clone(),
                current_entry,
                processed_count,
                total_count,
                status: session.status.clone(),
                estimated_time_remaining,
                errors: session.errors.clone(),
                successful_translations,
            }
        })
    }

    /// Pause session
    pub async fn pause_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = SequentialStatus::Paused;
            Ok(())
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }

    /// Resume session
    pub async fn resume_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            if matches!(session.status, SequentialStatus::Paused) {
                session.status = SequentialStatus::Running;

                let manager = Arc::new(self.clone());
                let session_id = session_id.to_string();
                tokio::spawn(async move {
                    manager.process_session(session_id).await;
                });

                Ok(())
            } else {
                Err(format!("Session {} is not paused", session_id))
            }
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }

    /// Stop session
    pub async fn stop_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = SequentialStatus::Idle;
            Ok(())
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }

    /// Get all active sessions
    pub async fn get_active_sessions(&self) -> Vec<SequentialProgress> {
        let sessions = self.active_sessions.lock().await;
        let session_ids: Vec<String> = sessions.keys().cloned().collect();

        let mut results = Vec::new();
        for session_id in session_ids {
            if let Some(progress) = self.get_progress(&session_id).await {
                results.push(progress);
            }
        }
        results
    }

    /// Process session (internal method)
    async fn process_session(&self, session_id: String) {
        loop {
            let should_continue = {
                let sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get(&session_id) {
                    matches!(session.status, SequentialStatus::Running)
                        && session.current_index < session.texts.len()
                } else {
                    false
                }
            };

            if !should_continue {
                break;
            }

            if let Err(_) = self.process_next_entry(&session_id).await {
                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(&session_id) {
                    session.status = SequentialStatus::Error;
                }
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            if session.current_index >= session.texts.len() {
                session.status = SequentialStatus::Completed;
            }
        }
    }

    /// Process next entry in session
    async fn process_next_entry(&self, session_id: &str) -> Result<(), String> {
        let (entry_id, source_text, text_type) = {
            let sessions = self.active_sessions.lock().await;
            if let Some(session) = sessions.get(session_id) {
                if session.current_index >= session.texts.len() {
                    return Ok(());
                }
                let text = &session.texts[session.current_index];
                (text.id, text.source_text.clone(), text.text_type.clone())
            } else {
                return Err("Session not found".to_string());
            }
        };

        let translation_settings = self.get_translation_settings(session_id).await;

        let project_id = {
            let sessions = self.active_sessions.lock().await;
            if let Some(session) = sessions.get(session_id) {
                Some(session.project_id)
            } else {
                None
            }
        };

        let request = crate::translation::runpod::SingleTranslationRequest {
            source_text: source_text.clone(),
            source_language: translation_settings.source_language,
            target_language: translation_settings.target_language,
            context: None,
            model: translation_settings.model,
            project_id,
            text_type,
        };

        println!("🔤 [RunPod Translation] Entry {} - Source: \"{}\"", entry_id, source_text);

        let app_handle = {
            let sessions = self.active_sessions.lock().await;
            if let Some(session) = sessions.get(session_id) {
                session.app_handle.clone()
            } else {
                return Err("Session not found".to_string());
            }
        };

        match self.client.translate(&app_handle, request).await {
            Ok(result) => {
                println!("✅ [RunPod Translation] Entry {} - Translated: \"{}\"", entry_id, result.translated_text);

                let successful_translation = SuccessfulTranslation {
                    entry_id,
                    translated_text: result.translated_text.clone(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(session_id) {
                    session.processed_entries.insert(entry_id, true);
                    session.successful_translations.push(successful_translation);
                    session.current_index += 1;
                }
                Ok(())
            }
            Err(e) => {
                let error = SequentialError {
                    entry_id,
                    error_message: format!("Translation failed: {}", e),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(session_id) {
                    session.errors.push(error);
                    session.processed_entries.insert(entry_id, false);
                    session.current_index += 1;
                }
                Ok(())
            }
        }
    }

    /// Generate unique session ID
    async fn generate_session_id(&self) -> String {
        let counter = {
            let mut counter = self.session_counter.lock().await;
            let current = *counter;
            *counter += 1;
            current
        };
        format!("runpod_seq_{}", counter)
    }

    /// Get translation settings for a session (with defaults)
    async fn get_translation_settings(&self, session_id: &str) -> TranslationSettings {
        let sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get(session_id) {
            let mut settings = session.translation_settings.clone();

            if settings.source_language.is_none() {
                settings.source_language = Some(get_default_source_language());
            }
            if settings.target_language.is_none() {
                settings.target_language = Some(get_default_target_language());
            }
            if settings.model.is_none() {
                settings.model = Some(get_default_model());
            }

            settings
        } else {
            TranslationSettings {
                source_language: Some(get_default_source_language()),
                target_language: Some(get_default_target_language()),
                model: Some(get_default_model()),
            }
        }
    }
}

impl Clone for SequentialTranslationManager {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            active_sessions: Arc::clone(&self.active_sessions),
            session_counter: Arc::clone(&self.session_counter),
        }
    }
}

