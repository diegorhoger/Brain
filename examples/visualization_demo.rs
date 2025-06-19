//! Visualization Demo - Task 8.1: Concept Graph Visualization
//!
//! This demo showcases the D3.js-based concept graph visualization system,
//! demonstrating interactive graph exploration, filtering, and real-time updates.

use anyhow::Result;
use brain::{
    ConceptGraphManager, ConceptGraphConfig, ConceptNode, ConceptType,
    VisualizationManager, VisualizationConfig
};
use brain::concept_graph::RelationshipType;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎨 Brain AI - Visualization System Demo");
    println!("========================================\n");

    // Initialize concept graph with sample data
    let concept_graph = create_sample_concept_graph().await?;
    println!("📊 Created sample concept graph with {} concepts", concept_graph.concept_count());

    // Initialize visualization manager
    let viz_config = VisualizationConfig {
        enable_concept_graph: true,
        enable_memory_timeline: true,
        enable_simulation_dashboard: true,
        max_graph_nodes: 50,
        default_layout: "force".to_string(),
        enable_interactions: true,
    };
    
    let viz_manager = VisualizationManager::new(viz_config);
    println!("✅ Initialized visualization manager");

    // Generate concept graph visualization data
    println!("\n🔄 Generating concept graph visualization data...");
    let graph_data = viz_manager.generate_concept_graph_data(&concept_graph).await?;
    
    println!("📈 Graph visualization data generated:");
    println!("   • Nodes: {}", graph_data.nodes.len());
    println!("   • Edges: {}", graph_data.edges.len());
    println!("   • Layout: {}", graph_data.metadata.layout_algorithm);
    println!("   • Generated at: {}", graph_data.metadata.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));

    // Display node details
    println!("\n🎯 Sample Concept Nodes:");
    for (i, node) in graph_data.nodes.iter().take(5).enumerate() {
        println!("   {}. {} [{}]", i + 1, node.name, node.node_type);
        println!("      • Size: {:.1}, Confidence: {:.2}, Connections: {}", 
                node.size, node.confidence, node.degree);
        println!("      • Color: {}, ID: {}", node.color, node.id);
    }

    // Display edge details
    println!("\n🔗 Sample Relationships:");
    for (i, edge) in graph_data.edges.iter().take(5).enumerate() {
        println!("   {}. {} → {} [{}]", i + 1, 
                get_node_name(&graph_data.nodes, &edge.source),
                get_node_name(&graph_data.nodes, &edge.target),
                edge.edge_type);
        println!("      • Weight: {:.2}, Color: {}", edge.weight, edge.color);
    }

    // Demonstrate filtering capabilities
    println!("\n🔍 Visualization Filtering Demo:");
    demonstrate_filtering(&graph_data);

    // Show node statistics by type
    println!("\n📊 Node Distribution by Type:");
    let mut type_counts = HashMap::new();
    for node in &graph_data.nodes {
        *type_counts.entry(node.node_type.clone()).or_insert(0) += 1;
    }
    for (node_type, count) in type_counts {
        let percentage = (count as f64 / graph_data.nodes.len() as f64) * 100.0;
        println!("   • {}: {} nodes ({:.1}%)", node_type, count, percentage);
    }

    // Show relationship statistics
    println!("\n🔗 Relationship Distribution:");
    let mut rel_counts = HashMap::new();
    for edge in &graph_data.edges {
        *rel_counts.entry(edge.edge_type.clone()).or_insert(0) += 1;
    }
    for (rel_type, count) in rel_counts {
        let percentage = (count as f64 / graph_data.edges.len() as f64) * 100.0;
        println!("   • {}: {} relationships ({:.1}%)", rel_type, count, percentage);
    }

    // Demonstrate graph metrics
    println!("\n📈 Graph Metrics:");
    let avg_connections = graph_data.edges.len() as f64 / graph_data.nodes.len() as f64;
    let avg_confidence = graph_data.nodes.iter().map(|n| n.confidence).sum::<f64>() / graph_data.nodes.len() as f64;
    let avg_weight = graph_data.edges.iter().map(|e| e.weight).sum::<f64>() / graph_data.edges.len() as f64;
    
    println!("   • Average connections per node: {:.2}", avg_connections);
    println!("   • Average node confidence: {:.2}", avg_confidence);
    println!("   • Average relationship weight: {:.2}", avg_weight);
    
    // Find most connected nodes
    let mut nodes_by_degree = graph_data.nodes.clone();
    nodes_by_degree.sort_by(|a, b| b.degree.cmp(&a.degree));
    
    println!("\n🏆 Most Connected Concepts:");
    for (i, node) in nodes_by_degree.iter().take(3).enumerate() {
        println!("   {}. {} ({} connections, {:.2} confidence)", 
                i + 1, node.name, node.degree, node.confidence);
    }

    // Demonstrate web interface information
    println!("\n🌐 Web Visualization Interface:");
    println!("   • Concept Graph: /visualization/concept-graph");
    println!("   • Memory Timeline: /visualization/memory-timeline");
    println!("   • Dashboard: /visualization/simulation-dashboard");
    println!("\n   • API Endpoints:");
    println!("     - GET /api/visualization/concept-graph");
    println!("     - GET /api/visualization/concept-graph/filtered");
    println!("     - GET /api/visualization/memory-timeline");
    println!("     - GET /api/visualization/simulation-dashboard");

    // Show sample JSON output
    println!("\n📄 Sample JSON Output (First Node):");
    if let Some(first_node) = graph_data.nodes.first() {
        let json_output = serde_json::to_string_pretty(first_node)?;
        println!("{}", json_output);
    }

    println!("\n✅ Visualization Demo Complete!");
    println!("   🎯 Task 8.1: Concept Graph Visualization - IMPLEMENTED");
    println!("   📊 D3.js-compatible data generation - READY");
    println!("   🌐 Web interface endpoints - CONFIGURED");
    println!("   🔍 Interactive filtering - SUPPORTED");
    println!("   📈 Real-time graph metrics - AVAILABLE");
    
    Ok(())
}

/// Create a sample concept graph for demonstration
async fn create_sample_concept_graph() -> Result<ConceptGraphManager> {
    let config = ConceptGraphConfig::default();
    let mut manager = ConceptGraphManager::new(config).await?;
    
    // Create sample concepts
    let concepts = vec![
        ("Natural Language Processing", ConceptType::Abstract, 0.95),
        ("Machine Learning", ConceptType::Abstract, 0.92),
        ("Neural Networks", ConceptType::Entity, 0.88),
        ("Deep Learning", ConceptType::Abstract, 0.90),
        ("Text Analysis", ConceptType::Action, 0.85),
        ("Pattern Recognition", ConceptType::Action, 0.82),
        ("Data Processing", ConceptType::Action, 0.78),
        ("Algorithm", ConceptType::Entity, 0.87),
        ("Training Data", ConceptType::Entity, 0.80),
        ("Model Accuracy", ConceptType::Attribute, 0.85),
        ("Feature Extraction", ConceptType::Action, 0.83),
        ("Classification", ConceptType::Action, 0.86),
        ("Regression", ConceptType::Action, 0.84),
        ("Clustering", ConceptType::Action, 0.81),
        ("Dimensionality Reduction", ConceptType::Action, 0.79),
    ];
    
    let mut concept_ids = Vec::new();
    
    // Add concepts to the graph
    for (content, concept_type, confidence) in concepts {
        let concept = ConceptNode::new(
            concept_type,
            content.to_string(),
            confidence,
            Some("demo".to_string()),
        );
        let id = manager.create_concept(concept).await?;
        concept_ids.push(id);
    }
    
    // Create relationships between concepts
    let relationships = vec![
        (0, 1, RelationshipType::SimilarTo, 0.85),      // NLP similar to ML
        (1, 2, RelationshipType::Uses, 0.90),           // ML uses Neural Networks
        (2, 3, RelationshipType::IsA, 0.95),            // Neural Networks is a Deep Learning
        (1, 4, RelationshipType::Uses, 0.88),           // ML uses Text Analysis
        (4, 0, RelationshipType::PartOf, 0.92),         // Text Analysis part of NLP
        (1, 5, RelationshipType::Uses, 0.83),           // ML uses Pattern Recognition
        (5, 6, RelationshipType::Uses, 0.75),           // Pattern Recognition uses Data Processing
        (1, 7, RelationshipType::Uses, 0.90),           // ML uses Algorithm
        (1, 8, RelationshipType::Uses, 0.85),           // ML uses Training Data
        (1, 9, RelationshipType::Has, 0.87),            // ML has Model Accuracy
        (4, 10, RelationshipType::Uses, 0.80),          // Text Analysis uses Feature Extraction
        (1, 11, RelationshipType::Uses, 0.85),          // ML uses Classification
        (1, 12, RelationshipType::Uses, 0.82),          // ML uses Regression
        (1, 13, RelationshipType::Uses, 0.78),          // ML uses Clustering
        (10, 14, RelationshipType::SimilarTo, 0.76),    // Feature Extraction similar to Dimensionality Reduction
    ];
    
    // Add relationships
    for (source_idx, target_idx, rel_type, weight) in relationships {
        if source_idx < concept_ids.len() && target_idx < concept_ids.len() {
            manager.create_relationship(
                concept_ids[source_idx],
                concept_ids[target_idx],
                rel_type,
                weight,
            ).await?;
        }
    }
    
    Ok(manager)
}

/// Get node name by ID for display purposes
fn get_node_name(nodes: &[brain::VisualizationNode], id: &str) -> String {
    nodes.iter()
        .find(|n| n.id == id)
        .map(|n| n.name.clone())
        .unwrap_or_else(|| format!("Node_{}", &id[..8]))
}

/// Demonstrate filtering capabilities
fn demonstrate_filtering(graph_data: &brain::GraphData) {
    // Filter by node type
    let abstract_nodes: Vec<_> = graph_data.nodes.iter()
        .filter(|n| n.node_type == "Abstract")
        .collect();
    println!("   • Abstract concepts: {}", abstract_nodes.len());
    
    // Filter by confidence
    let high_confidence_nodes: Vec<_> = graph_data.nodes.iter()
        .filter(|n| n.confidence >= 0.9)
        .collect();
    println!("   • High confidence nodes (≥0.9): {}", high_confidence_nodes.len());
    
    // Filter by connections
    let well_connected_nodes: Vec<_> = graph_data.nodes.iter()
        .filter(|n| n.degree >= 3)
        .collect();
    println!("   • Well-connected nodes (≥3 connections): {}", well_connected_nodes.len());
    
    // Filter relationships by weight
    let strong_relationships: Vec<_> = graph_data.edges.iter()
        .filter(|e| e.weight >= 0.8)
        .collect();
    println!("   • Strong relationships (≥0.8 weight): {}", strong_relationships.len());
} 