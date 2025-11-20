/// Wolf RPG specific text validation
///
/// This module provides Wolf RPG-specific validation rules that extend
/// the universal validation logic.
use crate::parsers::text::validation::ContentValidator;
use regex::Regex;

/// Wolf RPG text validator
///
/// Provides Wolf RPG-specific validation by combining universal validation
/// with engine-specific rules.
pub struct WolfRpgTextValidator;

impl WolfRpgTextValidator {
    /// Validate text for Wolf RPG projects
    ///
    /// This method applies universal validation first, then adds
    /// Wolf RPG-specific validation rules.
    pub fn validate_text(content: &str) -> bool {
        // First apply universal validation
        if !ContentValidator::validate_text(content) {
            return false;
        }

        // Add Wolf RPG-specific validation rules here

        // Skip if it has specific file extensions (Wolf RPG context)
        // This complements the universal validation which has a more generic file extension filter
        let file_extensions =
            Regex::new(r"\.(png|jpg|jpeg|gif|bmp|wav|mp3|ogg|txt|json|dat)").unwrap();
        if file_extensions.is_match(&content.to_lowercase()) {
            return false;
        }

        true
    }
}
