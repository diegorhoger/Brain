//! Task Scheduler for Agent Orchestration

use serde::{Deserialize, Serialize};
use brain_types::error::BrainError;
use super::dag::{AgentDAG, ExecutionPlan, ExecutionOrder};
use super::OrchestrationConfig;

/// Task scheduler for managing agent execution order
#[derive(Debug)]
pub struct TaskScheduler {
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn create_execution_plan(
        &self,
        dag: &AgentDAG,
        _config: &OrchestrationConfig,
    ) -> Result<ExecutionPlan, BrainError> {
        dag.create_execution_plan(ExecutionOrder::Topological)
    }
}

/// Scheduling strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulingStrategy {
    Fifo,
    Priority,
    ShortestFirst,
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Schedule decision result
#[derive(Debug, Clone)]
pub struct ScheduleDecision {
    pub node_id: String,
    pub priority: TaskPriority,
    pub estimated_duration_ms: u64,
}

/// Resource constraints for scheduling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub max_memory_mb: u64,
    pub max_cpu_cores: u32,
    pub max_concurrent_tasks: usize,
}

impl Default for ResourceConstraints {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024,
            max_cpu_cores: 4,
            max_concurrent_tasks: 10,
        }
    }
}
