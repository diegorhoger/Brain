//! Brain AI Infrastructure Layer
//! 
//! This crate provides concrete implementations of repository traits
//! and handles all I/O operations including database access, file system,
//! HTTP clients, and external API integrations.

pub mod memory;
pub mod concepts;
pub mod segmentation;
pub mod insights;
pub mod neural;
pub mod character_ingestion;
pub mod simulation;
pub mod database;
pub mod filesystem;
pub mod http;
pub mod config;
pub mod github_integration;
pub mod performance_monitor;
pub mod system_integration;

// Re-export key infrastructure components
pub use memory::{
    WorkingMemoryRepository, EpisodicMemoryRepository, SemanticMemoryRepository,
    average_embeddings
};
pub use concepts::{
    ConceptGraphManager, ConceptGraphConfig, HebbianConfig, 
    ConceptFormationConfig, ConceptFormationResult, SimilarityConfig,
    ConceptSubgraph, cosine_similarity
};
pub use segmentation::{InMemorySegmentRepository, BpeSegmenter, ContextMatrix, EntropyAnalyzer};
pub use insights::*;
pub use neural::*;
pub use character_ingestion::{
    FileCharacterIngestionRepository, CharacterPredictor, SimplePerformanceTracker, SimpleSegmentProvider
};
pub use simulation::{
    TextToStateParserImpl, StateValidatorImpl, SimulationEngineImpl, ActionConfig, ConfidenceConfig
};
pub use database::{DatabaseManager, DatabaseConfig as DbConfig};
pub use filesystem::*;
pub use http::*;
pub use config::{BrainConfig, DatabaseConfig as ConfigDbConfig};
pub use github_integration::{
    GitHubClient, GitHubLearningEngine, GitHubLearningConfig, GitHubLearningResult,
    RepositoryInfo, RepositoryFile, FileType, DetailedDataStructure, DetailedAPIEndpoint,
    DetailedArchitecturalPattern, DetailedDependency
};
pub use performance_monitor::{
    PerformanceMonitor, PerformanceConfig, AlertThresholds, SystemMetricsCollector,
    ComponentPerformanceTracker, PerformanceProfiler, AlertManager, PerformanceOptimizer,
    SystemMetrics as PerformanceSystemMetrics, ComponentPerformanceMetrics, OperationMetrics, ProfilerData,
    PerformanceAlert, AlertType, AlertSeverity, PerformanceSnapshot, PerformanceBottleneck,
    BottleneckType, BottleneckSeverity, OptimizationRecommendation, RecommendationType,
    RecommendationPriority, ImplementationEffort, OptimizationRule, PerformanceReport,
    ReportFormat
};
pub use system_integration::{
    BrainSystem, BrainSystemBuilder, BrainSystemConfig, ComponentRegistry, UnifiedAPI,
    WorkflowEngine, SystemHealth, SystemMetrics, ComponentMetrics, SystemEvent, EventType,
    ComponentStatus, HealthStatus, ComponentHealth, SystemComponent, IntegrationResult,
    IntegrationError, Workflow, WorkflowStep, WorkflowExecution, WorkflowStatus, StepResult,
    StepStatus, VERSION as BRAIN_SYSTEM_VERSION,
    CharacterPredictorComponent, BpeSegmenterComponent, MemorySystemComponent,
    ConceptGraphComponent, SimulationEngineComponent
};
