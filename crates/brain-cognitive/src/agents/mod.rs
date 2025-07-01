pub mod development;
pub mod security;
pub mod testing;
pub mod ops;
pub mod intelligence;
pub mod platform;
pub mod registry;
pub mod traits;

// Re-exports for convenience
pub use traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, AgentCapability};
pub use registry::AgentRegistry;
pub use development::{PlannerAgent, ArchitectAgent, DesignerAgent};
pub use security::{CyberSecurityAgent}; // Temporarily disabled: PromptSecurityAgent, PrivacyComplianceAgent, DataPrivacyAgent, EthicalAIAgent 

// Re-export all agents for easy access
pub use development::*;
pub use security::*;
pub use testing::*;
pub use ops::*;
pub use intelligence::*;
pub use platform::*; 