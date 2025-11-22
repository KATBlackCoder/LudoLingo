/// Wolf RPG specific text validation
///
/// This module provides Wolf RPG-specific validation rules that extend
/// the universal validation logic.
use crate::parsers::text::validation::ContentValidator;
use once_cell::sync::Lazy;
use regex::Regex;

/// Wolf RPG text validator
///
/// Provides Wolf RPG-specific validation by combining universal validation
/// with engine-specific rules.
pub struct WolfRpgTextValidator;

// Regex to match simple placeholders in format [NAME]
// Examples: [AT_1], [NEWLINE], [RIGHT_ALIGN], [CSELF_9], [ICON_1], etc.
static PLACEHOLDER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[[A-Z_][A-Z0-9_]*\]").unwrap()
});

// Regex to match nested placeholders in format [NAME_content] where content can contain nested brackets
// Examples: [F_SIMPLE_[CSELF_18]], [AX_[CSELF_13]], [AY_[CSELF_14]], etc.
// This catches placeholders that contain other placeholders inside them
static NESTED_PLACEHOLDER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[[A-Z_][A-Z0-9_]*[^\]]*\]").unwrap()
});

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

        // Skip text that contains only placeholders/codes without actual text
        // Examples: "\n", "\\n", "\n\n", "\\n\\n", "[AT_1][NEWLINE][CSELF_9]", etc.
        // This catches cases like the SetString command with only "\n" and "\n" as stringArgs
        // or texts with only placeholders like [AT_1][NEWLINE][RIGHT_ALIGN][CSELF_9]
        // or nested placeholders like [F_SIMPLE_[CSELF_18]][CSELF_7]
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return false;
        }
        
        // Step 1: Remove all placeholders iteratively to handle nested cases
        // This handles both simple [NAME] and nested [F_SIMPLE_[CSELF_18]] cases
        // Objective: Remove ALL placeholders to check if any real text remains
        let mut without_placeholders = trimmed.to_string();
        let mut previous_len = without_placeholders.len();
        
        loop {
            // First pass: Remove simple placeholders [NAME] (e.g., [CSELF_7], [NEWLINE], [AT_1])
            without_placeholders = PLACEHOLDER_REGEX.replace_all(&without_placeholders, "").to_string();
            
            // Second pass: Remove nested placeholders like [F_SIMPLE_[CSELF_18]]
            // This catches placeholders that contain other placeholders inside them
            without_placeholders = NESTED_PLACEHOLDER_REGEX.replace_all(&without_placeholders, "").to_string();
            
            // If no more changes, break
            if without_placeholders.len() == previous_len {
                break;
            }
            previous_len = without_placeholders.len();
        }
        
        let without_placeholders = without_placeholders.trim();
        
        // Step 2: Remove control characters and escape sequences
        let without_controls = without_placeholders
            .replace("\\n", "")
            .replace("\\r", "")
            .replace("\\t", "")
            .replace("\\", "")
            .replace("\n", "")
            .replace("\r", "")
            .replace("\t", "")
            .replace("@", "")
            .replace("<", "")
            .replace(">", "");
        let without_controls = without_controls.trim();
        
        // If nothing remains after removing placeholders and controls, skip
        if without_controls.is_empty() {
            return false;
        }
        
        // Step 2.5: Check if remaining content contains only digits (with or without special characters)
        // Examples: "3", "0", "3…", "3:", "0[CSELF_67]" (after placeholder removal becomes "0")
        // If only digits remain (no letters, no CJK characters), skip
        let only_digits_and_special: String = without_controls
            .chars()
            .filter(|c| {
                let ch = *c;
                // Keep only digits and common special characters
                ch.is_ascii_digit() || 
                ch == '.' || ch == ',' || ch == ':' || ch == ';' || ch == '…' || 
                ch == '-' || ch == '+' || ch == '=' || ch == '(' || ch == ')' ||
                ch == '[' || ch == ']' || ch == '{' || ch == '}' ||
                ch == '/' || ch == '\\' || ch == '?' || ch == '!' ||
                ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
            })
            .collect();
        
        // If everything was filtered out (only digits and special chars), check if original had only digits
        if only_digits_and_special.trim() == without_controls.trim() {
            // Check if original contains any letters or CJK characters
            let has_text = without_controls.chars().any(|c| {
                c.is_alphabetic() || 
                (c as u32 >= 0x3040 && c as u32 <= 0x9FFF) || // Hiragana, Katakana, CJK
                (c as u32 >= 0x3400 && c as u32 <= 0x4DBF) || // CJK Extension A
                (c as u32 >= 0x20000 && c as u32 <= 0x2A6DF) // CJK Extension B
            });
            
            // If no text found (only digits and special chars), skip
            if !has_text {
                return false;
            }
        }
        
        // Step 3: Remove all special characters and check if any actual text remains
        // Special characters like ():,?/ etc. don't count as text if they're the only characters
        let without_special_chars: String = without_controls
            .chars()
            .filter(|c| {
                let ch = *c;
                // Keep letters, digits, and CJK characters
                ch.is_alphanumeric() || 
                // Keep Japanese/Chinese characters
                (ch as u32 >= 0x3040 && ch as u32 <= 0x9FFF) || // Hiragana, Katakana, CJK
                (ch as u32 >= 0x3400 && ch as u32 <= 0x4DBF) || // CJK Extension A
                (ch as u32 >= 0x20000 && ch as u32 <= 0x2A6DF) || // CJK Extension B
                // Keep some punctuation that might be part of text
                ch == '！' || ch == '？' || ch == '。' || ch == '、' || // Japanese punctuation
                ch == '，' || // Chinese punctuation
                ch == '「' || ch == '」' || ch == '『' || ch == '』' // Japanese quotes
            })
            .collect();
        
        // If only special characters remain (no letters, digits, or CJK), skip
        if without_special_chars.trim().is_empty() {
            return false;
        }

        // Skip text containing "X[" (uppercase) - these are typically debug/error messages
        // Example: "\\>コモン159「X[戦]技能選択実行」：エラー"
        if content.contains("X[") {
            return false;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_only_texts_filtered() {
        // Test that texts containing only placeholders/codes are filtered out
        let placeholder_only_texts = vec![
            "\\n",
            "\n",
            "\\n\\n",
            "\n\n",
            "\\n\n",
            "  \\n  ",
            "  \n  ",
            "\\r",
            "\r",
            "\\t",
            "\t",
            "@",
            "[",
            "]",
            "<",
            ">",
            "\\",
            "  ",
            "\\n\\r\\t",
        ];

        for text in placeholder_only_texts {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                !result,
                "Text '{}' should be filtered out as it contains only placeholders",
                text.escape_debug()
            );
        }
    }

    #[test]
    fn test_texts_with_content_not_filtered() {
        // Test that texts with actual content are NOT filtered out
        let texts_with_content = vec![
            "\\nテスト",
            "テスト\\n",
            "\\nテスト\\n",
            "\\sys[100] ％",
            "<C>\\cself[7]",
            "\\>\\cself[9]",
            "\\font[1]+\\cself[15]",
            "\\ax[\\cself[13]]\n\\ay[0]<R>",
            "\\v[11]00円",
            "勇者",
            "テスト",
            "\\n勇者\\n",
        ];

        for text in texts_with_content {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                result,
                "Text '{}' should NOT be filtered out as it contains actual content",
                text.escape_debug()
            );
        }
    }

    #[test]
    fn test_x_bracket_filtered() {
        // Test that texts containing "X[" (uppercase) are filtered out
        // These are typically debug/error messages
        let x_bracket_texts = vec![
            "\\>コモン159「X[戦]技能選択実行」：エラー\n　アイテムコードが1万未満です。",
            "X[戦]テスト",
            "エラーX[移]",
            "X[共]万能ｳｨﾝﾄﾞｳ描画処理",
        ];

        for text in x_bracket_texts {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                !result,
                "Text '{}' should be filtered out as it contains X[",
                text.escape_debug()
            );
        }

        // Test that "x[" (lowercase) is NOT filtered
        let lowercase_x_texts = vec![
            "x[test]",
            "test x[123]",
        ];

        for text in lowercase_x_texts {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                result,
                "Text '{}' should NOT be filtered out as it contains lowercase x[",
                text.escape_debug()
            );
        }
    }

    #[test]
    fn test_placeholder_format_filtered() {
        // Test that texts containing only placeholders in format [NAME] are filtered out
        // Examples from actual Wolf RPG files: [AT_1][NEWLINE][RIGHT_ALIGN][CSELF_9]
        let placeholder_format_texts = vec![
            "[AT_1][NEWLINE][RIGHT_ALIGN][CSELF_9]",
            "[AT_1]",
            "[NEWLINE]",
            "[CSELF_9]",
            "[AT_1][NEWLINE]",
            "[ICON_1][FONT_2]",
            "[AT_1][NEWLINE][RIGHT_ALIGN][CSELF_9][ICON_1]",
            "  [AT_1][NEWLINE]  ",
            "[AT_1][NEWLINE][CSELF_8][CSELF_7][CSELF_10]",
            "[CENTER_TAG][CSELF_7]",
            "[RIGHT_ALIGN][CSELF_9]",
            "[CENTER_TAG]",
            "[RIGHT_ALIGN]",
            "[LEFT_BRACKETS][RIGHT_BRACKETS]",
        ];

        for text in placeholder_format_texts {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                !result,
                "Text '{}' should be filtered out as it contains only placeholders",
                text.escape_debug()
            );
        }

        // Test that texts with placeholders AND actual text are NOT filtered
        let texts_with_placeholders_and_content = vec![
            "[AT_1]テスト",
            "テスト[NEWLINE]",
            "[AT_1]勇者[NEWLINE]",
            "[CSELF_9]購入",
            "[AT_1][NEWLINE]テスト",
        ];

        for text in texts_with_placeholders_and_content {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                result,
                "Text '{}' should NOT be filtered out as it contains actual content",
                text.escape_debug()
            );
        }

        // Test that texts with only special characters (no placeholders, no text) are filtered
        let special_chars_only = vec![
            "():,?/",
            "()",
            ",,",
            "??",
            "//",
            "  ()  ",
            "():,?/[]",
        ];

        for text in special_chars_only {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                !result,
                "Text '{}' should be filtered out as it contains only special characters",
                text.escape_debug()
            );
        }

        // Test that texts with placeholders AND special characters only (no actual text) are filtered
        // Examples: [AT_1] :, [AT_1] ?, [NEWLINE] :, etc.
        let placeholder_with_special_chars_only = vec![
            "[AT_1] :",
            "[AT_1] ?",
            "[NEWLINE] :",
            "[AT_1] : ",
            " [AT_1] ? ",
            "[AT_1] : [NEWLINE]",
            "[AT_1] ? [NEWLINE]",
            "[CSELF_9] :",
            "[AT_1] ,",
            "[AT_1] /",
            "[AT_1] ()",
            "[AT_1] : ?",
            "[AT_1][NEWLINE] :",
            "[AT_1] : [CSELF_9]",
            "[AT_1] ? [NEWLINE] :",
        ];

        for text in placeholder_with_special_chars_only {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                !result,
                "Text '{}' should be filtered out as it contains only placeholders and special characters",
                text.escape_debug()
            );
        }

        // Test that texts with placeholders, special characters AND actual text are NOT filtered
        let placeholder_with_special_chars_and_text = vec![
            "[AT_1] : テスト",
            "[AT_1] ? 勇者",
            "[NEWLINE] : テスト",
            "[AT_1] : テスト :",
            "[AT_1] ? 勇者 ?",
            "テスト [AT_1] :",
            "[AT_1] : テスト [NEWLINE]",
        ];

        for text in placeholder_with_special_chars_and_text {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                result,
                "Text '{}' should NOT be filtered out as it contains actual content",
                text.escape_debug()
            );
        }
    }

    #[test]
    fn test_nested_placeholders_filtered() {
        // Test that texts containing nested placeholders are filtered out
        // Examples: [F_SIMPLE_[CSELF_18]][CSELF_7], [AX_[CSELF_13]], etc.
        let nested_placeholder_texts = vec![
            "[F_SIMPLE_[CSELF_18]][CSELF_7]",
            "[F_SIMPLE_[CSELF_18]]",
            "[AX_[CSELF_13]]",
            "[AY_[CSELF_14]]",
            "[F_SIMPLE_[CSELF_19]][CSELF_8][NEWLINE]",
            "[AX_[CSELF_13]][CSELF_7]",
            "[F_SIMPLE_[CSELF_18]][CSELF_7][AT_1]",
        ];

        for text in nested_placeholder_texts {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                !result,
                "Text '{}' should be filtered out as it contains only nested placeholders",
                text.escape_debug()
            );
        }

        // Test that texts with nested placeholders AND actual text are NOT filtered
        let nested_placeholders_with_content = vec![
            "[F_SIMPLE_[CSELF_18]]テスト",
            "テスト[F_SIMPLE_[CSELF_18]]",
            "[F_SIMPLE_[CSELF_18]]勇者[CSELF_7]",
            "[AX_[CSELF_13]]購入",
        ];

        for text in nested_placeholders_with_content {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                result,
                "Text '{}' should NOT be filtered out as it contains actual content",
                text.escape_debug()
            );
        }
    }

    #[test]
    fn test_digits_only_filtered() {
        // Test that texts containing only digits (with or without placeholders/special chars) are filtered
        // Examples: "3", "0", "3…", "3:", "0[CSELF_67]" (becomes "0" after placeholder removal)
        let digits_only_texts = vec![
            "3",
            "0",
            "123",
            "3…",
            "3:",
            "3,",
            "3.",
            "3 ",
            " 3 ",
            "0[CSELF_67]", // After placeholder removal: "0"
            "3[AT_1]",     // After placeholder removal: "3"
            "5[CSELF_9][NEWLINE]", // After placeholder removal: "5"
            "10:",
            "3…",
            "0.",
            "123…",
            "3 (",
            "0 )",
            "3/",
            "0\\",
        ];

        for text in digits_only_texts {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                !result,
                "Text '{}' should be filtered out as it contains only digits (with or without placeholders/special chars)",
                text.escape_debug()
            );
        }

        // Test that texts with digits AND actual text are NOT filtered
        let digits_with_text = vec![
            "3個",
            "レベル3",
            "3回",
            "0円",
            "123点",
            "3個[CSELF_7]", // Has text "個" before placeholder
            "レベル3[AT_1]", // Has text "レベル" before placeholder
            "3回目",
            "0個",
            "123個",
        ];

        for text in digits_with_text {
            let result = WolfRpgTextValidator::validate_text(text);
            assert!(
                result,
                "Text '{}' should NOT be filtered out as it contains actual text with digits",
                text.escape_debug()
            );
        }
    }
}
