//! Error types for the Brain architecture

use thiserror::Error;

/// Main error type for the Brain crate
#[derive(Error, Debug)]
pub enum BrainError {
    /// IO related errors
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    
    /// Serialization/deserialization errors
    #[error("Serialization error: {source}")]
    Serialization {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    /// Invalid input provided to a function
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// Mathematical computation errors
    #[error("Math error: {0}")]
    MathError(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Training related errors
    #[error("Training error: {0}")]
    TrainingError(String),
    
    /// Prediction related errors
    #[error("Prediction error: {0}")]
    PredictionError(String),
    
    /// Segmentation related errors
    #[error("Segmentation error: {0}")]
    SegmentationError(String),
    
    /// Parse related errors
    #[error("Parse error: {0}")]
    ParseError(String),
    
    /// Network related errors
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Item not found errors
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// Invalid query errors
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    
    /// Processing related errors
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    /// Database related errors
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    /// Memory system related errors
    #[error("Memory error: {0}")]
    MemoryError(String),
    
    /// Lock acquisition errors
    #[error("Lock error: {0}")]
    LockError(String),
    
    /// HTTP request errors
    #[error("HTTP error: {0}")]
    HttpError(String),
    
    /// Generic error with custom message
    #[error("Error: {0}")]
    Other(String),
}

impl From<serde_json::Error> for BrainError {
    fn from(error: serde_json::Error) -> Self {
        BrainError::Serialization { 
            source: Box::new(error) 
        }
    }
}

impl From<std::num::ParseIntError> for BrainError {
    fn from(error: std::num::ParseIntError) -> Self {
        BrainError::InvalidInput(format!("Failed to parse integer: {}", error))
    }
}

impl From<std::num::ParseFloatError> for BrainError {
    fn from(error: std::num::ParseFloatError) -> Self {
        BrainError::InvalidInput(format!("Failed to parse float: {}", error))
    }
}

impl From<anyhow::Error> for BrainError {
    fn from(error: anyhow::Error) -> Self {
        BrainError::Other(format!("Anyhow error: {}", error))
    }
}

/// Result type for the Brain crate
pub type Result<T> = std::result::Result<T, BrainError>;
