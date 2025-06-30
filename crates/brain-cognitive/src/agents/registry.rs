use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use brain_types::error::BrainError;
use crate::agents::traits::{BrainAgent, AgentMetadata, BrainResult};

/// Registry for managing and discovering Brain AI agents
pub struct AgentRegistry {
    /// Registered agents by ID
    agents: RwLock<HashMap<String, Arc<dyn BrainAgent>>>,
    
    /// Agent configurations loaded from JSON
    configurations: RwLock<HashMap<String, AgentConfiguration>>,
    
    /// Capability index for fast agent discovery
    capability_index: RwLock<HashMap<String, Vec<String>>>,
    
    /// Input type index for routing
    input_type_index: RwLock<HashMap<String, Vec<String>>>,
}

/// Configuration for an agent loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfiguration {
    /// Agent metadata
    pub metadata: AgentMetadata,
    
    /// Agent implementation details
    pub implementation: AgentImplementation,
    
    /// Configuration parameters
    pub config: HashMap<String, serde_json::Value>,
    
    /// Whether the agent is enabled
    pub enabled: bool,
}

/// Agent implementation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentImplementation {
    /// Type of implementation (e.g., "builtin", "plugin", "external")
    pub implementation_type: String,
    
    /// Implementation-specific configuration
    pub config: HashMap<String, serde_json::Value>,
    
    /// Required dependencies
    pub dependencies: Vec<String>,
}

/// Agent discovery query
#[derive(Debug, Clone)]
pub struct AgentQuery {
    /// Required input type
    pub input_type: Option<String>,
    
    /// Required capabilities
    pub capabilities: Vec<String>,
    
    /// Required tags
    pub tags: Vec<String>,
    
    /// Minimum confidence threshold
    pub min_confidence: Option<f32>,
    
    /// Maximum number of results
    pub limit: Option<usize>,
}

impl AgentRegistry {
    /// Create a new agent registry
    pub fn new() -> Self {
        Self {
            agents: RwLock::new(HashMap::new()),
            configurations: RwLock::new(HashMap::new()),
            capability_index: RwLock::new(HashMap::new()),
            input_type_index: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register a new agent
    pub fn register_agent(&self, agent: Arc<dyn BrainAgent>) -> BrainResult<()> {
        let metadata = agent.metadata().clone();
        let agent_id = metadata.id.clone();
        
        // Register the agent
        {
            let mut agents = self.agents.write()
                .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
            agents.insert(agent_id.clone(), agent);
        }
        
        // Update capability index
        {
            let mut capability_index = self.capability_index.write()
                .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
            
            for capability in &metadata.capabilities {
                capability_index
                    .entry(capability.clone())
                    .or_insert_with(Vec::new)
                    .push(agent_id.clone());
            }
        }
        
        // Update input type index
        {
            let mut input_type_index = self.input_type_index.write()
                .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
            
            for input_type in &metadata.supported_input_types {
                input_type_index
                    .entry(input_type.clone())
                    .or_insert_with(Vec::new)
                    .push(agent_id.clone());
            }
        }
        
        Ok(())
    }
    
    /// Unregister an agent
    pub fn unregister_agent(&self, agent_id: &str) -> BrainResult<()> {
        // Remove from agents
        let agent = {
            let mut agents = self.agents.write()
                .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
            agents.remove(agent_id)
        };
        
        if let Some(agent) = agent {
            let metadata = agent.metadata();
            
            // Remove from capability index
            {
                let mut capability_index = self.capability_index.write()
                    .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
                
                for capability in &metadata.capabilities {
                    if let Some(agent_list) = capability_index.get_mut(capability) {
                        agent_list.retain(|id| id != agent_id);
                        if agent_list.is_empty() {
                            capability_index.remove(capability);
                        }
                    }
                }
            }
            
            // Remove from input type index
            {
                let mut input_type_index = self.input_type_index.write()
                    .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
                
                for input_type in &metadata.supported_input_types {
                    if let Some(agent_list) = input_type_index.get_mut(input_type) {
                        agent_list.retain(|id| id != agent_id);
                        if agent_list.is_empty() {
                            input_type_index.remove(input_type);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Get an agent by ID
    pub fn get_agent(&self, agent_id: &str) -> BrainResult<Option<Arc<dyn BrainAgent>>> {
        let agents = self.agents.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(agents.get(agent_id).cloned())
    }
    
    /// Discover agents matching a query
    pub fn discover_agents(&self, query: &AgentQuery) -> BrainResult<Vec<Arc<dyn BrainAgent>>> {
        let agents = self.agents.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let mut candidates: Vec<Arc<dyn BrainAgent>> = Vec::new();
        
        // If input type is specified, use the input type index
        if let Some(input_type) = &query.input_type {
            let input_type_index = self.input_type_index.read()
                .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
            
            if let Some(agent_ids) = input_type_index.get(input_type) {
                for agent_id in agent_ids {
                    if let Some(agent) = agents.get(agent_id) {
                        candidates.push(agent.clone());
                    }
                }
            }
        } else {
            // If no input type specified, consider all agents
            candidates = agents.values().cloned().collect();
        }
        
        // Filter by capabilities
        if !query.capabilities.is_empty() {
            candidates.retain(|agent| {
                let metadata = agent.metadata();
                query.capabilities.iter().all(|capability| {
                    metadata.capabilities.contains(capability)
                })
            });
        }
        
        // Filter by tags
        if !query.tags.is_empty() {
            candidates.retain(|agent| {
                let metadata = agent.metadata();
                query.tags.iter().all(|tag| {
                    metadata.tags.contains(tag)
                })
            });
        }
        
        // Filter by confidence threshold
        if let Some(min_confidence) = query.min_confidence {
            candidates.retain(|agent| {
                agent.confidence_threshold() >= min_confidence
            });
        }
        
        // Apply limit
        if let Some(limit) = query.limit {
            candidates.truncate(limit);
        }
        
        Ok(candidates)
    }
    
    /// List all registered agents
    pub fn list_agents(&self) -> BrainResult<Vec<Arc<dyn BrainAgent>>> {
        let agents = self.agents.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(agents.values().cloned().collect())
    }
    
    /// Get agents by capability
    pub fn get_agents_by_capability(&self, capability: &str) -> BrainResult<Vec<Arc<dyn BrainAgent>>> {
        let capability_index = self.capability_index.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let agents = self.agents.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let mut result = Vec::new();
        if let Some(agent_ids) = capability_index.get(capability) {
            for agent_id in agent_ids {
                if let Some(agent) = agents.get(agent_id) {
                    result.push(agent.clone());
                }
            }
        }
        
        Ok(result)
    }
    
    /// Get agents by input type
    pub fn get_agents_by_input_type(&self, input_type: &str) -> BrainResult<Vec<Arc<dyn BrainAgent>>> {
        let input_type_index = self.input_type_index.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let agents = self.agents.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let mut result = Vec::new();
        if let Some(agent_ids) = input_type_index.get(input_type) {
            for agent_id in agent_ids {
                if let Some(agent) = agents.get(agent_id) {
                    result.push(agent.clone());
                }
            }
        }
        
        Ok(result)
    }
    
    /// Load agent configurations from JSON
    pub fn load_configurations(&self, config_data: &str) -> BrainResult<()> {
        let configs: Vec<AgentConfiguration> = serde_json::from_str(config_data)
            .map_err(|e| BrainError::ConfigError(format!("Failed to parse agent configurations: {}", e)))?;
        
        let mut configurations = self.configurations.write()
            .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        for config in configs {
            configurations.insert(config.metadata.id.clone(), config);
        }
        
        Ok(())
    }
    
    /// Get agent configuration
    pub fn get_configuration(&self, agent_id: &str) -> BrainResult<Option<AgentConfiguration>> {
        let configurations = self.configurations.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(configurations.get(agent_id).cloned())
    }
    
    /// Get statistics about registered agents
    pub fn get_statistics(&self) -> BrainResult<RegistryStatistics> {
        let agents = self.agents.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let capability_index = self.capability_index.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let input_type_index = self.input_type_index.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        Ok(RegistryStatistics {
            total_agents: agents.len(),
            total_capabilities: capability_index.len(),
            total_input_types: input_type_index.len(),
            agents_by_category: self.categorize_agents(&agents)?,
        })
    }
    
    /// Categorize agents by their tags
    fn categorize_agents(&self, agents: &HashMap<String, Arc<dyn BrainAgent>>) -> BrainResult<HashMap<String, usize>> {
        let mut categories = HashMap::new();
        
        for agent in agents.values() {
            let metadata = agent.metadata();
            for tag in &metadata.tags {
                *categories.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        Ok(categories)
    }
}

/// Statistics about the agent registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total number of registered agents
    pub total_agents: usize,
    
    /// Total number of unique capabilities
    pub total_capabilities: usize,
    
    /// Total number of unique input types
    pub total_input_types: usize,
    
    /// Number of agents by category/tag
    pub agents_by_category: HashMap<String, usize>,
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentQuery {
    /// Create a new empty query
    pub fn new() -> Self {
        Self {
            input_type: None,
            capabilities: Vec::new(),
            tags: Vec::new(),
            min_confidence: None,
            limit: None,
        }
    }
    
    /// Set the required input type
    pub fn with_input_type(mut self, input_type: String) -> Self {
        self.input_type = Some(input_type);
        self
    }
    
    /// Add a required capability
    pub fn with_capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
        self
    }
    
    /// Add a required tag
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    /// Set minimum confidence threshold
    pub fn with_min_confidence(mut self, confidence: f32) -> Self {
        self.min_confidence = Some(confidence);
        self
    }
    
    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Default for AgentQuery {
    fn default() -> Self {
        Self::new()
    }
} 