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

/// # Sequential Translation Functions
///
/// These functions provide common logic for sequential translation operations.
/// They work with the common SequentialSession structure while allowing
/// provider-specific customizations through additional parameters.

/// Generate a unique session ID with provider-specific prefix
///
/// # Arguments
/// * `prefix` - Provider prefix (e.g., "seq_" for Ollama, "runpod_seq_" for RunPod)
/// * `counter` - Mutable reference to session counter for uniqueness
///
/// # Returns
/// * Unique session ID string
pub fn common_generate_session_id(prefix: &str, counter: &mut u64) -> String {
    let current = *counter;
    *counter += 1;
    format!("{}{}", prefix, current)
}

/// Get progress information from a sequential session
///
/// # Arguments
/// * `session` - Reference to the sequential session
///
/// # Returns
/// * SequentialProgress structure with current status
pub fn common_get_session_progress(session: &SequentialSession) -> SequentialProgress {
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

    SequentialProgress {
        session_id: session.session_id.clone(),
        current_entry,
        processed_count,
        total_count,
        status: session.status.clone(),
        estimated_time_remaining,
        errors: session.errors.clone(),
        successful_translations: session.successful_translations.clone(),
        pause_time_remaining: None, // Sera mis à jour par le gestionnaire de session
    }
}

/// Pause a sequential session
///
/// # Arguments
/// * `session` - Mutable reference to the sequential session
pub fn common_pause_session(session: &mut SequentialSession) {
    session.status = SequentialStatus::Paused;
}

/// Resume a sequential session
///
/// # Arguments
/// * `session` - Mutable reference to the sequential session
pub fn common_resume_session(session: &mut SequentialSession) {
    if matches!(session.status, SequentialStatus::Paused) {
        session.status = SequentialStatus::Running;
    }
}

/// Stop a sequential session
///
/// # Arguments
/// * `session` - Mutable reference to the sequential session
pub fn common_stop_session(session: &mut SequentialSession) {
    session.status = SequentialStatus::Idle;
}

/// Get translation settings with defaults applied
///
/// # Arguments
/// * `settings` - Current translation settings
/// * `get_default_source` - Function to get default source language
/// * `get_default_target` - Function to get default target language
/// * `get_default_model` - Function to get default model
///
/// # Returns
/// * TranslationSettings with defaults applied where None
pub fn common_get_translation_settings<F1, F2, F3>(
    mut settings: TranslationSettings,
    get_default_source: F1,
    get_default_target: F2,
    get_default_model: F3,
) -> TranslationSettings
where
    F1: Fn() -> String,
    F2: Fn() -> String,
    F3: Fn() -> String,
{
    if settings.source_language.is_none() {
        settings.source_language = Some(get_default_source());
    }
    if settings.target_language.is_none() {
        settings.target_language = Some(get_default_target());
    }
    if settings.model.is_none() {
        settings.model = Some(get_default_model());
    }
    settings
}
