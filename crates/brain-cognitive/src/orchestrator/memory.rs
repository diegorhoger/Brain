//! Agent Memory Management for Orchestration

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Memory management system for orchestrated agents
#[derive(Debug)]
pub struct OrchestratorMemory {
    namespaces: Arc<RwLock<HashMap<String, AgentMemoryNamespace>>>,
}

impl OrchestratorMemory {
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn create_namespace(&self, agent_id: String) -> AgentMemoryNamespace {
        let mut namespaces = self.namespaces.write().await;
        let namespace = AgentMemoryNamespace::new(agent_id.clone());
        namespaces.insert(agent_id, namespace.clone());
        namespace
    }
    
    pub async fn get_namespace(&self, agent_id: &str) -> Option<AgentMemoryNamespace> {
        let namespaces = self.namespaces.read().await;
        namespaces.get(agent_id).cloned()
    }
    
    pub async fn memory_usage_mb(&self) -> f64 {
        let namespaces = self.namespaces.read().await;
        namespaces.len() as f64 * 0.1 // Estimate 0.1MB per namespace
    }
    
    pub async fn cleanup_unused_namespaces(&self) {
        let mut namespaces = self.namespaces.write().await;
        namespaces.retain(|_, namespace| !namespace.is_empty());
    }
}

/// Memory namespace for a specific agent
#[derive(Debug, Clone)]
pub struct AgentMemoryNamespace {
    pub agent_id: String,
    pub memory_entries: HashMap<String, serde_json::Value>,
    pub access_control: MemoryAccessControl,
}

impl AgentMemoryNamespace {
    pub fn new(agent_id: String) -> Self {
        Self {
            agent_id,
            memory_entries: HashMap::new(),
            access_control: MemoryAccessControl::default(),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.memory_entries.is_empty()
    }
    
    pub fn store(&mut self, key: String, value: serde_json::Value) {
        self.memory_entries.insert(key, value);
    }
    
    pub fn retrieve(&self, key: &str) -> Option<&serde_json::Value> {
        self.memory_entries.get(key)
    }
}

/// Registry for managing agent memory namespaces
#[derive(Debug, Clone)]
pub struct MemoryRegistry {
    pub registered_agents: Vec<String>,
}

impl MemoryRegistry {
    pub fn new() -> Self {
        Self {
            registered_agents: Vec::new(),
        }
    }
    
    pub fn register_agent(&mut self, agent_id: String) {
        if !self.registered_agents.contains(&agent_id) {
            self.registered_agents.push(agent_id);
        }
    }
}

/// Cross-agent memory sharing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossAgentMemoryShare {
    pub source_agent: String,
    pub target_agent: String,
    pub shared_keys: Vec<String>,
    pub permissions: SharePermissions,
}

/// Memory access control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccessControl {
    pub read_only: bool,
    pub shared_with: Vec<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for MemoryAccessControl {
    fn default() -> Self {
        Self {
            read_only: false,
            shared_with: Vec::new(),
            expires_at: None,
        }
    }
}

/// Permissions for memory sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharePermissions {
    ReadOnly,
    ReadWrite,
    WriteOnly,
}
