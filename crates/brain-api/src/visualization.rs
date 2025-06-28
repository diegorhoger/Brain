//! Visualization API Module
//!
//! This module provides web-based visualization capabilities for the Brain AI system,
//! including interactive concept graph exploration, memory timeline visualization,
//! and simulation results dashboards.

use axum::{
    extract::Query,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Visualization server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    /// Whether to enable concept graph visualization
    pub enable_concept_graph: bool,
    /// Whether to enable memory timeline visualization
    pub enable_memory_timeline: bool,
    /// Whether to enable simulation dashboard
    pub enable_simulation_dashboard: bool,
    /// Maximum number of nodes to display in graph
    pub max_graph_nodes: usize,
    /// Default graph layout algorithm
    pub default_layout: String,
    /// Enable interactive features
    pub enable_interactions: bool,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            enable_concept_graph: true,
            enable_memory_timeline: true,
            enable_simulation_dashboard: true,
            max_graph_nodes: 1000,
            default_layout: "force".to_string(),
            enable_interactions: true,
        }
    }
}

/// Node data structure for D3.js graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationNode {
    /// Unique node identifier
    pub id: String,
    /// Node display name
    pub name: String,
    /// Node type for visual styling
    pub node_type: String,
    /// Node size based on importance/connections
    pub size: f64,
    /// Node color category
    pub color: String,
    /// Additional metadata for tooltips
    pub metadata: HashMap<String, serde_json::Value>,
    /// Position hints for layout
    pub x: Option<f64>,
    pub y: Option<f64>,
    /// Connection count for sizing
    pub degree: usize,
    /// Confidence score (0-1)
    pub confidence: f64,
}

/// Edge data structure for D3.js graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationEdge {
    /// Source node ID
    pub source: String,
    /// Target node ID
    pub target: String,
    /// Edge weight/strength
    pub weight: f64,
    /// Edge type for styling
    pub edge_type: String,
    /// Edge color
    pub color: String,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Complete graph data for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    /// All nodes in the graph
    pub nodes: Vec<VisualizationNode>,
    /// All edges in the graph
    pub edges: Vec<VisualizationEdge>,
    /// Graph metadata
    pub metadata: GraphMetadata,
}

/// Graph metadata for visualization context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    /// Total number of nodes
    pub node_count: usize,
    /// Total number of edges
    pub edge_count: usize,
    /// Graph generation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Graph type identifier
    pub graph_type: String,
    /// Layout algorithm used
    pub layout_algorithm: String,
    /// Filters applied
    pub filters: HashMap<String, serde_json::Value>,
}

/// Visualization manager for coordinating visualization services
pub struct VisualizationManager {
    #[allow(dead_code)]
    config: VisualizationConfig,
}

impl VisualizationManager {
    /// Create a new visualization manager
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }

    /// Create the visualization router with all endpoints
    pub fn create_router(&self) -> Router {
        Router::new()
            .route("/api/graph", get(get_concept_graph_data))
            .route("/api/graph/filtered", get(get_filtered_concept_graph))
            .route("/api/timeline", get(get_memory_timeline_data))
            .route("/api/timeline/filtered", get(get_filtered_memory_timeline))
            .route("/api/dashboard", get(get_simulation_dashboard_data))
            .route("/api/dashboard/filtered", get(get_filtered_simulation_dashboard))
            .route("/graph", get(serve_concept_graph_page))
            .route("/timeline", get(serve_memory_timeline_page))
            .route("/dashboard", get(serve_simulation_dashboard_page))
    }
}

/// Query parameters for graph filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQueryParams {
    /// Filter by concept type
    pub concept_type: Option<String>,
    /// Minimum confidence threshold
    pub min_confidence: Option<f64>,
    /// Maximum number of nodes
    pub limit: Option<usize>,
    /// Search term for concept names
    pub search: Option<String>,
}

/// API endpoint handlers
async fn get_concept_graph_data() -> impl IntoResponse {
    info!("Generating concept graph data");
    Json(create_sample_graph_data())
}

async fn get_filtered_concept_graph(Query(params): Query<GraphQueryParams>) -> impl IntoResponse {
    debug!("Filtering concept graph with params: {:?}", params);
    Json(create_sample_graph_data())
}

async fn get_memory_timeline_data() -> impl IntoResponse {
    info!("Generating memory timeline data");
    Json(create_sample_timeline_data())
}

async fn get_filtered_memory_timeline(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    Json(create_sample_timeline_data())
}

async fn get_simulation_dashboard_data() -> impl IntoResponse {
    info!("Generating simulation dashboard data");
    Json(create_comprehensive_dashboard_data())
}

async fn get_filtered_simulation_dashboard(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    Json(create_comprehensive_dashboard_data())
}

async fn serve_concept_graph_page() -> impl IntoResponse {
    Html(include_str!("../../../web/concept_graph.html"))
}

async fn serve_memory_timeline_page() -> impl IntoResponse {
    Html(include_str!("../../../web/memory_timeline.html"))
}

async fn serve_simulation_dashboard_page() -> impl IntoResponse {
    Html(include_str!("../../../web/simulation_dashboard.html"))
}

// Sample data generation functions (placeholder implementations)
fn create_sample_graph_data() -> GraphData {
    let nodes = vec![
        VisualizationNode {
            id: "concept_1".to_string(),
            name: "Machine Learning".to_string(),
            node_type: "concept".to_string(),
            size: 15.0,
            color: "#4A90E2".to_string(),
            metadata: HashMap::new(),
            x: Some(100.0),
            y: Some(100.0),
            degree: 5,
            confidence: 0.95,
        },
        VisualizationNode {
            id: "concept_2".to_string(),
            name: "Neural Networks".to_string(),
            node_type: "concept".to_string(),
            size: 12.0,
            color: "#7ED321".to_string(),
            metadata: HashMap::new(),
            x: Some(200.0),
            y: Some(150.0),
            degree: 3,
            confidence: 0.88,
        },
    ];

    let edges = vec![
        VisualizationEdge {
            source: "concept_1".to_string(),
            target: "concept_2".to_string(),
            weight: 0.8,
            edge_type: "related".to_string(),
            color: "#999999".to_string(),
            metadata: HashMap::new(),
        },
    ];

    GraphData {
        nodes,
        edges,
        metadata: GraphMetadata {
            node_count: 2,
            edge_count: 1,
            timestamp: chrono::Utc::now(),
            graph_type: "concept_graph".to_string(),
            layout_algorithm: "force".to_string(),
            filters: HashMap::new(),
        },
    }
}

fn create_sample_timeline_data() -> serde_json::Value {
    serde_json::json!({
        "events": [
            {
                "id": "event_1",
                "timestamp": chrono::Utc::now(),
                "title": "System Initialization",
                "description": "Brain AI system started",
                "event_type": "system",
                "importance": 0.8,
                "related_concepts": ["initialization", "startup"],
                "metadata": {}
            }
        ],
        "metadata": {
            "event_count": 1,
            "start_time": chrono::Utc::now(),
            "end_time": chrono::Utc::now(),
            "timestamp": chrono::Utc::now(),
            "filters": {}
        }
    })
}

fn create_comprehensive_dashboard_data() -> serde_json::Value {
    serde_json::json!({
        "statistics": {
            "total_simulations": 150,
            "average_confidence": 0.75,
            "success_rate": 0.92,
            "total_branches_explored": 1200,
            "average_branches_per_simulation": 8.0,
            "common_outcomes": [
                {
                    "outcome": "Successful completion",
                    "frequency": 138,
                    "percentage": 92.0
                }
            ],
            "confidence_distribution": {
                "high_confidence": 90,
                "medium_confidence": 45,
                "low_confidence": 15
            }
        },
        "recent_simulations": [],
        "rule_insights": {
            "total_rules": 250,
            "active_rules": 200,
            "top_rules": [],
            "highest_confidence_rules": [],
            "recent_rules": [],
            "rule_performance": {
                "overall_success_rate": 0.85,
                "average_confidence": 0.78,
                "average_support": 0.65,
                "deprecated_rules": 10,
                "recent_rule_creation_rate": 5
            },
            "pattern_distribution": []
        },
        "performance_metrics": {
            "average_execution_time_ms": 250.0,
            "fastest_simulation_ms": 50,
            "slowest_simulation_ms": 1200,
            "memory_usage": {
                "average_memory_mb": 128.0,
                "peak_memory_mb": 256.0,
                "efficiency_score": 0.85
            },
            "resource_utilization": {
                "cpu_utilization": 45.0,
                "memory_utilization": 60.0,
                "throughput": 12.0
            }
        },
        "metadata": {
            "generated_at": chrono::Utc::now(),
            "data_freshness_minutes": 5,
            "data_sources": 3,
            "version": "1.0.0",
            "applied_filters": {}
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualization_config_default() {
        let config = VisualizationConfig::default();
        assert!(config.enable_concept_graph);
        assert!(config.enable_memory_timeline);
        assert!(config.enable_simulation_dashboard);
        assert_eq!(config.max_graph_nodes, 1000);
    }

    #[test]
    fn test_sample_graph_data_generation() {
        let graph_data = create_sample_graph_data();
        assert_eq!(graph_data.nodes.len(), 2);
        assert_eq!(graph_data.edges.len(), 1);
        assert_eq!(graph_data.metadata.node_count, 2);
        assert_eq!(graph_data.metadata.edge_count, 1);
    }

    #[test]
    fn test_visualization_manager_creation() {
        let config = VisualizationConfig::default();
        let manager = VisualizationManager::new(config);
        let _router = manager.create_router();
        // If we get here without panicking, router creation succeeded
    }
} 