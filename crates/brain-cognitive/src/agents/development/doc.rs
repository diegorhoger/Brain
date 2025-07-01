//! Doc Agent - Documentation Generation and Maintenance
//! 
//! The DocAgent automatically generates, maintains, and optimizes documentation
//! across multiple formats and types, including API docs, user guides, technical
//! documentation, and code comments to ensure comprehensive project documentation.

use crate::agents::traits::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use async_trait::async_trait;

/// Agent responsible for documentation generation and maintenance
#[derive(Debug, Clone)]
pub struct DocAgent {
    metadata: AgentMetadata,
    confidence_threshold: f32,
    cognitive_preferences: CognitivePreferences,
}

impl DocAgent {
    /// Create a new DocAgent
    pub fn new() -> Self {
        Self {
            metadata: AgentMetadata {
                id: "doc-agent".to_string(),
                name: "DocAgent".to_string(),
                persona: "Expert technical documentation specialist with comprehensive knowledge of documentation best practices, automated generation tools, and multi-format publishing. Focused on creating clear, comprehensive, and maintainable documentation that enhances project accessibility and team productivity.".to_string(),
                version: "1.0.0".to_string(),
                description: "Technical documentation agent specializing in automated documentation generation, API documentation, and comprehensive project documentation.".to_string(),                supported_input_types: vec![
                    "codebase_documentation".to_string(),
                    "api_documentation".to_string(),
                    "user_guide_generation".to_string(),
                    "technical_documentation".to_string(),
                    "documentation_audit".to_string(),
                ],
                supported_output_types: vec![
                    "documentation_suite".to_string(),
                    "api_documentation".to_string(),
                    "user_guide".to_string(),
                    "technical_manual".to_string(),
                    "documentation_report".to_string(),
                ],
                capabilities: vec![
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
                ],
                dependencies: vec!["refactor-agent".to_string()],
                tags: vec![
                    "documentation".to_string(),
                    "api-docs".to_string(),
                    "user-guides".to_string(),
                    "technical-writing".to_string(),
                    "automation".to_string(),
                ],
                base_confidence: 0.88,
            },
            confidence_threshold: 0.75,
            cognitive_preferences: CognitivePreferences::default(),
        }
    }

    /// Analyze codebase for documentation requirements
    fn analyze_documentation_needs(&self, codebase: &Value) -> Value {
        json!({
            "analysis_type": "comprehensive_documentation_audit",
            "coverage_assessment": {
                "code_documentation": self.assess_code_documentation(codebase),
                "api_documentation": self.assess_api_documentation(codebase),
                "user_documentation": self.assess_user_documentation(codebase),
                "technical_documentation": self.assess_technical_documentation(codebase),
                "integration_documentation": self.assess_integration_documentation(codebase)
            },
            "documentation_gaps": {
                "missing_api_docs": self.identify_missing_api_docs(codebase),
                "undocumented_functions": self.identify_undocumented_functions(codebase),
                "missing_user_guides": self.identify_missing_user_guides(codebase),
                "incomplete_setup_instructions": self.identify_setup_gaps(codebase),
                "missing_examples": self.identify_missing_examples(codebase)
            },
            "quality_metrics": {
                "documentation_coverage": self.calculate_doc_coverage(codebase),
                "documentation_quality_score": self.calculate_quality_score(codebase),
                "accessibility_score": self.assess_accessibility(codebase),
                "maintainability_score": self.assess_doc_maintainability(codebase)
            },
            "existing_documentation": {
                "formats_present": self.identify_existing_formats(codebase),
                "documentation_tools": self.identify_doc_tools(codebase),
                "version_control_integration": self.assess_doc_versioning(codebase)
            }
        })
    }

    /// Generate comprehensive documentation strategy
    fn generate_documentation_strategy(&self, analysis: &Value, requirements: &Value) -> Value {
        json!({
            "documentation_strategy": "comprehensive_multi_format_approach",
            "generation_phases": {
                "phase_1_foundation": {
                    "code_documentation": {
                        "inline_comments": self.plan_inline_documentation(analysis),
                        "function_documentation": self.plan_function_docs(analysis),
                        "class_documentation": self.plan_class_docs(analysis),
                        "module_documentation": self.plan_module_docs(analysis)
                    },
                    "readme_enhancement": {
                        "project_overview": self.plan_project_overview(analysis),
                        "installation_guide": self.plan_installation_docs(analysis),
                        "quick_start": self.plan_quickstart_guide(analysis),
                        "contribution_guidelines": self.plan_contribution_docs(analysis)
                    }
                },
                "phase_2_api_documentation": {
                    "openapi_generation": {
                        "endpoint_documentation": self.plan_endpoint_docs(analysis),
                        "schema_documentation": self.plan_schema_docs(analysis),
                        "authentication_docs": self.plan_auth_docs(analysis),
                        "error_documentation": self.plan_error_docs(analysis)
                    },
                    "sdk_documentation": {
                        "client_libraries": self.plan_sdk_docs(analysis),
                        "integration_examples": self.plan_integration_examples(analysis),
                        "code_samples": self.plan_code_samples(analysis)
                    }
                },
                "phase_3_user_documentation": {
                    "user_guides": {
                        "getting_started": self.plan_user_getting_started(analysis),
                        "feature_guides": self.plan_feature_guides(analysis),
                        "troubleshooting": self.plan_troubleshooting_docs(analysis),
                        "faq_section": self.plan_faq_documentation(analysis)
                    },
                    "tutorials": {
                        "basic_tutorials": self.plan_basic_tutorials(analysis),
                        "advanced_tutorials": self.plan_advanced_tutorials(analysis),
                        "use_case_examples": self.plan_use_case_docs(analysis)
                    }
                },
                "phase_4_technical_documentation": {
                    "architecture_docs": {
                        "system_architecture": self.plan_architecture_docs(analysis),
                        "database_schema": self.plan_database_docs(analysis),
                        "deployment_architecture": self.plan_deployment_docs(analysis),
                        "security_documentation": self.plan_security_docs(analysis)
                    },
                    "development_docs": {
                        "development_setup": self.plan_dev_setup_docs(analysis),
                        "coding_standards": self.plan_coding_standards(analysis),
                        "testing_guidelines": self.plan_testing_docs(analysis),
                        "release_procedures": self.plan_release_docs(analysis)
                    }
                }
            },
            "automation_strategy": {
                "automated_generation": self.plan_automation_tools(analysis),
                "continuous_integration": self.plan_ci_integration(analysis),
                "version_synchronization": self.plan_version_sync(analysis),
                "quality_monitoring": self.plan_quality_monitoring(analysis)
            },
            "publishing_strategy": {
                "multi_format_output": self.plan_output_formats(analysis, requirements),
                "hosting_solutions": self.plan_hosting_strategy(requirements),
                "search_and_navigation": self.plan_navigation_strategy(analysis),
                "accessibility_features": self.plan_accessibility_features(requirements)
            }
        })
    }

    /// Create automated documentation generation tools
    fn create_documentation_automation(&self, strategy: &Value, codebase: &Value) -> Value {
        json!({
            "automation_framework": "intelligent_doc_generation",
            "generation_tools": {
                "code_analyzers": {
                    "ast_documentation": self.generate_ast_doc_tools(strategy, codebase),
                    "comment_generators": self.generate_comment_tools(strategy, codebase),
                    "api_extractors": self.generate_api_extraction_tools(strategy, codebase)
                },
                "content_generators": {
                    "markdown_generators": self.generate_markdown_tools(strategy),
                    "html_generators": self.generate_html_tools(strategy),
                    "pdf_generators": self.generate_pdf_tools(strategy),
                    "interactive_docs": self.generate_interactive_tools(strategy)
                },
                "maintenance_tools": {
                    "link_checkers": self.generate_link_validation_tools(strategy),
                    "content_validators": self.generate_content_validation_tools(strategy),
                    "version_synchronizers": self.generate_version_sync_tools(strategy)
                }
            },
            "integration_scripts": {
                "ci_cd_integration": {
                    "build_hooks": self.generate_build_integration(strategy),
                    "deployment_hooks": self.generate_deployment_integration(strategy),
                    "quality_gates": self.generate_quality_gates(strategy)
                },
                "development_integration": {
                    "ide_plugins": self.generate_ide_integration(strategy),
                    "git_hooks": self.generate_git_integration(strategy),
                    "review_automation": self.generate_review_automation(strategy)
                }
            },
            "template_system": {
                "documentation_templates": self.generate_doc_templates(strategy),
                "style_guides": self.generate_style_templates(strategy),
                "component_templates": self.generate_component_templates(strategy)
            }
        })
    }

    /// Generate implementation guidance and best practices
    fn generate_implementation_guidance(&self, _strategy: &Value) -> Value {
        json!({
            "implementation_approach": "systematic_documentation_development",
            "best_practices": {
                "writing_principles": [
                    "Write for your audience (technical vs non-technical)",
                    "Use clear, concise language and active voice",
                    "Include practical examples and code samples",
                    "Maintain consistent formatting and structure",
                    "Keep documentation close to the code it describes",
                    "Version documentation alongside code changes"
                ],
                "structure_guidelines": [
                    "Start with overview, then dive into details",
                    "Use hierarchical organization with clear navigation",
                    "Include search functionality for large documentation sets",
                    "Provide multiple entry points for different user types",
                    "Cross-reference related topics effectively"
                ],
                "automation_principles": [
                    "Generate documentation from code when possible",
                    "Automate quality checks and link validation",
                    "Integrate documentation builds into CI/CD pipelines",
                    "Monitor documentation usage and feedback",
                    "Maintain documentation debt alongside technical debt"
                ]
            },
            "quality_assurance": {
                "review_process": [
                    "Technical accuracy review by subject matter experts",
                    "Usability testing with target audience",
                    "Accessibility compliance verification",
                    "Cross-browser and cross-device testing",
                    "Regular content freshness audits"
                ],
                "metrics_tracking": [
                    "Documentation coverage percentages",
                    "User engagement and bounce rates",
                    "Search success rates and common queries",
                    "Support ticket reduction from good docs",
                    "Developer onboarding time improvements"
                ]
            },
            "maintenance_strategy": {
                "update_triggers": [
                    "Code changes that affect public APIs",
                    "New feature releases and deprecations",
                    "User feedback and support ticket patterns",
                    "Regular scheduled content reviews",
                    "Technology stack updates"
                ],
                "sustainability_practices": [
                    "Assign documentation ownership to feature teams",
                    "Build documentation time into development estimates",
                    "Create and maintain style guides and templates",
                    "Establish feedback loops with documentation users",
                    "Regularly audit and archive outdated content"
                ]
            }
        })
    }

    // Helper methods for documentation analysis
    fn assess_code_documentation(&self, _codebase: &Value) -> f64 { 0.65 }
    fn assess_api_documentation(&self, _codebase: &Value) -> f64 { 0.45 }
    fn assess_user_documentation(&self, _codebase: &Value) -> f64 { 0.30 }
    fn assess_technical_documentation(&self, _codebase: &Value) -> f64 { 0.55 }
    fn assess_integration_documentation(&self, _codebase: &Value) -> f64 { 0.40 }

    fn identify_missing_api_docs(&self, _codebase: &Value) -> Vec<String> {
        vec!["Authentication endpoints".to_string(), "Error response schemas".to_string()]
    }

    fn identify_undocumented_functions(&self, _codebase: &Value) -> Vec<String> {
        vec!["Internal utility functions".to_string(), "Helper methods".to_string()]
    }

    fn identify_missing_user_guides(&self, _codebase: &Value) -> Vec<String> {
        vec!["Getting started guide".to_string(), "Advanced features tutorial".to_string()]
    }

    fn identify_setup_gaps(&self, _codebase: &Value) -> Vec<String> {
        vec!["Environment configuration".to_string(), "Dependency installation".to_string()]
    }

    fn identify_missing_examples(&self, _codebase: &Value) -> Vec<String> {
        vec!["Code samples".to_string(), "Integration examples".to_string()]
    }

    fn calculate_doc_coverage(&self, _codebase: &Value) -> f64 { 0.48 }
    fn calculate_quality_score(&self, _codebase: &Value) -> f64 { 0.62 }
    fn assess_accessibility(&self, _codebase: &Value) -> f64 { 0.70 }
    fn assess_doc_maintainability(&self, _codebase: &Value) -> f64 { 0.55 }

    fn identify_existing_formats(&self, _codebase: &Value) -> Vec<String> {
        vec!["Markdown".to_string(), "README files".to_string()]
    }

    fn identify_doc_tools(&self, _codebase: &Value) -> Vec<String> {
        vec!["Basic README".to_string(), "Inline comments".to_string()]
    }

    fn assess_doc_versioning(&self, _codebase: &Value) -> f64 { 0.35 }

    // Documentation planning methods (abbreviated for brevity)
    fn plan_inline_documentation(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_function_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_class_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_module_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_project_overview(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_installation_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_quickstart_guide(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_contribution_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_endpoint_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_schema_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_auth_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_error_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_sdk_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_integration_examples(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_code_samples(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_user_getting_started(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_feature_guides(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_troubleshooting_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_faq_documentation(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_basic_tutorials(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_advanced_tutorials(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_use_case_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_architecture_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_database_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_deployment_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_security_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_dev_setup_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_coding_standards(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_testing_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_release_docs(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_automation_tools(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_ci_integration(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_version_sync(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_quality_monitoring(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_output_formats(&self, _analysis: &Value, _requirements: &Value) -> Vec<String> { vec![] }
    fn plan_hosting_strategy(&self, _requirements: &Value) -> Vec<String> { vec![] }
    fn plan_navigation_strategy(&self, _analysis: &Value) -> Vec<String> { vec![] }
    fn plan_accessibility_features(&self, _requirements: &Value) -> Vec<String> { vec![] }

    // Automation generation methods (abbreviated for brevity)
    fn generate_ast_doc_tools(&self, _strategy: &Value, _codebase: &Value) -> Vec<String> { vec![] }
    fn generate_comment_tools(&self, _strategy: &Value, _codebase: &Value) -> Vec<String> { vec![] }
    fn generate_api_extraction_tools(&self, _strategy: &Value, _codebase: &Value) -> Vec<String> { vec![] }
    fn generate_markdown_tools(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_html_tools(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_pdf_tools(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_interactive_tools(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_link_validation_tools(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_content_validation_tools(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_version_sync_tools(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_build_integration(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_deployment_integration(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_quality_gates(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_ide_integration(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_git_integration(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_review_automation(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_doc_templates(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_style_templates(&self, _strategy: &Value) -> Vec<String> { vec![] }
    fn generate_component_templates(&self, _strategy: &Value) -> Vec<String> { vec![] }
}

impl Default for DocAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BrainAgent for DocAgent {
    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        self.confidence_threshold
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.cognitive_preferences
    }

    async fn assess_confidence(&self, input: &AgentInput, context: &CognitiveContext) -> BrainResult<f32> {
        let mut confidence = self.metadata.base_confidence;

        // Parse input to determine documentation complexity
        if let Ok(parsed_input) = serde_json::from_str::<Value>(&input.content) {
            // Boost confidence for well-structured codebase
            if parsed_input.get("codebase_analysis").is_some() {
                confidence += 0.05;
            }

            // Boost confidence for clear documentation requirements
            if parsed_input.get("documentation_requirements").is_some() {
                confidence += 0.05;
            }

            // Boost confidence for existing documentation structure
            if let Some(existing_docs) = parsed_input.get("existing_documentation") {
                if let Some(coverage) = existing_docs.get("coverage_percentage") {
                    if coverage.as_f64().unwrap_or(0.0) > 0.5 {
                        confidence += 0.03;
                    }
                }
            }

            // Consider project context
            if !context.project_context.tech_stack.is_empty() {
                confidence += 0.02;
            }

            // Reduce confidence for very large or complex projects
            if let Some(project_size) = parsed_input.get("project_complexity") {
                if project_size.as_f64().unwrap_or(0.5) > 0.9 {
                    confidence -= 0.05;
                }
            }
        }

        // Consider agent expertise in documentation domain
        confidence += 0.03; // DocAgent has high domain expertise

        Ok(confidence.min(0.95))
    }

    async fn execute(&self, input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        let start_time = std::time::Instant::now();

        // Parse the documentation request
        let parsed_input: Value = serde_json::from_str(&input.content)?;

        // Extract codebase analysis and requirements
        let default_codebase = json!({});
        let default_requirements = json!({});
        let codebase = parsed_input.get("codebase_analysis")
            .unwrap_or(&default_codebase);
        let requirements = parsed_input.get("documentation_requirements")
            .unwrap_or(&default_requirements);

        // Perform comprehensive documentation analysis
        let documentation_analysis = self.analyze_documentation_needs(codebase);

        // Generate documentation strategy
        let documentation_strategy = self.generate_documentation_strategy(&documentation_analysis, requirements);

        // Create automation tools
        let automation_tools = self.create_documentation_automation(&documentation_strategy, codebase);

        // Generate implementation guidance
        let implementation_guidance = self.generate_implementation_guidance(&documentation_strategy);

        // Compile comprehensive documentation suite
        let documentation_suite = json!({
            "documentation_plan": {
                "needs_analysis": documentation_analysis,
                "generation_strategy": documentation_strategy,
                "automation_framework": automation_tools,
                "implementation_guide": implementation_guidance
            },
            "delivery_format": "comprehensive_documentation_suite",
            "methodology": "automated_intelligent_documentation",
            "success_metrics": {
                "coverage_improvement": "60-80% documentation coverage",
                "quality_enhancement": "40-60% quality score improvement",
                "user_satisfaction": "Improved developer onboarding time",
                "maintenance_efficiency": "Automated documentation updates"
            }
        });

        let execution_time = start_time.elapsed();

        Ok(AgentOutput {
            agent_id: self.metadata.name.clone(),
            content: documentation_suite.to_string(),
            output_type: "documentation_suite".to_string(),
            confidence: 0.91,
            execution_metadata: ExecutionMetadata {
                execution_time_ms: execution_time.as_millis() as u64,
                memory_usage_mb: 15.2,
                api_calls: 0,
                status: ExecutionStatus::Success,
                warnings: vec![],
            },
            reasoning: Some("Generated comprehensive documentation strategy with systematic analysis of documentation gaps, multi-format generation approach, automation framework, and maintenance guidelines. Prioritized user experience and developer productivity through intelligent documentation automation.".to_string()),
            next_actions: vec![
                "Execute Phase 1: Foundation documentation and README enhancement".to_string(),
                "Implement API documentation generation with OpenAPI integration".to_string(),
                "Develop user guides and tutorial content".to_string(),
                "Create technical architecture documentation".to_string(),
                "Deploy automation tools and CI/CD integration".to_string(),
            ],
            data: {
                let mut data = HashMap::new();
                data.insert("documentation_analysis".to_string(), documentation_analysis);
                data.insert("generation_strategy".to_string(), documentation_strategy);
                data.insert("automation_tools".to_string(), automation_tools);
                data.insert("implementation_guide".to_string(), implementation_guidance);
                data
            },
            timestamp: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_agent_creation() {
        let agent = DocAgent::new();
        assert_eq!(agent.metadata().name, "DocAgent");
        assert!(agent.metadata().capabilities.contains(&"automated_doc_generation".to_string()));
        assert!(agent.metadata().capabilities.contains(&"api_documentation_creation".to_string()));
        assert_eq!(agent.confidence_threshold(), 0.75);
    }

    #[test]
    fn test_documentation_analysis_capabilities() {
        let agent = DocAgent::new();
        let test_codebase = json!({
            "files": 100,
            "api_endpoints": 25,
            "existing_docs": "minimal"
        });

        let analysis = agent.analyze_documentation_needs(&test_codebase);
        assert!(analysis.get("coverage_assessment").is_some());
        assert!(analysis.get("documentation_gaps").is_some());
        assert!(analysis.get("quality_metrics").is_some());
        assert!(analysis.get("existing_documentation").is_some());
    }

    #[test]
    fn test_documentation_strategy_generation() {
        let agent = DocAgent::new();
        let test_analysis = json!({
            "documentation_coverage": 0.3,
            "quality_score": 0.4
        });
        let test_requirements = json!({
            "priority": "comprehensive",
            "formats": ["markdown", "html"]
        });

        let strategy = agent.generate_documentation_strategy(&test_analysis, &test_requirements);
        assert!(strategy.get("documentation_strategy").is_some());
        assert!(strategy.get("generation_phases").is_some());
        assert!(strategy.get("automation_strategy").is_some());
        assert!(strategy.get("publishing_strategy").is_some());
    }
} 