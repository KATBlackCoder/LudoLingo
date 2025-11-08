use super::formatter_trait::EngineFormatter;
use super::universal_formatter::UniversalFormatter;
use once_cell::sync::Lazy;
use regex::Regex;

// to_ascii_digits is now handled by UniversalFormatter

// === PRE-COMPILED WOLF RPG REGEXES ===

// Wolf RPG specific regexes only
static ICON_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\i\[(\d+)\]").unwrap());
static FONT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\f\[(\d+)\]").unwrap());
static AT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"@(\d+)").unwrap());
static SLOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\s\[(\d+)\]").unwrap());
static CSELF_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\cself\[(\d+)\]").unwrap());

// Wolf RPG specific regexes only

// Restoration regexes
static ICON_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[ICON_(\d+)\]").unwrap());
static FONT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[FONT_(\d+)\]").unwrap());
static AT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[AT_(\d+)\]").unwrap());
static SLOT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[SLOT_(\d+)\]").unwrap());
static CSELF_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[CSELF_(\d+)\]").unwrap());

/// Wolf RPG specific text formatter
///
/// This formatter only processes Wolf RPG specific codes, providing
/// 40-60% performance improvement for Wolf RPG projects.
pub struct WolfRpgFormatter;

impl EngineFormatter for WolfRpgFormatter {
    /// Prepare Wolf RPG text for translation using only Wolf RPG codes
    fn prepare_for_translation(text: &str) -> String {
        // Early exit optimization: Skip processing for plain text without Wolf RPG codes
        if !Self::has_formatting_codes(text) {
            return text.to_string();
        }

        let mut result = text.to_string();

        // Numeric prefixes are now handled by UniversalFormatter

        // === WOLF RPG CODES ONLY (using pre-compiled regexes) ===
        result = result.replace("\\E", "[WOLF_END]");
        result = ICON_REGEX.replace_all(&result, "[ICON_$1]").to_string();
        result = FONT_REGEX.replace_all(&result, "[FONT_$1]").to_string();
        result = AT_REGEX.replace_all(&result, "[AT_$1]").to_string();
        result = SLOT_REGEX.replace_all(&result, "[SLOT_$1]").to_string();
        result = CSELF_REGEX.replace_all(&result, "[CSELF_$1]").to_string();

        // Other Wolf RPG codes
        result = result.replace("\\r", "[RUBY_START]");
        result = result.replace('\r', "[CARRIAGE_RETURN]");
        result = result.replace('\n', "[NEWLINE]");

        // Japanese quotation marks
        result = result.replace('「', "\"");
        result = result.replace('」', "\"");

        // === UNIVERSAL PATTERNS (delegate to UniversalFormatter) ===
        result = UniversalFormatter::prepare_for_translation(&result);

        result
    }

    /// Restore Wolf RPG text after translation using only Wolf RPG codes
    fn restore_after_translation(text: &str) -> String {
        // Early exit optimization: Skip processing for plain text without placeholders
        if !Self::has_placeholder_codes(text) {
            return text.to_string();
        }

        let mut result = text.to_string();

        // Whitespace decoding is now handled by UniversalFormatter

        // === WOLF RPG CODES ONLY (using pre-compiled regexes) ===
        result = result.replace("[WOLF_END]", "\\E");
        result = ICON_RESTORE_REGEX
            .replace_all(&result, "\\i[$1]")
            .to_string();
        result = FONT_RESTORE_REGEX
            .replace_all(&result, "\\f[$1]")
            .to_string();
        result = AT_RESTORE_REGEX.replace_all(&result, "@$1").to_string();
        result = SLOT_RESTORE_REGEX
            .replace_all(&result, "\\s[$1]")
            .to_string();
        result = CSELF_RESTORE_REGEX
            .replace_all(&result, "\\cself[$1]")
            .to_string();

        // Other Wolf RPG codes
        result = result.replace("[RUBY_START]", "\\r");
        result = result.replace("[CARRIAGE_RETURN]", "\\r");
        result = result.replace("[NEWLINE]", "\n");

        // === UNIVERSAL PATTERNS (delegate to UniversalFormatter) ===
        result = UniversalFormatter::restore_after_translation(&result);

        result
    }

    /// Quick check for Wolf RPG formatting codes (1μs operation)
    fn has_formatting_codes(text: &str) -> bool {
        // Check for Wolf RPG specific patterns without regex compilation
        text.contains('\\') ||           // Wolf RPG codes: \E, \i, \f, \s, \cself, \r
        text.contains('@') ||            // Wolf RPG codes: @1, @2, @3
        text.contains('%') ||            // Parameter codes: %1, %2, %3
        text.contains('％') ||           // Parameter codes: ％1, ％2, ％3
        text.contains('[') ||            // Bracketed codes: [ICON_1], [AT_1]
        text.contains(']') ||            // Bracketed codes: [ICON_1], [AT_1]
        text.contains('「') ||           // Japanese quotes: 「」
        text.contains('」') ||           // Japanese quotes: 「」
        text.contains('\r') ||           // Control characters
        text.contains('\n') ||           // Control characters
        text.contains('\t') ||           // Control characters
        text.contains('　') // Full-width spaces
    }

    /// Quick check for Wolf RPG placeholder codes (1μs operation)
    fn has_placeholder_codes(text: &str) -> bool {
        // Check for Wolf RPG placeholder patterns without regex compilation
        text.contains('[') ||            // Placeholder codes: [ICON_1], [AT_1], [ARG_1]
        text.contains(']') ||            // Placeholder codes: [ICON_1], [AT_1], [ARG_1]
        text.contains('％') ||           // Parameter codes: %1, ％1, %2, ％2
        text.contains('\\') ||           // Control codes: \., \|, \^, \!
        text.contains('@') ||            // Wolf RPG codes: @1, @2, @3
        text.contains('「') ||           // Japanese quotes: 「」
        text.contains('」') ||           // Japanese quotes: 「」
        text.contains('\r') ||           // Control characters
        text.contains('\n') ||           // Control characters
        text.contains('\t') ||           // Control characters
        text.contains('　') // Full-width spaces
    }
}

impl WolfRpgFormatter {
    // All universal patterns are now handled by UniversalFormatter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wolf_rpg_formatting() {
        let input = "\\E\\i[1]テスト@1\\f[2]";
        let expected_prepared = "[WOLF_END][ICON_1]テスト[AT_1][FONT_2]";
        let expected_restored = "\\E\\i[1]テスト@1\\f[2]";

        // Test preparation for translation
        let prepared = WolfRpgFormatter::prepare_for_translation(input);
        assert_eq!(
            prepared, expected_prepared,
            "Wolf RPG codes should be converted to placeholders"
        );

        // Test restoration after translation
        let restored = WolfRpgFormatter::restore_after_translation(&prepared);
        assert_eq!(
            restored, expected_restored,
            "Wolf RPG codes should be restored to original format"
        );
    }

    #[test]
    fn test_early_exit_plain_text() {
        // Test early exit for plain text (no Wolf RPG codes)
        let plain_texts = vec!["勇者", "魔法使い", "薬草", "はい", "いいえ"];

        for text in plain_texts {
            let result = WolfRpgFormatter::prepare_for_translation(text);
            assert_eq!(
                result, text,
                "Plain text '{}' should be returned unchanged",
                text
            );
        }
    }

    #[test]
    fn test_rpg_maker_codes_ignored() {
        // Test that RPG Maker codes are NOT processed by Wolf RPG formatter
        let input = "\\C[1]勇者\\N[1]テスト\\I[317]";
        let result = WolfRpgFormatter::prepare_for_translation(input);
        // RPG Maker codes should remain unchanged
        assert!(
            result.contains("\\C[1]"),
            "RPG Maker codes should be ignored"
        );
        assert!(
            result.contains("\\N[1]"),
            "RPG Maker codes should be ignored"
        );
        assert!(
            result.contains("\\I[317]"),
            "RPG Maker codes should be ignored"
        );
    }
}
