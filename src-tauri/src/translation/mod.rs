// Translation module exports
// This module handles translation services

pub mod common;
pub mod glossary;
pub mod ollama;
pub mod provider;
pub mod runpod;
pub mod service;

// Re-export main types
pub use glossary::{
    format_glossary_for_prompt, lookup_glossary_terms, map_text_type_to_category, GlossaryEntry,
};
pub use provider::{TranslationProvider, ProviderConfig};
pub use service::*;
