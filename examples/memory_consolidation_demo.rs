//! # Memory Consolidation and Cross-Memory Operations Demo
//! 
//! Demonstrates the advanced memory consolidation and cross-memory operations
//! implemented in Task 3.3, including:
//! - Advanced consolidation logic (working → episodic → semantic)
//! - Cross-memory query capabilities
//! - Background maintenance processes
//! - Pattern extraction from episodic to semantic memory
//! - Comprehensive memory analysis and reporting

use anyhow::Result;
use brain::memory::*;

fn main() -> Result<()> {
    println!("🧠 Memory Consolidation and Cross-Memory Operations Demo");
    println!("========================================================\n");

    // Initialize memory system with episodic database
    let mut system = MemorySystem::with_episodic_db(8, "consolidation_demo.db")?;
    
    // Configure for demonstration purposes
    let mut config = ConsolidationConfig::default();
    config.working_to_episodic_hours = 0; // Immediate consolidation
    config.min_access_count = 3;
    config.importance_threshold = 2.0;
    config.semantic_extraction_threshold = 0.6;
    system.configure_consolidation(config);

    println!("📚 Phase 1: Learning and Memory Population");
    println!("==========================================");

    // Learn various types of information
    let weather_id = system.learn("User frequently asks about weather conditions".to_string(), Priority::High)?;
    let news_id = system.learn("User frequently asks about current news".to_string(), Priority::High)?;
    let sports_id = system.learn("User frequently asks about sports scores".to_string(), Priority::Medium)?;
    let music_id = system.learn("User occasionally asks about music recommendations".to_string(), Priority::Medium)?;
    let _tech_id = system.learn("User rarely asks about technology updates".to_string(), Priority::Low)?;

    println!("✅ Added 5 items to working memory");

    // Simulate user interactions with different access patterns
    println!("\n🔄 Phase 2: Simulating User Interactions");
    println!("========================================");

    // High-frequency interactions
    for i in 0..8 {
        system.recall_working(weather_id);
        system.recall_working(news_id);
        if i < 5 {
            system.recall_working(sports_id);
        }
        if i < 3 {
            system.recall_working(music_id);
        }
        println!("  Interaction cycle {} completed", i + 1);
    }

    // Add some semantic concepts manually
    let weather_concept = SemanticConcept::new(
        "weather_patterns".to_string(),
        "Understanding of weather-related queries and patterns".to_string(),
        vec![0.8, 0.6, 0.4, 0.2, 0.1, 0.3, 0.7, 0.5],
    );
    
    let news_concept = SemanticConcept::new(
        "news_interest".to_string(),
        "User's interest in current events and news".to_string(),
        vec![0.7, 0.8, 0.3, 0.5, 0.2, 0.6, 0.4, 0.9],
    );

    system.store_concept(weather_concept)?;
    system.store_concept(news_concept)?;
    println!("✅ Added 2 semantic concepts");

    println!("\n📊 Phase 3: Memory Analysis Before Consolidation");
    println!("===============================================");

    let analysis_before = system.analyze_memory_state();
    println!("Working Memory: {} items, {} bytes", 
             analysis_before.working_memory.total_items, 
             analysis_before.working_memory.size_bytes);
    println!("Episodic Memory: {} items", 
             analysis_before.episodic_memory.as_ref().map(|e| e.total_items).unwrap_or(0));
    println!("Semantic Memory: {} items, {} bytes", 
             analysis_before.semantic_memory.total_items, 
             analysis_before.semantic_memory.size_bytes);
    println!("Total Memory: {} items, {} bytes", 
             analysis_before.total_items, 
             analysis_before.total_size_bytes);

    println!("\n🔄 Phase 4: Advanced Consolidation Process");
    println!("=========================================");

    let consolidation_result = system.consolidate()?;
    println!("Consolidation Results:");
    println!("  Working → Episodic: {} items", consolidation_result.working_to_episodic);
    println!("  Episodic → Semantic: {} patterns", consolidation_result.episodic_to_semantic);
    println!("  Forgotten Events: {} items", consolidation_result.forgotten_events);

    println!("\n📊 Phase 5: Memory Analysis After Consolidation");
    println!("==============================================");

    let analysis_after = system.analyze_memory_state();
    println!("Working Memory: {} items, {} bytes", 
             analysis_after.working_memory.total_items, 
             analysis_after.working_memory.size_bytes);
    println!("Episodic Memory: {} items", 
             analysis_after.episodic_memory.as_ref().map(|e| e.total_items).unwrap_or(0));
    println!("Semantic Memory: {} items, {} bytes", 
             analysis_after.semantic_memory.total_items, 
             analysis_after.semantic_memory.size_bytes);
    println!("Total Memory: {} items, {} bytes", 
             analysis_after.total_items, 
             analysis_after.total_size_bytes);

    println!("\n🔍 Phase 6: Cross-Memory Query Demonstrations");
    println!("============================================");

    // Query across all memory types
    let weather_results = system.query_all_memories("weather")?;
    println!("Cross-memory search for 'weather':");
    println!("  Working Memory: {} results", weather_results.working_results.len());
    println!("  Episodic Memory: {} results", weather_results.episodic_results.len());
    println!("  Semantic Memory: {} results", weather_results.semantic_results.len());

    let news_results = system.query_all_memories("news")?;
    println!("\nCross-memory search for 'news':");
    println!("  Working Memory: {} results", news_results.working_results.len());
    println!("  Episodic Memory: {} results", news_results.episodic_results.len());
    println!("  Semantic Memory: {} results", news_results.semantic_results.len());

    // Find related memories
    let related_results = system.find_related_memories("user frequently", 5)?;
    println!("\nRelated memories for 'user frequently':");
    println!("  Working Memory: {} results", related_results.working_results.len());
    println!("  Episodic Memory: {} results", related_results.episodic_results.len());
    println!("  Semantic Memory: {} results", related_results.semantic_results.len());

    println!("\n🛠️ Phase 7: Background Maintenance Process");
    println!("==========================================");

    // Add some low-priority items to demonstrate pruning
    for i in 0..3 {
        system.learn(format!("Temporary low priority item {}", i), Priority::Low)?;
    }

    let maintenance_report = system.run_maintenance()?;
    println!("Maintenance Report:");
    println!("  Working items pruned: {}", maintenance_report.working_items_pruned);
    println!("  Episodic events forgotten: {}", maintenance_report.episodic_events_forgotten);
    println!("  Semantic concepts merged: {}", maintenance_report.semantic_concepts_merged);
    println!("  Additional consolidation:");
    println!("    Working → Episodic: {}", maintenance_report.consolidation_result.working_to_episodic);
    println!("    Episodic → Semantic: {}", maintenance_report.consolidation_result.episodic_to_semantic);
    println!("    Forgotten: {}", maintenance_report.consolidation_result.forgotten_events);

    println!("\n📈 Phase 8: Pattern Extraction Demonstration");
    println!("===========================================");

    // Add more similar patterns to trigger semantic extraction
    for i in 0..4 {
        let content = format!("User frequently asks about topic {}", i);
        let id = system.learn(content, Priority::Medium)?;
        
        // Access multiple times to create consolidation candidates
        for _ in 0..4 {
            system.recall_working(id);
        }
    }

    // Run consolidation again to see pattern extraction
    let second_consolidation = system.consolidate()?;
    println!("Second consolidation (with pattern extraction):");
    println!("  Working → Episodic: {} items", second_consolidation.working_to_episodic);
    println!("  Episodic → Semantic: {} patterns", second_consolidation.episodic_to_semantic);
    println!("  Forgotten Events: {} items", second_consolidation.forgotten_events);

    println!("\n📊 Phase 9: Final Memory State Analysis");
    println!("======================================");

    let final_analysis = system.analyze_memory_state();
    println!("Final Memory State:");
    println!("  Working Memory: {} items, {} bytes", 
             final_analysis.working_memory.total_items, 
             final_analysis.working_memory.size_bytes);
    println!("  Episodic Memory: {} items", 
             final_analysis.episodic_memory.as_ref().map(|e| e.total_items).unwrap_or(0));
    println!("  Semantic Memory: {} items, {} bytes", 
             final_analysis.semantic_memory.total_items, 
             final_analysis.semantic_memory.size_bytes);
    println!("  Total Memory: {} items, {} bytes", 
             final_analysis.total_items, 
             final_analysis.total_size_bytes);

    // Show memory evolution
    println!("\n📈 Memory Evolution Summary:");
    println!("============================");
    println!("Working Memory Change: {} → {} items", 
             analysis_before.working_memory.total_items, 
             final_analysis.working_memory.total_items);
    println!("Episodic Memory Change: {} → {} items", 
             analysis_before.episodic_memory.as_ref().map(|e| e.total_items).unwrap_or(0),
             final_analysis.episodic_memory.as_ref().map(|e| e.total_items).unwrap_or(0));
    println!("Semantic Memory Change: {} → {} items", 
             analysis_before.semantic_memory.total_items, 
             final_analysis.semantic_memory.total_items);

    println!("\n✅ Task 3.3 Demonstration Complete!");
    println!("===================================");
    println!("Successfully demonstrated:");
    println!("  ✓ Advanced consolidation logic");
    println!("  ✓ Cross-memory query capabilities");
    println!("  ✓ Background maintenance processes");
    println!("  ✓ Pattern extraction and semantic concept formation");
    println!("  ✓ Comprehensive memory analysis and reporting");
    println!("  ✓ Memory evolution tracking and optimization");

    Ok(())
} 