// Single translation logic for Ollama
// Thin wrapper that delegates to common translation functions
// Uses Ollama-specific API calls with the generic translation framework

use crate::translation::common::functions::translate_single_common;
use crate::translation::common::types::{SingleTranslationRequest, SingleTranslationResult, TranslationSuggestion};
use crate::translation::ollama::OllamaClient;
use tauri::AppHandle;

/// Single translation manager
#[derive(Clone)]
pub struct SingleTranslationManager {
    client: std::sync::Arc<OllamaClient>,
}

impl SingleTranslationManager {
    /// Create new single translation manager
    pub fn new(client: std::sync::Arc<OllamaClient>) -> Self {
        Self { client }
    }

    /// Translate a single text entry
    /// Delegates to the common translation function
    pub async fn translate(
        &self,
        app_handle: &AppHandle,
        request: SingleTranslationRequest,
    ) -> Result<SingleTranslationResult, String> {
        // Delegate to common translation function
        translate_single_common(&*self.client, request, app_handle).await
    }

    /// Get translation suggestions for a text
    /// AppHandle is optional - if provided, glossary terms will be used
    pub async fn get_suggestions(
        &self,
        app_handle: Option<&AppHandle>,
        source_text: &str,
        context: Option<&str>,
        max_suggestions: usize,
    ) -> Result<Vec<TranslationSuggestion>, String> {
        let mut suggestions = Vec::new();

        // Primary suggestion from Ollama
        let request = SingleTranslationRequest {
            source_text: source_text.to_string(),
            source_language: None,
            target_language: None,
            context: context.map(|s| s.to_string()),
            model: None,
            project_id: None, // Suggestions don't have project context, use global terms only
            text_type: None,  // Suggestions don't have text_type context, no category filtering
        };

        // Translate with glossary if AppHandle is provided
        // If no AppHandle, build prompt without glossary (for backward compatibility)
        if let Some(handle) = app_handle {
            // Use translate method which includes glossary lookup
            match self.translate(handle, request.clone()).await {
                Ok(result) => {
                    suggestions.push(TranslationSuggestion {
                        suggestion: result.translated_text,
                        confidence: result.confidence.unwrap_or(0.8),
                        source: "ollama".to_string(),
                    });
                }
                Err(e) => {
                    return Err(format!("Failed to get Ollama suggestion: {}", e));
                }
            }
        } else {
            // No AppHandle - cannot get suggestions without glossary context
            return Err("AppHandle required for translation suggestions".to_string());
        }

        // TODO: Add glossary suggestions
        // TODO: Add similar text suggestions

        // Limit to max_suggestions
        suggestions.truncate(max_suggestions);

        Ok(suggestions)
    }
}
