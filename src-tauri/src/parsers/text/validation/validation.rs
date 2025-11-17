/// Universal validation: common logic for all engines
///
/// This struct provides unified validation logic that works for all engines
/// without engine-specific knowledge.
pub struct ContentValidator;

impl ContentValidator {
    /// Determine the initial translation status based on content and translation context
    ///
    /// This method determines if text should be marked as Ignored when translating
    /// from CJK to ASCII and the text is already in ASCII form.
    pub fn get_initial_status(
        content: &str,
        target_language: &str,
    ) -> crate::parsers::engine::TranslationStatus {
        let content = content.trim();

        // Check if we're translating to English/ASCII
        let is_ascii_target =
            target_language.to_lowercase() == "en" || target_language.to_lowercase() == "english";

        if is_ascii_target {
            // Check if the text is already in ASCII form
            let is_ascii_text = content.chars().all(|c| {
                c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace()
            });

            // If text is already ASCII and we're translating to English, ignore it
            // (no need to translate English to English)
            if is_ascii_text && content.chars().any(|c| c.is_alphabetic()) {
                return crate::parsers::engine::TranslationStatus::Ignored;
            }
        }

        // Default to NotTranslated for all other cases
        crate::parsers::engine::TranslationStatus::NotTranslated
    }

    /// Universal validation logic for text content
    ///
    /// This method determines if text should be translated based on common
    /// validation rules that apply to all engines.
    pub fn validate_text(content: &str) -> bool {
        let content = content.trim();

        // Skip empty or whitespace-only content
        if content.is_empty() {
            return false;
        }

        // Skip text that contains only placeholders (e.g., "[NEWLINE_1]", "[COLOR_1]", etc.)
        // Pattern: text wrapped entirely in brackets like [PLACEHOLDER_NAME] or [PLACEHOLDER_NAME_123]
        if content.starts_with('[') && content.ends_with(']') && content.len() > 2 {
            // Check if it's a valid placeholder pattern: [A-Z_][A-Z0-9_]*
            let inner = &content[1..content.len() - 1];
            if inner
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                && inner
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_ascii_uppercase() || c == '_')
            {
                return false;
            }
        }

        // Skip text that contains only placeholders with optional whitespace
        // Handles cases like " [NEWLINE_1] ", "[COLOR_1][COLOR_0]", etc.
        // But NOT "[COLOR_1]勇者[COLOR_0]" which contains real content
        if content.contains('[') && content.contains(']') {
            // Remove all placeholder patterns [SOMETHING] from the content
            // Then check if anything remains (real content)
            let mut remaining = content.to_string();
            let mut found_placeholder = false;

            // Find and remove all placeholder patterns
            loop {
                let mut changed = false;
                // Find pattern [A-Z_][A-Z0-9_]*
                if let Some(start) = remaining.find('[') {
                    if let Some(end) = remaining[start..].find(']') {
                        let placeholder = &remaining[start..start + end + 1];
                        let inner = &placeholder[1..placeholder.len() - 1];
                        // Check if it's a valid placeholder pattern
                        if !inner.is_empty()
                            && inner
                                .chars()
                                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                            && inner
                                .chars()
                                .next()
                                .map_or(false, |c| c.is_ascii_uppercase() || c == '_')
                        {
                            remaining.replace_range(start..start + end + 1, "");
                            found_placeholder = true;
                            changed = true;
                        }
                    }
                }
                if !changed {
                    break;
                }
            }

            // If we found placeholders and nothing remains (only whitespace), it's only placeholders
            if found_placeholder && remaining.trim().is_empty() {
                return false;
            }
        }

        // Skip single Japanese technical markers (〇 = maru/circle, ｘ = batsu/X)
        // These are used as yes/no markers in games and are not translatable text
        if content == "〇" || content == "ｘ" || content == "○" || content == "×" {
            return false;
        }

        // Note: Punctuation-only validation moved to engine-specific validators
        // (rpg_maker/text_validation.rs and wolfrpg/text_validation.rs)

        // Detect if the content visually looks like CJK (Han, Kana, Hangul) or uses JP punctuation
        let looks_cjk = content.chars().any(|c| {
            (c >= '\u{4E00}' && c <= '\u{9FFF}') || // CJK Unified Ideographs
            (c >= '\u{3040}' && c <= '\u{309F}') || // Hiragana
            (c >= '\u{30A0}' && c <= '\u{30FF}') || // Katakana
            (c >= '\u{AC00}' && c <= '\u{D7AF}') || // Hangul Syllables
            Self::contains_japanese_punctuation(content)
        });

        // Skip EVXXX event names (technical identifiers)
        // Skip any text that starts with "EV" followed by numbers (e.g., "EV0", "EV1", "EV123", "EV002物乞いＢ")
        if content.starts_with("EV") && content.len() >= 3 {
            let suffix = &content[2..];
            // Check if the suffix starts with numbers (allows for additional content after numbers)
            if suffix.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                return false;
            }
        }

        // Skip MAPXXX switch names (technical identifiers)
        if content.starts_with("MAP") && content.len() >= 4 {
            let suffix = &content[3..];
            if suffix.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }

        // Skip text that contains only Japanese quotation marks without any actual content
        // Handle cases: "「", "」", "「」", "「 ", " 」", "「 」", and variations with spaces
        let trimmed = content.trim();
        if trimmed == "「" || trimmed == "」" || trimmed == "「」" {
            return false;
        }

        // Also check for quotes with spaces between them (like "「 」")
        // Check if the text contains only Japanese quotes and spaces
        if trimmed.chars().all(|c| c == '「' || c == '」' || c == ' ')
            && trimmed.chars().any(|c| c == '「' || c == '」')
        {
            return false;
        }

        // Skip pure formatting codes (like "\\n[2]" alone)
        if content == "\\n[1]"
            || content == "\\n[2]"
            || content == "\\n[3]"
            || content == "\\n[4]"
            || content == "\\n[5]"
        {
            return false;
        }

        // Note: File path validation moved to engine-specific validators
        // (rpg_maker/text_validation.rs and wolfrpg/text_validation.rs)

        // Skip file extensions (e.g., "image.png", "sound.mp3", "file.txt")
        // Pattern: ends with dot + extension (2-4 letters), optionally preceded by alphanumeric
        if content.contains('.') {
            // Check if it looks like a file extension pattern
            // Examples: "file.png", "image.jpg", "sound.mp3", ".png", "test.txt"
            let parts: Vec<&str> = content.split('.').collect();
            if parts.len() == 2 {
                let extension = parts[1];
                // Extension should be short (2-4 chars) and alphanumeric only
                if extension.len() >= 2
                    && extension.len() <= 4
                    && extension.chars().all(|c| c.is_ascii_alphanumeric())
                    && extension.chars().any(|c| c.is_ascii_alphabetic())
                {
                    return false;
                }
            }
            // Multiple dots might be a path like "path/to/file.ext"
            if parts.len() > 2 {
                return false;
            }
        }

        // Skip JavaScript code and expressions
        if content.contains("user.")
            || content.contains("use.")
            || content.contains("&&")
            || content.contains("==")
        {
            return false;
        }

        // Skip technical markers
        if content == "終わり" || content == "==" || content.starts_with("==") {
            return false;
        }

        // Skip any text with pipe characters (e.g., "はい|262|380", "戻る|492|380", "text|more|text")
        if content.contains('|') {
            return false;
        }

        // Skip sound effect-like short ASCII words only when embedded in CJK-looking content
        if looks_cjk && content.chars().all(|c| c.is_ascii_alphabetic()) && content.len() <= 20 {
            return false;
        }

        // Skip pure ASCII/Latin text only when content overall looks CJK
        if looks_cjk && content.chars().all(Self::is_ascii_or_fullwidth_latin) {
            return false;
        }

        // Skip technical variable names and identifiers
        if content.contains('_')
            && content
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == 'x' || c == 'X')
        {
            return false;
        }

        // Skip numeric-only content
        if content.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        // Skip very short content only in CJK-looking context
        // But allow Japanese/Chinese characters even if short
        if looks_cjk && content.len() <= 3 {
            // If it contains non-ASCII characters or Japanese punctuation, it might be translatable
            if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii())
                || Self::contains_japanese_punctuation(content)
            {
                return true;
            }
            return false;
        }

        // If content contains Japanese characters or other translatable text, allow it
        if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii()) {
            return true;
        }

        // If content contains common Japanese punctuation or quotes, allow it
        if Self::contains_japanese_punctuation(content) {
            return true;
        }

        // Allow reasonable ASCII text that looks like translatable content
        // (alphabetic characters, reasonable length, not just technical identifiers)
        if content.chars().any(|c| c.is_alphabetic())
            && content.len() >= 2
            && content.len() <= 100
            && !content.chars().all(|c| c.is_ascii_digit())
        {
            return true;
        }

        false
    }

    /// Optional warnings for text that passed validation
    ///
    /// This method can provide warnings about text that might need special attention
    /// during translation, such as very long text or unusual formatting.
    pub fn get_warnings(content: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        let content = content.trim();

        // Very long text warning
        if content.len() > 1000 {
            warnings.push("Very long text - may need special handling".to_string());
        }

        // Unusual formatting warning
        if content.contains("\\") && content.len() < 10 {
            warnings.push("Short text with formatting codes - verify translation".to_string());
        }

        // Mixed script warning
        let has_cjk = content.chars().any(|c| {
            (c >= '\u{4E00}' && c <= '\u{9FFF}') || // CJK Unified Ideographs
            (c >= '\u{3040}' && c <= '\u{309F}') || // Hiragana
            (c >= '\u{30A0}' && c <= '\u{30FF}') || // Katakana
            (c >= '\u{AC00}' && c <= '\u{D7AF}') // Hangul Syllables
        });
        let has_ascii = content.chars().any(|c| c.is_ascii_alphanumeric());

        if has_cjk && has_ascii {
            warnings.push("Mixed script content - verify translation direction".to_string());
        }

        warnings
    }

    /// Helper function to check if a character is ASCII or full-width Latin
    fn is_ascii_or_fullwidth_latin(c: char) -> bool {
        c.is_ascii_alphanumeric() ||
        c.is_ascii_punctuation() ||
        c.is_ascii_whitespace() ||
        // Full-width Latin characters (U+FF21-FF5A for letters, U+FF10-FF19 for digits)
        (c >= '\u{FF21}' && c <= '\u{FF5A}') || // Full-width uppercase letters
        (c >= '\u{FF41}' && c <= '\u{FF5A}') || // Full-width lowercase letters  
        (c >= '\u{FF10}' && c <= '\u{FF19}') // Full-width digits
    }

    /// Helper function to check if content contains Japanese punctuation
    fn contains_japanese_punctuation(content: &str) -> bool {
        content.contains('「')
            || content.contains('」')
            || content.contains('、')
            || content.contains('。')
            || content.contains('・')
            || content.contains('…')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_japanese_quotation_marks_validation() {
        // Test cases for Japanese quotation marks that should be filtered out
        let quotation_texts = vec![
            "「",     // Only opening quote
            "」",     // Only closing quote
            "「」",   // Empty quotes
            "「 ",    // Opening quote with space
            " 」",    // Closing quote with space
            "「 」",  // Empty quotes with space
            " 「",    // Space before opening quote
            " 」",    // Space before closing quote
            " 「 」", // Space around empty quotes
        ];

        println!("Testing Japanese quotation marks (should be filtered out):");
        for text in quotation_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (should be filtered)"
                } else {
                    "FILTERED (correct)"
                }
            );
            assert!(!result, 
                "Text '{}' should be filtered out because it contains only Japanese quotation marks", text);
        }

        // Test cases for legitimate text that should NOT be filtered out
        let legitimate_texts = vec![
            "「勇者」",       // Quotes with content
            "「魔法使い」",   // Quotes with content
            "勇者",           // No quotes
            "魔法使い",       // No quotes
            "「こんにちは」", // Quotes with content
        ];

        println!("\nTesting legitimate texts with quotes (should NOT be filtered out):");
        for text in legitimate_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (correct)"
                } else {
                    "FILTERED (should not be)"
                }
            );
            assert!(result, "Text '{}' should NOT be filtered out", text);
        }
    }

    #[test]
    fn test_ev_validation() {
        // Test cases for EV text that should be filtered out
        let ev_texts = vec![
            "EV0",           // Pure technical identifier
            "EV1",           // Pure technical identifier
            "EV123",         // Pure technical identifier
            "EV002物乞いＢ", // EV + numbers + Japanese content
            "EV123見張り",   // EV + numbers + Japanese content
            "EV456テスト",   // EV + numbers + Japanese content
            "EV999",         // Pure technical identifier
        ];

        println!("Testing EV texts (should be filtered out):");
        for text in ev_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (should be filtered)"
                } else {
                    "FILTERED (correct)"
                }
            );
            assert!(
                !result,
                "Text '{}' should be filtered out because it starts with EV followed by numbers",
                text
            );
        }

        // Test cases for legitimate text that should NOT be filtered out
        let legitimate_texts = vec![
            "EVENT",       // Not EV + numbers
            "EVENT物乞い", // Not EV + numbers
            "EVENTテスト", // Not EV + numbers
            "EVENT123",    // Not EV + numbers (EVENT is different from EV)
            "EVENTBEGGAR", // Not EV + numbers (no underscores to avoid technical variable rule)
        ];

        println!("\nTesting legitimate texts (should NOT be filtered out):");
        for text in legitimate_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (correct)"
                } else {
                    "FILTERED (should not be)"
                }
            );
            assert!(result, "Text '{}' should NOT be filtered out", text);
        }
    }

    #[test]
    fn test_pipe_separated_coordinates_filtering() {
        // Test cases for any text with pipe characters that should be filtered out
        let pipe_texts = vec![
            "はい|262|380",
            "戻る|492|380",
            "text|123|456",
            "日本語|999|888",
            "abc|1|2|3",
            "test|0|0",
            "text|more|text",  // Pipe with non-numeric parts
            "abc|def|ghi",     // All non-numeric
            "test|123abc|456", // Mixed numeric and alphabetic
        ];

        println!("Testing texts with pipes (should be filtered out):");
        for text in pipe_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (should be filtered)"
                } else {
                    "FILTERED (correct)"
                }
            );
            assert!(
                !result,
                "Text '{}' should be filtered out because it contains pipe characters",
                text
            );
        }

        // Test cases for legitimate text that should NOT be filtered out
        let legitimate_texts = vec![
            "はい",             // Just Japanese text
            "戻る",             // Just Japanese text
            "hello world",      // Regular text
            "日本語のテキスト", // Japanese text
        ];

        println!("\nTesting legitimate texts (should NOT be filtered out):");
        for text in legitimate_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (correct)"
                } else {
                    "FILTERED (should not be)"
                }
            );
            assert!(result, "Text '{}' should NOT be filtered out", text);
        }
    }

    #[test]
    fn test_placeholder_only_validation() {
        // Test cases for placeholders that should be filtered out
        let placeholder_texts = vec![
            "[NEWLINE_1]",
            "[COLOR_1]",
            "[ITEM_5]",
            "[CTRL_WAIT]",
            "[ARG_1]",
            "[ICON_3]",
            "[WEAPON_1]",
            "[ARMOR_3]",
            "[COLOR_0]",
            "[NUM_PREFIX_100]",
        ];

        println!("Testing placeholder-only texts (should be filtered out):");
        for text in placeholder_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (should be filtered)"
                } else {
                    "FILTERED (correct)"
                }
            );
            assert!(
                !result,
                "Text '{}' should be filtered out because it contains only a placeholder",
                text
            );
        }

        // Test cases for multiple placeholders that should be filtered out
        let multiple_placeholders = vec![
            "[COLOR_1][COLOR_0]",
            "[ITEM_5] [ITEM_5]",
            "[CTRL_WAIT] [CTRL_NEWLINE]",
            " [NEWLINE_1] ",
            "[COLOR_1] [COLOR_0] [ITEM_5]",
        ];

        println!("\nTesting multiple placeholder-only texts (should be filtered out):");
        for text in multiple_placeholders {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (should be filtered)"
                } else {
                    "FILTERED (correct)"
                }
            );
            assert!(
                !result,
                "Text '{}' should be filtered out because it contains only placeholders",
                text
            );
        }

        // Test cases for legitimate text with placeholders that should NOT be filtered out
        let legitimate_texts = vec![
            "[COLOR_1]勇者[COLOR_0]",
            "Hello [ITEM_5] world",
            "[ICON_3]The tentacle monster",
            "Text with [CTRL_WAIT] content",
            "[COLOR_1]Le héros[COLOR_0]",
        ];

        println!("\nTesting legitimate texts with placeholders (should NOT be filtered out):");
        for text in legitimate_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (correct)"
                } else {
                    "FILTERED (should not be)"
                }
            );
            assert!(result, "Text '{}' should NOT be filtered out", text);
        }
    }

    #[test]
    fn test_punctuation_only_validation() {
        // Test cases for punctuation-only texts that should be filtered out
        let punctuation_texts = vec![
            "?", "!", ".", "? !", "...", "？", "！", "。", "、", "？！", "? ! .", "…", "・", "：",
            "；",
        ];

        println!("Testing punctuation-only texts (should be filtered out):");
        for text in punctuation_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (should be filtered)"
                } else {
                    "FILTERED (correct)"
                }
            );
            assert!(
                !result,
                "Text '{}' should be filtered out because it contains only punctuation",
                text
            );
        }

        // Test cases for legitimate text with punctuation that should NOT be filtered out
        let legitimate_texts = vec![
            "Hello?",
            "World!",
            "Test.",
            "こんにちは？",
            "勇者！",
            "魔法使い。",
            "Text with ? punctuation",
            "Hello! World.",
        ];

        println!("\nTesting legitimate texts with punctuation (should NOT be filtered out):");
        for text in legitimate_texts {
            let result = ContentValidator::validate_text(text);
            println!(
                "  '{}' -> {}",
                text,
                if result {
                    "PASSED (correct)"
                } else {
                    "FILTERED (should not be)"
                }
            );
            assert!(result, "Text '{}' should NOT be filtered out", text);
        }
    }
}
