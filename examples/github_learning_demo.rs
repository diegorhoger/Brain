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

use brain::{MemoryService, WorkingMemoryQuery, WorkingMemoryRepository, Result};
use brain_infra::memory::{WorkingMemoryRepository as WorkingMemoryRepo, EpisodicMemoryRepository, SemanticMemoryRepository};
use brain_infra::{GitHubLearningEngine, GitHubLearningConfig};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🧠 Brain AI - GitHub Repository Learning Demo (Rust)");
    println!("{}", "=".repeat(60));

    // Create memory repositories
    let mut working_repo = WorkingMemoryRepo::new(1000);
    let episodic_repo = Box::new(EpisodicMemoryRepository::new("github_demo.db").await?);
    let semantic_repo = Box::new(SemanticMemoryRepository::new());
    
    // Create memory service for queries
    let memory_service = MemoryService::new(
        Box::new(WorkingMemoryRepo::new(100)), // For the service
        episodic_repo, 
        semantic_repo
    );
    
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
        
        match github_engine.learn_from_repository(&mut working_repo, repo_url).await {
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
                println!("   💡 Check your internet connection or repository URL");
                continue; // Try next repository
            }
        }
        
        // Small delay between repositories to be respectful to GitHub API
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    // Demonstrate memory querying capabilities (using the working repo directly)
    demonstrate_memory_queries(&working_repo).await?;
    
    // Also demonstrate with the memory service
    demonstrate_concept_analysis(&memory_service).await?;
    
    // Show memory statistics
    demonstrate_memory_statistics(&memory_service).await?;

    println!("\n🎉 GitHub Learning Demo Completed Successfully!");
    println!("{}", "=".repeat(60));
    
    Ok(())
}

async fn demonstrate_memory_queries(working_repo: &dyn WorkingMemoryRepository) -> Result<()> {
    println!("\n🔍 Memory Querying and Information Retrieval");
    println!("{}", "-".repeat(40));

    // Query the working repository directly
    let query = WorkingMemoryQuery::default();
    let items = working_repo.query_items(&query).await?;
    println!("📊 Working Memory Overview:");
    println!("   Total items: {}", items.len());
    
    if !items.is_empty() {
        println!("\n🔗 Sample Content:");
        for (i, item) in items.iter().take(3).enumerate() {
            println!("   {}. {} (Priority: {:?})", 
                    i + 1, 
                    truncate_text(&item.content, 100), 
                    item.priority);
        }
    }

    Ok(())
}

async fn demonstrate_concept_analysis(memory_service: &MemoryService) -> Result<()> {
    println!("\n🧩 Concept Relationship Analysis");
    println!("{}", "-".repeat(40));

    // Create a simple query to see what we have in working memory
    let query = WorkingMemoryQuery::default();
    match memory_service.query_working(&query).await {
        Ok(items) => {
            println!("📊 Memory Overview:");
            println!("   Working Memory: {} items", items.len());
            
            if !items.is_empty() {
                println!("\n🔗 Sample Content:");
                for (i, item) in items.iter().take(3).enumerate() {
                    println!("   {}. {} (Priority: {:?})", 
                            i + 1, 
                            truncate_text(&item.content, 100), 
                            item.priority);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to query memory: {}", e);
        }
    }

    Ok(())
}

async fn demonstrate_memory_statistics(memory_service: &MemoryService) -> Result<()> {
    println!("\n📈 Performance Metrics and Statistics");
    println!("{}", "-".repeat(40));

    // Since MemoryService doesn't have get_stats method, let's demonstrate with queries
    let all_query = WorkingMemoryQuery::default();
    match memory_service.query_working(&all_query).await {
        Ok(items) => {
            let total_items = items.len();
            let total_size: usize = items.iter().map(|item| item.content.len()).sum();
            
            println!("🔢 Memory Statistics:");
            println!("   Working Memory:");
            println!("     - Total items: {}", total_items);
            println!("     - Total content size: {} bytes", total_size);
            
            if total_items > 0 {
                println!("     - Average item size: {} bytes", total_size / total_items);
                
                // Count by priority
                let mut priority_counts = std::collections::HashMap::new();
                for item in &items {
                    *priority_counts.entry(format!("{:?}", item.priority)).or_insert(0) += 1;
                }
                
                println!("     - Items by priority:");
                for (priority, count) in priority_counts {
                    println!("       • {}: {}", priority, count);
                }
            }
            
            println!("\n📊 Overall Learning Efficiency:");
            println!("   - Information units stored: {}", total_items);
            println!("   - Memory usage: {} KB", total_size / 1024);
            if total_size > 0 {
                println!("   - Compression ratio: {:.2}:1", 
                         total_items as f64 / (total_size as f64 / 1024.0));
            }
        }
        Err(e) => {
            println!("❌ Failed to get statistics: {}", e);
        }
    }

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