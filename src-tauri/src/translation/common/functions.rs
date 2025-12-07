//! # Translation Architecture
//!
//! This module provides the core translation functionality that works across all providers (Ollama, RunPod, etc.).
//! The architecture is designed to eliminate code duplication while maintaining provider-specific optimizations.
//!
//! ## Design Principles
//!
//! - **DRY (Don't Repeat Yourself)**: Common logic is centralized in this module
//! - **Provider Agnostic**: Translation functions work with any provider implementing `TranslationClient`
//! - **Zero External Dependencies**: Uses only Rust standard library futures
//! - **Type Safety**: Strong typing ensures compile-time correctness
//!
//! ## Adding New Translation Providers
//!
//! To add a new translation provider:
//! 1. Create a new module under `src-tauri/src/translation/` (e.g., `openai/`)
//! 2. Implement the `TranslationClient` trait for your client struct
//! 3. Use the common functions (`translate_single_common`) in your manager implementations
//! 4. Export your types through your `mod.rs`
//!
//! ```rust,ignore
//! use crate::translation::common::functions::TranslationClient;
//!
//! impl TranslationClient for MyProviderClient {
//!     fn call_api(&self, prompt: &str, model: Option<String>) -> /* ... */ {
//!         // Your provider-specific API call
//!     }
//!
//!     fn list_models(&self) -> /* ... */ {
//!         // List available models for your provider
//!     }
//!
//!     fn test_connection(&self) -> /* ... */ {
//!         // Test connectivity to your provider
//!     }
//! }
//! ```

use crate::translation::common::types::*;
use crate::translation::glossary::lookup_glossary_terms;
use crate::translation::ollama::common::{build_translation_prompt, parse_translation_response, validate_translation_request};
use tauri::AppHandle;

/// # Translation Client Trait
///
/// This trait defines the interface that all translation providers must implement.
/// It abstracts the differences between providers (Ollama, RunPod, OpenAI, etc.) while
/// providing a consistent interface for the common translation functions.
///
/// ## Required Methods
///
/// - `call_api`: Execute a translation request with the provider
/// - `list_models`: Get available models from the provider
/// - `test_connection`: Verify provider connectivity
///
/// ## Implementation Notes
///
/// - All methods return futures for async operations
/// - Error handling uses `Result<T, String>` for simplicity
/// - Model names are strings for maximum flexibility
/// - Connection testing should be lightweight (e.g., list models)
pub trait TranslationClient: Send + Sync + 'static {
    /// Execute a translation API call with the given prompt and optional model
    ///
    /// # Arguments
    /// * `prompt` - The formatted prompt containing the text to translate
    /// * `model` - Optional model name override (uses provider default if None)
    ///
    /// # Returns
    /// * `Result<String, String>` - Translated text or error message
    ///
    /// # Example
    /// ```rust,ignore
    /// let result = client.call_api("Translate 'Hello' to French", Some("llama2")).await?;
    /// assert_eq!(result, "Bonjour");
    /// ```
    fn call_api(&self, prompt: &str, model: Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + '_>>;

    /// List all available models for this provider
    ///
    /// # Returns
    /// * `Result<Vec<String>, String>` - List of model names or error
    ///
    /// # Example
    /// ```rust,ignore
    /// let models = client.list_models().await?;
    /// assert!(models.contains(&"llama2".to_string()));
    /// ```
    fn list_models(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<String>, String>> + Send + '_>>;

    /// Test connectivity to the translation provider
    ///
    /// This should perform a lightweight operation (like listing models)
    /// to verify the provider is accessible and responding.
    ///
    /// # Returns
    /// * `Result<(), String>` - Success or error message
    ///
    /// # Example
    /// ```rust,ignore
    /// client.test_connection().await?;
    /// println!("Provider is accessible!");
    /// ```
    fn test_connection(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + '_>>;
}

/// Common single translation function that works with any client
pub async fn translate_single_common<T: TranslationClient>(
    client: &T,
    request: SingleTranslationRequest,
    app_handle: &AppHandle,
) -> Result<SingleTranslationResult, String> {
    // Validate request using common validation
    validate_translation_request(&request.source_text)?;

    let start_time = std::time::Instant::now();

    // Lookup glossary terms (common logic)
    let source_lang = request.source_language.as_deref().unwrap_or("ja");
    let target_lang = request.target_language.as_deref().unwrap_or("fr");

    let category = crate::translation::glossary::map_text_type_to_category(request.text_type.as_deref());

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

    // Build prompt using common prompt builder
    let prompt = build_translation_prompt(
        &request.source_text,
        request.source_language.as_deref(),
        request.target_language.as_deref(),
        glossary_terms.as_deref(),
    );

    // Get model (clone to avoid move)
    let model = request.model.clone();

    // Call the client's API (provider-specific)
    let response = client.call_api(&prompt, model.clone()).await?;

    // Parse response using common parser
    let translated_text = parse_translation_response(&response)?;

    let processing_time = start_time.elapsed().as_millis() as u64;

    Ok(SingleTranslationResult {
        translated_text,
        model_used: model.unwrap_or_else(|| "ludolingo:latest".to_string()),
        confidence: Some(0.8), // TODO: Implement actual confidence scoring
        processing_time_ms: processing_time,
    })
}

// Note: Sequential functions are kept in individual managers
// as they have provider-specific session structures and logic.
// Only the single translation function is truly common across providers.
