// RunPod integration module exports
// This module handles RunPod Ollama API integration via HTTP

pub mod client;
pub mod common;
pub mod sequential;
pub mod single;

// Re-export main types
pub use client::{check_runpod_status, ModelInfo, RunPodClient, RunPodConfig};
pub use common::{
    build_translation_prompt, get_default_model, get_default_source_language,
    get_default_target_language, parse_translation_response, validate_translation_request,
    DEFAULT_MODEL, DEFAULT_SOURCE_LANGUAGE, DEFAULT_TARGET_LANGUAGE, MAX_TEXT_LENGTH,
};
pub use sequential::SequentialTranslationManager;
pub use single::SingleTranslationManager;
pub use crate::translation::common::types::{
    SequentialError, SequentialProgress, SequentialStatus, SequentialTranslationRequest, TranslationText,
    SingleTranslationRequest, SingleTranslationResult, TranslationSuggestion,
};
