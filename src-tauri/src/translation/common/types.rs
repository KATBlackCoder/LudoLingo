//! # Common Translation Types
//!
//! This module defines all the data structures shared between translation providers.
//! These types ensure consistency across the entire translation system while
//! maintaining full compatibility with Tauri commands and frontend interfaces.
//!
//! ## Design Goals
//!
//! - **Provider Agnostic**: Types work with any translation provider
//! - **Tauri Compatible**: All types implement required traits for IPC
//! - **Type Safe**: Strong typing prevents runtime errors
//! - **Serializable**: JSON serialization for frontend communication
//!
//! ## Type Categories
//!
//! - **Request Types**: Input structures for translation requests
//! - **Response Types**: Output structures for translation results
//! - **Progress Types**: Real-time progress tracking during batch operations
//! - **Session Types**: Internal state management for sequential translations
//!
//! ## Naming Conventions
//!
//! - `Single*`: Individual translation operations
//! - `Sequential*`: Batch/multi-text operations
//! - `*Request`: Input structures
//! - `*Result`: Output structures
//! - `*Progress`: Progress tracking structures
//! - `*Error`: Error reporting structures

use serde::{Deserialize, Serialize};

/// Pause settings for sequential translation operations
///
/// Controls automatic pausing during batch translation to prevent overheating
/// and allow for system cooling breaks.
///
/// # Fields
/// * `enabled` - Whether automatic pausing is enabled (default: true)
/// * `batch_size` - Number of translations before taking a pause (default: 150)
/// * `pause_duration_minutes` - Duration of pause in minutes (default: 5)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PauseSettings {
    pub enabled: bool,
    pub batch_size: u32,
    pub pause_duration_minutes: u32,
}

/// Request structure for single text translation operations
///
/// This structure represents a request to translate a single piece of text.
/// It contains all the parameters needed for a translation operation, including
/// language preferences, glossary settings, and model selection.
///
/// # Fields
/// * `source_text` - The text to be translated (required)
/// * `source_language` - Source language code (e.g., "ja", "en") - defaults to provider default
/// * `target_language` - Target language code (e.g., "fr", "es") - defaults to provider default
/// * `context` - Additional context for better translation quality
/// * `model` - Specific model to use - uses provider default if None
/// * `project_id` - Project ID for glossary lookup (None = global only, Some(id) = global + project-specific)
/// * `text_type` - Text category for glossary filtering ('dialogue', 'system', 'item', 'skill', 'other')
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTranslationRequest {
    pub source_text: String,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub context: Option<String>,
    pub model: Option<String>,
    pub project_id: Option<i64>,
    pub text_type: Option<String>,
}

/// Single translation result - common structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTranslationResult {
    pub translated_text: String,
    pub model_used: String,
    pub confidence: Option<f32>,
    pub processing_time_ms: u64,
}

/// Translation suggestion - common structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationSuggestion {
    pub suggestion: String,
    pub confidence: f32,
    pub source: String, // "ollama", "runpod", "glossary", "similar"
}

/// Translation text with metadata - common structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationText {
    pub id: i32,
    pub source_text: String,
    pub context: Option<String>,
    pub text_type: Option<String>, // Text type for category filtering: 'dialogue', 'system', 'item', 'skill', 'other'
}

/// Request structure for batch/sequential translation operations
///
/// This structure represents a request to translate multiple texts in sequence.
/// It's used for processing entire game files or batches of text entries.
/// The system supports resuming from specific entries and provides real-time progress tracking.
///
/// # Fields
/// * `project_id` - Project identifier for database storage and glossary lookup
/// * `texts` - Array of text entries to translate
/// * `start_from` - Optional entry ID to resume translation from (for interrupted sessions)
/// * `source_language` - Override project's default source language
/// * `target_language` - Override project's default target language
/// * `model` - Override default model for this batch
/// * `pause_settings` - Optional pause configuration for batch processing
///
/// # Example
/// ```json
/// {
///   "projectId": 1,
///   "texts": [{"id": 1, "sourceText": "Hello world"}],
///   "startFrom": null,
///   "sourceLanguage": "en",
///   "targetLanguage": "fr",
///   "model": "llama2",
///   "pauseSettings": {
///     "enabled": true,
///     "batchSize": 150,
///     "pauseDurationMinutes": 5
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequentialTranslationRequest {
    pub project_id: i64,
    pub texts: Vec<TranslationText>,
    pub start_from: Option<i32>,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub model: Option<String>,
    pub pause_settings: Option<PauseSettings>,
}

/// Sequential status enum - common structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SequentialStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Error,
}

/// Sequential progress - common structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequentialProgress {
    pub session_id: String,
    pub current_entry: Option<i32>,
    pub processed_count: i32,
    pub total_count: i32,
    pub status: SequentialStatus,
    pub estimated_time_remaining: Option<i64>, // seconds
    pub errors: Vec<SequentialError>,
    pub successful_translations: Vec<SuccessfulTranslation>,
    pub pause_time_remaining: Option<i64>, // seconds remaining in current pause
}

/// Sequential error - common structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequentialError {
    pub entry_id: i32,
    pub error_message: String,
    pub timestamp: i64,
}

/// Successful translation - common structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessfulTranslation {
    pub entry_id: i32,
    pub translated_text: String,
    pub model_used: String,
    pub timestamp: i64,
    pub processing_time_ms: u64,
}

/// Sequential session data structure - common fields
///
/// This structure contains the common fields used by all sequential translation sessions.
/// Provider-specific fields (like app_handle) are managed separately
/// in the provider-specific implementations.
#[derive(Debug, Clone)]
pub struct SequentialSession {
    pub session_id: String,
    pub project_id: i64,
    pub texts: Vec<TranslationText>,
    pub current_index: usize,
    pub processed_entries: std::collections::HashMap<i32, bool>, // entry_id -> success
    pub errors: Vec<SequentialError>,
    pub successful_translations: Vec<SuccessfulTranslation>,
    pub status: SequentialStatus,
    pub start_time: std::time::Instant,
    pub translation_settings: TranslationSettings,
    pub pause_settings: PauseSettings, // Configuration des pauses
    pub batch_counter: usize,           // Compteur interne pour les pauses
}

/// Translation settings for a session
#[derive(Debug, Clone)]
pub struct TranslationSettings {
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub model: Option<String>,
}
