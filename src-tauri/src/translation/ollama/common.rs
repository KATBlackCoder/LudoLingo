// Common utilities and constants for Ollama translation module
// Centralizes shared logic to avoid duplication (DRY principle)

/// Default translation model
pub const DEFAULT_MODEL: &str = "ludolingo:latest";

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
/// - GLOSSARY section support for terminological consistency
///
/// Glossary terms behavior:
/// - glossary_terms contains ALWAYS global terms (project_id IS NULL)
/// - IF project_id was provided during lookup: glossary_terms ALSO contains project-specific terms
/// - All terms are combined and formatted together: "GLOSSARY:\nTerm1: Translation1\nTerm2: Translation2\n\nTranslate from ..."
pub fn build_translation_prompt(
    source_text: &str,
    source_language: Option<&str>,
    target_language: Option<&str>,
    glossary_terms: Option<&[(String, String)]>,
) -> String {
    let source_lang = source_language.unwrap_or(DEFAULT_SOURCE_LANGUAGE);
    let target_lang = target_language.unwrap_or(DEFAULT_TARGET_LANGUAGE);

    // Format glossary section if terms are provided
    let glossary_section = if let Some(terms) = glossary_terms {
        crate::translation::glossary::format_glossary_for_prompt(terms)
    } else {
        String::new()
    };

    // Build final prompt with optional glossary prefix
    if glossary_section.is_empty() {
        format!(
            "Translate from {} to {}: {}",
            source_lang, target_lang, source_text
        )
    } else {
        format!(
            "{}Translate from {} to {}: {}",
            glossary_section, source_lang, target_lang, source_text
        )
    }
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

/// Get translation model options - minimal API parameters
/// All other parameters (temperature, top_p, etc.) are defined in the Modelfile
/// This follows DRY principle: single source of truth for model parameters
pub fn get_translation_model_options() -> ollama_rs::models::ModelOptions {
    ollama_rs::models::ModelOptions::default()
        .num_ctx(2048) // Context window - required for API initialization
}
