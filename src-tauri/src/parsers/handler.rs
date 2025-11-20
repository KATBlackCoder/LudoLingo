// Game Engine Handler Trait
// Defines the common interface for all game engine handlers

use crate::parsers::engine::{TextEntry, TranslationEntry};
use std::path::{Path, PathBuf};

/// Result of project structure validation
/// Contains detailed errors and warnings about the project structure
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the project structure is valid
    pub is_valid: bool,
    /// List of errors found during validation
    pub errors: Vec<String>,
    /// List of warnings found during validation
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new(is_valid: bool, errors: Vec<String>, warnings: Vec<String>) -> Self {
        Self {
            is_valid,
            errors,
            warnings,
        }
    }

    /// Create a valid validation result with no errors or warnings
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create an invalid validation result with errors
    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Create a validation result with warnings only
    pub fn with_warnings(warnings: Vec<String>) -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings,
        }
    }
}

/// Common trait for all game engine handlers
/// 
/// This trait defines the interface that all game engine handlers must implement.
/// It provides a uniform way to interact with different game engines (RPG Maker MV/MZ, WolfRPG, etc.)
/// without needing to know the specific implementation details.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use crate::parsers::handler::GameEngineHandler;
/// use std::path::Path;
/// 
/// // Get a handler from the factory
/// let handler = EngineFactory::create_handler(game_path)?;
/// 
/// // Validate the project structure
/// let validation = handler.validate_project_structure(game_path)?;
/// if !validation.is_valid {
///     eprintln!("Errors: {:?}", validation.errors);
/// }
/// 
/// // Extract all texts
/// let texts = handler.extract_all_texts(game_path)?;
/// 
/// // Inject translations
/// handler.inject_all_texts(game_path, &translations)?;
/// ```
pub trait GameEngineHandler: Send + Sync {
    /// Returns the human-readable name of the game engine
    /// 
    /// # Examples
    /// 
    /// - RPG Maker MZ: `"RPG Maker MZ"`
    /// - RPG Maker MV: `"RPG Maker MV"`
    /// - Wolf RPG Editor: `"Wolf RPG Editor"`
    /// 
    /// # Returns
    /// 
    /// A string slice containing the engine name
    fn engine_name(&self) -> &str;

    /// Validates the project structure and returns detailed errors and warnings
    /// 
    /// This method checks if the project directory has the correct structure for the game engine.
    /// It verifies the presence of required directories and files, and returns a detailed
    /// validation result with any issues found.
    /// 
    /// # Arguments
    /// 
    /// * `game_path` - Path to the game project root directory
    /// 
    /// # Returns
    /// 
    /// * `Ok(ValidationResult)` - Validation result with errors and warnings
    /// * `Err(String)` - Error occurred during validation (e.g., I/O error)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// let validation = handler.validate_project_structure(game_path)?;
    /// if !validation.is_valid {
    ///     for error in &validation.errors {
    ///         eprintln!("Error: {}", error);
    ///     }
    ///     for warning in &validation.warnings {
    ///         eprintln!("Warning: {}", warning);
    ///     }
    /// }
    /// ```
    fn validate_project_structure(&self, game_path: &Path) -> Result<ValidationResult, String>;

    /// Extracts all translatable texts from the game project
    /// 
    /// This method scans the project directory and extracts all texts that can be translated.
    /// It validates the project structure first, then extracts texts from all supported files.
    /// 
    /// # Arguments
    /// 
    /// * `game_path` - Path to the game project root directory
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<TextEntry>)` - List of extracted text entries
    /// * `Err(String)` - Error occurred during extraction (e.g., invalid structure, I/O error)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// let texts = handler.extract_all_texts(game_path)?;
    /// println!("Extracted {} text entries", texts.len());
    /// for entry in &texts {
    ///     println!("  - {}: {}", entry.id, entry.source_text);
    /// }
    /// ```
    fn extract_all_texts(&self, game_path: &Path) -> Result<Vec<TextEntry>, String>;

    /// Injects translations back into the game project files
    /// 
    /// This method takes a list of translations and injects them into the appropriate files
    /// in the game project. It matches translations to their original locations using the
    /// entry IDs.
    /// 
    /// # Arguments
    /// 
    /// * `game_path` - Path to the game project root directory
    /// * `translations` - Slice of translation entries to inject
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - Translations injected successfully
    /// * `Err(String)` - Error occurred during injection (e.g., file not found, I/O error)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// let translations = vec![
    ///     TranslationEntry {
    ///         id: "actor_1_name".to_string(),
    ///         translated_text: "Hero".to_string(),
    ///     },
    ///     // ... more translations
    /// ];
    /// 
    /// handler.inject_all_texts(game_path, &translations)?;
    /// println!("Translations injected successfully");
    /// ```
    fn inject_all_texts(
        &self,
        game_path: &Path,
        translations: &[TranslationEntry],
    ) -> Result<(), String>;

    /// Counts the number of files that will be processed during extraction/injection
    /// 
    /// This method provides an estimate of how many files will be processed, which is useful
    /// for progress tracking and user feedback.
    /// 
    /// # Arguments
    /// 
    /// * `game_path` - Path to the game project root directory
    /// 
    /// # Returns
    /// 
    /// The number of files that will be processed
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// let file_count = handler.count_files_to_process(game_path);
    /// println!("Will process {} files", file_count);
    /// ```
    fn count_files_to_process(&self, game_path: &Path) -> usize;

    /// Returns the data root directory path for the game engine
    /// 
    /// This method returns the path to the directory containing the game's data files.
    /// The location varies by engine:
    /// 
    /// - RPG Maker MZ: `data/`
    /// - RPG Maker MV: `www/data/`
    /// - Wolf RPG Editor: `dump/`
    /// 
    /// # Arguments
    /// 
    /// * `game_path` - Path to the game project root directory
    /// 
    /// # Returns
    /// 
    /// The path to the data root directory
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// let data_root = handler.get_data_root(game_path);
    /// println!("Data root: {}", data_root.display());
    /// ```
    fn get_data_root(&self, game_path: &Path) -> PathBuf;
}

