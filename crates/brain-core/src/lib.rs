//! Brain Core Domain Logic
//! 
//! This crate contains the core domain logic and abstractions for the Brain system.
//! It defines traits and data structures without any I/O dependencies.

pub mod concepts;
pub mod insights;
pub mod memory;
pub mod neural;
pub mod segmentation;
pub mod character_ingestion;
pub mod simulation;

// Re-export commonly used types - be specific to avoid conflicts
pub use concepts::*;
pub use insights::*;
pub use memory::*;
pub use neural::*;
pub use simulation::*;

// Segmentation exports
pub use segmentation::{SegmentationService, SegmentRepository, SegmentStats, BpeConfig, PruningConfig, BpeStats};

// Character ingestion exports  
pub use character_ingestion::{
    CharacterVocab, ModelConfig, PredictionMode, InputType, PerformanceMetrics,
    PredictionFeedback, PerformanceComparison, CharacterPredictorModel,
    CharacterPredictorService, CharacterIngestionRepository, PerformanceTracker,
    utils as character_utils
};

// Re-export the character ingestion SegmentProvider with a different name to avoid conflict
pub use character_ingestion::SegmentProvider as CharacterSegmentProvider;
// Re-export the segmentation SegmentProvider with a different name 
pub use segmentation::SegmentProvider as SegmentationProvider;
