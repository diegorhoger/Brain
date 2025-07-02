//! Learning Loop Integration
//! 
//! This module implements the learning loop for continuous agent improvement:
//! - Success/failure pattern recognition
//! - Agent confidence calibration
//! - User feedback integration
//! - Automated agent parameter tuning
//! - Adaptive learning strategies

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::agents::traits::BrainResult;
use crate::meta::MetaMemoryRepository;
use super::{
    EvolutionConfig, AgentPerformanceMetrics, 
    LearningInsight, InsightCategory,
};

/// Learning loop engine for continuous improvement
pub struct LearningLoopEngine {
    /// Configuration for learning
    pub config: EvolutionConfig,
    
    /// Pattern recognition system
    pub pattern_recognizer: Arc<PatternRecognizer>,
    
    /// Confidence calibration system
    pub confidence_calibrator: Arc<ConfidenceCalibrator>,
    
    /// Feedback integration system
    pub feedback_integrator: Arc<FeedbackIntegrator>,
    
    /// Parameter tuning system
    pub parameter_tuner: Arc<ParameterTuner>,
    
    /// Learning strategy manager
    pub strategy_manager: Arc<LearningStrategyManager>,
    
    /// Meta-memory integration
    pub meta_memory: Arc<dyn MetaMemoryRepository>,
    
    /// Current learning state
    pub learning_state: RwLock<LearningState>,
}

/// Current state of the learning system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningState {
    /// Active learning strategies
    pub active_strategies: Vec<LearningStrategy>,
    
    /// Current learning phase
    pub current_phase: LearningPhase,
    
    /// Learning progress metrics
    pub progress_metrics: LearningProgressMetrics,
    
    /// Recent learning insights
    pub recent_insights: Vec<LearningInsight>,
    
    /// Learning goals and targets
    pub learning_goals: Vec<LearningGoal>,
    
    /// Adaptation history
    pub adaptation_history: Vec<AdaptationRecord>,
}

/// Learning strategies available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningStrategy {
    /// Reactive learning from immediate feedback
    ReactiveLearning,
    
    /// Proactive learning from patterns
    ProactiveLearning,
    
    /// Collaborative learning from other agents
    CollaborativeLearning,
    
    /// Exploratory learning with experimentation
    ExploratoryLearning,
    
    /// Conservative learning with minimal risk
    ConservativeLearning,
    
    /// Aggressive learning with rapid adaptation
    AggressiveLearning,
    
    /// Selective learning focusing on specific areas
    SelectiveLearning,
    
    /// Continuous learning with ongoing adaptation
    ContinuousLearning,
}

/// Phases of the learning process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningPhase {
    /// Initial learning phase
    Initialization,
    
    /// Active learning and exploration
    Exploration,
    
    /// Exploitation of learned patterns
    Exploitation,
    
    /// Refinement and optimization
    Refinement,
    
    /// Maintenance and monitoring
    Maintenance,
    
    /// Adaptation to new contexts
    Adaptation,
}

/// Metrics tracking learning progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningProgressMetrics {
    /// Overall learning rate (0.0 to 1.0)
    pub learning_rate: f32,
    
    /// Knowledge acquisition rate
    pub knowledge_acquisition_rate: f32,
    
    /// Pattern recognition accuracy
    pub pattern_recognition_accuracy: f32,
    
    /// Confidence calibration accuracy
    pub confidence_calibration_accuracy: f32,
    
    /// Feedback integration effectiveness
    pub feedback_integration_effectiveness: f32,
    
    /// Parameter tuning success rate
    pub parameter_tuning_success_rate: f32,
    
    /// Adaptation speed (0.0 to 1.0)
    pub adaptation_speed: f32,
    
    /// Learning efficiency (0.0 to 1.0)
    pub learning_efficiency: f32,
    
    /// Knowledge retention rate
    pub knowledge_retention_rate: f32,
}

/// Learning goal definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningGoal {
    /// Goal identifier
    pub goal_id: String,
    
    /// Target metric to improve
    pub target_metric: String,
    
    /// Current value of the metric
    pub current_value: f32,
    
    /// Target value to achieve
    pub target_value: f32,
    
    /// Priority level
    pub priority: GoalPriority,
    
    /// Deadline for achievement
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Progress towards goal (0.0 to 1.0)
    pub progress: f32,
    
    /// Learning strategies assigned to this goal
    pub assigned_strategies: Vec<LearningStrategy>,
    
    /// Goal status
    pub status: GoalStatus,
}

/// Priority levels for learning goals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalPriority {
    /// Critical goal requiring immediate attention
    Critical,
    
    /// High priority goal
    High,
    
    /// Medium priority goal
    Medium,
    
    /// Low priority goal
    Low,
    
    /// Optional goal
    Optional,
}

/// Status of learning goals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    /// Goal is active and being pursued
    Active,
    
    /// Goal has been achieved
    Achieved,
    
    /// Goal has been paused
    Paused,
    
    /// Goal has been cancelled
    Cancelled,
    
    /// Goal is blocked by dependencies
    Blocked,
}

/// Record of adaptation made to an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationRecord {
    /// Adaptation identifier
    pub adaptation_id: String,
    
    /// Target agent that was adapted
    pub target_agent_id: String,
    
    /// Timestamp of adaptation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Type of adaptation
    pub adaptation_type: AdaptationType,
    
    /// Adaptation details
    pub adaptation_details: String,
    
    /// Reason for adaptation
    pub reason: String,
    
    /// Learning strategy that triggered adaptation
    pub triggering_strategy: LearningStrategy,
    
    /// Performance before adaptation
    pub before_performance: Option<AgentPerformanceMetrics>,
    
    /// Performance after adaptation
    pub after_performance: Option<AgentPerformanceMetrics>,
    
    /// Success status of adaptation
    pub success: bool,
    
    /// Lessons learned
    pub lessons_learned: Vec<String>,
}

/// Types of adaptations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptationType {
    /// Parameter adjustment
    ParameterAdjustment,
    
    /// Behavior modification
    BehaviorModification,
    
    /// Strategy change
    StrategyChange,
    
    /// Configuration update
    ConfigurationUpdate,
    
    /// Learning rate adjustment
    LearningRateAdjustment,
    
    /// Threshold modification
    ThresholdModification,
    
    /// Feature addition
    FeatureAddition,
    
    /// Feature removal
    FeatureRemoval,
}

/// Pattern recognition system
pub struct PatternRecognizer {
    /// Configuration for pattern recognition
    pub config: PatternRecognitionConfig,
    
    /// Detected patterns
    pub detected_patterns: RwLock<HashMap<String, DetectedPattern>>,
    
    /// Pattern templates
    pub pattern_templates: Vec<PatternTemplate>,
    
    /// Pattern matching algorithms
    pub matchers: Vec<Box<dyn PatternMatcher>>,
}

/// Configuration for pattern recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRecognitionConfig {
    /// Minimum pattern confidence threshold
    pub min_pattern_confidence: f32,
    
    /// Maximum number of patterns to track
    pub max_tracked_patterns: usize,
    
    /// Pattern decay rate (how quickly old patterns are forgotten)
    pub pattern_decay_rate: f32,
    
    /// Minimum occurrences to establish a pattern
    pub min_pattern_occurrences: u32,
    
    /// Pattern update frequency (in seconds)
    pub update_frequency: u64,
}

/// Detected pattern in agent behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    /// Pattern identifier
    pub pattern_id: String,
    
    /// Type of pattern
    pub pattern_type: PatternType,
    
    /// Pattern description
    pub description: String,
    
    /// Confidence in pattern (0.0 to 1.0)
    pub confidence: f32,
    
    /// Number of times pattern was observed
    pub occurrence_count: u32,
    
    /// Context conditions where pattern occurs
    pub context_conditions: Vec<String>,
    
    /// Associated agents
    pub associated_agents: Vec<String>,
    
    /// Pattern strength (0.0 to 1.0)
    pub strength: f32,
    
    /// When pattern was first detected
    pub first_detected: chrono::DateTime<chrono::Utc>,
    
    /// When pattern was last observed
    pub last_observed: chrono::DateTime<chrono::Utc>,
    
    /// Predicted outcomes when pattern occurs
    pub predicted_outcomes: Vec<PredictedOutcome>,
}

/// Types of patterns that can be detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Success patterns leading to good outcomes
    SuccessPattern,
    
    /// Failure patterns leading to poor outcomes
    FailurePattern,
    
    /// Performance patterns affecting execution
    PerformancePattern,
    
    /// User interaction patterns
    UserInteractionPattern,
    
    /// Context-dependent patterns
    ContextualPattern,
    
    /// Temporal patterns occurring at specific times
    TemporalPattern,
    
    /// Collaborative patterns between agents
    CollaborativePattern,
    
    /// Learning patterns in adaptation
    LearningPattern,
    
    /// Optimization patterns from parameter tuning
    OptimizationPattern,
}

/// Predicted outcome from a pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedOutcome {
    /// Outcome description
    pub description: String,
    
    /// Probability of outcome (0.0 to 1.0)
    pub probability: f32,
    
    /// Expected impact (positive or negative)
    pub expected_impact: f32,
    
    /// Confidence in prediction (0.0 to 1.0)
    pub confidence: f32,
    
    /// Timeframe for outcome
    pub timeframe: OutcomeTimeframe,
}

/// Timeframes for predicted outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutcomeTimeframe {
    /// Immediate outcome
    Immediate,
    
    /// Short term (minutes to hours)
    ShortTerm,
    
    /// Medium term (hours to days)
    MediumTerm,
    
    /// Long term (days to weeks)
    LongTerm,
}

/// Template for pattern recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTemplate {
    /// Template identifier
    pub template_id: String,
    
    /// Template name
    pub name: String,
    
    /// Pattern conditions to match
    pub conditions: Vec<PatternCondition>,
    
    /// Expected pattern indicators
    pub indicators: Vec<PatternIndicator>,
    
    /// Template priority
    pub priority: u8,
    
    /// Minimum confidence required for match
    pub min_confidence: f32,
}

/// Condition for pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCondition {
    /// Metric name to check
    pub metric_name: String,
    
    /// Comparison operator
    pub operator: ComparisonOperator,
    
    /// Value to compare against
    pub value: f32,
    
    /// Weight of this condition
    pub weight: f32,
}

/// Comparison operators for pattern conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Equal to
    Equal,
    
    /// Not equal to
    NotEqual,
    
    /// Greater than
    GreaterThan,
    
    /// Less than
    LessThan,
    
    /// Greater than or equal
    GreaterThanOrEqual,
    
    /// Less than or equal
    LessThanOrEqual,
    
    /// Within range
    WithinRange,
    
    /// Outside range
    OutsideRange,
}

/// Indicator of pattern presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternIndicator {
    /// Indicator name
    pub name: String,
    
    /// Description of what this indicates
    pub description: String,
    
    /// Strength of indication (0.0 to 1.0)
    pub strength: f32,
    
    /// Required for pattern detection
    pub required: bool,
}

/// Trait for pattern matching algorithms
pub trait PatternMatcher: Send + Sync {
    /// Match patterns in the given data
    fn match_patterns(
        &self,
        data: &[AgentPerformanceMetrics],
        templates: &[PatternTemplate],
    ) -> BrainResult<Vec<DetectedPattern>>;
    
    /// Get matcher name
    fn name(&self) -> &str;
    
    /// Get matcher confidence level
    fn confidence_level(&self) -> f32;
}

/// Confidence calibration system
#[derive(Debug)]
pub struct ConfidenceCalibrator {
    /// Configuration for confidence calibration
    pub config: ConfidenceCalibrationConfig,
    
    /// Historical confidence vs actual performance data
    pub calibration_data: RwLock<HashMap<String, Vec<ConfidenceDataPoint>>>,
    
    /// Calibration models for different agents
    pub calibration_models: RwLock<HashMap<String, CalibrationModel>>,
}

/// Configuration for confidence calibration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceCalibrationConfig {
    /// Minimum data points required for calibration
    pub min_calibration_points: u32,
    
    /// Calibration update frequency
    pub update_frequency: u64,
    
    /// Confidence adjustment sensitivity
    pub adjustment_sensitivity: f32,
    
    /// Maximum confidence adjustment per update
    pub max_adjustment: f32,
    
    /// Learning rate for calibration
    pub learning_rate: f32,
}

/// Data point for confidence calibration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceDataPoint {
    /// Predicted confidence level
    pub predicted_confidence: f32,
    
    /// Actual performance outcome
    pub actual_performance: f32,
    
    /// Context information
    pub context: String,
    
    /// Timestamp of prediction
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Agent that made the prediction
    pub agent_id: String,
}

/// Calibration model for confidence adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationModel {
    /// Model identifier
    pub model_id: String,
    
    /// Target agent
    pub agent_id: String,
    
    /// Calibration curve parameters
    pub curve_parameters: Vec<f32>,
    
    /// Model accuracy
    pub accuracy: f32,
    
    /// Number of training points
    pub training_points: u32,
    
    /// Last update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
    
    /// Model version
    pub version: String,
}

/// Feedback integration system
pub struct FeedbackIntegrator {
    /// Configuration for feedback integration
    pub config: FeedbackIntegrationConfig,
    
    /// Feedback queue
    pub feedback_queue: RwLock<Vec<UserFeedback>>,
    
    /// Processed feedback history
    pub feedback_history: RwLock<Vec<ProcessedFeedback>>,
    
    /// Feedback processors
    pub processors: Vec<Box<dyn FeedbackProcessor>>,
}

/// Configuration for feedback integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackIntegrationConfig {
    /// Feedback processing frequency
    pub processing_frequency: u64,
    
    /// Minimum feedback confidence threshold
    pub min_feedback_confidence: f32,
    
    /// Feedback weight in learning
    pub feedback_weight: f32,
    
    /// Maximum feedback age (in hours)
    pub max_feedback_age_hours: u32,
    
    /// Feedback aggregation strategy
    pub aggregation_strategy: FeedbackAggregationStrategy,
}

/// Strategies for aggregating feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackAggregationStrategy {
    /// Simple average
    Average,
    
    /// Weighted average by recency
    WeightedByRecency,
    
    /// Weighted average by confidence
    WeightedByConfidence,
    
    /// Median value
    Median,
    
    /// Most recent feedback
    MostRecent,
}

/// User feedback data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    /// Feedback identifier
    pub feedback_id: String,
    
    /// Target agent
    pub agent_id: String,
    
    /// User identifier
    pub user_id: String,
    
    /// Feedback type
    pub feedback_type: FeedbackType,
    
    /// Feedback rating (0.0 to 1.0)
    pub rating: f32,
    
    /// Textual feedback
    pub comment: Option<String>,
    
    /// Specific aspects rated
    pub aspect_ratings: HashMap<String, f32>,
    
    /// Context when feedback was given
    pub context: String,
    
    /// Feedback timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Confidence in feedback (0.0 to 1.0)
    pub confidence: f32,
}

/// Types of user feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackType {
    /// Overall satisfaction rating
    Satisfaction,
    
    /// Quality assessment
    Quality,
    
    /// Accuracy evaluation
    Accuracy,
    
    /// Usefulness rating
    Usefulness,
    
    /// Speed/efficiency feedback
    Efficiency,
    
    /// Clarity and understandability
    Clarity,
    
    /// Completeness assessment
    Completeness,
    
    /// Error reporting
    ErrorReport,
}

/// Processed and analyzed feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedFeedback {
    /// Original feedback identifier
    pub original_feedback_id: String,
    
    /// Processing timestamp
    pub processed_timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Extracted insights
    pub insights: Vec<FeedbackInsight>,
    
    /// Recommended actions
    pub recommended_actions: Vec<FeedbackAction>,
    
    /// Processing confidence
    pub processing_confidence: f32,
    
    /// Feedback impact assessment
    pub impact_assessment: ImpactAssessment,
}

/// Insight extracted from feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackInsight {
    /// Insight description
    pub description: String,
    
    /// Insight category
    pub category: InsightCategory,
    
    /// Insight confidence
    pub confidence: f32,
    
    /// Supporting evidence
    pub evidence: Vec<String>,
    
    /// Actionability score (0.0 to 1.0)
    pub actionability: f32,
}

/// Action recommended based on feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackAction {
    /// Action description
    pub description: String,
    
    /// Action type
    pub action_type: ActionType,
    
    /// Priority level
    pub priority: ActionPriority,
    
    /// Expected impact
    pub expected_impact: f32,
    
    /// Implementation complexity
    pub complexity: f32,
    
    /// Estimated implementation time
    pub estimated_time_hours: f32,
}

/// Types of feedback actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Adjust agent parameters
    ParameterAdjustment,
    
    /// Modify agent behavior
    BehaviorModification,
    
    /// Update training data
    TrainingDataUpdate,
    
    /// Improve documentation
    DocumentationImprovement,
    
    /// Enhance user interface
    UIEnhancement,
    
    /// Fix identified issues
    IssueFix,
    
    /// Add new features
    FeatureAddition,
}

/// Priority levels for actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionPriority {
    /// Critical action
    Critical,
    
    /// High priority
    High,
    
    /// Medium priority
    Medium,
    
    /// Low priority
    Low,
    
    /// Optional enhancement
    Optional,
}

/// Impact assessment of feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// Overall impact score (0.0 to 1.0)
    pub overall_impact: f32,
    
    /// Impact on specific metrics
    pub metric_impacts: HashMap<String, f32>,
    
    /// Affected user segments
    pub affected_user_segments: Vec<String>,
    
    /// Potential reach of changes
    pub potential_reach: f32,
    
    /// Risk assessment
    pub risk_level: f32,
}

/// Trait for feedback processors
pub trait FeedbackProcessor: Send + Sync {
    /// Process feedback and extract insights
    fn process_feedback(&self, feedback: &UserFeedback) -> BrainResult<ProcessedFeedback>;
    
    /// Get processor name
    fn name(&self) -> &str;
    
    /// Get supported feedback types
    fn supported_types(&self) -> Vec<FeedbackType>;
}

/// Parameter tuning system
pub struct ParameterTuner {
    /// Configuration for parameter tuning
    pub config: ParameterTuningConfig,
    
    /// Active tuning experiments
    pub active_experiments: RwLock<HashMap<String, TuningExperiment>>,
    
    /// Tuning history
    pub tuning_history: RwLock<Vec<TuningResult>>,
    
    /// Tuning strategies
    pub strategies: Vec<Box<dyn TuningStrategy>>,
}

/// Configuration for parameter tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterTuningConfig {
    /// Maximum concurrent experiments
    pub max_concurrent_experiments: u8,
    
    /// Experiment duration (in hours)
    pub experiment_duration_hours: u32,
    
    /// Minimum improvement threshold
    pub min_improvement_threshold: f32,
    
    /// Safety margin for parameter changes
    pub safety_margin: f32,
    
    /// Tuning frequency
    pub tuning_frequency: u64,
}

/// Tuning experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningExperiment {
    /// Experiment identifier
    pub experiment_id: String,
    
    /// Target agent
    pub target_agent_id: String,
    
    /// Parameters being tuned
    pub parameters: HashMap<String, ParameterRange>,
    
    /// Experiment status
    pub status: ExperimentStatus,
    
    /// Start timestamp
    pub start_timestamp: chrono::DateTime<chrono::Utc>,
    
    /// End timestamp
    pub end_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Baseline performance
    pub baseline_performance: AgentPerformanceMetrics,
    
    /// Current best performance
    pub best_performance: Option<AgentPerformanceMetrics>,
    
    /// Best parameter values
    pub best_parameters: Option<HashMap<String, f32>>,
    
    /// Experiment results
    pub results: Vec<ExperimentResult>,
}

/// Range for parameter tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterRange {
    /// Minimum value
    pub min_value: f32,
    
    /// Maximum value
    pub max_value: f32,
    
    /// Current value
    pub current_value: f32,
    
    /// Step size for adjustments
    pub step_size: f32,
    
    /// Parameter type
    pub parameter_type: ParameterType,
}

/// Types of parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    /// Continuous numeric parameter
    Continuous,
    
    /// Discrete numeric parameter
    Discrete,
    
    /// Boolean parameter
    Boolean,
    
    /// Categorical parameter
    Categorical,
}

/// Status of tuning experiments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentStatus {
    /// Experiment is running
    Running,
    
    /// Experiment completed successfully
    Completed,
    
    /// Experiment failed
    Failed,
    
    /// Experiment was cancelled
    Cancelled,
    
    /// Experiment is paused
    Paused,
}

/// Result from a tuning experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    /// Parameter values tested
    pub parameter_values: HashMap<String, f32>,
    
    /// Performance metrics achieved
    pub performance_metrics: AgentPerformanceMetrics,
    
    /// Improvement over baseline
    pub improvement: f32,
    
    /// Statistical significance
    pub significance: f32,
    
    /// Experiment iteration
    pub iteration: u32,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Overall tuning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningResult {
    /// Experiment that produced this result
    pub experiment_id: String,
    
    /// Target agent
    pub agent_id: String,
    
    /// Tuning success status
    pub success: bool,
    
    /// Performance improvement achieved
    pub improvement: f32,
    
    /// Final parameter values
    pub final_parameters: HashMap<String, f32>,
    
    /// Tuning strategy used
    pub strategy_used: String,
    
    /// Lessons learned
    pub lessons_learned: Vec<String>,
    
    /// Completion timestamp
    pub completed_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Trait for tuning strategies
pub trait TuningStrategy: Send + Sync {
    /// Suggest next parameter values to try
    fn suggest_parameters(
        &self,
        experiment: &TuningExperiment,
        history: &[ExperimentResult],
    ) -> BrainResult<HashMap<String, f32>>;
    
    /// Evaluate if experiment should continue
    fn should_continue(&self, experiment: &TuningExperiment) -> bool;
    
    /// Get strategy name
    fn name(&self) -> &str;
    
    /// Get strategy description
    fn description(&self) -> &str;
}

/// Learning strategy manager
pub struct LearningStrategyManager {
    /// Available learning strategies
    pub available_strategies: Vec<LearningStrategy>,
    
    /// Strategy effectiveness history
    pub strategy_effectiveness: RwLock<HashMap<LearningStrategy, StrategyMetrics>>,
    
    /// Current strategy assignments
    pub strategy_assignments: RwLock<HashMap<String, Vec<LearningStrategy>>>,
    
    /// Strategy selection algorithm
    pub selection_algorithm: Box<dyn StrategySelector>,
}

/// Metrics for strategy effectiveness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyMetrics {
    /// Number of times strategy was used
    pub usage_count: u32,
    
    /// Success rate (0.0 to 1.0)
    pub success_rate: f32,
    
    /// Average improvement achieved
    pub average_improvement: f32,
    
    /// Average time to achieve improvement
    pub average_time_to_improvement: f32,
    
    /// Strategy confidence (0.0 to 1.0)
    pub confidence: f32,
    
    /// Last used timestamp
    pub last_used: chrono::DateTime<chrono::Utc>,
}

/// Trait for strategy selection algorithms
pub trait StrategySelector: Send + Sync {
    /// Select best strategies for a given context
    fn select_strategies(
        &self,
        context: &LearningContext,
        available_strategies: &[LearningStrategy],
        strategy_metrics: &HashMap<LearningStrategy, StrategyMetrics>,
    ) -> BrainResult<Vec<LearningStrategy>>;
    
    /// Get selector name
    fn name(&self) -> &str;
}

/// Context for learning strategy selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningContext {
    /// Target agent identifier
    pub agent_id: String,
    
    /// Current performance level
    pub current_performance: f32,
    
    /// Learning goals
    pub goals: Vec<LearningGoal>,
    
    /// Available time for learning
    pub time_budget: Option<u32>,
    
    /// Risk tolerance
    pub risk_tolerance: f32,
    
    /// Context characteristics
    pub characteristics: HashMap<String, String>,
}

/// Implementation for LearningLoopEngine
impl LearningLoopEngine {
    /// Create a new learning loop engine
    pub fn new(
        config: EvolutionConfig,
        meta_memory: Arc<dyn MetaMemoryRepository>,
    ) -> BrainResult<Self> {
        Ok(Self {
            config: config.clone(),
            pattern_recognizer: Arc::new(PatternRecognizer::new(PatternRecognitionConfig::default())?),
            confidence_calibrator: Arc::new(ConfidenceCalibrator::new(ConfidenceCalibrationConfig::default())?),
            feedback_integrator: Arc::new(FeedbackIntegrator::new(FeedbackIntegrationConfig::default())?),
            parameter_tuner: Arc::new(ParameterTuner::new(ParameterTuningConfig::default())?),
            strategy_manager: Arc::new(LearningStrategyManager::new()?),
            meta_memory,
            learning_state: RwLock::new(LearningState::default()),
        })
    }
    
    /// Start the learning loop
    pub async fn start_learning(&self) -> BrainResult<()> {
        let mut state = self.learning_state.write().await;
        state.current_phase = LearningPhase::Initialization;
        
        // Initialize learning strategies
        state.active_strategies = vec![
            LearningStrategy::ReactiveLearning,
            LearningStrategy::ProactiveLearning,
            LearningStrategy::ContinuousLearning,
        ];
        
        Ok(())
    }
    
    /// Process a learning cycle
    pub async fn process_learning_cycle(
        &self,
        agent_id: String,
        performance_data: &[AgentPerformanceMetrics],
    ) -> BrainResult<LearningCycleResult> {
        let cycle_start = chrono::Utc::now();
        
        // Step 1: Pattern Recognition - Detect success/failure patterns
        let patterns_detected = self.pattern_recognizer
            .recognize_patterns(performance_data)
            .await?;
            
        // Step 2: Confidence Calibration - Adjust confidence based on actual performance
        let confidence_adjustments = self.confidence_calibrator
            .calibrate_agent_confidence(&agent_id, performance_data)
            .await?;
            
        // Step 3: Feedback Integration - Process user feedback for insights
        let feedback_insights = self.feedback_integrator
            .process_agent_feedback(&agent_id)
            .await?;
            
        // Step 4: Parameter Tuning - Automatically adjust agent parameters
        let parameter_adjustments = self.parameter_tuner
            .check_and_tune_parameters(&agent_id, performance_data)
            .await?;
            
        // Step 5: Generate Learning Insights from all collected data
        let learning_insights = self.generate_learning_insights(
            &patterns_detected,
            &confidence_adjustments,
            &feedback_insights,
        ).await?;
        
        // Step 6: Calculate overall improvement achieved
        let overall_improvement = self.calculate_overall_improvement(
            &confidence_adjustments,
            &feedback_insights,
            &parameter_adjustments,
        ).await?;
        
        // Step 7: Update learning state with new insights and progress
        self.update_learning_state(
            agent_id.clone(),
            &learning_insights,
            overall_improvement,
        ).await?;
        
        // Step 8: Store insights in meta-memory for future learning
        self.store_learning_insights(&agent_id, &learning_insights).await?;
        
        Ok(LearningCycleResult {
            agent_id,
            cycle_timestamp: cycle_start,
            patterns_detected,
            confidence_adjustments,
            feedback_insights,
            parameter_adjustments,
            learning_insights,
            overall_improvement,
        })
    }
    
    /// Calculate overall improvement from all learning components
    async fn calculate_overall_improvement(
        &self,
        confidence_result: &ConfidenceCalibrationResult,
        feedback_result: &FeedbackProcessingResult,
        parameter_result: &ParameterTuningResult,
    ) -> BrainResult<f32> {
        let mut total_improvement = 0.0;
        let mut improvement_count = 0;
        
        // Factor in confidence calibration improvements
        if confidence_result.new_accuracy > 0.0 {
            total_improvement += confidence_result.new_accuracy;
            improvement_count += 1;
        }
        
        // Factor in feedback sentiment improvements
        if feedback_result.overall_sentiment > 0.0 {
            total_improvement += feedback_result.overall_sentiment;
            improvement_count += 1;
        }
        
        // Factor in parameter tuning improvements
        if parameter_result.tuning_performed && parameter_result.expected_improvement > 0.0 {
            total_improvement += parameter_result.expected_improvement;
            improvement_count += 1;
        }
        
        // Calculate weighted average improvement
        if improvement_count > 0 {
            Ok(total_improvement / improvement_count as f32)
        } else {
            Ok(0.0)
        }
    }
    
    /// Update learning state with new insights and progress
    async fn update_learning_state(
        &self,
        agent_id: String,
        insights: &[LearningInsight],
        improvement: f32,
    ) -> BrainResult<()> {
        let mut state = self.learning_state.write().await;
        
        // Add new insights to recent insights (keep last 50)
        state.recent_insights.extend_from_slice(insights);
        let insights_len = state.recent_insights.len();
        if insights_len > 50 {
            state.recent_insights.drain(0..insights_len - 50);
        }
        
        // Update progress metrics
        state.progress_metrics.learning_efficiency += improvement * 0.1; // Gradual improvement
        state.progress_metrics.learning_efficiency = state.progress_metrics.learning_efficiency.min(1.0);
        
        state.progress_metrics.knowledge_acquisition_rate += improvement * 0.05;
        state.progress_metrics.knowledge_acquisition_rate = state.progress_metrics.knowledge_acquisition_rate.min(1.0);
        
        // Update learning phase if appropriate
        if improvement > 0.3 {
            state.current_phase = LearningPhase::Exploitation;
        } else if improvement > 0.1 {
            state.current_phase = LearningPhase::Refinement;
        } else {
            state.current_phase = LearningPhase::Exploration;
        }
        
        // Record adaptation
        let adaptation_record = AdaptationRecord {
            adaptation_id: format!("adapt_{}_{}", agent_id, chrono::Utc::now().timestamp()),
            target_agent_id: agent_id,
            timestamp: chrono::Utc::now(),
            adaptation_type: AdaptationType::LearningRateAdjustment,
            adaptation_details: format!("Applied learning insights with {:.2}% improvement", improvement * 100.0),
            reason: "Learning cycle optimization".to_string(),
            triggering_strategy: LearningStrategy::ContinuousLearning,
            before_performance: None,
            after_performance: None,
            success: improvement > 0.05,
            lessons_learned: insights.iter().map(|i| i.description.clone()).collect(),
        };
        
        state.adaptation_history.push(adaptation_record);
        if state.adaptation_history.len() > 100 {
            state.adaptation_history.drain(0..1);
        }
        
        Ok(())
    }
    
    /// Store learning insights in meta-memory for persistence
    async fn store_learning_insights(
        &self,
        agent_id: &str,
        insights: &[LearningInsight],
    ) -> BrainResult<()> {
        for insight in insights {
            let source = format!("learning_insight_{}_{}", agent_id, insight.insight_id);
            
            let _memory_item = crate::meta::MetaMemoryItem::new(
                uuid::Uuid::new_v4(), // component_id
                crate::meta::KnowledgeType::Pattern, // Use Pattern as closest knowledge type
                (insight.confidence as f64).clamp(0.0, 1.0), // initial_confidence
                source.clone(), // source
            );
            
            // Note: store_item requires &mut self, so we can't call it directly here
            // This would need to be redesigned to work with the MetaMemoryRepository trait
            // For now, we'll just log the insight storage attempt
            log::info!("Would store learning insight: {} for agent {}", insight.insight_id, agent_id);
        }
        
        Ok(())
    }

    /// Generate learning insights from various sources
    async fn generate_learning_insights(
        &self,
        patterns: &[DetectedPattern],
        confidence_result: &ConfidenceCalibrationResult,
        feedback_result: &FeedbackProcessingResult,
    ) -> BrainResult<Vec<LearningInsight>> {
        let mut insights = Vec::new();
        
        // Generate insights from patterns
        for pattern in patterns {
            let insight = LearningInsight {
                insight_id: format!("pattern_insight_{}", pattern.pattern_id),
                category: match pattern.pattern_type {
                    PatternType::SuccessPattern => InsightCategory::PerformanceOptimization,
                    PatternType::FailurePattern => InsightCategory::ErrorPrevention,
                    PatternType::PerformancePattern => InsightCategory::PerformanceOptimization,
                    PatternType::UserInteractionPattern => InsightCategory::UserPreferences,
                    PatternType::ContextualPattern => InsightCategory::ContextualOptimization,
                    PatternType::TemporalPattern => InsightCategory::PerformanceOptimization,
                    PatternType::CollaborativePattern => InsightCategory::CollaborationOptimization,
                    PatternType::LearningPattern => InsightCategory::BehaviorAdaptation,
                    PatternType::OptimizationPattern => InsightCategory::PerformanceOptimization,
                },
                description: format!(
                    "Detected {} pattern: {} (confidence: {:.2}, strength: {:.2})",
                    match pattern.pattern_type {
                        PatternType::SuccessPattern => "success",
                        PatternType::FailurePattern => "failure",
                        PatternType::PerformancePattern => "performance",
                        PatternType::UserInteractionPattern => "user interaction",
                        PatternType::ContextualPattern => "contextual",
                        PatternType::TemporalPattern => "temporal",
                        PatternType::CollaborativePattern => "collaborative",
                        PatternType::LearningPattern => "learning",
                        PatternType::OptimizationPattern => "optimization",
                    },
                    pattern.description,
                    pattern.confidence,
                    pattern.strength
                ),
                confidence: pattern.confidence,
                discovered_timestamp: chrono::Utc::now(),
                validation_count: pattern.occurrence_count,
                applicable_agents: pattern.associated_agents.clone(),
                evidence: pattern.context_conditions.clone(),
            };
            insights.push(insight);
        }
        
        // Generate insights from confidence calibration
        if confidence_result.new_accuracy > 0.7 {
            let insight = LearningInsight {
                insight_id: format!("confidence_insight_{}", chrono::Utc::now().timestamp()),
                category: InsightCategory::BehaviorAdaptation,
                description: format!(
                    "Confidence calibration improved to {:.2}% accuracy with {} adjustments",
                    confidence_result.new_accuracy * 100.0,
                    confidence_result.adjustments.len()
                ),
                confidence: confidence_result.confidence,
                discovered_timestamp: chrono::Utc::now(),
                validation_count: 1,
                applicable_agents: vec![confidence_result.agent_id.clone()],
                evidence: confidence_result.adjustments.iter()
                    .map(|adj| format!("{}: {}", adj.context, adj.reason))
                    .collect(),
            };
            insights.push(insight);
        }
        
        // Generate insights from feedback
        if feedback_result.processed_count > 0 {
            let sentiment_description = if feedback_result.overall_sentiment > 0.7 {
                "positive"
            } else if feedback_result.overall_sentiment > 0.3 {
                "neutral"
            } else {
                "negative"
            };
            
            let insight = LearningInsight {
                insight_id: format!("feedback_insight_{}", chrono::Utc::now().timestamp()),
                category: InsightCategory::UserPreferences,
                description: format!(
                    "Processed {} feedback items with {} overall sentiment. Key insights: {}",
                    feedback_result.processed_count,
                    sentiment_description,
                    feedback_result.key_insights.len()
                ),
                confidence: if feedback_result.processed_count > 5 { 0.8 } else { 0.6 },
                discovered_timestamp: chrono::Utc::now(),
                validation_count: feedback_result.processed_count,
                applicable_agents: vec!["all".to_string()], // Feedback insights apply broadly
                evidence: feedback_result.key_insights.iter()
                    .map(|insight| insight.description.clone())
                    .collect(),
            };
            insights.push(insight);
        }
        
        Ok(insights)
    }
}

/// Result of a learning cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningCycleResult {
    /// Agent that was processed
    pub agent_id: String,
    
    /// Cycle timestamp
    pub cycle_timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Patterns detected in this cycle
    pub patterns_detected: Vec<DetectedPattern>,
    
    /// Confidence calibration adjustments
    pub confidence_adjustments: ConfidenceCalibrationResult,
    
    /// Feedback processing insights
    pub feedback_insights: FeedbackProcessingResult,
    
    /// Parameter tuning adjustments
    pub parameter_adjustments: ParameterTuningResult,
    
    /// Learning insights generated
    pub learning_insights: Vec<LearningInsight>,
    
    /// Overall improvement achieved
    pub overall_improvement: f32,
}

/// Result of confidence calibration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceCalibrationResult {
    /// Agent that was calibrated
    pub agent_id: String,
    
    /// Calibration adjustments made
    pub adjustments: Vec<ConfidenceAdjustment>,
    
    /// New calibration accuracy
    pub new_accuracy: f32,
    
    /// Calibration confidence
    pub confidence: f32,
}

/// Individual confidence adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceAdjustment {
    /// Context where adjustment applies
    pub context: String,
    
    /// Adjustment factor
    pub adjustment_factor: f32,
    
    /// Reason for adjustment
    pub reason: String,
}

/// Result of feedback processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackProcessingResult {
    /// Number of feedback items processed
    pub processed_count: u32,
    
    /// Key insights extracted
    pub key_insights: Vec<FeedbackInsight>,
    
    /// Recommended actions
    pub recommended_actions: Vec<FeedbackAction>,
    
    /// Overall sentiment
    pub overall_sentiment: f32,
}

/// Result of parameter tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterTuningResult {
    /// Whether tuning was performed
    pub tuning_performed: bool,
    
    /// Parameters that were adjusted
    pub adjusted_parameters: HashMap<String, f32>,
    
    /// Expected improvement
    pub expected_improvement: f32,
    
    /// Tuning confidence
    pub confidence: f32,
}

/// Default implementations
impl Default for LearningState {
    fn default() -> Self {
        Self {
            active_strategies: vec![LearningStrategy::ContinuousLearning],
            current_phase: LearningPhase::Initialization,
            progress_metrics: LearningProgressMetrics::default(),
            recent_insights: Vec::new(),
            learning_goals: Vec::new(),
            adaptation_history: Vec::new(),
        }
    }
}

impl Default for LearningProgressMetrics {
    fn default() -> Self {
        Self {
            learning_rate: 0.5,
            knowledge_acquisition_rate: 0.5,
            pattern_recognition_accuracy: 0.7,
            confidence_calibration_accuracy: 0.7,
            feedback_integration_effectiveness: 0.6,
            parameter_tuning_success_rate: 0.6,
            adaptation_speed: 0.5,
            learning_efficiency: 0.6,
            knowledge_retention_rate: 0.8,
        }
    }
}

impl Default for PatternRecognitionConfig {
    fn default() -> Self {
        Self {
            min_pattern_confidence: 0.7,
            max_tracked_patterns: 1000,
            pattern_decay_rate: 0.01,
            min_pattern_occurrences: 3,
            update_frequency: 3600, // 1 hour
        }
    }
}

impl Default for ConfidenceCalibrationConfig {
    fn default() -> Self {
        Self {
            min_calibration_points: 10,
            update_frequency: 3600, // 1 hour
            adjustment_sensitivity: 0.1,
            max_adjustment: 0.2,
            learning_rate: 0.01,
        }
    }
}

impl Default for FeedbackIntegrationConfig {
    fn default() -> Self {
        Self {
            processing_frequency: 1800, // 30 minutes
            min_feedback_confidence: 0.6,
            feedback_weight: 0.3,
            max_feedback_age_hours: 168, // 1 week
            aggregation_strategy: FeedbackAggregationStrategy::WeightedByRecency,
        }
    }
}

impl Default for ParameterTuningConfig {
    fn default() -> Self {
        Self {
            max_concurrent_experiments: 3,
            experiment_duration_hours: 24,
            min_improvement_threshold: 0.05,
            safety_margin: 0.1,
            tuning_frequency: 86400, // 1 day
        }
    }
}

// Implementation stubs for the various components
impl PatternRecognizer {
    pub fn new(config: PatternRecognitionConfig) -> BrainResult<Self> {
        Ok(Self {
            config,
            detected_patterns: RwLock::new(HashMap::new()),
            pattern_templates: Self::create_default_templates(),
            matchers: Vec::new(),
        })
    }
    
    /// Create default pattern templates for common success/failure patterns
    fn create_default_templates() -> Vec<PatternTemplate> {
        vec![
            // Success pattern: High performance with low errors
            PatternTemplate {
                template_id: "success_high_performance".to_string(),
                name: "High Performance Success".to_string(),
                conditions: vec![
                    PatternCondition {
                        metric_name: "success_rate".to_string(),
                        operator: ComparisonOperator::GreaterThan,
                        value: 0.8,
                        weight: 1.0,
                    },
                    PatternCondition {
                        metric_name: "error_rate".to_string(),
                        operator: ComparisonOperator::LessThan,
                        value: 0.1,
                        weight: 0.8,
                    },
                ],
                indicators: vec![
                    PatternIndicator {
                        name: "high_success_rate".to_string(),
                        description: "Consistent high success rate".to_string(),
                        strength: 0.9,
                        required: true,
                    },
                ],
                priority: 1,
                min_confidence: 0.7,
            },
            // Failure pattern: Low performance with high errors
            PatternTemplate {
                template_id: "failure_low_performance".to_string(),
                name: "Low Performance Failure".to_string(),
                conditions: vec![
                    PatternCondition {
                        metric_name: "success_rate".to_string(),
                        operator: ComparisonOperator::LessThan,
                        value: 0.5,
                        weight: 1.0,
                    },
                    PatternCondition {
                        metric_name: "error_rate".to_string(),
                        operator: ComparisonOperator::GreaterThan,
                        value: 0.3,
                        weight: 0.9,
                    },
                ],
                indicators: vec![
                    PatternIndicator {
                        name: "high_error_rate".to_string(),
                        description: "Consistently high error rate".to_string(),
                        strength: 0.8,
                        required: true,
                    },
                ],
                priority: 1,
                min_confidence: 0.6,
            },
        ]
    }
    
    pub async fn recognize_patterns(&self, data: &[AgentPerformanceMetrics]) -> BrainResult<Vec<DetectedPattern>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut detected_patterns = Vec::new();
        
        // Analyze success/failure patterns
        let success_pattern = self.detect_success_failure_patterns(data).await?;
        if let Some(pattern) = success_pattern {
            detected_patterns.push(pattern);
        }
        
        // Analyze performance trends
        let performance_patterns = self.detect_performance_patterns(data).await?;
        detected_patterns.extend(performance_patterns);
        
        // Analyze temporal patterns
        let temporal_patterns = self.detect_temporal_patterns(data).await?;
        detected_patterns.extend(temporal_patterns);
        
        // Update detected patterns cache
        {
            let mut patterns_cache = self.detected_patterns.write().await;
            for pattern in &detected_patterns {
                patterns_cache.insert(pattern.pattern_id.clone(), pattern.clone());
            }
        }
        
        Ok(detected_patterns)
    }
    
    /// Detect success/failure patterns in performance data
    async fn detect_success_failure_patterns(&self, data: &[AgentPerformanceMetrics]) -> BrainResult<Option<DetectedPattern>> {
        if data.len() < 3 {
            return Ok(None);
        }
        
        // Calculate success and error rates
        let total_executions: u64 = data.iter().map(|m| m.execution_metrics.total_executions).sum();
        let total_errors: f32 = data.iter().map(|m| m.execution_metrics.error_rate * m.execution_metrics.total_executions as f32).sum();
        
        if total_executions == 0 {
            return Ok(None);
        }
        
        let success_rate = 1.0 - (total_errors / total_executions as f32);
        let avg_error_rate = total_errors / total_executions as f32;
        
        // Determine pattern type based on performance thresholds
        let (pattern_type, confidence, description) = if success_rate > 0.8 && avg_error_rate < 0.1 {
            (PatternType::SuccessPattern, 0.85, "High success rate with low error rate")
        } else if success_rate < 0.5 || avg_error_rate > 0.3 {
            (PatternType::FailurePattern, 0.8, "Poor performance with high error rate")
        } else {
            return Ok(None); // No clear pattern
        };
        
        // Store pattern in cache
        {
            let mut patterns_cache = self.detected_patterns.write().await;
            let pattern_id = format!("pattern_{}", chrono::Utc::now().timestamp());
            
            let pattern = DetectedPattern {
                pattern_id: pattern_id.clone(),
                pattern_type: pattern_type.clone(),
                description: description.to_string(),
                confidence,
                occurrence_count: 1,
                context_conditions: vec![
                    format!("success_rate: {:.2}", success_rate),
                    format!("error_rate: {:.2}", avg_error_rate),
                ],
                associated_agents: data.iter()
                    .map(|m| m.agent_id.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect(),
                strength: confidence,
                first_detected: chrono::Utc::now(),
                last_observed: chrono::Utc::now(),
                predicted_outcomes: vec![PredictedOutcome {
                    description: if matches!(pattern_type, PatternType::SuccessPattern) {
                        "Continued high performance expected".to_string()
                    } else {
                        "Performance improvement needed".to_string()
                    },
                    probability: confidence,
                    expected_impact: if matches!(pattern_type, PatternType::SuccessPattern) { 0.7 } else { -0.6 },
                    confidence,
                    timeframe: OutcomeTimeframe::ShortTerm,
                }],
            };
            
            patterns_cache.insert(pattern_id, pattern.clone());
            Ok(Some(pattern))
        }
    }
    
    /// Detect performance trend patterns
    async fn detect_performance_patterns(&self, data: &[AgentPerformanceMetrics]) -> BrainResult<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();
        
        if data.len() < 5 {
            return Ok(patterns);
        }
        
        // Extract response times for trend analysis
        let response_times: Vec<f32> = data.iter()
            .map(|m| m.execution_metrics.avg_execution_time_ms as f32)
            .collect();
        
        if let Some(trend) = self.analyze_trend(&response_times) {
            let pattern = DetectedPattern {
                pattern_id: format!("perf_trend_{}", chrono::Utc::now().timestamp()),
                pattern_type: PatternType::PerformancePattern,
                description: trend.description,
                confidence: trend.confidence,
                occurrence_count: 1,
                context_conditions: vec![format!("trend: {}", trend.direction)],
                associated_agents: data.iter()
                    .map(|m| m.agent_id.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect(),
                strength: trend.confidence,
                first_detected: chrono::Utc::now(),
                last_observed: chrono::Utc::now(),
                predicted_outcomes: vec![PredictedOutcome {
                    description: trend.prediction,
                    probability: trend.confidence,
                    expected_impact: trend.expected_impact,
                    confidence: trend.confidence,
                    timeframe: OutcomeTimeframe::MediumTerm,
                }],
            };
            patterns.push(pattern);
        }
        
        Ok(patterns)
    }
    
    /// Detect temporal patterns (time-based behaviors)
    async fn detect_temporal_patterns(&self, _data: &[AgentPerformanceMetrics]) -> BrainResult<Vec<DetectedPattern>> {
        // Placeholder for temporal pattern detection
        // This would analyze patterns based on time of day, day of week, etc.
        Ok(Vec::new())
    }
    
    /// Analyze trend in a series of values
    fn analyze_trend(&self, values: &[f32]) -> Option<TrendAnalysis> {
        if values.len() < 3 {
            return None;
        }
        
        // Simple linear regression to detect trend
        let n = values.len() as f32;
        let x_sum: f32 = (0..values.len()).map(|i| i as f32).sum();
        let y_sum: f32 = values.iter().sum();
        let xy_sum: f32 = values.iter().enumerate().map(|(i, &y)| i as f32 * y).sum();
        let x2_sum: f32 = (0..values.len()).map(|i| (i as f32).powi(2)).sum();
        
        let slope = (n * xy_sum - x_sum * y_sum) / (n * x2_sum - x_sum.powi(2));
        let intercept = (y_sum - slope * x_sum) / n;
        
        // Calculate confidence based on correlation coefficient
        let y_mean = y_sum / n;
        let ss_tot: f32 = values.iter().map(|&y| (y - y_mean).powi(2)).sum();
        let ss_res: f32 = values.iter().enumerate()
            .map(|(i, &y)| (y - (slope * i as f32 + intercept)).powi(2))
            .sum();
        
        let r_squared = 1.0 - (ss_res / ss_tot);
        let confidence = r_squared.sqrt().min(1.0).max(0.0);
        
        let direction = if slope > 0.0 { "improving" } else { "degrading" };
        
        Some(TrendAnalysis {
            direction: direction.to_string(),
            confidence,
            description: format!("Performance trend shows {} pattern", direction),
            prediction: format!("Trend expected to continue in {} direction", direction),
            expected_impact: if slope > 0.0 { 0.6 } else { -0.4 },
        })
    }
}

/// Helper struct for trend analysis
struct TrendAnalysis {
    direction: String,
    confidence: f32,
    description: String,
    prediction: String,
    expected_impact: f32,
}

impl ConfidenceCalibrator {
    pub fn new(config: ConfidenceCalibrationConfig) -> BrainResult<Self> {
        Ok(Self {
            config,
            calibration_data: RwLock::new(HashMap::new()),
            calibration_models: RwLock::new(HashMap::new()),
        })
    }
    
    pub async fn calibrate_agent_confidence(&self, agent_id: &str, data: &[AgentPerformanceMetrics]) -> BrainResult<ConfidenceCalibrationResult> {
        if data.is_empty() {
            return Ok(ConfidenceCalibrationResult {
                agent_id: agent_id.to_string(),
                adjustments: Vec::new(),
                new_accuracy: 0.5,
                confidence: 0.3,
            });
        }
        
        let mut adjustments = Vec::new();
        
        // Analyze confidence vs performance correlation
        let mut confidence_accuracy_pairs = Vec::new();
        for metrics in data.iter().take(20) { // Analyze last 20 data points
            let predicted_confidence = metrics.quality_metrics.accuracy;
            let actual_accuracy = metrics.quality_metrics.accuracy;
            confidence_accuracy_pairs.push((predicted_confidence, actual_accuracy));
        }
        
        if confidence_accuracy_pairs.len() < 3 {
            return Ok(ConfidenceCalibrationResult {
                agent_id: agent_id.to_string(),
                adjustments: Vec::new(),
                new_accuracy: 0.5,
                confidence: 0.3,
            });
        }
        
        // Calculate calibration metrics
        let calibration_analysis = self.analyze_calibration(&confidence_accuracy_pairs);
        
        // Generate adjustments based on calibration issues
        if calibration_analysis.overconfidence > 0.1 {
            adjustments.push(ConfidenceAdjustment {
                context: "overconfidence_correction".to_string(),
                adjustment_factor: -calibration_analysis.overconfidence * 0.5,
                reason: format!("Agent shows overconfidence: predicted {:.2} vs actual {:.2}", 
                    calibration_analysis.avg_predicted, calibration_analysis.avg_actual),
            });
        }
        
        if calibration_analysis.underconfidence > 0.1 {
            adjustments.push(ConfidenceAdjustment {
                context: "underconfidence_correction".to_string(),
                adjustment_factor: calibration_analysis.underconfidence * 0.3,
                reason: format!("Agent shows underconfidence: predicted {:.2} vs actual {:.2}", 
                    calibration_analysis.avg_predicted, calibration_analysis.avg_actual),
            });
        }
        
        // Context-specific adjustments
        let high_accuracy_contexts = data.iter()
            .filter(|m| m.quality_metrics.accuracy > 0.8)
            .count();
        let low_accuracy_contexts = data.iter()
            .filter(|m| m.quality_metrics.accuracy < 0.4)
            .count();
            
        if high_accuracy_contexts > low_accuracy_contexts {
            adjustments.push(ConfidenceAdjustment {
                context: "high_performance_context".to_string(),
                adjustment_factor: 0.1,
                reason: "Agent performs well in current context".to_string(),
            });
        }
        
        // Calculate new accuracy and confidence
        let new_accuracy = if adjustments.is_empty() {
            calibration_analysis.correlation.abs()
        } else {
            (calibration_analysis.correlation.abs() + 0.1).min(1.0)
        };
        
        let confidence = if adjustments.len() > 1 { 0.8 } else if adjustments.len() == 1 { 0.6 } else { 0.4 };
        
        // Store calibration data for future analysis
        {
            let mut calibration_data = self.calibration_data.write().await;
            let agent_data = calibration_data.entry(agent_id.to_string()).or_insert_with(Vec::new);
            
            for (predicted, actual) in confidence_accuracy_pairs {
                agent_data.push(ConfidenceDataPoint {
                    predicted_confidence: predicted,
                    actual_performance: actual,
                    context: "performance_calibration".to_string(),
                    timestamp: chrono::Utc::now(),
                    agent_id: agent_id.to_string(),
                });
            }
            
            // Keep only last 100 data points
            if agent_data.len() > 100 {
                agent_data.drain(0..agent_data.len() - 100);
            }
        }
        
        Ok(ConfidenceCalibrationResult {
            agent_id: agent_id.to_string(),
            adjustments,
            new_accuracy,
            confidence,
        })
    }
    
    /// Analyze calibration quality
    fn analyze_calibration(&self, pairs: &[(f32, f32)]) -> CalibrationAnalysis {
        if pairs.is_empty() {
            return CalibrationAnalysis::default();
        }
        
        let n = pairs.len() as f32;
        let sum_predicted: f32 = pairs.iter().map(|(p, _)| *p).sum();
        let sum_actual: f32 = pairs.iter().map(|(_, a)| *a).sum();
        let avg_predicted = sum_predicted / n;
        let avg_actual = sum_actual / n;
        
        // Calculate correlation
        let sum_xy: f32 = pairs.iter().map(|(p, a)| p * a).sum();
        let sum_x2: f32 = pairs.iter().map(|(p, _)| p * p).sum();
        let sum_y2: f32 = pairs.iter().map(|(_, a)| a * a).sum();
        
        let correlation = if n > 1.0 {
            let numerator = n * sum_xy - sum_predicted * sum_actual;
            let denominator = ((n * sum_x2 - sum_predicted.powi(2)) * (n * sum_y2 - sum_actual.powi(2))).sqrt();
            if denominator > 0.0 { numerator / denominator } else { 0.0 }
        } else {
            0.0
        };
        
        let overconfidence = if avg_predicted > avg_actual { avg_predicted - avg_actual } else { 0.0 };
        let underconfidence = if avg_actual > avg_predicted { avg_actual - avg_predicted } else { 0.0 };
        
        CalibrationAnalysis {
            avg_predicted,
            avg_actual,
            correlation,
            overconfidence,
            underconfidence,
        }
    }
}

/// Helper struct for calibration analysis
#[derive(Debug, Clone)]
struct CalibrationAnalysis {
    avg_predicted: f32,
    avg_actual: f32,
    correlation: f32,
    overconfidence: f32,
    underconfidence: f32,
}

impl Default for CalibrationAnalysis {
    fn default() -> Self {
        Self {
            avg_predicted: 0.5,
            avg_actual: 0.5,
            correlation: 0.0,
            overconfidence: 0.0,
            underconfidence: 0.0,
        }
    }
}

impl FeedbackIntegrator {
    pub fn new(config: FeedbackIntegrationConfig) -> BrainResult<Self> {
        Ok(Self {
            config,
            feedback_queue: RwLock::new(Vec::new()),
            feedback_history: RwLock::new(Vec::new()),
            processors: Vec::new(),
        })
    }
    
    pub async fn process_agent_feedback(&self, agent_id: &str) -> BrainResult<FeedbackProcessingResult> {
        // Get pending feedback from queue
        let feedback_to_process = {
            let mut queue = self.feedback_queue.write().await;
            let agent_feedback: Vec<UserFeedback> = queue
                .drain(..)
                .filter(|f| f.agent_id == agent_id)
                .collect();
            agent_feedback
        };
        
        if feedback_to_process.is_empty() {
            return Ok(FeedbackProcessingResult {
                processed_count: 0,
                key_insights: Vec::new(),
                recommended_actions: Vec::new(),
                overall_sentiment: 0.5,
            });
        }
        
        let mut key_insights = Vec::new();
        let mut recommended_actions = Vec::new();
        let mut total_sentiment = 0.0;
        let mut processed_count = 0;
        
        // Process each feedback item
        for feedback in &feedback_to_process {
            if feedback.confidence < self.config.min_feedback_confidence {
                continue; // Skip low-confidence feedback
            }
            
            // Extract insights from feedback
            let insights = self.extract_insights_from_feedback(feedback).await?;
            key_insights.extend(insights);
            
            // Generate recommended actions
            let actions = self.generate_actions_from_feedback(feedback).await?;
            recommended_actions.extend(actions);
            
            total_sentiment += feedback.rating;
            processed_count += 1;
        }
        
        let overall_sentiment = if processed_count > 0 {
            total_sentiment / processed_count as f32
        } else {
            0.5
        };
        
        // Prioritize and deduplicate actions
        recommended_actions = self.prioritize_actions(recommended_actions);
        
        // Store processed feedback in history
        {
            let mut history = self.feedback_history.write().await;
            for feedback in feedback_to_process {
                history.push(ProcessedFeedback {
                    original_feedback_id: feedback.feedback_id.clone(),
                    processed_timestamp: chrono::Utc::now(),
                    insights: key_insights.clone(),
                    recommended_actions: recommended_actions.clone(),
                    processing_confidence: 0.8,
                    impact_assessment: self.assess_feedback_impact(&feedback, &key_insights),
                });
            }
            
            // Keep only last 100 processed feedback items
            let history_len = history.len();
            if history_len > 100 {
                history.drain(0..history_len - 100);
            }
        }
        
        Ok(FeedbackProcessingResult {
            processed_count,
            key_insights,
            recommended_actions,
            overall_sentiment,
        })
    }
    
    /// Add user feedback to processing queue
    pub async fn add_user_feedback(&self, feedback: UserFeedback) -> BrainResult<()> {
        let mut queue = self.feedback_queue.write().await;
        queue.push(feedback);
        
        // Limit queue size
        let queue_len = queue.len();
        if queue_len > 1000 {
            queue.drain(0..queue_len - 1000);
        }
        
        Ok(())
    }
    
    /// Extract insights from individual feedback
    async fn extract_insights_from_feedback(&self, feedback: &UserFeedback) -> BrainResult<Vec<FeedbackInsight>> {
        let mut insights = Vec::new();
        
        // Analyze feedback rating and type
        match feedback.feedback_type {
            FeedbackType::Accuracy => {
                if feedback.rating < 0.5 {
                    insights.push(FeedbackInsight {
                        description: "Low accuracy reported by user".to_string(),
                        category: InsightCategory::ErrorPrevention,
                        confidence: 0.8,
                        evidence: vec![
                            format!("User rating: {:.2}", feedback.rating),
                            feedback.comment.clone().unwrap_or_default(),
                        ],
                        actionability: 0.9,
                    });
                } else if feedback.rating > 0.8 {
                    insights.push(FeedbackInsight {
                        description: "High accuracy appreciated by user".to_string(),
                        category: InsightCategory::PerformanceOptimization,
                        confidence: 0.7,
                        evidence: vec![format!("User rating: {:.2}", feedback.rating)],
                        actionability: 0.6,
                    });
                }
            },
            FeedbackType::Efficiency => {
                if feedback.rating < 0.6 {
                    insights.push(FeedbackInsight {
                        description: "Performance issues reported by user".to_string(),
                        category: InsightCategory::PerformanceOptimization,
                        confidence: 0.8,
                        evidence: vec![
                            format!("Efficiency rating: {:.2}", feedback.rating),
                            feedback.context.clone(),
                        ],
                        actionability: 0.8,
                    });
                }
            },
            FeedbackType::Usefulness => {
                if feedback.rating < 0.5 {
                    insights.push(FeedbackInsight {
                        description: "Output usefulness needs improvement".to_string(),
                        category: InsightCategory::UserPreferences,
                        confidence: 0.7,
                        evidence: vec![
                            format!("Usefulness rating: {:.2}", feedback.rating),
                            feedback.comment.clone().unwrap_or_default(),
                        ],
                        actionability: 0.7,
                    });
                }
            },
            FeedbackType::ErrorReport => {
                insights.push(FeedbackInsight {
                    description: "Error reported by user needs investigation".to_string(),
                    category: InsightCategory::ErrorPrevention,
                    confidence: 0.9,
                    evidence: vec![
                        feedback.comment.clone().unwrap_or_default(),
                        feedback.context.clone(),
                    ],
                    actionability: 0.95,
                });
            },
            _ => {
                // General feedback analysis
                if feedback.rating < 0.4 {
                    insights.push(FeedbackInsight {
                        description: "General dissatisfaction reported".to_string(),
                        category: InsightCategory::UserPreferences,
                        confidence: 0.6,
                        evidence: vec![format!("Overall rating: {:.2}", feedback.rating)],
                        actionability: 0.5,
                    });
                }
            }
        }
        
        // Analyze aspect ratings for specific insights
        for (aspect, rating) in &feedback.aspect_ratings {
            if *rating < 0.5 {
                insights.push(FeedbackInsight {
                    description: format!("Poor {} performance reported", aspect),
                    category: InsightCategory::PerformanceOptimization,
                    confidence: 0.7,
                    evidence: vec![format!("{} rating: {:.2}", aspect, rating)],
                    actionability: 0.8,
                });
            }
        }
        
        Ok(insights)
    }
    
    /// Generate recommended actions from feedback
    async fn generate_actions_from_feedback(&self, feedback: &UserFeedback) -> BrainResult<Vec<FeedbackAction>> {
        let mut actions = Vec::new();
        
        match feedback.feedback_type {
            FeedbackType::Accuracy if feedback.rating < 0.6 => {
                actions.push(FeedbackAction {
                    description: "Improve accuracy validation and quality checks".to_string(),
                    action_type: ActionType::ParameterAdjustment,
                    priority: ActionPriority::High,
                    expected_impact: 0.7,
                    complexity: 0.6,
                    estimated_time_hours: 8.0,
                });
            },
            FeedbackType::Efficiency if feedback.rating < 0.6 => {
                actions.push(FeedbackAction {
                    description: "Optimize response time and resource usage".to_string(),
                    action_type: ActionType::ParameterAdjustment,
                    priority: ActionPriority::Medium,
                    expected_impact: 0.6,
                    complexity: 0.7,
                    estimated_time_hours: 12.0,
                });
            },
            FeedbackType::ErrorReport => {
                actions.push(FeedbackAction {
                    description: "Investigate and fix reported error".to_string(),
                    action_type: ActionType::IssueFix,
                    priority: ActionPriority::Critical,
                    expected_impact: 0.8,
                    complexity: 0.5,
                    estimated_time_hours: 4.0,
                });
            },
            FeedbackType::Usefulness if feedback.rating < 0.5 => {
                actions.push(FeedbackAction {
                    description: "Enhance output relevance and user value".to_string(),
                    action_type: ActionType::BehaviorModification,
                    priority: ActionPriority::Medium,
                    expected_impact: 0.5,
                    complexity: 0.8,
                    estimated_time_hours: 16.0,
                });
            },
            _ => {}
        }
        
        // General improvement actions for very low ratings
        if feedback.rating < 0.3 {
            actions.push(FeedbackAction {
                description: "Comprehensive agent behavior review".to_string(),
                action_type: ActionType::BehaviorModification,
                priority: ActionPriority::High,
                expected_impact: 0.6,
                complexity: 0.9,
                estimated_time_hours: 24.0,
            });
        }
        
        Ok(actions)
    }
    
    /// Prioritize and deduplicate actions
    fn prioritize_actions(&self, mut actions: Vec<FeedbackAction>) -> Vec<FeedbackAction> {
        // Sort by priority and expected impact
        actions.sort_by(|a, b| {
            let priority_order = |p: &ActionPriority| match p {
                ActionPriority::Critical => 0,
                ActionPriority::High => 1,
                ActionPriority::Medium => 2,
                ActionPriority::Low => 3,
                ActionPriority::Optional => 4,
            };
            
            let a_priority = priority_order(&a.priority);
            let b_priority = priority_order(&b.priority);
            
            a_priority.cmp(&b_priority)
                .then_with(|| b.expected_impact.partial_cmp(&a.expected_impact).unwrap_or(std::cmp::Ordering::Equal))
        });
        
        // Take top 10 actions to avoid overwhelming the system
        actions.truncate(10);
        actions
    }
    
    /// Assess the potential impact of feedback
    fn assess_feedback_impact(&self, feedback: &UserFeedback, insights: &[FeedbackInsight]) -> ImpactAssessment {
        let mut overall_impact = feedback.confidence * feedback.rating;
        
        // Adjust impact based on feedback type
        match feedback.feedback_type {
            FeedbackType::ErrorReport => overall_impact = (overall_impact * 1.5).min(1.0),
            FeedbackType::Accuracy => overall_impact = (overall_impact * 1.2).min(1.0),
            _ => {}
        }
        
        // Calculate average actionability from insights
        let avg_actionability = if !insights.is_empty() {
            insights.iter().map(|i| i.actionability).sum::<f32>() / insights.len() as f32
        } else {
            0.5
        };
        
        let risk_level = if matches!(feedback.feedback_type, FeedbackType::ErrorReport) {
            0.8
        } else if feedback.rating < 0.3 {
            0.6
        } else {
            0.3
        };
        
        ImpactAssessment {
            overall_impact: (overall_impact + avg_actionability) / 2.0,
            metric_impacts: std::collections::HashMap::new(),
            affected_user_segments: vec!["current_user".to_string()],
            potential_reach: feedback.confidence,
            risk_level,
        }
    }
}

impl ParameterTuner {
    pub fn new(config: ParameterTuningConfig) -> BrainResult<Self> {
        Ok(Self {
            config,
            active_experiments: RwLock::new(HashMap::new()),
            tuning_history: RwLock::new(Vec::new()),
            strategies: Vec::new(),
        })
    }
    
    pub async fn check_and_tune_parameters(&self, agent_id: &str, data: &[AgentPerformanceMetrics]) -> BrainResult<ParameterTuningResult> {
        if data.len() < 5 {
            return Ok(ParameterTuningResult {
                tuning_performed: false,
                adjusted_parameters: HashMap::new(),
                expected_improvement: 0.0,
                confidence: 0.0,
            });
        }
        
        // Analyze performance trends to determine if tuning is needed
        let performance_analysis = self.analyze_performance_trends(data);
        
        if !performance_analysis.needs_tuning {
            return Ok(ParameterTuningResult {
                tuning_performed: false,
                adjusted_parameters: HashMap::new(),
                expected_improvement: 0.0,
                confidence: 0.5,
            });
        }
        
        let mut adjusted_parameters = HashMap::new();
        let mut total_expected_improvement = 0.0;
        
        // Parameter tuning based on performance issues
        if performance_analysis.slow_response_time {
            // Tune for speed optimization
            adjusted_parameters.insert("response_timeout".to_string(), 0.8); // Reduce timeout
            adjusted_parameters.insert("batch_size".to_string(), 0.5); // Smaller batches
            adjusted_parameters.insert("concurrency_limit".to_string(), 1.2); // More concurrency
            total_expected_improvement += 0.15;
        }
        
        if performance_analysis.low_accuracy {
            // Tune for accuracy improvement
            adjusted_parameters.insert("confidence_threshold".to_string(), 1.1); // Higher confidence requirement
            adjusted_parameters.insert("validation_steps".to_string(), 1.3); // More validation
            adjusted_parameters.insert("learning_rate".to_string(), 0.8); // Lower learning rate for stability
            total_expected_improvement += 0.20;
        }
        
        if performance_analysis.high_resource_usage {
            // Tune for resource efficiency
            adjusted_parameters.insert("memory_limit".to_string(), 0.8); // Reduce memory usage
            adjusted_parameters.insert("gc_frequency".to_string(), 1.2); // More frequent garbage collection
            adjusted_parameters.insert("cache_size".to_string(), 0.9); // Smaller cache
            total_expected_improvement += 0.10;
        }
        
        if performance_analysis.inconsistent_quality {
            // Tune for consistency
            adjusted_parameters.insert("temperature".to_string(), 0.9); // Lower temperature for more consistent outputs
            adjusted_parameters.insert("top_k".to_string(), 0.8); // More focused selection
            adjusted_parameters.insert("repetition_penalty".to_string(), 1.1); // Reduce repetition
            total_expected_improvement += 0.12;
        }
        
        // Adaptive learning rate based on recent performance
        let recent_accuracy: f32 = data.iter().take(5).map(|m| m.quality_metrics.accuracy).sum::<f32>() / 5.0;
        if recent_accuracy < 0.6 {
            adjusted_parameters.insert("exploration_rate".to_string(), 1.2); // More exploration
            total_expected_improvement += 0.08;
        } else if recent_accuracy > 0.9 {
            adjusted_parameters.insert("exploitation_rate".to_string(), 1.1); // More exploitation
            total_expected_improvement += 0.05;
        }
        
        // Calculate tuning confidence based on data quality and consistency
        let confidence = self.calculate_tuning_confidence(data, &performance_analysis);
        
        // Store tuning result for future analysis
        {
            let mut tuning_history = self.tuning_history.write().await;
            tuning_history.push(TuningResult {
                experiment_id: format!("auto_tune_{}", chrono::Utc::now().timestamp_millis()),
                agent_id: agent_id.to_string(),
                success: true, // Will be determined later
                improvement: total_expected_improvement,
                final_parameters: adjusted_parameters.clone(),
                strategy_used: "adaptive_performance_tuning".to_string(),
                lessons_learned: vec![
                    format!("Performance analysis: {:?}", performance_analysis),
                    format!("Tuned {} parameters", adjusted_parameters.len()),
                ],
                completed_timestamp: chrono::Utc::now(),
            });
            
            // Keep only last 50 tuning results
            let tuning_history_len = tuning_history.len();
            if tuning_history_len > 50 {
                tuning_history.drain(0..tuning_history_len - 50);
            }
        }
        
        Ok(ParameterTuningResult {
            tuning_performed: !adjusted_parameters.is_empty(),
            adjusted_parameters,
            expected_improvement: total_expected_improvement,
            confidence,
        })
    }
    
    /// Analyze performance trends to determine tuning needs
    fn analyze_performance_trends(&self, data: &[AgentPerformanceMetrics]) -> PerformanceAnalysis {
        if data.len() < 3 {
            return PerformanceAnalysis::default();
        }
        
        // Calculate average metrics
        let avg_response_time: f32 = data.iter().map(|m| m.execution_metrics.avg_execution_time_ms as f32).sum::<f32>() / data.len() as f32;
        let avg_accuracy: f32 = data.iter().map(|m| m.quality_metrics.accuracy).sum::<f32>() / data.len() as f32;
        let avg_memory_usage: f32 = data.iter().map(|m| m.resource_metrics.avg_memory_usage_mb as f32).sum::<f32>() / data.len() as f32;
        
        // Calculate variance for consistency analysis
        let accuracy_variance: f32 = data.iter()
            .map(|m| (m.quality_metrics.accuracy - avg_accuracy).powi(2))
            .sum::<f32>() / data.len() as f32;
        
        // Determine performance issues
        let slow_response_time = avg_response_time > 2.0; // More than 2 seconds
        let low_accuracy = avg_accuracy < 0.7; // Less than 70% accuracy
        let high_resource_usage = avg_memory_usage > 1000.0; // More than 1GB
        let inconsistent_quality = accuracy_variance > 0.1; // High variance in accuracy
        
        let needs_tuning = slow_response_time || low_accuracy || high_resource_usage || inconsistent_quality;
        
        PerformanceAnalysis {
            needs_tuning,
            slow_response_time,
            low_accuracy,
            high_resource_usage,
            inconsistent_quality,
            avg_response_time,
            avg_accuracy,
            avg_memory_usage,
            _accuracy_variance: accuracy_variance,
        }
    }
    
    /// Calculate confidence in tuning recommendations
    fn calculate_tuning_confidence(&self, data: &[AgentPerformanceMetrics], analysis: &PerformanceAnalysis) -> f32 {
        let mut confidence = 0.5; // Base confidence
        
        // More data points increase confidence
        confidence += (data.len() as f32 / 20.0).min(0.2);
        
        // Clear performance issues increase confidence
        if analysis.slow_response_time && analysis.avg_response_time > 3.0 {
            confidence += 0.2;
        }
        if analysis.low_accuracy && analysis.avg_accuracy < 0.5 {
            confidence += 0.2;
        }
        if analysis.high_resource_usage && analysis.avg_memory_usage > 2000.0 {
            confidence += 0.1;
        }
        
        // Multiple issues increase confidence in need for tuning
        let issue_count = [
            analysis.slow_response_time,
            analysis.low_accuracy,
            analysis.high_resource_usage,
            analysis.inconsistent_quality,
        ].iter().filter(|&&x| x).count();
        
        confidence += (issue_count as f32 * 0.1).min(0.3);
        
        confidence.min(1.0)
    }
}

/// Helper struct for performance analysis
#[derive(Debug, Clone)]
struct PerformanceAnalysis {
    needs_tuning: bool,
    slow_response_time: bool,
    low_accuracy: bool,
    high_resource_usage: bool,
    inconsistent_quality: bool,
    avg_response_time: f32,
    avg_accuracy: f32,
    avg_memory_usage: f32,
    _accuracy_variance: f32,
}

impl Default for PerformanceAnalysis {
    fn default() -> Self {
        Self {
            needs_tuning: false,
            slow_response_time: false,
            low_accuracy: false,
            high_resource_usage: false,
            inconsistent_quality: false,
            avg_response_time: 1.0,
            avg_accuracy: 0.8,
            avg_memory_usage: 512.0,
            _accuracy_variance: 0.05,
        }
    }
}

impl LearningStrategyManager {
    pub fn new() -> BrainResult<Self> {
        Ok(Self {
            available_strategies: vec![
                LearningStrategy::ReactiveLearning,
                LearningStrategy::ProactiveLearning,
                LearningStrategy::ContinuousLearning,
            ],
            strategy_effectiveness: RwLock::new(HashMap::new()),
            strategy_assignments: RwLock::new(HashMap::new()),
            selection_algorithm: Box::new(SimpleStrategySelector),
        })
    }
}

/// Simple strategy selector implementation
struct SimpleStrategySelector;

impl StrategySelector for SimpleStrategySelector {
    fn select_strategies(
        &self,
        _context: &LearningContext,
        available_strategies: &[LearningStrategy],
        _strategy_metrics: &HashMap<LearningStrategy, StrategyMetrics>,
    ) -> BrainResult<Vec<LearningStrategy>> {
        Ok(available_strategies.to_vec())
    }
    
    fn name(&self) -> &str {
        "SimpleStrategySelector"
    }
} 