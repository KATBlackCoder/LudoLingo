/// Text formatters for different game engines
///
/// This module contains engine-specific formatters that convert
/// game-specific text codes to universal placeholders for translation.
pub mod formatter_trait;
pub mod rpg_maker_formatter;
pub mod universal_formatter;
pub mod wolf_rpg_formatter;

// Re-export for convenience
pub use formatter_trait::EngineFormatter;
pub use rpg_maker_formatter::RpgMakerFormatter;
pub use universal_formatter::UniversalFormatter;
pub use wolf_rpg_formatter::WolfRpgFormatter;

