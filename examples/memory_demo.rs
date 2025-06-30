//! # Memory Module Demonstration
//!
//! This example demonstrates the Brain project's memory system foundation,
//! showing working memory operations, memory consolidation, and statistics.

use brain::{MemoryService, Priority, WorkingMemoryQuery, Result};
use brain_infra::memory::{WorkingMemoryRepository, EpisodicMemoryRepository, SemanticMemoryRepository};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain Memory Module Demonstration");
    println!("=======================================\n");

    // Create memory repositories
    let working_repo = Box::new(WorkingMemoryRepository::new(10));
    let episodic_repo = Box::new(EpisodicMemoryRepository::new("memory_demo.db").await?);
    let semantic_repo = Box::new(SemanticMemoryRepository::new());
    
    // Create memory service
    let mut memory_service = MemoryService::new(working_repo, episodic_repo, semantic_repo);

    println!("📝 Learning Phase - Adding information to working memory");
    println!("---------------------------------------------------------");

    // Learn various pieces of information with different priorities
    let critical_info = memory_service.learn(
        "Emergency shutdown procedure for neural network".to_string(),
        Priority::Critical,
    ).await?;
    println!("✅ Learned critical info (ID: {})", critical_info);

    let high_info = memory_service.learn(
        "User prefers transformer architecture over RNN".to_string(),
        Priority::High,
    ).await?;
    println!("✅ Learned high priority info (ID: {})", high_info);

    let medium_info = memory_service.learn(
        "Project deadline is end of quarter".to_string(),
        Priority::Medium,
    ).await?;
    println!("✅ Learned medium priority info (ID: {})", medium_info);

    let low_info = memory_service.learn(
        "Coffee machine is on the second floor".to_string(),
        Priority::Low,
    ).await?;
    println!("✅ Learned low priority info (ID: {})", low_info);

    // Simulate multiple accesses to important information
    println!("\n🔄 Access Pattern Simulation");
    println!("-----------------------------");
    
    for i in 1..=3 {
        println!("Access {} - Retrieving critical information", i);
        if let Some(item) = memory_service.recall_working(critical_info).await? {
            println!("   Retrieved: {}", item.content);
            println!("   Access count: {}, Importance: {:.3}", 
                    item.access_count, item.importance_score());
        }
        
        // Small delay to simulate time passing
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Access other items too
    let _ = memory_service.recall_working(high_info).await?;
    let _ = memory_service.recall_working(medium_info).await?;

    // Test working memory queries
    println!("\n🔍 Working Memory Queries");
    println!("-------------------------");
    
    let query = WorkingMemoryQuery {
        priority: Some(Priority::High),
        ..Default::default()
    };
    let high_priority_items = memory_service.query_working(&query).await?;
    println!("High priority items: {}", high_priority_items.len());
    for item in &high_priority_items {
        println!("   - {} (Priority: {:?})", item.content, item.priority);
    }

    // Test capacity management
    println!("\n🚀 Capacity Management Test");
    println!("-----------------------------");
    
    // Add more items to test capacity limits
    for i in 1..=8 {
        let content = format!("Additional learning item #{}", i);
        let id = memory_service.learn(content, Priority::Low).await?;
        println!("Added item {} (ID: {})", i, id);
    }

    // Query all items
    let all_query = WorkingMemoryQuery::default();
    let all_items = memory_service.query_working(&all_query).await?;
    println!("\nTotal working memory items: {}", all_items.len());

    // Test consolidation process
    println!("\n🔄 Memory Consolidation Process");
    println!("-------------------------------");
    
    let consolidation_result = memory_service.consolidate().await?;
    println!("Consolidation completed:");
    println!("  - {} items moved to episodic memory", consolidation_result.working_to_episodic);
    println!("  - {} items extracted to semantic memory", consolidation_result.episodic_to_semantic);
    println!("  - {} items forgotten", consolidation_result.forgotten_events);

    // Test cross-memory search
    println!("\n🔍 Cross-Memory Search Demo");
    println!("---------------------------");
    
    let search_terms = vec!["neural", "transformer", "deadline"];
    for term in search_terms {
        println!("\n🎯 Searching for: '{}'", term);
        let results = memory_service.query_all_memories(term).await?;
        
        let total = results.working_results.len() + results.episodic_results.len() + results.semantic_results.len();
        if total > 0 {
            println!("   Found {} total memories:", total);
            
            if !results.working_results.is_empty() {
                println!("   Working Memory:");
                for item in &results.working_results {
                    println!("     - {}", item.content);
                }
            }
            
            if !results.episodic_results.is_empty() {
                println!("   Episodic Memory:");
                for event in &results.episodic_results {
                    println!("     - {}", event.content);
                }
            }
            
            if !results.semantic_results.is_empty() {
                println!("   Semantic Memory:");
                for concept in &results.semantic_results {
                    println!("     - {} ({})", concept.name, concept.description);
                }
            }
        } else {
            println!("   No memories found");
        }
    }

    println!("\n✅ Memory Module Demonstration Complete!");
    println!("==========================================");
    println!("Key Features Demonstrated:");
    println!("• ✅ Working memory with priority-based management");
    println!("• ✅ Automatic capacity management and eviction");
    println!("• ✅ Access pattern tracking and importance scoring");
    println!("• ✅ Memory queries and filtering");
    println!("• ✅ Consolidation pipeline with episodic memory");
    println!("• ✅ Cross-memory search capabilities");
    println!("• ✅ SQLite persistence for episodic memory");
    println!("• ✅ Semantic memory with concept storage");

    Ok(())
} 