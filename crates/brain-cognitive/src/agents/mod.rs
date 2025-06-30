pub mod traits;
pub mod registry;

// Agent categories
pub mod development;

// Future agent categories (will be implemented in later phases)
// pub mod security;
// pub mod testing;
// pub mod ops;
// pub mod intelligence;
// pub mod platform;

// Re-exports for convenience
pub use traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext};
pub use registry::AgentRegistry;
pub use development::{PlannerAgent, ArchitectAgent, DesignerAgent}; 