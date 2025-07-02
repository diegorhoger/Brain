//! Agent Orchestration System
//! 
//! This module provides the DAG execution engine for coordinating multiple agents
//! in complex workflows with dependency management and parallel execution.

pub mod dag;
pub mod executor;
pub mod scheduler;
pub mod memory;
pub mod communication;

// Re-export key types and traits
pub use dag::{
    AgentDAG, AgentNode, ExecutionPlan, DependencyGraph,
    DAGBuilder, DAGValidationError, ExecutionOrder
};

pub use executor::{
    DAGExecutor, ExecutionEngine, ExecutionContext,
    ExecutionResult, ExecutionMetrics, RetryPolicy
};

pub use scheduler::{
    TaskScheduler, SchedulingStrategy, TaskPriority,
    ScheduleDecision, ResourceConstraints
};

pub use memory::{
    OrchestratorMemory, AgentMemoryNamespace, MemoryRegistry,
    CrossAgentMemoryShare, MemoryAccessControl
};

pub use communication::{
    AgentCommunicationBus, MessageBus, AgentMessage,
    CommunicationProtocol, EventTrigger
};

use crate::agents::traits::{BrainAgent, CognitiveContext, AgentInput, AgentOutput, BrainResult};
use crate::agents::registry::{AgentRegistry, AgentQuery};
use brain_types::error::BrainError;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Add missing dependencies for integration
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Configuration for the agent orchestration system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationConfig {
    /// Maximum number of concurrent agent executions
    pub max_concurrent_agents: usize,
    
    /// Default timeout for agent execution (in seconds)
    pub default_timeout_seconds: u64,
    
    /// Maximum retry attempts for failed agents
    pub max_retry_attempts: u32,
    
    /// Enable cross-agent memory sharing
    pub enable_memory_sharing: bool,
    
    /// Enable real-time agent communication
    pub enable_agent_communication: bool,
    
    /// Memory cleanup interval (in seconds)
    pub memory_cleanup_interval_seconds: u64,
    
    /// Enable integration with existing WorkflowEngine
    pub enable_workflow_integration: bool,
    
    /// Enable agent registry integration
    pub enable_registry_integration: bool,
    
    /// Default confidence threshold for agent execution
    pub default_confidence_threshold: f32,
}

impl Default for OrchestrationConfig {
    fn default() -> Self {
        Self {
            max_concurrent_agents: 10,
            default_timeout_seconds: 300,
            max_retry_attempts: 3,
            enable_memory_sharing: true,
            enable_agent_communication: true,
            memory_cleanup_interval_seconds: 3600,
            enable_workflow_integration: true,
            enable_registry_integration: true,
            default_confidence_threshold: 0.7,
        }
    }
}

/// Main orchestrator that coordinates agent execution workflows
pub struct AgentOrchestrator {
    /// DAG execution engine
    executor: Arc<DAGExecutor>,
    
    /// Task scheduler
    scheduler: Arc<TaskScheduler>,
    
    /// Memory management system
    memory: Arc<OrchestratorMemory>,
    
    /// Communication bus for agent messaging
    communication: Arc<AgentCommunicationBus>,
    
    /// Integration with existing agent registry
    agent_registry: Option<Arc<AgentRegistry>>,
    
    /// Workflow integration adapter
    workflow_adapter: Option<Arc<WorkflowAdapter>>,
    
    /// Configuration settings
    config: OrchestrationConfig,
}

/// Adapter for integrating with existing WorkflowEngine
pub struct WorkflowAdapter {
    /// Reference to orchestrator for DAG execution
    orchestrator: Option<std::sync::Weak<AgentOrchestrator>>,
    
    /// Workflow conversion cache
    workflow_cache: std::sync::RwLock<HashMap<String, ConvertedWorkflow>>,
}

/// Workflow converted to DAG format
#[derive(Clone)]
pub struct ConvertedWorkflow {
    /// Original workflow ID
    pub workflow_id: String,
    
    /// Generated agents for workflow steps
    pub agents: Vec<Arc<dyn BrainAgent>>,
    
    /// Generated inputs for agents
    pub inputs: Vec<AgentInput>,
    
    /// Dependency mapping from workflow steps
    pub dependencies: HashMap<String, Vec<String>>,
    
    /// Conversion timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Enhanced workflow execution result with DAG capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedWorkflowResult {
    /// Original workflow ID
    pub workflow_id: String,
    
    /// Execution ID for tracking
    pub execution_id: String,
    
    /// DAG execution results
    pub agent_outputs: Vec<AgentOutput>,
    
    /// Execution metrics from DAG engine
    pub execution_metrics: ExecutionMetrics,
    
    /// Workflow-specific results
    pub workflow_status: WorkflowExecutionStatus,
    
    /// Detailed step results
    pub step_results: HashMap<String, WorkflowStepResult>,
    
    /// Overall execution time
    pub total_duration_ms: u64,
    
    /// Timestamp
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

/// Status of workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowExecutionStatus {
    Completed,
    PartiallyCompleted,
    Failed,
    Cancelled,
}

/// Result of individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepResult {
    /// Step ID from original workflow
    pub step_id: String,
    
    /// Corresponding agent output
    pub agent_output: Option<AgentOutput>,
    
    /// Step execution status
    pub status: StepExecutionStatus,
    
    /// Execution duration
    pub duration_ms: u64,
    
    /// Any errors encountered
    pub error: Option<String>,
}

/// Status of individual step execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepExecutionStatus {
    Completed,
    Failed,
    Skipped,
    TimedOut,
}

impl AgentOrchestrator {
    /// Create a new agent orchestrator with default configuration
    pub fn new() -> Self {
        Self::with_config(OrchestrationConfig::default())
    }
    
    /// Create a new agent orchestrator with custom configuration
    pub fn with_config(config: OrchestrationConfig) -> Self {
        let executor = Arc::new(DAGExecutor::new()
            .with_confidence_threshold(config.default_confidence_threshold));
        let scheduler = Arc::new(TaskScheduler::new());
        let memory = Arc::new(OrchestratorMemory::new());
        let communication = Arc::new(AgentCommunicationBus::new());
        
        Self {
            executor,
            scheduler,
            memory,
            communication,
            agent_registry: None,
            workflow_adapter: None,
            config,
        }
    }
    
    /// Integrate with existing agent registry
    pub fn with_agent_registry(mut self, registry: Arc<AgentRegistry>) -> Self {
        self.agent_registry = Some(registry);
        self
    }
    
    /// Enable workflow integration
    pub fn with_workflow_integration(mut self) -> Self {
        self.workflow_adapter = Some(Arc::new(WorkflowAdapter::new()));
        self
    }
    
    /// Execute a workflow using DAG orchestration (integration with WorkflowEngine)
    pub async fn execute_workflow_with_dag(
        &self,
        workflow_id: &str,
        workflow_steps: Vec<WorkflowStepDefinition>,
        context: &CognitiveContext,
    ) -> BrainResult<EnhancedWorkflowResult> {
        let execution_id = Uuid::new_v4().to_string();
        let start_time = std::time::Instant::now();
        
        // Convert workflow to DAG format
        let converted = self.convert_workflow_to_dag(workflow_id, workflow_steps).await?;
        
        // Execute using DAG engine
        let dag_result = self.execute_workflow(
            converted.agents.clone(),
            converted.inputs.clone(),
            context,
        ).await;
        
        let total_duration = start_time.elapsed();
        
        match dag_result {
            Ok(agent_outputs) => {
                // Convert DAG results back to workflow format
                let step_results = self.map_agent_outputs_to_steps(&agent_outputs, &converted).await?;
                let execution_metrics = self.executor.get_metrics().await?;
                
                Ok(EnhancedWorkflowResult {
                    workflow_id: workflow_id.to_string(),
                    execution_id,
                    agent_outputs,
                    execution_metrics,
                    workflow_status: WorkflowExecutionStatus::Completed,
                    step_results,
                    total_duration_ms: total_duration.as_millis() as u64,
                    completed_at: Utc::now(),
                })
            }
            Err(_e) => {
                // Handle partial completion
                let step_results = HashMap::new(); // TODO: Extract partial results
                let execution_metrics = self.executor.get_metrics().await.unwrap_or_default();
                
                Ok(EnhancedWorkflowResult {
                    workflow_id: workflow_id.to_string(),
                    execution_id,
                    agent_outputs: Vec::new(),
                    execution_metrics,
                    workflow_status: WorkflowExecutionStatus::Failed,
                    step_results,
                    total_duration_ms: total_duration.as_millis() as u64,
                    completed_at: Utc::now(),
                })
            }
        }
    }
    
    /// Discover and select agents using the registry
    pub async fn discover_agents_for_workflow(
        &self,
        requirements: &WorkflowRequirements,
    ) -> BrainResult<Vec<Arc<dyn BrainAgent>>> {
        if let Some(registry) = &self.agent_registry {
            let mut discovered_agents = Vec::new();
            
            for requirement in &requirements.agent_requirements {
                let query = AgentQuery::new()
                    .with_input_type(requirement.input_type.clone())
                    .with_capability(requirement.required_capability.clone())
                    .with_min_confidence(requirement.min_confidence);
                
                let agents = registry.discover_agents(&query)?;
                
                if agents.is_empty() {
                    return Err(BrainError::ExecutionError(
                        format!("No agents found for requirement: {:?}", requirement)
                    ));
                }
                
                // Select the best agent based on confidence and capabilities
                let best_agent = self.select_best_agent(&agents, requirement).await?;
                discovered_agents.push(best_agent);
            }
            
            Ok(discovered_agents)
        } else {
            Err(BrainError::ExecutionError(
                "Agent registry not integrated".to_string()
            ))
        }
    }
    
    /// Execute a workflow using agent discovery and DAG orchestration
    pub async fn execute_discovered_workflow(
        &self,
        requirements: &WorkflowRequirements,
        context: &CognitiveContext,
    ) -> BrainResult<Vec<AgentOutput>> {
        // Discover suitable agents
        let agents = self.discover_agents_for_workflow(requirements).await?;
        
        // Generate inputs for agents
        let inputs = self.generate_inputs_for_requirements(requirements).await?;
        
        // Execute using standard DAG workflow
        self.execute_workflow(agents, inputs, context).await
    }
    
    /// Execute a workflow defined by agent dependencies and inputs
    pub async fn execute_workflow(
        &self,
        agents: Vec<Arc<dyn BrainAgent>>,
        inputs: Vec<AgentInput>,
        context: &CognitiveContext,
    ) -> BrainResult<Vec<AgentOutput>> {
        // Build DAG from agent dependencies
        let mut dag = DAGBuilder::new()
            .with_agents(agents)
            .with_inputs(inputs)
            .build()?;
        
        // Validate DAG structure
        dag.validate().map_err(|e| BrainError::Other(format!("{:?}", e)))?;
        
        // Create execution plan
        let plan = self.scheduler.create_execution_plan(&dag, &self.config)?;
        
        // Execute the plan
        self.executor.execute_plan(plan, &mut dag, context).await
    }
    
    /// Convert workflow steps to DAG format
    async fn convert_workflow_to_dag(
        &self,
        workflow_id: &str,
        steps: Vec<WorkflowStepDefinition>,
    ) -> BrainResult<ConvertedWorkflow> {
        let mut agents = Vec::new();
        let mut inputs = Vec::new();
        let mut dependencies = HashMap::new();
        
        for step in steps {
            // Convert step to agent (this would typically involve agent registry lookup)
            if let Some(registry) = &self.agent_registry {
                let query = AgentQuery::new()
                    .with_input_type(step.input_type.clone())
                    .with_capability(step.required_capability.clone());
                
                let discovered_agents = registry.discover_agents(&query)?;
                
                if let Some(agent) = discovered_agents.first() {
                    agents.push(agent.clone());
                    
                    // Create input for the agent
                    let input = AgentInput::new(
                        step.input_type,
                        step.input_data,
                        format!("workflow_{}", workflow_id),
                    );
                    inputs.push(input);
                    
                    // Map dependencies
                    dependencies.insert(step.id.clone(), step.dependencies);
                }
            }
        }
        
        Ok(ConvertedWorkflow {
            workflow_id: workflow_id.to_string(),
            agents,
            inputs,
            dependencies,
            created_at: Utc::now(),
        })
    }
    
    /// Map agent outputs back to workflow step results
    async fn map_agent_outputs_to_steps(
        &self,
        outputs: &[AgentOutput],
        _converted: &ConvertedWorkflow,
    ) -> BrainResult<HashMap<String, WorkflowStepResult>> {
        let mut step_results = HashMap::new();
        
        for (index, output) in outputs.iter().enumerate() {
            let step_id = format!("step_{}", index); // This should map to actual step IDs
            
            let step_result = WorkflowStepResult {
                step_id: step_id.clone(),
                agent_output: Some(output.clone()),
                status: if output.confidence > 0.5 {
                    StepExecutionStatus::Completed
                } else {
                    StepExecutionStatus::Failed
                },
                duration_ms: output.execution_metadata.execution_time_ms,
                error: None,
            };
            
            step_results.insert(step_id, step_result);
        }
        
        Ok(step_results)
    }
    
    /// Select the best agent for a requirement
    async fn select_best_agent(
        &self,
        agents: &[Arc<dyn BrainAgent>],
        requirement: &AgentRequirement,
    ) -> BrainResult<Arc<dyn BrainAgent>> {
        // Simple selection based on confidence threshold
        for agent in agents {
            if agent.confidence_threshold() >= requirement.min_confidence {
                return Ok(agent.clone());
            }
        }
        
        // Fallback to first agent if none meet confidence requirement
        agents.first()
            .cloned()
            .ok_or_else(|| BrainError::ExecutionError("No suitable agent found".to_string()))
    }
    
    /// Generate inputs for workflow requirements
    async fn generate_inputs_for_requirements(
        &self,
        requirements: &WorkflowRequirements,
    ) -> BrainResult<Vec<AgentInput>> {
        let mut inputs = Vec::new();
        
        for requirement in &requirements.agent_requirements {
            let input = AgentInput::new(
                requirement.input_type.clone(),
                requirement.input_data.clone(),
                Uuid::new_v4().to_string(),
            );
            inputs.push(input);
        }
        
        Ok(inputs)
    }
    
    /// Get orchestrator statistics and metrics
    pub async fn get_metrics(&self) -> OrchestrationMetrics {
        OrchestrationMetrics {
            total_executions: self.executor.total_executions().await,
            successful_executions: self.executor.successful_executions().await,
            failed_executions: self.executor.failed_executions().await,
            average_execution_time_ms: self.executor.average_execution_time().await,
            active_agents: self.executor.active_agents().await,
            memory_usage_mb: self.memory.memory_usage_mb().await,
        }
    }
    
    /// Get access to the agent communication bus
    pub fn communication_bus(&self) -> &AgentCommunicationBus {
        &self.communication
    }
    
    /// Get the number of active communication channels
    pub async fn get_communication_channel_count(&self) -> usize {
        self.communication.get_channel_count().await
    }
    
    /// Get access to the integrated agent registry
    pub fn agent_registry(&self) -> Option<&AgentRegistry> {
        self.agent_registry.as_ref().map(|r| r.as_ref())
    }
    
    /// Get workflow adapter for advanced workflow integration
    pub fn workflow_adapter(&self) -> Option<&WorkflowAdapter> {
        self.workflow_adapter.as_ref().map(|w| w.as_ref())
    }
}

/// Requirements for workflow execution
#[derive(Debug, Clone)]
pub struct WorkflowRequirements {
    /// Individual agent requirements
    pub agent_requirements: Vec<AgentRequirement>,
    
    /// Overall workflow constraints
    pub constraints: WorkflowConstraints,
}

/// Requirements for a specific agent in the workflow
#[derive(Debug, Clone)]
pub struct AgentRequirement {
    /// Required input type
    pub input_type: String,
    
    /// Required capability
    pub required_capability: String,
    
    /// Input data for the agent
    pub input_data: String,
    
    /// Minimum confidence threshold
    pub min_confidence: f32,
    
    /// Dependencies on other agents (by index)
    pub dependencies: Vec<usize>,
}

/// Constraints for workflow execution
#[derive(Debug, Clone)]
pub struct WorkflowConstraints {
    /// Maximum execution time
    pub max_duration_seconds: u64,
    
    /// Required minimum confidence
    pub min_overall_confidence: f32,
    
    /// Allow partial completion
    pub allow_partial_completion: bool,
}

/// Definition of a workflow step for conversion
#[derive(Debug, Clone)]
pub struct WorkflowStepDefinition {
    /// Step identifier
    pub id: String,
    
    /// Required input type
    pub input_type: String,
    
    /// Required capability
    pub required_capability: String,
    
    /// Input data
    pub input_data: String,
    
    /// Dependencies on other steps
    pub dependencies: Vec<String>,
}

impl WorkflowAdapter {
    /// Create a new workflow adapter
    pub fn new() -> Self {
        Self {
            orchestrator: None,
            workflow_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }
    
    /// Set the orchestrator reference
    pub fn set_orchestrator(&mut self, orchestrator: std::sync::Weak<AgentOrchestrator>) {
        self.orchestrator = Some(orchestrator);
    }
    
    /// Convert a legacy workflow to enhanced DAG execution
    pub async fn execute_legacy_workflow(
        &self,
        workflow_id: &str,
        steps: Vec<WorkflowStepDefinition>,
        context: &CognitiveContext,
    ) -> BrainResult<EnhancedWorkflowResult> {
        if let Some(orchestrator_weak) = &self.orchestrator {
            if let Some(orchestrator) = orchestrator_weak.upgrade() {
                return orchestrator.execute_workflow_with_dag(workflow_id, steps, context).await;
            }
        }
        
        Err(BrainError::ExecutionError(
            "Orchestrator not available for workflow execution".to_string()
        ))
    }
}

/// Metrics and statistics for orchestration performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMetrics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_execution_time_ms: f64,
    pub active_agents: usize,
    pub memory_usage_mb: f64,
}

impl Default for AgentOrchestrator {
    fn default() -> Self {
        Self::new()
    }
} 