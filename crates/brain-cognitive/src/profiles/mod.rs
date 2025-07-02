use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::agents::traits::{
    BrainResult, CognitivePreferenceProfile, InteractionMode, DetailLevel, 
    EmotionalSensitivity, AutonomyLevel, CommunicationStyle, CognitiveLoadSettings,
    PacingPreference, VerbosityLevel
};

pub mod manager;
pub mod adapters; 
pub mod presets;

/// Core CPP trait for profile management
#[async_trait]
pub trait CognitiveProfileManager: Send + Sync {
    /// Load a user's cognitive preference profile
    async fn load_profile(&self, user_id: &str) -> BrainResult<CognitivePreferenceProfile>;
    
    /// Save a user's cognitive preference profile
    async fn save_profile(&self, user_id: &str, profile: &CognitivePreferenceProfile) -> BrainResult<()>;
    
    /// Update specific preferences within a profile
    async fn update_preferences(&self, user_id: &str, updates: ProfileUpdates) -> BrainResult<CognitivePreferenceProfile>;
    
    /// Get available profile presets
    async fn get_presets(&self) -> BrainResult<Vec<ProfilePreset>>;
    
    /// Apply a preset to a user's profile
    async fn apply_preset(&self, user_id: &str, preset_id: &str) -> BrainResult<CognitivePreferenceProfile>;
    
    /// Get profile analytics and usage patterns
    async fn get_profile_analytics(&self, user_id: &str) -> BrainResult<ProfileAnalytics>;
}

/// CPP system for agent behavior adaptation
#[async_trait]
pub trait BehaviorAdapter: Send + Sync {
    /// Adapt agent behavior based on cognitive preferences
    async fn adapt_behavior(
        &self,
        agent_id: &str,
        profile: &CognitivePreferenceProfile,
        context: &AdaptationContext
    ) -> BrainResult<BehaviorConfiguration>;
    
    /// Get recommended adaptations for an agent
    async fn get_recommendations(
        &self,
        agent_id: &str,
        profile: &CognitivePreferenceProfile
    ) -> BrainResult<Vec<AdaptationRecommendation>>;
    
    /// Validate behavior configuration
    async fn validate_configuration(&self, config: &BehaviorConfiguration) -> BrainResult<bool>;
}

/// Profile update operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdates {
    /// Updated interaction mode
    pub interaction_mode: Option<InteractionMode>,
    
    /// Updated detail level preference
    pub detail_level: Option<DetailLevel>,
    
    /// Updated emotional sensitivity
    pub emotional_sensitivity: Option<EmotionalSensitivity>,
    
    /// Updated autonomy level
    pub autonomy_level: Option<AutonomyLevel>,
    
    /// Updated communication style
    pub communication_style: Option<CommunicationStyle>,
    
    /// Updated cognitive load settings
    pub cognitive_load_settings: Option<CognitiveLoadSettings>,
    
    /// Custom preference overrides
    pub custom_preferences: HashMap<String, serde_json::Value>,
}

/// Predefined profile preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePreset {
    /// Unique preset identifier
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Preset description
    pub description: String,
    
    /// Target user persona
    pub target_persona: String,
    
    /// Pre-configured profile
    pub profile: CognitivePreferenceProfile,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Usage popularity score
    pub popularity_score: f32,
}

/// Profile usage analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileAnalytics {
    /// User identifier
    pub user_id: String,
    
    /// Profile usage statistics
    pub usage_stats: ProfileUsageStats,
    
    /// Preference evolution over time
    pub preference_evolution: Vec<PreferenceSnapshot>,
    
    /// Agent interaction patterns
    pub agent_interactions: HashMap<String, AgentInteractionStats>,
    
    /// Satisfaction metrics
    pub satisfaction_metrics: SatisfactionMetrics,
    
    /// Optimization recommendations
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
}

/// Profile usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUsageStats {
    /// Total sessions using this profile
    pub total_sessions: u64,
    
    /// Average session duration (minutes)
    pub avg_session_duration: f64,
    
    /// Most used interaction mode
    pub most_used_mode: InteractionMode,
    
    /// Preference change frequency
    pub change_frequency: f64,
    
    /// Last profile update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Point-in-time preference snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceSnapshot {
    /// Snapshot timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Profile state at this time
    pub profile: CognitivePreferenceProfile,
    
    /// Trigger for the change
    pub change_trigger: String,
    
    /// User satisfaction at this point
    pub satisfaction_score: Option<f32>,
}

/// Agent interaction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInteractionStats {
    /// Agent identifier
    pub agent_id: String,
    
    /// Total interactions
    pub interaction_count: u64,
    
    /// Average interaction satisfaction
    pub avg_satisfaction: f32,
    
    /// Most effective configuration
    pub best_configuration: BehaviorConfiguration,
    
    /// Success rate with current profile
    pub success_rate: f32,
}

/// User satisfaction metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatisfactionMetrics {
    /// Overall satisfaction score (0.0 to 1.0)
    pub overall_score: f32,
    
    /// Satisfaction by interaction mode
    pub mode_satisfaction: HashMap<InteractionMode, f32>,
    
    /// Satisfaction by agent category
    pub agent_satisfaction: HashMap<String, f32>,
    
    /// Cognitive load comfort level
    pub cognitive_load_comfort: f32,
    
    /// Preference stability score
    pub stability_score: f32,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    /// Recommendation identifier
    pub id: String,
    
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    
    /// Recommended change
    pub description: String,
    
    /// Expected impact
    pub expected_impact: f32,
    
    /// Confidence in recommendation
    pub confidence: f32,
    
    /// Specific changes to apply
    pub proposed_changes: ProfileUpdates,
}

/// Type of optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    InteractionModeAdjustment,
    DetailLevelOptimization,
    AutonomyLevelTuning,
    CognitiveLoadReduction,
    CommunicationStyleImprovement,
    EmotionalSensitivityCalibration,
    CustomPreferenceUpdate,
}

/// Behavior configuration for agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfiguration {
    /// Agent identifier
    pub agent_id: String,
    
    /// Verbosity level adaptation
    pub verbosity: VerbosityLevel,
    
    /// Response chunking settings
    pub chunking_enabled: bool,
    pub max_chunk_size: usize,
    
    /// Confirmation requirements
    pub requires_confirmation: Vec<String>,
    
    /// Autonomy boundaries
    pub autonomy_boundaries: AutonomyBoundaries,
    
    /// Communication adaptations
    pub communication_adaptations: CommunicationAdaptations,
    
    /// Cognitive load management
    pub cognitive_load_management: CognitiveLoadManagement,
    
    /// Custom agent-specific settings
    pub custom_settings: HashMap<String, serde_json::Value>,
}

/// Autonomy boundaries configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyBoundaries {
    /// Maximum risk tolerance
    pub max_risk_tolerance: f32,
    
    /// Actions requiring user confirmation
    pub confirmation_required: Vec<String>,
    
    /// Automatic approval thresholds
    pub auto_approval_thresholds: HashMap<String, f32>,
    
    /// Escalation procedures
    pub escalation_procedures: Vec<EscalationRule>,
}

/// Communication adaptations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationAdaptations {
    /// Tone adjustments
    pub tone: CommunicationTone,
    
    /// Technical depth level
    pub technical_depth: TechnicalDepth,
    
    /// Use of examples and analogies
    pub example_usage: ExampleUsage,
    
    /// Emotional awareness settings
    pub emotional_awareness: EmotionalAwareness,
}

/// Cognitive load management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveLoadManagement {
    /// Information pacing
    pub pacing: PacingPreference,
    
    /// Progressive disclosure settings
    pub progressive_disclosure: ProgressiveDisclosure,
    
    /// Context switching management
    pub context_switching: ContextSwitchingSettings,
    
    /// Attention management
    pub attention_management: AttentionManagement,
}

/// Communication tone settings
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CommunicationTone {
    Professional,
    Friendly,
    Casual,
    Formal,
    Encouraging,
    Direct,
    Adaptive,
}

/// Technical depth level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TechnicalDepth {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Adaptive,
}

/// Example usage preferences
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExampleUsage {
    Minimal,
    Balanced,
    Extensive,
    Adaptive,
}

/// Emotional awareness settings
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EmotionalAwareness {
    Disabled,
    Basic,
    Enhanced,
    Empathetic,
}

/// Progressive disclosure configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressiveDisclosure {
    /// Enable progressive disclosure
    pub enabled: bool,
    
    /// Information layers
    pub layers: Vec<DisclosureLayer>,
    
    /// Trigger for next layer
    pub layer_triggers: Vec<LayerTrigger>,
}

/// Context switching management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSwitchingSettings {
    /// Minimize context switches
    pub minimize_switches: bool,
    
    /// Context preservation duration
    pub preservation_duration: u64,
    
    /// Transition assistance
    pub transition_assistance: bool,
}

/// Attention management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionManagement {
    /// Focus enhancement techniques
    pub focus_enhancement: bool,
    
    /// Distraction filtering
    pub distraction_filtering: bool,
    
    /// Priority highlighting
    pub priority_highlighting: bool,
}

/// Information disclosure layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisclosureLayer {
    /// Layer identifier
    pub id: String,
    
    /// Layer name
    pub name: String,
    
    /// Information to reveal
    pub content_types: Vec<String>,
    
    /// Complexity level
    pub complexity_level: u8,
}

/// Trigger for revealing next layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerTrigger {
    UserRequest,
    TimeDelay(u64),
    ConfidenceThreshold(f32),
    ComprehensionSignal,
    InteractionCount(u32),
}

/// Escalation rule for autonomy boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    /// Rule identifier
    pub id: String,
    
    /// Trigger condition
    pub trigger: EscalationTrigger,
    
    /// Action to take
    pub action: EscalationAction,
    
    /// Escalation priority
    pub priority: u8,
}

/// Escalation trigger condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationTrigger {
    ConfidenceBelowThreshold(f32),
    RiskAboveThreshold(f32),
    UnknownOperation,
    UserDisagreement,
    SystemError,
}

/// Escalation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationAction {
    RequestConfirmation,
    TransferToHuman,
    PauseExecution,
    NotifyUser,
    LogIncident,
}

/// Adaptation context for behavior modification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationContext {
    /// Current session information
    pub session_id: String,
    
    /// User interaction history
    pub interaction_history: Vec<InteractionSummary>,
    
    /// Current task complexity
    pub task_complexity: f32,
    
    /// Time constraints
    pub time_constraints: Option<TimeConstraints>,
    
    /// Environmental factors
    pub environmental_factors: EnvironmentalFactors,
}

/// Interaction summary for adaptation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionSummary {
    /// Interaction timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Agent involved
    pub agent_id: String,
    
    /// User satisfaction
    pub satisfaction: Option<f32>,
    
    /// Cognitive load level
    pub cognitive_load: f32,
    
    /// Adaptation effectiveness
    pub adaptation_effectiveness: Option<f32>,
}

/// Time constraints for adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraints {
    /// Available time for task (minutes)
    pub available_time: u64,
    
    /// Time pressure level
    pub pressure_level: TimePressure,
    
    /// Deadline urgency
    pub deadline_urgency: UrgencyLevel,
}

/// Environmental factors affecting adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalFactors {
    /// Device type (mobile, desktop, tablet)
    pub device_type: DeviceType,
    
    /// Network conditions
    pub network_quality: NetworkQuality,
    
    /// Distraction level
    pub distraction_level: DistractionLevel,
    
    /// Multitasking indicator
    pub multitasking: bool,
}

/// Adaptation recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationRecommendation {
    /// Recommendation identifier
    pub id: String,
    
    /// Target behavior aspect
    pub target_aspect: BehaviorAspect,
    
    /// Recommended configuration
    pub recommended_config: BehaviorConfiguration,
    
    /// Expected improvement
    pub expected_improvement: f32,
    
    /// Implementation priority
    pub priority: u8,
    
    /// Supporting rationale
    pub rationale: String,
}

/// Behavior aspect for adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorAspect {
    Verbosity,
    Autonomy,
    CommunicationStyle,
    CognitiveLoad,
    EmotionalSensitivity,
    TechnicalDepth,
    ResponsePacing,
}

/// Time pressure level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimePressure {
    Low,
    Medium,
    High,
    Critical,
}

/// Urgency level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Device type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    Wearable,
    IoT,
}

/// Network quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkQuality {
    Poor,
    Fair,
    Good,
    Excellent,
}

/// Distraction level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistractionLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Default implementations
impl Default for ProfileUpdates {
    fn default() -> Self {
        Self {
            interaction_mode: None,
            detail_level: None,
            emotional_sensitivity: None,
            autonomy_level: None,
            communication_style: None,
            cognitive_load_settings: None,
            custom_preferences: HashMap::new(),
        }
    }
}

impl Default for BehaviorConfiguration {
    fn default() -> Self {
        Self {
            agent_id: String::new(),
            verbosity: VerbosityLevel::Standard,
            chunking_enabled: true,
            max_chunk_size: 5,
            requires_confirmation: Vec::new(),
            autonomy_boundaries: AutonomyBoundaries::default(),
            communication_adaptations: CommunicationAdaptations::default(),
            cognitive_load_management: CognitiveLoadManagement::default(),
            custom_settings: HashMap::new(),
        }
    }
}

impl Default for AutonomyBoundaries {
    fn default() -> Self {
        Self {
            max_risk_tolerance: 0.7,
            confirmation_required: Vec::new(),
            auto_approval_thresholds: HashMap::new(),
            escalation_procedures: Vec::new(),
        }
    }
}

impl Default for CommunicationAdaptations {
    fn default() -> Self {
        Self {
            tone: CommunicationTone::Adaptive,
            technical_depth: TechnicalDepth::Adaptive,
            example_usage: ExampleUsage::Balanced,
            emotional_awareness: EmotionalAwareness::Basic,
        }
    }
}

impl Default for CognitiveLoadManagement {
    fn default() -> Self {
        Self {
            pacing: PacingPreference::Adaptive,
            progressive_disclosure: ProgressiveDisclosure::default(),
            context_switching: ContextSwitchingSettings::default(),
            attention_management: AttentionManagement::default(),
        }
    }
}

impl Default for ProgressiveDisclosure {
    fn default() -> Self {
        Self {
            enabled: true,
            layers: Vec::new(),
            layer_triggers: Vec::new(),
        }
    }
}

impl Default for ContextSwitchingSettings {
    fn default() -> Self {
        Self {
            minimize_switches: true,
            preservation_duration: 300, // 5 minutes
            transition_assistance: true,
        }
    }
}

impl Default for AttentionManagement {
    fn default() -> Self {
        Self {
            focus_enhancement: true,
            distraction_filtering: true,
            priority_highlighting: true,
        }
    }
} 