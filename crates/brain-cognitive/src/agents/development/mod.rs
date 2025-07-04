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
pub mod frontend_coder;
pub mod backend_coder;
pub mod refactor;
pub mod doc;
pub mod deployer;
pub mod maintainer;

// Re-export development agents for easier access
pub use planner::PlannerAgent;
pub use architect::ArchitectAgent;
pub use designer::DesignerAgent;
pub use schema::SchemaAgent;
pub use api::APIAgent;
pub use frontend_coder::FrontendCoder;
pub use backend_coder::BackendCoder;
pub use refactor::RefactorAgent;
pub use doc::DocAgent;
pub use deployer::DeployerAgent;
pub use maintainer::MaintainerAgent;

/// Collection of all development lifecycle agents
pub struct DevelopmentAgents;

impl DevelopmentAgents {
    /// Get all available development agents
    pub fn all_agents() -> Vec<Box<dyn crate::agents::traits::BrainAgent + Send + Sync>> {
        vec![
            Box::new(PlannerAgent::new()),
            Box::new(ArchitectAgent::new()),
            Box::new(DesignerAgent::new()),
            Box::new(SchemaAgent::new()),
            Box::new(APIAgent::new()),
            Box::new(FrontendCoder::new()),
            Box::new(BackendCoder::new()),
            Box::new(RefactorAgent::new()),
            Box::new(DocAgent::new()),
            Box::new(DeployerAgent::new()),
            Box::new(MaintainerAgent::new()),
        ]
    }

    /// Get agent capabilities mapping
    pub fn agent_capabilities() -> std::collections::HashMap<String, Vec<String>> {
        let mut capabilities = std::collections::HashMap::new();
        
        capabilities.insert("planner-agent".to_string(), vec![
            "requirements_analysis".to_string(),
            "product_specification".to_string(),
            "feature_prioritization".to_string(),
            "user_story_generation".to_string(),
            "acceptance_criteria_definition".to_string(),
            "technical_requirement_extraction".to_string(),
            "project_scope_definition".to_string(),
            "stakeholder_requirement_analysis".to_string(),
            "risk_assessment_planning".to_string(),
            "timeline_estimation".to_string(),
        ]);

        capabilities.insert("architect-agent".to_string(), vec![
            "system_architecture_design".to_string(),
            "technology_stack_selection".to_string(),
            "scalability_planning".to_string(),
            "security_architecture_design".to_string(),
            "performance_optimization_planning".to_string(),
            "integration_strategy_design".to_string(),
            "deployment_architecture_planning".to_string(),
            "data_flow_architecture".to_string(),
            "microservices_architecture".to_string(),
            "infrastructure_planning".to_string(),
        ]);

        capabilities.insert("designer-agent".to_string(), vec![
            "ui_design_system_creation".to_string(),
            "user_experience_optimization".to_string(),
            "wireframe_generation".to_string(),
            "component_library_design".to_string(),
            "responsive_design_planning".to_string(),
            "accessibility_compliance_design".to_string(),
            "user_interaction_design".to_string(),
            "visual_design_system".to_string(),
            "prototyping_and_mockups".to_string(),
            "design_pattern_implementation".to_string(),
        ]);

        capabilities.insert("schema-agent".to_string(), vec![
            "entity_relationship_design".to_string(),
            "schema_normalization".to_string(),
            "indexing_optimization".to_string(),
            "data_validation_design".to_string(),
            "migration_planning".to_string(),
            "performance_tuning".to_string(),
            "multi_database_support".to_string(),
            "data_security_planning".to_string(),
            "scalability_modeling".to_string(),
            "backup_strategy_design".to_string(),
        ]);

        capabilities.insert("api-agent".to_string(), vec![
            "openapi_specification_generation".to_string(),
            "authentication_strategy_design".to_string(),
            "rate_limiting_framework".to_string(),
            "error_handling_design".to_string(),
            "api_versioning_strategy".to_string(),
            "endpoint_optimization".to_string(),
            "api_documentation_generation".to_string(),
            "security_implementation".to_string(),
            "testing_strategy_design".to_string(),
            "performance_monitoring".to_string(),
        ]);

        capabilities.insert("frontend-coder".to_string(), vec![
            "component_generation".to_string(),
            "react_development".to_string(),
            "vue_development".to_string(),
            "angular_development".to_string(),
            "state_management".to_string(),
            "api_integration".to_string(),
            "responsive_design".to_string(),
            "accessibility_implementation".to_string(),
            "performance_optimization".to_string(),
            "testing_implementation".to_string(),
        ]);

        capabilities.insert("backend-coder".to_string(), vec![
            "api_development".to_string(),
            "database_integration".to_string(),
            "authentication_implementation".to_string(),
            "microservices_architecture".to_string(),
            "performance_optimization".to_string(),
            "security_implementation".to_string(),
            "testing_implementation".to_string(),
            "deployment_configuration".to_string(),
            "monitoring_setup".to_string(),
            "scalability_design".to_string(),
        ]);

        capabilities.insert("refactor-agent".to_string(), vec![
            "code_smell_detection".to_string(),
            "performance_optimization".to_string(),
            "code_quality_improvement".to_string(),
            "design_pattern_application".to_string(),
            "duplicate_code_elimination".to_string(),
            "dead_code_removal".to_string(),
            "dependency_optimization".to_string(),
            "security_vulnerability_fixes".to_string(),
            "maintainability_enhancement".to_string(),
            "automated_refactoring".to_string(),
        ]);

        capabilities.insert("doc-agent".to_string(), vec![
            "automated_doc_generation".to_string(),
            "api_documentation_creation".to_string(),
            "user_guide_development".to_string(),
            "technical_manual_writing".to_string(),
            "code_comment_generation".to_string(),
            "documentation_quality_assessment".to_string(),
            "multi_format_publishing".to_string(),
            "documentation_versioning".to_string(),
            "integration_guide_creation".to_string(),
            "documentation_maintenance".to_string(),
        ]);

        capabilities.insert("deployer-agent".to_string(), vec![
            "deployment_orchestration".to_string(),
            "infrastructure_automation".to_string(),
            "container_management".to_string(),
            "ci_cd_pipeline_creation".to_string(),
            "environment_provisioning".to_string(),
            "zero_downtime_deployment".to_string(),
            "rollback_strategy_design".to_string(),
            "health_monitoring_setup".to_string(),
            "security_compliance_automation".to_string(),
            "scaling_automation".to_string(),
        ]);

        capabilities.insert("maintainer-agent".to_string(), vec![
            "system_health_monitoring".to_string(),
            "performance_optimization".to_string(),
            "security_patch_management".to_string(),
            "database_maintenance".to_string(),
            "log_management".to_string(),
            "backup_recovery_validation".to_string(),
            "capacity_planning".to_string(),
            "incident_response_automation".to_string(),
            "system_upgrade_coordination".to_string(),
            "operational_excellence_optimization".to_string(),
        ]);

        capabilities
    }
} 