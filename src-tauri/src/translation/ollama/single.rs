// Single translation logic
// Handles translation of individual text entries using Ollama with custom Modelfile
// Ultra-simple prompt: "Translate from {source} to {target}: {text}"
//
// Example Modelfile for translation:
// FROM llama3.2:3b
// PARAMETER temperature 0.3
// SYSTEM """You are a professional translator. Translate the given text accurately and naturally.
// Respond only with the translated text, no explanations or additional content."""
//
// Usage: ollama create translation-model -f Modelfile

use crate::translation::ollama::OllamaClient;
use serde::{Deserialize, Serialize};

// Import for Ollama API calls
use ollama_rs::generation::completion::request::GenerationRequest;

/// Single translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTranslationRequest {
    pub source_text: String,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub context: Option<String>,
    pub model: Option<String>,
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
    pub async fn translate(&self, request: SingleTranslationRequest) -> Result<SingleTranslationResult, String> {
        let start_time = std::time::Instant::now();

        // Build prompt for Ollama
        let prompt = self.build_translation_prompt(&request)?;

        // Get model for both API call and result (clone to avoid move)
        let model = request.model.clone();

        // Call Ollama API
        let ollama_response = self.call_ollama_api(&prompt, request.model).await?;

        // Parse and clean response
        let translated_text = self.parse_translation_response(&ollama_response)?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(SingleTranslationResult {
            translated_text,
            model_used: model.unwrap_or_else(|| "llama2".to_string()),
            confidence: Some(0.8), // TODO: Implement actual confidence scoring
            processing_time_ms: processing_time,
        })
    }

    /// Get translation suggestions for a text
    pub async fn get_suggestions(
        &self,
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
        };

        match self.translate(request).await {
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

        // TODO: Add glossary suggestions
        // TODO: Add similar text suggestions

        // Limit to max_suggestions
        suggestions.truncate(max_suggestions);

        Ok(suggestions)
    }

    /// Validate translation request
    pub fn validate_request(&self, request: &SingleTranslationRequest) -> Result<(), String> {
        if request.source_text.trim().is_empty() {
            return Err("Source text cannot be empty".to_string());
        }

        if request.source_text.len() > 10000 {
            return Err("Source text is too long (max 10000 characters)".to_string());
        }

        Ok(())
    }

    /// Build translation prompt for Ollama (ultra simple for custom Modelfile)
    fn build_translation_prompt(&self, request: &SingleTranslationRequest) -> Result<String, String> {
        let source_lang = request.source_language.as_deref().unwrap_or("ja");
        let target_lang = request.target_language.as_deref().unwrap_or("fr");

        // Ultra simple prompt - Modelfile handles all the complex formatting
        // Format: "Translate from {source} to {target}: {text}"
        let prompt = format!("Translate from {} to {}: {}", source_lang, target_lang, request.source_text);

        Ok(prompt)
    }

    /// Call Ollama API using the client
    async fn call_ollama_api(&self, prompt: &str, model: Option<String>) -> Result<String, String> {
        // Get model name (use provided model or default)
        let model = model.unwrap_or_else(|| "llama3.2:3b".to_string());

        // Create generation request
        let request = GenerationRequest::new(model, prompt.to_string());

        // Call Ollama API
        match self.client.inner().generate(request).await {
            Ok(response) => Ok(response.response),
            Err(e) => Err(format!("Ollama API call failed: {}", e)),
        }
    }

    /// Parse translation response from Ollama
    fn parse_translation_response(&self, response: &str) -> Result<String, String> {
        // Clean up the response
        let translated = response.trim();

        if translated.is_empty() {
            return Err("Empty translation response".to_string());
        }

        // Remove common artifacts
        let cleaned = translated
            .replace("Translation:", "")
            .replace("Traduction:", "")
            .trim()
            .to_string();

        Ok(cleaned)
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
