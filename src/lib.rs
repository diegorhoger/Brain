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
pub mod insight_extraction;
pub mod simulation_engine;

// Python API module for Task 7.1
#[cfg(feature = "python")]
pub mod python_api;

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
pub use insight_extraction::{
    PatternDetector, PatternDetectionConfig, DetectedPattern, PatternType,
    PatternDetectionResult, TemporalInfo, DetectionStats,
    // Rule Formalization Framework (Task 5.2)
    RuleFormalizationEngine, RuleFormalizationConfig, Rule, RulePattern, RuleOutcome,
    RuleMetrics, RuleDatabase, RuleQuery, RuleValidationResult, RuleComparison,
    ComparisonRecommendation, TemporalConstraints
};
pub use simulation_engine::{
    SimulationEngine, SimulationState, StateProperty, StateTransition,
    TextToStateParser, StateValidator, SimulationConfig
};

/// Current version of the Brain architecture
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 