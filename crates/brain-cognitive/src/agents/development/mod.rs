//! Development Lifecycle Agents Module
//! 
//! This module contains specialized agents for the software development lifecycle.
//! Each agent focuses on a specific aspect of development and can collaborate
//! with other agents to create comprehensive development solutions.

pub mod planner;

// Re-export development agents for easier access
pub use planner::PlannerAgent;

// TODO: Add remaining development agents:
// pub mod architect;
// pub mod designer;
// pub mod schema;
// pub mod api;
// pub mod frontend_coder;
// pub mod backend_coder;
// pub mod refactor;
// pub mod doc;
// pub mod deployer;
// pub mod maintainer;

/// Collection of all development lifecycle agents
pub struct DevelopmentAgents;

impl DevelopmentAgents {
    /// Get a vector of all available development agents
    pub fn all_agents() -> Vec<Box<dyn crate::agents::traits::BrainAgent + Send + Sync>> {
        vec![
            Box::new(PlannerAgent::new()),
            // TODO: Add other agents as they are implemented
        ]
    }

    /// Get agent names and their capabilities for discovery
    pub fn agent_capabilities() -> Vec<(&'static str, Vec<&'static str>)> {
        vec![
            ("PlannerAgent", vec![
                "requirement_analysis",
                "task_decomposition", 
                "dependency_mapping",
                "timeline_estimation",
                "risk_assessment",
                "resource_planning",
                "specification_writing",
                "stakeholder_analysis"
            ]),
            // TODO: Add capabilities for other agents
        ]
    }
} 