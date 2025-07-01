use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use brain_types::error::BrainError;

// Type alias for Result with BrainError
pub type BrainResult<T> = Result<T, BrainError>;
use crate::meta::MetaMemoryRepository;
use crate::conversation::ConversationService;

/// Agent capabilities that define what an agent can do
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentCapability {
    Analysis,
    Security,
    Monitoring,
    ContentModeration,
    Compliance,
    DataGovernance,
    EthicalAI,
    Development,
    Testing,
    Planning,
    Architecture,
    Design,
    Documentation,
    Debugging,
    Optimization,
    Integration,
    Deployment,
    // Testing & QA capabilities
    QualityAssurance,
    PerformanceAnalysis,
    ReportGeneration,
    Infrastructure,
    // Operations capabilities
    BuildOptimization,
    CICDManagement,
    CostOptimization,
    EmergencyResponse,
    RollbackManagement,
    IncidentManagement,
    DatabaseManagement,
    AutoScaling,
    ReplicationManagement,
    PerformanceOptimization,
    DriftDetection,
    ComplianceMonitoring,
    AutoRemediation,
    RiskAssessment,
    BackupManagement,
    DisasterRecovery,
    DataProtection,
    Analytics,
    AlertManagement,
}

/// Core trait that all Brain AI agents must implement
#[async_trait]
pub trait BrainAgent: Send + Sync {
    /// Execute the agent with given input and cognitive context
    async fn execute(
        &self, 
        input: AgentInput, 
        context: &CognitiveContext
    ) -> BrainResult<AgentOutput>;
    
    /// Get agent metadata (name, persona, capabilities)
    fn metadata(&self) -> &AgentMetadata;
    
    /// Minimum confidence threshold for agent to proceed with actions
    fn confidence_threshold(&self) -> f32;
    
    /// Agent's cognitive preferences and behavioral settings
    fn cognitive_preferences(&self) -> &CognitivePreferences;
    
    /// Check if agent can handle the given input type
    fn can_handle(&self, input_type: &str) -> bool {
        self.metadata().supported_input_types.contains(&input_type.to_string())
    }
    
    /// Get agent's current confidence level for a specific task
    async fn assess_confidence(
        &self, 
        input: &AgentInput, 
        context: &CognitiveContext
    ) -> BrainResult<f32>;
}

/// Metadata describing an agent's capabilities and characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Unique identifier for the agent
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Agent's persona and behavioral description
    pub persona: String,
    
    /// Detailed description of the agent's purpose and functionality
    pub description: String,
    
    /// Version of the agent implementation
    pub version: String,
    
    /// List of input types this agent can process
    pub supported_input_types: Vec<String>,
    
    /// List of output types this agent can produce
    pub supported_output_types: Vec<String>,
    
    /// Agent's primary capabilities and skills
    pub capabilities: Vec<String>,
    
    /// Dependencies on other agents (for orchestration)
    pub dependencies: Vec<String>,
    
    /// Tags for categorization and discovery
    pub tags: Vec<String>,
    
    /// Agent's confidence level (0.0 to 1.0)
    pub base_confidence: f32,
}

/// Input data structure for agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInput {
    /// Type of input (e.g., "code_request", "documentation", "analysis")
    pub input_type: String,
    
    /// Primary content/data for the agent to process
    pub content: String,
    
    /// Additional parameters and configuration
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Context from previous agent executions
    pub previous_outputs: Vec<AgentOutput>,
    
    /// User preferences and requirements
    pub user_preferences: HashMap<String, serde_json::Value>,
    
    /// Session identifier for tracking
    pub session_id: String,
    
    /// Timestamp of input creation
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Output data structure from agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    /// Agent that produced this output
    pub agent_id: String,
    
    /// Type of output produced
    pub output_type: String,
    
    /// Primary result content
    pub content: String,
    
    /// Structured data results
    pub data: HashMap<String, serde_json::Value>,
    
    /// Agent's confidence in this output (0.0 to 1.0)
    pub confidence: f32,
    
    /// Reasoning or explanation for the output
    pub reasoning: Option<String>,
    
    /// Suggested next actions or agents to invoke
    pub next_actions: Vec<String>,
    
    /// Metadata about the execution
    pub execution_metadata: ExecutionMetadata,
    
    /// Timestamp of output creation
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Metadata about agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Time taken to execute (in milliseconds)
    pub execution_time_ms: u64,
    
    /// Memory usage during execution
    pub memory_usage_mb: f64,
    
    /// Number of external API calls made
    pub api_calls: u32,
    
    /// Success/failure status
    pub status: ExecutionStatus,
    
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

/// Status of agent execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Success,
    PartialSuccess,
    Failed,
    Timeout,
    Cancelled,
}

/// Shared context for agent execution
pub struct CognitiveContext {
    /// Access to meta-memory system
    pub meta_memory: Arc<dyn MetaMemoryRepository>,
    
    /// Conversation service for RAG and context
    pub conversation_service: Arc<dyn ConversationService>,
    
    /// Current project state and file system context
    pub project_context: ProjectContext,
    
    /// User's cognitive preference profile
    pub cognitive_profile: CognitivePreferenceProfile,
    
    /// Session tracking and agent interaction history
    pub session_history: Vec<AgentOutput>,
    
    /// Global configuration and settings
    pub config: HashMap<String, serde_json::Value>,
    
    /// Current working directory
    pub working_directory: std::path::PathBuf,
}

/// Project context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    /// Project name and metadata
    pub project_name: String,
    pub project_version: String,
    pub project_description: Option<String>,
    
    /// Technology stack and frameworks
    pub tech_stack: Vec<String>,
    
    /// Current git branch and commit
    pub git_branch: Option<String>,
    pub git_commit: Option<String>,
    
    /// Active files and recent changes
    pub active_files: Vec<String>,
    pub recent_changes: Vec<String>,
    
    /// Project structure and important directories
    pub directory_structure: HashMap<String, Vec<String>>,
}

/// User's cognitive preference profile (CPP)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitivePreferenceProfile {
    /// User's preferred interaction mode
    pub interaction_mode: InteractionMode,
    
    /// Preferred level of detail in responses
    pub detail_level: DetailLevel,
    
    /// Emotional sensitivity settings
    pub emotional_sensitivity: EmotionalSensitivity,
    
    /// Decision autonomy preferences
    pub autonomy_level: AutonomyLevel,
    
    /// Communication style preferences
    pub communication_style: CommunicationStyle,
    
    /// Cognitive load management settings
    pub cognitive_load_settings: CognitiveLoadSettings,
}

/// Agent's behavioral preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitivePreferences {
    /// Preferred verbosity level
    pub verbosity: VerbosityLevel,
    
    /// Risk tolerance for autonomous actions
    pub risk_tolerance: f32,
    
    /// Preference for collaboration vs independence
    pub collaboration_preference: f32,
    
    /// Learning and adaptation settings
    pub learning_enabled: bool,
    pub adaptation_rate: f32,
    
    /// Creativity level for problem solving (0.0 to 1.0)
    pub creativity_level: f32,
    
    /// Preferred level of detail in analysis (0.0 to 1.0)
    pub detail_level: f32,
    
    /// Collaboration style preference
    pub collaboration_style: String,
}

// Enums for cognitive preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionMode {
    Focused,
    Collaborative,
    Exploratory,
    Autonomous,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DetailLevel {
    Minimal,
    Standard,
    Detailed,
    Comprehensive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EmotionalSensitivity {
    Low,
    Medium,
    High,
    Adaptive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutonomyLevel {
    Manual,        // User confirms every action
    ConfirmFirst,  // Ask before major actions
    SemiAuto,      // Proceed with minor actions, confirm major ones
    FullAuto,      // Proceed autonomously within confidence thresholds
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Technical,
    Adaptive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerbosityLevel {
    Minimal,
    Standard,
    Detailed,
    Verbose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveLoadSettings {
    /// Maximum number of items to present at once
    pub max_items_per_chunk: usize,
    
    /// Preferred pacing for information delivery
    pub pacing_preference: PacingPreference,
    
    /// Enable progressive disclosure
    pub progressive_disclosure: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PacingPreference {
    Fast,
    Medium,
    Slow,
    Adaptive,
}

impl Default for CognitivePreferenceProfile {
    fn default() -> Self {
        Self {
            interaction_mode: InteractionMode::Collaborative,
            detail_level: DetailLevel::Standard,
            emotional_sensitivity: EmotionalSensitivity::Medium,
            autonomy_level: AutonomyLevel::ConfirmFirst,
            communication_style: CommunicationStyle::Adaptive,
            cognitive_load_settings: CognitiveLoadSettings {
                max_items_per_chunk: 5,
                pacing_preference: PacingPreference::Medium,
                progressive_disclosure: true,
            },
        }
    }
}

impl Default for CognitivePreferences {
    fn default() -> Self {
        Self {
            verbosity: VerbosityLevel::Standard,
            risk_tolerance: 0.7,
            collaboration_preference: 0.8,
            learning_enabled: true,
            adaptation_rate: 0.1,
            creativity_level: 0.5,
            detail_level: 0.7,
            collaboration_style: "adaptive".to_string(),
        }
    }
}

impl AgentInput {
    /// Create a new agent input with minimal required fields
    pub fn new(input_type: String, content: String, session_id: String) -> Self {
        Self {
            input_type,
            content,
            parameters: HashMap::new(),
            previous_outputs: Vec::new(),
            user_preferences: HashMap::new(),
            session_id,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Add a parameter to the input
    pub fn with_parameter(mut self, key: String, value: serde_json::Value) -> Self {
        self.parameters.insert(key, value);
        self
    }
    
    /// Add previous agent outputs for context
    pub fn with_previous_outputs(mut self, outputs: Vec<AgentOutput>) -> Self {
        self.previous_outputs = outputs;
        self
    }
}

impl AgentOutput {
    /// Create a new agent output with minimal required fields
    pub fn new(
        agent_id: String, 
        output_type: String, 
        content: String, 
        confidence: f32
    ) -> Self {
        Self {
            agent_id,
            output_type,
            content,
            data: HashMap::new(),
            confidence,
            reasoning: None,
            next_actions: Vec::new(),
            execution_metadata: ExecutionMetadata {
                execution_time_ms: 0,
                memory_usage_mb: 0.0,
                api_calls: 0,
                status: ExecutionStatus::Success,
                warnings: Vec::new(),
            },
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Add structured data to the output
    pub fn with_data(mut self, key: String, value: serde_json::Value) -> Self {
        self.data.insert(key, value);
        self
    }
    
    /// Add reasoning explanation
    pub fn with_reasoning(mut self, reasoning: String) -> Self {
        self.reasoning = Some(reasoning);
        self
    }
    
    /// Add suggested next actions
    pub fn with_next_actions(mut self, actions: Vec<String>) -> Self {
        self.next_actions = actions;
        self
    }
} 