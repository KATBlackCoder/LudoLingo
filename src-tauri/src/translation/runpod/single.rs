// Single translation logic for RunPod
// Thin wrapper that delegates to common translation functions

use crate::translation::common::functions::translate_single_common;
use crate::translation::common::types::{SingleTranslationRequest, SingleTranslationResult, TranslationSuggestion};
use crate::translation::runpod::RunPodClient;
use tauri::AppHandle;

/// Single translation manager for RunPod
#[derive(Clone)]
pub struct SingleTranslationManager {
    client: std::sync::Arc<RunPodClient>,
}

impl SingleTranslationManager {
    /// Create new single translation manager
    pub fn new(client: std::sync::Arc<RunPodClient>) -> Self {
        Self { client }
    }

    /// Translate a single text entry
    /// Delegates to the common translation function
    pub async fn translate(
        &self,
        app_handle: &AppHandle,
        request: SingleTranslationRequest,
    ) -> Result<SingleTranslationResult, String> {
        println!("🔍 [Rust] RunPod SingleTranslationManager::translate called");

        // Delegate to common translation function
        translate_single_common(&*self.client, request, app_handle).await
    }

    /// Get translation suggestions for a text
    pub async fn get_suggestions(
        &self,
        app_handle: Option<&AppHandle>,
        source_text: &str,
        context: Option<&str>,
        max_suggestions: usize,
    ) -> Result<Vec<TranslationSuggestion>, String> {
        let mut suggestions = Vec::new();

        let request = SingleTranslationRequest {
            source_text: source_text.to_string(),
            source_language: None,
            target_language: None,
            context: context.map(|s| s.to_string()),
            model: None,
            project_id: None,
            text_type: None,
        };

        if let Some(handle) = app_handle {
            match self.translate(handle, request.clone()).await {
                Ok(result) => {
                    suggestions.push(TranslationSuggestion {
                        suggestion: result.translated_text,
                        confidence: result.confidence.unwrap_or(0.8),
                        source: "runpod".to_string(),
                    });
                }
                Err(e) => {
                    return Err(format!("Failed to get RunPod suggestion: {}", e));
                }
            }
        } else {
            // No AppHandle - cannot get suggestions without glossary context
            return Err("AppHandle required for translation suggestions".to_string());
        }

        suggestions.truncate(max_suggestions);
        Ok(suggestions)
    }

}
