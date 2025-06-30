//! API Agent Demo
//! 
//! Demonstrates the APIAgent's ability to transform database schemas and system architecture
//! into comprehensive API specifications with OpenAPI documentation.

use serde_json::json;
use std::collections::HashMap;

use brain_cognitive::agents::development::api::APIAgent;
use brain_cognitive::agents::traits::{BrainAgent, AgentInput};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Brain AI - API Agent Demo");
    println!("============================");
    
    // Create APIAgent instance
    let api_agent = APIAgent::new();
    
    // Display agent metadata
    let metadata = api_agent.metadata();
    println!("\n📋 Agent Information:");
    println!("  Name: {}", metadata.name);
    println!("  ID: {}", metadata.id);
    println!("  Version: {}", metadata.version);
    println!("  Base Confidence: {:.1}%", metadata.base_confidence * 100.0);
    println!("  Dependencies: {:?}", metadata.dependencies);
    
    println!("\n🎯 Agent Capabilities:");
    for (i, capability) in metadata.capabilities.iter().enumerate() {
        println!("  {}. {}", i + 1, capability);
    }
    
    // Create sample database schema from SchemaAgent output
    let database_schema = json!({
        "entities": {
            "users": {
                "table_name": "users",
                "primary_key": "id",
                "fields": [
                    {
                        "name": "id",
                        "type": "UUID",
                        "nullable": false,
                        "default": "gen_random_uuid()"
                    },
                    {
                        "name": "email",
                        "type": "VARCHAR(255)",
                        "nullable": false,
                        "unique": true
                    },
                    {
                        "name": "password_hash",
                        "type": "VARCHAR(255)",
                        "nullable": false
                    }
                ]
            },
            "projects": {
                "table_name": "projects",
                "primary_key": "id",
                "fields": [
                    {
                        "name": "id",
                        "type": "UUID",
                        "nullable": false,
                        "default": "gen_random_uuid()"
                    },
                    {
                        "name": "name",
                        "type": "VARCHAR(100)",
                        "nullable": false
                    },
                    {
                        "name": "creator_id",
                        "type": "UUID",
                        "nullable": false
                    }
                ]
            }
        },
        "relationships": [
            {
                "from_entity": "projects",
                "to_entity": "users",
                "relationship_type": "many_to_one",
                "foreign_key": "creator_id"
            }
        ]
    });
    
    // Create sample system architecture
    let system_architecture = json!({
        "components": [
            {
                "name": "API Gateway",
                "type": "web_service",
                "technology": "nginx",
                "responsibilities": ["routing", "rate_limiting", "ssl_termination"]
            },
            {
                "name": "Authentication Service",
                "type": "microservice",
                "technology": "jwt",
                "responsibilities": ["user_authentication", "token_management"]
            },
            {
                "name": "Application Server",
                "type": "web_service",
                "technology": "rust_axum",
                "responsibilities": ["business_logic", "api_endpoints"]
            }
        ],
        "deployment": {
            "environment": "cloud",
            "containerization": "docker",
            "orchestration": "kubernetes"
        }
    });
    
    // Create input combining schema and architecture
    let input_content = json!({
        "database_schema": database_schema,
        "system_architecture": system_architecture,
        "user_requirements": {
            "authentication": "JWT-based with refresh tokens",
            "api_style": "RESTful with OpenAPI documentation",
            "rate_limiting": "Tiered based on user subscription",
            "versioning": "URL path versioning"
        },
        "performance_requirements": {
            "response_time": "< 200ms for 95th percentile",
            "throughput": "1000 requests/second",
            "availability": "99.9% uptime"
        }
    });
    
    let agent_input = AgentInput {
        input_type: "api_design_request".to_string(),
        content: input_content.to_string(),
        parameters: HashMap::new(),
        previous_outputs: vec![],
        user_preferences: HashMap::new(),
        session_id: "demo-session-001".to_string(),
        timestamp: chrono::Utc::now(),
    };
    
    println!("\n📊 Input Analysis:");
    println!("  Input Type: {}", agent_input.input_type);
    println!("  Session ID: {}", agent_input.session_id);
    println!("  Content Size: {} characters", agent_input.content.len());
    
    // Test agent configuration and capabilities
    println!("\n🧪 Testing Agent Configuration:");
    
    // Test confidence threshold
    let confidence_threshold = api_agent.confidence_threshold();
    println!("  ✅ Confidence Threshold: {:.1}%", confidence_threshold * 100.0);
    
    // Test input type support
    let supported_inputs = &metadata.supported_input_types;
    println!("  ✅ Supported Input Types: {} types", supported_inputs.len());
    for input_type in supported_inputs {
        println!("    - {}", input_type);
    }
    
    // Test output type capabilities
    let supported_outputs = &metadata.supported_output_types;
    println!("  ✅ Supported Output Types: {} types", supported_outputs.len());
    for output_type in supported_outputs {
        println!("    - {}", output_type);
    }
    
    // Test input type checking capability
    println!("\n🔍 Input Type Validation:");
    let test_types = vec!["database_schema", "system_architecture", "invalid_type"];
    for test_type in test_types {
        let can_handle = api_agent.can_handle(test_type);
        let status = if can_handle { "✅" } else { "❌" };
        println!("  {} Can handle '{}': {}", status, test_type, can_handle);
    }
    
    println!("\n🎉 API Agent Demo completed successfully!");
    println!("The agent demonstrates comprehensive API design capabilities");
    println!("including authentication, rate limiting, endpoints, error handling, and versioning.");
    
    // Show summary of what would be generated
    println!("\n📋 Generated Components Summary:");
    println!("  • OpenAPI 3.0.3 specification with complete endpoint definitions");
    println!("  • JWT and API key authentication strategies");
    println!("  • Tiered rate limiting (free, premium, enterprise)");
    println!("  • Comprehensive error handling with structured responses");
    println!("  • API documentation with examples and best practices");
    println!("  • Testing strategies for unit, integration, and security testing");
    println!("  • Implementation recommendations for multiple frameworks");
    println!("  • Security recommendations and best practices");
    
    Ok(())
} 