// Translation Provider Trait
// Defines the common interface for all translation providers

use tauri::AppHandle;

/// Configuration for a translation provider
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    /// Provider-specific identifier (pod_id for RunPod, session_id for others)
    pub id: Option<String>,
    /// Host for the provider (e.g., localhost for Ollama)
    pub host: Option<String>,
    /// Port for the provider (e.g., 11434 for Ollama)
    pub port: Option<u16>,
    /// Model to use for translation
    pub model: Option<String>,
    /// Source language for translation
    pub source_language: Option<String>,
    /// Target language for translation
    pub target_language: Option<String>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            id: None,
            host: Some("localhost".to_string()),
            port: Some(11434),
            model: None,
            source_language: None,
            target_language: None,
        }
    }
}

// Re-export types from common module for convenience
pub use crate::translation::common::types::{
    SequentialTranslationRequest, SingleTranslationRequest,
    SequentialProgress, SingleTranslationResult, TranslationSuggestion,
};

/// Common trait for all translation providers
///
/// This trait defines the interface that all translation providers must implement.
/// It provides a uniform way to interact with different translation providers (Ollama, RunPod, etc.)
/// without needing to know the specific implementation details.
///
/// # Example
///
/// ```rust,no_run
/// use crate::translation::provider::{TranslationProvider, ProviderConfig};
/// use crate::translation::factory::TranslationProviderFactory;
///
/// // Get a provider from the factory
/// let config = ProviderConfig::default();
/// let provider = TranslationProviderFactory::create_provider("ollama", config)?;
///
/// // Check provider status
/// let status = provider.check_status(config)?;
///
/// // Start a sequential translation
/// let session_id = provider.start_sequential_translation(app_handle, request)?;
/// ```
pub trait TranslationProvider: Send + Sync {
    /// Returns the name of the translation provider
    ///
    /// # Examples
    ///
    /// - Ollama: `"ollama"`
    /// - RunPod: `"runpod"`
    ///
    /// # Returns
    ///
    /// A string slice containing the provider name
    fn provider_name(&self) -> &str;

    /// Checks the status of the translation provider and returns detailed information
    ///
    /// This method verifies if the provider is available and functioning correctly.
    /// It returns detailed status information that can be used for debugging or user feedback.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the provider
    ///
    /// # Returns
    ///
    /// * `Ok(serde_json::Value)` - Status information as JSON
    /// * `Err(String)` - Error occurred during status check
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let status = provider.check_status(config)?;
    /// println!("Provider status: {:?}", status);
    /// ```
    fn check_status(&self, config: ProviderConfig) -> Result<serde_json::Value, String>;

    /// Starts a sequential translation session
    ///
    /// This method initiates a batch translation process that will translate multiple texts
    /// sequentially. The session can be monitored and controlled through other methods.
    ///
    /// # Arguments
    ///
    /// * `app` - Tauri app handle for event emission
    /// * `request` - Sequential translation request with texts and parameters
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Session ID for tracking the translation
    /// * `Err(String)` - Error occurred during session start
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let session_id = provider.start_sequential_translation(app, request)?;
    /// println!("Started translation session: {}", session_id);
    /// ```
    fn start_sequential_translation(
        &self,
        app: AppHandle,
        request: SequentialTranslationRequest,
    ) -> Result<String, String>;

    /// Gets the progress of a sequential translation session
    ///
    /// This method returns the current progress of an ongoing translation session,
    /// including completion status, errors, and successful translations.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session ID returned by start_sequential_translation
    ///
    /// # Returns
    ///
    /// * `Ok(Some(SequentialProgress))` - Current progress information
    /// * `Ok(None)` - Session not found
    /// * `Err(String)` - Error occurred during progress retrieval
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some(progress) = provider.get_sequential_progress(&session_id)? {
    ///     println!("Progress: {}/{}", progress.processed_count, progress.total_count);
    /// }
    /// ```
    fn get_sequential_progress(&self, session_id: &str) -> Result<Option<SequentialProgress>, String>;

    /// Pauses a sequential translation session
    ///
    /// This method temporarily stops an ongoing translation session.
    /// The session can be resumed later with resume_sequential_session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session ID to pause
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Session paused successfully
    /// * `Err(String)` - Error occurred during pause
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// provider.pause_sequential_session(&session_id)?;
    /// println!("Translation session paused");
    /// ```
    fn pause_sequential_session(&self, session_id: &str) -> Result<(), String>;

    /// Resumes a paused sequential translation session
    ///
    /// This method restarts a previously paused translation session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session ID to resume
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Session resumed successfully
    /// * `Err(String)` - Error occurred during resume
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// provider.resume_sequential_session(&session_id)?;
    /// println!("Translation session resumed");
    /// ```
    fn resume_sequential_session(&self, session_id: &str) -> Result<(), String>;

    /// Stops a sequential translation session
    ///
    /// This method permanently stops an ongoing translation session.
    /// The session cannot be resumed after being stopped.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session ID to stop
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Session stopped successfully
    /// * `Err(String)` - Error occurred during stop
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// provider.stop_sequential_session(&session_id)?;
    /// println!("Translation session stopped");
    /// ```
    fn stop_sequential_session(&self, session_id: &str) -> Result<(), String>;

    /// Translates a single text
    ///
    /// This method performs a one-off translation of a single text without
    /// creating a session. It's useful for quick translations or suggestions.
    ///
    /// # Arguments
    ///
    /// * `app` - Tauri app handle for event emission
    /// * `request` - Single translation request with text and parameters
    ///
    /// # Returns
    ///
    /// * `Ok(SingleTranslationResult)` - Translation result with text and metadata
    /// * `Err(String)` - Error occurred during translation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = provider.translate_single_text(app, request)?;
    /// println!("Translated: {}", result.translated_text);
    /// ```
    fn translate_single_text(
        &self,
        app: AppHandle,
        request: SingleTranslationRequest,
    ) -> Result<SingleTranslationResult, String>;

    /// Gets translation suggestions for a text
    ///
    /// This method provides multiple translation suggestions for a given text,
    /// which can be useful for quality control or alternative translations.
    ///
    /// # Arguments
    ///
    /// * `app` - Tauri app handle for event emission
    /// * `source_text` - The text to get suggestions for
    /// * `context` - Optional context for better suggestions
    /// * `count` - Number of suggestions to return
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<TranslationSuggestion>)` - List of translation suggestions
    /// * `Err(String)` - Error occurred during suggestion generation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let suggestions = provider.get_translation_suggestions(app, "Hello", Some("Greeting"), 3)?;
    /// for suggestion in suggestions {
    ///     println!("{} (confidence: {})", suggestion.suggestion, suggestion.confidence);
    /// }
    /// ```
    fn get_translation_suggestions(
        &self,
        app: AppHandle,
        source_text: &str,
        context: Option<&str>,
        count: usize,
    ) -> Result<Vec<TranslationSuggestion>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Mock implementation for testing
    struct MockProvider;

    impl TranslationProvider for MockProvider {
        fn provider_name(&self) -> &str {
            "mock"
        }

        fn check_status(&self, _config: ProviderConfig) -> Result<serde_json::Value, String> {
            Ok(serde_json::json!({"status": "ok"}))
        }

        fn start_sequential_translation(
            &self,
            _app: AppHandle,
            _request: SequentialTranslationRequest,
        ) -> Result<String, String> {
            Ok("session_123".to_string())
        }

        fn get_sequential_progress(&self, _session_id: &str) -> Result<Option<SequentialProgress>, String> {
            Ok(None)
        }

        fn pause_sequential_session(&self, _session_id: &str) -> Result<(), String> {
            Ok(())
        }

        fn resume_sequential_session(&self, _session_id: &str) -> Result<(), String> {
            Ok(())
        }

        fn stop_sequential_session(&self, _session_id: &str) -> Result<(), String> {
            Ok(())
        }

        fn translate_single_text(
            &self,
            _app: AppHandle,
            _request: SingleTranslationRequest,
        ) -> Result<SingleTranslationResult, String> {
            // Mock implementation - in real providers, this would use the AppHandle
            Ok(SingleTranslationResult {
                translated_text: "translated".to_string(),
                model_used: "mock".to_string(),
                confidence: Some(1.0),
                processing_time_ms: 100,
            })
        }

        fn get_translation_suggestions(
            &self,
            _app: AppHandle,
            _source_text: &str,
            _context: Option<&str>,
            _count: usize,
        ) -> Result<Vec<TranslationSuggestion>, String> {
            // Mock implementation - in real providers, this would use the AppHandle
            Ok(vec![TranslationSuggestion {
                suggestion: "suggestion".to_string(),
                confidence: 0.9,
                source: "mock".to_string(),
            }])
        }
    }

    #[test]
    fn test_trait_can_be_used_as_return_type() {
        // Test that TranslationProvider can be used as Box<dyn TranslationProvider>
        let provider: Box<dyn TranslationProvider> = Box::new(MockProvider);
        assert_eq!(provider.provider_name(), "mock");
    }

    #[test]
    fn test_trait_is_send_sync() {
        // Test that TranslationProvider is Send + Sync
        let provider = Arc::new(MockProvider);
        let provider_clone = Arc::clone(&provider);

        // This would fail to compile if TranslationProvider wasn't Send + Sync
        std::thread::spawn(move || {
            assert_eq!(provider_clone.provider_name(), "mock");
        });
    }

    #[test]
    fn test_provider_config_default() {
        let config = ProviderConfig::default();
        assert_eq!(config.host, Some("localhost".to_string()));
        assert_eq!(config.port, Some(11434));
        assert_eq!(config.id, None);
        assert_eq!(config.model, None);
        assert_eq!(config.source_language, None);
        assert_eq!(config.target_language, None);
    }

    #[test]
    fn test_mock_provider_implementation() {
        let provider = MockProvider;

        // Test provider_name
        assert_eq!(provider.provider_name(), "mock");

        // Test check_status
        let config = ProviderConfig::default();
        let status = provider.check_status(config).unwrap();
        assert_eq!(status["status"], "ok");

        // Note: translate_single_text and get_translation_suggestions require AppHandle
        // which is not available in unit tests. These are tested through integration tests
        // or by the actual provider implementations.
    }
}
