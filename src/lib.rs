//! Brain AI - Multi-Crate Re-export Library
//! 
//! This module re-exports functionality from all Brain AI sub-crates to maintain
//! backward compatibility with existing examples and provide a unified API.

#![allow(ambiguous_glob_reexports)]
#![allow(unused_imports)]

// Re-export all types - this provides the foundation
pub use brain_types::*;

// Re-export core functionality with wildcard imports
pub use brain_core::*;

// Re-export infrastructure functionality 
pub use brain_infra::{
    // System integration with specific path to avoid conflicts
    system_integration::{
        BrainSystem, BrainSystemConfig, BrainSystemBuilder, ComponentRegistry,
        UnifiedAPI, WorkflowEngine, SystemHealth, SystemMetrics, SystemEvent,
        ComponentStatus, SystemComponent, Workflow, WorkflowExecution,
        HealthStatus, ComponentHealth, ComponentMetrics, EventType,
        IntegrationError, IntegrationResult,
    },
};

// Re-export cognitive functionality
pub use brain_cognitive::*;

// Re-export API functionality  
pub use brain_api::*;

// Convenience type aliases for common patterns
pub type Result<T> = std::result::Result<T, brain_types::BrainError>;

// Common authentication result type for backward compatibility
pub type AuthResult<T> = Result<T>;

// Re-export commonly used external types
pub use uuid::Uuid;
pub use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};

/// Initialize the Brain AI system with default configuration
pub async fn initialize() -> Result<BrainSystem> {
    BrainSystemBuilder::new().build().await
}

/// Initialize the Brain AI system with custom configuration
pub async fn initialize_with_config(config: BrainSystemConfig) -> Result<BrainSystem> {
    BrainSystemBuilder::new()
        .with_config(config)
        .build()
        .await
}

// Re-export query and export functionality for backward compatibility
// Note: These may need to be implemented or examples updated to use new APIs

/// Legacy compatibility - may need implementation
pub struct QueryEngine;
impl QueryEngine {
    pub fn new() -> Self { Self }
}

/// Legacy compatibility - may need implementation  
pub struct ExportSystem;
impl ExportSystem {
    pub fn new() -> Self { Self }
}

/// Legacy compatibility - may need implementation
pub struct SpecializedQueryEngine;
impl SpecializedQueryEngine {
    pub fn new() -> Self { Self }
}

/// Legacy compatibility enum
pub enum QueryResult {
    Concepts(Vec<ConceptNode>),
    Memories(Vec<String>),
    Rules(Vec<String>),
}

/// Legacy compatibility
pub struct ConceptQueryResult {
    pub id: String,
    pub confidence: f64,
    pub content: String,
}

/// Legacy compatibility  
pub struct SimilarityConfig {
    pub threshold: f64,
    pub max_results: usize,
}

/// Legacy compatibility for segment discovery
pub mod segment_discovery {
    pub use brain_infra::BpeSegmenter;
    pub use brain_core::BpeConfig;
    
    // Legacy compatibility types
    pub struct StorageConfig {
        pub path: String,
    }
    
    pub struct PruningConfig {
        pub max_vocab_size: usize,
    }
}

/// Legacy compatibility for memory system
pub mod memory {
    // Legacy MemorySystem wrapper
    pub struct MemorySystem;
    impl MemorySystem {
        pub fn new() -> Self { Self }
    }
}

/// Legacy compatibility for concept graph
pub mod concept_graph {
    pub use brain_infra::{ConceptGraphManager, ConceptGraphConfig};
    pub use brain_core::{ConceptNode, ConceptType, ConceptRepository};
}

/// Legacy compatibility for visualization
pub mod visualization {
    pub use brain_api::visualization::{VisualizationManager, VisualizationConfig};
}

/// Legacy compatibility for query language - these will need proper implementation
pub mod query_language {
    // Placeholder for backward compatibility
}

/// Legacy compatibility for export system
pub mod export_system {
    // Placeholder for backward compatibility
}

/// Legacy compatibility for specialized queries
pub mod specialized_queries {
    // Placeholder for backward compatibility
} 