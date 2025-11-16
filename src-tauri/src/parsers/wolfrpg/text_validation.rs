/// Wolf RPG specific text validation
///
/// This module provides Wolf RPG-specific validation rules that extend
/// the universal validation logic.
use crate::parsers::text::validation::ContentValidator;

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
        // For now, we rely on universal validation
        // Future Wolf RPG-specific rules can be added here

        true
    }
}

