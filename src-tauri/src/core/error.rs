// Error handling for the application

use std::fmt;

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;

/// Application error types
#[derive(Debug)]
pub enum AppError {
    /// File system operations failed
    FileSystem(String),
    /// JSON parsing failed
    Parsing(String),
    /// Validation failed
    Validation(String),
    /// Translation operation failed
    Translation(String),
    /// Configuration error
    Config(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            AppError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Translation(msg) => write!(f, "Translation error: {}", msg),
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Parsing(err.to_string())
    }
}
