//! Brain AI Self-Evolution System
//! 
//! This module implements the self-evolution capabilities for Brain AI agents:
//! - Meta-agent framework for agent improvement
//! - Performance monitoring and analysis
//! - Self-improvement suggestion system
//! - Agent behavior optimization
//! - Learning loop integration

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::agents::traits::{BrainAgent, CognitiveContext, BrainResult};
use crate::meta::MetaMemoryRepository;

// Sub-modules
pub mod meta_agent;
pub mod performance;
pub mod learning_loop;
pub mod optimization;
pub mod integration;

// Re-export key types
pub use meta_agent::*;
pub use performance::*;
pub use learning_loop::*;
pub use optimization::{OptimizationManager, OptimizationStrategy};
pub use meta_agent::OptimizationResult as MetaAgentOptimizationResult;
pub use integration::{
    LearningIntegrationEngine,
    LearningIntegrationConfig,
    AdaptationState,
    ActiveAdaptation,
    AdaptationCheckpoint,
    RollbackPlan,
    RollbackStep,
    SystemPerformanceMetrics,
    LearningPhase,
    SophisticatedPatternAnalyzer,
    AutomatedParameterOptimizer,
    AdaptiveBehaviorModifier,
    IntegratedPerformanceTracker,
    OptimizationResults,
    BehaviorModificationResults,
    OptimizationOpportunity,
    OpportunityType,
    ResourceEstimate,
    OptimizationResult as IntegrationOptimizationResult,
    ParameterChange,
    OptimizationExperiment,
    ExperimentStatus,
    SuccessRateTracker,
    OptimizationOutcome,
    BehaviorModificationOpportunity,
    BehaviorModificationType,
    BehaviorModificationResult,
    BehaviorChangeRecord,
    SystemPerformanceSnapshot,
    PerformanceTrend,
    TrendAnalysis,
    TrendDirection,
};

/// Core trait for meta-agents that can analyze and improve other agents
#[async_trait]
pub trait MetaAgent: BrainAgent {
    /// Analyze an agent's performance and behavior
    async fn analyze_agent(
        &self,
        target_agent_id: String,
        performance_data: AgentPerformanceData,
        context: &CognitiveContext,
    ) -> BrainResult<AgentAnalysis>;
    
    /// Generate improvement suggestions for an agent
    async fn suggest_improvements(
        &self,
        agent_analysis: AgentAnalysis,
        context: &CognitiveContext,
    ) -> BrainResult<ImprovementSuggestions>;
    
    /// Apply optimizations to an agent's behavior
    async fn optimize_agent_behavior(
        &self,
        target_agent_id: String,
        improvements: ImprovementSuggestions,
        context: &CognitiveContext,
    ) -> BrainResult<OptimizationResult>;
    
    /// Validate that improvements are working as expected
    async fn validate_improvements(
        &self,
        target_agent_id: String,
        before_metrics: AgentPerformanceMetrics,
        after_metrics: AgentPerformanceMetrics,
        context: &CognitiveContext,
    ) -> BrainResult<ValidationResult>;
}

/// Evolution orchestrator that manages the self-improvement process
pub struct EvolutionOrchestrator {
    /// Collection of meta-agents for different improvement aspects
    pub meta_agents: HashMap<String, Arc<dyn MetaAgent>>,
    
    /// Performance monitoring system
    pub performance_monitor: Arc<AgentPerformanceMonitor>,
    
    /// Learning loop integration
    pub learning_loop: Arc<LearningLoopEngine>,
    
    /// Configuration for evolution process
    pub config: EvolutionConfig,
    
    /// Memory for tracking evolution history
    pub evolution_memory: Arc<EvolutionMemory>,
}

/// Configuration for the evolution system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    /// How often to run performance analysis (in seconds)
    pub analysis_interval: u64,
    
    /// Minimum confidence threshold for applying improvements
    pub improvement_confidence_threshold: f32,
    
    /// Maximum number of concurrent optimizations
    pub max_concurrent_optimizations: u8,
    
    /// Enable/disable different types of evolution
    pub enable_performance_optimization: bool,
    pub enable_behavior_adaptation: bool,
    pub enable_capability_expansion: bool,
    
    /// Safety settings
    pub enable_rollback: bool,
    pub validation_period_hours: u32,
    
    /// Learning settings
    pub learning_rate: f32,
    pub adaptation_sensitivity: f32,
}

/// Memory system for tracking evolution history and decisions
#[derive(Debug)]
pub struct EvolutionMemory {
    /// Performance history for all agents
    pub performance_history: HashMap<String, Vec<AgentPerformanceSnapshot>>,
    
    /// Applied improvements and their outcomes
    pub improvement_history: HashMap<String, Vec<ImprovementRecord>>,
    
    /// Learning patterns and insights
    pub learning_insights: Vec<LearningInsight>,
    
    /// Configuration for memory management
    pub config: EvolutionMemoryConfig,
}

/// Configuration for evolution memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionMemoryConfig {
    /// Maximum number of performance snapshots to keep per agent
    pub max_performance_snapshots: usize,
    
    /// Maximum age of improvement records (in days)
    pub improvement_record_retention_days: u32,
    
    /// Memory cleanup interval (in hours)
    pub cleanup_interval_hours: u32,
    
    /// Enable memory persistence to disk
    pub enable_persistence: bool,
    
    /// Path for memory persistence
    pub persistence_path: Option<String>,
}

/// Snapshot of agent performance at a specific point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformanceSnapshot {
    /// Agent identifier
    pub agent_id: String,
    
    /// Timestamp of the snapshot
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Performance metrics
    pub metrics: AgentPerformanceMetrics,
    
    /// Context when snapshot was taken
    pub context_summary: String,
    
    /// Version of the agent at this snapshot
    pub agent_version: String,
}

/// Record of an improvement applied to an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRecord {
    /// Improvement identifier
    pub improvement_id: String,
    
    /// Target agent
    pub agent_id: String,
    
    /// When the improvement was applied
    pub applied_timestamp: chrono::DateTime<chrono::Utc>,
    
    /// The improvement that was applied
    pub improvement: ImprovementSuggestion,
    
    /// Performance before improvement
    pub before_metrics: AgentPerformanceMetrics,
    
    /// Performance after improvement (if available)
    pub after_metrics: Option<AgentPerformanceMetrics>,
    
    /// Success/failure status
    pub status: ImprovementStatus,
    
    /// Notes and observations
    pub notes: Vec<String>,
}

/// Status of an applied improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImprovementStatus {
    /// Currently being applied
    InProgress,
    
    /// Successfully applied and validated
    Success,
    
    /// Applied but validation failed
    ValidationFailed,
    
    /// Rolled back due to issues
    RolledBack,
    
    /// Failed to apply
    Failed,
    
    /// Pending validation
    PendingValidation,
}

/// Learning insight discovered through evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsight {
    /// Unique identifier for the insight
    pub insight_id: String,
    
    /// Category of insight
    pub category: InsightCategory,
    
    /// The insight description
    pub description: String,
    
    /// Confidence in this insight (0.0 to 1.0)
    pub confidence: f32,
    
    /// Supporting evidence
    pub evidence: Vec<String>,
    
    /// When this insight was discovered
    pub discovered_timestamp: chrono::DateTime<chrono::Utc>,
    
    /// How many times this insight has been validated
    pub validation_count: u32,
    
    /// Agents this insight applies to
    pub applicable_agents: Vec<String>,
}

/// Categories of learning insights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsightCategory {
    /// Performance optimization patterns
    PerformanceOptimization,
    
    /// Behavior adaptation patterns
    BehaviorAdaptation,
    
    /// User preference patterns
    UserPreferences,
    
    /// Context-specific optimizations
    ContextualOptimization,
    
    /// Error patterns and prevention
    ErrorPrevention,
    
    /// Resource utilization patterns
    ResourceOptimization,
    
    /// Collaboration patterns
    CollaborationOptimization,
}

/// Evolution orchestrator implementation
impl EvolutionOrchestrator {
    /// Create a new evolution orchestrator
    pub fn new(
        config: EvolutionConfig,
        meta_memory: Arc<dyn MetaMemoryRepository>,
    ) -> BrainResult<Self> {
        let performance_monitor = Arc::new(AgentPerformanceMonitor::new(
            config.clone(),
            meta_memory.clone(),
        )?);
        
        let learning_loop = Arc::new(LearningLoopEngine::new(
            config.clone(),
            meta_memory.clone(),
        )?);
        
        let evolution_memory = Arc::new(EvolutionMemory::new(
            EvolutionMemoryConfig::default(),
        )?);
        
        Ok(Self {
            meta_agents: HashMap::new(),
            performance_monitor,
            learning_loop,
            config,
            evolution_memory,
        })
    }
    
    /// Register a meta-agent for a specific improvement aspect
    pub fn register_meta_agent(
        &mut self,
        aspect: String,
        meta_agent: Arc<dyn MetaAgent>,
    ) -> BrainResult<()> {
        self.meta_agents.insert(aspect, meta_agent);
        Ok(())
    }
    
    /// Start the evolution process for all registered agents
    pub async fn start_evolution_process(&self, context: &CognitiveContext) -> BrainResult<()> {
        // Start performance monitoring
        self.performance_monitor.start_monitoring().await?;
        
        // Start learning loop
        self.learning_loop.start_learning().await?;
        
        // Initialize evolution cycle
        self.run_evolution_cycle(context).await?;
        
        Ok(())
    }
    
    /// Run a single evolution cycle
    async fn run_evolution_cycle(&self, context: &CognitiveContext) -> BrainResult<()> {
        // Get all agent performance data
        let performance_data = self.performance_monitor.get_all_agent_performance().await?;
        
        // Analyze each agent using meta-agents
        for (agent_id, agent_performance) in performance_data {
            for (_aspect, meta_agent) in &self.meta_agents {
                // Analyze the agent
                let analysis = meta_agent.analyze_agent(
                    agent_id.clone(),
                    agent_performance.clone(),
                    context,
                ).await?;
                
                // Generate improvement suggestions
                let suggestions = meta_agent.suggest_improvements(analysis, context).await?;
                
                // Apply improvements if confidence is high enough
                if suggestions.overall_confidence >= self.config.improvement_confidence_threshold {
                    let optimization_result = meta_agent.optimize_agent_behavior(
                        agent_id.clone(),
                        suggestions,
                        context,
                    ).await?;
                    
                    // Record the improvement
                    self.record_improvement(agent_id.clone(), optimization_result).await?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Record an improvement for tracking and analysis
    async fn record_improvement(
        &self,
        _agent_id: String,
        _optimization_result: OptimizationResult,
    ) -> BrainResult<()> {
        // Implementation for recording improvements
        // This would integrate with the evolution memory system
        Ok(())
    }
    
    /// Get evolution statistics and insights
    pub async fn get_evolution_stats(&self) -> BrainResult<EvolutionStatistics> {
        let total_improvements = self.evolution_memory.improvement_history.values()
            .map(|records| records.len())
            .sum::<usize>();
        
        let successful_improvements = self.evolution_memory.improvement_history.values()
            .flatten()
            .filter(|record| matches!(record.status, ImprovementStatus::Success))
            .count();
        
        let active_insights = self.evolution_memory.learning_insights.len();
        
        Ok(EvolutionStatistics {
            total_improvements,
            successful_improvements,
            active_insights,
            monitored_agents: self.performance_monitor.get_monitored_agent_count().await?,
            evolution_cycles_completed: 0, // TODO: Track this
            last_evolution_cycle: chrono::Utc::now(), // TODO: Track this
        })
    }
}

/// Statistics about the evolution system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStatistics {
    /// Total number of improvements attempted
    pub total_improvements: usize,
    
    /// Number of successful improvements
    pub successful_improvements: usize,
    
    /// Number of active learning insights
    pub active_insights: usize,
    
    /// Number of agents being monitored
    pub monitored_agents: usize,
    
    /// Number of evolution cycles completed
    pub evolution_cycles_completed: u64,
    
    /// When the last evolution cycle was completed
    pub last_evolution_cycle: chrono::DateTime<chrono::Utc>,
}

/// Implementation for evolution memory
impl EvolutionMemory {
    /// Create a new evolution memory system
    pub fn new(config: EvolutionMemoryConfig) -> BrainResult<Self> {
        Ok(Self {
            performance_history: HashMap::new(),
            improvement_history: HashMap::new(),
            learning_insights: Vec::new(),
            config,
        })
    }
    
    /// Add a performance snapshot for an agent
    pub async fn add_performance_snapshot(
        &mut self,
        snapshot: AgentPerformanceSnapshot,
    ) -> BrainResult<()> {
        let agent_snapshots = self.performance_history
            .entry(snapshot.agent_id.clone())
            .or_insert_with(Vec::new);
        
        agent_snapshots.push(snapshot);
        
        // Enforce maximum snapshots limit
        if agent_snapshots.len() > self.config.max_performance_snapshots {
            agent_snapshots.remove(0);
        }
        
        Ok(())
    }
    
    /// Add an improvement record
    pub async fn add_improvement_record(
        &mut self,
        record: ImprovementRecord,
    ) -> BrainResult<()> {
        let agent_improvements = self.improvement_history
            .entry(record.agent_id.clone())
            .or_insert_with(Vec::new);
        
        agent_improvements.push(record);
        
        Ok(())
    }
    
    /// Add a learning insight
    pub async fn add_learning_insight(
        &mut self,
        insight: LearningInsight,
    ) -> BrainResult<()> {
        self.learning_insights.push(insight);
        Ok(())
    }
    
    /// Get performance history for an agent
    pub fn get_agent_performance_history(
        &self,
        agent_id: &str,
    ) -> Option<&Vec<AgentPerformanceSnapshot>> {
        self.performance_history.get(agent_id)
    }
    
    /// Get improvement history for an agent
    pub fn get_agent_improvement_history(
        &self,
        agent_id: &str,
    ) -> Option<&Vec<ImprovementRecord>> {
        self.improvement_history.get(agent_id)
    }
    
    /// Get all learning insights for a category
    pub fn get_insights_by_category(
        &self,
        category: InsightCategory,
    ) -> Vec<&LearningInsight> {
        self.learning_insights.iter()
            .filter(|insight| insight.category == category)
            .collect()
    }
}

/// Default configuration implementations
impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            analysis_interval: 3600, // 1 hour
            improvement_confidence_threshold: 0.8,
            max_concurrent_optimizations: 3,
            enable_performance_optimization: true,
            enable_behavior_adaptation: true,
            enable_capability_expansion: false, // More experimental
            enable_rollback: true,
            validation_period_hours: 24,
            learning_rate: 0.1,
            adaptation_sensitivity: 0.7,
        }
    }
}

impl Default for EvolutionMemoryConfig {
    fn default() -> Self {
        Self {
            max_performance_snapshots: 100,
            improvement_record_retention_days: 90,
            cleanup_interval_hours: 24,
            enable_persistence: true,
            persistence_path: Some("evolution_memory.json".to_string()),
        }
    }
} 