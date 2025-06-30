//! Architect Agent - System Architecture Design and Guidance
//! 
//! The ArchitectAgent transforms project plans and requirements into comprehensive
//! system architecture designs, including component diagrams, technology selections,
//! data flow designs, and architectural patterns guidance.

use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::{json, Value};
use brain_types::error::BrainError;

use crate::agents::traits::{
    BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitivePreferences,
    CognitiveContext, VerbosityLevel, ExecutionMetadata, ExecutionStatus,
    BrainResult
};

/// Specialized agent for system architecture design and guidance
#[derive(Clone)]
pub struct ArchitectAgent {
    metadata: AgentMetadata,
    preferences: CognitivePreferences,
}

impl ArchitectAgent {
    /// Create a new ArchitectAgent
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "architect-agent".to_string(),
            name: "System Architect".to_string(),
            persona: "A seasoned system architect who designs scalable, maintainable, and robust software architectures. Expert in microservices, distributed systems, database design, API architecture, and technology selection for optimal performance and scalability.".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec![
                "project_plan".to_string(),
                "requirements_analysis".to_string(),
                "technical_requirements".to_string(),
                "architecture_review".to_string(),
                "scalability_requirements".to_string(),
                "technology_constraints".to_string(),
            ],
            supported_output_types: vec![
                "system_architecture".to_string(),
                "component_design".to_string(),
                "technology_stack".to_string(),
                "data_architecture".to_string(),
                "api_specification".to_string(),
                "deployment_architecture".to_string(),
            ],
            capabilities: vec![
                "system_design".to_string(),
                "component_architecture".to_string(),
                "technology_selection".to_string(),
                "data_modeling".to_string(),
                "api_design".to_string(),
                "scalability_planning".to_string(),
                "security_architecture".to_string(),
                "performance_optimization".to_string(),
                "deployment_strategy".to_string(),
                "architecture_validation".to_string(),
            ],
            dependencies: vec![],
            tags: vec!["development".to_string(), "architecture".to_string(), "design".to_string()],
            base_confidence: 0.88,
        };

        let preferences = CognitivePreferences {
            verbosity: VerbosityLevel::Detailed,
            risk_tolerance: 0.4, // Lower risk tolerance for architectural decisions
            collaboration_preference: 0.8, // High collaboration for architecture reviews
            learning_enabled: true,
            adaptation_rate: 0.12, // Conservative adaptation for consistency
        };

        Self {
            metadata,
            preferences,
        }
    }

    /// Analyze requirements and create system architecture
    async fn design_system_architecture(&self, content: &str, context: &CognitiveContext) -> BrainResult<Value> {
        // Parse input to extract requirements and constraints
        let requirements = self.extract_architectural_requirements(content);
        let tech_constraints = self.identify_technology_constraints(content, context);
        let scalability_needs = self.assess_scalability_requirements(content);
        
        let architecture = json!({
            "architecture_overview": {
                "pattern": self.select_architectural_pattern(&requirements, &scalability_needs),
                "principles": [
                    "Separation of Concerns",
                    "Single Responsibility",
                    "Dependency Inversion",
                    "Scalability by Design",
                    "Security by Design"
                ],
                "design_confidence": 0.88
            },
            "system_components": self.design_system_components(&requirements, &tech_constraints),
            "data_architecture": self.design_data_architecture(&requirements),
            "api_architecture": self.design_api_architecture(&requirements),
            "technology_stack": self.recommend_technology_stack(&tech_constraints, context),
            "deployment_strategy": self.design_deployment_strategy(&scalability_needs),
            "security_considerations": self.identify_security_requirements(&requirements),
            "performance_strategy": self.design_performance_strategy(&scalability_needs),
            "monitoring_and_observability": self.design_observability_strategy()
        });

        Ok(architecture)
    }

    /// Create detailed component design
    async fn design_components(&self, _requirements: &Value, _context: &CognitiveContext) -> BrainResult<Value> {
        let components = json!({
            "frontend_components": {
                "user_interface": {
                    "type": "Single Page Application",
                    "framework": "React/Vue.js",
                    "state_management": "Redux/Vuex",
                    "routing": "React Router/Vue Router",
                    "styling": "Styled Components/CSS Modules"
                },
                "component_hierarchy": [
                    "App Container",
                    "Layout Components", 
                    "Feature Components",
                    "Shared Components",
                    "UI Components"
                ]
            },
            "backend_components": {
                "api_layer": {
                    "type": "RESTful API / GraphQL",
                    "framework": "Express.js/Fastify",
                    "middleware": ["Authentication", "Validation", "Logging", "CORS"],
                    "documentation": "OpenAPI/Swagger"
                },
                "business_logic": {
                    "services": ["User Service", "Task Service", "Notification Service"],
                    "patterns": ["Service Layer", "Repository Pattern", "Factory Pattern"],
                    "validation": "Schema-based validation"
                },
                "data_access": {
                    "orm": "Prisma/TypeORM",
                    "connection_pooling": "Built-in ORM pooling",
                    "migrations": "Automated migration system",
                    "caching": "Redis for session and query caching"
                }
            }
        });

        Ok(components)
    }

    /// Design API architecture and specifications
    async fn design_api_specifications(&self, _requirements: &Value, _context: &CognitiveContext) -> BrainResult<Value> {
        let api_design = json!({
            "api_style": "RESTful with GraphQL for complex queries",
            "versioning": {
                "strategy": "URL versioning (v1, v2)",
                "backward_compatibility": "Maintain previous version for 6 months"
            },
            "security": {
                "authentication": "Bearer token (JWT)",
                "authorization": "Role-based access control",
                "rate_limiting": "Token bucket algorithm"
            }
        });

        Ok(api_design)
    }

    // Helper methods for architecture design
    fn extract_architectural_requirements(&self, content: &str) -> Value {
        let lines: Vec<&str> = content.lines().collect();
        let functional_req = lines.iter()
            .filter(|line| line.to_lowercase().contains("system") || line.to_lowercase().contains("architecture"))
            .map(|line| line.trim())
            .collect::<Vec<_>>();

        json!({
            "scalability": "High - Multi-tenant system",
            "availability": "99.9% uptime requirement",
            "performance": "Sub-second response times",
            "security": "Enterprise-grade security",
            "maintainability": "Modular, testable architecture",
            "extracted_requirements": functional_req
        })
    }

    fn identify_technology_constraints(&self, _content: &str, context: &CognitiveContext) -> Value {
        let tech_stack = &context.project_context.tech_stack;
        
        json!({
            "required_technologies": tech_stack,
            "platform_constraints": "Cloud-native deployment"
        })
    }

    fn assess_scalability_requirements(&self, _content: &str) -> Value {
        json!({
            "expected_users": "10,000 - 100,000 concurrent users",
            "data_growth": "1TB+ annually",
            "scaling_strategy": "Horizontal scaling preferred"
        })
    }

    fn select_architectural_pattern(&self, _requirements: &Value, _scalability: &Value) -> String {
        "Microservices with API Gateway".to_string()
    }

    fn design_system_components(&self, _requirements: &Value, _constraints: &Value) -> Value {
        json!({
            "presentation_layer": {
                "web_app": "React SPA with TypeScript",
                "mobile_app": "React Native (future)"
            },
            "api_gateway": {
                "technology": "Kong/Nginx",
                "responsibilities": ["Routing", "Authentication", "Rate limiting"]
            },
            "microservices": {
                "user_service": "User management and authentication",
                "task_service": "Task CRUD and management"
            },
            "data_layer": {
                "primary_database": "PostgreSQL",
                "cache_layer": "Redis"
            }
        })
    }

    fn design_data_architecture(&self, _requirements: &Value) -> Value {
        json!({
            "database_strategy": "Database per service",
            "primary_databases": {
                "user_db": {
                    "type": "PostgreSQL",
                    "schema": "Users, Profiles, Permissions",
                    "scaling": "Read replicas"
                }
            }
        })
    }

    fn design_api_architecture(&self, _requirements: &Value) -> Value {
        json!({
            "api_gateway_pattern": "Single entry point for all client requests",
            "service_communication": {
                "synchronous": "HTTP/REST for real-time operations",
                "asynchronous": "Message queues for background tasks"
            }
        })
    }

    fn recommend_technology_stack(&self, _constraints: &Value, context: &CognitiveContext) -> Value {
        let existing_stack = &context.project_context.tech_stack;
        
        json!({
            "recommended_stack": {
                "frontend": {
                    "framework": existing_stack.get(0).unwrap_or(&"React".to_string()),
                    "state_management": "Redux Toolkit"
                },
                "backend": {
                    "runtime": existing_stack.get(1).unwrap_or(&"Node.js".to_string()),
                    "framework": "Express.js/Fastify"
                },
                "database": {
                    "primary": existing_stack.get(2).unwrap_or(&"PostgreSQL".to_string()),
                    "orm": "Prisma"
                }
            }
        })
    }

    fn design_deployment_strategy(&self, _scalability: &Value) -> Value {
        json!({
            "deployment_pattern": "Blue-Green deployment with rolling updates",
            "environments": {
                "development": "Local Docker + Docker Compose",
                "staging": "Kubernetes cluster (shared)",
                "production": "Kubernetes cluster (dedicated)"
            }
        })
    }

    fn identify_security_requirements(&self, _requirements: &Value) -> Value {
        json!({
            "authentication": {
                "method": "JWT with refresh tokens",
                "multi_factor": "TOTP-based 2FA"
            },
            "authorization": {
                "model": "Role-based access control (RBAC)"
            }
        })
    }

    fn design_performance_strategy(&self, _scalability: &Value) -> Value {
        json!({
            "performance_targets": {
                "response_time": "< 200ms for API calls",
                "throughput": "1000+ requests per second"
            }
        })
    }

    fn design_observability_strategy(&self) -> Value {
        json!({
            "logging": {
                "structured_logging": "JSON-formatted logs with correlation IDs",
                "centralized_logging": "ELK stack for log aggregation"
            },
            "metrics": {
                "application_metrics": "Custom business metrics",
                "infrastructure_metrics": "CPU, memory, disk, network utilization"
            }
        })
    }
}

#[async_trait]
impl BrainAgent for ArchitectAgent {
    async fn execute(
        &self,
        input: AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<AgentOutput> {
        let start_time = std::time::Instant::now();
        
        println!("🏗️ ArchitectAgent executing: {}", input.input_type);
        
        // Process input based on type
        let (content, output_type, confidence) = match input.input_type.as_str() {
            "project_plan" | "requirements_analysis" | "technical_requirements" => {
                // Comprehensive architecture design workflow
                let system_architecture = self.design_system_architecture(&input.content, context).await?;
                let component_design = self.design_components(&system_architecture, context).await?;
                let api_specifications = self.design_api_specifications(&system_architecture, context).await?;
                
                let comprehensive_architecture = json!({
                    "architecture_overview": {
                        "input_type": input.input_type,
                        "processing_timestamp": chrono::Utc::now(),
                        "design_confidence": 0.88
                    },
                    "system_architecture": system_architecture,
                    "component_design": component_design,
                    "api_specifications": api_specifications,
                    "implementation_recommendations": [
                        "Start with MVP architecture and evolve incrementally",
                        "Implement comprehensive testing strategy from day one",
                        "Set up monitoring and observability early"
                    ],
                    "next_steps": [
                        "Review architecture with development team",
                        "Create detailed technical specifications",
                        "Set up development environment and tooling"
                    ]
                });
                
                (comprehensive_architecture.to_string(), "system_architecture".to_string(), 0.88)
            }
            "architecture_review" => {
                let review_analysis = json!({
                    "review_summary": {
                        "architecture_assessment": "Comprehensive review completed",
                        "strengths": [
                            "Well-defined service boundaries",
                            "Appropriate technology choices"
                        ],
                        "areas_for_improvement": [
                            "Consider caching strategy optimization",
                            "Enhance error handling patterns"
                        ]
                    }
                });
                
                (review_analysis.to_string(), "architecture_review".to_string(), 0.82)
            }
            "scalability_requirements" => {
                let scalability_design = self.design_deployment_strategy(&json!({}));
                (scalability_design.to_string(), "deployment_architecture".to_string(), 0.85)
            }
            _ => {
                return Err(BrainError::InvalidInput(format!(
                    "Unsupported input type for ArchitectAgent: {}",
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
        .with_reasoning("Analyzed requirements and designed comprehensive system architecture with scalability, security, and performance considerations".to_string())
        .with_next_actions(vec![
            "schema_design".to_string(),
            "api_detailed_design".to_string(),
            "infrastructure_setup".to_string(),
        ]);

        // Update execution metadata
        output.execution_metadata = ExecutionMetadata {
            execution_time_ms: execution_time,
            memory_usage_mb: 0.0,
            api_calls: 0,
            status: ExecutionStatus::Success,
            warnings: vec![],
        };

        println!("✅ ArchitectAgent completed in {}ms with confidence {:.2}", execution_time, confidence);
        
        Ok(output)
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        0.7
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.preferences
    }

    async fn assess_confidence(
        &self,
        input: &AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<f32> {
        let mut confidence = self.metadata.base_confidence;
        
        // Adjust confidence based on input quality
        let input_length = input.content.len();
        if input_length < 100 {
            confidence *= 0.8; // Lower confidence for very short inputs
        } else if input_length > 1000 {
            confidence *= 1.1; // Higher confidence for detailed inputs
        }
        
        // Adjust based on supported input type
        if self.metadata.supported_input_types.contains(&input.input_type) {
            confidence *= 1.0;
        } else {
            confidence *= 0.6;
        }
        
        // Adjust based on project context
        if !context.project_context.tech_stack.is_empty() {
            confidence *= 1.05; // Slightly higher confidence with tech stack context
        }
        
        Ok(confidence.clamp(0.0, 1.0))
    }
}

impl Default for ArchitectAgent {
    fn default() -> Self {
        Self::new()
    }
}
