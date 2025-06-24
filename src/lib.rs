//! Brain - A post-transformer developmental AI architecture
//! 
//! This crate implements the foundational components of the Brain architecture,
//! starting with character-level prediction and building up to more complex
//! cognitive capabilities.

// Task 10.1 - Core System Integration Module (NEW)
pub mod system_integration;

// Task 10.2 - Performance Monitoring and Optimization (NEW)
pub mod performance_monitor;

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

// Task 10.1 - Unified System Integration Exports (NEW)
pub use system_integration::{
    BrainSystem, BrainSystemConfig, BrainSystemBuilder, SystemComponent,
    ComponentStatus, SystemHealth, SystemMetrics, UnifiedAPI, WorkflowEngine,
    ComponentRegistry, SystemEvent, EventType, IntegrationResult
};

// Task 10.2 - Performance Monitoring and Optimization Exports (NEW)
pub use performance_monitor::{
    PerformanceMonitor, PerformanceConfig, SystemMetrics as PerfSystemMetrics,
    ComponentPerformanceMetrics, PerformanceSnapshot, PerformanceBottleneck,
    OptimizationRecommendation, PerformanceAlert, AlertType, AlertSeverity,
    BottleneckType, RecommendationType, PerformanceReport, ReportFormat
};

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

// Add this line after the existing module declarations
pub mod github_integration;

// Task 12 - Web Interface Module (NEW)
pub mod web_server;

// Add this line to the exports section
pub use github_integration::{
    GitHubClient, GitHubLearningEngine, GitHubLearningConfig, GitHubLearningResult,
    RepositoryInfo, RepositoryFile, FileType
};

// Task 12 exports - Web Interface
pub use web_server::{
    WebServer, ProcessRequest, QueryRequest, SimulationRequest, ProcessResponse,
    StatusResponse, StatsResponse, HealthResponse, start_web_server
};

// Task 13.1 - Conversational Intelligence Layer (NEW)
pub mod conversation;

// Task 13.4 - Training Data Collection (NEW)
pub mod training_data;

// Task 13.5 - Specialized Model Training (NEW)
pub mod conversational_model;
pub mod training_pipeline;

// Task 13.1 exports - Conversational Intelligence Layer
pub use conversation::{
    RagOrchestrator, RagRequest, RagResponse, ChatMessage, ConversationContext,
    RetrievedKnowledge, ResponseQuality, AnthropicRequest, AnthropicResponse,
    AnthropicMessage, AnthropicContent, AnthropicUsage
};

// Task 13.4 exports - Training Data Collection
pub use training_data::{
    TrainingDataCollector, TrainingDataConfig, ConversationRecord, MessageRecord,
    ConversationMetadata, ConversationQualityMetrics, QualityAssessor, DataAnonymizer,
    ConversationAnalytics, TrainingDataset, DatasetFilter, ExportFormat,
    ConversationType, ComplexityLevel
};

// Task 13.5 exports - Specialized Model Training
pub use conversational_model::{
    BrainConversationalModel, ConversationalModelConfig, ModelArchitecture, KnowledgeIntegrationMode,
    CognitiveIntegrationConfig, TrainingConfig, TrainingMetrics, EvaluationMetrics
};
pub use training_pipeline::{
    BrainTrainingPipeline, TrainingPipelineConfig, DataPreparationConfig, TrainingSchedule,
    TrainingPhase, EvaluationConfig, BenchmarkConfig, ExperimentConfig, CheckpointConfig,
    ExternalModelConfig, TrainingPipelineResult, PipelineTrainingState, EvaluationResult,
    ExperimentTracker, MetricEntry
};

/// Current version of the Brain architecture
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 