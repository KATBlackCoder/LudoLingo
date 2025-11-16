// Glossary lookup module for translation
// Handles communication with frontend to retrieve glossary terms via Tauri events
// Uses event system for bidirectional communication with request_id matching

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Listener};
use uuid::Uuid;

/// Glossary entry structure matching frontend GlossaryEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryEntry {
    pub id: i64,
    pub source_term: String,
    pub translated_term: String,
    pub source_language: String,
    pub target_language: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,  // NULL = global pour tous les projets, INTEGER = spécifique à un projet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Request payload for glossary lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryLookupRequest {
    pub request_id: String,
    pub source_language: String,
    pub target_language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,  // NULL = global uniquement, INTEGER = combine global + project-specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,  // Filter glossary terms by category (None = all categories)
}

/// Response payload for glossary lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryLookupResponse {
    pub request_id: String,
    pub success: bool,
    pub data: Option<Vec<GlossaryEntry>>,
    pub error: Option<String>,
}

/// Map text_type to glossary category
/// text_type values: character, dialogue, system, item, skill, general, other
/// Mapping: dialogue → character (dialogue is character speaking), other → general
/// Special case: text_type 'general' → None (no filter, retrieves all terms including 'general' category)
pub fn map_text_type_to_category(text_type: Option<&str>) -> Option<String> {
    match text_type {
        Some("general") => None,  // 'general' text_type means no category filter (retrieves all terms)
        Some("character") => Some("character".to_string()),
        Some("dialogue") => Some("character".to_string()),  // Dialogue = character speaking, maps to 'character' category
        Some("system") => Some("system".to_string()),
        Some("item") => Some("item".to_string()),
        Some("skill") => Some("skill".to_string()),
        Some("other") => Some("general".to_string()),  // 'other' maps to 'general' category (glossary doesn't have 'other' category)
        _ => None,  // No category filter if text_type is unknown or None
    }
}

/// Lookup glossary terms for a specific language pair
/// Uses Tauri event system to communicate with frontend
/// 
/// Behavior:
/// - ALWAYS retrieves global terms (project_id IS NULL) - available for all projects
/// - IF project_id is provided: ALSO retrieves project-specific terms (project_id = ?)
/// - COMBINES both types: global + project-specific (if project_id provided)
/// - IF category is provided: FILTERS terms by category (terms matching the category OR category = 'general')
///   - category 'general' is ALWAYS included (applies to all categories as default)
/// - IF category is None: retrieves ALL terms (no category filter)
/// 
/// Returns all terms matching source_language AND target_language (and category if provided), combined for prompt enrichment
pub async fn lookup_glossary_terms(
    app_handle: &AppHandle,
    source_language: &str,
    target_language: &str,
    project_id: Option<i64>,
    category: Option<String>,
) -> Result<Vec<(String, String)>, String> {
    // Generate unique request ID
    let request_id = Uuid::new_v4().to_string();

    // Create request payload
    let request = GlossaryLookupRequest {
        request_id: request_id.clone(),
        source_language: source_language.to_string(),
        target_language: target_language.to_string(),
        project_id,
        category,
    };

    log::debug!(
        "Emitting glossary-lookup-request: request_id={}, source_language={}, target_language={}, project_id={:?}, category={:?}",
        request_id,
        source_language,
        target_language,
        project_id,
        request.category
    );

    // Emit request event to frontend
    app_handle
        .emit("glossary-lookup-request", &request)
        .map_err(|e| format!("Failed to emit glossary-lookup-request: {}", e))?;

    // Setup channel for response communication
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Result<Vec<(String, String)>, String>>();
    let request_id_clone = request_id.clone();

    // Listen for response event (global event listener)
    let listener_id = app_handle.listen("glossary-lookup-response", move |event| {
        let tx_clone = tx.clone();
        let request_id_check = request_id_clone.clone();
        
        // Parse payload in a blocking way since we're in a sync callback
        let payload_str = event.payload();
        let payload: Result<GlossaryLookupResponse, _> = serde_json::from_str(&payload_str);

        match payload {
            Ok(response) => {
                // Check if this response matches our request
                if response.request_id == request_id_check {
                    log::debug!(
                        "Received glossary-lookup-response for request_id={}",
                        request_id_check
                    );

                    let result = if response.success {
                        if let Some(entries) = response.data {
                            let terms: Vec<(String, String)> = entries
                                .into_iter()
                                .map(|e| (e.source_term, e.translated_term))
                                .collect();
                            Ok(terms)
                        } else {
                            Ok(Vec::new())
                        }
                    } else {
                        Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
                    };

                    // Send result through channel (non-blocking send)
                    let _ = tx_clone.send(result);
                } else {
                    log::debug!(
                        "Ignoring glossary-lookup-response with mismatched request_id: {} (expected: {})",
                        response.request_id,
                        request_id_check
                    );
                }
            }
            Err(e) => {
                log::error!("Failed to parse glossary-lookup-response: {}", e);
                let _ = tx_clone.send(Err(format!("Failed to parse response: {}", e)));
            }
        }
    });

    // Wait for response with timeout (10 seconds)
    tokio::select! {
        result = rx.recv() => {
            // Response received
            app_handle.unlisten(listener_id);
            result.unwrap_or_else(|| Err("Channel closed".to_string()))
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {
            // Timeout
            app_handle.unlisten(listener_id);
            Err("Timeout waiting for glossary-lookup-response".to_string())
        }
    }
}

/// Format glossary terms for inclusion in translation prompt
/// Format: "GLOSSARY:\nTerm1: Translation1\nTerm2: Translation2\n\n"
pub fn format_glossary_for_prompt(terms: &[(String, String)]) -> String {
    if terms.is_empty() {
        return String::new();
    }

    let mut formatted = String::from("GLOSSARY:\n");
    for (source, target) in terms {
        formatted.push_str(&format!("{}: {}\n", source, target));
    }
    formatted.push('\n'); // Extra newline before prompt

    formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_glossary_for_prompt_empty() {
        let terms: Vec<(String, String)> = Vec::new();
        let result = format_glossary_for_prompt(&terms);
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_glossary_for_prompt_single() {
        let terms = vec![("お兄ちゃん".to_string(), "Oni-san".to_string())];
        let result = format_glossary_for_prompt(&terms);
        assert_eq!(result, "GLOSSARY:\nお兄ちゃん: Oni-san\n\n");
    }

    #[test]
    fn test_format_glossary_for_prompt_multiple() {
        let terms = vec![
            ("お兄ちゃん".to_string(), "Oni-san".to_string()),
            ("魔法".to_string(), "Magic".to_string()),
            ("剣".to_string(), "Sword".to_string()),
        ];
        let result = format_glossary_for_prompt(&terms);
        let expected = "GLOSSARY:\nお兄ちゃん: Oni-san\n魔法: Magic\n剣: Sword\n\n";
        assert_eq!(result, expected);
    }
}

