/// Universal validation for text content
///
/// This module provides unified validation logic that works for all engines
/// without engine-specific knowledge.
pub mod validation;

pub use validation::ContentValidator;

