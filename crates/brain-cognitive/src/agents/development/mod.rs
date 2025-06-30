//! Development Lifecycle Agents Module
//! 
//! This module contains specialized agents for the software development lifecycle.
//! Each agent focuses on a specific aspect of development and can collaborate
//! with other agents to create comprehensive development solutions.

pub mod planner;
pub mod architect;
pub mod designer;
pub mod schema;
pub mod api;

// Re-export development agents for easier access
pub use planner::PlannerAgent;
pub use architect::ArchitectAgent;
pub use designer::DesignerAgent;
pub use schema::SchemaAgent;
pub use api::APIAgent;

// TODO: Add remaining development agents:
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
            Box::new(ArchitectAgent::new()),
            Box::new(DesignerAgent::new()),
            Box::new(SchemaAgent::new()),
            Box::new(APIAgent::new()),
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
            ("ArchitectAgent", vec![
                "system_design",
                "component_architecture",
                "technology_selection",
                "data_modeling",
                "api_design",
                "scalability_planning",
                "security_architecture",
                "performance_optimization",
                "deployment_strategy",
                "architecture_validation"
            ]),
            ("DesignerAgent", vec![
                "ui_mockups",
                "component_design",
                "user_flow_mapping",
                "accessibility_planning",
                "design_system_creation",
                "responsive_design",
                "interaction_design",
                "visual_hierarchy",
                "usability_analysis",
                "prototype_creation"
            ]),
            ("SchemaAgent", vec![
                "entity_relationship_design",
                "schema_normalization",
                "indexing_optimization",
                "data_validation_design",
                "migration_planning",
                "performance_tuning",
                "multi_database_support",
                "data_security_planning",
                "scalability_modeling",
                "backup_strategy_design"
            ]),
            ("APIAgent", vec![
                "rest_api_design",
                "graphql_schema_design",
                "authentication_planning",
                "authorization_design",
                "rate_limiting_strategy",
                "api_versioning",
                "documentation_generation",
                "testing_framework_design",
                "performance_optimization",
                "error_handling_design"
            ]),
            // TODO: Add capabilities for other agents
        ]
    }
} 