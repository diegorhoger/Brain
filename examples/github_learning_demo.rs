#!/usr/bin/env cargo run --example github_learning_demo
//! GitHub Learning Demo for Brain AI (Rust Implementation)
//!
//! This comprehensive demo showcases Brain AI's ability to learn from GitHub
//! repositories using real API integration, analyzing code patterns, documentation,
//! and project structure.
//!
//! Features demonstrated:
//! - Real repository learning with GitHub API
//! - Memory storage and intelligent querying
//! - Concept discovery and relationship mapping
//! - Performance monitoring and error handling
//! - Export capabilities and data analysis
//!
//! Usage:
//!     cargo run --example github_learning_demo
//!
//! Environment Variables:
//!     GITHUB_TOKEN: Optional GitHub personal access token for higher rate limits
//!
//! Requirements:
//!     - Internet connection for repository access
//!     - Optional: GitHub token for private repos or higher rate limits

use brain::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🧠 Brain AI - GitHub Repository Learning Demo (Rust)");
    println!("{}", "=".repeat(60));

    // Initialize Brain AI components with working memory capacity
    let mut memory_system = MemorySystem::new(1000); // 1000 item capacity
    let github_token = env::var("GITHUB_TOKEN").ok();
    
    if github_token.is_some() {
        println!("✅ GitHub token found - using authenticated API");
    } else {
        println!("⚠️  No GitHub token - using public API (rate limited)");
        println!("   Set GITHUB_TOKEN environment variable for better performance");
    }

    // Create GitHub learning engine with configuration
    let config = GitHubLearningConfig {
        max_files: 50,
        max_file_size: 50_000, // 50KB per file
        include_code: true,
        include_docs: true,
        include_config: true,
        ..Default::default()
    };

    let github_engine = GitHubLearningEngine::new(github_token.clone(), Some(config));

    // Example repositories to learn from
    let repositories = vec![
        "rust-lang/mdbook",           // Documentation tool
        "BurntSushi/ripgrep",        // Command-line tool
        "tokio-rs/tokio",            // Async runtime (smaller subset)
    ];

    println!("\n🚀 Starting GitHub Repository Learning");
    println!("{}", "-".repeat(40));

    for repo_url in repositories {
        println!("\n📂 Learning from repository: {}", repo_url);
        
        let start_time = std::time::Instant::now();
        
        match github_engine.learn_from_repository(&mut memory_system, repo_url).await {
            Ok(result) => {
                let duration = start_time.elapsed();
                
                println!("✅ Learning completed successfully!");
                println!("   Repository: {}", result.repository);
                println!("   Files processed: {}", result.files_processed);
                println!("   Total content size: {} bytes", result.total_content_size);
                println!("   Concepts discovered: {}", result.concepts_discovered);
                println!("   Memory entries created: {}", result.memory_entries_created);
                println!("   Learning time: {}ms", result.learning_time_ms);
                println!("   Total time: {:.2}s", duration.as_secs_f64());
                
                println!("\n📋 Summary: {}", result.summary);
                
                if !result.key_insights.is_empty() {
                    println!("\n💡 Key Insights:");
                    for (i, insight) in result.key_insights.iter().enumerate() {
                        println!("   {}. {}", i + 1, insight);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to learn from {}: {}", repo_url, e);
                
                // Provide helpful error guidance
                match e {
                    BrainError::NetworkError(_) => {
                        println!("   💡 Check your internet connection");
                    }
                    BrainError::InvalidInput(_) => {
                        println!("   💡 Repository URL format might be invalid");
                    }
                    BrainError::NotFound(_) => {
                        println!("   💡 Repository not found or might be private - set GITHUB_TOKEN");
                    }
                    _ => {
                        println!("   💡 See error message above for details");
                    }
                }
                
                continue; // Try next repository
            }
        }
        
        // Small delay between repositories to be respectful to GitHub API
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    // Demonstrate memory querying capabilities
    demonstrate_memory_queries(&memory_system).await?;
    
    // Demonstrate concept relationships
    demonstrate_concept_analysis(&memory_system).await?;
    
    // Show memory statistics
    demonstrate_memory_statistics(&memory_system).await?;

    println!("\n🎉 GitHub Learning Demo Completed Successfully!");
    println!("{}", "=".repeat(60));
    
    Ok(())
}

async fn demonstrate_memory_queries(memory_system: &MemorySystem) -> Result<()> {
    println!("\n🔍 Memory Querying and Information Retrieval");
    println!("{}", "-".repeat(40));

    let queries = vec![
        "documentation generation",
        "Rust programming patterns", 
        "async runtime",
        "command line tools",
        "configuration management",
    ];

    for query in queries {
        println!("\n🔎 Query: '{}'", query);
        
        // Query cross-memory to search all memory types at once
        match memory_system.query_all_memories(query) {
            Ok(results) => {
                let total_results = results.working_results.len() + 
                                  results.episodic_results.len() + 
                                  results.semantic_results.len();
                                  
                if total_results == 0 {
                    println!("   No results found");
                } else {
                    println!("   Found {} results:", total_results);
                    
                    // Show working memory results
                    for (i, result) in results.working_results.iter().take(2).enumerate() {
                        println!("   {}. {} (Working Memory)", 
                                i + 1, 
                                truncate_text(&result.content, 80));
                    }
                    
                    // Show episodic memory results  
                    for (i, result) in results.episodic_results.iter().take(2).enumerate() {
                        println!("   {}. {} (Episodic Memory)", 
                                i + 3, 
                                truncate_text(&result.content, 80));
                    }
                    
                    // Show semantic memory results
                    for (i, result) in results.semantic_results.iter().take(2).enumerate() {
                        println!("   {}. {} (Semantic Memory)", 
                                i + 5, 
                                truncate_text(&result.description, 80));
                    }
                }
            }
            Err(e) => {
                println!("   Error querying memory: {}", e);
            }
        }
    }

    Ok(())
}

async fn demonstrate_concept_analysis(memory_system: &MemorySystem) -> Result<()> {
    println!("\n🧩 Concept Relationship Analysis");
    println!("{}", "-".repeat(40));

    // Get memory statistics to understand what we've learned
    let stats = memory_system.get_stats();
    
    println!("📊 Memory Overview:");
    if let Some(working_stats) = stats.get("working") {
        println!("   Working Memory: {} items", working_stats.total_items);
    }
    if let Some(episodic_stats) = stats.get("episodic") {
        println!("   Episodic Memory: {} items", episodic_stats.total_items);
    }
    if let Some(semantic_stats) = stats.get("semantic") {
        println!("   Semantic Memory: {} items", semantic_stats.total_items);
    }

    // Calculate total memory size
    let total_size: usize = stats.values().map(|s| s.size_bytes).sum();
    println!("   Total Memory Size: {} bytes", total_size);

    // Try to find patterns in learned content
    if let Some(semantic_stats) = stats.get("semantic") {
        if semantic_stats.total_items > 0 {
            println!("\n🔗 Semantic Concepts Discovered:");
            println!("   Brain AI has discovered {} semantic concepts", semantic_stats.total_items);
            println!("   These represent patterns and relationships extracted from repositories");
        }
    }

    Ok(())
}

async fn demonstrate_memory_statistics(memory_system: &MemorySystem) -> Result<()> {
    println!("\n📈 Performance Metrics and Statistics");
    println!("{}", "-".repeat(40));

    let stats = memory_system.get_stats();
    
    println!("🔢 Detailed Memory Statistics:");
    
    // Working Memory stats
    if let Some(working_stats) = stats.get("working") {
        println!("   Working Memory:");
        println!("     - Total items: {}", working_stats.total_items);
        println!("     - Size: {} bytes", working_stats.size_bytes);
        println!("     - Access count: {}", working_stats.access_count);
        println!("     - Consolidations: {}", working_stats.consolidation_count);
    }

    // Episodic Memory stats
    if let Some(episodic_stats) = stats.get("episodic") {
        println!("   Episodic Memory:");
        println!("     - Total events: {}", episodic_stats.total_items);
        println!("     - Size: {} bytes", episodic_stats.size_bytes);
        println!("     - Access count: {}", episodic_stats.access_count);
        println!("     - Storage efficiency: {:.1}%", 
                 if episodic_stats.size_bytes > 0 { 
                     (episodic_stats.total_items as f64 / episodic_stats.size_bytes as f64) * 100000.0 
                 } else { 
                     0.0 
                 });
    }

    // Semantic Memory stats
    if let Some(semantic_stats) = stats.get("semantic") {
        println!("   Semantic Memory:");
        println!("     - Total concepts: {}", semantic_stats.total_items);
        println!("     - Size: {} bytes", semantic_stats.size_bytes);
        println!("     - Access count: {}", semantic_stats.access_count);
        println!("     - Concept density: {:.2}", 
                 if semantic_stats.size_bytes > 0 { 
                     semantic_stats.total_items as f64 / semantic_stats.size_bytes as f64 * 1000.0 
                 } else { 
                     0.0 
                 });
    }

    // Overall efficiency
    let total_items: usize = stats.values().map(|s| s.total_items).sum();
    let total_size: usize = stats.values().map(|s| s.size_bytes).sum();
    
    println!("\n📊 Overall Learning Efficiency:");
    println!("   - Total information units: {}", total_items);
    println!("   - Total memory usage: {} KB", total_size / 1024);
    println!("   - Learning compression ratio: {:.2}:1", 
             if total_size > 0 { 
                 total_items as f64 / (total_size as f64 / 1024.0) 
             } else { 
                 0.0 
             });

    Ok(())
}

/// Helper function to truncate text for display
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}

/// Helper function to format file size
#[allow(dead_code)]
fn format_file_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
} 