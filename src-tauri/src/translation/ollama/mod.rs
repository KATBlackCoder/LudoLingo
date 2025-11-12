// Ollama integration module exports
// This module handles Ollama API integration

pub mod client;
pub mod sequential;
pub mod single;
// pub mod models;

// Re-export main types
pub use client::{ModelInfo, OllamaClient, OllamaConfig, OllamaMode};
pub use sequential::{SequentialTranslationManager, SequentialTranslationRequest, SequentialProgress, SequentialStatus, SequentialError, TranslationText};
pub use single::{SingleTranslationManager, SingleTranslationRequest, SingleTranslationResult, TranslationSuggestion};
