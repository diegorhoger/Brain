//! Planner Agent - Project Planning and Specification Creation
//! 
//! The PlannerAgent transforms user requirements and ideas into actionable development
//! specifications, breaking down complex projects into manageable tasks and providing
//! architectural guidance for successful project execution.

use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::{json, Value};
use brain_types::error::BrainError;

use crate::agents::traits::{
    BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitivePreferences,
    CognitiveContext, VerbosityLevel, ExecutionMetadata, ExecutionStatus,
    BrainResult
};

/// Specialized agent for project planning and specification creation
#[derive(Clone)]
pub struct PlannerAgent {
    metadata: AgentMetadata,
    preferences: CognitivePreferences,
}

impl PlannerAgent {
    /// Create a new PlannerAgent instance
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "planner-agent".to_string(),
            name: "Project Planner".to_string(),
            persona: "A strategic project planning specialist who transforms ideas into actionable development roadmaps. Expert in breaking down complex requirements into manageable tasks, identifying dependencies, and creating comprehensive project specifications.".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec![
                "project_idea".to_string(),
                "requirements_doc".to_string(),
                "feature_request".to_string(),
                "user_story".to_string(),
                "business_requirements".to_string(),
            ],
            supported_output_types: vec![
                "project_plan".to_string(),
                "task_breakdown".to_string(),
                "technical_spec".to_string(),
                "project_roadmap".to_string(),
                "requirement_analysis".to_string(),
            ],
            capabilities: vec![
                "requirement_analysis".to_string(),
                "task_decomposition".to_string(),
                "dependency_mapping".to_string(),
                "timeline_estimation".to_string(),
                "risk_assessment".to_string(),
                "resource_planning".to_string(),
                "specification_writing".to_string(),
                "stakeholder_analysis".to_string(),
            ],
            dependencies: vec![],
            tags: vec![
                "development".to_string(),
                "planning".to_string(),
                "strategy".to_string(),
                "requirements".to_string(),
            ],
            base_confidence: 0.85,
        };

        let preferences = CognitivePreferences {
            verbosity: VerbosityLevel::Detailed,
            risk_tolerance: 0.6, // Moderate risk tolerance for comprehensive planning
            collaboration_preference: 0.9, // High collaboration for stakeholder alignment
            learning_enabled: true,
            adaptation_rate: 0.15, // Moderate adaptation to maintain planning consistency
        };

        Self { metadata, preferences }
    }

    /// Analyze project requirements and extract key components
    async fn analyze_requirements(&self, content: &str, _context: &CognitiveContext) -> BrainResult<Value> {
        // Extract key information from the requirements
        let mut analysis = HashMap::new();
        
        // Basic requirements parsing (in a real implementation, this would use NLP)
        let requirements = self.extract_requirements(content);
        let stakeholders = self.identify_stakeholders(content);
        let constraints = self.identify_constraints(content);
        let success_criteria = self.define_success_criteria(content);
        
        analysis.insert("requirements", requirements);
        analysis.insert("stakeholders", stakeholders);
        analysis.insert("constraints", constraints);
        analysis.insert("success_criteria", success_criteria);
        analysis.insert("complexity_estimate", self.estimate_complexity(content));
        
        Ok(json!(analysis))
    }

    /// Break down project into actionable tasks
    async fn create_task_breakdown(&self, _requirements: &Value, _context: &CognitiveContext) -> BrainResult<Value> {
        let mut tasks = Vec::new();
        let mut task_id = 1;

        // Phase 1: Project Setup
        tasks.push(json!({
            "id": task_id,
            "title": "Project Initialization",
            "description": "Set up project infrastructure and development environment",
            "phase": "setup",
            "estimated_hours": 8,
            "dependencies": [],
            "subtasks": [
                "Repository setup and branching strategy",
                "Development environment configuration",
                "CI/CD pipeline setup",
                "Documentation framework"
            ],
            "priority": "high"
        }));
        task_id += 1;

        // Phase 2: Architecture & Design
        tasks.push(json!({
            "id": task_id,
            "title": "System Architecture Design",
            "description": "Define system architecture and technical design",
            "phase": "architecture",
            "estimated_hours": 16,
            "dependencies": [1],
            "subtasks": [
                "Technology stack selection",
                "Database schema design",
                "API design and specification",
                "Security architecture planning"
            ],
            "priority": "high"
        }));
        task_id += 1;

        // Phase 3: Core Development
        tasks.push(json!({
            "id": task_id,
            "title": "Core Feature Implementation",
            "description": "Implement primary application features and functionality",
            "phase": "development",
            "estimated_hours": 40,
            "dependencies": [2],
            "subtasks": [
                "Backend API implementation",
                "Frontend component development",
                "Database integration",
                "Business logic implementation"
            ],
            "priority": "high"
        }));
        task_id += 1;

        // Phase 4: Integration & Testing
        tasks.push(json!({
            "id": task_id,
            "title": "Integration and Testing",
            "description": "Comprehensive testing and system integration",
            "phase": "testing",
            "estimated_hours": 24,
            "dependencies": [3],
            "subtasks": [
                "Unit test implementation",
                "Integration testing",
                "End-to-end testing",
                "Performance testing"
            ],
            "priority": "medium"
        }));
        task_id += 1;

        // Phase 5: Deployment & Launch
        tasks.push(json!({
            "id": task_id,
            "title": "Deployment and Launch",
            "description": "Production deployment and go-live activities",
            "phase": "deployment",
            "estimated_hours": 16,
            "dependencies": [4],
            "subtasks": [
                "Production environment setup",
                "Deployment automation",
                "Monitoring and alerting",
                "Launch preparation"
            ],
            "priority": "medium"
        }));

        Ok(json!({
            "tasks": tasks,
            "total_estimated_hours": tasks.iter().map(|t| t["estimated_hours"].as_u64().unwrap_or(0)).sum::<u64>(),
            "phases": ["setup", "architecture", "development", "testing", "deployment"],
            "critical_path": [1, 2, 3, 4, 5]
        }))
    }

    /// Generate comprehensive project roadmap
    async fn create_project_roadmap(&self, task_breakdown: &Value, _context: &CognitiveContext) -> BrainResult<Value> {
        let total_hours = task_breakdown["total_estimated_hours"].as_u64().unwrap_or(0);
        let weeks_estimate = (total_hours as f64 / 40.0).ceil() as u64; // Assuming 40 hours/week
        
        let roadmap = json!({
            "timeline": {
                "estimated_duration_weeks": weeks_estimate,
                "estimated_duration_hours": total_hours,
                "confidence_level": 0.75
            },
            "milestones": [
                {
                    "name": "Project Kickoff",
                    "week": 1,
                    "deliverables": ["Project charter", "Team setup", "Environment ready"]
                },
                {
                    "name": "Architecture Complete",
                    "week": 2,
                    "deliverables": ["Technical specification", "API contracts", "Database schema"]
                },
                {
                    "name": "MVP Development",
                    "week": std::cmp::max(weeks_estimate.saturating_sub(4), 3),
                    "deliverables": ["Core features implemented", "Basic testing complete"]
                },
                {
                    "name": "Production Ready",
                    "week": weeks_estimate,
                    "deliverables": ["Full testing complete", "Deployment ready", "Documentation complete"]
                }
            ],
            "risks": [
                {
                    "description": "Scope creep during development",
                    "impact": "high",
                    "probability": "medium",
                    "mitigation": "Regular stakeholder reviews and change control process"
                },
                {
                    "description": "Technical complexity higher than estimated",
                    "impact": "medium",
                    "probability": "medium",
                    "mitigation": "Architecture review and proof-of-concept development"
                }
            ],
            "success_metrics": [
                "Project delivered on time and within budget",
                "All core requirements implemented",
                "Quality metrics meet defined standards",
                "Stakeholder satisfaction > 90%"
            ]
        });

        Ok(roadmap)
    }

    // Helper methods for requirement analysis
    fn extract_requirements(&self, content: &str) -> Value {
        // Simplified requirement extraction (in practice, would use NLP)
        let lines: Vec<&str> = content.lines().collect();
        let functional_req = lines.iter()
            .filter(|line| line.to_lowercase().contains("must") || line.to_lowercase().contains("should"))
            .map(|line| line.trim())
            .collect::<Vec<_>>();
            
        json!({
            "functional": functional_req,
            "non_functional": ["Performance", "Security", "Scalability", "Usability"],
            "total_count": functional_req.len()
        })
    }

    fn identify_stakeholders(&self, _content: &str) -> Value {
        json!({
            "primary": ["Product Owner", "Development Team", "End Users"],
            "secondary": ["QA Team", "DevOps", "Marketing"],
            "decision_makers": ["Product Owner", "Technical Lead"]
        })
    }

    fn identify_constraints(&self, _content: &str) -> Value {
        json!({
            "timeline": "Project timeline constraints",
            "budget": "Resource and budget limitations",
            "technical": "Technology stack limitations",
            "regulatory": "Compliance requirements"
        })
    }

    fn define_success_criteria(&self, _content: &str) -> Value {
        json!({
            "acceptance_criteria": [
                "All functional requirements implemented",
                "Performance benchmarks met",
                "Security requirements satisfied",
                "User acceptance testing passed"
            ],
            "kpis": [
                "Time to market",
                "Budget adherence",
                "Quality metrics",
                "User satisfaction"
            ]
        })
    }

    fn estimate_complexity(&self, content: &str) -> Value {
        let word_count = content.split_whitespace().count();
        let complexity_score = match word_count {
            0..=100 => "low",
            101..=500 => "medium",
            501..=1000 => "high",
            _ => "very_high"
        };

        json!({
            "score": complexity_score,
            "factors": {
                "requirement_count": word_count / 10, // Rough estimate
                "integration_complexity": "medium",
                "technical_risk": "medium"
            }
        })
    }
}

#[async_trait]
impl BrainAgent for PlannerAgent {
    async fn execute(
        &self,
        input: AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<AgentOutput> {
        let start_time = std::time::Instant::now();
        
        println!("🎯 PlannerAgent executing: {}", input.input_type);
        
        // Process input based on type
        let (content, output_type, confidence) = match input.input_type.as_str() {
            "project_idea" | "requirements_doc" | "feature_request" => {
                // Comprehensive planning workflow
                let requirements_analysis = self.analyze_requirements(&input.content, context).await?;
                let task_breakdown = self.create_task_breakdown(&requirements_analysis, context).await?;
                let roadmap = self.create_project_roadmap(&task_breakdown, context).await?;
                
                let comprehensive_plan = json!({
                    "project_overview": {
                        "input_type": input.input_type,
                        "processing_timestamp": chrono::Utc::now(),
                        "analysis_confidence": 0.85
                    },
                    "requirements_analysis": requirements_analysis,
                    "task_breakdown": task_breakdown,
                    "project_roadmap": roadmap,
                    "recommendations": [
                        "Conduct stakeholder workshop to validate requirements",
                        "Create detailed user stories for development team",
                        "Set up regular sprint planning and review cycles",
                        "Establish clear communication channels and reporting"
                    ],
                    "next_steps": [
                        "Review and approve project plan with stakeholders",
                        "Begin detailed technical architecture design",
                        "Set up project management and tracking tools",
                        "Schedule team kickoff meeting"
                    ]
                });
                
                (comprehensive_plan.to_string(), "project_plan".to_string(), 0.85)
            }
            "user_story" => {
                // User story analysis and breakdown
                let story_analysis = json!({
                    "story_breakdown": {
                        "acceptance_criteria": [
                            "User can perform the requested action",
                            "System validates input appropriately",
                            "Error handling provides clear feedback",
                            "Performance meets requirements"
                        ],
                        "implementation_tasks": [
                            "Frontend UI components",
                            "Backend API endpoints",
                            "Data validation logic",
                            "Testing implementation"
                        ],
                        "estimated_effort": "8-16 hours",
                        "complexity": "medium"
                    }
                });
                
                (story_analysis.to_string(), "task_breakdown".to_string(), 0.80)
            }
            _ => {
                return Err(BrainError::InvalidInput(format!(
                    "Unsupported input type for PlannerAgent: {}",
                    input.input_type
                )));
            }
        };

        let execution_time = start_time.elapsed().as_millis() as u64;
        
        let mut output = AgentOutput::new(
            self.metadata.id.clone(),
            output_type,
            content,
            confidence,
        )
        .with_reasoning("Analyzed requirements and created comprehensive project plan with task breakdown, timeline estimation, and risk assessment".to_string())
        .with_next_actions(vec![
            "architect_review".to_string(),
            "stakeholder_approval".to_string(),
            "technical_design".to_string(),
        ]);

        // Update execution metadata
        output.execution_metadata = ExecutionMetadata {
            execution_time_ms: execution_time,
            memory_usage_mb: 2.5,
            api_calls: 0,
            status: ExecutionStatus::Success,
            warnings: vec![],
        };

        println!("✅ PlannerAgent completed in {}ms with confidence {:.2}", 
                 execution_time, confidence);

        Ok(output)
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        0.70 // Require high confidence for planning decisions
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.preferences
    }

    async fn assess_confidence(
        &self,
        input: &AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<f32> {
        // Assess confidence based on input clarity and available context
        let mut confidence = self.metadata.base_confidence;
        
        // Adjust based on input completeness
        let content_length = input.content.len();
        if content_length < 50 {
            confidence -= 0.2; // Low confidence for very brief inputs
        } else if content_length > 500 {
            confidence += 0.1; // Higher confidence for detailed inputs
        }
        
        // Adjust based on context availability
        if !context.session_history.is_empty() {
            confidence += 0.05; // Slight boost for ongoing context
        }
        
        // Adjust based on cognitive profile preferences
        if context.cognitive_profile.detail_level == crate::agents::traits::DetailLevel::Comprehensive {
            confidence += 0.05; // Higher confidence when detailed analysis is preferred
        }
        
        Ok(confidence.clamp(0.0, 1.0))
    }
}

impl Default for PlannerAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planner_agent_creation() {
        let agent = PlannerAgent::new();
        assert_eq!(agent.metadata().name, "Project Planner");
        assert!(agent.metadata().capabilities.contains(&"requirement_analysis".to_string()));
        assert!(agent.can_handle("project_idea"));
    }

    // Note: Integration tests requiring CognitiveContext are temporarily disabled
    // until mock implementations are properly set up for MetaMemoryRepository trait
} 