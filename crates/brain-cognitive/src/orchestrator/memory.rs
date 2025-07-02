//! Agent Memory Management for Orchestration
//! 
//! Integrates orchestrator memory with the existing MetaMemory system for comprehensive
//! tracking of agent execution, orchestration outcomes, and memory namespaces.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::meta::{
    MetaMemoryService, KnowledgeType, 
};
use brain_types::error::BrainError;

/// Enhanced memory management system for orchestrated agents with MetaMemory integration
pub struct OrchestratorMemory {
    /// Agent memory namespaces for isolated agent memory
    namespaces: Arc<RwLock<HashMap<String, AgentMemoryNamespace>>>,
    
    /// Integration with MetaMemory system for tracking orchestration components
    meta_memory: Option<Arc<MetaMemoryService>>,
    
    /// Configuration for memory tracking
    config: OrchestrationMemoryConfig,
}

/// Configuration for orchestration memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMemoryConfig {
    /// Enable MetaMemory integration for tracking
    pub enable_meta_memory: bool,
    
    /// Track agent execution results in MetaMemory
    pub track_agent_execution: bool,
    
    /// Track DAG execution outcomes
    pub track_dag_execution: bool,
    
    /// Track orchestration decisions and confidence
    pub track_orchestration_decisions: bool,
    
    /// Minimum confidence threshold for tracking
    pub min_confidence_tracking: f64,
    
    /// Maximum memory namespaces to maintain
    pub max_namespaces: usize,
}

impl Default for OrchestrationMemoryConfig {
    fn default() -> Self {
        Self {
            enable_meta_memory: true,
            track_agent_execution: true,
            track_dag_execution: true,
            track_orchestration_decisions: true,
            min_confidence_tracking: 0.3,
            max_namespaces: 100,
        }
    }
}

impl OrchestratorMemory {
    /// Create new orchestrator memory with default configuration
    pub fn new() -> Self {
        Self::with_config(OrchestrationMemoryConfig::default())
    }
    
    /// Create new orchestrator memory with custom configuration
    pub fn with_config(config: OrchestrationMemoryConfig) -> Self {
        Self {
            namespaces: Arc::new(RwLock::new(HashMap::new())),
            meta_memory: None,
            config,
        }
    }
    
    /// Create new orchestrator memory with MetaMemory integration
    pub fn with_meta_memory(
        config: OrchestrationMemoryConfig,
        meta_memory: Arc<MetaMemoryService>,
    ) -> Self {
        Self {
            namespaces: Arc::new(RwLock::new(HashMap::new())),
            meta_memory: Some(meta_memory),
            config,
        }
    }
    
    /// Create memory namespace for an agent
    pub async fn create_namespace(&self, agent_id: String) -> Result<AgentMemoryNamespace, BrainError> {
        let mut namespaces = self.namespaces.write().await;
        
        // Check capacity limits
        if namespaces.len() >= self.config.max_namespaces {
            return Err(BrainError::Other(
                format!("Maximum namespaces exceeded: {}", self.config.max_namespaces)
            ));
        }
        
        let namespace = AgentMemoryNamespace::new(agent_id.clone());
        namespaces.insert(agent_id.clone(), namespace.clone());
        
        // Track namespace creation in MetaMemory if enabled
        if self.config.enable_meta_memory {
            if let Some(meta_memory) = &self.meta_memory {
                let _ = meta_memory.track_component(
                    namespace.id,
                    KnowledgeType::OrchestrationNamespace,
                    0.8, // High confidence for namespace creation
                    format!("Orchestrator namespace for agent: {}", agent_id),
                ).await;
            }
        }
        
        Ok(namespace)
    }
    
    /// Get memory namespace for an agent
    pub async fn get_namespace(&self, agent_id: &str) -> Option<AgentMemoryNamespace> {
        let namespaces = self.namespaces.read().await;
        let namespace = namespaces.get(agent_id).cloned();
        
        // Mark as accessed in MetaMemory
        if let Some(ref ns) = namespace {
            if self.config.enable_meta_memory {
                if let Some(meta_memory) = &self.meta_memory {
                    let _ = meta_memory.mark_accessed(ns.id).await;
                }
            }
        }
        
        namespace
    }
    
    /// Track agent execution result in MetaMemory
    pub async fn track_agent_execution(
        &self,
        agent_id: &str,
        execution_id: Uuid,
        success: bool,
        confidence: f64,
        execution_time_ms: u64,
    ) -> Result<(), BrainError> {
        if !self.config.track_agent_execution || confidence < self.config.min_confidence_tracking {
            return Ok(());
        }
        
        if let Some(meta_memory) = &self.meta_memory {
            // Track the execution result
            let execution_result_id = meta_memory.track_component(
                execution_id,
                KnowledgeType::AgentExecution,
                confidence,
                format!("Agent {} execution", agent_id),
            ).await.map_err(|e| BrainError::Other(format!("MetaMemory tracking failed: {}", e)))?;
            
            // Update confidence based on success
            meta_memory.update_confidence(execution_id, success)
                .await
                .map_err(|e| BrainError::Other(format!("Confidence update failed: {}", e)))?;
            
            // Store execution metadata
            if let Some(mut namespace) = self.get_namespace(agent_id).await {
                let metadata = ExecutionMetadata {
                    execution_id,
                    success,
                    confidence,
                    execution_time_ms,
                    timestamp: Utc::now(),
                    meta_memory_id: execution_result_id,
                };
                
                namespace.store_execution_metadata(metadata).await;
            }
        }
        
        Ok(())
    }
    
    /// Track DAG execution outcome
    pub async fn track_dag_execution(
        &self,
        dag_id: Uuid,
        execution_plan_id: Uuid,
        success: bool,
        overall_confidence: f64,
        agent_count: usize,
        _total_execution_time_ms: u64,
    ) -> Result<(), BrainError> {
        if !self.config.track_dag_execution || overall_confidence < self.config.min_confidence_tracking {
            return Ok(());
        }
        
        if let Some(meta_memory) = &self.meta_memory {
            // Track DAG execution
            let _ = meta_memory.track_component(
                dag_id,
                KnowledgeType::DAGExecution,
                overall_confidence,
                format!("DAG execution with {} agents", agent_count),
            ).await.map_err(|e| BrainError::Other(format!("DAG tracking failed: {}", e)))?;
            
            // Track execution plan
            let _ = meta_memory.track_component(
                execution_plan_id,
                KnowledgeType::ExecutionPlan,
                if success { 0.9 } else { 0.3 },
                format!("Execution plan for DAG {}", dag_id),
            ).await.map_err(|e| BrainError::Other(format!("Plan tracking failed: {}", e)))?;
            
            // Update confidence based on success
            meta_memory.update_confidence(dag_id, success)
                .await
                .map_err(|e| BrainError::Other(format!("DAG confidence update failed: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Track orchestration decision
    pub async fn track_orchestration_decision(
        &self,
        decision_id: Uuid,
        decision_type: OrchestrationDecisionType,
        confidence: f64,
        outcome_success: Option<bool>,
    ) -> Result<(), BrainError> {
        if !self.config.track_orchestration_decisions || confidence < self.config.min_confidence_tracking {
            return Ok(());
        }
        
        if let Some(meta_memory) = &self.meta_memory {
            let _ = meta_memory.track_component(
                decision_id,
                KnowledgeType::OrchestrationDecision,
                confidence,
                format!("Orchestration decision: {:?}", decision_type),
            ).await.map_err(|e| BrainError::Other(format!("Decision tracking failed: {}", e)))?;
            
            // Update confidence if outcome is known
            if let Some(success) = outcome_success {
                meta_memory.update_confidence(decision_id, success)
                    .await
                    .map_err(|e| BrainError::Other(format!("Decision confidence update failed: {}", e)))?;
            }
        }
        
        Ok(())
    }
    
    /// Get memory usage statistics
    pub async fn get_memory_stats(&self) -> OrchestrationMemoryStats {
        let namespaces = self.namespaces.read().await;
        
        OrchestrationMemoryStats {
            total_namespaces: namespaces.len(),
            active_namespaces: namespaces.values().filter(|ns| !ns.is_empty()).count(),
            total_memory_entries: namespaces.values().map(|ns| ns.memory_entries.len()).sum(),
            estimated_memory_mb: namespaces.len() as f64 * 0.1, // Rough estimate
            meta_memory_integration: self.meta_memory.is_some(),
        }
    }
    
    pub async fn memory_usage_mb(&self) -> f64 {
        let stats = self.get_memory_stats().await;
        stats.estimated_memory_mb
    }
    
    pub async fn cleanup_unused_namespaces(&self) {
        let mut namespaces = self.namespaces.write().await;
        namespaces.retain(|_, namespace| !namespace.is_empty());
    }
    
    /// Create a memory share between two agents
    pub async fn create_memory_share(
        &self,
        source_agent: String,
        target_agent: String,
        shared_keys: Vec<String>,
        permissions: SharePermissions,
    ) -> Result<CrossAgentMemoryShare, BrainError> {
        // Validate that both agents have namespaces
        let source_ns = self.get_namespace(&source_agent).await
            .ok_or_else(|| BrainError::Other(format!("Source agent namespace not found: {}", source_agent)))?;
        
        let _target_ns = self.get_namespace(&target_agent).await
            .ok_or_else(|| BrainError::Other(format!("Target agent namespace not found: {}", target_agent)))?;
        
        // Validate that shared keys exist in source namespace
        for key in &shared_keys {
            if !source_ns.memory_entries.contains_key(key) {
                return Err(BrainError::Other(format!("Shared key '{}' not found in source namespace", key)));
            }
        }
        
        let share = CrossAgentMemoryShare {
            source_agent: source_agent.clone(),
            target_agent: target_agent.clone(),
            shared_keys,
            permissions,
        };
        
        // Update target agent's access control
        if let Some(mut target_ns) = self.get_namespace(&target_agent).await {
            target_ns.access_control.shared_with.push(source_agent.clone());
        }
        
        // Track the memory share creation in MetaMemory
        if self.config.enable_meta_memory {
            if let Some(meta_memory) = &self.meta_memory {
                let share_id = Uuid::new_v4();
                let _ = meta_memory.track_component(
                    share_id,
                    KnowledgeType::OrchestrationDecision,
                    0.8,
                    format!("Memory share from {} to {}", source_agent, target_agent),
                ).await;
            }
        }
        
        Ok(share)
    }
    
    /// Access shared memory from another agent
    pub async fn access_shared_memory(
        &self,
        requesting_agent: &str,
        share: &CrossAgentMemoryShare,
        key: &str,
    ) -> Result<Option<serde_json::Value>, BrainError> {
        // Validate access permissions
        if share.target_agent != requesting_agent {
            return Err(BrainError::Other("Access denied: not the target agent".to_string()));
        }
        
        if !share.shared_keys.contains(&key.to_string()) {
            return Err(BrainError::Other(format!("Key '{}' not shared", key)));
        }
        
        match share.permissions {
            SharePermissions::WriteOnly => {
                return Err(BrainError::Other("Read access denied: write-only permission".to_string()));
            }
            SharePermissions::ReadOnly | SharePermissions::ReadWrite => {
                // Allowed to read
            }
        }
        
        // Get the value from source agent's namespace
        if let Some(source_ns) = self.get_namespace(&share.source_agent).await {
            Ok(source_ns.retrieve(key).cloned())
        } else {
            Err(BrainError::Other("Source agent namespace not found".to_string()))
        }
    }
    
    /// Write to shared memory (if permissions allow)
    pub async fn write_shared_memory(
        &self,
        requesting_agent: &str,
        share: &CrossAgentMemoryShare,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), BrainError> {
        // Validate access permissions
        if share.target_agent != requesting_agent {
            return Err(BrainError::Other("Access denied: not the target agent".to_string()));
        }
        
        if !share.shared_keys.contains(&key.to_string()) {
            return Err(BrainError::Other(format!("Key '{}' not shared", key)));
        }
        
        match share.permissions {
            SharePermissions::ReadOnly => {
                return Err(BrainError::Other("Write access denied: read-only permission".to_string()));
            }
            SharePermissions::WriteOnly | SharePermissions::ReadWrite => {
                // Allowed to write
            }
        }
        
        // Write to source agent's namespace (the namespace that owns the data)
        let mut namespaces = self.namespaces.write().await;
        if let Some(source_ns) = namespaces.get_mut(&share.source_agent) {
            source_ns.store(key.to_string(), value);
            Ok(())
        } else {
            Err(BrainError::Other("Source agent namespace not found".to_string()))
        }
    }
    
    /// Revoke memory share
    pub async fn revoke_memory_share(
        &self,
        share: &CrossAgentMemoryShare,
    ) -> Result<(), BrainError> {
        // Remove the sharing relationship from target agent's access control
        let mut namespaces = self.namespaces.write().await;
        if let Some(target_ns) = namespaces.get_mut(&share.target_agent) {
            target_ns.access_control.shared_with.retain(|agent| agent != &share.source_agent);
        }
        
        // Track the revocation in MetaMemory
        if self.config.enable_meta_memory {
            if let Some(meta_memory) = &self.meta_memory {
                let revocation_id = Uuid::new_v4();
                let _ = meta_memory.track_component(
                    revocation_id,
                    KnowledgeType::OrchestrationDecision,
                    0.7,
                    format!("Memory share revoked from {} to {}", share.source_agent, share.target_agent),
                ).await;
            }
        }
        
        Ok(())
    }
    
    /// Validate access control for a memory operation
    pub async fn validate_access(
        &self,
        agent_id: &str,
        operation: MemoryOperation,
    ) -> Result<bool, BrainError> {
        if let Some(namespace) = self.get_namespace(agent_id).await {
            // Check if access control allows the operation
            match operation {
                MemoryOperation::Read => {
                    // Always allow reads to own namespace
                    Ok(true)
                }
                MemoryOperation::Write => {
                    // Check if namespace is read-only
                    Ok(!namespace.access_control.read_only)
                }
                MemoryOperation::Share => {
                    // Always allow sharing from own namespace
                    Ok(true)
                }
            }
        } else {
            Err(BrainError::Other(format!("Namespace not found for agent: {}", agent_id)))
        }
    }
    
    /// Get all active memory shares for an agent
    pub async fn get_agent_shares(&self, agent_id: &str) -> Vec<String> {
        if let Some(namespace) = self.get_namespace(agent_id).await {
            namespace.access_control.shared_with.clone()
        } else {
            Vec::new()
        }
    }
}

/// Enhanced memory namespace for agents with execution tracking
#[derive(Debug, Clone)]
pub struct AgentMemoryNamespace {
    /// Unique identifier for this namespace
    pub id: Uuid,
    
    /// Agent ID this namespace belongs to
    pub agent_id: String,
    
    /// General memory entries
    pub memory_entries: HashMap<String, serde_json::Value>,
    
    /// Execution metadata history
    pub execution_history: Vec<ExecutionMetadata>,
    
    /// Access control settings
    pub access_control: MemoryAccessControl,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last accessed timestamp
    pub last_accessed: DateTime<Utc>,
}

impl AgentMemoryNamespace {
    pub fn new(agent_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            agent_id,
            memory_entries: HashMap::new(),
            execution_history: Vec::new(),
            access_control: MemoryAccessControl::default(),
            created_at: now,
            last_accessed: now,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.memory_entries.is_empty() && self.execution_history.is_empty()
    }
    
    pub fn store(&mut self, key: String, value: serde_json::Value) {
        self.memory_entries.insert(key, value);
        self.last_accessed = Utc::now();
    }
    
    pub fn retrieve(&self, key: &str) -> Option<&serde_json::Value> {
        self.memory_entries.get(key)
    }
    
    pub async fn store_execution_metadata(&mut self, metadata: ExecutionMetadata) {
        self.execution_history.push(metadata);
        self.last_accessed = Utc::now();
        
        // Keep only recent execution history (last 100 entries)
        if self.execution_history.len() > 100 {
            self.execution_history.remove(0);
        }
    }
    
    pub fn get_recent_executions(&self, limit: usize) -> &[ExecutionMetadata] {
        let start = if self.execution_history.len() > limit {
            self.execution_history.len() - limit
        } else {
            0
        };
        &self.execution_history[start..]
    }
}

/// Execution metadata for tracking agent performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    pub execution_id: Uuid,
    pub success: bool,
    pub confidence: f64,
    pub execution_time_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub meta_memory_id: Uuid,
}

/// Types of orchestration decisions to track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrchestrationDecisionType {
    AgentSelection,
    ExecutionOrdering,
    ResourceAllocation,
    FailureRecovery,
    ConfidenceThreshold,
    RetryStrategy,
    TaskPrioritization,
}

/// Statistics for orchestration memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMemoryStats {
    pub total_namespaces: usize,
    pub active_namespaces: usize,
    pub total_memory_entries: usize,
    pub estimated_memory_mb: f64,
    pub meta_memory_integration: bool,
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
    pub expires_at: Option<DateTime<Utc>>,
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

/// Types of memory operations for access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryOperation {
    Read,
    Write,
    Share,
}
