// RPG Maker file parsers module exports
// Each file type has its own parser with extract/inject methods

// ============================================================================
// IMPLEMENTED PARSERS
// ============================================================================

// Core game data parsers
pub mod actors; // Characters/actors data
pub mod armors; // Equipment and armor
pub mod classes; // Character classes
pub mod enemies; // Enemy/monster data
pub mod items; // Items and consumables
pub mod skills; // Abilities and skills
pub mod states; // Status effects and states
pub mod system;
pub mod troops; // Enemy encounter groups
pub mod weapons; // Weapons and equipment // System settings and terms

// Map data parsers
pub mod map_data;
pub mod map_infos; // Map information (names, structure) // Map content (events, tiles)

// ============================================================================
// UTILITY MODULES
// ============================================================================

pub mod common; // Shared parsing utilities and types
pub mod common_events; // Common events text extraction
pub mod handler; // Centralized extraction/injection handler

// ============================================================================
// FUTURE PARSERS (not yet implemented)
// ============================================================================

// None - all core parsers implemented!
