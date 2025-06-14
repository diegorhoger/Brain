//! # Memory Module Demonstration
//!
//! This example demonstrates the Brain project's memory system foundation,
//! showing working memory operations, memory consolidation, and statistics.

use anyhow::Result;
use brain::{MemorySystem, Priority};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    println!("🧠 Brain Memory Module Demonstration");
    println!("=======================================\n");

    // Create a memory system with capacity for 10 working memory items
    let mut memory_system = MemorySystem::new(10);

    println!("📝 Learning Phase - Adding information to working memory");
    println!("---------------------------------------------------------");

    // Learn various pieces of information with different priorities
    let critical_info = memory_system.learn(
        "Emergency shutdown procedure for neural network".to_string(),
        Priority::Critical,
    )?;
    println!("✅ Learned critical info (ID: {})", critical_info);

    let high_info = memory_system.learn(
        "User prefers transformer architecture over RNN".to_string(),
        Priority::High,
    )?;
    println!("✅ Learned high priority info (ID: {})", high_info);

    let medium_info = memory_system.learn(
        "Project deadline is end of quarter".to_string(),
        Priority::Medium,
    )?;
    println!("✅ Learned medium priority info (ID: {})", medium_info);

    let low_info = memory_system.learn(
        "Coffee machine is on the second floor".to_string(),
        Priority::Low,
    )?;
    println!("✅ Learned low priority info (ID: {})", low_info);

    // Simulate multiple accesses to important information
    println!("\n🔄 Access Pattern Simulation");
    println!("-----------------------------");
    
    for i in 1..=3 {
        println!("Access {} - Retrieving critical information", i);
        if let Some(item) = memory_system.recall_working(critical_info) {
            println!("   Retrieved: {}", item.content);
            println!("   Access count: {}, Importance: {:.3}", 
                    item.access_count, item.importance_score());
        }
        
        // Small delay to simulate time passing
        thread::sleep(Duration::from_millis(100));
    }

    // Access other items too
    memory_system.recall_working(high_info);
    memory_system.recall_working(medium_info);

    println!("\n📊 Memory System Statistics");
    println!("----------------------------");
    
    let stats = memory_system.get_stats();
    for (memory_type, stat) in stats {
        println!("📈 {} Memory:", memory_type.to_uppercase());
        println!("   Total items: {}", stat.total_items);
        println!("   Size (bytes): {}", stat.size_bytes);
        println!("   Access count: {}", stat.access_count);
        println!("   Last access: {}", stat.last_access.format("%H:%M:%S"));
    }

    // Test capacity management
    println!("\n🚀 Capacity Management Test");
    println!("-----------------------------");
    
    // Add more items to test capacity limits
    for i in 1..=8 {
        let content = format!("Additional learning item #{}", i);
        let id = memory_system.learn(content, Priority::Low)?;
        println!("Added item {} (ID: {})", i, id);
    }

    println!("\nFinal memory stats after capacity test:");
    let final_stats = memory_system.get_stats();
    for (memory_type, stat) in final_stats {
        println!("📈 {} Memory - {} items", memory_type.to_uppercase(), stat.total_items);
    }

    // Test consolidation process
    println!("\n🔄 Memory Consolidation Process");
    println!("-------------------------------");
    
    let consolidation_result = memory_system.consolidate()?;
    println!("Consolidation completed: {} items moved to episodic memory", consolidation_result.working_to_episodic);
    println!("Note: Full consolidation implementation will be completed in subtask 3.2");

    println!("\n✅ Memory Module Demonstration Complete!");
    println!("==========================================");
    println!("Key Features Demonstrated:");
    println!("• ✅ Working memory with priority-based management");
    println!("• ✅ Automatic capacity management and eviction");
    println!("• ✅ Access pattern tracking and importance scoring");
    println!("• ✅ Memory statistics and monitoring");
    println!("• ✅ Consolidation candidate identification");
    println!("\nNext Steps (Task 3 Subtasks):");
    println!("• 🔲 3.2: Episodic memory with SQLite persistence");
    println!("• 🔲 3.3: Semantic memory with vector similarity");
    println!("• 🔲 3.4: Full consolidation pipeline integration");

    Ok(())
} 