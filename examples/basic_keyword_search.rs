#!/usr/bin/env cargo run --example basic_keyword_search
//! Basic Keyword Search Demo
//!
//! Tests if simple keyword pattern matching can find the stored PocketFlow knowledge.

use brain::{MemoryService, WorkingMemoryQuery, Priority, Result};
use brain_infra::memory::{WorkingMemoryRepository, EpisodicMemoryRepository, SemanticMemoryRepository};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "=".repeat(55));
    println!("🔍 BASIC KEYWORD SEARCH DEMO");
    println!("{}", "=".repeat(55));

    // Create memory repositories
    let working_repo = Box::new(WorkingMemoryRepository::new(100));
    let episodic_repo = Box::new(EpisodicMemoryRepository::new("memory.db").await?);
    let semantic_repo = Box::new(SemanticMemoryRepository::new());
    
    // Create memory service
    let mut memory_service = MemoryService::new(working_repo, episodic_repo, semantic_repo);
    
    println!("\n🧠 Loading Simple Test Knowledge");
    
    let simple_knowledge = vec![
        "PocketFlow is an efficient deep learning framework",
        "It optimizes neural network models for mobile deployment",
        "PocketFlow supports quantization and pruning techniques",
        "The framework reduces model size while maintaining accuracy",
        "Mobile deployment requires optimized neural networks",
        "Quantization converts float32 to lower precision formats",
        "Pruning removes unnecessary network connections",
        "The goal is faster inference on mobile devices"
    ];

    for knowledge in simple_knowledge.iter() {
        let _id = memory_service.learn(knowledge.to_string(), Priority::High).await?;
        println!("✅ Stored: {}", knowledge);
    }

    println!("\n🔍 Testing Basic Keyword Searches");
    
    let search_terms = vec!["PocketFlow", "mobile", "quantization", "pruning"];
    
    for search_term in &search_terms {
        println!("\n🎯 Searching for: '{}'", search_term);
        
        let query = WorkingMemoryQuery {
            content_pattern: Some(search_term.to_string()),
            limit: Some(5),
            ..Default::default()
        };

        let results = memory_service.query_working(&query).await?;
        
        if !results.is_empty() {
            println!("   ✅ Found {} items:", results.len());
            for (i, item) in results.iter().enumerate() {
                println!("     {}. {} (Priority: {:?}, Score: {:.2})", 
                        i + 1, item.content, item.priority, item.importance_score());
            }
        } else {
            println!("   ❌ No items found");
        }
    }

    println!("\n🔍 Testing Phrase Searches");
    
    let phrases = vec!["neural network", "deep learning", "model size"];
    
    for phrase in &phrases {
        println!("\n🎯 Searching for phrase: '{}'", phrase);
        
        let query = WorkingMemoryQuery {
            content_pattern: Some(phrase.to_string()),
            limit: Some(5),
            ..Default::default()
        };

        let results = memory_service.query_working(&query).await?;
        
        if !results.is_empty() {
            println!("   ✅ Found {} items:", results.len());
            for (i, item) in results.iter().enumerate() {
                println!("     {}. {} (Priority: {:?}, Score: {:.2})", 
                        i + 1, item.content, item.priority, item.importance_score());
            }
        } else {
            println!("   ❌ No items found");
        }
    }

    println!("\n🔄 Testing Cross-Memory Search");
    
    let search_terms = vec!["optimization", "framework", "accuracy"];
    
    for search_term in &search_terms {
        println!("\n🎯 Cross-memory search for: '{}'", search_term);
        
        let results = memory_service.query_all_memories(search_term).await?;
        
        let total = results.working_results.len() + results.episodic_results.len() + results.semantic_results.len();
        if total > 0 {
            println!("   ✅ Found {} total memories:", total);
            for (i, item) in results.working_results.iter().enumerate() {
                println!("     {}. {} (Priority: {:?}, Score: {:.2})", 
                        i + 1, item.content, item.priority, item.importance_score());
            }
        } else {
            println!("   ❌ No memories found");
        }
    }

    println!("\n📊 Memory Statistics Summary");
    println!("{}", "-".repeat(40));
    
    // Since MemoryService doesn't have get_stats, we check individual repositories
    println!("✅ Search demo completed successfully!");
    println!("   - Stored {} knowledge items", simple_knowledge.len());
    println!("   - Tested keyword and phrase searches");
    println!("   - Demonstrated cross-memory queries");
    
    Ok(())
} 