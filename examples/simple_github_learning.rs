#!/usr/bin/env cargo run --example simple_github_learning
//! Simple GitHub Learning Example
//!
//! This example demonstrates how to use Brain AI to learn from a GitHub repository
//! and then query the learned information.
//!
//! Usage:
//!     cargo run --example simple_github_learning
//!     GITHUB_TOKEN=your_token cargo run --example simple_github_learning

use brain::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Brain AI with memory capacity
    let mut brain = MemorySystem::new(500);
    
    // Set up GitHub learning
    let github_token = env::var("GITHUB_TOKEN").ok();
    let config = GitHubLearningConfig {
        max_files: 20,           // Limit to 20 files for quick demo
        max_file_size: 30_000,   // 30KB max per file
        include_code: true,
        include_docs: true,
        include_config: false,   // Skip config files for this demo
        ..Default::default()
    };
    
    let github_engine = GitHubLearningEngine::new(github_token, Some(config));
    
    println!("🧠 Brain AI - Simple GitHub Learning Example");
    println!("Learning from 'rust-lang/mdbook' repository...\n");
    
    // Learn from repository
    match github_engine.learn_from_repository(&mut brain, "rust-lang/mdbook").await {
        Ok(result) => {
            println!("✅ Learning completed!");
            println!("   Files processed: {}", result.files_processed);
            println!("   Concepts discovered: {}", result.concepts_discovered);
            println!("   Learning time: {}ms", result.learning_time_ms);
            println!("   Summary: {}\n", result.summary);
        }
        Err(e) => {
            println!("❌ Learning failed: {}", e);
            return Err(e);
        }
    }
    
    // Query what we learned
    println!("🔍 Querying learned information:");
    
    let queries = vec![
        "markdown",
        "documentation", 
        "Rust",
    ];
    
    for query in queries {
        println!("\n🔎 Query: '{}'", query);
        
        match brain.query_all_memories(query) {
            Ok(results) => {
                let total = results.working_results.len() + 
                          results.episodic_results.len() + 
                          results.semantic_results.len();
                          
                if total > 0 {
                    println!("   Found {} related memories", total);
                    
                    // Show first working memory result
                    if let Some(first) = results.working_results.first() {
                        let preview = if first.content.len() > 100 {
                            format!("{}...", &first.content[..97])
                        } else {
                            first.content.clone()
                        };
                        println!("   Example: {}", preview);
                    }
                } else {
                    println!("   No results found");
                }
            }
            Err(e) => {
                println!("   Query error: {}", e);
            }
        }
    }
    
    // Show memory statistics
    println!("\n📊 Brain Memory Statistics:");
    let stats = brain.get_stats();
    
    if let Some(working_stats) = stats.get("working") {
        println!("   Working Memory: {} items", working_stats.total_items);
    }
    if let Some(semantic_stats) = stats.get("semantic") {
        println!("   Semantic Memory: {} concepts", semantic_stats.total_items);
    }
    
    let total_size: usize = stats.values().map(|s| s.size_bytes).sum();
    println!("   Total size: {} KB", total_size / 1024);
    
    println!("\n🎉 Example completed! Brain AI successfully learned from the repository.");
    
    Ok(())
} 