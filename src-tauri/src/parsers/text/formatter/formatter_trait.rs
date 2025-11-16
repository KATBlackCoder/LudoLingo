/// Trait for engine-specific text formatters
///
/// This trait defines the interface for engine-specific text formatting
/// that only processes codes relevant to that specific engine type.
pub trait EngineFormatter {
    /// Prepare text for translation by converting engine-specific codes to placeholders
    fn prepare_for_translation(text: &str) -> String;

    /// Restore text after translation by converting placeholders back to engine-specific codes
    fn restore_after_translation(text: &str) -> String;

    /// Quick check if text contains engine-specific formatting codes
    fn has_formatting_codes(text: &str) -> bool;

    /// Quick check if text contains engine-specific placeholder codes
    fn has_placeholder_codes(text: &str) -> bool;
}
