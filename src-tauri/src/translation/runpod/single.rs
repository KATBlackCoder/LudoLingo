// Single translation logic for RunPod
// Handles translation of individual text entries using RunPod Ollama via HTTP
// Uses Chat mode to leverage Modelfile few-shot examples

use crate::translation::runpod::{
    build_translation_prompt, get_default_model,
    parse_translation_response, validate_translation_request, RunPodClient,
};
use crate::translation::glossary::lookup_glossary_terms;
use crate::translation::runpod::client::{ChatMessage, ChatOptions};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

/// Single translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTranslationRequest {
    pub source_text: String,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub context: Option<String>,
    pub model: Option<String>,
    pub project_id: Option<i64>,  // For glossary lookup: None = global only, Some(id) = global + project-specific
    pub text_type: Option<String>,  // Text type for category filtering: 'dialogue', 'system', 'item', 'skill', 'other'
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
    pub source: String, // "runpod", "glossary", "similar"
}

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
    /// AppHandle is required for glossary lookup via Tauri events
    pub async fn translate(
        &self,
        app_handle: &AppHandle,
        request: SingleTranslationRequest,
    ) -> Result<SingleTranslationResult, String> {
        println!("🔍 [Rust] SingleTranslationManager::translate called");
        println!("🔍 [Rust] Source text length: {} chars", request.source_text.len());
        println!("🔍 [Rust] Model: {:?}", request.model);
        
        // Validate request
        validate_translation_request(&request.source_text)?;

        let start_time = std::time::Instant::now();

        // Lookup glossary terms for language pair
        let source_lang = request.source_language.as_deref().unwrap_or("ja");
        let target_lang = request.target_language.as_deref().unwrap_or("fr");
        
        let category = crate::translation::glossary::map_text_type_to_category(request.text_type.as_deref());
        
        let glossary_terms = match lookup_glossary_terms(app_handle, source_lang, target_lang, request.project_id, category).await {
            Ok(terms) => {
                log::debug!("Found {} glossary terms for {}-{}", terms.len(), source_lang, target_lang);
                Some(terms)
            }
            Err(e) => {
                log::warn!("Failed to lookup glossary terms: {}, continuing without glossary", e);
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
        
        println!("🔍 [Rust] Using model: {:?}", model);
        println!("🔍 [Rust] Prompt built, length: {} chars", prompt.len());

        // Call RunPod API
        let runpod_response = self.call_runpod_api(&prompt, request.model).await?;
        
        println!("🔍 [Rust] RunPod response received, length: {} chars", runpod_response.len());

        // Parse and clean response
        let translated_text = parse_translation_response(&runpod_response)?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(SingleTranslationResult {
            translated_text,
            model_used: model.unwrap_or_else(get_default_model),
            confidence: Some(0.8), // TODO: Implement actual confidence scoring
            processing_time_ms: processing_time,
        })
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
            let prompt = build_translation_prompt(
                &request.source_text,
                request.source_language.as_deref(),
                request.target_language.as_deref(),
                None,
            );
            match self.call_runpod_api(&prompt, request.model.clone()).await {
                Ok(response) => {
                    match parse_translation_response(&response) {
                        Ok(translated) => {
                            suggestions.push(TranslationSuggestion {
                                suggestion: translated,
                                confidence: 0.8,
                                source: "runpod".to_string(),
                            });
                        }
                        Err(e) => {
                            return Err(format!("Failed to parse translation response: {}", e));
                        }
                    }
                }
                Err(e) => {
                    return Err(format!("Failed to get RunPod suggestion: {}", e));
                }
            }
        }

        suggestions.truncate(max_suggestions);
        Ok(suggestions)
    }

    /// Validate translation request
    pub fn validate_request(&self, request: &SingleTranslationRequest) -> Result<(), String> {
        validate_translation_request(&request.source_text)
    }

    /// Call RunPod API using Chat mode
    async fn call_runpod_api(&self, prompt: &str, model: Option<String>) -> Result<String, String> {
        // Get model: use provided model if valid, or fetch first available model from RunPod
        let model = if let Some(m) = model {
            if m.is_empty() {
                // Model is empty string, fetch first available model
                println!("⚠️ [Rust] Model is empty, fetching first available model from RunPod");
                self.get_first_available_model().await?
            } else {
                // Check if model exists on RunPod
                match self.validate_model_exists(&m).await {
                    Ok(true) => {
                        println!("✅ [Rust] Model '{}' exists on RunPod", m);
                        m
                    },
                    Ok(false) => {
                        println!("⚠️ [Rust] Model '{}' not found on RunPod, using first available model", m);
                        self.get_first_available_model().await?
                    },
                    Err(e) => {
                        println!("⚠️ [Rust] Failed to validate model '{}': {}, using first available model", m, e);
                        self.get_first_available_model().await?
                    }
                }
            }
        } else {
            // Model is None, fetch first available model
            println!("⚠️ [Rust] Model is None, fetching first available model from RunPod");
            self.get_first_available_model().await?
        };
        
        println!("🔍 [Rust] call_runpod_api called with model: {}", model);
        println!("🔍 [Rust] Prompt length: {} chars", prompt.len());

        // Create chat options matching Modelfile parameters
        let options = ChatOptions {
            temperature: Some(0.15),
            top_p: Some(0.85),
            top_k: Some(40),
            repeat_penalty: Some(1.15),
            repeat_last_n: Some(64),
        };

        // Use Chat mode
        let messages = vec![ChatMessage::user(prompt.to_string())];

        match self.client.chat(&model, messages, Some(options)).await {
            Ok(response) => {
                println!("✅ [Rust] RunPod API call successful, response length: {} chars", response.message.content.len());
                Ok(response.message.content)
            },
            Err(e) => {
                println!("❌ [Rust] RunPod API call failed: {}", e);
                Err(format!("RunPod API call failed: {}", e))
            },
        }
    }

    /// Validate if a model exists on RunPod
    async fn validate_model_exists(&self, model_name: &str) -> Result<bool, String> {
        match self.client.list_models().await {
            Ok(models) => {
                let exists = models.iter().any(|m| m.name == model_name);
                Ok(exists)
            },
            Err(e) => Err(format!("Failed to list RunPod models: {}", e))
        }
    }

    /// Get first available model from RunPod
    async fn get_first_available_model(&self) -> Result<String, String> {
        match self.client.list_models().await {
            Ok(models) => {
                if models.is_empty() {
                    Err("No models available on RunPod. Please ensure at least one model is loaded.".to_string())
                } else {
                    let first_model = models[0].name.clone();
                    println!("✅ [Rust] Using first available model: {}", first_model);
                    Ok(first_model)
                }
            },
            Err(e) => Err(format!("Failed to list RunPod models: {}", e))
        }
    }

    /// Estimate translation quality/confidence
    pub fn estimate_confidence(&self, source_text: &str, translated_text: &str) -> f32 {
        let source_words = source_text.split_whitespace().count();
        let translated_words = translated_text.split_whitespace().count();

        if source_words == 0 {
            return 0.0;
        }

        let ratio = translated_words as f32 / source_words as f32;

        if (0.8..=1.5).contains(&ratio) {
            0.9
        } else if (0.5..=2.0).contains(&ratio) {
            0.7
        } else {
            0.4
        }
    }
}

