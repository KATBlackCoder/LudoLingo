use super::formatter_trait::EngineFormatter;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

/// Convert full-width digits to ASCII digits within a string
fn to_ascii_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '０'..='９' => char::from_u32('0' as u32 + (c as u32 - '０' as u32)).unwrap(),
            d => d,
        })
        .collect()
}

// === PRE-COMPILED UNIVERSAL REGEXES ===

// Universal regexes (needed by all engines)
static ARG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[%％]([0-9０-９]+)").unwrap());
static NUM_PREFIX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([0-9０-９]{3})[＿_](.+)$").unwrap());

// Whitespace regexes (needed by all engines)
static FW_SPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(　+)").unwrap());
static LEADING_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^( +)").unwrap());
static TRAILING_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"( +)$").unwrap());
static MULTI_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"( {2,})").unwrap());
static TABS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\t+)").unwrap());

// Restoration regexes
static ARG_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[ARG_(\d+)\]").unwrap());
static NUM_PREFIX_RESTORE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[NUM_PREFIX_(\d{3})\]").unwrap());
static FW_SPACE_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[FWSPC_(\d+)\]").unwrap());
static SPC_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[SPC_(\d+)\]").unwrap());
static TAB_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[TAB_(\d+)\]").unwrap());

/// Universal text formatter for common patterns
///
/// This formatter handles universal patterns that are common across all engines:
/// - Parameter placeholders: %n, ％n
/// - Numeric prefixes: 100＿text, ２００_text
/// - Whitespace encoding: spaces, tabs, full-width spaces
/// - Control codes: \., \|, \^, \!
///
/// This formatter is used by ALL engine-specific formatters for universal patterns.
pub struct UniversalFormatter;

impl EngineFormatter for UniversalFormatter {
    /// Prepare text for translation using only universal patterns
    fn prepare_for_translation(text: &str) -> String {
        // Early exit optimization: Skip processing for plain text without universal patterns
        if !Self::has_formatting_codes(text) {
            return text.to_string();
        }

        let mut result = text.to_string();

        // Preserve numeric map/area prefixes
        if let Some(caps) = NUM_PREFIX_REGEX.captures(&result) {
            let prefix_ascii = to_ascii_digits(&caps[1]);
            let tail = caps[2].to_string();
            result = format!("[NUM_PREFIX_{}]{}", prefix_ascii, tail);
        }

        // === UNIVERSAL CODES (using pre-compiled regexes) ===
        result = ARG_REGEX
            .replace_all(&result, |caps: &Captures| {
                format!("[ARG_{}]", to_ascii_digits(&caps[1]))
            })
            .to_string();

        // Control codes
        result = result.replace("\\.", "[CTRL_DOT]");
        result = result.replace("\\|", "[CTRL_WAIT]");
        result = result.replace("\\^", "[CTRL_INSTANT]");
        result = result.replace("\\!", "[CTRL_INPUT]");
        result = result.replace("\\{", "[CTRL_OPEN_BRACE]");

        // Control characters (literal newline, tab, etc.)
        result = result.replace('\n', "[CTRL_NEWLINE]");
        result = result.replace('\r', "[CTRL_CARRIAGE_RETURN]");
        result = result.replace('\t', "[CTRL_TAB]");

        // === WHITESPACE ENCODING ===
        result = Self::encode_whitespace_placeholders(&result);

        result
    }

    /// Restore text after translation using only universal patterns
    fn restore_after_translation(text: &str) -> String {
        // Early exit optimization: Skip processing for plain text without placeholders
        if !Self::has_placeholder_codes(text) {
            return text.to_string();
        }

        let mut result = text.to_string();

        // === WHITESPACE DECODING ===
        result = Self::decode_whitespace_placeholders(&result);

        // Control characters (literal newline, tab, etc.)
        result = result.replace("[CTRL_NEWLINE]", "\n");
        result = result.replace("[CTRL_CARRIAGE_RETURN]", "\r");
        result = result.replace("[CTRL_TAB]", "\t");

        // === UNIVERSAL CODES ===
        result = ARG_RESTORE_REGEX.replace_all(&result, "%$1").to_string();

        // Restore numeric prefix placeholders
        result = NUM_PREFIX_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| format!("{}＿", &caps[1]))
            .to_string();

        // Restore control codes
        result = result.replace("[CTRL_DOT]", "\\.");
        result = result.replace("[CTRL_WAIT]", "\\|");
        result = result.replace("[CTRL_INSTANT]", "\\^");
        result = result.replace("[CTRL_INPUT]", "\\!");
        result = result.replace("[CTRL_OPEN_BRACE]", "\\{");

        result
    }

    /// Quick check for universal formatting codes (1μs operation)
    fn has_formatting_codes(text: &str) -> bool {
        // Check for universal patterns without regex compilation
        text.contains('%') ||            // Parameter codes: %1, %2, %3
        text.contains('％') ||           // Parameter codes: ％1, ％2, ％3
        text.contains('\\') ||           // Control codes: \., \|, \^, \!
        text.contains('\r') ||           // Control characters
        text.contains('\n') ||           // Control characters
        text.contains('\t') ||           // Control characters
        text.contains('　') ||           // Full-width spaces
        text.contains('＿') ||           // Numeric prefixes: 100＿text
        text.contains('_') // Numeric prefixes: 100_text
    }

    /// Quick check for universal placeholder codes (1μs operation)
    fn has_placeholder_codes(text: &str) -> bool {
        // Check for universal placeholder patterns without regex compilation
        text.contains('[') ||            // Placeholder codes: [ARG_1], [CTRL_DOT]
        text.contains(']') ||            // Placeholder codes: [ARG_1], [CTRL_DOT]
        text.contains('％') ||           // Parameter codes: %1, ％1, %2, ％2
        text.contains('\\') ||           // Control codes: \., \|, \^, \!
        text.contains('\r') ||           // Control characters
        text.contains('\n') ||           // Control characters
        text.contains('\t') ||           // Control characters
        text.contains('　') // Full-width spaces
    }
}

impl UniversalFormatter {
    /// Encode significant whitespace using pre-compiled regexes
    fn encode_whitespace_placeholders(input: &str) -> String {
        let mut result = input.to_string();

        // Encode full-width spaces
        result = FW_SPACE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count = caps[1].chars().count();
                format!("[FWSPC_{}]", count)
            })
            .to_string();

        // Encode leading ASCII spaces
        result = LEADING_SPACES_REGEX
            .replace(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[SPC_{}]", count)
            })
            .to_string();

        // Encode trailing ASCII spaces
        result = TRAILING_SPACES_REGEX
            .replace(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[SPC_{}]", count)
            })
            .to_string();

        // Encode interior runs of ASCII spaces
        result = MULTI_SPACES_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[SPC_{}]", count)
            })
            .to_string();

        // Encode tabs
        result = TABS_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[TAB_{}]", count)
            })
            .to_string();

        result
    }

    /// Decode whitespace placeholders using pre-compiled regexes
    fn decode_whitespace_placeholders(input: &str) -> String {
        let mut result = input.to_string();

        // Decode full-width spaces
        result = FW_SPACE_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count: usize = caps[1].parse().unwrap_or(0);
                "　".repeat(count)
            })
            .to_string();

        // Decode ASCII spaces
        result = SPC_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count: usize = caps[1].parse().unwrap_or(0);
                " ".repeat(count)
            })
            .to_string();

        // Decode tabs
        result = TAB_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count: usize = caps[1].parse().unwrap_or(0);
                "\t".repeat(count)
            })
            .to_string();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_formatting() {
        let input = "%1は瞑想した！";
        let expected_prepared = "[ARG_1]は瞑想した！";
        let expected_restored = "%1は瞑想した！";

        // Test preparation for translation
        let prepared = UniversalFormatter::prepare_for_translation(input);
        assert_eq!(
            prepared, expected_prepared,
            "Parameter placeholder should be converted to [ARG_1]"
        );

        // Test restoration after translation
        let restored = UniversalFormatter::restore_after_translation(&prepared);
        assert_eq!(
            restored, expected_restored,
            "Parameter placeholder should be restored to %1"
        );
    }

    #[test]
    fn test_early_exit_plain_text() {
        // Test early exit for plain text (no universal codes)
        let plain_texts = vec!["勇者", "魔法使い", "薬草", "はい", "いいえ"];

        for text in plain_texts {
            let result = UniversalFormatter::prepare_for_translation(text);
            assert_eq!(
                result, text,
                "Plain text '{}' should be returned unchanged",
                text
            );
        }
    }

    #[test]
    fn test_engine_specific_codes_ignored() {
        // Test that engine-specific codes are NOT processed by universal formatter
        let input = "\\C[1]勇者\\N[1]テスト\\E\\i[1]@1";
        let result = UniversalFormatter::prepare_for_translation(input);
        // Engine-specific codes should remain unchanged
        assert!(
            result.contains("\\C[1]"),
            "RPG Maker codes should be ignored"
        );
        assert!(
            result.contains("\\N[1]"),
            "RPG Maker codes should be ignored"
        );
        assert!(result.contains("\\E"), "Wolf RPG codes should be ignored");
        assert!(
            result.contains("\\i[1]"),
            "Wolf RPG codes should be ignored"
        );
        assert!(result.contains("@1"), "Wolf RPG codes should be ignored");
    }
}
