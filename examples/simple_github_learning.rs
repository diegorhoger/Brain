//! Simple GitHub Learning Demo
//!
//! This is a streamlined demonstration of Brain AI's GitHub learning capabilities.
//! 
//! A more focused version that shows the core functionality:
//! - Learning from a single repository
//! - Storing knowledge in memory
//! - Simple querying of learned content

use brain::{MemoryService, WorkingMemoryQuery, Result, WorkingMemoryRepository};
use brain_infra::memory::{
    WorkingMemoryRepository as WorkingMemoryRepo, 
    EpisodicMemoryRepository as EpisodicMemoryRepo, 
    SemanticMemoryRepository as SemanticMemoryRepo
};
use brain_infra::{GitHubLearningEngine, GitHubLearningConfig};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Simple GitHub Learning Demo");
    println!("==============================");
    
    // Ensure data directory exists
    std::fs::create_dir_all("data").map_err(|e| brain::BrainError::Io { source: e })?;
    
    // Create memory repositories using concrete types
    let mut working_repo = WorkingMemoryRepo::new(100);
    let episodic_repo = Box::new(EpisodicMemoryRepo::new("data/simple_github_demo.db").await?);
    let semantic_repo = Box::new(SemanticMemoryRepo::new());

    // Create memory service for potential future use
    let _memory_service = MemoryService::new(
        Box::new(WorkingMemoryRepo::new(100)),
        episodic_repo,
        semantic_repo,
    );

    // Get GitHub token if available
    let github_token = env::var("GITHUB_TOKEN").ok();

    // Create GitHub learning configuration
    let config = GitHubLearningConfig {
        max_files: 20,          // Fewer files for simplicity
        max_file_size: 30_000,  // 30KB per file
        include_code: true,
        include_docs: true,
        include_config: false,  // Skip config files for simplicity
        ..Default::default()
    };

    let github_engine = GitHubLearningEngine::new(github_token, Some(config));

    // Learn from a single, simple repository
    let repo_url = "BurntSushi/ripgrep";  // A well-documented Rust tool
    
    println!("📚 Learning from repository: {}", repo_url);
    println!("⏳ This may take a moment...\n");

    match github_engine.learn_from_repository(&mut working_repo, repo_url).await {
        Ok(result) => {
            println!("✅ Learning completed successfully!");
            println!("   Files processed: {}", result.files_processed);
            println!("   Memory entries: {}", result.memory_entries_created);
            println!("   Concepts discovered: {}", result.concepts_discovered);
            println!("   Learning time: {}ms\n", result.learning_time_ms);
            
            // Query what we learned
            println!("🔍 Querying learned knowledge:");
            println!("{}", "-".repeat(35));
            
            let query = WorkingMemoryQuery::default();
            match working_repo.query_items(&query).await {
                Ok(items) => {
                    println!("📊 Total items in memory: {}\n", items.len());
                    
                    if !items.is_empty() {
                        println!("🔗 Sample content:");
                        for (i, item) in items.iter().take(3).enumerate() {
                            let content = if item.content.len() > 120 {
                                format!("{}...", &item.content[..120])
                            } else {
                                item.content.clone()
                            };
                            println!("   {}. {} (Priority: {:?})", i + 1, content, item.priority);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Error querying memory: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Learning failed: {}", e);
            println!("💡 Make sure you have internet connectivity");
            if env::var("GITHUB_TOKEN").is_err() {
                println!("💡 Set GITHUB_TOKEN environment variable for better rate limits");
            }
            return Ok(()); // Don't fail the demo completely
        }
    }

    println!("\n🎉 Simple GitHub Learning Demo Completed!");
    println!("   Repository knowledge has been stored in memory");
    println!("   Try the more comprehensive 'github_learning_demo' for advanced features");
    
    Ok(())
} 