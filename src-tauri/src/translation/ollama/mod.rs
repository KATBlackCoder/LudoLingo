// Ollama integration module exports
// This module handles Ollama API integration

pub mod client;
pub mod common;
pub mod sequential;
pub mod single;
// pub mod models;

// Re-export main types
pub use client::{check_ollama_status, ModelInfo, OllamaClient, OllamaConfig};
pub use common::{
    build_translation_prompt, get_default_model, get_default_source_language,
    get_default_target_language, get_translation_model_options, parse_translation_response,
    validate_translation_request, DEFAULT_MODEL, DEFAULT_SOURCE_LANGUAGE, DEFAULT_TARGET_LANGUAGE,
    MAX_TEXT_LENGTH,
};
pub use sequential::{
    SequentialError, SequentialProgress, SequentialStatus, SequentialTranslationManager,
    SequentialTranslationRequest, TranslationText,
};
pub use single::{
    SingleTranslationManager, SingleTranslationRequest, SingleTranslationResult,
    TranslationSuggestion,
};
