// Ollama integration module exports
// This module handles Ollama API integration

pub mod client;

// Future modules
// pub mod batch;
// pub mod single;
// pub mod models;

// Re-export main types
pub use client::{OllamaClient, OllamaConfig, OllamaMode, ModelInfo};
