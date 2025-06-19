//! Memory Timeline Visualization Demo
//!
//! This demo showcases the memory timeline visualization capabilities,
//! demonstrating how episodic memory events are displayed chronologically
//! with interactive filtering and exploration features.

use brain::memory::MemorySystem;
use brain::visualization::{VisualizationManager, VisualizationConfig};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("🧠 Brain AI - Memory Timeline Visualization Demo");
    println!("=================================================");
    
    // Create memory system with episodic database
    let db_path = "demo_memory.db";
    let memory_system = MemorySystem::with_episodic_db(100, db_path)?; // 100 item working memory capacity
    
    // Create visualization manager
    let viz_config = VisualizationConfig::default();
    let viz_manager = VisualizationManager::new(viz_config);
    
    // Generate timeline data (uses sample data for demo)
    println!("\n⏰ Generating memory timeline data...");
    let timeline_data = viz_manager.generate_memory_timeline_data(&memory_system).await?;
    
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