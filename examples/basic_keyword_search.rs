#!/usr/bin/env cargo run --example basic_keyword_search
//! Basic Keyword Search Test
//!
//! Tests if simple keyword pattern matching can find the stored PocketFlow knowledge.

use brain::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🔍 Basic Keyword Search Test - PocketFlow Knowledge");
    println!("{}", "=".repeat(55));

    // Initialize memory system
    let mut memory_system = MemorySystem::new(2000);
    
    println!("\n🧠 Loading Simple Test Knowledge");
    println!("{}", "-".repeat(40));

    // Load simple, direct knowledge
    let simple_knowledge = vec![
        "PocketFlow has three architecture patterns: Node-Flow, Async Parallel Processing, and Batch Optimization.",
        "Node-Flow architecture uses BaseNode and Flow classes.",
        "BatchNode optimizes LLM API costs through batching.",
        "PocketFlow enables agent-based workflows.",
        "PocketFlow is a 100-line framework.",
    ];

    for (i, knowledge) in simple_knowledge.iter().enumerate() {
        match memory_system.learn(knowledge.to_string(), Priority::High) {
            Ok(_) => println!("✅ Stored: {}", knowledge),
            Err(e) => println!("❌ Failed to store {}: {}", i + 1, e),
        }
    }

    println!("\n🔍 Testing Simple Keyword Searches");
    println!("{}", "-".repeat(40));

    // Test simple keyword searches
    let keywords = vec![
        "PocketFlow",
        "architecture",
        "patterns",
        "Node-Flow", 
        "BatchNode",
        "agent-based",
        "100-line",
        "three",
        "optimization",
    ];

    for keyword in keywords {
        println!("\n🎯 Searching for: '{}'", keyword);
        
        let query = WorkingMemoryQuery {
            content_pattern: Some(keyword.to_string()),
            priority: None,
            min_importance: None,
            created_after: None,
            limit: Some(5),
        };

        match memory_system.query_working(&query) {
            Ok(items) => {
                if !items.is_empty() {
                    println!("   ✅ Found {} items:", items.len());
                    for (i, item) in items.iter().enumerate() {
                        println!("     {}. {}", i + 1, item.content);
                    }
                } else {
                    println!("   ❌ No items found");
                }
            }
            Err(e) => {
                println!("   ❌ Search failed: {}", e);
            }
        }
    }

    println!("\n🔍 Testing Question-Based Searches");
    println!("{}", "-".repeat(40));

    // Test searches with question words
    let question_searches = vec![
        "architecture patterns",
        "Node-Flow architecture", 
        "BatchNode costs",
        "agent workflows",
        "100-line framework",
    ];

    for search_term in question_searches {
        println!("\n🎯 Searching for: '{}'", search_term);
        
        let query = WorkingMemoryQuery {
            content_pattern: Some(search_term.to_string()),
            priority: None,
            min_importance: None,
            created_after: None,
            limit: Some(3),
        };

        match memory_system.query_working(&query) {
            Ok(items) => {
                if !items.is_empty() {
                    println!("   ✅ Found {} items:", items.len());
                    for (i, item) in items.iter().enumerate() {
                        println!("     {}. {}", i + 1, item.content);
                    }
                } else {
                    println!("   ❌ No items found");
                }
            }
            Err(e) => {
                println!("   ❌ Search failed: {}", e);
            }
        }
    }

    println!("\n🔍 Testing find_related_memories");
    println!("{}", "-".repeat(40));

    let related_searches = vec![
        "architecture",
        "PocketFlow",
        "BatchNode",
    ];

    for search_term in related_searches {
        println!("\n🎯 Related search for: '{}'", search_term);
        
        match memory_system.find_related_memories(search_term, 3) {
            Ok(results) => {
                let total = results.working_results.len() + results.episodic_results.len() + results.semantic_results.len();
                if total > 0 {
                    println!("   ✅ Found {} related memories:", total);
                    for (i, item) in results.working_results.iter().enumerate() {
                        println!("     {}. {}", i + 1, item.content);
                    }
                } else {
                    println!("   ❌ No related memories found");
                }
            }
            Err(e) => {
                println!("   ❌ Related search failed: {}", e);
            }
        }
    }

    println!("\n📊 Memory System Statistics");
    println!("{}", "-".repeat(40));
    
    let stats = memory_system.get_stats();
    for (memory_type, memory_stats) in stats.iter() {
        println!("{}: {} items, {} bytes", 
                memory_type, 
                memory_stats.total_items, 
                memory_stats.size_bytes);
    }

    println!("\n✅ Basic Keyword Search Test Complete!");

    Ok(())
} 