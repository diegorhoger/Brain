pub mod traits;
pub mod registry;

// Agent categories
pub mod development;
pub mod security;
// pub mod testing;
// pub mod ops;
// pub mod intelligence;
// pub mod platform;

// Re-exports for convenience
pub use traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, AgentCapability};
pub use registry::AgentRegistry;
pub use development::{PlannerAgent, ArchitectAgent, DesignerAgent};
pub use security::{CyberSecurityAgent}; // Temporarily disabled: PromptSecurityAgent, PrivacyComplianceAgent, DataPrivacyAgent, EthicalAIAgent 