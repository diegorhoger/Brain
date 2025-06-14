//! Brain - A post-transformer developmental AI architecture
//! 
//! This crate implements the foundational components of the Brain architecture,
//! starting with character-level prediction and building up to more complex
//! cognitive capabilities.

pub mod character_ingestion;
pub mod error;
pub mod integration;
pub mod segment_discovery;
pub mod neural_architecture;
pub mod utils;
pub mod memory;
pub mod concept_graph;

pub use character_ingestion::{
    CharacterPredictor, CharacterVocab, ModelConfig
};
pub use integration::{
    PredictionFeedback, InputType, PerformanceMetrics, PerformanceComparison,
    PredictionFeedbackTrait, SegmentAwarePredictor, SegmentProvider, PerformanceTracker,
    PredictionMode, AdaptiveSegmentSelector, IntegrationManager
};
pub use segment_discovery::{
    BpeSegmenter, BpeConfig, SegmentPair, SegmentStats, BpeStats,
    PruningConfig, StorageConfig, FeedbackBpeSegmenter
};
pub use error::{BrainError, Result};
pub use memory::*;
pub use concept_graph::{
    ConceptGraphManager, ConceptGraphConfig, ConceptNode, ConceptType,
    ConceptGraphStats, ConceptQuery
};

/// Current version of the Brain architecture
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 