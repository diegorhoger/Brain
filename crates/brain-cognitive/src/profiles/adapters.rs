use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use brain_types::error::BrainError;
use crate::agents::traits::{
    BrainResult, CognitivePreferenceProfile, InteractionMode, DetailLevel,
    EmotionalSensitivity, AutonomyLevel, CommunicationStyle, VerbosityLevel,
    PacingPreference
};
use super::{
    BehaviorAdapter, BehaviorConfiguration, AdaptationContext, AdaptationRecommendation,
    BehaviorAspect, AutonomyBoundaries, CommunicationAdaptations, CognitiveLoadManagement,
    CommunicationTone, TechnicalDepth, ExampleUsage, EmotionalAwareness,
    ProgressiveDisclosure, ContextSwitchingSettings, AttentionManagement, EscalationRule,
    EscalationTrigger, EscalationAction
};

/// Standard behavior adapter implementation
pub struct StandardBehaviorAdapter {
    /// Agent-specific adaptation rules
    agent_rules: Arc<HashMap<String, AgentAdaptationRules>>,
    
    /// Global adaptation settings
    #[allow(dead_code)]
    global_settings: AdaptationSettings,
}

/// Agent-specific adaptation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAdaptationRules {
    /// Agent identifier
    pub agent_id: String,
    
    /// Agent category (development, security, etc.)
    pub category: String,
    
    /// Verbosity adaptation rules
    pub verbosity_rules: VerbosityAdaptationRules,
    
    /// Autonomy adaptation rules
    pub autonomy_rules: AutonomyAdaptationRules,
    
    /// Communication adaptation rules
    pub communication_rules: CommunicationAdaptationRules,
    
    /// Cognitive load adaptation rules
    pub cognitive_load_rules: CognitiveLoadAdaptationRules,
    
    /// Custom adaptation parameters
    pub custom_parameters: HashMap<String, serde_json::Value>,
}

/// Verbosity adaptation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbosityAdaptationRules {
    /// Base verbosity level
    pub base_level: VerbosityLevel,
    
    /// Adaptation based on detail preference
    pub detail_level_multipliers: HashMap<DetailLevel, f32>,
    
    /// Adaptation based on interaction mode
    pub interaction_mode_adjustments: HashMap<InteractionMode, VerbosityAdjustment>,
    
    /// Emotional sensitivity adjustments
    pub emotional_sensitivity_adjustments: HashMap<EmotionalSensitivity, VerbosityAdjustment>,
}

/// Verbosity adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbosityAdjustment {
    /// Level adjustment
    pub level_change: i8,
    
    /// Enable additional explanations
    pub enable_explanations: bool,
    
    /// Include emotional cues
    pub include_emotional_cues: bool,
    
    /// Add encouragement
    pub add_encouragement: bool,
}

/// Autonomy adaptation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyAdaptationRules {
    /// Default risk tolerance
    pub default_risk_tolerance: f32,
    
    /// Actions requiring confirmation by autonomy level
    pub confirmation_requirements: HashMap<AutonomyLevel, Vec<String>>,
    
    /// Escalation procedures by autonomy level
    pub escalation_procedures: HashMap<AutonomyLevel, Vec<EscalationRule>>,
    
    /// Auto-approval thresholds
    pub auto_approval_thresholds: HashMap<String, f32>,
}

/// Communication adaptation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationAdaptationRules {
    /// Tone mapping by communication style
    pub tone_mapping: HashMap<CommunicationStyle, CommunicationTone>,
    
    /// Technical depth by interaction mode
    pub technical_depth_mapping: HashMap<InteractionMode, TechnicalDepth>,
    
    /// Example usage by detail level
    pub example_usage_mapping: HashMap<DetailLevel, ExampleUsage>,
    
    /// Emotional awareness by sensitivity level
    pub emotional_awareness_mapping: HashMap<EmotionalSensitivity, EmotionalAwareness>,
    
    /// Custom communication parameters
    pub custom_parameters: HashMap<String, serde_json::Value>,
}

/// Cognitive load adaptation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveLoadAdaptationRules {
    /// Chunk size by cognitive load settings
    pub chunk_size_mapping: HashMap<u8, usize>, // max_items_per_chunk -> actual chunk size
    
    /// Pacing adjustments
    pub pacing_adjustments: HashMap<PacingPreference, PacingSettings>,
    
    /// Progressive disclosure settings
    pub progressive_disclosure_settings: ProgressiveDisclosureSettings,
    
    /// Context switching rules
    pub context_switching_rules: ContextSwitchingRules,
    
    /// Attention management rules
    pub attention_management_rules: AttentionManagementRules,
}

/// Pacing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingSettings {
    /// Delay between information chunks (milliseconds)
    pub chunk_delay_ms: u64,
    
    /// Maximum information rate (items per second)
    pub max_info_rate: f32,
    
    /// Adaptive pacing enabled
    pub adaptive_enabled: bool,
}

/// Progressive disclosure settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressiveDisclosureSettings {
    /// Enable by default
    pub enabled_by_default: bool,
    
    /// Initial disclosure level
    pub initial_level: u8,
    
    /// Trigger thresholds for next level
    pub trigger_thresholds: HashMap<String, f32>,
}

/// Context switching rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSwitchingRules {
    /// Minimize switches by default
    pub minimize_by_default: bool,
    
    /// Context preservation duration by interaction mode
    pub preservation_duration_mapping: HashMap<InteractionMode, u64>,
    
    /// Transition assistance settings
    pub transition_assistance_settings: TransitionAssistanceSettings,
}

/// Transition assistance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionAssistanceSettings {
    /// Provide context summaries
    pub provide_summaries: bool,
    
    /// Highlight changes
    pub highlight_changes: bool,
    
    /// Offer navigation aids
    pub offer_navigation: bool,
}

/// Attention management rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionManagementRules {
    /// Focus enhancement techniques
    pub focus_enhancement_mapping: HashMap<InteractionMode, Vec<FocusEnhancementTechnique>>,
    
    /// Distraction filtering rules
    pub distraction_filtering_rules: DistractionFilteringRules,
    
    /// Priority highlighting rules
    pub priority_highlighting_rules: PriorityHighlightingRules,
}

/// Focus enhancement technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FocusEnhancementTechnique {
    MinimizeInterruptions,
    ProgressIndicators,
    BreakReminders,
    PriorityQueuing,
    DeepWorkMode,
}

/// Distraction filtering rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistractionFilteringRules {
    /// Filter low-priority notifications
    pub filter_low_priority: bool,
    
    /// Defer non-urgent information
    pub defer_non_urgent: bool,
    
    /// Batch similar information
    pub batch_similar_info: bool,
}

/// Priority highlighting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityHighlightingRules {
    /// Highlight critical information
    pub highlight_critical: bool,
    
    /// Use visual emphasis
    pub use_visual_emphasis: bool,
    
    /// Priority ordering
    pub priority_ordering: bool,
}

/// Global adaptation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationSettings {
    /// Enable adaptive behavior
    pub adaptive_enabled: bool,
    
    /// Learning rate for adaptation
    pub learning_rate: f32,
    
    /// Confidence threshold for applying adaptations
    pub confidence_threshold: f32,
    
    /// Maximum adaptation intensity
    pub max_adaptation_intensity: f32,
    
    /// Adaptation persistence duration
    pub adaptation_persistence_duration: u64,
}

impl StandardBehaviorAdapter {
    /// Create a new standard behavior adapter
    pub fn new() -> Self {
        Self {
            agent_rules: Arc::new(Self::default_agent_rules()),
            global_settings: AdaptationSettings::default(),
        }
    }
    
    /// Initialize default agent rules
    fn default_agent_rules() -> HashMap<String, AgentAdaptationRules> {
        let mut rules = HashMap::new();
        
        // Development agents
        for agent_id in ["PlannerAgent", "ArchitectAgent", "DesignerAgent", "SchemaAgent", "APIAgent",
                        "FrontendCoder", "BackendCoder", "RefactorAgent", "DocAgent", "DeployerAgent", "MaintainerAgent"] {
            rules.insert(agent_id.to_string(), Self::development_agent_rules(agent_id));
        }
        
        // Security agents
        for agent_id in ["CyberSecurityAgent", "PromptSecurityAgent", "PrivacyComplianceAgent", 
                        "DataPrivacyAgent", "EthicalAIAgent"] {
            rules.insert(agent_id.to_string(), Self::security_agent_rules(agent_id));
        }
        
        // Testing & operations agents
        for agent_id in ["QAAgent", "SandboxEnvironmentAgent", "ObservabilityAgent", "BuildOptimizerAgent",
                        "DriftDetectionAgent", "HotfixAgent", "BackupRecoveryAgent", "ReplicationScalingAgent"] {
            rules.insert(agent_id.to_string(), Self::operations_agent_rules(agent_id));
        }
        
        // Intelligence & platform agents
        for agent_id in ["UserBehaviorAnalystAgent", "FeatureExperimentationAgent", "MLOpsAgent", 
                        "ModelTrainingAgent", "DataIngestionAgent", "LocalizationAgent", 
                        "PlatformCompatibilityAgent", "DataVisualizationAgent", "APIGatewayAgent",
                        "ServiceMeshAgent", "ContainerOrchestrationAgent", "InfrastructureProvisioningAgent",
                        "SystemOrchestrationAgent"] {
            rules.insert(agent_id.to_string(), Self::intelligence_agent_rules(agent_id));
        }
        
        rules
    }
    
    /// Create rules for development agents
    fn development_agent_rules(agent_id: &str) -> AgentAdaptationRules {
        AgentAdaptationRules {
            agent_id: agent_id.to_string(),
            category: "development".to_string(),
            verbosity_rules: VerbosityAdaptationRules {
                base_level: VerbosityLevel::Standard,
                detail_level_multipliers: [
                    (DetailLevel::Minimal, 0.7),
                    (DetailLevel::Standard, 1.0),
                    (DetailLevel::Detailed, 1.3),
                    (DetailLevel::Comprehensive, 1.6),
                ].iter().cloned().collect(),
                interaction_mode_adjustments: [
                    (InteractionMode::Focused, VerbosityAdjustment {
                        level_change: -1,
                        enable_explanations: false,
                        include_emotional_cues: false,
                        add_encouragement: false,
                    }),
                    (InteractionMode::Collaborative, VerbosityAdjustment {
                        level_change: 1,
                        enable_explanations: true,
                        include_emotional_cues: true,
                        add_encouragement: true,
                    }),
                ].iter().cloned().collect(),
                emotional_sensitivity_adjustments: HashMap::new(),
            },
            autonomy_rules: AutonomyAdaptationRules {
                default_risk_tolerance: 0.7,
                confirmation_requirements: [
                    (AutonomyLevel::Manual, vec!["file_modification".to_string(), "code_generation".to_string()]),
                    (AutonomyLevel::ConfirmFirst, vec!["major_refactor".to_string(), "schema_change".to_string()]),
                    (AutonomyLevel::SemiAuto, vec!["architecture_change".to_string()]),
                    (AutonomyLevel::FullAuto, vec![]),
                ].iter().cloned().collect(),
                escalation_procedures: HashMap::new(),
                auto_approval_thresholds: HashMap::new(),
            },
            communication_rules: CommunicationAdaptationRules {
                tone_mapping: [
                    (CommunicationStyle::Technical, CommunicationTone::Professional),
                    (CommunicationStyle::Casual, CommunicationTone::Friendly),
                    (CommunicationStyle::Formal, CommunicationTone::Formal),
                    (CommunicationStyle::Adaptive, CommunicationTone::Adaptive),
                ].iter().cloned().collect(),
                technical_depth_mapping: [
                    (InteractionMode::Focused, TechnicalDepth::Advanced),
                    (InteractionMode::Collaborative, TechnicalDepth::Intermediate),
                    (InteractionMode::Exploratory, TechnicalDepth::Advanced),
                    (InteractionMode::Autonomous, TechnicalDepth::Expert),
                ].iter().cloned().collect(),
                example_usage_mapping: HashMap::new(),
                emotional_awareness_mapping: HashMap::new(),
                custom_parameters: HashMap::new(),
            },
            cognitive_load_rules: CognitiveLoadAdaptationRules {
                chunk_size_mapping: [
                    (3, 3),
                    (5, 5),
                    (7, 7),
                    (10, 10),
                ].iter().cloned().collect(),
                pacing_adjustments: HashMap::new(),
                progressive_disclosure_settings: ProgressiveDisclosureSettings {
                    enabled_by_default: true,
                    initial_level: 1,
                    trigger_thresholds: HashMap::new(),
                },
                context_switching_rules: ContextSwitchingRules {
                    minimize_by_default: true,
                    preservation_duration_mapping: HashMap::new(),
                    transition_assistance_settings: TransitionAssistanceSettings {
                        provide_summaries: true,
                        highlight_changes: true,
                        offer_navigation: true,
                    },
                },
                attention_management_rules: AttentionManagementRules {
                    focus_enhancement_mapping: HashMap::new(),
                    distraction_filtering_rules: DistractionFilteringRules {
                        filter_low_priority: true,
                        defer_non_urgent: true,
                        batch_similar_info: true,
                    },
                    priority_highlighting_rules: PriorityHighlightingRules {
                        highlight_critical: true,
                        use_visual_emphasis: true,
                        priority_ordering: true,
                    },
                },
            },
            custom_parameters: HashMap::new(),
        }
    }
    
    /// Create rules for security agents
    fn security_agent_rules(agent_id: &str) -> AgentAdaptationRules {
        let mut rules = Self::development_agent_rules(agent_id);
        rules.category = "security".to_string();
        rules.autonomy_rules.default_risk_tolerance = 0.3; // Lower risk tolerance for security
        rules.autonomy_rules.confirmation_requirements.insert(
            AutonomyLevel::SemiAuto, 
            vec!["security_policy_change".to_string(), "permission_modification".to_string()]
        );
        rules
    }
    
    /// Create rules for operations agents
    fn operations_agent_rules(agent_id: &str) -> AgentAdaptationRules {
        let mut rules = Self::development_agent_rules(agent_id);
        rules.category = "operations".to_string();
        rules.autonomy_rules.default_risk_tolerance = 0.5; // Medium risk tolerance for operations
        rules
    }
    
    /// Create rules for intelligence agents
    fn intelligence_agent_rules(agent_id: &str) -> AgentAdaptationRules {
        let mut rules = Self::development_agent_rules(agent_id);
        rules.category = "intelligence".to_string();
        rules.autonomy_rules.default_risk_tolerance = 0.8; // Higher risk tolerance for intelligence
        rules
    }
    
    /// Adapt verbosity based on profile and rules
    fn adapt_verbosity(
        &self,
        profile: &CognitivePreferenceProfile,
        rules: &VerbosityAdaptationRules,
        _context: &AdaptationContext
    ) -> VerbosityLevel {
        let mut base_multiplier = rules.detail_level_multipliers
            .get(&profile.detail_level)
            .copied()
            .unwrap_or(1.0);
        
        // Apply interaction mode adjustments
        if let Some(adjustment) = rules.interaction_mode_adjustments.get(&profile.interaction_mode) {
            base_multiplier += adjustment.level_change as f32 * 0.2;
        }
        
        // Convert multiplier to verbosity level
        match base_multiplier {
            x if x <= 0.8 => VerbosityLevel::Minimal,
            x if x <= 1.2 => VerbosityLevel::Standard,
            x if x <= 1.5 => VerbosityLevel::Detailed,
            _ => VerbosityLevel::Verbose,
        }
    }
    
    /// Generate autonomy boundaries based on profile and rules
    fn generate_autonomy_boundaries(
        &self,
        profile: &CognitivePreferenceProfile,
        rules: &AutonomyAdaptationRules,
        _context: &AdaptationContext
    ) -> AutonomyBoundaries {
        let confirmation_required = rules.confirmation_requirements
            .get(&profile.autonomy_level)
            .cloned()
            .unwrap_or_default();
        
        let escalation_procedures = rules.escalation_procedures
            .get(&profile.autonomy_level)
            .cloned()
            .unwrap_or_else(|| {
                vec![
                    EscalationRule {
                        id: "default_escalation".to_string(),
                        trigger: EscalationTrigger::ConfidenceBelowThreshold(0.7),
                        action: EscalationAction::RequestConfirmation,
                        priority: 1,
                    }
                ]
            });
        
        AutonomyBoundaries {
            max_risk_tolerance: rules.default_risk_tolerance,
            confirmation_required,
            auto_approval_thresholds: rules.auto_approval_thresholds.clone(),
            escalation_procedures,
        }
    }
    
    /// Generate communication adaptations
    fn generate_communication_adaptations(
        &self,
        profile: &CognitivePreferenceProfile,
        rules: &CommunicationAdaptationRules,
        _context: &AdaptationContext
    ) -> CommunicationAdaptations {
        let tone = rules.tone_mapping
            .get(&profile.communication_style)
            .copied()
            .unwrap_or(CommunicationTone::Adaptive);
        
        let technical_depth = rules.technical_depth_mapping
            .get(&profile.interaction_mode)
            .copied()
            .unwrap_or(TechnicalDepth::Adaptive);
        
        let example_usage = rules.example_usage_mapping
            .get(&profile.detail_level)
            .copied()
            .unwrap_or(ExampleUsage::Balanced);
        
        let emotional_awareness = rules.emotional_awareness_mapping
            .get(&profile.emotional_sensitivity)
            .copied()
            .unwrap_or(EmotionalAwareness::Basic);
        
        CommunicationAdaptations {
            tone,
            technical_depth,
            example_usage,
            emotional_awareness,
        }
    }
    
    /// Generate cognitive load management settings
    fn generate_cognitive_load_management(
        &self,
        profile: &CognitivePreferenceProfile,
        rules: &CognitiveLoadAdaptationRules,
        _context: &AdaptationContext
    ) -> CognitiveLoadManagement {
        let progressive_disclosure = ProgressiveDisclosure {
            enabled: profile.cognitive_load_settings.progressive_disclosure && rules.progressive_disclosure_settings.enabled_by_default,
            layers: Vec::new(),
            layer_triggers: Vec::new(),
        };
        
        let context_switching = ContextSwitchingSettings {
            minimize_switches: rules.context_switching_rules.minimize_by_default,
            preservation_duration: rules.context_switching_rules.preservation_duration_mapping
                .get(&profile.interaction_mode)
                .copied()
                .unwrap_or(300),
            transition_assistance: rules.context_switching_rules.transition_assistance_settings.provide_summaries,
        };
        
        let attention_management = AttentionManagement {
            focus_enhancement: true,
            distraction_filtering: rules.attention_management_rules.distraction_filtering_rules.filter_low_priority,
            priority_highlighting: rules.attention_management_rules.priority_highlighting_rules.highlight_critical,
        };
        
        CognitiveLoadManagement {
            pacing: profile.cognitive_load_settings.pacing_preference.clone(),
            progressive_disclosure,
            context_switching,
            attention_management,
        }
    }
}

impl Default for StandardBehaviorAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AdaptationSettings {
    fn default() -> Self {
        Self {
            adaptive_enabled: true,
            learning_rate: 0.1,
            confidence_threshold: 0.7,
            max_adaptation_intensity: 0.8,
            adaptation_persistence_duration: 3600, // 1 hour
        }
    }
}

#[async_trait]
impl BehaviorAdapter for StandardBehaviorAdapter {
    /// Adapt agent behavior based on cognitive preferences
    async fn adapt_behavior(
        &self,
        agent_id: &str,
        profile: &CognitivePreferenceProfile,
        context: &AdaptationContext
    ) -> BrainResult<BehaviorConfiguration> {
        let rules = self.agent_rules.get(agent_id)
            .ok_or_else(|| BrainError::NotFound(format!("No adaptation rules found for agent: {}", agent_id)))?;
        
        let verbosity = self.adapt_verbosity(profile, &rules.verbosity_rules, context);
        
        let chunking_enabled = profile.cognitive_load_settings.progressive_disclosure;
        let max_chunk_size = rules.cognitive_load_rules.chunk_size_mapping
            .get(&profile.cognitive_load_settings.max_items_per_chunk)
            .copied()
            .unwrap_or(profile.cognitive_load_settings.max_items_per_chunk as usize);
        
        let requires_confirmation = rules.autonomy_rules.confirmation_requirements
            .get(&profile.autonomy_level)
            .cloned()
            .unwrap_or_default();
        
        let autonomy_boundaries = self.generate_autonomy_boundaries(profile, &rules.autonomy_rules, context);
        let communication_adaptations = self.generate_communication_adaptations(profile, &rules.communication_rules, context);
        let cognitive_load_management = self.generate_cognitive_load_management(profile, &rules.cognitive_load_rules, context);
        
        Ok(BehaviorConfiguration {
            agent_id: agent_id.to_string(),
            verbosity,
            chunking_enabled,
            max_chunk_size,
            requires_confirmation,
            autonomy_boundaries,
            communication_adaptations,
            cognitive_load_management,
            custom_settings: rules.custom_parameters.clone(),
        })
    }
    
    /// Get recommended adaptations for an agent
    async fn get_recommendations(
        &self,
        agent_id: &str,
        profile: &CognitivePreferenceProfile
    ) -> BrainResult<Vec<AdaptationRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Example recommendation: suggest higher verbosity for collaborative mode
        if matches!(profile.interaction_mode, InteractionMode::Collaborative) 
            && matches!(profile.detail_level, DetailLevel::Minimal | DetailLevel::Standard) {
            recommendations.push(AdaptationRecommendation {
                id: format!("{}_verbosity_rec_1", agent_id),
                target_aspect: BehaviorAspect::Verbosity,
                recommended_config: BehaviorConfiguration {
                    agent_id: agent_id.to_string(),
                    verbosity: VerbosityLevel::Detailed,
                    ..Default::default()
                },
                expected_improvement: 0.2,
                priority: 2,
                rationale: "Collaborative mode works better with more detailed responses".to_string(),
            });
        }
        
        // Example recommendation: suggest autonomy adjustment for experienced users
        if matches!(profile.detail_level, DetailLevel::Detailed | DetailLevel::Comprehensive)
            && matches!(profile.autonomy_level, AutonomyLevel::Manual) {
            recommendations.push(AdaptationRecommendation {
                id: format!("{}_autonomy_rec_1", agent_id),
                target_aspect: BehaviorAspect::Autonomy,
                recommended_config: BehaviorConfiguration {
                    agent_id: agent_id.to_string(),
                    autonomy_boundaries: AutonomyBoundaries {
                        max_risk_tolerance: 0.8,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                expected_improvement: 0.25,
                priority: 1,
                rationale: "Detailed preference suggests experience; higher autonomy would improve efficiency".to_string(),
            });
        }
        
        Ok(recommendations)
    }
    
    /// Validate behavior configuration
    async fn validate_configuration(&self, config: &BehaviorConfiguration) -> BrainResult<bool> {
        // Validate verbosity level is reasonable
        if matches!(config.verbosity, VerbosityLevel::Verbose) && config.max_chunk_size > 15 {
            return Ok(false); // Too much information at once
        }
        
        // Validate autonomy boundaries are consistent
        if config.autonomy_boundaries.max_risk_tolerance > 1.0 || config.autonomy_boundaries.max_risk_tolerance < 0.0 {
            return Ok(false); // Invalid risk tolerance
        }
        
        // Validate chunk size is reasonable
        if config.max_chunk_size == 0 || config.max_chunk_size > 50 {
            return Ok(false); // Invalid chunk size
        }
        
        Ok(true)
    }
}
