//! Error types for the Brain architecture

use std::fmt;

/// Main error type for the Brain crate
#[derive(Debug)]
pub enum BrainError {
    /// IO related errors
    Io { source: std::io::Error },
    
    /// Serialization/deserialization errors
    Serialization { source: Box<dyn std::error::Error + Send + Sync> },
    
    /// Invalid input provided to a function
    InvalidInput(String),
    
    /// Mathematical computation errors
    MathError(String),
    
    /// Configuration errors
    ConfigError(String),
    
    /// Training related errors
    TrainingError(String),
    
    /// Prediction related errors
    PredictionError(String),
    
    /// Segmentation related errors
    SegmentationError(String),
    
    /// Generic error with custom message
    Other(String),
}

impl fmt::Display for BrainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrainError::Io { source } => write!(f, "IO error: {}", source),
            BrainError::Serialization { source } => write!(f, "Serialization error: {}", source),
            BrainError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BrainError::MathError(msg) => write!(f, "Math error: {}", msg),
            BrainError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            BrainError::TrainingError(msg) => write!(f, "Training error: {}", msg),
            BrainError::PredictionError(msg) => write!(f, "Prediction error: {}", msg),
            BrainError::SegmentationError(msg) => write!(f, "Segmentation error: {}", msg),
            BrainError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for BrainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BrainError::Io { source } => Some(source),
            BrainError::Serialization { source } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for BrainError {
    fn from(error: std::io::Error) -> Self {
        BrainError::Io { source: error }
    }
}

impl From<serde_json::Error> for BrainError {
    fn from(error: serde_json::Error) -> Self {
        BrainError::Serialization { source: Box::new(error) }
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

/// Result type for the Brain crate
pub type Result<T> = std::result::Result<T, BrainError>; 