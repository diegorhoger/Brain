//! Directed Acyclic Graph (DAG) implementation for agent orchestration
//! 
//! This module provides the core data structures and algorithms for building
//! and validating agent execution graphs based on dependencies.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use brain_types::error::BrainError;
use crate::agents::traits::{BrainAgent, AgentInput, AgentOutput, BrainResult};

/// Directed Acyclic Graph representing agent execution workflow
pub struct AgentDAG {
    /// Graph nodes representing agents and their state
    pub nodes: HashMap<String, AgentNode>,
    
    /// Adjacency list representing dependencies (node_id -> dependencies)
    pub dependencies: HashMap<String, Vec<String>>,
    
    /// Reverse adjacency list (node_id -> dependents)
    pub dependents: HashMap<String, Vec<String>>,
    
    /// Root nodes with no dependencies
    pub roots: Vec<String>,
    
    /// Leaf nodes with no dependents
    pub leaves: Vec<String>,
}

/// Node in the agent DAG representing an agent and its execution state
pub struct AgentNode {
    /// Unique identifier for the node
    pub id: String,
    
    /// Reference to the agent implementation
    pub agent: Arc<dyn BrainAgent>,
    
    /// Input data for this agent
    pub input: AgentInput,
    
    /// Current execution state
    pub state: NodeState,
    
    /// Output from execution (if completed)
    pub output: Option<AgentOutput>,
    
    /// Error from execution (if failed)
    pub error: Option<BrainError>,
    
    /// Execution priority (higher values execute first)
    pub priority: i32,
    
    /// Estimated execution time in milliseconds
    pub estimated_duration_ms: u64,
}

/// State of a node in the execution DAG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeState {
    /// Waiting for dependencies to complete
    Pending,
    
    /// Ready to execute (dependencies satisfied)
    Ready,
    
    /// Currently executing
    Executing,
    
    /// Successfully completed
    Completed,
    
    /// Failed with error
    Failed,
    
    /// Cancelled before execution
    Cancelled,
    
    /// Skipped due to conditional logic
    Skipped,
}

/// Execution plan derived from DAG analysis
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    /// Ordered execution waves (can be executed in parallel within each wave)
    pub execution_waves: Vec<ExecutionWave>,
    
    /// Total estimated execution time
    pub estimated_total_duration_ms: u64,
    
    /// Maximum parallelism (nodes that can execute simultaneously)
    pub max_parallelism: usize,
    
    /// Critical path through the DAG
    pub critical_path: Vec<String>,
    
    /// Execution order for topological sort
    pub execution_order: ExecutionOrder,
}

/// A wave of agents that can be executed in parallel
#[derive(Debug, Clone)]
pub struct ExecutionWave {
    /// Node IDs that can execute in this wave
    pub node_ids: Vec<String>,
    
    /// Wave number (0-based)
    pub wave_number: usize,
    
    /// Estimated duration for this wave (longest task)
    pub estimated_duration_ms: u64,
}

/// Execution order strategies for the DAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionOrder {
    /// Topological sort (dependency order)
    Topological,
    
    /// Priority-based (highest priority first)
    Priority,
    
    /// Critical path first
    CriticalPath,
    
    /// Shortest duration first
    ShortestFirst,
    
    /// Resource-optimized order
    ResourceOptimized,
}

/// Dependency graph structure for analysis
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Forward edges (node -> its dependencies)
    pub forward_edges: HashMap<String, HashSet<String>>,
    
    /// Backward edges (node -> nodes that depend on it)
    pub backward_edges: HashMap<String, HashSet<String>>,
    
    /// In-degree count for each node
    pub in_degrees: HashMap<String, usize>,
    
    /// Out-degree count for each node
    pub out_degrees: HashMap<String, usize>,
}

/// Builder for constructing agent DAGs
pub struct DAGBuilder {
    agents: Vec<Arc<dyn BrainAgent>>,
    inputs: Vec<AgentInput>,
    explicit_dependencies: HashMap<String, Vec<String>>,
    priorities: HashMap<String, i32>,
    estimated_durations: HashMap<String, u64>,
}

/// Validation errors for DAG structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DAGValidationError {
    /// Circular dependency detected
    CyclicDependency { cycle: Vec<String> },
    
    /// Missing dependency reference
    MissingDependency { node: String, missing_dep: String },
    
    /// Duplicate node IDs
    DuplicateNode { node_id: String },
    
    /// Empty DAG
    EmptyDAG,
    
    /// Invalid input/output type mismatch
    TypeMismatch { from_node: String, to_node: String, expected: String, actual: String },
    
    /// Unreachable nodes
    UnreachableNodes { nodes: Vec<String> },
}

impl AgentDAG {
    /// Create a new empty DAG
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
            roots: Vec::new(),
            leaves: Vec::new(),
        }
    }
    
    /// Add a node to the DAG
    pub fn add_node(&mut self, node: AgentNode) -> BrainResult<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(BrainError::Other(
                format!("Node with ID '{}' already exists", node.id)
            ));
        }
        
        let node_id = node.id.clone();
        self.nodes.insert(node_id.clone(), node);
        self.dependencies.entry(node_id.clone()).or_insert_with(Vec::new);
        self.dependents.entry(node_id).or_insert_with(Vec::new);
        
        Ok(())
    }
    
    /// Add a dependency relationship between nodes
    pub fn add_dependency(&mut self, dependent: &str, dependency: &str) -> BrainResult<()> {
        // Validate nodes exist
        if !self.nodes.contains_key(dependent) {
            return Err(BrainError::Other(
                format!("Dependent node '{}' does not exist", dependent)
            ));
        }
        if !self.nodes.contains_key(dependency) {
            return Err(BrainError::Other(
                format!("Dependency node '{}' does not exist", dependency)
            ));
        }
        
        // Add dependency
        self.dependencies
            .entry(dependent.to_string())
            .or_insert_with(Vec::new)
            .push(dependency.to_string());
        
        // Add dependent
        self.dependents
            .entry(dependency.to_string())
            .or_insert_with(Vec::new)
            .push(dependent.to_string());
        
        Ok(())
    }
    
    /// Validate the DAG structure for correctness
    pub fn validate(&self) -> Result<(), DAGValidationError> {
        // Check for empty DAG
        if self.nodes.is_empty() {
            return Err(DAGValidationError::EmptyDAG);
        }
        
        // Check for cycles using DFS
        if let Some(cycle) = self.detect_cycle() {
            return Err(DAGValidationError::CyclicDependency { cycle });
        }
        
        // Check for missing dependencies
        for (node_id, deps) in &self.dependencies {
            for dep in deps {
                if !self.nodes.contains_key(dep) {
                    return Err(DAGValidationError::MissingDependency {
                        node: node_id.clone(),
                        missing_dep: dep.clone(),
                    });
                }
            }
        }
        
        // Check for unreachable nodes
        let reachable = self.get_reachable_nodes();
        let unreachable: Vec<String> = self.nodes.keys()
            .filter(|&id| !reachable.contains(id))
            .cloned()
            .collect();
        
        if !unreachable.is_empty() {
            return Err(DAGValidationError::UnreachableNodes { nodes: unreachable });
        }
        
        Ok(())
    }
    
    /// Detect cycles in the DAG using DFS
    fn detect_cycle(&self) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        
        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                if let Some(cycle) = self.dfs_cycle_detection(
                    node_id, 
                    &mut visited, 
                    &mut rec_stack, 
                    &mut path
                ) {
                    return Some(cycle);
                }
            }
        }
        
        None
    }
    
    /// DFS helper for cycle detection
    fn dfs_cycle_detection(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        path.push(node_id.to_string());
        
        if let Some(dependents) = self.dependents.get(node_id) {
            for dependent in dependents {
                if !visited.contains(dependent) {
                    if let Some(cycle) = self.dfs_cycle_detection(
                        dependent, 
                        visited, 
                        rec_stack, 
                        path
                    ) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dependent) {
                    // Found cycle, extract it from path
                    let cycle_start = path.iter().position(|x| x == dependent).unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(dependent.clone());
                    return Some(cycle);
                }
            }
        }
        
        rec_stack.remove(node_id);
        path.pop();
        None
    }
    
    /// Get all nodes reachable from root nodes
    fn get_reachable_nodes(&self) -> HashSet<String> {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start from root nodes (nodes with no dependencies)
        for (node_id, deps) in &self.dependencies {
            if deps.is_empty() {
                queue.push_back(node_id.clone());
                reachable.insert(node_id.clone());
            }
        }
        
        // BFS to find all reachable nodes
        while let Some(node_id) = queue.pop_front() {
            if let Some(dependents) = self.dependents.get(&node_id) {
                for dependent in dependents {
                    if !reachable.contains(dependent) {
                        reachable.insert(dependent.clone());
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }
        
        reachable
    }
    
    /// Generate execution plan from the DAG
    pub fn create_execution_plan(&self, order: ExecutionOrder) -> BrainResult<ExecutionPlan> {
        // Validate first
        self.validate().map_err(|e| BrainError::Other(format!("{:?}", e)))?;
        
        // Create topological ordering
        let topo_order = self.topological_sort()?;
        
        // Group into execution waves
        let execution_waves = self.create_execution_waves(&topo_order);
        
        // Calculate metrics
        let estimated_total_duration_ms = execution_waves
            .iter()
            .map(|wave| wave.estimated_duration_ms)
            .sum();
        
        let max_parallelism = execution_waves
            .iter()
            .map(|wave| wave.node_ids.len())
            .max()
            .unwrap_or(0);
        
        let critical_path = self.find_critical_path();
        
        Ok(ExecutionPlan {
            execution_waves,
            estimated_total_duration_ms,
            max_parallelism,
            critical_path,
            execution_order: order,
        })
    }
    
    /// Perform topological sort of the DAG
    fn topological_sort(&self) -> BrainResult<Vec<String>> {
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // Calculate in-degrees
        for node_id in self.nodes.keys() {
            let deps = self.dependencies.get(node_id).map(|d| d.len()).unwrap_or(0);
            in_degree.insert(node_id.clone(), deps);
            
            if deps == 0 {
                queue.push_back(node_id.clone());
            }
        }
        
        // Process nodes with zero in-degree
        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());
            
            // Reduce in-degree for dependents
            if let Some(dependents) = self.dependents.get(&node_id) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }
        
        // Check if all nodes were processed (no cycles)
        if result.len() != self.nodes.len() {
            return Err(BrainError::Other(
                "Cyclic dependency detected in DAG".to_string()
            ));
        }
        
        Ok(result)
    }
    
    /// Create execution waves from topological order
    fn create_execution_waves(&self, topo_order: &[String]) -> Vec<ExecutionWave> {
        let mut waves = Vec::new();
        let mut completed = HashSet::new();
        let mut wave_number = 0;
        
        while completed.len() < self.nodes.len() {
            let mut current_wave = Vec::new();
            let mut max_duration = 0;
            
            // Find all nodes that can execute in this wave
            for node_id in topo_order {
                if completed.contains(node_id) {
                    continue;
                }
                
                // Check if all dependencies are completed
                let empty_deps = Vec::new();
                let deps = self.dependencies.get(node_id).unwrap_or(&empty_deps);
                if deps.iter().all(|dep| completed.contains(dep)) {
                    current_wave.push(node_id.clone());
                    
                    // Update wave duration
                    if let Some(node) = self.nodes.get(node_id) {
                        max_duration = max_duration.max(node.estimated_duration_ms);
                    }
                }
            }
            
            // Mark current wave nodes as completed
            for node_id in &current_wave {
                completed.insert(node_id.clone());
            }
            
            if !current_wave.is_empty() {
                waves.push(ExecutionWave {
                    node_ids: current_wave,
                    wave_number,
                    estimated_duration_ms: max_duration,
                });
                wave_number += 1;
            } else {
                // Safety check to prevent infinite loop
                break;
            }
        }
        
        waves
    }
    
    /// Find the critical path (longest path) through the DAG
    fn find_critical_path(&self) -> Vec<String> {
        // For now, return a simple path from root to leaf
        // TODO: Implement proper critical path analysis
        if let Some(root) = self.roots.first() {
            vec![root.clone()]
        } else {
            Vec::new()
        }
    }
    
    /// Update DAG structure after adding/removing nodes
    pub fn update_structure(&mut self) {
        // Update roots and leaves
        self.roots = self.nodes
            .keys()
            .filter(|id| {
                self.dependencies
                    .get(*id)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .cloned()
            .collect();
        
        self.leaves = self.nodes
            .keys()
            .filter(|id| {
                self.dependents
                    .get(*id)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .cloned()
            .collect();
    }
    
    /// Get input for a specific agent (needed for orchestrator integration)
    pub fn get_input_for_agent(&self, agent_id: &str) -> Option<&AgentInput> {
        self.nodes.get(agent_id).map(|node| &node.input)
    }
    
    /// Get agent node by ID
    pub fn get_node(&self, node_id: &str) -> Option<&AgentNode> {
        self.nodes.get(node_id)
    }
    
    /// Get mutable agent node by ID
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut AgentNode> {
        self.nodes.get_mut(node_id)
    }
    
    /// Update node state
    pub fn update_node_state(&mut self, node_id: &str, state: NodeState) -> BrainResult<()> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.state = state;
            Ok(())
        } else {
            Err(BrainError::Other(format!("Node '{}' not found", node_id)))
        }
    }
    
    /// Set node output
    pub fn set_node_output(&mut self, node_id: &str, output: AgentOutput) -> BrainResult<()> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.output = Some(output);
            node.state = NodeState::Completed;
            Ok(())
        } else {
            Err(BrainError::Other(format!("Node '{}' not found", node_id)))
        }
    }
    
    /// Set node error
    pub fn set_node_error(&mut self, node_id: &str, error: BrainError) -> BrainResult<()> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.error = Some(error);
            node.state = NodeState::Failed;
            Ok(())
        } else {
            Err(BrainError::Other(format!("Node '{}' not found", node_id)))
        }
    }
}

impl DAGBuilder {
    /// Create a new DAG builder
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            inputs: Vec::new(),
            explicit_dependencies: HashMap::new(),
            priorities: HashMap::new(),
            estimated_durations: HashMap::new(),
        }
    }
    
    /// Add agents to the builder
    pub fn with_agents(mut self, agents: Vec<Arc<dyn BrainAgent>>) -> Self {
        self.agents = agents;
        self
    }
    
    /// Add inputs to the builder
    pub fn with_inputs(mut self, inputs: Vec<AgentInput>) -> Self {
        self.inputs = inputs;
        self
    }
    
    /// Add explicit dependency
    pub fn with_dependency(mut self, dependent: String, dependency: String) -> Self {
        self.explicit_dependencies
            .entry(dependent)
            .or_insert_with(Vec::new)
            .push(dependency);
        self
    }
    
    /// Set node priority
    pub fn with_priority(mut self, node_id: String, priority: i32) -> Self {
        self.priorities.insert(node_id, priority);
        self
    }
    
    /// Set estimated duration
    pub fn with_duration(mut self, node_id: String, duration_ms: u64) -> Self {
        self.estimated_durations.insert(node_id, duration_ms);
        self
    }
    
    /// Build the DAG from configured components
    pub fn build(self) -> BrainResult<AgentDAG> {
        let mut dag = AgentDAG::new();
        
        // Create nodes from agents
        for (i, agent) in self.agents.into_iter().enumerate() {
            let node_id = format!("agent_{}", i);
            let input = self.inputs.get(i).cloned().unwrap_or_else(|| {
                AgentInput::new(
                    "default".to_string(),
                    "".to_string(),
                    "default_session".to_string(),
                )
            });
            
            let priority = self.priorities.get(&node_id).copied().unwrap_or(0);
            let estimated_duration_ms = self.estimated_durations.get(&node_id).copied().unwrap_or(1000);
            
            let node = AgentNode {
                id: node_id.clone(),
                agent,
                input,
                state: NodeState::Pending,
                output: None,
                error: None,
                priority,
                estimated_duration_ms,
            };
            
            dag.add_node(node)?;
        }
        
        // Add explicit dependencies
        for (dependent, dependencies) in self.explicit_dependencies {
            for dependency in dependencies {
                dag.add_dependency(&dependent, &dependency)?;
            }
        }
        
        // Update DAG structure
        dag.update_structure();
        
        Ok(dag)
    }
}

impl Default for DAGBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AgentDAG {
    fn default() -> Self {
        Self::new()
    }
} 