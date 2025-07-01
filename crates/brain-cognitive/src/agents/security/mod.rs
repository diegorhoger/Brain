pub mod cyber_security;
// pub mod prompt_security;
// pub mod privacy_compliance;
// pub mod data_privacy;
// pub mod ethical_ai;

pub use cyber_security::CyberSecurityAgent;
// pub use prompt_security::PromptSecurityAgent;
// pub use privacy_compliance::PrivacyComplianceAgent;
// pub use data_privacy::DataPrivacyAgent;
// pub use ethical_ai::EthicalAIAgent;

use std::collections::HashMap;
use crate::agents::traits::AgentCapability;

/// Get all available security agents and their capabilities
pub fn get_security_agents() -> HashMap<String, Vec<AgentCapability>> {
    let mut agents = HashMap::new();
    
    agents.insert(
        "CyberSecurityAgent".to_string(),
        vec![
            AgentCapability::Analysis,
            AgentCapability::Security,
            AgentCapability::Monitoring,
        ]
    );
    
    // Temporarily disabled - compilation issues
    /*
    agents.insert(
        "PromptSecurityAgent".to_string(),
        vec![
            AgentCapability::Analysis,
            AgentCapability::Security,
            AgentCapability::ContentModeration,
        ]
    );
    
    agents.insert(
        "PrivacyComplianceAgent".to_string(),
        vec![
            AgentCapability::Analysis,
            AgentCapability::Compliance,
            AgentCapability::DataGovernance,
        ]
    );
    
    agents.insert(
        "DataPrivacyAgent".to_string(),
        vec![
            AgentCapability::DataGovernance,
            AgentCapability::Security,
            AgentCapability::Analysis,
        ]
    );
    
    agents.insert(
        "EthicalAIAgent".to_string(),
        vec![
            AgentCapability::Analysis,
            AgentCapability::Compliance,
            AgentCapability::EthicalAI,
        ]
    );
    */
    
    agents
}

/// Security agent categories for organizational purposes
pub enum SecurityAgentCategory {
    Infrastructure,
    Application,
    Data,
    Compliance,
    Ethics,
}

/// Get agents by security category
pub fn get_agents_by_category(category: SecurityAgentCategory) -> Vec<String> {
    match category {
        SecurityAgentCategory::Infrastructure => vec!["CyberSecurityAgent".to_string()],
        SecurityAgentCategory::Application => vec![], // vec!["PromptSecurityAgent".to_string()],
        SecurityAgentCategory::Data => vec![], // vec!["DataPrivacyAgent".to_string()],
        SecurityAgentCategory::Compliance => vec![], // vec!["PrivacyComplianceAgent".to_string()],
        SecurityAgentCategory::Ethics => vec![], // vec!["EthicalAIAgent".to_string()],
    }
} 