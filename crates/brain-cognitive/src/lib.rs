//! Brain Cognitive Architecture
//! 
//! This crate contains the cognitive components of the Brain AI system:
//! - Conversation management and RAG orchestration
//! - Training data collection and quality assessment
//! - Independent intelligence orchestration
//! - Meta-memory systems with confidence tracking
//! - Curiosity-driven learning engines
//! - Conversational models and training pipelines
//! - Cognitive Preference Profiles (CPP) system

// Core conversation components
pub mod conversation;
pub mod training;
pub mod intelligence;
pub mod meta;
pub mod learning;
pub mod models;
pub mod meta_memory;

// New agent infrastructure
pub mod agents;
pub mod context;

// Agent orchestration system
pub mod orchestrator;

// Cognitive Preference Profiles system
pub mod profiles;

// Self-Evolution System
pub mod evolution;

// Re-export key conversation types
pub use conversation::{
    RagOrchestrator, RagRequest, RagResponse, ConversationContext,
    ChatMessage, RetrievedKnowledge, ResponseQuality,
    ConversationThread, UserProfile, TemporalContext,
    SafetyFlags, SourceAttribution,
    ConversationService, KnowledgeRetriever, ResponseGenerator,
};

// Re-export training types
pub use training::{
    TrainingDataCollector, TrainingDataConfig, ExportFormat,
    ConversationRecord, MessageRecord, ConversationMetadata,
    ComplexityLevel, ConversationType, UserExpertise,
    KnowledgeSourceRecord, UserFeedback, ConversationQualityMetrics,
    QualityAssessor, QualityModel, QualityModelType, QualityThresholds,
    PatternAnalyzer, PatternType, ConversationPattern,
    DataAnonymizer, AnonymizationRule, PiiType, PiiDetector, ReplacementStrategy,
    ConversationAnalytics, QualityTrend, DatasetFilter,
    TrainingDataset, DatasetMetadata, DatasetStatistics,
};

// Re-export intelligence types
pub use intelligence::{
    IntelligenceService,
    ConversationalModel,
    IndependentIntelligenceOrchestrator,
    IndependentIntelligenceConfig,
    ExternalFallbackConfig,
    PerformanceMonitoringConfig,
    TransitionConfig,
    ImprovementConfig,
    IndependencePerformanceMetrics,
    ModelPerformanceSnapshot,
    RoutingStatistics,
    RoutingDecision,
    ConversationRoute,
    QualityComparison,
    IndependentResponse,
    IndependenceStatus,
    IndependenceLevel,
    CognitiveKnowledge,
    CognitiveKnowledgeType,
    MemoryState,
    MemoryUtilizationMetrics,
    ConversationalInput,
};

// Re-export meta-memory types from meta module
pub use meta::{
    MetaMemoryService, MetaMemoryQueryBuilder,
    MetaMemoryError, MetaMemorySortField, MetaMemoryResult,
    // Traits
    MetaMemoryRepository, MetaMemoryAnalytics, MetaMemoryMaintenance,
    // Analysis types
    PerformanceMetrics, IntegrityReport, IntegrityIssue, IssueSeverity,
    MaintenanceReport,
};

// Re-export learning types
pub use learning::{
    CuriosityLearningEngine, CuriosityConfig, LearningPriority,
    CuriosityDrive, KnowledgeGap
};

// Re-export model types
pub use models::{
    BrainConversationalModel, ConversationalModelConfig,
    ModelArchitecture, KnowledgeIntegrationMode
};

pub use meta_memory::{
    MetaMemorySystem,
    MetaMemoryItem as SimpleMetaMemoryItem,
    MetaMemoryConfig as SimpleMetaMemoryConfig,
    MetaMemoryQuery as SimpleMetaMemoryQuery,
    MetaMemoryStats as SimpleMetaMemoryStats,
    KnowledgeType as SimpleKnowledgeType
};

// Re-export profiles types
pub use profiles::{
    // Core traits
    CognitiveProfileManager, BehaviorAdapter,
    // Data structures
    ProfileUpdates, ProfilePreset, ProfileAnalytics, BehaviorConfiguration,
    AdaptationContext, AdaptationRecommendation, AutonomyBoundaries,
    CommunicationAdaptations, CognitiveLoadManagement,
    // Manager implementations
    manager::{InMemoryProfileManager, FileBasedProfileManager},
    // Adapter implementations
    adapters::StandardBehaviorAdapter,
    // Preset utilities
    presets::{PresetManager, PresetUtils, ExperienceLevel, WorkContext, UserPreferences},
};

// Re-export orchestrator types
pub use orchestrator::{
    // Core orchestration
    AgentOrchestrator, OrchestrationConfig, OrchestrationMetrics,
    // DAG structures
    AgentDAG, AgentNode, ExecutionPlan, DependencyGraph,
    DAGBuilder, DAGValidationError, ExecutionOrder,
    // Executor components
    DAGExecutor, ExecutionEngine, ExecutionContext,
    ExecutionResult, RetryPolicy,
    // Scheduler components
    TaskScheduler, SchedulingStrategy, TaskPriority,
    ScheduleDecision, ResourceConstraints,
    // Memory management
    OrchestratorMemory, AgentMemoryNamespace, MemoryRegistry,
    CrossAgentMemoryShare, MemoryAccessControl, SharePermissions, MemoryOperation,
    // New orchestration memory types
    OrchestrationMemoryConfig, ExecutionMetadata, OrchestrationDecisionType,
    OrchestrationMemoryStats,
    // Communication system
    AgentCommunicationBus, MessageBus, AgentMessage,
    CommunicationProtocol, EventTrigger, MessageType, EventType, TriggerCondition,
    // New comprehensive communication types
    CommunicationConfig, CommunicationMetrics, StoredMessage, DeliveryStatus,
    // Workflow integration (Task 4.1.5)
    WorkflowAdapter, ConvertedWorkflow, EnhancedWorkflowResult,
    WorkflowExecutionStatus, WorkflowStepResult, StepExecutionStatus,
    WorkflowRequirements, AgentRequirement, WorkflowConstraints,
    WorkflowStepDefinition,
};

// Re-export evolution types
pub use evolution::{
    // Core evolution structures
    EvolutionOrchestrator, EvolutionConfig, EvolutionMemory,
    ImprovementRecord, LearningInsight,
    // Performance monitoring
    AgentPerformanceMonitor, AgentPerformanceMetrics,
    AgentPerformanceSnapshot, PerformanceTrends, PerformanceIssue,
    PerformanceBenchmarks, ExecutionBenchmarks, QualityBenchmarks,
    ResourceBenchmarks, UserBenchmarks,
    // Meta-agent framework
    MetaAgent, AgentAnalysis, ImprovementSuggestions, OptimizationResult,
    ValidationResult, PerformanceAnalysisMetaAgent,
    // Learning loop
    LearningLoopEngine, PatternRecognizer, ConfidenceCalibrator,
    FeedbackIntegrator, ParameterTuner, LearningState,
    LearningStrategy,
    // Phase 5.2: Learning Integration
    LearningIntegrationEngine, LearningIntegrationConfig,
    LearningCycleResult,
    AdaptationState, SystemPerformanceMetrics, LearningPhase,
    SophisticatedPatternAnalyzer, AutomatedParameterOptimizer,
    AdaptiveBehaviorModifier, IntegratedPerformanceTracker,
};
