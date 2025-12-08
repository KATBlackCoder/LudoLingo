// Sequential translation logic for RunPod
// Thin wrapper that delegates to common sequential functions

use crate::translation::common::functions::{
    common_generate_session_id, common_get_session_progress, common_get_translation_settings,
    common_pause_session, common_resume_session, common_stop_session,
};
use crate::translation::common::types::*;
use crate::translation::runpod::{get_default_model, get_default_source_language, get_default_target_language, SingleTranslationManager};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;


/// RunPod-specific sequential session wrapper
/// Extends common SequentialSession with RunPod-specific fields
#[derive(Debug)]
pub struct RunPodSequentialSession {
    /// Common session data
    pub common: SequentialSession,
    /// RunPod-specific: App handle for glossary lookup
    pub app_handle: AppHandle,
    /// RunPod-specific: End time of current pause (for progress tracking)
    pub pause_end_time: Option<std::time::Instant>,
}

/// Sequential translation manager for RunPod
pub struct SequentialTranslationManager {
    client: Arc<SingleTranslationManager>,
    active_sessions: Arc<Mutex<HashMap<String, RunPodSequentialSession>>>,
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
        println!(
            "🔧 [RunPod Sequential] start_session called with {} texts",
            request.texts.len()
        );
        let session_id = self.generate_session_id().await;

        let session = RunPodSequentialSession {
            common: SequentialSession {
                session_id: session_id.clone(),
                project_id: request.project_id,
                texts: request.texts.clone(),
                current_index: request
                    .start_from
                    .map(|id| {
                        request
                            .texts
                            .iter()
                            .position(|text| text.id == id)
                            .unwrap_or(0)
                    })
                    .unwrap_or(0),
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
                pause_settings: request.pause_settings.unwrap_or_else(|| crate::translation::common::types::PauseSettings {
                    enabled: true,
                    batch_size: 150,
                    pause_duration_minutes: 5,
                }),
                batch_counter: 0,
            },
            app_handle,
            pause_end_time: None, // No pause active initially
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
            // Use common function but drain successful_translations first
            let successful_translations = session.common.successful_translations.drain(..).collect::<Vec<_>>();
            let mut progress = common_get_session_progress(&session.common);

            // Calculate remaining pause time if pause is active
            progress.pause_time_remaining = session.pause_end_time
                .and_then(|end_time| {
                    let now = std::time::Instant::now();
                    if now < end_time {
                        Some(end_time.duration_since(now).as_secs() as i64)
                    } else {
                        None
                    }
                });

            progress.successful_translations = successful_translations;
            progress
        })
    }

    /// Pause session
    pub async fn pause_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            common_pause_session(&mut session.common);
            Ok(())
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }

    /// Resume session
    pub async fn resume_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            common_resume_session(&mut session.common);

            // RunPod-specific: Restart processing in background
            let manager = Arc::new(self.clone());
            let session_id = session_id.to_string();
            tokio::spawn(async move {
                manager.process_session(session_id).await;
            });

            Ok(())
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }

    /// Stop session
    pub async fn stop_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            common_stop_session(&mut session.common);
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
                    matches!(session.common.status, SequentialStatus::Running)
                        && session.common.current_index < session.common.texts.len()
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
                    session.common.status = SequentialStatus::Error;
                }
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            if session.common.current_index >= session.common.texts.len() {
                session.common.status = SequentialStatus::Completed;
            }
        }
    }

    /// Process next entry in session
    async fn process_next_entry(&self, session_id: &str) -> Result<(), String> {
        let (entry_id, source_text, text_type) = {
            let sessions = self.active_sessions.lock().await;
            if let Some(session) = sessions.get(session_id) {
                if session.common.current_index >= session.common.texts.len() {
                    return Ok(());
                }
                let text = &session.common.texts[session.common.current_index];
                (text.id, text.source_text.clone(), text.text_type.clone())
            } else {
                return Err("Session not found".to_string());
            }
        };

        let translation_settings = self.get_translation_settings(session_id).await;

        let project_id = {
            let sessions = self.active_sessions.lock().await;
            if let Some(session) = sessions.get(session_id) {
                Some(session.common.project_id)
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

        println!(
            "🔤 [RunPod Translation] Entry {} - Source: \"{}\"",
            entry_id, source_text
        );

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
                println!(
                    "✅ [RunPod Translation] Entry {} - Translated: \"{}\"",
                    entry_id, result.translated_text
                );

                let successful_translation = SuccessfulTranslation {
                    entry_id,
                    translated_text: result.translated_text.clone(),
                    model_used: result.model_used,
                    timestamp: chrono::Utc::now().timestamp(),
                    processing_time_ms: result.processing_time_ms,
                };

                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(session_id) {
                    session.common.processed_entries.insert(entry_id, true);
                    session.common.successful_translations.push(successful_translation);
                    session.common.current_index += 1;

                    // Check if pause is enabled and batch size reached
                    session.common.batch_counter += 1;
                    if session.common.pause_settings.enabled &&
                       session.common.batch_counter >= session.common.pause_settings.batch_size as usize {

                        println!(
                            "⏸️ [RunPod Sequential] Batch of {} translations completed ({} total processed). Taking a {}-minute break to prevent overheating...",
                            session.common.pause_settings.batch_size,
                            session.common.processed_entries.len(),
                            session.common.pause_settings.pause_duration_minutes
                        );

                        // Set pause end time for progress tracking
                        let pause_duration = std::time::Duration::from_secs(
                            (session.common.pause_settings.pause_duration_minutes * 60) as u64
                        );
                        session.pause_end_time = Some(std::time::Instant::now() + pause_duration);

                        // Reset counter for next batch
                        session.common.batch_counter = 0;

                        // Release lock before sleeping
                        drop(sessions);

                        // Configurable pause duration
                        tokio::time::sleep(pause_duration).await;

                        // Clear pause end time after pause is complete
                        {
                            let mut sessions = self.active_sessions.lock().await;
                            if let Some(session) = sessions.get_mut(session_id) {
                                session.pause_end_time = None;
                            }
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                let error = SequentialError {
                    entry_id,
                    error_message: format!("Translation failed: {}", e),
                    timestamp: chrono::Utc::now().timestamp(),
                };

                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(session_id) {
                    session.common.errors.push(error);
                    session.common.processed_entries.insert(entry_id, false);
                    session.common.current_index += 1;
                }
                Ok(())
            }
        }
    }

    /// Generate unique session ID
    async fn generate_session_id(&self) -> String {
        let mut counter = self.session_counter.lock().await;
        common_generate_session_id("runpod_seq_", &mut *counter)
    }

    /// Get translation settings for a session (with defaults)
    async fn get_translation_settings(&self, session_id: &str) -> TranslationSettings {
        let sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get(session_id) {
            let settings = session.common.translation_settings.clone();

            common_get_translation_settings(
                settings,
                get_default_source_language,
                get_default_target_language,
                get_default_model,
            )
        } else {
            // Fallback defaults if session not found
            common_get_translation_settings(
                TranslationSettings {
                    source_language: None,
                    target_language: None,
                    model: None,
                },
                get_default_source_language,
                get_default_target_language,
                get_default_model,
            )
        }
    }
}

impl Clone for RunPodSequentialSession {
    fn clone(&self) -> Self {
        Self {
            common: self.common.clone(),
            app_handle: self.app_handle.clone(),
            pause_end_time: self.pause_end_time,
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
