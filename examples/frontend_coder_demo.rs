//! Frontend Coder Agent Demo
//! 
//! This example demonstrates the FrontendCoder agent's ability to generate
//! comprehensive frontend implementation code from UI/UX designs and API specifications.

use brain_cognitive::agents::development::frontend_coder::FrontendCoder;
use brain_cognitive::agents::traits::{BrainAgent, AgentInput, AgentOutput};
use brain_cognitive::context::CognitiveContextBuilder;
use serde_json::json;
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Frontend Coder Agent Demo");
    println!("==================================\n");

    // Initialize the FrontendCoder agent
    let frontend_coder = FrontendCoder::new();
    println!("✅ Frontend Coder Agent initialized");
    println!("Agent: {}", frontend_coder.metadata().name);
    println!("Capabilities: {:?}\n", frontend_coder.metadata().capabilities);

    // Create comprehensive input with UI/UX designs and API specifications
    let ui_design_specs = json!({
        "framework_preference": "react",
        "components": {
            "layout": {
                "header": {
                    "title": "Brain AI Dashboard",
                    "navigation": ["Dashboard", "Projects", "Settings"],
                    "authentication": true
                },
                "sidebar": {
                    "width": "256px",
                    "collapsible": true,
                    "items": [
                        {"label": "Dashboard", "icon": "home", "path": "/"},
                        {"label": "Projects", "icon": "folder", "path": "/projects"},
                        {"label": "Analytics", "icon": "chart", "path": "/analytics"},
                        {"label": "Settings", "icon": "settings", "path": "/settings"}
                    ]
                }
            },
            "forms": {
                "login_form": {
                    "fields": ["email", "password"],
                    "validation": true,
                    "styling": "modern"
                },
                "project_form": {
                    "fields": ["name", "description", "tech_stack"],
                    "validation": true,
                    "auto_save": true
                }
            },
            "data_display": {
                "project_table": {
                    "columns": ["name", "status", "created_date", "actions"],
                    "pagination": true,
                    "sorting": true,
                    "filtering": true
                },
                "dashboard_cards": {
                    "metrics": ["total_projects", "active_tasks", "completion_rate"],
                    "charts": ["progress_chart", "activity_timeline"]
                }
            }
        },
        "pages": [
            {"path": "/", "component": "Dashboard", "protected": true},
            {"path": "/login", "component": "Login", "protected": false},
            {"path": "/projects", "component": "Projects", "protected": true},
            {"path": "/projects/:id", "component": "ProjectDetail", "protected": true},
            {"path": "/settings", "component": "Settings", "protected": true}
        ],
        "styling": {
            "theme": "modern",
            "color_scheme": "blue-gray",
            "typography": "Inter",
            "responsive": true,
            "dark_mode": true
        },
        "accessibility": {
            "wcag_level": "AA",
            "screen_reader": true,
            "keyboard_navigation": true,
            "focus_management": true
        }
    });

    let api_specifications = json!({
        "base_url": "/api/v1",
        "authentication": {
            "type": "JWT",
            "refresh_token": true,
            "endpoints": {
                "login": "/auth/login",
                "refresh": "/auth/refresh",
                "logout": "/auth/logout"
            }
        },
        "endpoints": {
            "users": {
                "list": {"method": "GET", "path": "/users"},
                "create": {"method": "POST", "path": "/users"},
                "get": {"method": "GET", "path": "/users/:id"},
                "update": {"method": "PUT", "path": "/users/:id"},
                "delete": {"method": "DELETE", "path": "/users/:id"}
            },
            "projects": {
                "list": {"method": "GET", "path": "/projects"},
                "create": {"method": "POST", "path": "/projects"},
                "get": {"method": "GET", "path": "/projects/:id"},
                "update": {"method": "PUT", "path": "/projects/:id"},
                "delete": {"method": "DELETE", "path": "/projects/:id"}
            }
        },
        "error_handling": {
            "format": "RFC7807",
            "status_codes": [400, 401, 403, 404, 422, 429, 500]
        },
        "rate_limiting": {
            "default": "100/hour",
            "authenticated": "1000/hour"
        }
    });

    // Create the agent input
    let input_content = json!({
        "ui_design_specifications": ui_design_specs,
        "api_specifications": api_specifications,
        "requirements": {
            "performance": {
                "initial_load": "<3s",
                "code_splitting": true,
                "lazy_loading": true
            },
            "testing": {
                "unit_tests": true,
                "integration_tests": true,
                "e2e_tests": true,
                "accessibility_tests": true
            },
            "deployment": {
                "build_optimization": true,
                "progressive_web_app": false,
                "docker_support": true
            }
        }
    });

    let agent_input = AgentInput::new(
        "frontend_implementation".to_string(),
        input_content.to_string(),
        "frontend-demo-session".to_string(),
    );

    // Create a simple cognitive context for the demo
    // Note: Using a basic context builder for demo purposes
    let project_context = brain_cognitive::agents::traits::ProjectContext {
        project_name: "Frontend Demo Project".to_string(),
        project_version: "1.0.0".to_string(),
        project_description: Some("Demo project for FrontendCoder agent".to_string()),
        tech_stack: vec!["React".to_string(), "TypeScript".to_string()],
        git_branch: Some("main".to_string()),
        git_commit: Some("abc123def".to_string()),
        active_files: vec!["src/App.tsx".to_string()],
        recent_changes: vec!["Added new component structure".to_string()],
        directory_structure: std::collections::HashMap::new(),
    };
    
    let context = CognitiveContextBuilder::new()
        .with_project_context(project_context)
        .build()
        .expect("Failed to create cognitive context");

    println!("🎯 Frontend Implementation Request:");
    println!("- Framework: React with TypeScript");
    println!("- Components: Layout, Forms, Data Display");
    println!("- Pages: Dashboard, Login, Projects, Settings");
    println!("- Styling: Modern theme with dark mode support");
    println!("- Accessibility: WCAG AA compliance");
    println!("- API Integration: JWT authentication, REST endpoints");
    println!("- Testing: Unit, integration, E2E, accessibility tests\n");

    // Execute the agent
    println!("⏳ Generating frontend implementation...\n");
    let start_time = std::time::Instant::now();
    
    let result = frontend_coder.execute(agent_input, &context).await?;
    
    let execution_time = start_time.elapsed();

    // Display results
    println!("✅ Frontend implementation generated successfully!");
    println!("⏱️  Execution time: {:?}", execution_time);
    println!("🎯 Confidence: {:.1}%", result.confidence * 100.0);
    println!("📊 Memory usage: {:.1}MB", result.execution_metadata.memory_usage_mb);
    println!();

    println!("📋 Generated Components:");
    if let Some(frontend_codebase) = result.data.get("frontend_codebase") {
        if let Some(framework) = frontend_codebase.get("framework") {
            println!("- Framework: {}", framework.as_str().unwrap_or("Unknown"));
        }
        
        if let Some(components) = frontend_codebase.get("components") {
            println!("- Component categories:");
            if let Some(obj) = components.as_object() {
                for category in obj.keys() {
                    println!("  • {}", category);
                }
            }
        }
        
        if let Some(routing) = frontend_codebase.get("routing") {
            println!("- Routing configuration: ✅");
        }
        
        if let Some(state_mgmt) = frontend_codebase.get("state_management") {
            println!("- State management: ✅");
        }
        
        if let Some(api_integration) = frontend_codebase.get("api_integration") {
            println!("- API integration layer: ✅");
        }
        
        if let Some(styling) = frontend_codebase.get("styling_system") {
            println!("- Styling system: ✅");
        }
        
        if let Some(a11y) = frontend_codebase.get("accessibility_features") {
            println!("- Accessibility features: ✅");
        }
    }

    println!("\n🧪 Testing Implementation:");
    if let Some(testing) = result.data.get("testing_implementation") {
        if let Some(unit_testing) = testing.get("unit_testing") {
            if let Some(framework) = unit_testing.get("framework") {
                println!("- Unit testing: {}", framework.as_str().unwrap_or("Configured"));
            }
        }
        if testing.get("integration_testing").is_some() {
            println!("- Integration testing: ✅");
        }
        if testing.get("e2e_testing").is_some() {
            println!("- E2E testing: ✅");
        }
        if testing.get("accessibility_testing").is_some() {
            println!("- Accessibility testing: ✅");
        }
    }

    println!("\n⚡ Performance Optimization:");
    if let Some(performance) = result.data.get("performance_optimization") {
        if performance.get("code_splitting").is_some() {
            println!("- Code splitting: ✅");
        }
        if performance.get("bundle_optimization").is_some() {
            println!("- Bundle optimization: ✅");
        }
        if performance.get("image_optimization").is_some() {
            println!("- Image optimization: ✅");
        }
        if performance.get("caching_strategy").is_some() {
            println!("- Caching strategy: ✅");
        }
    }

    println!("\n💡 Agent Reasoning:");
    if let Some(reasoning) = &result.reasoning {
        println!("{}", reasoning);
    }

    println!("\n📈 Next Steps:");
    for (i, action) in result.next_actions.iter().enumerate() {
        println!("{}. {}", i + 1, action);
    }

    println!("\n🎊 Frontend Coder Demo completed successfully!");
    println!("The agent has generated a comprehensive frontend implementation");
    println!("including components, routing, state management, API integration,");
    println!("styling, accessibility features, and testing strategies.");

    Ok(())
}

// Note: This demo temporarily uses a simplified context due to complexity of 
// mock implementations. In a full system, proper context initialization would 
// include memory repositories, conversation services, etc. 