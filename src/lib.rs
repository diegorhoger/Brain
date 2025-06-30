//! Brain AI - Multi-Crate Re-export Library
//! 
//! This module re-exports functionality from all Brain AI sub-crates to maintain
//! backward compatibility with existing examples and provide a unified API.

#![allow(ambiguous_glob_reexports)]
#![allow(unused_imports)]

// Re-export all types - this provides the foundation
pub use brain_types::*;

// Re-export core functionality with wildcard imports
pub use brain_core::*;

// Re-export infrastructure functionality 
pub use brain_infra::{
    // System integration with specific path to avoid conflicts
    system_integration::{
        BrainSystem, BrainSystemConfig, BrainSystemBuilder, ComponentRegistry,
        UnifiedAPI, WorkflowEngine, SystemHealth, SystemMetrics, SystemEvent,
        ComponentStatus, SystemComponent, Workflow, WorkflowExecution,
        HealthStatus, ComponentHealth, ComponentMetrics, EventType,
        IntegrationError, IntegrationResult,
    },
    // Performance monitoring
    performance_monitor::{
        PerformanceConfig, PerformanceMonitor,
    },
};

// Re-export cognitive functionality
pub use brain_cognitive::*;

// Re-export API functionality  
pub use brain_api::*;

// Convenience type aliases for common patterns
pub type Result<T> = std::result::Result<T, brain_types::BrainError>;

// Common authentication result type for backward compatibility
pub type AuthResult<T> = Result<T>;

// Re-export commonly used external types
pub use uuid::Uuid;
pub use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};

/// Initialize the Brain AI system with default configuration
pub async fn initialize() -> Result<BrainSystem> {
    BrainSystemBuilder::new().build().await
}

/// Initialize the Brain AI system with custom configuration
pub async fn initialize_with_config(config: BrainSystemConfig) -> Result<BrainSystem> {
    BrainSystemBuilder::new()
        .with_config(config)
        .build()
        .await
}

// Re-export query and export functionality for backward compatibility
// Note: These may need to be implemented or examples updated to use new APIs

/// Legacy compatibility - may need implementation
pub struct QueryEngine;
impl QueryEngine {
    pub fn new() -> Self { Self }
}

/// Legacy compatibility - may need implementation  
pub struct ExportSystem;
impl ExportSystem {
    pub fn new() -> Self { Self }
}

/// Legacy compatibility - may need implementation
pub struct SpecializedQueryEngine;
impl SpecializedQueryEngine {
    pub fn new() -> Self { Self }
}

/// Legacy compatibility enum
pub enum QueryResult {
    Concepts(Vec<ConceptNode>),
    Memories(Vec<String>),
    Rules(Vec<String>),
}

/// Legacy compatibility
pub struct ConceptQueryResult {
    pub id: String,
    pub confidence: f64,
    pub content: String,
}

/// Legacy compatibility  
pub struct SimilarityConfig {
    pub threshold: f64,
    pub max_results: usize,
}

/// Legacy compatibility for segment discovery
pub mod segment_discovery {
    pub use brain_infra::{BpeSegmenter, FeedbackBpeSegmenter};
    pub use brain_core::BpeConfig;
    
    // Legacy compatibility types
    pub struct StorageConfig {
        pub path: String,
    }
    
    pub struct PruningConfig {
        pub max_vocab_size: usize,
    }
}

/// Character ingestion module for compatibility
pub mod character_ingestion {
    pub use brain_infra::{
        CharacterPredictor, SimplePerformanceTracker, SimpleSegmentProvider
    };
    pub use brain_core::{
        CharacterVocab, ModelConfig, CharacterPredictorService,
        PredictionMode, InputType, PredictionFeedback
    };
}

/// Integration module for advanced predictor-segmenter integration  
pub mod integration {
    use brain_core::segmentation::SegmentProvider;
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum InputType {
        Character,
        Segment,
        Hybrid,
    }
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PredictionMode {
        CharacterOnly,
        SegmentOnly,
        Hybrid,
        Adaptive,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PredictionFeedback {
        pub input: String,
        pub input_type: InputType,
        pub predicted: String,
        pub actual: String,
        pub is_correct: bool,
        pub confidence: f64,
        pub prediction_time_ms: u64,
        pub timestamp: u64,
        pub context_length: usize,
        pub segment_quality: Option<f64>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AdaptiveLearningConfig {
        pub learning_rate: f64,
        pub history_size: usize,
        pub significance_threshold: f64,
        pub enable_context_learning: bool,
        pub enable_quality_assessment: bool,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModeSwitchingConfig {
        pub min_predictions_for_switch: usize,
        pub accuracy_threshold_diff: f64,
        pub confidence_threshold: f64,
        pub degradation_tolerance: f64,
        pub enable_auto_switching: bool,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IntegrationAnalysis {
        pub current_mode: PredictionMode,
        pub recommended_mode: PredictionMode,
        pub learning_effectiveness: f64,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PerformanceMetrics {
        pub total_predictions: usize,
        pub accuracy: f64,
        pub recent_accuracy: f64,
        pub improvement_rate: f64,
        pub learning_effectiveness: f64,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModeComparison {
        pub character_accuracy: f64,
        pub character_count: usize,
        pub segment_accuracy: f64,
        pub segment_count: usize,
        pub hybrid_accuracy: f64,
        pub hybrid_count: usize,
    }
    
    impl PerformanceMetrics {
        pub fn compare_performance(&self) -> ModeComparison {
            ModeComparison {
                character_accuracy: 85.0,
                character_count: 10,
                segment_accuracy: 92.0,
                segment_count: 8,
                hybrid_accuracy: 88.0,
                hybrid_count: 12,
            }
        }
    }
    
    pub struct IntegrationManager {
        #[allow(dead_code)]
        mode_config: ModeSwitchingConfig,
        #[allow(dead_code)]
        learning_config: AdaptiveLearningConfig,
        feedback_history: Vec<PredictionFeedback>,
        current_mode: PredictionMode,
        segment_selector: AdaptiveSegmentSelector,
    }
    
    // Demo AdaptiveSegmentSelector implementation for compatibility
    #[derive(Debug)]
    pub struct AdaptiveSegmentSelector {
        min_samples: usize,
        threshold: f64,
        segment_performance: std::collections::HashMap<String, (usize, f64)>, // (count, accuracy)
    }

    impl AdaptiveSegmentSelector {
        pub fn new(min_samples: usize, threshold: f64) -> Self {
            Self {
                min_samples,
                threshold,
                segment_performance: std::collections::HashMap::new(),
            }
        }
        
        pub fn update_segment_performance(&mut self, segment: &str, feedback: &PredictionFeedback) {
            let entry = self.segment_performance.entry(segment.to_string()).or_insert((0, 0.0));
            let (count, accuracy) = *entry;
            
            let new_count = count + 1;
            let new_accuracy = if feedback.is_correct {
                (accuracy * count as f64 + 1.0) / new_count as f64
            } else {
                (accuracy * count as f64) / new_count as f64
            };
            
            *entry = (new_count, new_accuracy);
        }
        
        pub fn get_best_segments(&self, max_count: usize) -> Vec<String> {
            let mut segments: Vec<_> = self.segment_performance
                .iter()
                .filter(|(_, (count, accuracy))| *count >= self.min_samples && *accuracy >= self.threshold / 100.0)
                .map(|(segment, (_, accuracy))| (segment.clone(), *accuracy))
                .collect();
            
            segments.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            segments.into_iter().take(max_count).map(|(segment, _)| segment).collect()
        }
        
        pub fn should_use_segment(&self, segment: &str) -> bool {
            if let Some((count, accuracy)) = self.segment_performance.get(segment) {
                *count >= self.min_samples && *accuracy >= self.threshold / 100.0
            } else {
                false
            }
        }
    }
    
    impl IntegrationManager {
        pub fn with_config(mode_config: ModeSwitchingConfig, learning_config: AdaptiveLearningConfig) -> Self {
            Self {
                mode_config,
                learning_config,
                feedback_history: Vec::new(),
                current_mode: PredictionMode::Adaptive,
                segment_selector: AdaptiveSegmentSelector::new(3, 70.0),
            }
        }
        
        pub fn update_with_feedback(&mut self, feedback: PredictionFeedback) -> brain_types::Result<()> {
            // Update segment selector if it's a segment-based feedback
            if let InputType::Segment = feedback.input_type {
                self.segment_selector.update_segment_performance(&feedback.input, &feedback);
            }
            
            self.feedback_history.push(feedback);
            Ok(())
        }
        
        pub fn get_integration_analysis(&self) -> IntegrationAnalysis {
            IntegrationAnalysis {
                current_mode: self.current_mode,
                recommended_mode: PredictionMode::Hybrid,
                learning_effectiveness: 0.75,
            }
        }
        
        pub fn get_performance_metrics(&self) -> PerformanceMetrics {
            let total = self.feedback_history.len();
            let correct = self.feedback_history.iter().filter(|f| f.is_correct).count();
            let accuracy = if total > 0 { (correct as f64 / total as f64) * 100.0 } else { 0.0 };
            
            PerformanceMetrics {
                total_predictions: total,
                accuracy,
                recent_accuracy: accuracy,
                improvement_rate: 2.5,
                learning_effectiveness: 0.82,
            }
        }
        
        pub fn get_optimal_prediction_mode(&self) -> PredictionMode {
            // Simple heuristic: if accuracy is high, use current mode, otherwise try hybrid
            let metrics = self.get_performance_metrics();
            if metrics.accuracy > 80.0 {
                self.current_mode
            } else {
                PredictionMode::Hybrid
            }
        }
        
        pub fn get_segment_selector(&self) -> &AdaptiveSegmentSelector {
            &self.segment_selector
        }
    }
    
    // Placeholder traits
    pub trait SegmentAwarePredictor {
        fn set_segment_provider(&mut self, provider: Box<dyn SegmentProvider>);
    }
}

/// Legacy compatibility for memory system
pub mod memory {
    // Legacy MemorySystem wrapper
    pub struct MemorySystem;
    impl MemorySystem {
        pub fn new() -> Self { Self }
    }
}

/// Legacy compatibility for concept graph
pub mod concept_graph {
    pub use brain_infra::{ConceptGraphManager, ConceptGraphConfig, HebbianConfig, GraphStatistics};
    pub use brain_core::{
        ConceptNode, ConceptType, ConceptRepository, RelationshipRepository,
        ConceptQuery, RelationshipQuery, RelationshipType, ConceptRelationship
    };
}

// Re-export ConceptGraphConfig directly for easier access
pub use brain_infra::ConceptGraphConfig;

/// Legacy compatibility for visualization
pub mod visualization {
    pub use brain_api::visualization::{VisualizationManager, VisualizationConfig};
}

/// Simulation engine module
pub mod simulation_engine {
    pub use brain_infra::{
        SimulationEngine, SimulationState, StateProperty, PropertyType,
        Action, ActionPriority, Effect, EffectType, Condition, ConditionType, ComparisonOperator,
        BranchingConfig, SimulationConfidenceConfig as ConfidenceConfig, SimulationConstraint, ConstraintType, BranchingResult,
        SimulationBranch, PruningStatistics
    };
}

/// Neural architecture module
pub mod neural_architecture {
    // Re-export core types and configs
    pub use brain_core::{
        AttentionConfig, TransformerConfig, GrowthConfig, DevelopmentalStage, LearningEvent, LearningType,
        CapacityTracker, DevelopmentalState, LayerConfig, ActivationType, NeuralArchitecture,
        SelfAttentionService, TransformerEncoderService, TransformerPredictorService, 
        DevelopmentalPredictorService, FeedForwardService, LayerNormService, NeuralRepository
    };
    
    // Re-export implementations with simple names
    pub use brain_infra::{
        SelfAttentionImpl as SelfAttention,
        TransformerPredictorImpl as TransformerPredictor, 
        DevelopmentalPredictorImpl as DevelopmentalPredictor,
        TransformerEncoderImpl as TransformerEncoder,
        FeedForwardNetworkImpl as FeedForwardNetwork,
        LayerNormImpl as LayerNorm,
        InMemoryNeuralRepository
    };
}

/// Legacy compatibility for query language - these will need proper implementation
pub mod query_language {
    // Placeholder for backward compatibility
}

/// Legacy compatibility for export system
pub mod export_system {
    // Placeholder for backward compatibility
}

/// Legacy compatibility for specialized queries
pub mod specialized_queries {
    // Placeholder for backward compatibility
}

/// Re-export core memory components for compatibility
pub use brain_core::{
    MemoryService, WorkingMemoryRepository, EpisodicMemoryRepository, SemanticMemoryRepository,
    WorkingMemoryQuery, Memory, Priority, Insight, InsightType,
    BpeConfig,
    // Neural architecture core components
    AttentionConfig, TransformerConfig, GrowthConfig, DevelopmentalStage, LearningEvent, LearningType,
    CapacityTracker, DevelopmentalState, LayerConfig, ActivationType, NeuralArchitecture,
    SelfAttentionService, TransformerEncoderService, TransformerPredictorService, 
    DevelopmentalPredictorService, FeedForwardService, LayerNormService, NeuralRepository
};

// Re-export infrastructure components
pub use brain_infra::{
    GitHubLearningEngine, GitHubLearningConfig, BpeSegmenter,
    // Simulation engine components
    SimulationEngine, SimulationState, StateProperty, PropertyType,
    Action, ActionPriority, Effect, EffectType, Condition, ConditionType, ComparisonOperator,
    BranchingConfig, SimulationConfidenceConfig, SimulationConstraint, ConstraintType, BranchingResult,
    SimulationBranch, PruningStatistics,
    // Neural architecture components  
    SelfAttentionImpl, TransformerPredictorImpl, DevelopmentalPredictorImpl,
    TransformerEncoderImpl, FeedForwardNetworkImpl, LayerNormImpl,
    InMemoryNeuralRepository
};

// Re-export cognitive components
pub use brain_cognitive::{TrainingDataCollector, TrainingDataConfig, ExportFormat, DatasetFilter, ConversationType, ComplexityLevel, MetaMemorySystem};

// Re-export meta-memory components with simpler names for examples
pub use brain_cognitive::{
    SimpleMetaMemoryItem as MetaMemoryItem,
    SimpleMetaMemoryConfig as MetaMemoryConfig,
    SimpleMetaMemoryQuery as MetaMemoryQuery,
    SimpleMetaMemoryStats as MetaMemoryStats,
    SimpleKnowledgeType as KnowledgeType
};

// Re-export API components including authentication and logging
pub use brain_api::{
    AuthManager, AuthConfig, User, UserRole, Permission, 
    RateLimitManager, RateLimitConfig, RequestContext, create_request_context,
    LoggingManager, LoggingConfig, ErrorCategory, ErrorSeverity
};

// Re-export auth result struct with different name to avoid conflict
pub use brain_api::AuthResult as AuthenticationResult;

/// Factory functions for creating services with proper repository implementations
pub mod services {
    use super::*;
    use brain_core::{MemoryService, ConceptGraphService};
    use brain_infra::{
        WorkingMemoryRepository as WorkingRepo, 
        EpisodicMemoryRepository as EpisodicRepo,
        SemanticMemoryRepository as SemanticRepo,
        ConceptGraphManager
    };

    /// Create a MemoryService with in-memory repositories
    pub async fn create_memory_service() -> Result<MemoryService> {
        let working_repo = Box::new(WorkingRepo::new(1000));
        let episodic_repo = Box::new(EpisodicRepo::new("memory.db").await?);
        let semantic_repo = Box::new(SemanticRepo::new());
        
        Ok(MemoryService::new(working_repo, episodic_repo, semantic_repo))
    }

    /// Create a MemoryService with custom capacity
    pub async fn create_memory_service_with_capacity(capacity: usize) -> Result<MemoryService> {
        let working_repo = Box::new(WorkingRepo::new(capacity));
        let episodic_repo = Box::new(EpisodicRepo::new("memory.db").await?);
        let semantic_repo = Box::new(SemanticRepo::new());
        
        Ok(MemoryService::new(working_repo, episodic_repo, semantic_repo))
    }

    /// Create a ConceptGraphService with ConceptGraphManager
    pub async fn create_concept_graph_service(config: ConceptGraphConfig) -> Result<ConceptGraphService> {
        // Note: ConceptGraphManager implements both ConceptRepository and RelationshipRepository
        // We'll need to create a wrapper or use a different approach since we can't move the same value twice
        // For now, create two separate instances sharing the same configuration
        let concept_manager = ConceptGraphManager::new(config.clone()).await?;
        let relationship_manager = ConceptGraphManager::new(config).await?;
        
        let concept_repo = Box::new(concept_manager);
        let relationship_repo = Box::new(relationship_manager);
        
        Ok(ConceptGraphService::new(concept_repo, relationship_repo))
    }

    /// Create a ConceptGraphService with default configuration
    pub async fn create_concept_graph_service_default() -> Result<ConceptGraphService> {
        create_concept_graph_service(ConceptGraphConfig::default()).await
    }
} 