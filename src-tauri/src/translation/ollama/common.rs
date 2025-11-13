// Common utilities and constants for Ollama translation module
// Centralizes shared logic to avoid duplication (DRY principle)

/// Default translation model
pub const DEFAULT_MODEL: &str = "llama3.2:3b";

/// Default source language (Japanese)
pub const DEFAULT_SOURCE_LANGUAGE: &str = "ja";

/// Default target language (French)
pub const DEFAULT_TARGET_LANGUAGE: &str = "fr";

/// Maximum text length for translation (characters)
pub const MAX_TEXT_LENGTH: usize = 10000;

/// Build translation prompt for Ollama
/// Simple prompt format: "Translate from {source} to {target}: {text}"
/// The Modelfile (ludolingo.modelfile) handles:
/// - SYSTEM instructions for professional translation
/// - Few-shot examples (MESSAGE user/assistant) via Chat mode
/// - Placeholder preservation rules
/// - Adult content handling
pub fn build_translation_prompt(
    source_text: &str,
    source_language: Option<&str>,
    target_language: Option<&str>,
) -> String {
    let source_lang = source_language.unwrap_or(DEFAULT_SOURCE_LANGUAGE);
    let target_lang = target_language.unwrap_or(DEFAULT_TARGET_LANGUAGE);

    format!("Translate from {} to {}: {}", source_lang, target_lang, source_text)
}

/// Parse and clean translation response from Ollama
pub fn parse_translation_response(response: &str) -> Result<String, String> {
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

/// Validate translation request
pub fn validate_translation_request(source_text: &str) -> Result<(), String> {
    if source_text.trim().is_empty() {
        return Err("Source text cannot be empty".to_string());
    }

    if source_text.len() > MAX_TEXT_LENGTH {
        return Err(format!(
            "Source text is too long (max {} characters)",
            MAX_TEXT_LENGTH
        ));
    }

    Ok(())
}

/// Get default model name
pub fn get_default_model() -> String {
    DEFAULT_MODEL.to_string()
}

/// Get default source language
pub fn get_default_source_language() -> String {
    DEFAULT_SOURCE_LANGUAGE.to_string()
}

/// Get default target language
pub fn get_default_target_language() -> String {
    DEFAULT_TARGET_LANGUAGE.to_string()
}

/// Get translation model options matching the Modelfile parameters
/// These options ensure consistency with the Modelfile configuration
pub fn get_translation_model_options() -> ollama_rs::models::ModelOptions {
    ollama_rs::models::ModelOptions::default()
        .temperature(0.15) // Low temperature for consistent translations (matches Modelfile)
        .top_p(0.85) // Balanced creativity (matches Modelfile)
        .top_k(40) // Focused vocabulary selection (matches Modelfile)
        .repeat_penalty(1.15) // Prevent repetition (matches Modelfile)
        .repeat_last_n(64) // Look back 64 tokens (matches Modelfile)
}

