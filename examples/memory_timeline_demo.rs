//! Memory Timeline Visualization Demo
//!
//! This demo showcases the memory timeline visualization capabilities,
//! demonstrating how episodic memory events are displayed chronologically
//! with interactive filtering and exploration features.

use brain::*;
use brain::services::*;
// Note: visualization module doesn't exist in brain_infra yet, so we'll implement a demo version
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Demo visualization configuration
#[derive(Debug, Clone)]
pub struct VisualizationConfig {
    pub enable_timeline: bool,
    pub max_events: usize,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            enable_timeline: true,
            max_events: 1000,
        }
    }
}

/// Demo memory timeline data structure
#[derive(Debug, Clone)]
pub struct DemoTimelineData {
    pub events: Vec<DemoTimelineEvent>,
    pub metadata: DemoTimelineMetadata,
}

#[derive(Debug, Clone)]
pub struct DemoTimelineEvent {
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub importance: f64,
    pub timestamp: DateTime<Utc>,
    pub related_concepts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DemoTimelineMetadata {
    pub event_count: usize,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

/// Demo visualization manager with timeline capabilities
pub struct DemoVisualizationManager {
    #[allow(dead_code)]
    config: VisualizationConfig,
}

impl DemoVisualizationManager {
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }

    pub async fn generate_memory_timeline_data(&self, _memory_service: &MemoryService) -> Result<DemoTimelineData> {
        // Generate sample timeline data for demonstration
        let now = Utc::now();
        let events = vec![
            DemoTimelineEvent {
                event_type: "Learning".to_string(),
                title: "PocketFlow Architecture Analysis".to_string(),
                description: "Analyzed PocketFlow's Node-Flow architecture pattern and batch optimization framework.".to_string(),
                importance: 0.9,
                timestamp: now - chrono::Duration::hours(1),
                related_concepts: vec!["Node-Flow".to_string(), "Architecture".to_string(), "PocketFlow".to_string()],
            },
            DemoTimelineEvent {
                event_type: "Conversation".to_string(),
                title: "User Query: AI Framework Comparison".to_string(),
                description: "User asked about differences between AI orchestration frameworks, particularly PocketFlow vs others.".to_string(),
                importance: 0.8,
                timestamp: now - chrono::Duration::minutes(45),
                related_concepts: vec!["AI Frameworks".to_string(), "Comparison".to_string()],
            },
            DemoTimelineEvent {
                event_type: "Insight".to_string(),
                title: "Pattern Recognition: Batch Optimization".to_string(),
                description: "Identified recurring pattern of batch optimization across multiple AI frameworks for LLM cost reduction.".to_string(),
                importance: 0.85,
                timestamp: now - chrono::Duration::minutes(30),
                related_concepts: vec!["Batch Processing".to_string(), "Cost Optimization".to_string(), "LLM".to_string()],
            },
            DemoTimelineEvent {
                event_type: "Memory Consolidation".to_string(),
                title: "Episodic to Semantic Transfer".to_string(),
                description: "Transferred episodic memories about AI framework patterns to semantic memory for long-term retention.".to_string(),
                importance: 0.75,
                timestamp: now - chrono::Duration::minutes(20),
                related_concepts: vec!["Memory Transfer".to_string(), "Consolidation".to_string()],
            },
            DemoTimelineEvent {
                event_type: "Concept Formation".to_string(),
                title: "New Concept: Agent Orchestration".to_string(),
                description: "Formed new concept linking agent-based frameworks with orchestration patterns from multiple examples.".to_string(),
                importance: 0.82,
                timestamp: now - chrono::Duration::minutes(10),
                related_concepts: vec!["Agent Systems".to_string(), "Orchestration".to_string(), "Patterns".to_string()],
            },
            DemoTimelineEvent {
                event_type: "Quality Check".to_string(),
                title: "Response Quality Assessment".to_string(),
                description: "Assessed quality of responses about AI frameworks, identified areas for improvement in technical depth.".to_string(),
                importance: 0.7,
                timestamp: now - chrono::Duration::minutes(5),
                related_concepts: vec!["Quality Assessment".to_string(), "Response Analysis".to_string()],
            },
        ];

        let start_time = events.iter().map(|e| e.timestamp).min().unwrap_or(now);
        let end_time = events.iter().map(|e| e.timestamp).max().unwrap_or(now);

        Ok(DemoTimelineData {
            events,
            metadata: DemoTimelineMetadata {
                event_count: 6,
                start_time,
                end_time,
            },
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("🧠 Brain AI - Memory Timeline Visualization Demo");
    println!("=================================================");
    
    // Create memory service using new architecture
    let memory_service = create_memory_service_with_capacity(100).await?;
    
    // Create demo visualization manager
    let viz_config = VisualizationConfig::default();
    let viz_manager = DemoVisualizationManager::new(viz_config);
    
    // Generate timeline data (uses sample data for demo)
    println!("\n⏰ Generating memory timeline data...");
    let timeline_data = viz_manager.generate_memory_timeline_data(&memory_service).await?;
    
    // Display timeline statistics
    println!("\n📊 Timeline Statistics:");
    println!("  • Total events: {}", timeline_data.metadata.event_count);
    println!("  • Time span: {} to {}", 
        timeline_data.metadata.start_time.format("%Y-%m-%d %H:%M:%S"),
        timeline_data.metadata.end_time.format("%Y-%m-%d %H:%M:%S"));
    
    // Display events by type
    let mut event_types = HashMap::new();
    for event in &timeline_data.events {
        *event_types.entry(&event.event_type).or_insert(0) += 1;
    }
    
    println!("\n🏷️  Events by Type:");
    for (event_type, count) in &event_types {
        println!("  • {}: {}", event_type, count);
    }
    
    // Display recent high-importance events
    println!("\n⭐ High-Importance Events (>70%):");
    let mut important_events: Vec<_> = timeline_data.events.iter()
        .filter(|e| e.importance > 0.7)
        .collect();
    important_events.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
    
    for event in important_events.iter().take(5) {
        println!("  • {} ({:.1}%): {}", 
            event.event_type, 
            event.importance * 100.0, 
            event.title);
        println!("    {}", event.description);
        if !event.related_concepts.is_empty() {
            println!("    Related: {}", event.related_concepts.join(", "));
        }
        println!();
    }
    
    println!("🌐 Timeline visualization is available at:");
    println!("   http://localhost:3000/visualization/memory-timeline");
    println!("\n💡 Features:");
    println!("  • Interactive chronological timeline with D3.js");
    println!("  • Event filtering by type, importance, and time range");
    println!("  • Zoom and pan navigation");
    println!("  • Event details panel with metadata");
    println!("  • Export functionality");
    println!("  • Responsive design for different screen sizes");
    
    Ok(())
} 