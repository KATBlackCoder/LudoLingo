// Translation module exports
// This module handles translation services

pub mod glossary;
pub mod ollama;
pub mod service;

// Re-export main types
pub use glossary::{format_glossary_for_prompt, lookup_glossary_terms, GlossaryEntry};
pub use service::*;
