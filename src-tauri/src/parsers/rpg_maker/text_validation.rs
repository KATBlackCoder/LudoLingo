/// RPG Maker specific text validation
///
/// This module provides RPG Maker-specific validation rules that extend
/// the universal validation logic.
use crate::parsers::text::validation::ContentValidator;

/// RPG Maker text validator
///
/// Provides RPG Maker-specific validation by combining universal validation
/// with engine-specific rules.
pub struct RpgMakerTextValidator;

impl RpgMakerTextValidator {
    /// Validate text for RPG Maker projects
    ///
    /// This method applies universal validation first, then adds
    /// RPG Maker-specific validation rules.
    pub fn validate_text(content: &str) -> bool {
        // First apply universal validation
        if !ContentValidator::validate_text(content) {
            return false;
        }

        // RPG Maker-specific validation: Skip text that contains only punctuation marks
        // This includes common punctuation in various languages
        // Must NOT contain any letters or digits
        let has_letters_or_digits = content.chars().any(|c| c.is_alphanumeric());
        if !has_letters_or_digits {
            // Check if it contains at least one punctuation mark
            let has_punctuation = content.chars().any(|c| {
                c.is_ascii_punctuation()
                    || c == '？' // Full-width question mark
                    || c == '！' // Full-width exclamation
                    || c == '。' // Japanese period
                    || c == '、' // Japanese comma
                    || c == '：' // Full-width colon
                    || c == '；' // Full-width semicolon
                    || c == '…' // Ellipsis
                    || c == '・' // Japanese middle dot
                    || c == '〇' // Japanese circle (maru) - technical marker
                    || c == '○' // Japanese circle variant
                    || c == 'ｘ' // Japanese X (batsu) - technical marker
                    || c == '×' // Japanese X variant
            });
            // If no letters/digits AND has punctuation, it's only punctuation
            if has_punctuation {
                return false;
            }
        }

        // RPG Maker-specific validation: Skip file names and extensions (images, sounds, etc.)
        // Only filter if it looks like a file path/name, not just text ending with punctuation
        // Allow RPG Maker formatting codes: \n[, \C[, \N[
        if content.contains('/')
            || (content.contains('\\')
                && !content.contains("\\n[")
                && !content.contains("\\C[")
                && !content.contains("\\N["))
        {
            return false;
        }

        true
    }
}
