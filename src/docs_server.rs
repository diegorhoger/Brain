//! Documentation Server Module - Task 7.3
//!
//! This module provides a web server for serving interactive API documentation,
//! including OpenAPI/Swagger UI, tutorials, and examples.

use anyhow::{anyhow, Result};
use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::{info, warn};

/// Documentation server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsConfig {
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Whether to enable CORS
    pub enable_cors: bool,
    /// Path to static documentation files
    pub docs_path: String,
    /// API base URL for examples
    pub api_base_url: String,
    /// Whether to enable interactive examples
    pub enable_examples: bool,
}

impl Default for DocsConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 8081,
            enable_cors: true,
            docs_path: "docs".to_string(),
            api_base_url: "http://localhost:8080/api/v1".to_string(),
            enable_examples: true,
        }
    }
}

/// Documentation server state
#[derive(Clone)]
pub struct DocsServer {
    config: DocsConfig,
}

/// API example for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiExample {
    /// Example title
    pub title: String,
    /// Example description
    pub description: String,
    /// HTTP method
    pub method: String,
    /// API endpoint
    pub endpoint: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body (if applicable)
    pub request_body: Option<serde_json::Value>,
    /// Expected response
    pub expected_response: serde_json::Value,
    /// Response status code
    pub status_code: u16,
    /// Example category
    pub category: String,
}

/// Tutorial content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorial {
    /// Tutorial ID
    pub id: String,
    /// Tutorial title
    pub title: String,
    /// Tutorial description
    pub description: String,
    /// Tutorial content in markdown
    pub content: String,
    /// Difficulty level
    pub difficulty: String,
    /// Estimated time to complete
    pub estimated_time_minutes: u32,
    /// Prerequisites
    pub prerequisites: Vec<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl DocsServer {
    /// Create new documentation server
    pub fn new(config: DocsConfig) -> Self {
        Self { config }
    }

    /// Start the documentation server
    pub async fn start(&self) -> Result<()> {
        let app = self.create_router();

        let addr = SocketAddr::new(
            self.config.bind_address.parse()?,
            self.config.port,
        );

        info!("Starting documentation server on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await
            .map_err(|e| anyhow!("Failed to bind to address {}: {}", addr, e))?;
        
        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| anyhow!("Documentation server failed: {}", e))?;

        Ok(())
    }

    /// Create the router with all routes
    fn create_router(&self) -> Router {
        let mut app = Router::new()
            .route("/", get(serve_index))
            .route("/openapi.yaml", get(serve_openapi_spec))
            .route("/openapi.json", get(serve_openapi_json))
            .route("/swagger-ui", get(serve_swagger_ui))
            .route("/examples", get(list_examples))
            .route("/examples/:category", get(get_examples_by_category))
            .route("/tutorials", get(list_tutorials))
            .route("/tutorials/:id", get(get_tutorial))
            .route("/health", get(health_check))
            .route("/try-api", post(try_api_endpoint));

        // Add CORS if enabled
        if self.config.enable_cors {
            app = app.layer(
                ServiceBuilder::new().layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
            );
        }

        // Serve static files
        app = app.nest_service("/static", ServeDir::new(&self.config.docs_path));

        app
    }
}

/// Serve the main documentation index page
async fn serve_index() -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Brain AI Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h1 { color: #333; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 20px; text-decoration: none; color: #007acc; }
        .nav a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>Brain AI Documentation</h1>
    <div class="nav">
        <a href="/swagger-ui">API Documentation</a>
        <a href="/examples">Examples</a>
        <a href="/tutorials">Tutorials</a>
        <a href="/health">Health Check</a>
    </div>
    <p>Welcome to the Brain AI documentation server. This is a modular AI architecture built with Rust.</p>
    <p>Use the navigation links above to explore the API documentation and examples.</p>
</body>
</html>
    "#)
}

/// Serve the OpenAPI specification in YAML format
async fn serve_openapi_spec() -> impl IntoResponse {
    let spec = include_str!("../docs/api/openapi.yaml");
    Response::builder()
        .header(header::CONTENT_TYPE, "application/x-yaml")
        .body(spec.to_string())
        .unwrap()
}

/// Serve the OpenAPI specification in JSON format
async fn serve_openapi_json() -> impl IntoResponse {
    // Convert YAML to JSON
    let yaml_spec = include_str!("../docs/api/openapi.yaml");
    match serde_yaml::from_str::<serde_json::Value>(yaml_spec) {
        Ok(json_spec) => Json(json_spec).into_response(),
        Err(e) => {
            warn!("Failed to convert OpenAPI spec to JSON: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to convert OpenAPI spec to JSON"
                })),
            )
                .into_response()
        }
    }
}

/// Serve Swagger UI for interactive API documentation
async fn serve_swagger_ui() -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Brain AI API Documentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui.css" />
    <style>
        html {
            box-sizing: border-box;
            overflow: -moz-scrollbars-vertical;
            overflow-y: scroll;
        }
        *, *:before, *:after {
            box-sizing: inherit;
        }
        body {
            margin:0;
            background: #fafafa;
        }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            const ui = SwaggerUIBundle({
                url: '/openapi.json',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            });
        }
    </script>
</body>
</html>
    "#)
}

/// List all available API examples
async fn list_examples() -> impl IntoResponse {
    let examples = get_api_examples();
    Json(examples)
}

/// Get examples by category
async fn get_examples_by_category(Path(category): Path<String>) -> impl IntoResponse {
    let examples = get_api_examples();
    let filtered: Vec<_> = examples
        .into_iter()
        .filter(|example| example.category == category)
        .collect();
    Json(filtered)
}

/// List all available tutorials
async fn list_tutorials() -> impl IntoResponse {
    let tutorials = get_tutorials();
    Json(tutorials)
}

/// Get specific tutorial by ID
async fn get_tutorial(Path(id): Path<String>) -> impl IntoResponse {
    let tutorials = get_tutorials();
    if let Some(tutorial) = tutorials.into_iter().find(|t| t.id == id) {
        Json(tutorial).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Tutorial not found",
                "tutorial_id": id
            })),
        )
            .into_response()
    }
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "brain-ai-docs",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Try API endpoint with live examples
async fn try_api_endpoint(
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {
    // In a real implementation, this would proxy the request to the actual API
    // For now, return a mock response
    Json(serde_json::json!({
        "message": "This is a mock response. In production, this would proxy to the actual API.",
        "request_received": request,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Get predefined API examples
fn get_api_examples() -> Vec<ApiExample> {
    vec![
        ApiExample {
            title: "Text Segmentation".to_string(),
            description: "Break down text into processable segments".to_string(),
            method: "POST".to_string(),
            endpoint: "/segment".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("Authorization".to_string(), "Bearer YOUR_JWT_TOKEN".to_string());
                headers
            },
            request_body: Some(serde_json::json!({
                "text": "The quick brown fox jumps over the lazy dog",
                "max_segments": 10
            })),
            expected_response: serde_json::json!({
                "segments": [
                    {
                        "text": "The",
                        "start": 0,
                        "end": 3,
                        "confidence": 0.95,
                        "segment_type": "word"
                    },
                    {
                        "text": "quick",
                        "start": 4,
                        "end": 9,
                        "confidence": 0.92,
                        "segment_type": "word"
                    }
                ],
                "processing_time_ms": 15,
                "metadata": {
                    "algorithm": "bpe",
                    "total_segments": 2
                }
            }),
            status_code: 200,
            category: "text_processing".to_string(),
        },
        ApiExample {
            title: "Learn Information".to_string(),
            description: "Add new information to the system's knowledge base".to_string(),
            method: "POST".to_string(),
            endpoint: "/learn".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("Authorization".to_string(), "Bearer YOUR_JWT_TOKEN".to_string());
                headers
            },
            request_body: Some(serde_json::json!({
                "information": "User prefers coffee in the morning",
                "priority": "medium",
                "context": {
                    "source": "user_preference",
                    "confidence": 0.8
                }
            })),
            expected_response: serde_json::json!({
                "success": true,
                "memory_id": "mem_12345",
                "concepts_created": 2,
                "rules_generated": 1,
                "processing_time_ms": 45
            }),
            status_code: 200,
            category: "learning".to_string(),
        },
        ApiExample {
            title: "Run Simulation".to_string(),
            description: "Execute a predictive simulation based on a scenario".to_string(),
            method: "POST".to_string(),
            endpoint: "/simulate".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("Authorization".to_string(), "Bearer YOUR_JWT_TOKEN".to_string());
                headers
            },
            request_body: Some(serde_json::json!({
                "scenario": "What happens if the user wakes up early?",
                "max_steps": 5,
                "confidence_threshold": 0.6
            })),
            expected_response: serde_json::json!({
                "outcome": "User will likely have extra time for morning routine and may choose to have a longer breakfast",
                "confidence": 0.75,
                "steps": 3,
                "execution_time_ms": 120,
                "metadata": {
                    "simulation_type": "branching",
                    "factors_considered": ["time_availability", "user_habits", "preferences"]
                }
            }),
            status_code: 200,
            category: "simulation".to_string(),
        },
        ApiExample {
            title: "Query Memory".to_string(),
            description: "Search through the system's memory using natural language".to_string(),
            method: "POST".to_string(),
            endpoint: "/query/memory".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("Authorization".to_string(), "Bearer YOUR_JWT_TOKEN".to_string());
                headers
            },
            request_body: Some(serde_json::json!({
                "query": "coffee preferences",
                "limit": 10,
                "memory_types": ["episodic", "semantic"]
            })),
            expected_response: serde_json::json!({
                "results": [
                    {
                        "content": "User prefers coffee in the morning",
                        "memory_type": "semantic",
                        "relevance": 0.95,
                        "timestamp": "2024-01-15T08:30:00Z",
                        "importance": "medium"
                    }
                ],
                "total_results": 1,
                "query_time_ms": 8
            }),
            status_code: 200,
            category: "querying".to_string(),
        },
        ApiExample {
            title: "Export Data".to_string(),
            description: "Export system data in JSON format".to_string(),
            method: "POST".to_string(),
            endpoint: "/export".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("Authorization".to_string(), "Bearer YOUR_JWT_TOKEN".to_string());
                headers
            },
            request_body: Some(serde_json::json!({
                "format": "json",
                "data_types": ["concepts", "memories"],
                "include_metadata": true
            })),
            expected_response: serde_json::json!({
                "format": "json",
                "data": "eyJjb25jZXB0cyI6W10sIm1lbW9yaWVzIjpbXX0=",
                "metadata": {
                    "export_time": "2024-01-15T10:00:00Z",
                    "total_items": 150,
                    "data_types": ["concepts", "memories"]
                },
                "stats": {
                    "concepts_exported": 75,
                    "memories_exported": 75
                }
            }),
            status_code: 200,
            category: "export".to_string(),
        },
    ]
}

/// Get predefined tutorials
fn get_tutorials() -> Vec<Tutorial> {
    vec![
        Tutorial {
            id: "getting-started".to_string(),
            title: "Getting Started with Brain AI API".to_string(),
            description: "Learn the basics of using the Brain AI API".to_string(),
            content: r#"
# Getting Started with Brain AI API

Welcome to the Brain AI API! This tutorial will guide you through the basic concepts and help you make your first API calls.

## Prerequisites

- Basic understanding of REST APIs
- API key or JWT token for authentication
- HTTP client (curl, Postman, or programming language of choice)

## Step 1: Authentication

Before making any API calls, you need to authenticate. The Brain AI API supports two authentication methods:

### JWT Tokens
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "your_username", "password": "your_password"}'
```

### API Keys
```bash
curl -X GET http://localhost:8080/api/v1/system/status \
  -H "X-API-Key: your_api_key"
```

## Step 2: Your First API Call

Let's start with a simple system status check:

```bash
curl -X GET http://localhost:8080/api/v1/system/status \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Step 3: Text Segmentation

Try segmenting some text:

```bash
curl -X POST http://localhost:8080/api/v1/segment \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"text": "Hello, Brain AI!", "max_segments": 10}'
```

## Next Steps

- Explore the [Advanced Querying Tutorial](#)
- Learn about [Memory Management](#)
- Try [Simulation Examples](#)
            "#.to_string(),
            difficulty: "Beginner".to_string(),
            estimated_time_minutes: 15,
            prerequisites: vec!["Basic REST API knowledge".to_string()],
            tags: vec!["getting-started".to_string(), "authentication".to_string(), "basics".to_string()],
        },
        Tutorial {
            id: "advanced-querying".to_string(),
            title: "Advanced Querying Techniques".to_string(),
            description: "Master the Brain AI query language and advanced search capabilities".to_string(),
            content: r#"
# Advanced Querying Techniques

The Brain AI system provides powerful querying capabilities that allow you to search across concepts, memories, and rules using both natural language and SQL-like syntax.

## Natural Language Queries

Start with simple natural language queries:

```bash
curl -X POST http://localhost:8080/api/v1/query/memory \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"query": "user preferences about food", "limit": 20}'
```

## SQL-like Structured Queries

For more precise control, use the advanced query endpoint:

```bash
curl -X POST http://localhost:8080/api/v1/query/advanced \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"query": "SELECT * FROM concepts WHERE confidence > 0.8 ORDER BY created_at DESC LIMIT 10"}'
```

## Relationship Traversal

Find related concepts using the relationship traversal feature:

```bash
curl -X POST http://localhost:8080/api/v1/query/related-concepts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"concept_name": "coffee", "max_depth": 2, "limit": 15}'
```

## Best Practices

1. **Use specific queries**: More specific queries return more relevant results
2. **Set appropriate limits**: Balance between completeness and performance
3. **Consider confidence thresholds**: Filter results by confidence when accuracy is important
4. **Use relationship traversal**: Discover unexpected connections in your data

## Query Performance Tips

- Index frequently queried fields
- Use filters to reduce result sets
- Cache common queries
- Monitor query performance metrics
            "#.to_string(),
            difficulty: "Intermediate".to_string(),
            estimated_time_minutes: 30,
            prerequisites: vec!["Getting Started Tutorial".to_string(), "SQL basics".to_string()],
            tags: vec!["querying".to_string(), "advanced".to_string(), "sql".to_string(), "relationships".to_string()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_config_default() {
        let config = DocsConfig::default();
        assert_eq!(config.port, 8081);
        assert!(config.enable_cors);
        assert!(config.enable_examples);
    }

    #[test]
    fn test_api_examples_generation() {
        let examples = get_api_examples();
        assert!(!examples.is_empty());
        
        // Check that we have examples for different categories
        let categories: std::collections::HashSet<_> = examples.iter()
            .map(|e| &e.category)
            .collect();
        assert!(categories.contains(&"text_processing".to_string()));
        assert!(categories.contains(&"learning".to_string()));
        assert!(categories.contains(&"querying".to_string()));
    }

    #[test]
    fn test_tutorials_generation() {
        let tutorials = get_tutorials();
        assert!(!tutorials.is_empty());
        
        // Check that tutorials have required fields
        for tutorial in &tutorials {
            assert!(!tutorial.id.is_empty());
            assert!(!tutorial.title.is_empty());
            assert!(!tutorial.content.is_empty());
            assert!(tutorial.estimated_time_minutes > 0);
        }
    }

    #[tokio::test]
    async fn test_health_check_response() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
} 