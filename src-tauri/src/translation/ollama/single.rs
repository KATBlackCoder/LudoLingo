// Single translation logic
// Handles translation of individual text entries using Ollama with custom Modelfile
// Uses Chat mode to leverage Modelfile few-shot examples (MESSAGE user/assistant)
//
// The Modelfile (ludolingo.modelfile) contains:
// - SYSTEM instructions for professional game localization translation
// - Few-shot examples (MESSAGE user/assistant) that guide translation patterns
// - Optimized parameters (temperature, top_p, etc.) for consistent translations
//
// Chat mode is used instead of Completion mode to:
// - Leverage the few-shot examples in the Modelfile
// - Benefit from the SYSTEM instructions
// - Improve translation quality and consistency
//
// Usage: ollama create translation-model -f ludolingo.modelfile

use crate::translation::glossary::lookup_glossary_terms;
use crate::translation::ollama::{
    build_translation_prompt, get_default_model, get_translation_model_options,
    parse_translation_response, validate_translation_request, OllamaClient,
};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

// Import for Ollama API calls - Using Chat mode to leverage Modelfile few-shot examples
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};

/// Single translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTranslationRequest {
    pub source_text: String,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub context: Option<String>,
    pub model: Option<String>,
    pub project_id: Option<i64>, // For glossary lookup: None = global only, Some(id) = global + project-specific
    pub text_type: Option<String>, // Text type for category filtering: 'dialogue', 'system', 'item', 'skill', 'other'
}

/// Single translation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTranslationResult {
    pub translated_text: String,
    pub model_used: String,
    pub confidence: Option<f32>,
    pub processing_time_ms: u64,
}

/// Translation suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationSuggestion {
    pub suggestion: String,
    pub confidence: f32,
    pub source: String, // "ollama", "glossary", "similar"
}

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
    /// AppHandle is required for glossary lookup via Tauri events
    pub async fn translate(
        &self,
        app_handle: &AppHandle,
        request: SingleTranslationRequest,
    ) -> Result<SingleTranslationResult, String> {
        // Validate request
        validate_translation_request(&request.source_text)?;

        let start_time = std::time::Instant::now();

        // Lookup glossary terms for language pair
        // Behavior: ALWAYS retrieves global terms, IF project_id provided ALSO retrieves project-specific terms
        // Result: Combined global + project-specific terms (if project_id provided) or only global terms
        // IF text_type provided: FILTERS terms by category (mapped from text_type to category)
        // Special case: text_type 'general' → None (no filter, retrieves all terms including 'general' category)
        let source_lang = request.source_language.as_deref().unwrap_or("ja");
        let target_lang = request.target_language.as_deref().unwrap_or("fr");

        // Map text_type to category for glossary filtering
        // text_type 'general' → None (no filter, retrieves all terms)
        // category 'general' in glossary → always included regardless of filter (applies to all categories)
        let category =
            crate::translation::glossary::map_text_type_to_category(request.text_type.as_deref());

        let glossary_terms = match lookup_glossary_terms(
            app_handle,
            source_lang,
            target_lang,
            request.project_id,
            category,
        )
        .await
        {
            Ok(terms) => {
                log::debug!(
                    "Found {} glossary terms for {}-{}",
                    terms.len(),
                    source_lang,
                    target_lang
                );
                Some(terms)
            }
            Err(e) => {
                log::warn!(
                    "Failed to lookup glossary terms: {}, continuing without glossary",
                    e
                );
                None
            }
        };

        // Build prompt for Ollama with glossary terms
        let prompt = build_translation_prompt(
            &request.source_text,
            request.source_language.as_deref(),
            request.target_language.as_deref(),
            glossary_terms.as_deref(),
        );

        // Get model for both API call and result (clone to avoid move)
        let model = request.model.clone();

        // Call Ollama API
        let ollama_response = self.call_ollama_api(&prompt, request.model).await?;

        // Parse and clean response
        let translated_text = parse_translation_response(&ollama_response)?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(SingleTranslationResult {
            translated_text,
            model_used: model.unwrap_or_else(get_default_model),
            confidence: Some(0.8), // TODO: Implement actual confidence scoring
            processing_time_ms: processing_time,
        })
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
            // No AppHandle - build prompt without glossary
            let prompt = build_translation_prompt(
                &request.source_text,
                request.source_language.as_deref(),
                request.target_language.as_deref(),
                None, // No glossary terms
            );
            match self.call_ollama_api(&prompt, request.model.clone()).await {
                Ok(response) => match parse_translation_response(&response) {
                    Ok(translated) => {
                        suggestions.push(TranslationSuggestion {
                            suggestion: translated,
                            confidence: 0.8,
                            source: "ollama".to_string(),
                        });
                    }
                    Err(e) => {
                        return Err(format!("Failed to parse translation response: {}", e));
                    }
                },
                Err(e) => {
                    return Err(format!("Failed to get Ollama suggestion: {}", e));
                }
            }
        }

        // TODO: Add glossary suggestions
        // TODO: Add similar text suggestions

        // Limit to max_suggestions
        suggestions.truncate(max_suggestions);

        Ok(suggestions)
    }

    /// Validate translation request (wrapper for common validation)
    pub fn validate_request(&self, request: &SingleTranslationRequest) -> Result<(), String> {
        validate_translation_request(&request.source_text)
    }

    /// Call Ollama API using Chat mode
    /// Chat mode allows the Modelfile's few-shot examples (MESSAGE user/assistant) to be used
    /// This significantly improves translation quality by leveraging the examples in the Modelfile
    async fn call_ollama_api(&self, prompt: &str, model: Option<String>) -> Result<String, String> {
        // Get model name (use provided model or default)
        let model = model.unwrap_or_else(get_default_model);

        // Get model options matching the Modelfile parameters
        let options = get_translation_model_options();

        // Use Chat mode to leverage Modelfile few-shot examples
        // The Modelfile contains MESSAGE user/assistant examples that guide translation
        let mut history: Vec<ChatMessage> = Vec::new();
        let request = ChatMessageRequest::new(model, vec![ChatMessage::user(prompt.to_string())])
            .options(options);

        // Call Ollama API with Chat mode
        // Note: send_chat_messages_with_history takes &mut self, so we need to clone the client
        let mut client = self.client.inner().clone();
        match client
            .send_chat_messages_with_history(&mut history, request)
            .await
        {
            Ok(response) => Ok(response.message.content),
            Err(e) => Err(format!("Ollama API call failed: {}", e)),
        }
    }

    /// Estimate translation quality/confidence
    pub fn estimate_confidence(&self, source_text: &str, translated_text: &str) -> f32 {
        // Simple confidence estimation based on:
        // - Text length ratio (source vs translated)
        // - Presence of expected patterns
        // - TODO: Implement more sophisticated analysis

        let source_words = source_text.split_whitespace().count();
        let translated_words = translated_text.split_whitespace().count();

        if source_words == 0 {
            return 0.0;
        }

        let ratio = translated_words as f32 / source_words as f32;

        // Ideal ratio between 0.8 and 1.5 for good translations
        if (0.8..=1.5).contains(&ratio) {
            0.9
        } else if (0.5..=2.0).contains(&ratio) {
            0.7
        } else {
            0.4
        }
    }
}
