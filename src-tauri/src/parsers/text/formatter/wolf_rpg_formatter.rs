use super::formatter_trait::EngineFormatter;
use super::universal_formatter::UniversalFormatter;
use once_cell::sync::Lazy;
use regex::Regex;

// to_ascii_digits is now handled by UniversalFormatter

// === PRE-COMPILED WOLF RPG REGEXES ===

// Wolf RPG specific regexes only

// ICON_REGEX: \i[number] - Affiche une icône dans le texte
// Exemple: \i[1] affiche l'icône numéro 1
static ICON_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\i\[(\d+)\]").unwrap());

// FONT_REGEX: \f[number] - Change la police de caractères (format simple avec nombre uniquement)
// Exemple: \f[2] change la police pour le numéro 2
static FONT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\f\[(\d+)\]").unwrap());

// AT_REGEX: @number - Référence à une variable ou un paramètre
// Exemple: @1 référence le paramètre 1
static AT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"@(\d+)").unwrap());

// SLOT_REGEX: \s[number] - Référence à un slot d'équipement
// Exemple: \s[0] référence le slot d'équipement 0
static SLOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\s\[(\d+)\]").unwrap());

// CSELF_REGEX: \cself[number] - Référence à une variable du personnage courant
// Exemple: \cself[19] référence la variable 19 du personnage actuel
static CSELF_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\cself\[(\d+)\]").unwrap());

// COLOR_REGEX_LOWER: \c[number] - Change la couleur du texte (minuscule, couleur de base)
// Exemple: \c[2] change la couleur pour le numéro 2
// Note: Différent de \C[ (majuscule) qui est une couleur spéciale
static COLOR_REGEX_LOWER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\c\[(\d+)\]").unwrap());

// COLOR_REGEX_UPPER: \C[number] - Change la couleur du texte (majuscule, couleur spéciale)
// Exemple: \C[3] change la couleur spéciale pour le numéro 3
// Note: Différent de \c[ (minuscule) qui est une couleur de base
static COLOR_REGEX_UPPER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\C\[(\d+)\]").unwrap());

// SYS_REGEX: \sys[number] - Référence à une variable système
// Exemple: \sys[100] référence la variable système 100
static SYS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\sys\[(\d+)\]").unwrap());

// FONT_FULL_REGEX: \font[number] - Change la police complète (format alternatif)
// Exemple: \font[1] change la police pour le numéro 1
static FONT_FULL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\font\[(\d+)\]").unwrap());

// AX_REGEX: \ax[expression] - Position X absolue pour le texte
// Exemple: \ax[\cself[13]] positionne le texte à la position X définie par la variable \cself[13]
static AX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\ax\[([^\]]+)\]").unwrap());

// AY_REGEX: \ay[expression] - Position Y absolue pour le texte
// Exemple: \ay[0] positionne le texte à la position Y 0
static AY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\ay\[([^\]]+)\]").unwrap());

// V_REGEX: \v[number] - Affiche la valeur d'une variable
// Exemple: \v[11] affiche la valeur de la variable 11
static V_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\v\[(\d+)\]").unwrap());

// F_SIMPLE_REGEX: \f[expression] - Change la police avec contenu complexe (peut contenir des codes imbriqués)
// Exemple: \f[\cself[19]] change la police avec une expression contenant \cself[19]
// Note: Différent de FONT_REGEX qui ne gère que \f[number] simple
static F_SIMPLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\f\[([^\]]+)\]").unwrap());

// CDB_REGEX: \cdb[type:index:field] - Accès à la base de données (format type:index:field)
// Exemple: \cdb[23:1:4] accède au type 23, index 1, champ 4 de la base de données
// Format: type (ex: 21=items, 23=armes), index (ID de l'item), field (champ à afficher)
static CDB_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\cdb\[(\d+:\d+:\d+)\]").unwrap());

// INDENT_REGEX: \-[number] - Indentation négative (vers la gauche)
// Exemple: \-[1] décale le texte d'1 unité vers la gauche
static INDENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\-\[(\d+)\]").unwrap());

// SPACE_REGEX: \space[number] - Contrôle de l'espacement
// Exemple: \space[0] définit l'espacement à 0
static SPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\space\[(\d+)\]").unwrap());

// Wolf RPG specific regexes only

// === RESTORATION REGEXES ===
// Ces regexes restaurent les codes Wolf RPG après traduction
// Format: [CODE_NAME] -> \code[value]

// ICON_RESTORE_REGEX: Restaure [ICON_number] vers \i[number]
static ICON_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[ICON_(\d+)\]").unwrap());

// FONT_RESTORE_REGEX: Restaure [FONT_number] vers \f[number]
static FONT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[FONT_(\d+)\]").unwrap());

// AT_RESTORE_REGEX: Restaure [AT_number] vers @number
static AT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[AT_(\d+)\]").unwrap());

// SLOT_RESTORE_REGEX: Restaure [SLOT_number] vers \s[number]
static SLOT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[SLOT_(\d+)\]").unwrap());

// CSELF_RESTORE_REGEX: Restaure [CSELF_number] vers \cself[number]
static CSELF_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[CSELF_(\d+)\]").unwrap());

// COLOR_RESTORE_LOWER_REGEX: Restaure [COLOR_LOWER_number] vers \c[number] (couleur de base)
// Préserve la distinction minuscule/majuscule importante dans Wolf RPG
static COLOR_RESTORE_LOWER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[COLOR_LOWER_(\d+)\]").unwrap());

// COLOR_RESTORE_UPPER_REGEX: Restaure [COLOR_UPPER_number] vers \C[number] (couleur spéciale)
// Préserve la distinction minuscule/majuscule importante dans Wolf RPG
static COLOR_RESTORE_UPPER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[COLOR_UPPER_(\d+)\]").unwrap());

// SYS_RESTORE_REGEX: Restaure [SYS_number] vers \sys[number]
static SYS_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[SYS_(\d+)\]").unwrap());

// FONT_FULL_RESTORE_REGEX: Restaure [FONT_FULL_number] vers \font[number]
static FONT_FULL_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[FONT_FULL_(\d+)\]").unwrap());

// AX_RESTORE_REGEX: Restaure [AX_expression] vers \ax[expression]
static AX_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[AX_([^\]]+)\]").unwrap());

// AY_RESTORE_REGEX: Restaure [AY_expression] vers \ay[expression]
static AY_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[AY_([^\]]+)\]").unwrap());

// V_RESTORE_REGEX: Restaure [V_number] vers \v[number]
static V_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[V_(\d+)\]").unwrap());

// F_SIMPLE_RESTORE_REGEX: Restaure [F_SIMPLE_expression] vers \f[expression]
// Gère les expressions complexes avec codes imbriqués
static F_SIMPLE_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[F_SIMPLE_([^\]]+)\]").unwrap());

// CDB_RESTORE_REGEX: Restaure [CDB_type:index:field] vers \cdb[type:index:field]
static CDB_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[CDB_(\d+:\d+:\d+)\]").unwrap());

// INDENT_RESTORE_REGEX: Restaure [INDENT_number] vers \-[number]
static INDENT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[INDENT_(\d+)\]").unwrap());

// SPACE_RESTORE_REGEX: Restaure [SPACE_number] vers \space[number]
static SPACE_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[SPACE_(\d+)\]").unwrap());

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
        // Color codes: preserve case distinction (\\c[ vs \\C[)
        result = COLOR_REGEX_LOWER
            .replace_all(&result, "[COLOR_LOWER_$1]")
            .to_string();
        result = COLOR_REGEX_UPPER
            .replace_all(&result, "[COLOR_UPPER_$1]")
            .to_string();
        
        // Additional Wolf RPG codes
        result = SYS_REGEX.replace_all(&result, "[SYS_$1]").to_string();
        result = FONT_FULL_REGEX.replace_all(&result, "[FONT_FULL_$1]").to_string();
        result = AX_REGEX.replace_all(&result, "[AX_$1]").to_string();
        result = AY_REGEX.replace_all(&result, "[AY_$1]").to_string();
        result = V_REGEX.replace_all(&result, "[V_$1]").to_string();
        // \f[ with complex content (not just a number) - must be after FONT_REGEX
        // FONT_REGEX already handles \f[number], so F_SIMPLE_REGEX handles \f[complex]
        result = F_SIMPLE_REGEX.replace_all(&result, "[F_SIMPLE_$1]").to_string();
        // \cdb[ with format number:number:number
        result = CDB_REGEX.replace_all(&result, "[CDB_$1]").to_string();
        // \-[number] indentation and \space[number] spacing
        result = INDENT_REGEX.replace_all(&result, "[INDENT_$1]").to_string();
        result = SPACE_REGEX.replace_all(&result, "[SPACE_$1]").to_string();
        result = result.replace("<C>", "[CENTER_TAG]");
        result = result.replace("\\>", "[RIGHT_ALIGN]");
        result = result.replace("<R>", "[RIGHT_TAG]");
        result = result.replace("<<", "[LEFT_BRACKETS]");
        result = result.replace(">>", "[RIGHT_BRACKETS]");

        // Other Wolf RPG codes
        result = result.replace("\\r", "[RUBY_START]");
        result = result.replace('\r', "[CARRIAGE_RETURN]");
        result = result.replace('\n', "[NEWLINE]");

        // === UNIVERSAL PATTERNS (delegate to UniversalFormatter) ===
        // Note: Japanese quotation marks normalization is now handled by UniversalFormatter
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
        // Color codes: restore with correct case
        result = COLOR_RESTORE_LOWER_REGEX
            .replace_all(&result, "\\c[$1]")
            .to_string();
        result = COLOR_RESTORE_UPPER_REGEX
            .replace_all(&result, "\\C[$1]")
            .to_string();
        
        // Additional Wolf RPG codes restoration
        result = SYS_RESTORE_REGEX.replace_all(&result, "\\sys[$1]").to_string();
        result = FONT_FULL_RESTORE_REGEX.replace_all(&result, "\\font[$1]").to_string();
        result = AX_RESTORE_REGEX.replace_all(&result, "\\ax[$1]").to_string();
        result = AY_RESTORE_REGEX.replace_all(&result, "\\ay[$1]").to_string();
        result = V_RESTORE_REGEX.replace_all(&result, "\\v[$1]").to_string();
        result = F_SIMPLE_RESTORE_REGEX.replace_all(&result, "\\f[$1]").to_string();
        result = CDB_RESTORE_REGEX.replace_all(&result, "\\cdb[$1]").to_string();
        result = INDENT_RESTORE_REGEX.replace_all(&result, "\\-[$1]").to_string();
        result = SPACE_RESTORE_REGEX.replace_all(&result, "\\space[$1]").to_string();
        result = result.replace("[CENTER_TAG]", "<C>");
        result = result.replace("[RIGHT_ALIGN]", "\\>");
        result = result.replace("[RIGHT_TAG]", "<R>");
        result = result.replace("[LEFT_BRACKETS]", "<<");
        result = result.replace("[RIGHT_BRACKETS]", ">>");

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
        // Note: \\C[ is now supported as a Wolf RPG color code, so we use \\N[ and \\I[ instead
        let input = "\\N[1]勇者\\I[317]テスト";
        let result = WolfRpgFormatter::prepare_for_translation(input);
        // RPG Maker codes should remain unchanged
        assert!(
            result.contains("\\N[1]"),
            "RPG Maker codes should be ignored"
        );
        assert!(
            result.contains("\\I[317]"),
            "RPG Maker codes should be ignored"
        );
    }

    #[test]
    fn test_wolf_rpg_color_codes() {
        // Test Wolf RPG color codes (\\c[ lowercase and \\C[ uppercase)
        let input_lower = "\\E\\c[2]ほのか\n「さて、着替え着替え」";
        let expected_prepared_lower =
            "[WOLF_END][COLOR_LOWER_2]ほのか[NEWLINE]\"さて、着替え着替え\"";
        let expected_restored_lower = "\\E\\c[2]ほのか\n\"さて、着替え着替え\"";

        let prepared_lower = WolfRpgFormatter::prepare_for_translation(input_lower);
        assert_eq!(
            prepared_lower, expected_prepared_lower,
            "Wolf RPG lowercase color code should be converted to [COLOR_LOWER_X]"
        );

        let restored_lower = WolfRpgFormatter::restore_after_translation(&prepared_lower);
        assert_eq!(
            restored_lower, expected_restored_lower,
            "Wolf RPG lowercase color code should be restored to \\c[X]"
        );

        // Test uppercase color code
        let input_upper = "\\E\\C[3]いぶき\n「テスト」";
        let expected_prepared_upper = "[WOLF_END][COLOR_UPPER_3]いぶき[NEWLINE]\"テスト\"";
        let expected_restored_upper = "\\E\\C[3]いぶき\n\"テスト\"";

        let prepared_upper = WolfRpgFormatter::prepare_for_translation(input_upper);
        assert_eq!(
            prepared_upper, expected_prepared_upper,
            "Wolf RPG uppercase color code should be converted to [COLOR_UPPER_X]"
        );

        let restored_upper = WolfRpgFormatter::restore_after_translation(&prepared_upper);
        assert_eq!(
            restored_upper, expected_restored_upper,
            "Wolf RPG uppercase color code should be restored to \\C[X]"
        );
    }

    #[test]
    fn test_additional_wolf_rpg_codes() {
        // Test new Wolf RPG codes: \sys, <C>, \>, \font, \ax, \ay, <R>, \v
        let input = "<<　\\sys[100] ％　>>";
        let prepared = WolfRpgFormatter::prepare_for_translation(input);
        assert!(prepared.contains("[SYS_100]"), "\\sys[100] should be converted to [SYS_100]");
        
        let restored = WolfRpgFormatter::restore_after_translation(&prepared);
        assert!(restored.contains("\\sys[100]"), "\\sys[100] should be restored");

        // Test <C> tag
        let input_c = "<C>\\cself[7]";
        let prepared_c = WolfRpgFormatter::prepare_for_translation(input_c);
        assert!(prepared_c.contains("[CENTER_TAG]"), "<C> should be converted to [CENTER_TAG]");
        let restored_c = WolfRpgFormatter::restore_after_translation(&prepared_c);
        assert!(restored_c.contains("<C>"), "<C> should be restored");

        // Test \> alignment
        let input_right = "@1\n\\>\\cself[9]";
        let prepared_right = WolfRpgFormatter::prepare_for_translation(input_right);
        assert!(prepared_right.contains("[RIGHT_ALIGN]"), "\\> should be converted to [RIGHT_ALIGN]");
        let restored_right = WolfRpgFormatter::restore_after_translation(&prepared_right);
        assert!(restored_right.contains("\\>"), "\\> should be restored");

        // Test \font
        let input_font = "\\font[1]+\\cself[15]";
        let prepared_font = WolfRpgFormatter::prepare_for_translation(input_font);
        assert!(prepared_font.contains("[FONT_FULL_1]"), "\\font[1] should be converted to [FONT_FULL_1]");
        let restored_font = WolfRpgFormatter::restore_after_translation(&prepared_font);
        assert!(restored_font.contains("\\font[1]"), "\\font[1] should be restored");

        // Test \ax, \ay, <R>
        let input_ax_ay = "\\f[\\cself[19]]所持数\\ax[\\cself[13]]\n\\ay[0]<R>\\f[\\cself[19]]\\cself[97]";
        let prepared_ax_ay = WolfRpgFormatter::prepare_for_translation(input_ax_ay);
        assert!(prepared_ax_ay.contains("[AX_"), "\\ax[...] should be converted");
        assert!(prepared_ax_ay.contains("[AY_"), "\\ay[...] should be converted");
        assert!(prepared_ax_ay.contains("[RIGHT_TAG]"), "<R> should be converted to [RIGHT_TAG]");
        let restored_ax_ay = WolfRpgFormatter::restore_after_translation(&prepared_ax_ay);
        assert!(restored_ax_ay.contains("\\ax["), "\\ax[...] should be restored");
        assert!(restored_ax_ay.contains("\\ay["), "\\ay[...] should be restored");
        assert!(restored_ax_ay.contains("<R>"), "<R> should be restored");

        // Test \v
        let input_v = "\\f[20]現在の借金\\v[11]00円";
        let prepared_v = WolfRpgFormatter::prepare_for_translation(input_v);
        assert!(prepared_v.contains("[V_11]"), "\\v[11] should be converted to [V_11]");
        let restored_v = WolfRpgFormatter::restore_after_translation(&prepared_v);
        assert!(restored_v.contains("\\v[11]"), "\\v[11] should be restored");

        // Test << and >>
        let input_brackets = "<<　\\sys[100] ％　>>";
        let prepared_brackets = WolfRpgFormatter::prepare_for_translation(input_brackets);
        assert!(prepared_brackets.contains("[LEFT_BRACKETS]"), "<< should be converted to [LEFT_BRACKETS]");
        assert!(prepared_brackets.contains("[RIGHT_BRACKETS]"), ">> should be converted to [RIGHT_BRACKETS]");
        let restored_brackets = WolfRpgFormatter::restore_after_translation(&prepared_brackets);
        assert!(restored_brackets.contains("<<"), "<< should be restored");
        assert!(restored_brackets.contains(">>"), ">> should be restored");

        // Test \f[ with complex content
        let input_f = "\\f[\\cself[19]]購入";
        let prepared_f = WolfRpgFormatter::prepare_for_translation(input_f);
        assert!(prepared_f.contains("[F_SIMPLE_"), "\\f[\\cself[19]] should be converted to [F_SIMPLE_...]");
        let restored_f = WolfRpgFormatter::restore_after_translation(&prepared_f);
        assert!(restored_f.contains("\\f[\\cself[19]]"), "\\f[\\cself[19]] should be restored");

        // Test \cdb[
        let input_cdb = "\\E\\c[2]ほのか\n「おお！　\\cdb[23:1:4]00円も入ってる！」";
        let prepared_cdb = WolfRpgFormatter::prepare_for_translation(input_cdb);
        assert!(prepared_cdb.contains("[CDB_23:1:4]"), "\\cdb[23:1:4] should be converted to [CDB_23:1:4]");
        let restored_cdb = WolfRpgFormatter::restore_after_translation(&prepared_cdb);
        assert!(restored_cdb.contains("\\cdb[23:1:4]"), "\\cdb[23:1:4] should be restored");

        // Test multiple \cdb[
        let input_cdb_multi = "\\>\\f[5]レベル\\cself[30]  / \\cdb[21:78:0] \\cdb[21:80:0] \\cdb[21:81:0]";
        let prepared_cdb_multi = WolfRpgFormatter::prepare_for_translation(input_cdb_multi);
        assert!(prepared_cdb_multi.contains("[CDB_21:78:0]"), "\\cdb[21:78:0] should be converted");
        assert!(prepared_cdb_multi.contains("[CDB_21:80:0]"), "\\cdb[21:80:0] should be converted");
        assert!(prepared_cdb_multi.contains("[CDB_21:81:0]"), "\\cdb[21:81:0] should be converted");
        let restored_cdb_multi = WolfRpgFormatter::restore_after_translation(&prepared_cdb_multi);
        assert!(restored_cdb_multi.contains("\\cdb[21:78:0]"), "\\cdb[21:78:0] should be restored");
        assert!(restored_cdb_multi.contains("\\cdb[21:80:0]"), "\\cdb[21:80:0] should be restored");
        assert!(restored_cdb_multi.contains("\\cdb[21:81:0]"), "\\cdb[21:81:0] should be restored");
    }

    #[test]
    fn test_indent_and_space_codes() {
        // Test \-[number] indentation and \space[number] spacing codes
        let input = "\\-[1]<C>\\E\\space[0]\\f[\\cself[17]]\\cself[7]";
        let expected_prepared = "[INDENT_1][CENTER_TAG][WOLF_END][SPACE_0][F_SIMPLE_[CSELF_17]][CSELF_7]";
        let expected_restored = "\\-[1]<C>\\E\\space[0]\\f[\\cself[17]]\\cself[7]";

        let prepared = WolfRpgFormatter::prepare_for_translation(input);
        assert_eq!(
            prepared, expected_prepared,
            "Indent and space codes should be converted to placeholders"
        );

        let restored = WolfRpgFormatter::restore_after_translation(&prepared);
        assert_eq!(
            restored, expected_restored,
            "Indent and space codes should be restored to original format"
        );

        // Test multiple levels
        let input_multi = "\\-[2]\\space[5]\\-[0]\\space[1]";
        let expected_prepared_multi = "[INDENT_2][SPACE_5][INDENT_0][SPACE_1]";
        let expected_restored_multi = "\\-[2]\\space[5]\\-[0]\\space[1]";

        let prepared_multi = WolfRpgFormatter::prepare_for_translation(input_multi);
        assert_eq!(
            prepared_multi, expected_prepared_multi,
            "Multiple indent and space codes should be converted"
        );

        let restored_multi = WolfRpgFormatter::restore_after_translation(&prepared_multi);
        assert_eq!(
            restored_multi, expected_restored_multi,
            "Multiple indent and space codes should be restored"
        );
    }

    #[test]
    fn test_formatting_codes_detection() {
        // Test that has_formatting_codes and has_placeholder_codes work correctly
        // with the specific string: "\\-[1]<C>\\E\\space[0]\\f[\\cself[17]]\\cself[7]"

        let original_string = "\\-[1]<C>\\E\\space[0]\\f[\\cself[17]]\\cself[7]";
        let prepared_string = "[INDENT_1][CENTER_TAG][WOLF_END][SPACE_0][F_SIMPLE_[CSELF_17]][CSELF_7]";

        // Test that original string is detected as having formatting codes
        assert!(
            WolfRpgFormatter::has_formatting_codes(original_string),
            "Original string should be detected as having formatting codes"
        );

        // Test that prepared string is detected as having placeholder codes
        assert!(
            WolfRpgFormatter::has_placeholder_codes(prepared_string),
            "Prepared string should be detected as having placeholder codes"
        );

        // Test that plain text without codes is not detected
        let plain_text = "Hello world";
        assert!(
            !WolfRpgFormatter::has_formatting_codes(plain_text),
            "Plain text should not be detected as having formatting codes"
        );
        assert!(
            !WolfRpgFormatter::has_placeholder_codes(plain_text),
            "Plain text should not be detected as having placeholder codes"
        );
    }
}
