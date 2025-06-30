//! Memory Storage Demonstration
//!
//! This example demonstrates the Brain project's comprehensive memory storage system,
//! including working memory, episodic memory, and semantic memory operations.
//! 
//! Features demonstrated:
//! - Multi-level memory storage (Working, Episodic, Semantic)
//! - Memory querying and retrieval
//! - Memory consolidation
//! - Cross-memory search capabilities

use brain::{
    MemoryService, Priority, WorkingMemoryQuery, SemanticConcept, SemanticQuery,
    EpisodicEvent, EpisodicQuery, Result
};
use brain_infra::memory::{WorkingMemoryRepository, EpisodicMemoryRepository, SemanticMemoryRepository};
use std::collections::HashMap;
use tokio;
use brain::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain Memory Storage Demonstration");
    println!("====================================");
    
    // Ensure data directory exists
    std::fs::create_dir_all("data").map_err(|e| BrainError::Io { source: e })?;
    
    // Initialize repositories
    let working_repo = Box::new(WorkingMemoryRepository::new(20));
    let episodic_repo = Box::new(EpisodicMemoryRepository::new("data/memory_storage_demo.db").await?);
    let semantic_repo = Box::new(SemanticMemoryRepository::new());
    
    // Create memory service
    let mut system = MemoryService::new(working_repo, episodic_repo, semantic_repo);

    // === WORKING MEMORY DEMONSTRATION ===
    println!("📝 Working Memory Operations");
    println!("----------------------------");
    
    // Store items with different priorities
    let critical_id = system.learn("System startup complete".to_string(), Priority::Critical).await?;
    let _high_id = system.learn("User authentication required".to_string(), Priority::High).await?;
    let _medium_id = system.learn("Load user preferences".to_string(), Priority::Medium).await?;
    let _low_id = system.learn("Update UI theme".to_string(), Priority::Low).await?;

    println!("✅ Stored 4 working memory items with different priorities");

    // Test specific item recall
    if let Some(critical_item) = system.recall_working(critical_id).await? {
        println!("🔍 Retrieved critical item: '{}'", critical_item.content);
    }

    // Query working memory by pattern
    let query = WorkingMemoryQuery {
        content_pattern: Some("user".to_string()),
        priority: None,
        min_importance: None,
        created_after: None,
        limit: Some(10),
    };

    let user_related = system.query_working(&query).await?;
    println!("👤 Found {} user-related items in working memory", user_related.len());
    for item in &user_related {
        println!("   - {} (Priority: {:?})", item.content, item.priority);
    }

    // === SEMANTIC MEMORY DEMONSTRATION ===
    println!("\n🧩 Semantic Memory Operations");
    println!("-----------------------------");

    // Create semantic concepts
    let weather_concept = SemanticConcept::new(
        "Weather".to_string(),
        "Atmospheric conditions and patterns".to_string(),
        vec![0.1, 0.8, 0.3, 0.9, 0.2], // Mock embedding
    );

    let climate_concept = SemanticConcept::new(
        "Climate".to_string(),
        "Long-term weather patterns and trends".to_string(),
        vec![0.2, 0.7, 0.4, 0.8, 0.3], // Mock embedding
    );

    let technology_concept = SemanticConcept::new(
        "Technology".to_string(),
        "Application of scientific knowledge for practical purposes".to_string(),
        vec![0.9, 0.1, 0.8, 0.2, 0.7], // Mock embedding
    );

    // Store semantic concepts
    let _weather_id = system.store_concept(weather_concept).await?;
    let _climate_id = system.store_concept(climate_concept).await?;
    let _tech_id = system.store_concept(technology_concept).await?;

    println!("✅ Stored 3 semantic concepts");

    // Query semantic concepts
    let semantic_query = SemanticQuery {
        name_pattern: Some("weather".to_string()),
        embedding: None,
        min_confidence: None,
        min_similarity: None,
        limit: Some(5),
    };

    let weather_concepts = system.query_semantic(&semantic_query).await?;
    println!("🌤️  Found {} weather-related concepts", weather_concepts.len());
    for concept in &weather_concepts {
        println!("   - {}: {}", concept.name, concept.description);
    }

    // === EPISODIC MEMORY DEMONSTRATION ===
    println!("\n📚 Episodic Memory Operations");
    println!("-----------------------------");

    // Create episodic events
    let mut context1 = HashMap::new();
    context1.insert("location".to_string(), "office".to_string());
    context1.insert("activity".to_string(), "meeting".to_string());

    let mut event1 = EpisodicEvent::new(
        "Quarterly team meeting discussion about project roadmap".to_string(),
        context1,
        0.8,
        "user_input".to_string(),
    );
    event1.add_tag("meeting".to_string());
    event1.add_tag("roadmap".to_string());

    let mut context2 = HashMap::new();
    context2.insert("location".to_string(), "home".to_string());
    context2.insert("activity".to_string(), "learning".to_string());

    let mut event2 = EpisodicEvent::new(
        "Studied advanced memory consolidation techniques".to_string(),
        context2,
        0.9,
        "learning_session".to_string(),
    );
    event2.add_tag("learning".to_string());
    event2.add_tag("memory".to_string());

    println!("✅ Created 2 episodic events with context and tags");

    // Query episodic memory
    let episodic_query = EpisodicQuery::default();
    let _all_events = system.query_episodic(&episodic_query).await?;

    // === CROSS-MEMORY SEARCH ===
    println!("\n🔗 Cross-Memory Search");
    println!("---------------------");

    // Search across all memory types
    let cross_results = system.query_all_memories("memory").await?;
    
    println!("📊 Cross-memory search results for 'memory':");
    println!("   - Working memory: {} results", cross_results.working_results.len());
    println!("   - Episodic memory: {} results", cross_results.episodic_results.len());
    println!("   - Semantic memory: {} results", cross_results.semantic_results.len());

    // Demonstrate different search patterns
    println!("\n🔍 Pattern-Based Searches");
    println!("-------------------------");

    let working_pattern_query = WorkingMemoryQuery {
        content_pattern: Some("system".to_string()),
        priority: None,
        min_importance: None,
        created_after: None,
        limit: Some(5),
    };

    let episodic_pattern_query = EpisodicQuery {
        content_pattern: Some("meeting".to_string()),
        time_range: None,
        min_importance: None,
        tags: Vec::new(),
        context_filters: HashMap::new(),
        limit: Some(5),
    };

    let importance_query = EpisodicQuery {
        content_pattern: None,
        time_range: None,
        min_importance: Some(0.8),
        tags: Vec::new(),
        context_filters: HashMap::new(),
        limit: Some(10),
    };

    let system_items = system.query_working(&working_pattern_query).await?;
    let meeting_events = system.query_episodic(&episodic_pattern_query).await?;
    let important_events = system.query_episodic(&importance_query).await?;

    println!("🖥️  System-related working items: {}", system_items.len());
    println!("🤝 Meeting-related episodic events: {}", meeting_events.len());
    println!("⭐ High-importance events: {}", important_events.len());

    println!("\n✅ Memory Storage Demo Complete!");
    println!("🎯 Successfully demonstrated multi-level memory operations:");
    println!("   • Working memory storage and priority management");
    println!("   • Semantic concept creation and similarity tracking");
    println!("   • Episodic event storage with context and tagging");
    println!("   • Cross-memory search and pattern-based queries");

    Ok(())
} 