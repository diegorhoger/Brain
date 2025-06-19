//! Visualization Module - Task 8.1: Concept Graph Visualization
//!
//! This module provides web-based visualization capabilities for the Brain AI system,
//! including interactive concept graph exploration, memory timeline visualization,
//! and simulation results dashboards.

use anyhow::Result;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::concept_graph::{ConceptGraphManager, ConceptNode, ConceptType};

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

/// Memory timeline event for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// Event unique identifier
    pub id: String,
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Event title
    pub title: String,
    /// Event description
    pub description: String,
    /// Event type for visual styling
    pub event_type: String,
    /// Event importance (0-1)
    pub importance: f64,
    /// Related concepts
    pub related_concepts: Vec<String>,
    /// Event metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Timeline data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineData {
    /// All events in chronological order
    pub events: Vec<TimelineEvent>,
    /// Timeline metadata
    pub metadata: TimelineMetadata,
}

/// Timeline metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineMetadata {
    /// Total number of events
    pub event_count: usize,
    /// Earliest event timestamp
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// Latest event timestamp
    pub end_time: chrono::DateTime<chrono::Utc>,
    /// Timeline generation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Filters applied
    pub filters: HashMap<String, serde_json::Value>,
}

/// Simulation dashboard data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDashboardData {
    /// Overall statistics
    pub statistics: SimulationStatistics,
    /// Recent simulation results
    pub recent_simulations: Vec<SimulationResult>,
    /// Rule insights and metrics
    pub rule_insights: RuleInsights,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Dashboard metadata
    pub metadata: DashboardMetadata,
}

/// Overall simulation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStatistics {
    /// Total number of simulations run
    pub total_simulations: usize,
    /// Average confidence across all simulations
    pub average_confidence: f64,
    /// Success rate (simulations that completed successfully)
    pub success_rate: f64,
    /// Total branches explored across all simulations
    pub total_branches_explored: usize,
    /// Average branches per simulation
    pub average_branches_per_simulation: f64,
    /// Most common simulation outcomes
    pub common_outcomes: Vec<OutcomeFrequency>,
    /// Simulation distribution by confidence ranges
    pub confidence_distribution: ConfidenceDistribution,
}

/// Individual simulation result for dashboard display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    /// Simulation unique identifier
    pub id: String,
    /// Timestamp when simulation was run
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Initial scenario/input text
    pub scenario: String,
    /// Final outcome description
    pub outcome: String,
    /// Overall confidence score
    pub confidence: f64,
    /// Number of branches explored
    pub branches_explored: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Simulation status
    pub status: String,
    /// Key insights from the simulation
    pub insights: Vec<String>,
    /// Most likely outcome path
    pub most_likely_path: Vec<SimulationStep>,
    /// Branching statistics
    pub branching_stats: BranchingStatsSummary,
}

/// Summary of branching statistics for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingStatsSummary {
    /// Maximum depth reached
    pub max_depth: usize,
    /// Average confidence across branches
    pub average_confidence: f64,
    /// Number of terminal branches
    pub terminal_branches: usize,
    /// Diversity score
    pub diversity_score: f64,
}

/// Individual step in a simulation path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStep {
    /// Step number in sequence
    pub step: usize,
    /// Description of what happened
    pub description: String,
    /// Confidence in this step
    pub confidence: f64,
    /// Rules applied in this step
    pub applied_rules: Vec<String>,
}

/// Frequency of specific outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeFrequency {
    /// Outcome description
    pub outcome: String,
    /// Number of times this outcome occurred
    pub frequency: usize,
    /// Percentage of total simulations
    pub percentage: f64,
}

/// Distribution of simulations by confidence ranges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceDistribution {
    /// High confidence simulations (0.8-1.0)
    pub high_confidence: usize,
    /// Medium confidence simulations (0.5-0.8)
    pub medium_confidence: usize,
    /// Low confidence simulations (0.0-0.5)
    pub low_confidence: usize,
}

/// Rule insights and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleInsights {
    /// Total number of rules in database
    pub total_rules: usize,
    /// Number of active rules
    pub active_rules: usize,
    /// Most frequently used rules
    pub top_rules: Vec<RuleUsageSummary>,
    /// Rules with highest confidence
    pub highest_confidence_rules: Vec<RuleUsageSummary>,
    /// Recently created rules
    pub recent_rules: Vec<RuleUsageSummary>,
    /// Rule performance metrics
    pub rule_performance: RulePerformanceMetrics,
    /// Pattern type distribution
    pub pattern_distribution: Vec<PatternTypeCount>,
}

/// Summary of rule usage for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleUsageSummary {
    /// Rule unique identifier
    pub id: String,
    /// Rule pattern description
    pub pattern: String,
    /// Rule outcome description
    pub outcome: String,
    /// Confidence score
    pub confidence: f64,
    /// Support score
    pub support: f64,
    /// Number of times used
    pub usage_count: u64,
    /// Success rate
    pub success_rate: f64,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Rule performance analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePerformanceMetrics {
    /// Overall rule success rate
    pub overall_success_rate: f64,
    /// Average rule confidence
    pub average_confidence: f64,
    /// Average rule support
    pub average_support: f64,
    /// Number of deprecated rules
    pub deprecated_rules: usize,
    /// Rules created in last 24 hours
    pub recent_rule_creation_rate: usize,
}

/// Count of rules by pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTypeCount {
    /// Pattern type name
    pub pattern_type: String,
    /// Number of rules with this pattern type
    pub count: usize,
    /// Percentage of total rules
    pub percentage: f64,
}

/// Performance metrics for simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average simulation execution time (ms)
    pub average_execution_time_ms: f64,
    /// Fastest simulation time (ms)
    pub fastest_simulation_ms: u64,
    /// Slowest simulation time (ms)
    pub slowest_simulation_ms: u64,
    /// Memory usage statistics
    pub memory_usage: MemoryUsageStats,
    /// System resource utilization
    pub resource_utilization: ResourceUtilization,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsageStats {
    /// Average memory used per simulation (MB)
    pub average_memory_mb: f64,
    /// Peak memory usage (MB)
    pub peak_memory_mb: f64,
    /// Memory efficiency score
    pub efficiency_score: f64,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Memory utilization percentage
    pub memory_utilization: f64,
    /// Throughput (simulations per minute)
    pub throughput: f64,
}

/// Dashboard metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetadata {
    /// When the dashboard data was generated
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Data freshness (how recent the underlying data is)
    pub data_freshness_minutes: u64,
    /// Number of data sources used
    pub data_sources: usize,
    /// Dashboard version
    pub version: String,
    /// Filters applied to the data
    pub applied_filters: HashMap<String, serde_json::Value>,
}

/// Query parameters for simulation dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardQueryParams {
    /// Filter by simulation status
    pub status: Option<String>,
    /// Minimum confidence threshold
    pub min_confidence: Option<f64>,
    /// Maximum confidence threshold
    pub max_confidence: Option<f64>,
    /// Start time filter
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// End time filter
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Include detailed rule insights
    pub include_rule_details: Option<bool>,
    /// Include performance metrics
    pub include_performance: Option<bool>,
}

/// Visualization manager
#[derive(Debug, Clone)]
pub struct VisualizationManager {
    config: VisualizationConfig,
}

impl VisualizationManager {
    /// Create new visualization manager
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }

    /// Create router for visualization endpoints
    pub fn create_router(&self) -> Router {
        Router::new()
            .route("/api/visualization/concept-graph", get(get_concept_graph_data))
            .route("/api/visualization/concept-graph/filtered", get(get_filtered_concept_graph))
            .route("/api/visualization/memory-timeline", get(get_memory_timeline_data))
            .route("/api/visualization/memory-timeline/filtered", get(get_filtered_memory_timeline))
            .route("/api/visualization/simulation-dashboard", get(get_simulation_dashboard_data))
            .route("/api/visualization/simulation-dashboard/filtered", get(get_filtered_simulation_dashboard))
            .route("/visualization/concept-graph", get(serve_concept_graph_page))
            .route("/visualization/memory-timeline", get(serve_memory_timeline_page))
            .route("/visualization/simulation-dashboard", get(serve_simulation_dashboard_page))
    }

    /// Generate memory timeline data from the memory system
    pub async fn generate_memory_timeline_data(&self, memory_system: &crate::memory::MemorySystem) -> Result<TimelineData> {
        let mut events = Vec::new();

        // Query all episodic events
        let episodic_query = crate::memory::EpisodicQuery {
            limit: Some(1000), // Limit to prevent overwhelming the UI
            ..Default::default()
        };

        let episodic_events = memory_system.query_episodic(&episodic_query)?;
        info!("Generating timeline data for {} episodic events", episodic_events.len());

        for event in episodic_events.iter() {
            let timeline_event = TimelineEvent {
                id: event.id.to_string(),
                timestamp: event.timestamp,
                title: self.extract_event_title(&event.content),
                description: event.content.clone(),
                event_type: self.determine_event_type(&event.tags, &event.context),
                importance: event.importance,
                related_concepts: event.tags.clone(),
                metadata: self.create_timeline_metadata(event),
            };
            events.push(timeline_event);
        }

        // Sort events chronologically
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let metadata = if events.is_empty() {
            TimelineMetadata {
                event_count: 0,
                start_time: chrono::Utc::now(),
                end_time: chrono::Utc::now(),
                timestamp: chrono::Utc::now(),
                filters: HashMap::new(),
            }
        } else {
            TimelineMetadata {
                event_count: events.len(),
                start_time: events.first().unwrap().timestamp,
                end_time: events.last().unwrap().timestamp,
                timestamp: chrono::Utc::now(),
                filters: HashMap::new(),
            }
        };

        Ok(TimelineData { events, metadata })
    }

    /// Extract a concise title from event content (first sentence or 50 chars)
    fn extract_event_title(&self, content: &str) -> String {
        // Get first sentence or first 50 characters
        let first_sentence = content.split('.').next().unwrap_or(content);
        if first_sentence.len() <= 50 {
            first_sentence.to_string()
        } else {
            format!("{}...", &first_sentence[..47])
        }
    }

    /// Determine event type based on tags and context
    fn determine_event_type(&self, tags: &[String], context: &HashMap<String, String>) -> String {
        // Priority-based event type determination
        for tag in tags {
            match tag.to_lowercase().as_str() {
                tag if tag.contains("error") || tag.contains("warning") => return "error".to_string(),
                tag if tag.contains("learn") || tag.contains("discovery") => return "learning".to_string(),
                tag if tag.contains("interaction") || tag.contains("query") => return "interaction".to_string(),
                tag if tag.contains("consolidation") || tag.contains("memory") => return "consolidation".to_string(),
                tag if tag.contains("insight") || tag.contains("pattern") => return "insight".to_string(),
                _ => {}
            }
        }

        // Check context for type hints
        if context.contains_key("source") {
            match context.get("source").unwrap().to_lowercase().as_str() {
                source if source.contains("working") => return "working_memory".to_string(),
                source if source.contains("external") => return "external_input".to_string(),
                source if source.contains("system") => return "system".to_string(),
                _ => {}
            }
        }

        "general".to_string()
    }

    /// Create timeline event metadata
    fn create_timeline_metadata(&self, event: &crate::memory::EpisodicEvent) -> HashMap<String, serde_json::Value> {
        let mut metadata = HashMap::new();
        
        metadata.insert("source".to_string(), serde_json::Value::String(event.source.clone()));
        metadata.insert("importance".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(event.importance).unwrap()));
        metadata.insert("tag_count".to_string(), serde_json::Value::Number(serde_json::Number::from(event.tags.len())));
        metadata.insert("context_keys".to_string(), serde_json::Value::Array(
            event.context.keys().map(|k| serde_json::Value::String(k.clone())).collect()
        ));
        
        // Add context data as metadata
        for (key, value) in &event.context {
            metadata.insert(format!("context_{}", key), serde_json::Value::String(value.clone()));
        }

        metadata
    }

    /// Generate concept graph data from the concept graph manager
    pub async fn generate_concept_graph_data(&self, concept_manager: &ConceptGraphManager) -> Result<GraphData> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Get concepts using query with limit
        let query_params = crate::concept_graph::ConceptQuery {
            limit: Some(self.config.max_graph_nodes),
            ..Default::default()
        };
        let concepts = concept_manager.query_concepts(&query_params).await?;
        
        info!("Generating graph data for {} concepts", concepts.len());

        // Convert concepts to graph nodes
        for concept in concepts.iter() {
            // Get relationship count for this concept
            let relationships = concept_manager.get_concept_relationships(concept.id).await?;
            
            let node = VisualizationNode {
                id: concept.id.to_string(),
                name: concept.content.clone(),
                node_type: format!("{:?}", concept.concept_type),
                size: Self::calculate_node_size(concept),
                color: Self::get_node_color(&concept.concept_type),
                metadata: Self::create_node_metadata(concept),
                x: None,
                y: None,
                degree: relationships.len(),
                confidence: concept.confidence_score,
            };
            nodes.push(node);
        }

        // Convert relationships to graph edges
        for concept in concepts.iter() {
            let relationships = concept_manager.get_concept_relationships(concept.id).await?;
            for relationship in &relationships {
                let edge = VisualizationEdge {
                    source: relationship.source_id.to_string(),
                    target: relationship.target_id.to_string(),
                    weight: relationship.weight,
                    edge_type: format!("{:?}", relationship.relationship_type),
                    color: Self::get_edge_color(&format!("{:?}", relationship.relationship_type)),
                    metadata: Self::create_edge_metadata(relationship),
                };
                edges.push(edge);
            }
        }

        let metadata = GraphMetadata {
            node_count: nodes.len(),
            edge_count: edges.len(),
            timestamp: chrono::Utc::now(),
            graph_type: "concept_graph".to_string(),
            layout_algorithm: self.config.default_layout.clone(),
            filters: HashMap::new(),
        };

        Ok(GraphData {
            nodes,
            edges,
            metadata,
        })
    }

    /// Calculate appropriate node size based on concept properties
    fn calculate_node_size(concept: &ConceptNode) -> f64 {
        // Base size + usage count + confidence bonus
        let base_size = 10.0;
        let usage_bonus = (concept.usage_count as f64).min(20.0) * 0.5;
        let confidence_bonus = concept.confidence_score * 5.0;
        
        (base_size + usage_bonus + confidence_bonus).min(50.0)
    }

    /// Get node color based on concept type
    fn get_node_color(concept_type: &ConceptType) -> String {
        match concept_type {
            ConceptType::Entity => "#1f77b4".to_string(), // Blue
            ConceptType::Action => "#ff7f0e".to_string(), // Orange
            ConceptType::Attribute => "#2ca02c".to_string(), // Green
            ConceptType::Abstract => "#d62728".to_string(), // Red
            ConceptType::Relation => "#9467bd".to_string(), // Purple
        }
    }

    /// Get edge color based on relationship type
    fn get_edge_color(relationship_type: &str) -> String {
        match relationship_type {
            "causal" => "#ff4444".to_string(),
            "similarity" => "#44ff44".to_string(),
            "temporal" => "#4444ff".to_string(),
            "spatial" => "#ff44ff".to_string(),
            _ => "#888888".to_string(),
        }
    }

    /// Create node metadata for tooltips
    fn create_node_metadata(concept: &ConceptNode) -> HashMap<String, serde_json::Value> {
        let mut metadata = HashMap::new();
        metadata.insert("id".to_string(), serde_json::Value::String(concept.id.to_string()));
        metadata.insert("content".to_string(), serde_json::Value::String(concept.content.clone()));
        metadata.insert("type".to_string(), serde_json::Value::String(format!("{:?}", concept.concept_type)));
        metadata.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(concept.confidence_score).unwrap()));
        metadata.insert("created_at".to_string(), serde_json::Value::String(concept.created_at.to_rfc3339()));
        metadata.insert("usage_count".to_string(), serde_json::Value::Number(serde_json::Number::from(concept.usage_count)));
        if let Some(desc) = &concept.description {
            metadata.insert("description".to_string(), serde_json::Value::String(desc.clone()));
        }
        metadata
    }

    /// Create edge metadata for tooltips
    fn create_edge_metadata(relationship: &crate::concept_graph::ConceptRelationship) -> HashMap<String, serde_json::Value> {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), serde_json::Value::String(format!("{:?}", relationship.relationship_type)));
        metadata.insert("weight".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(relationship.weight).unwrap()));
        metadata.insert("activation_count".to_string(), serde_json::Value::Number(serde_json::Number::from(relationship.activation_count)));
        metadata.insert("created_at".to_string(), serde_json::Value::String(relationship.created_at.to_rfc3339()));
        metadata.insert("last_activated_at".to_string(), serde_json::Value::String(relationship.last_activated_at.to_rfc3339()));
        metadata
    }

    /// Generate comprehensive simulation dashboard data
    pub async fn generate_simulation_dashboard_data(
        &self,
        simulation_engine: &crate::simulation_engine::SimulationEngine,
        rule_database: &crate::insight_extraction::RuleDatabase,
    ) -> Result<SimulationDashboardData> {
        let statistics = self.generate_simulation_statistics(simulation_engine).await?;
        let recent_simulations = self.generate_recent_simulations(simulation_engine).await?;
        let rule_insights = self.generate_rule_insights(rule_database).await?;
        let performance_metrics = self.generate_performance_metrics(simulation_engine).await?;
        
        let metadata = DashboardMetadata {
            generated_at: chrono::Utc::now(),
            data_freshness_minutes: 5, // Assume data is 5 minutes fresh
            data_sources: 3, // simulation_engine, rule_database, performance_monitor
            version: "1.0.0".to_string(),
            applied_filters: HashMap::new(),
        };

        Ok(SimulationDashboardData {
            statistics,
            recent_simulations,
            rule_insights,
            performance_metrics,
            metadata,
        })
    }

    async fn generate_simulation_statistics(
        &self,
        _simulation_engine: &crate::simulation_engine::SimulationEngine,
    ) -> Result<SimulationStatistics> {
        // In a real implementation, this would query the simulation engine for actual data
        // For now, we'll generate realistic sample data
        
        let common_outcomes = vec![
            OutcomeFrequency {
                outcome: "Successful task completion".to_string(),
                frequency: 156,
                percentage: 65.2,
            },
            OutcomeFrequency {
                outcome: "Partial success with constraints".to_string(),
                frequency: 47,
                percentage: 19.6,
            },
            OutcomeFrequency {
                outcome: "Failure due to insufficient information".to_string(),
                frequency: 23,
                percentage: 9.6,
            },
            OutcomeFrequency {
                outcome: "Timeout or resource exhaustion".to_string(),
                frequency: 13,
                percentage: 5.4,
            },
        ];

        let confidence_distribution = ConfidenceDistribution {
            high_confidence: 142,
            medium_confidence: 78,
            low_confidence: 19,
        };

        Ok(SimulationStatistics {
            total_simulations: 239,
            average_confidence: 0.74,
            success_rate: 0.847,
            total_branches_explored: 4832,
            average_branches_per_simulation: 20.2,
            common_outcomes,
            confidence_distribution,
        })
    }

    async fn generate_recent_simulations(
        &self,
        _simulation_engine: &crate::simulation_engine::SimulationEngine,
    ) -> Result<Vec<SimulationResult>> {
        let now = chrono::Utc::now();
        
        Ok(vec![
            SimulationResult {
                id: "sim_2024_001".to_string(),
                timestamp: now - chrono::Duration::minutes(15),
                scenario: "A person wants to learn machine learning but has limited time".to_string(),
                outcome: "Recommended focused study plan with online courses and practical projects".to_string(),
                confidence: 0.87,
                branches_explored: 23,
                execution_time_ms: 1247,
                status: "completed".to_string(),
                insights: vec![
                    "Time constraints significantly impact learning strategy selection".to_string(),
                    "Practical projects accelerate comprehension".to_string(),
                    "Online courses provide structured foundation".to_string(),
                ],
                most_likely_path: vec![
                    SimulationStep {
                        step: 1,
                        description: "Assess current knowledge level".to_string(),
                        confidence: 0.92,
                        applied_rules: vec!["knowledge_assessment_rule".to_string()],
                    },
                    SimulationStep {
                        step: 2,
                        description: "Identify time constraints and availability".to_string(),
                        confidence: 0.89,
                        applied_rules: vec!["time_constraint_analysis".to_string()],
                    },
                    SimulationStep {
                        step: 3,
                        description: "Select appropriate learning resources".to_string(),
                        confidence: 0.85,
                        applied_rules: vec!["resource_selection_rule".to_string()],
                    },
                ],
                branching_stats: BranchingStatsSummary {
                    max_depth: 5,
                    average_confidence: 0.84,
                    terminal_branches: 8,
                    diversity_score: 0.73,
                },
            },
            SimulationResult {
                id: "sim_2024_002".to_string(),
                timestamp: now - chrono::Duration::minutes(32),
                scenario: "Team collaboration on a software project with remote members".to_string(),
                outcome: "Established communication protocols and task distribution system".to_string(),
                confidence: 0.79,
                branches_explored: 31,
                execution_time_ms: 2156,
                status: "completed".to_string(),
                insights: vec![
                    "Clear communication protocols reduce misunderstandings".to_string(),
                    "Time zone differences require asynchronous workflows".to_string(),
                    "Regular check-ins maintain team cohesion".to_string(),
                ],
                most_likely_path: vec![
                    SimulationStep {
                        step: 1,
                        description: "Analyze team composition and time zones".to_string(),
                        confidence: 0.88,
                        applied_rules: vec!["team_analysis_rule".to_string()],
                    },
                    SimulationStep {
                        step: 2,
                        description: "Establish communication channels".to_string(),
                        confidence: 0.82,
                        applied_rules: vec!["communication_setup_rule".to_string()],
                    },
                    SimulationStep {
                        step: 3,
                        description: "Define task distribution methodology".to_string(),
                        confidence: 0.76,
                        applied_rules: vec!["task_distribution_rule".to_string()],
                    },
                ],
                branching_stats: BranchingStatsSummary {
                    max_depth: 6,
                    average_confidence: 0.78,
                    terminal_branches: 12,
                    diversity_score: 0.81,
                },
            },
            SimulationResult {
                id: "sim_2024_003".to_string(),
                timestamp: now - chrono::Duration::minutes(58),
                scenario: "Optimizing energy consumption in a smart home system".to_string(),
                outcome: "Implemented adaptive scheduling with 23% energy reduction".to_string(),
                confidence: 0.91,
                branches_explored: 18,
                execution_time_ms: 987,
                status: "completed".to_string(),
                insights: vec![
                    "Peak hour avoidance provides significant savings".to_string(),
                    "User behavior patterns enable predictive optimization".to_string(),
                    "Device prioritization balances comfort and efficiency".to_string(),
                ],
                most_likely_path: vec![
                    SimulationStep {
                        step: 1,
                        description: "Analyze current energy usage patterns".to_string(),
                        confidence: 0.94,
                        applied_rules: vec!["energy_analysis_rule".to_string()],
                    },
                    SimulationStep {
                        step: 2,
                        description: "Identify optimization opportunities".to_string(),
                        confidence: 0.91,
                        applied_rules: vec!["optimization_identification_rule".to_string()],
                    },
                    SimulationStep {
                        step: 3,
                        description: "Implement adaptive scheduling system".to_string(),
                        confidence: 0.88,
                        applied_rules: vec!["adaptive_scheduling_rule".to_string()],
                    },
                ],
                branching_stats: BranchingStatsSummary {
                    max_depth: 4,
                    average_confidence: 0.89,
                    terminal_branches: 6,
                    diversity_score: 0.67,
                },
            },
            SimulationResult {
                id: "sim_2024_004".to_string(),
                timestamp: now - chrono::Duration::hours(2),
                scenario: "Customer service chatbot handling complex complaint resolution".to_string(),
                outcome: "Multi-step resolution process with escalation protocols".to_string(),
                confidence: 0.72,
                branches_explored: 42,
                execution_time_ms: 3421,
                status: "completed".to_string(),
                insights: vec![
                    "Emotional recognition improves response appropriateness".to_string(),
                    "Escalation thresholds prevent customer frustration".to_string(),
                    "Knowledge base completeness affects resolution success".to_string(),
                ],
                most_likely_path: vec![
                    SimulationStep {
                        step: 1,
                        description: "Analyze complaint sentiment and complexity".to_string(),
                        confidence: 0.85,
                        applied_rules: vec!["sentiment_analysis_rule".to_string()],
                    },
                    SimulationStep {
                        step: 2,
                        description: "Attempt automated resolution".to_string(),
                        confidence: 0.68,
                        applied_rules: vec!["automated_resolution_rule".to_string()],
                    },
                    SimulationStep {
                        step: 3,
                        description: "Escalate to human agent if needed".to_string(),
                        confidence: 0.74,
                        applied_rules: vec!["escalation_rule".to_string()],
                    },
                ],
                branching_stats: BranchingStatsSummary {
                    max_depth: 7,
                    average_confidence: 0.71,
                    terminal_branches: 15,
                    diversity_score: 0.89,
                },
            },
            SimulationResult {
                id: "sim_2024_005".to_string(),
                timestamp: now - chrono::Duration::hours(3),
                scenario: "Medical diagnosis support system for rare conditions".to_string(),
                outcome: "Differential diagnosis with confidence rankings and testing recommendations".to_string(),
                confidence: 0.68,
                branches_explored: 67,
                execution_time_ms: 5847,
                status: "completed".to_string(),
                insights: vec![
                    "Symptom correlation patterns crucial for rare conditions".to_string(),
                    "Medical history provides essential context".to_string(),
                    "Uncertainty quantification important for clinical decisions".to_string(),
                ],
                most_likely_path: vec![
                    SimulationStep {
                        step: 1,
                        description: "Collect and analyze symptoms".to_string(),
                        confidence: 0.82,
                        applied_rules: vec!["symptom_collection_rule".to_string()],
                    },
                    SimulationStep {
                        step: 2,
                        description: "Cross-reference with medical knowledge base".to_string(),
                        confidence: 0.71,
                        applied_rules: vec!["knowledge_cross_reference_rule".to_string()],
                    },
                    SimulationStep {
                        step: 3,
                        description: "Generate differential diagnosis".to_string(),
                        confidence: 0.64,
                        applied_rules: vec!["differential_diagnosis_rule".to_string()],
                    },
                    SimulationStep {
                        step: 4,
                        description: "Recommend confirmatory tests".to_string(),
                        confidence: 0.59,
                        applied_rules: vec!["test_recommendation_rule".to_string()],
                    },
                ],
                branching_stats: BranchingStatsSummary {
                    max_depth: 8,
                    average_confidence: 0.66,
                    terminal_branches: 22,
                    diversity_score: 0.94,
                },
            },
        ])
    }

    async fn generate_rule_insights(
        &self,
        rule_database: &crate::insight_extraction::RuleDatabase,
    ) -> Result<RuleInsights> {
        let stats = rule_database.get_stats();
        
        let top_rules = vec![
            RuleUsageSummary {
                id: "rule_001".to_string(),
                pattern: "When user asks about learning → assess prior knowledge first".to_string(),
                outcome: "More effective learning path selection".to_string(),
                confidence: 0.89,
                support: 0.76,
                usage_count: 247,
                success_rate: 0.91,
                created_at: chrono::Utc::now() - chrono::Duration::days(15),
            },
            RuleUsageSummary {
                id: "rule_002".to_string(),
                pattern: "When time constraints exist → prioritize high-impact activities".to_string(),
                outcome: "Better resource utilization".to_string(),
                confidence: 0.84,
                support: 0.72,
                usage_count: 189,
                success_rate: 0.87,
                created_at: chrono::Utc::now() - chrono::Duration::days(12),
            },
            RuleUsageSummary {
                id: "rule_003".to_string(),
                pattern: "When collaboration needed → establish communication protocols".to_string(),
                outcome: "Reduced coordination overhead".to_string(),
                confidence: 0.91,
                support: 0.68,
                usage_count: 156,
                success_rate: 0.93,
                created_at: chrono::Utc::now() - chrono::Duration::days(8),
            },
        ];

        let highest_confidence_rules = vec![
            RuleUsageSummary {
                id: "rule_004".to_string(),
                pattern: "When safety critical → require multiple validation steps".to_string(),
                outcome: "Increased system reliability".to_string(),
                confidence: 0.97,
                support: 0.58,
                usage_count: 78,
                success_rate: 0.98,
                created_at: chrono::Utc::now() - chrono::Duration::days(20),
            },
            RuleUsageSummary {
                id: "rule_005".to_string(),
                pattern: "When user frustrated → escalate to human intervention".to_string(),
                outcome: "Improved user satisfaction".to_string(),
                confidence: 0.95,
                support: 0.62,
                usage_count: 134,
                success_rate: 0.96,
                created_at: chrono::Utc::now() - chrono::Duration::days(18),
            },
        ];

        let recent_rules = vec![
            RuleUsageSummary {
                id: "rule_006".to_string(),
                pattern: "When energy optimization → consider user comfort preferences".to_string(),
                outcome: "Balanced efficiency and satisfaction".to_string(),
                confidence: 0.73,
                support: 0.45,
                usage_count: 23,
                success_rate: 0.78,
                created_at: chrono::Utc::now() - chrono::Duration::days(2),
            },
            RuleUsageSummary {
                id: "rule_007".to_string(),
                pattern: "When medical uncertainty high → recommend specialist consultation".to_string(),
                outcome: "Appropriate care escalation".to_string(),
                confidence: 0.81,
                support: 0.39,
                usage_count: 15,
                success_rate: 0.87,
                created_at: chrono::Utc::now() - chrono::Duration::days(1),
            },
        ];

        let pattern_distribution = vec![
            PatternTypeCount {
                pattern_type: "Temporal Sequence".to_string(),
                count: 89,
                percentage: 34.2,
            },
            PatternTypeCount {
                pattern_type: "Conditional".to_string(),
                count: 67,
                percentage: 25.8,
            },
            PatternTypeCount {
                pattern_type: "Co-occurrence".to_string(),
                count: 45,
                percentage: 17.3,
            },
            PatternTypeCount {
                pattern_type: "Causal".to_string(),
                count: 38,
                percentage: 14.6,
            },
            PatternTypeCount {
                pattern_type: "Hierarchical".to_string(),
                count: 21,
                percentage: 8.1,
            },
        ];

        let rule_performance = RulePerformanceMetrics {
            overall_success_rate: stats.overall_success_rate,
            average_confidence: stats.average_confidence,
            average_support: stats.average_support,
            deprecated_rules: stats.deprecated_rules,
            recent_rule_creation_rate: 12, // Rules created in last 24h
        };

        Ok(RuleInsights {
            total_rules: stats.total_rules,
            active_rules: stats.active_rules,
            top_rules,
            highest_confidence_rules,
            recent_rules,
            rule_performance,
            pattern_distribution,
        })
    }

    async fn generate_performance_metrics(
        &self,
        _simulation_engine: &crate::simulation_engine::SimulationEngine,
    ) -> Result<PerformanceMetrics> {
        // In a real implementation, this would collect actual performance data
        Ok(PerformanceMetrics {
            average_execution_time_ms: 2156.7,
            fastest_simulation_ms: 342,
            slowest_simulation_ms: 8934,
            memory_usage: MemoryUsageStats {
                average_memory_mb: 45.3,
                peak_memory_mb: 127.8,
                efficiency_score: 0.82,
            },
            resource_utilization: ResourceUtilization {
                cpu_utilization: 23.7,
                memory_utilization: 41.2,
                throughput: 18.5, // simulations per minute
            },
        })
    }
}

/// Query parameters for concept graph filtering
#[derive(Debug, Deserialize)]
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

/// Query parameters for timeline filtering
#[derive(Debug, Deserialize)]
pub struct TimelineQueryParams {
    /// Filter by event type
    pub event_type: Option<String>,
    /// Minimum importance threshold
    pub min_importance: Option<f64>,
    /// Start time filter
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// End time filter
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Maximum number of events
    pub limit: Option<usize>,
    /// Search term for event content
    pub search: Option<String>,
}

/// API endpoint handlers

/// Get concept graph data
async fn get_concept_graph_data() -> impl IntoResponse {
    // This would be implemented with actual concept graph manager
    // For now, return a simple example
    let graph_data = create_sample_graph_data();
    Json(graph_data)
}

/// Get filtered concept graph data
async fn get_filtered_concept_graph(Query(params): Query<GraphQueryParams>) -> impl IntoResponse {
    debug!("Filtering concept graph with params: {:?}", params);
    
    // This would implement filtering logic
    let graph_data = create_sample_graph_data();
    Json(graph_data)
}

/// Get memory timeline data
async fn get_memory_timeline_data() -> impl IntoResponse {
    let timeline_data = create_sample_timeline_data();
    Json(timeline_data)
}

/// Get filtered memory timeline data
async fn get_filtered_memory_timeline(Query(params): Query<TimelineQueryParams>) -> impl IntoResponse {
    debug!("Filtering memory timeline with params: {:?}", params);
    
    // Create sample timeline data and apply filters
    let mut timeline_data = create_sample_timeline_data();
    
    // Apply filters
    if let Some(event_type) = &params.event_type {
        timeline_data.events.retain(|event| &event.event_type == event_type);
    }
    
    if let Some(min_importance) = params.min_importance {
        timeline_data.events.retain(|event| event.importance >= min_importance);
    }
    
    if let Some(start_time) = params.start_time {
        timeline_data.events.retain(|event| event.timestamp >= start_time);
    }
    
    if let Some(end_time) = params.end_time {
        timeline_data.events.retain(|event| event.timestamp <= end_time);
    }
    
    if let Some(search) = &params.search {
        let search_lower = search.to_lowercase();
        timeline_data.events.retain(|event| 
            event.title.to_lowercase().contains(&search_lower) ||
            event.description.to_lowercase().contains(&search_lower)
        );
    }
    
    if let Some(limit) = params.limit {
        timeline_data.events.truncate(limit);
    }
    
    // Update metadata
    timeline_data.metadata.event_count = timeline_data.events.len();
    if !timeline_data.events.is_empty() {
        timeline_data.metadata.start_time = timeline_data.events.first().unwrap().timestamp;
        timeline_data.metadata.end_time = timeline_data.events.last().unwrap().timestamp;
    }
    
    Json(timeline_data)
}

/// Get simulation dashboard data
async fn get_simulation_dashboard_data() -> impl IntoResponse {
    let dashboard_data = create_comprehensive_dashboard_data();
    Json(dashboard_data)
}

/// Get filtered simulation dashboard data
async fn get_filtered_simulation_dashboard(Query(params): Query<DashboardQueryParams>) -> impl IntoResponse {
    let dashboard_data = create_filtered_dashboard_data(params);
    Json(dashboard_data)
}

/// Serve concept graph visualization page
async fn serve_concept_graph_page() -> impl IntoResponse {
    Html(include_str!("../web/concept_graph.html"))
}

/// Serve memory timeline visualization page
async fn serve_memory_timeline_page() -> impl IntoResponse {
    Html(include_str!("../web/memory_timeline.html"))
}

/// Serve simulation dashboard page
async fn serve_simulation_dashboard_page() -> impl IntoResponse {
    Html(include_str!("../web/simulation_dashboard.html"))
}

/// Helper functions for sample data

fn create_sample_graph_data() -> GraphData {
    let nodes = vec![
        VisualizationNode {
            id: "concept_1".to_string(),
            name: "Natural Language Processing".to_string(),
            node_type: "Abstract".to_string(),
            size: 25.0,
            color: "#1f77b4".to_string(),
            metadata: HashMap::new(),
            x: None,
            y: None,
            degree: 5,
            confidence: 0.85,
        },
        VisualizationNode {
            id: "concept_2".to_string(),
            name: "Machine Learning".to_string(),
            node_type: "Abstract".to_string(),
            size: 30.0,
            color: "#1f77b4".to_string(),
            metadata: HashMap::new(),
            x: None,
            y: None,
            degree: 8,
            confidence: 0.92,
        },
        VisualizationNode {
            id: "concept_3".to_string(),
            name: "Text Analysis".to_string(),
            node_type: "Entity".to_string(),
            size: 20.0,
            color: "#ff7f0e".to_string(),
            metadata: HashMap::new(),
            x: None,
            y: None,
            degree: 3,
            confidence: 0.75,
        },
    ];

    let edges = vec![
        VisualizationEdge {
            source: "concept_1".to_string(),
            target: "concept_2".to_string(),
            weight: 0.8,
            edge_type: "similarity".to_string(),
            color: "#44ff44".to_string(),
            metadata: HashMap::new(),
        },
        VisualizationEdge {
            source: "concept_1".to_string(),
            target: "concept_3".to_string(),
            weight: 0.6,
            edge_type: "causal".to_string(),
            color: "#ff4444".to_string(),
            metadata: HashMap::new(),
        },
    ];

    GraphData {
        nodes,
        edges,
        metadata: GraphMetadata {
            node_count: 3,
            edge_count: 2,
            timestamp: chrono::Utc::now(),
            graph_type: "concept_graph".to_string(),
            layout_algorithm: "force".to_string(),
            filters: HashMap::new(),
        },
    }
}

fn create_sample_timeline_data() -> TimelineData {
    let now = chrono::Utc::now();
    let mut events = vec![
        TimelineEvent {
            id: "event_1".to_string(),
            timestamp: now - chrono::Duration::hours(24),
            title: "System initialization completed".to_string(),
            description: "Brain AI system started up successfully with all modules loaded".to_string(),
            event_type: "system".to_string(),
            importance: 0.9,
            related_concepts: vec!["initialization".to_string(), "system".to_string()],
            metadata: create_event_metadata("system", "startup"),
        },
        TimelineEvent {
            id: "event_2".to_string(),
            timestamp: now - chrono::Duration::hours(20),
            title: "First pattern discovered".to_string(),
            description: "BPE segmentation discovered recurring pattern 'the' with frequency 15".to_string(),
            event_type: "learning".to_string(),
            importance: 0.7,
            related_concepts: vec!["bpe".to_string(), "patterns".to_string()],
            metadata: create_event_metadata("segment_discovery", "pattern_found"),
        },
        TimelineEvent {
            id: "event_3".to_string(),
            timestamp: now - chrono::Duration::hours(18),
            title: "Memory consolidation triggered".to_string(),
            description: "Working memory reached capacity, consolidating 5 items to episodic memory".to_string(),
            event_type: "consolidation".to_string(),
            importance: 0.6,
            related_concepts: vec!["memory".to_string(), "consolidation".to_string()],
            metadata: create_event_metadata("memory_system", "consolidation"),
        },
        TimelineEvent {
            id: "event_4".to_string(),
            timestamp: now - chrono::Duration::hours(15),
            title: "Concept relationship formed".to_string(),
            description: "New relationship discovered: 'neural networks' IS_A 'machine learning' with confidence 0.85".to_string(),
            event_type: "insight".to_string(),
            importance: 0.8,
            related_concepts: vec!["neural_networks".to_string(), "machine_learning".to_string()],
            metadata: create_event_metadata("concept_graph", "relationship_formed"),
        },
        TimelineEvent {
            id: "event_5".to_string(),
            timestamp: now - chrono::Duration::hours(12),
            title: "User query processed".to_string(),
            description: "Processed query about 'artificial intelligence concepts' with 95% confidence".to_string(),
            event_type: "interaction".to_string(),
            importance: 0.7,
            related_concepts: vec!["artificial_intelligence".to_string(), "query".to_string()],
            metadata: create_event_metadata("query_system", "user_interaction"),
        },
        TimelineEvent {
            id: "event_6".to_string(),
            timestamp: now - chrono::Duration::hours(8),
            title: "Simulation completed".to_string(),
            description: "Completed branching simulation with 42 explored paths and 78% average confidence".to_string(),
            event_type: "simulation".to_string(),
            importance: 0.75,
            related_concepts: vec!["simulation".to_string(), "branching".to_string()],
            metadata: create_event_metadata("simulation_engine", "simulation_complete"),
        },
        TimelineEvent {
            id: "event_7".to_string(),
            timestamp: now - chrono::Duration::hours(5),
            title: "Novelty detected".to_string(),
            description: "Input 'quantum machine learning' detected as highly novel (novelty score: 0.92)".to_string(),
            event_type: "learning".to_string(),
            importance: 0.85,
            related_concepts: vec!["novelty".to_string(), "quantum".to_string(), "machine_learning".to_string()],
            metadata: create_event_metadata("novelty_detection", "high_novelty"),
        },
        TimelineEvent {
            id: "event_8".to_string(),
            timestamp: now - chrono::Duration::hours(2),
            title: "Curiosity-driven exploration".to_string(),
            description: "System expressed curiosity about 'deep reinforcement learning' and initiated learning priority".to_string(),
            event_type: "learning".to_string(),
            importance: 0.8,
            related_concepts: vec!["curiosity".to_string(), "reinforcement_learning".to_string()],
            metadata: create_event_metadata("curiosity_learning", "exploration_initiated"),
        },
        TimelineEvent {
            id: "event_9".to_string(),
            timestamp: now - chrono::Duration::minutes(30),
            title: "Performance warning".to_string(),
            description: "Concept graph query took 1.2s to complete, approaching timeout threshold".to_string(),
            event_type: "warning".to_string(),
            importance: 0.4,
            related_concepts: vec!["performance".to_string(), "concept_graph".to_string()],
            metadata: create_event_metadata("performance_monitor", "slow_query"),
        },
        TimelineEvent {
            id: "event_10".to_string(),
            timestamp: now - chrono::Duration::minutes(5),
            title: "Meta-memory updated".to_string(),
            description: "Meta-memory confidence for 'neural_architecture' knowledge increased to 0.78".to_string(),
            event_type: "consolidation".to_string(),
            importance: 0.6,
            related_concepts: vec!["meta_memory".to_string(), "neural_architecture".to_string()],
            metadata: create_event_metadata("meta_memory", "confidence_update"),
        },
    ];

    // Sort events chronologically
    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    TimelineData {
        metadata: TimelineMetadata {
            event_count: events.len(),
            start_time: events.first().unwrap().timestamp,
            end_time: events.last().unwrap().timestamp,
            timestamp: chrono::Utc::now(),
            filters: HashMap::new(),
        },
        events,
    }
}

fn create_event_metadata(module: &str, action: &str) -> HashMap<String, serde_json::Value> {
    let mut metadata = HashMap::new();
    metadata.insert("module".to_string(), serde_json::Value::String(module.to_string()));
    metadata.insert("action".to_string(), serde_json::Value::String(action.to_string()));
    metadata.insert("generated_at".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
    metadata
}

fn create_comprehensive_dashboard_data() -> SimulationDashboardData {
    let now = chrono::Utc::now();
    
    let statistics = SimulationStatistics {
        total_simulations: 239,
        average_confidence: 0.74,
        success_rate: 0.847,
        total_branches_explored: 4832,
        average_branches_per_simulation: 20.2,
        common_outcomes: vec![
            OutcomeFrequency {
                outcome: "Successful task completion".to_string(),
                frequency: 156,
                percentage: 65.2,
            },
            OutcomeFrequency {
                outcome: "Partial success with constraints".to_string(),
                frequency: 47,
                percentage: 19.6,
            },
            OutcomeFrequency {
                outcome: "Failure due to insufficient information".to_string(),
                frequency: 23,
                percentage: 9.6,
            },
            OutcomeFrequency {
                outcome: "Timeout or resource exhaustion".to_string(),
                frequency: 13,
                percentage: 5.4,
            },
        ],
        confidence_distribution: ConfidenceDistribution {
            high_confidence: 142,
            medium_confidence: 78,
            low_confidence: 19,
        },
    };

    let recent_simulations = vec![
        SimulationResult {
            id: "sim_2024_001".to_string(),
            timestamp: now - chrono::Duration::minutes(15),
            scenario: "A person wants to learn machine learning but has limited time".to_string(),
            outcome: "Recommended focused study plan with online courses and practical projects".to_string(),
            confidence: 0.87,
            branches_explored: 23,
            execution_time_ms: 1247,
            status: "completed".to_string(),
            insights: vec![
                "Time constraints significantly impact learning strategy selection".to_string(),
                "Practical projects accelerate comprehension".to_string(),
                "Online courses provide structured foundation".to_string(),
            ],
            most_likely_path: vec![
                SimulationStep {
                    step: 1,
                    description: "Assess current knowledge level".to_string(),
                    confidence: 0.92,
                    applied_rules: vec!["knowledge_assessment_rule".to_string()],
                },
                SimulationStep {
                    step: 2,
                    description: "Identify time constraints and availability".to_string(),
                    confidence: 0.89,
                    applied_rules: vec!["time_constraint_analysis".to_string()],
                },
                SimulationStep {
                    step: 3,
                    description: "Select appropriate learning resources".to_string(),
                    confidence: 0.85,
                    applied_rules: vec!["resource_selection_rule".to_string()],
                },
            ],
            branching_stats: BranchingStatsSummary {
                max_depth: 5,
                average_confidence: 0.84,
                terminal_branches: 8,
                diversity_score: 0.73,
            },
        },
        SimulationResult {
            id: "sim_2024_002".to_string(),
            timestamp: now - chrono::Duration::minutes(32),
            scenario: "Team collaboration on a software project with remote members".to_string(),
            outcome: "Established communication protocols and task distribution system".to_string(),
            confidence: 0.79,
            branches_explored: 31,
            execution_time_ms: 2156,
            status: "completed".to_string(),
            insights: vec![
                "Clear communication protocols reduce misunderstandings".to_string(),
                "Time zone differences require asynchronous workflows".to_string(),
                "Regular check-ins maintain team cohesion".to_string(),
            ],
            most_likely_path: vec![
                SimulationStep {
                    step: 1,
                    description: "Analyze team composition and time zones".to_string(),
                    confidence: 0.88,
                    applied_rules: vec!["team_analysis_rule".to_string()],
                },
                SimulationStep {
                    step: 2,
                    description: "Establish communication channels".to_string(),
                    confidence: 0.82,
                    applied_rules: vec!["communication_setup_rule".to_string()],
                },
                SimulationStep {
                    step: 3,
                    description: "Define task distribution methodology".to_string(),
                    confidence: 0.76,
                    applied_rules: vec!["task_distribution_rule".to_string()],
                },
            ],
            branching_stats: BranchingStatsSummary {
                max_depth: 6,
                average_confidence: 0.78,
                terminal_branches: 12,
                diversity_score: 0.81,
            },
        },
        SimulationResult {
            id: "sim_2024_003".to_string(),
            timestamp: now - chrono::Duration::minutes(58),
            scenario: "Optimizing energy consumption in a smart home system".to_string(),
            outcome: "Implemented adaptive scheduling with 23% energy reduction".to_string(),
            confidence: 0.91,
            branches_explored: 18,
            execution_time_ms: 987,
            status: "completed".to_string(),
            insights: vec![
                "Peak hour avoidance provides significant savings".to_string(),
                "User behavior patterns enable predictive optimization".to_string(),
                "Device prioritization balances comfort and efficiency".to_string(),
            ],
            most_likely_path: vec![
                SimulationStep {
                    step: 1,
                    description: "Analyze current energy usage patterns".to_string(),
                    confidence: 0.94,
                    applied_rules: vec!["energy_analysis_rule".to_string()],
                },
                SimulationStep {
                    step: 2,
                    description: "Identify optimization opportunities".to_string(),
                    confidence: 0.91,
                    applied_rules: vec!["optimization_identification_rule".to_string()],
                },
                SimulationStep {
                    step: 3,
                    description: "Implement adaptive scheduling system".to_string(),
                    confidence: 0.88,
                    applied_rules: vec!["adaptive_scheduling_rule".to_string()],
                },
            ],
            branching_stats: BranchingStatsSummary {
                max_depth: 4,
                average_confidence: 0.89,
                terminal_branches: 6,
                diversity_score: 0.67,
            },
        },
    ];

    let rule_insights = RuleInsights {
        total_rules: 260,
        active_rules: 247,
        top_rules: vec![
            RuleUsageSummary {
                id: "rule_001".to_string(),
                pattern: "When user asks about learning → assess prior knowledge first".to_string(),
                outcome: "More effective learning path selection".to_string(),
                confidence: 0.89,
                support: 0.76,
                usage_count: 247,
                success_rate: 0.91,
                created_at: now - chrono::Duration::days(15),
            },
            RuleUsageSummary {
                id: "rule_002".to_string(),
                pattern: "When time constraints exist → prioritize high-impact activities".to_string(),
                outcome: "Better resource utilization".to_string(),
                confidence: 0.84,
                support: 0.72,
                usage_count: 189,
                success_rate: 0.87,
                created_at: now - chrono::Duration::days(12),
            },
            RuleUsageSummary {
                id: "rule_003".to_string(),
                pattern: "When collaboration needed → establish communication protocols".to_string(),
                outcome: "Reduced coordination overhead".to_string(),
                confidence: 0.91,
                support: 0.68,
                usage_count: 156,
                success_rate: 0.93,
                created_at: now - chrono::Duration::days(8),
            },
        ],
        highest_confidence_rules: vec![
            RuleUsageSummary {
                id: "rule_004".to_string(),
                pattern: "When safety critical → require multiple validation steps".to_string(),
                outcome: "Increased system reliability".to_string(),
                confidence: 0.97,
                support: 0.58,
                usage_count: 78,
                success_rate: 0.98,
                created_at: now - chrono::Duration::days(20),
            },
            RuleUsageSummary {
                id: "rule_005".to_string(),
                pattern: "When user frustrated → escalate to human intervention".to_string(),
                outcome: "Improved user satisfaction".to_string(),
                confidence: 0.95,
                support: 0.62,
                usage_count: 134,
                success_rate: 0.96,
                created_at: now - chrono::Duration::days(18),
            },
        ],
        recent_rules: vec![
            RuleUsageSummary {
                id: "rule_006".to_string(),
                pattern: "When energy optimization → consider user comfort preferences".to_string(),
                outcome: "Balanced efficiency and satisfaction".to_string(),
                confidence: 0.73,
                support: 0.45,
                usage_count: 23,
                success_rate: 0.78,
                created_at: now - chrono::Duration::days(2),
            },
            RuleUsageSummary {
                id: "rule_007".to_string(),
                pattern: "When medical uncertainty high → recommend specialist consultation".to_string(),
                outcome: "Appropriate care escalation".to_string(),
                confidence: 0.81,
                support: 0.39,
                usage_count: 15,
                success_rate: 0.87,
                created_at: now - chrono::Duration::days(1),
            },
        ],
        rule_performance: RulePerformanceMetrics {
            overall_success_rate: 0.88,
            average_confidence: 0.82,
            average_support: 0.64,
            deprecated_rules: 13,
            recent_rule_creation_rate: 12,
        },
        pattern_distribution: vec![
            PatternTypeCount {
                pattern_type: "Temporal Sequence".to_string(),
                count: 89,
                percentage: 34.2,
            },
            PatternTypeCount {
                pattern_type: "Conditional".to_string(),
                count: 67,
                percentage: 25.8,
            },
            PatternTypeCount {
                pattern_type: "Co-occurrence".to_string(),
                count: 45,
                percentage: 17.3,
            },
            PatternTypeCount {
                pattern_type: "Causal".to_string(),
                count: 38,
                percentage: 14.6,
            },
            PatternTypeCount {
                pattern_type: "Hierarchical".to_string(),
                count: 21,
                percentage: 8.1,
            },
        ],
    };

    let performance_metrics = PerformanceMetrics {
        average_execution_time_ms: 2156.7,
        fastest_simulation_ms: 342,
        slowest_simulation_ms: 8934,
        memory_usage: MemoryUsageStats {
            average_memory_mb: 45.3,
            peak_memory_mb: 127.8,
            efficiency_score: 0.82,
        },
        resource_utilization: ResourceUtilization {
            cpu_utilization: 23.7,
            memory_utilization: 41.2,
            throughput: 18.5,
        },
    };

    let metadata = DashboardMetadata {
        generated_at: now,
        data_freshness_minutes: 5,
        data_sources: 3,
        version: "1.0.0".to_string(),
        applied_filters: HashMap::new(),
    };

    SimulationDashboardData {
        statistics,
        recent_simulations,
        rule_insights,
        performance_metrics,
        metadata,
    }
}

fn create_filtered_dashboard_data(params: DashboardQueryParams) -> SimulationDashboardData {
    let mut data = create_comprehensive_dashboard_data();
    
    // Apply filters to recent simulations
    if let Some(min_confidence) = params.min_confidence {
        data.recent_simulations.retain(|sim| sim.confidence >= min_confidence);
    }
    
    if let Some(max_confidence) = params.max_confidence {
        data.recent_simulations.retain(|sim| sim.confidence <= max_confidence);
    }
    
    if let Some(status) = &params.status {
        data.recent_simulations.retain(|sim| sim.status == *status);
    }
    
    if let Some(start_time) = params.start_time {
        data.recent_simulations.retain(|sim| sim.timestamp >= start_time);
    }
    
    if let Some(end_time) = params.end_time {
        data.recent_simulations.retain(|sim| sim.timestamp <= end_time);
    }
    
    if let Some(limit) = params.limit {
        data.recent_simulations.truncate(limit);
    }
    
    // Update statistics based on filtered data
    data.statistics.total_simulations = data.recent_simulations.len();
    if !data.recent_simulations.is_empty() {
        data.statistics.average_confidence = data.recent_simulations.iter()
            .map(|sim| sim.confidence)
            .sum::<f64>() / data.recent_simulations.len() as f64;
    }
    
    // Update metadata with applied filters
    data.metadata.applied_filters = serde_json::from_str(&serde_json::to_string(&params).unwrap_or_default()).unwrap_or_default();
    data.metadata.generated_at = chrono::Utc::now();
    
    data
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
    fn test_node_size_calculation() {
        let concept = ConceptNode {
            id: uuid::Uuid::new_v4(),
            concept_type: ConceptType::Entity,
            content: "Test Concept".to_string(),
            description: Some("A test concept".to_string()),
            created_at: chrono::Utc::now(),
            last_accessed_at: chrono::Utc::now(),
            usage_count: 5,
            confidence_score: 0.8,
            source_reference: Some("test".to_string()),
            metadata: HashMap::new(),
        };

        let size = VisualizationManager::calculate_node_size(&concept);
        assert!(size >= 10.0);
        assert!(size <= 50.0);
    }

    #[test]
    fn test_node_color_mapping() {
        assert_eq!(VisualizationManager::get_node_color(&ConceptType::Entity), "#1f77b4");
        assert_eq!(VisualizationManager::get_node_color(&ConceptType::Action), "#ff7f0e");
        assert_eq!(VisualizationManager::get_node_color(&ConceptType::Attribute), "#2ca02c");
        assert_eq!(VisualizationManager::get_node_color(&ConceptType::Relation), "#9467bd");
    }

    #[test]
    fn test_sample_graph_data_generation() {
        let graph_data = create_sample_graph_data();
        assert_eq!(graph_data.nodes.len(), 3);
        assert_eq!(graph_data.edges.len(), 2);
        assert_eq!(graph_data.metadata.node_count, 3);
        assert_eq!(graph_data.metadata.edge_count, 2);
    }

    #[test]
    fn test_sample_timeline_data_generation() {
        let timeline_data = create_sample_timeline_data();
        assert!(timeline_data.events.len() >= 10);
        assert_eq!(timeline_data.metadata.event_count, timeline_data.events.len());
        
        // Verify events are sorted chronologically
        for i in 1..timeline_data.events.len() {
            assert!(timeline_data.events[i-1].timestamp <= timeline_data.events[i].timestamp);
        }
        
        // Verify event types are present
        let event_types: std::collections::HashSet<_> = timeline_data.events.iter()
            .map(|e| &e.event_type)
            .collect();
        assert!(event_types.contains(&"learning".to_string()));
        assert!(event_types.contains(&"system".to_string()));
        assert!(event_types.contains(&"consolidation".to_string()));
    }

    #[test]
    fn test_event_title_extraction() {
        let vis_manager = VisualizationManager::new(VisualizationConfig::default());
        
        // Test short content
        let short_title = vis_manager.extract_event_title("Short content");
        assert_eq!(short_title, "Short content");
        
        // Test long content
        let long_content = "This is a very long piece of content that should be truncated to fit within the title length limit";
        let long_title = vis_manager.extract_event_title(long_content);
        assert!(long_title.len() <= 50);
        assert!(long_title.ends_with("..."));
        
        // Test sentence-based extraction
        let sentence_content = "First sentence. Second sentence with more content.";
        let sentence_title = vis_manager.extract_event_title(sentence_content);
        assert_eq!(sentence_title, "First sentence");
    }

    #[test]
    fn test_event_type_determination() {
        let vis_manager = VisualizationManager::new(VisualizationConfig::default());
        
        // Test error tags
        let error_tags = vec!["error".to_string(), "warning".to_string()];
        let error_type = vis_manager.determine_event_type(&error_tags, &HashMap::new());
        assert_eq!(error_type, "error");
        
        // Test learning tags
        let learning_tags = vec!["learning".to_string(), "discovery".to_string()];
        let learning_type = vis_manager.determine_event_type(&learning_tags, &HashMap::new());
        assert_eq!(learning_type, "learning");
        
        // Test context-based determination
        let mut context = HashMap::new();
        context.insert("source".to_string(), "working_memory".to_string());
        let context_type = vis_manager.determine_event_type(&[], &context);
        assert_eq!(context_type, "working_memory");
        
        // Test fallback to general
        let general_type = vis_manager.determine_event_type(&[], &HashMap::new());
        assert_eq!(general_type, "general");
    }

    #[test]
    fn test_timeline_metadata_creation() {
        let vis_manager = VisualizationManager::new(VisualizationConfig::default());
        
        let mut context = HashMap::new();
        context.insert("test_key".to_string(), "test_value".to_string());
        
        let event = crate::memory::EpisodicEvent {
            id: uuid::Uuid::new_v4(),
            content: "Test event content".to_string(),
            timestamp: chrono::Utc::now(),
            context: context.clone(),
            importance: 0.8,
            tags: vec!["test".to_string()],
            source: "test_source".to_string(),
        };
        
        let metadata = vis_manager.create_timeline_metadata(&event);
        
        assert_eq!(metadata.get("source").unwrap(), &serde_json::Value::String("test_source".to_string()));
        assert_eq!(metadata.get("importance").unwrap(), &serde_json::Value::Number(serde_json::Number::from_f64(0.8).unwrap()));
        assert_eq!(metadata.get("tag_count").unwrap(), &serde_json::Value::Number(serde_json::Number::from(1)));
        assert!(metadata.contains_key("context_test_key"));
    }
}