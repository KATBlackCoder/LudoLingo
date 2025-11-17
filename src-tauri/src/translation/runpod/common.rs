// Common utilities for RunPod translation module
// Reuses utilities from ollama::common to avoid duplication

// Re-export common utilities from ollama module
pub use crate::translation::ollama::common::{
    build_translation_prompt, get_default_model, get_default_source_language,
    get_default_target_language, parse_translation_response, validate_translation_request,
    DEFAULT_MODEL, DEFAULT_SOURCE_LANGUAGE, DEFAULT_TARGET_LANGUAGE, MAX_TEXT_LENGTH,
};
