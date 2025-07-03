//! Agent API Module - REST endpoints for Brain AI Agent System
//!
//! This module provides comprehensive REST API endpoints for:
//! - Individual agent execution
//! - Agent status monitoring
//! - Cognitive Preference Profile (CPP) management
//! - DAG workflow orchestration
//! - Real-time agent communication

use brain_cognitive::{
    // Core agent types
    agents::{
        traits::{AgentInput, CognitiveContext},
        registry::AgentRegistry,
    },
    // Orchestration types
    orchestrator::AgentOrchestrator,
    // Evolution types for performance monitoring (simplified for now)
    evolution::{AgentPerformanceMonitor, EvolutionConfig},
    // Meta memory for cognitive context
    meta::MetaMemoryRepository,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use brain_types::error::BrainError;
use async_trait::async_trait;

// Type alias for Result with BrainError for convenience
type Result<T> = std::result::Result<T, BrainError>;

// ============================================================================
// PLACEHOLDER IMPLEMENTATIONS FOR DEMO PURPOSES
// ============================================================================

/// Simple placeholder implementation for MetaMemoryRepository
/// TODO: Replace with proper implementation
struct SimplePlaceholderMetaMemoryRepository;

impl SimplePlaceholderMetaMemoryRepository {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl brain_cognitive::meta::MetaMemoryRepository for SimplePlaceholderMetaMemoryRepository {
    async fn store_item(&mut self, _item: brain_cognitive::meta::MetaMemoryItem) -> brain_cognitive::meta::MetaMemoryResult<Uuid> {
        Ok(Uuid::new_v4())
    }
    
    async fn get_item(&self, _id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<Option<brain_cognitive::meta::MetaMemoryItem>> {
        Ok(None)
    }
    
    async fn get_item_by_component(&self, _component_id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<Option<brain_cognitive::meta::MetaMemoryItem>> {
        Ok(None)
    }
    
    async fn query_items(&self, _query: &brain_cognitive::meta::MetaMemoryQuery) -> brain_cognitive::meta::MetaMemoryResult<Vec<brain_cognitive::meta::MetaMemoryItem>> {
        Ok(Vec::new())
    }
    
    async fn remove_item(&mut self, _id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<bool> {
        Ok(true)
    }
    
    async fn batch_update(&mut self, _items: Vec<brain_cognitive::meta::MetaMemoryItem>) -> brain_cognitive::meta::MetaMemoryResult<Vec<Uuid>> {
        Ok(Vec::new())
    }
    
    async fn count_items(&self) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        Ok(0)
    }
    
    async fn clear_all(&mut self) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        Ok(0)
    }
}

/// Simple placeholder implementation for ConversationService
/// TODO: Replace with proper implementation
struct SimplePlaceholderConversationService;

impl SimplePlaceholderConversationService {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl brain_cognitive::conversation::ConversationService for SimplePlaceholderConversationService {
    async fn process_conversation(
        &mut self,
        _request: brain_cognitive::conversation::RagRequest,
        _memory_repo: &mut dyn brain_core::memory::WorkingMemoryRepository,
        _concept_repo: &mut dyn brain_core::concepts::ConceptRepository,
        _insight_repo: &mut dyn brain_core::insights::InsightRepository,
    ) -> std::result::Result<brain_cognitive::conversation::RagResponse, BrainError> {
        Ok(brain_cognitive::conversation::RagResponse {
            response: "Placeholder response".to_string(),
            conversation_id: "placeholder".to_string(),
            context_used: Vec::new(),
            confidence_score: 0.5,
            response_quality: Default::default(),
        })
    }
    
    fn get_conversation_stats(&self) -> HashMap<String, usize> {
        HashMap::new()
    }
    
    fn clear_conversation(&mut self, _conversation_id: &str) -> bool {
        true
    }
}

// ============================================================================
// Request/Response Structures
// ============================================================================

/// Request to execute a single agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecutionRequest {
    /// The input content for the agent to process
    pub input: String,
    /// Type of input being provided (e.g., "code_request", "analysis")
    pub input_type: String,
    /// Optional execution context and metadata
    pub context: Option<ExecutionContext>,
    /// Priority level for execution (1-10, higher = more priority)
    pub priority: Option<u8>,
    /// Maximum execution time in seconds
    pub timeout_seconds: Option<u64>,
    /// Additional parameters for the agent
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Response from agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecutionResponse {
    /// Unique execution ID for tracking
    pub execution_id: String,
    /// Whether the execution was successful
    pub success: bool,
    /// The agent's output content
    pub content: String,
    /// Structured data from the agent
    pub data: HashMap<String, serde_json::Value>,
    /// Agent's confidence in the result (0.0 to 1.0)
    pub confidence: f32,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Timestamp when execution started
    pub started_at: DateTime<Utc>,
    /// Timestamp when execution completed
    pub completed_at: DateTime<Utc>,
    /// Any error message if execution failed
    pub error: Option<String>,
    /// Agent's reasoning or explanation
    pub reasoning: Option<String>,
    /// Suggested next actions
    pub next_actions: Vec<String>,
    /// Resource usage information
    pub resource_usage: Option<ResourceUsage>,
}

/// Context information for agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// User ID for personalization
    pub user_id: Option<String>,
    /// Session ID for tracking
    pub session_id: String,
    /// Project context information
    pub project_context: Option<ProjectContext>,
    /// Previous agent outputs for chaining
    pub previous_outputs: Vec<AgentExecutionResponse>,
    /// User preferences
    pub user_preferences: Option<HashMap<String, serde_json::Value>>,
}

/// Project context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    /// Project name
    pub name: String,
    /// Project version or description
    pub version: Option<String>,
    /// Technology stack
    pub tech_stack: Vec<String>,
    /// Active files in the project
    pub active_files: Vec<String>,
    /// Recent changes or context
    pub recent_changes: Vec<String>,
}

/// Resource usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Memory usage in MB
    pub memory_mb: f64,
    /// CPU time used
    pub cpu_time_ms: u64,
    /// Number of API calls made
    pub api_calls: u32,
    /// Estimated cost (if applicable)
    pub estimated_cost: Option<f64>,
}

/// Request to execute a workflow of multiple agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionRequest {
    /// List of agents to execute with their inputs
    pub agents: Vec<WorkflowAgent>,
    /// Global context for all agents in the workflow
    pub context: Option<ExecutionContext>,
    /// Execution strategy (sequential, parallel, dag)
    pub execution_strategy: WorkflowExecutionStrategy,
    /// Maximum total execution time in seconds
    pub timeout_seconds: Option<u64>,
    /// Whether to stop on first error or continue
    pub continue_on_error: bool,
}

/// Agent definition within a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAgent {
    /// Name/ID of the agent to execute
    pub agent_name: String,
    /// Input for this specific agent
    pub input: String,
    /// Input type for this agent
    pub input_type: String,
    /// Dependencies on other agents in the workflow
    pub dependencies: Vec<String>,
    /// Priority within the workflow
    pub priority: Option<u8>,
    /// Agent-specific parameters
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Workflow execution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowExecutionStrategy {
    /// Execute agents one by one in order
    Sequential,
    /// Execute all agents simultaneously
    Parallel,
    /// Execute based on dependency graph (DAG)
    DAG,
}

/// Response from workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionResponse {
    /// Unique workflow execution ID
    pub workflow_id: String,
    /// Overall success status
    pub success: bool,
    /// Results from individual agents
    pub agent_results: Vec<AgentExecutionResponse>,
    /// Total execution time
    pub total_execution_time_ms: u64,
    /// Workflow started timestamp
    pub started_at: DateTime<Utc>,
    /// Workflow completed timestamp
    pub completed_at: DateTime<Utc>,
    /// Any workflow-level errors
    pub workflow_errors: Vec<String>,
    /// Summary of resource usage across all agents
    pub total_resource_usage: ResourceUsage,
}

/// Agent information in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// Agent's unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Agent's description and capabilities
    pub description: String,
    /// Agent's persona
    pub persona: String,
    /// Version of the agent
    pub version: String,
    /// Categories this agent belongs to
    pub categories: Vec<String>,
    /// Supported input types
    pub supported_input_types: Vec<String>,
    /// Supported output types
    pub supported_output_types: Vec<String>,
    /// Agent's capabilities
    pub capabilities: Vec<String>,
    /// Base confidence level
    pub base_confidence: f32,
    /// Current availability status
    pub status: AgentStatus,
    /// Performance metrics
    pub performance_metrics: Option<AgentPerformanceInfo>,
}

/// Agent status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Available,
    Busy,
    Unavailable,
    Error,
}

/// Agent performance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformanceInfo {
    /// Average execution time in milliseconds
    pub avg_execution_time_ms: f64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Average confidence score
    pub avg_confidence: f64,
    /// Total number of executions
    pub total_executions: u64,
    /// Last execution timestamp
    pub last_execution: Option<DateTime<Utc>>,
}

/// Response containing list of available agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentListResponse {
    /// List of available agents
    pub agents: Vec<AgentInfo>,
    /// Total number of agents
    pub total_count: usize,
    /// Agents grouped by category
    pub categories: HashMap<String, Vec<String>>,
    /// System status information
    pub system_status: SystemStatus,
}

/// System status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// Overall system health
    pub health: SystemHealth,
    /// Number of active executions
    pub active_executions: usize,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Memory usage information
    pub memory_usage: SystemMemoryUsage,
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// System memory usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemoryUsage {
    /// Used memory in MB
    pub used_mb: f64,
    /// Total available memory in MB
    pub total_mb: f64,
    /// Memory usage percentage
    pub usage_percent: f64,
}

/// Agent status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatusResponse {
    /// Agent information
    pub agent_info: AgentInfo,
    /// Current execution status
    pub execution_status: AgentExecutionStatus,
    /// Performance metrics
    pub performance_metrics: AgentPerformanceInfo,
    /// Resource usage
    pub resource_usage: ResourceUsage,
    /// Health check results
    pub health_check: AgentHealthCheck,
}

/// Agent execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecutionStatus {
    /// Current status
    pub status: AgentStatus,
    /// Number of active executions
    pub active_executions: usize,
    /// Queue length
    pub queue_length: usize,
    /// Last activity timestamp
    pub last_activity: Option<DateTime<Utc>>,
}

/// Agent health check results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHealthCheck {
    /// Overall health status
    pub status: SystemHealth,
    /// Health check timestamp
    pub checked_at: DateTime<Utc>,
    /// Specific health checks
    pub checks: Vec<HealthCheckResult>,
}

/// Individual health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Name of the health check
    pub name: String,
    /// Check result status
    pub status: SystemHealth,
    /// Additional details
    pub message: Option<String>,
    /// Check duration in milliseconds
    pub duration_ms: u64,
}

// CPP (Cognitive Preference Profile) Management Structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileListResponse {
    pub profiles: Vec<ProfileInfo>,
    pub total_count: usize,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileRequest {
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
    pub preferences: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileResponse {
    pub profile_id: String,
    pub success: bool,
    pub message: String,
}

// ============================================================================
// CORE AGENT API MANAGER
// ============================================================================

/// Main manager for agent API operations
pub struct AgentApiManager {
    /// Registry of all available agents
    agent_registry: Arc<Mutex<AgentRegistry>>,
    /// Agent orchestrator for workflow execution
    #[allow(dead_code)]
    orchestrator: Arc<AgentOrchestrator>,
    /// Performance monitoring
    #[allow(dead_code)]
    performance_monitor: Arc<Mutex<AgentPerformanceMonitor>>,
    /// Active execution tracking
    active_executions: Arc<Mutex<HashMap<String, ExecutionContext>>>,
    /// System start time for uptime calculation
    system_start_time: DateTime<Utc>,
}

impl AgentApiManager {
    /// Create a new AgentApiManager
    pub async fn new() -> Result<Self> {
        let agent_registry = Arc::new(Mutex::new(AgentRegistry::new()));
        
        // Load all 37 cognitive agents into the registry
        Self::load_all_agents(&agent_registry).await?;
        
        // Create orchestrator (no arguments needed)
        let orchestrator = Arc::new(AgentOrchestrator::new());
        
        // Create performance monitor with basic config
        let evolution_config = EvolutionConfig::default();
        // For now, we'll create a simple placeholder repository
        // TODO: Replace with proper repository implementation
        let memory_repo: Arc<dyn MetaMemoryRepository> = Arc::new(
            SimplePlaceholderMetaMemoryRepository::new()
        );
        let performance_monitor = Arc::new(Mutex::new(
            AgentPerformanceMonitor::new(evolution_config, memory_repo)?
        ));
        
        let active_executions = Arc::new(Mutex::new(HashMap::new()));
        
        Ok(Self {
            agent_registry,
            orchestrator,
            performance_monitor,
            active_executions,
            system_start_time: Utc::now(),
        })
    }

    /// Load all 37 cognitive agents into the registry
    async fn load_all_agents(registry: &Arc<Mutex<AgentRegistry>>) -> Result<()> {
        let registry_guard = registry.lock().await;
        
        // Load Development Agents (11 agents)
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::PlannerAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::ArchitectAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::DesignerAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::SchemaAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::APIAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::FrontendCoder::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::BackendCoder::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::RefactorAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::DocAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::DeployerAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::development::MaintainerAgent::new()))?;
        
        // Load Security Agents (5 agents)
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::security::CyberSecurityAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::security::PromptSecurityAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::security::PrivacyComplianceAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::security::DataPrivacyAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::security::EthicalAIAgent::new()))?;
        
        // Load Testing & Operations Agents (8 agents)
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::testing::QAAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::testing::SandboxEnvironmentAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::ops::ObservabilityAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::ops::BuildOptimizerAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::ops::DriftDetectionAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::ops::HotfixAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::ops::BackupRecoveryAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::ops::ReplicationScalingAgent::new()))?;
        
        // Load Intelligence & Platform Agents (13 agents)
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::intelligence::UserBehaviorAnalystAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::intelligence::FeatureExperimentationAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::intelligence::MLOpsAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::intelligence::ModelTrainingAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::intelligence::DataIngestionAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::LocalizationAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::PlatformCompatibilityAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::DataVisualizationAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::ApiGatewayAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::ServiceMeshAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::ContainerOrchestrationAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::InfrastructureProvisioningAgent::new()))?;
        registry_guard.register_agent(Arc::new(brain_cognitive::agents::platform::SystemOrchestrationAgent::new()))?;
        
        println!("✅ Successfully loaded all 37 agents into registry:");
        println!("   • 11 Development Agents");
        println!("   • 5 Security Agents");
        println!("   • 8 Testing & Operations Agents");
        println!("   • 13 Intelligence & Platform Agents");
        
        Ok(())
    }

    /// List all available agents with their metadata and performance metrics
    pub async fn list_agents(&self) -> Result<AgentListResponse> {
        let registry = self.agent_registry.lock().await;
        let all_agents = registry.list_agents()?;
        
        let mut agents = Vec::new();
        let mut categories: HashMap<String, Vec<String>> = HashMap::new();
        
        // Convert agent metadata to API format
        for agent in all_agents {
            let metadata = agent.metadata();
            
            // Create performance info (simplified for now)
            let performance_metrics = Some(AgentPerformanceInfo {
                avg_execution_time_ms: 150.0, // Default values
                success_rate: 0.95,
                avg_confidence: metadata.base_confidence as f64,
                total_executions: 0,
                last_execution: None,
            });
            
            let agent_info = AgentInfo {
                id: metadata.id.clone(),
                name: metadata.name.clone(),
                description: metadata.description.clone(),
                persona: metadata.persona.clone(),
                version: metadata.version.clone(),
                categories: metadata.tags.clone(), // Using tags as categories
                supported_input_types: metadata.supported_input_types.clone(),
                supported_output_types: metadata.supported_output_types.clone(),
                capabilities: metadata.capabilities.clone(),
                base_confidence: metadata.base_confidence,
                status: AgentStatus::Available,
                performance_metrics,
            };
            
            // Group by categories (using first tag as primary category)
            if let Some(primary_category) = metadata.tags.first() {
                categories
                    .entry(primary_category.clone())
                    .or_insert_with(Vec::new)
                    .push(metadata.id.clone());
            }
            
            agents.push(agent_info);
        }
        
        // Get system status
        let active_executions = self.active_executions.lock().await;
        let system_status = SystemStatus {
            health: SystemHealth::Healthy,
            active_executions: active_executions.len(),
            uptime_seconds: (Utc::now() - self.system_start_time).num_seconds() as u64,
            memory_usage: SystemMemoryUsage {
                used_mb: 256.0, // Placeholder values
                total_mb: 1024.0,
                usage_percent: 25.0,
            },
        };
        
        Ok(AgentListResponse {
            total_count: agents.len(),
            agents,
            categories,
            system_status,
        })
    }

    /// Execute a single agent
    pub async fn execute_agent(
        &self,
        agent_name: &str,
        request: AgentExecutionRequest,
    ) -> Result<AgentExecutionResponse> {
        let execution_id = Uuid::new_v4().to_string();
        let started_at = Utc::now();

        // Store execution context
        if let Some(context) = &request.context {
            let mut executions = self.active_executions.lock().await;
            executions.insert(execution_id.clone(), context.clone());
        }

        // Get agent from registry
        let registry = self.agent_registry.lock().await;
        let agent = registry.get_agent(agent_name)?
            .ok_or_else(|| BrainError::NotFound(format!("Agent '{}' not found", agent_name)))?;

        let _metadata = agent.metadata().clone();
        drop(registry); // Release lock early

        // Create cognitive context (simplified)
        let cognitive_context = self.create_cognitive_context(&request.context).await?;

        // Prepare agent input with proper constructor
        let session_id = request.context
            .as_ref()
            .map(|c| c.session_id.clone())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
            
        let agent_input = AgentInput::new(
            request.input_type.clone(),
            request.input.clone(),
            session_id,
        );

        // Execute agent
        let start_time = std::time::Instant::now();
        let result = agent.execute(agent_input, &cognitive_context).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        let completed_at = Utc::now();

        // Clean up execution context
        {
            let mut executions = self.active_executions.lock().await;
            executions.remove(&execution_id);
        }

        // Build response based on execution result
        match result {
            Ok(output) => {
                Ok(AgentExecutionResponse {
                    execution_id,
                    success: true,
                    content: output.content,
                    data: output.data,
                    confidence: output.confidence,
                    execution_time_ms,
                    started_at,
                    completed_at,
                    error: None,
                    reasoning: output.reasoning,
                    next_actions: output.next_actions,
                    resource_usage: Some(ResourceUsage {
                        memory_mb: output.execution_metadata.memory_usage_mb,
                        cpu_time_ms: output.execution_metadata.execution_time_ms,
                        api_calls: output.execution_metadata.api_calls,
                        estimated_cost: None,
                    }),
                })
            }
            Err(error) => {
                Ok(AgentExecutionResponse {
                    execution_id,
                    success: false,
                    content: String::new(),
                    data: HashMap::new(),
                    confidence: 0.0,
                    execution_time_ms,
                    started_at,
                    completed_at,
                    error: Some(format!("{}", error)),
                    reasoning: None,
                    next_actions: Vec::new(),
                    resource_usage: Some(ResourceUsage {
                        memory_mb: 0.0,
                        cpu_time_ms: execution_time_ms,
                        api_calls: 0,
                        estimated_cost: None,
                    }),
                })
            }
        }
    }

    /// Get status information for a specific agent
    pub async fn get_agent_status(&self, agent_name: &str) -> Result<AgentStatusResponse> {
        let registry = self.agent_registry.lock().await;
        let agent = registry.get_agent(agent_name)?
            .ok_or_else(|| BrainError::NotFound(format!("Agent '{}' not found", agent_name)))?;

        let metadata = agent.metadata();
        
        // Create agent info
        let agent_info = AgentInfo {
            id: metadata.id.clone(),
            name: metadata.name.clone(),
            description: metadata.description.clone(),
            persona: metadata.persona.clone(),
            version: metadata.version.clone(),
            categories: metadata.tags.clone(),
            supported_input_types: metadata.supported_input_types.clone(),
            supported_output_types: metadata.supported_output_types.clone(),
            capabilities: metadata.capabilities.clone(),
            base_confidence: metadata.base_confidence,
            status: AgentStatus::Available,
            performance_metrics: None,
        };

        // Create performance metrics (simplified)
        let performance_metrics = AgentPerformanceInfo {
            avg_execution_time_ms: 150.0,
            success_rate: 0.95,
            avg_confidence: metadata.base_confidence as f64,
            total_executions: 0,
            last_execution: None,
        };

        // Create execution status
        let execution_status = AgentExecutionStatus {
            status: AgentStatus::Available,
            active_executions: 0,
            queue_length: 0,
            last_activity: None,
        };

        // Create resource usage
        let resource_usage = ResourceUsage {
            memory_mb: 10.0,
            cpu_time_ms: 0,
            api_calls: 0,
            estimated_cost: None,
        };

        // Create health check
        let health_check = AgentHealthCheck {
            status: SystemHealth::Healthy,
            checked_at: Utc::now(),
            checks: vec![
                HealthCheckResult {
                    name: "Agent Availability".to_string(),
                    status: SystemHealth::Healthy,
                    message: Some("Agent is ready for execution".to_string()),
                    duration_ms: 1,
                }
            ],
        };

        Ok(AgentStatusResponse {
            agent_info,
            execution_status,
            performance_metrics,
            resource_usage,
            health_check,
        })
    }

    /// Execute a workflow of multiple agents
    pub async fn execute_workflow(
        &self,
        request: WorkflowExecutionRequest,
    ) -> Result<WorkflowExecutionResponse> {
        let workflow_id = Uuid::new_v4().to_string();
        let started_at = Utc::now();
        
        let mut agent_results = Vec::new();
        let mut workflow_errors = Vec::new();
        let mut total_resource_usage = ResourceUsage {
            memory_mb: 0.0,
            cpu_time_ms: 0,
            api_calls: 0,
            estimated_cost: None,
        };

        // For now, implement simple sequential execution
        // TODO: Implement proper DAG-based execution
        for workflow_agent in request.agents {
            let execution_request = AgentExecutionRequest {
                input: workflow_agent.input,
                input_type: workflow_agent.input_type,
                context: request.context.clone(),
                priority: workflow_agent.priority,
                timeout_seconds: request.timeout_seconds,
                parameters: workflow_agent.parameters,
            };

            match self.execute_agent(&workflow_agent.agent_name, execution_request).await {
                Ok(result) => {
                    // Accumulate resource usage
                    if let Some(usage) = &result.resource_usage {
                        total_resource_usage.memory_mb += usage.memory_mb;
                        total_resource_usage.cpu_time_ms += usage.cpu_time_ms;
                        total_resource_usage.api_calls += usage.api_calls;
                    }
                    agent_results.push(result);
                }
                Err(error) => {
                    workflow_errors.push(format!("Agent '{}' failed: {}", workflow_agent.agent_name, error));
                    if !request.continue_on_error {
                        break;
                    }
                }
            }
        }

        let completed_at = Utc::now();
        let total_execution_time_ms = (completed_at - started_at).num_milliseconds() as u64;

        Ok(WorkflowExecutionResponse {
            workflow_id,
            success: workflow_errors.is_empty(),
            agent_results,
            total_execution_time_ms,
            started_at,
            completed_at,
            workflow_errors,
            total_resource_usage,
        })
    }

    /// Create a basic cognitive context for agent execution
    async fn create_cognitive_context(
        &self,
        execution_context: &Option<ExecutionContext>,
    ) -> Result<CognitiveContext> {
        // For now, create a minimal cognitive context
        // In a full implementation, this would be much more sophisticated
        
        let meta_memory: Arc<dyn MetaMemoryRepository> = Arc::new(
            SimplePlaceholderMetaMemoryRepository::new()
        );
        
        let conversation_service = Arc::new(
            SimplePlaceholderConversationService::new()
        );
        
        // Create project context
        let project_context = if let Some(exec_ctx) = execution_context {
            if let Some(proj_ctx) = &exec_ctx.project_context {
                brain_cognitive::agents::traits::ProjectContext {
                    project_name: proj_ctx.name.clone(),
                    project_version: proj_ctx.version.clone().unwrap_or_default(),
                    project_description: None,
                    tech_stack: proj_ctx.tech_stack.clone(),
                    git_branch: None,
                    git_commit: None,
                    active_files: proj_ctx.active_files.clone(),
                    recent_changes: proj_ctx.recent_changes.clone(),
                    directory_structure: HashMap::new(),
                }
            } else {
                brain_cognitive::agents::traits::ProjectContext {
                    project_name: "default".to_string(),
                    project_version: "1.0.0".to_string(),
                    project_description: None,
                    tech_stack: vec![],
                    git_branch: None,
                    git_commit: None,
                    active_files: vec![],
                    recent_changes: vec![],
                    directory_structure: HashMap::new(),
                }
            }
        } else {
            brain_cognitive::agents::traits::ProjectContext {
                project_name: "default".to_string(),
                project_version: "1.0.0".to_string(),
                project_description: None,
                tech_stack: vec![],
                git_branch: None,
                git_commit: None,
                active_files: vec![],
                recent_changes: vec![],
                directory_structure: HashMap::new(),
            }
        };
        
        Ok(CognitiveContext {
            meta_memory,
            conversation_service,
            project_context,
            cognitive_profile: brain_cognitive::agents::traits::CognitivePreferenceProfile::default(),
            session_history: vec![],
            config: HashMap::new(),
            working_directory: std::env::current_dir().unwrap_or_default(),
        })
    }

    // ============================================================================
    // CPP (Cognitive Preference Profile) Management Methods
    // ============================================================================

    /// List all cognitive preference profiles for a user
    pub async fn list_profiles(&self, user_id: &str) -> Result<ProfileListResponse> {
        // For now, return a basic response
        // In a full implementation, this would query the actual profile manager
        Ok(ProfileListResponse {
            profiles: vec![],
            total_count: 0,
            user_id: user_id.to_string(),
        })
    }

    /// Create a new cognitive preference profile
    pub async fn create_profile(&self, _request: CreateProfileRequest) -> Result<CreateProfileResponse> {
        // For now, return a success response with a generated ID
        // In a full implementation, this would create the actual profile
        let profile_id = Uuid::new_v4().to_string();
        
        Ok(CreateProfileResponse {
            profile_id,
            success: true,
            message: "Profile created successfully".to_string(),
        })
    }
} 