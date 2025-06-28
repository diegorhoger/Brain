//! Brain - A post-transformer developmental AI architecture
//! 
//! This crate provides a unified interface to the Brain AI system, which is built
//! using a modular multi-crate architecture for better maintainability and reusability.

// Re-export core types and errors
pub use brain_types::{BrainError, Result};

// Re-export core functionality (avoiding conflicts)
pub use brain_core::{
    memory,
    concepts,
    insights,
    neural,
    segmentation,
};

// Re-export infrastructure (avoiding conflicts)
pub use brain_infra::{
    database,
    filesystem,
    http,
};

// Re-export cognitive architecture
pub use brain_cognitive::{
    conversation,
    intelligence,
    learning,
    meta,
    models,
};

// Legacy compatibility - keep the main modules that aren't migrated yet
pub mod docs_server;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Convenience re-exports for common functionality
pub mod prelude {
    //! Common imports for Brain AI applications
    
    pub use crate::{BrainError, Result};
    pub use crate::{memory, concepts, insights, neural, segmentation};
    pub use crate::{conversation, intelligence, learning, meta, models};
    pub use crate::{database, filesystem, http};
}

// High-level system interface
pub struct BrainSystem {
    // This will be the main entry point for the unified system
    // Implementation will be added as we migrate
}

impl BrainSystem {
    /// Create a new Brain AI system instance
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    /// Initialize the system with default configuration
    pub async fn initialize() -> Result<Self> {
        let system = Self::new()?;
        // TODO: Add initialization logic
        Ok(system)
    }
}

impl Default for BrainSystem {
    fn default() -> Self {
        Self::new().expect("Failed to create default BrainSystem")
    }
} 