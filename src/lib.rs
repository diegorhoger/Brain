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

// Task 7.2 modules - Query Language and Export System
pub mod query_language;
pub mod export_system;
pub mod specialized_queries;

// Task 7.3 modules - Authentication, Logging, and Documentation
pub mod auth;
pub mod rate_limiting;
pub mod logging;

// Task 9.1 module - Meta-Memory System
pub mod meta_memory;

// Task 9.2 module - Novelty Detection
pub mod novelty_detection;

// Task 9.3 module - Curiosity-Driven Learning
pub mod curiosity_learning;

// Task 8 module - Visualization Components
pub mod visualization;

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

// Task 7.2 exports - Query Language and Export System
pub use query_language::{
    QueryEngine, QueryResult, ConceptQueryResult, MemoryQueryResult, 
    RuleQueryResult, QueryStats, OrderBy, OrderDirection, QueryTarget,
    QueryCondition, QueryOperator, QueryValue
};
pub use export_system::{
    ExportSystem, JsonGraphExport, GraphNode, GraphEdge, ExportMetadata, 
    ExportConfig, ExportStats, GraphStats, CsvExport
};
pub use specialized_queries::{
    SpecializedQueryEngine, RelatedConceptsResult, RuleChainResult, 
    ConceptRelationship, RuleConnection, ChainStatistics, QueryMetadata,
    TemporalQueryConfig, SimilarityConfig
};

// Task 7.3 exports - Authentication, Logging, and Documentation
pub use auth::{
    AuthManager, AuthConfig, AuthResult, UserRole, Permission, User, ApiKey, 
    TokenClaims
};
pub use rate_limiting::{
    RateLimitManager, RateLimitConfig, RateLimitResult, LimiterType, 
    RateLimitStats, RequestContext, create_request_context
};
pub use logging::{
    LoggingManager, LoggingConfig, ApiRequestLog, PerformanceMetrics as LoggingPerformanceMetrics, 
    ErrorLog, ErrorSeverity, ErrorCategory, UsageAnalytics
};

// Task 9.1 exports - Meta-Memory System
pub use meta_memory::{
    MetaMemorySystem, MetaMemoryItem, MetaMemoryConfig, MetaMemoryStats,
    MetaMemoryQuery, KnowledgeType
};

// Task 9.2 exports - Novelty Detection
pub use novelty_detection::{
    NoveltyDetectionEngine, NoveltyDetectionConfig, NoveltyAssessment, NoveltyContext,
    NoveltyMethod, NoveltyLevel, NoveltyDetectionStats
};

// Task 9.3 exports - Curiosity-Driven Learning
pub use curiosity_learning::{
    CuriosityLearningEngine, CuriosityConfig, LearningPriority, KnowledgeGap,
    InterestModel, LearningEvent, CuriosityStats, CuriosityDrive
};

// Task 8 exports - Visualization Components
pub use visualization::{
    VisualizationManager, VisualizationConfig, GraphData, VisualizationNode, VisualizationEdge,
    GraphMetadata, TimelineData, TimelineEvent, TimelineMetadata, GraphQueryParams
};

/// Current version of the Brain architecture
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 