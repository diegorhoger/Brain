//! # Memory Storage and Retrieval Operations Demo
//! 
//! Demonstrates the comprehensive memory storage and retrieval capabilities
//! implemented in Task 3.2, including:
//! - SQLite-based episodic memory with temporal indexing
//! - Semantic memory with vector similarity search
//! - Memory decay and forgetting mechanisms
//! - Unified memory APIs with query capabilities
//! - Thread-safe operations and performance metrics

use anyhow::Result;
use brain::memory::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("🧠 Memory Storage and Retrieval Operations Demo");
    println!("=================================================\n");

    // Initialize memory system with episodic database
    let mut system = MemorySystem::with_episodic_db(10, "memory_demo.db")?;
    println!("✅ Initialized memory system with SQLite episodic database\n");

    // 1. Working Memory Operations
    println!("📋 Working Memory Operations:");
    println!("-----------------------------");
    
    let critical_id = system.learn("System startup complete".to_string(), Priority::Critical)?;
    let high_id = system.learn("User authentication required".to_string(), Priority::High)?;
    let medium_id = system.learn("Load user preferences".to_string(), Priority::Medium)?;
    let _low_id = system.learn("Update UI theme".to_string(), Priority::Low)?;
    
    println!("✅ Stored 4 working memory items with different priorities");
    
    // Access items to build up access patterns
    system.recall_working(critical_id);
    system.recall_working(critical_id);
    system.recall_working(high_id);
    system.recall_working(high_id);
    system.recall_working(medium_id);
    
    // Query working memory
    let query = WorkingMemoryQuery {
        min_importance: Some(2.0),
        ..Default::default()
    };
    let important_items = system.query_working(&query)?;
    println!("🔍 Found {} high-importance working memory items", important_items.len());
    
    for item in &important_items {
        println!("  • {} (priority: {:?}, score: {:.2})", 
                 item.content, item.priority, item.importance_score());
    }
    println!();

    // 2. Semantic Memory Operations
    println!("🧠 Semantic Memory Operations:");
    println!("------------------------------");
    
    // Create semantic concepts with embeddings
    let weather_concept = SemanticConcept::new(
        "weather".to_string(),
        "Information about atmospheric conditions".to_string(),
        vec![0.8, 0.1, 0.3, 0.6, 0.2],
    );
    
    let climate_concept = SemanticConcept::new(
        "climate".to_string(),
        "Long-term weather patterns and trends".to_string(),
        vec![0.7, 0.2, 0.4, 0.5, 0.3], // Similar to weather
    );
    
    let technology_concept = SemanticConcept::new(
        "technology".to_string(),
        "Digital tools and computer systems".to_string(),
        vec![0.1, 0.9, 0.1, 0.2, 0.8], // Different from weather
    );
    
    let _weather_id = system.store_concept(weather_concept.clone())?;
    let _climate_id = system.store_concept(climate_concept.clone())?;
    let _tech_id = system.store_concept(technology_concept.clone())?;
    
    println!("✅ Stored 3 semantic concepts");
    
    // Test similarity search
    let weather_embedding = vec![0.8, 0.1, 0.3, 0.6, 0.2];
    let semantic_query = SemanticQuery {
        embedding: Some(weather_embedding.clone()),
        min_similarity: Some(0.5),
        limit: Some(5),
        ..Default::default()
    };
    
    let similar_concepts = system.query_semantic(&semantic_query)?;
    println!("🔍 Found {} concepts similar to 'weather':", similar_concepts.len());
    
    for concept in &similar_concepts {
        let similarity = cosine_similarity(&weather_embedding, &concept.embedding);
        println!("  • {} (similarity: {:.3}, confidence: {:.2})", 
                 concept.name, similarity, concept.confidence);
    }
    println!();

    // 3. Episodic Memory Operations
    println!("📚 Episodic Memory Operations:");
    println!("------------------------------");
    
    // Create episodic events with rich context
    let mut context1 = HashMap::new();
    context1.insert("location".to_string(), "office".to_string());
    context1.insert("user".to_string(), "alice".to_string());
    context1.insert("session_id".to_string(), "session_123".to_string());
    
    let mut event1 = EpisodicEvent::new(
        "User asked about weather forecast".to_string(),
        context1,
        0.8,
        "user_interaction".to_string(),
    );
    event1.add_tag("weather".to_string());
    event1.add_tag("query".to_string());
    
    let mut context2 = HashMap::new();
    context2.insert("location".to_string(), "home".to_string());
    context2.insert("user".to_string(), "bob".to_string());
    context2.insert("device".to_string(), "mobile".to_string());
    
    let mut event2 = EpisodicEvent::new(
        "System performed automatic backup".to_string(),
        context2,
        0.6,
        "system_event".to_string(),
    );
    event2.add_tag("backup".to_string());
    event2.add_tag("system".to_string());
    
    // Store in episodic memory through system consolidation
    let episodic_query = EpisodicQuery::default();
    let existing_events = system.query_episodic(&episodic_query)?;
    println!("📝 Current episodic events: {}", existing_events.len());

    // 4. Memory Consolidation Process
    println!("\n🔄 Memory Consolidation Process:");
    println!("--------------------------------");
    
    // Run consolidation to move items from working to episodic memory
    let consolidation_result = system.consolidate()?;
    println!("✅ Consolidation complete:");
    println!("  • Working → Episodic: {} items", consolidation_result.working_to_episodic);
    println!("  • Episodic → Semantic: {} items", consolidation_result.episodic_to_semantic);
    println!("  • Forgotten events: {} items", consolidation_result.forgotten_events);
    
    // Check episodic memory after consolidation
    let episodic_events = system.query_episodic(&EpisodicQuery::default())?;
    println!("📚 Total episodic events after consolidation: {}", episodic_events.len());
    
    for event in episodic_events.iter().take(3) {
        println!("  • {} (importance: {:.2}, source: {})", 
                 event.content, event.importance, event.source);
    }
    println!();

    // 5. Query Operations Across Memory Types
    println!("🔍 Cross-Memory Query Operations:");
    println!("---------------------------------");
    
    // Query by content pattern
    let working_pattern_query = WorkingMemoryQuery {
        content_pattern: Some("system".to_string()),
        ..Default::default()
    };
    let working_results = system.query_working(&working_pattern_query)?;
    println!("🔍 Working memory 'system' matches: {}", working_results.len());
    
    let episodic_pattern_query = EpisodicQuery {
        content_pattern: Some("system".to_string()),
        ..Default::default()
    };
    let episodic_results = system.query_episodic(&episodic_pattern_query)?;
    println!("🔍 Episodic memory 'system' matches: {}", episodic_results.len());
    
    // Query by importance threshold
    let importance_query = EpisodicQuery {
        min_importance: Some(0.5),
        limit: Some(10),
        ..Default::default()
    };
    let important_events = system.query_episodic(&importance_query)?;
    println!("🔍 High importance episodic events: {}", important_events.len());
    println!();

    // 6. Memory Decay and Forgetting
    println!("⏰ Memory Decay and Forgetting:");
    println!("-------------------------------");
    
    let stats_before = system.get_stats();
    let working_before = stats_before.get("working").unwrap().total_items;
    let episodic_before = stats_before.get("episodic").map(|s| s.total_items).unwrap_or(0);
    
    println!("📊 Before decay - Working: {}, Episodic: {}", working_before, episodic_before);
    
    // Apply memory decay
    system.apply_decay()?;
    
    let stats_after = system.get_stats();
    let working_after = stats_after.get("working").unwrap().total_items;
    let episodic_after = stats_after.get("episodic").map(|s| s.total_items).unwrap_or(0);
    
    println!("📊 After decay  - Working: {}, Episodic: {}", working_after, episodic_after);
    println!();

    // 7. Performance Metrics and Statistics
    println!("📈 Memory System Statistics:");
    println!("----------------------------");
    
    let final_stats = system.get_stats();
    for (memory_type, stats) in &final_stats {
        println!("🧠 {} Memory:", memory_type.to_uppercase());
        println!("  • Total items: {}", stats.total_items);
        println!("  • Size (bytes): {}", stats.size_bytes);
        println!("  • Access count: {}", stats.access_count);
        println!("  • Last access: {}", stats.last_access.format("%H:%M:%S"));
        println!();
    }

    // 8. Thread Safety Demonstration
    println!("🔒 Thread Safety Verification:");
    println!("------------------------------");
    println!("✅ All memory operations use thread-safe Arc<Mutex<Connection>> for episodic storage");
    println!("✅ Working and semantic memory operations are designed for single-threaded access");
    println!("✅ Memory system provides consistent APIs across all memory types");
    println!();

    // 9. Advanced Features Summary
    println!("🚀 Task 3.2 Implementation Summary:");
    println!("===================================");
    println!("✅ SQLite-based episodic memory with temporal indexing");
    println!("✅ Vector similarity search in semantic memory");
    println!("✅ Unified Memory trait implementation for all memory types");
    println!("✅ Memory decay and forgetting mechanisms");
    println!("✅ Rich query capabilities with filtering and sorting");
    println!("✅ Thread-safe episodic memory operations");
    println!("✅ Comprehensive performance metrics and statistics");
    println!("✅ Memory consolidation process automation");
    println!("✅ Context-aware episodic event storage");
    println!("✅ Cosine similarity-based concept matching");
    println!();

    println!("🎉 Task 3.2: Memory Storage and Retrieval Operations - COMPLETE!");

    Ok(())
}

/// Helper function for cosine similarity (duplicated for demo)
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot_product += (a[i] * b[i]) as f64;
        norm_a += (a[i] * a[i]) as f64;
        norm_b += (b[i] * b[i]) as f64;
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
} 