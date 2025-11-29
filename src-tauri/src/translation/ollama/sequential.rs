// Sequential translation logic
// Handles translation of text entries one by one using Ollama
// More realistic approach given Ollama's resource constraints

use crate::translation::ollama::{
    get_default_model, get_default_source_language, get_default_target_language,
    SingleTranslationManager,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

/// Translation text with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationText {
    pub id: i32,
    pub source_text: String,
    pub context: Option<String>,
    pub text_type: Option<String>, // Text type for category filtering: 'dialogue', 'system', 'item', 'skill', 'other'
}

/// Sequential translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequentialTranslationRequest {
    pub project_id: i64,
    pub texts: Vec<TranslationText>,
    pub start_from: Option<i32>,         // Resume from specific entry
    pub source_language: Option<String>, // Override project default
    pub target_language: Option<String>, // Override project default
    pub model: Option<String>,           // Override default model
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
    pub app_handle: AppHandle,                     // Required for glossary lookup
    pub batch_counter: usize,                      // Count translations in current batch (pause after 500)
}

/// Sequential translation manager
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
    /// AppHandle is required for glossary lookup during translations
    pub async fn start_session(
        &self,
        app_handle: AppHandle,
        request: SequentialTranslationRequest,
    ) -> Result<String, String> {
        println!(
            "🔧 [Sequential] start_session called with {} texts",
            request.texts.len()
        );
        println!("🔧 [Sequential] Request settings - source_language: {:?}, target_language: {:?}, model: {:?}", request.source_language, request.target_language, request.model);
        let session_id = self.generate_session_id().await;
        println!("🆔 [Sequential] Generated session_id: {}", session_id);

        let session = SequentialSession {
            session_id: session_id.clone(),
            project_id: request.project_id,
            texts: request.texts.clone(),
            current_index: request
                .start_from
                .map(|id| {
                    // Find index of text to resume from
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
            app_handle,
            batch_counter: 0, // Initialize batch counter for pause management
        };

        // Store session
        println!("💾 [Sequential] Storing session in HashMap");
        {
            let mut sessions = self.active_sessions.lock().await;
            sessions.insert(session_id.clone(), session);
            println!(
                "✅ [Sequential] Session stored, total sessions: {}",
                sessions.len()
            );
        }

        // Start processing in background
        println!("🚀 [Sequential] Starting background processing task");
        let manager = Arc::new(self.clone());
        let session_id_clone = session_id.clone();
        tokio::spawn(async move {
            println!(
                "⚙️ [Sequential] Background task started for session: {}",
                session_id_clone
            );
            manager.process_session(session_id_clone).await;
        });

        println!("✅ [Sequential] start_session completed successfully");
        Ok(session_id)
    }

    /// Get session progress
    pub async fn get_progress(&self, session_id: &str) -> Option<SequentialProgress> {
        let mut sessions = self.active_sessions.lock().await;
        sessions.get_mut(session_id).map(|session| {
            let total_count = session.texts.len() as i32;
            let processed_count = session.processed_entries.len() as i32;
            let current_entry = session.texts.get(session.current_index).map(|text| text.id);

            // Estimate remaining time (rough calculation: 3 seconds per entry)
            let avg_time_per_entry = 3.0; // seconds
            let remaining_entries = total_count - processed_count;
            let estimated_time_remaining = if processed_count > 0 {
                Some((remaining_entries as f64 * avg_time_per_entry) as i64)
            } else {
                None
            };

            // Get successful translations and clear them (consume them)
            let successful_translations = session
                .successful_translations
                .drain(..)
                .collect::<Vec<_>>();

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

                // Restart processing
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

            // Process next entry
            let translation_successful = self.process_next_entry(&session_id).await.is_ok();
            
            if !translation_successful {
                // Mark session as error if processing fails
                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(&session_id) {
                    session.status = SequentialStatus::Error;
                }
                break;
            }

            // Increment batch counter and check for pause after successful translation
            {
                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(&session_id) {
                    session.batch_counter += 1;
                    
                    // Pause after every 500 translations to prevent overheating
                    if session.batch_counter >= 500 {
                        println!(
                            "⏸️ [Sequential] Batch of 500 translations completed ({} total processed). Taking a 12-minute break to prevent overheating...",
                            session.processed_entries.len()
                        );
                        
                        // Reset counter for next batch
                        session.batch_counter = 0;
                        
                        // Release lock before sleeping
                        drop(sessions);
                        
                        // 12-minute pause (between 10-15 min as requested)
                        tokio::time::sleep(tokio::time::Duration::from_secs(720)).await;
                        
                        println!("▶️ [Sequential] Break over, resuming translations...");
                    }
                }
            }

            // Small delay between translations to prevent overwhelming Ollama
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        // Mark as completed when done
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
                    return Ok(()); // No more entries
                }
                let text = &session.texts[session.current_index];
                (text.id, text.source_text.clone(), text.text_type.clone())
            } else {
                return Err("Session not found".to_string());
            }
        };

        // Get translation settings (from request or project defaults)
        let translation_settings = self.get_translation_settings(session_id).await;

        // Get project_id from session for glossary lookup
        // Glossary lookup behavior: ALWAYS retrieves global terms, IF project_id provided ALSO retrieves project-specific terms
        let project_id = {
            let sessions = self.active_sessions.lock().await;
            if let Some(session) = sessions.get(session_id) {
                Some(session.project_id)
            } else {
                None
            }
        };

        // Create translation request with real text from DB
        // project_id is passed to glossary lookup: if Some(id), combines global + project-specific terms
        // if None, retrieves only global terms
        // text_type is passed to glossary lookup: mapped to category for filtering glossary terms
        let request = crate::translation::ollama::SingleTranslationRequest {
            source_text: source_text.clone(),
            source_language: translation_settings.source_language,
            target_language: translation_settings.target_language,
            context: None, // Could be filled from DB if needed
            model: translation_settings.model,
            project_id, // Pass project_id for glossary lookup (combines global + project-specific if provided)
            text_type,  // Pass text_type for category filtering in glossary lookup
        };

        // Log source text before translation
        println!(
            "🔤 [Translation] Entry {} - Source: \"{}\"",
            entry_id, source_text
        );

        // Get AppHandle from session for glossary lookup
        let app_handle = {
            let sessions = self.active_sessions.lock().await;
            if let Some(session) = sessions.get(session_id) {
                session.app_handle.clone()
            } else {
                return Err("Session not found".to_string());
            }
        };

        // Translate using single manager with glossary support
        match self.client.translate(&app_handle, request).await {
            Ok(result) => {
                println!(
                    "✅ [Translation] Entry {} - Source: \"{}\" → Translated: \"{}\"",
                    entry_id, source_text, result.translated_text
                );

                // Create successful translation record
                let successful_translation = SuccessfulTranslation {
                    entry_id,
                    translated_text: result.translated_text.clone(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                // Mark as processed and store successful translation
                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(session_id) {
                    session.processed_entries.insert(entry_id, true);
                    session.successful_translations.push(successful_translation);
                    session.current_index += 1;
                }
                Ok(())
            }
            Err(e) => {
                // Record translation error
                let error = SequentialError {
                    entry_id,
                    error_message: format!("Translation failed: {}", e),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                let mut sessions = self.active_sessions.lock().await;
                if let Some(session) = sessions.get_mut(session_id) {
                    session.errors.push(error);
                    session.processed_entries.insert(entry_id, false);
                    session.current_index += 1; // Continue to next even on error
                }
                Ok(()) // Don't fail the whole session on single entry error
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
        format!("seq_{}", counter)
    }

    /// Get translation settings for a session (with defaults)
    async fn get_translation_settings(&self, session_id: &str) -> TranslationSettings {
        let sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get(session_id) {
            let mut settings = session.translation_settings.clone();

            println!("📋 [Sequential] Session {} translation_settings - source_language: {:?}, target_language: {:?}, model: {:?}",
                     session_id, settings.source_language, settings.target_language, settings.model);

            // Apply defaults if not specified
            if settings.source_language.is_none() {
                settings.source_language = Some(get_default_source_language());
                println!(
                    "⚠️ [Sequential] Applied default source_language '{}' for session {}",
                    settings.source_language.as_ref().unwrap(),
                    session_id
                );
            }
            if settings.target_language.is_none() {
                settings.target_language = Some(get_default_target_language());
                println!(
                    "⚠️ [Sequential] Applied default target_language '{}' for session {}",
                    settings.target_language.as_ref().unwrap(),
                    session_id
                );
            }
            if settings.model.is_none() {
                settings.model = Some(get_default_model());
                println!(
                    "⚠️ [Sequential] Applied default model '{}' for session {}",
                    settings.model.as_ref().unwrap(),
                    session_id
                );
            }

            println!("✅ [Sequential] Final settings for session {} - source_language: {:?}, target_language: {:?}, model: {:?}",
                     session_id, settings.source_language, settings.target_language, settings.model);

            settings
        } else {
            // Fallback defaults if session not found
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
