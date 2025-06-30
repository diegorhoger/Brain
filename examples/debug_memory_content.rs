#!/usr/bin/env cargo run --example debug_memory_content
//! Debug Memory Content
//!
//! This debug example shows what's actually stored in Brain AI's memory
//! after learning from PocketFlow to understand why insights aren't being generated.

use brain::{MemoryService, WorkingMemoryQuery, SemanticQuery, Result};
use brain_infra::{
    memory::{WorkingMemoryRepository, EpisodicMemoryRepository, SemanticMemoryRepository},
    GitHubLearningEngine, GitHubLearningConfig
};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🔍 Brain AI Memory Debug - PocketFlow Content Analysis");
    println!("{}", "=".repeat(60));

    // Create memory repositories
    let mut working_repo = WorkingMemoryRepository::new(100);
    let episodic_repo = Box::new(EpisodicMemoryRepository::new("debug_memory.db").await?);
    let semantic_repo = Box::new(SemanticMemoryRepository::new());
    
    // Create memory service with a separate working repo for queries
    let working_repo_for_service = Box::new(WorkingMemoryRepository::new(100));
    let memory_service = MemoryService::new(working_repo_for_service, episodic_repo, semantic_repo);
    
    // Get GitHub token
    let github_token = env::var("GITHUB_TOKEN").ok();
    
    // Create GitHub learning configuration
    let config = GitHubLearningConfig {
        max_files: 50,
        max_file_size: 50_000,
        include_code: true,
        include_docs: true,
        include_config: true,
        ..Default::default()
    };

    let github_engine = GitHubLearningEngine::new(github_token.clone(), Some(config));

    println!("\n🚀 Learning from PocketFlow Repository");
    println!("{}", "-".repeat(40));

    // Learn from PocketFlow repository (pass working_repo directly)
    let pocketflow_url = "https://github.com/The-Pocket/PocketFlow";
    match github_engine.learn_from_repository(&mut working_repo, pocketflow_url).await {
        Ok(result) => {
            println!("✅ Learning completed!");
            println!("   Files processed: {}", result.files_processed);
            println!("   Concepts discovered: {}", result.concepts_discovered);
            println!("   Memory entries: {}", result.memory_entries_created);
        }
        Err(e) => {
            println!("❌ Learning failed: {}", e);
            return Err(e);
        }
    }

    println!("\n🔍 Analyzing Working Memory Content");
    println!("{}", "-".repeat(40));

    // Query working memory to see what was stored
    let query = WorkingMemoryQuery {
        content_pattern: None,
        priority: None,
        min_importance: None,
        created_after: None,
        limit: Some(20), // Show first 20 items
    };

    match memory_service.query_working(&query).await {
        Ok(items) => {
            println!("Found {} working memory items:", items.len());
            for (i, item) in items.iter().enumerate() {
                println!("\n📝 Item {}: (Priority: {:?}, Importance: {:.3})", 
                        i + 1, item.priority, item.importance_score());
                
                // Show first 200 characters of content
                let content_preview = if item.content.len() > 200 {
                    format!("{}...", &item.content[..200])
                } else {
                    item.content.clone()
                };
                println!("   Content: {}", content_preview);
            }
        }
        Err(e) => {
            println!("❌ Failed to query working memory: {}", e);
        }
    }

    println!("\n🔍 Searching for Specific Patterns");
    println!("{}", "-".repeat(40));

    // Search for specific architectural terms
    let search_terms = vec![
        "agent", "workflow", "flow", "orchestration", "llm", "framework",
        "pattern", "architecture", "design", "component", "class", "function"
    ];

    for term in search_terms {
        let query = WorkingMemoryQuery {
            content_pattern: Some(term.to_string()),
            priority: None,
            min_importance: None,
            created_after: None,
            limit: Some(5),
        };

        match memory_service.query_working(&query).await {
            Ok(items) => {
                if !items.is_empty() {
                    println!("\n🎯 Found {} items containing '{}':", items.len(), term);
                    for (i, item) in items.iter().enumerate() {
                        let content_preview = if item.content.len() > 150 {
                            format!("{}...", &item.content[..150])
                        } else {
                            item.content.clone()
                        };
                        println!("   {}. {}", i + 1, content_preview);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to search for '{}': {}", term, e);
            }
        }
    }

    println!("\n🔍 Analyzing Semantic Memory");
    println!("{}", "-".repeat(40));

    let semantic_query = SemanticQuery {
        name_pattern: None,
        embedding: None,
        min_confidence: Some(0.1),
        min_similarity: None,
        limit: Some(10),
    };

    match memory_service.query_semantic(&semantic_query).await {
        Ok(concepts) => {
            if !concepts.is_empty() {
                println!("Found {} semantic concepts:", concepts.len());
                for (i, concept) in concepts.iter().enumerate() {
                    println!("   {}. {} (confidence: {:.3})", 
                            i + 1, concept.name, concept.confidence);
                    println!("      Description: {}", concept.description);
                }
            } else {
                println!("No semantic concepts found");
            }
        }
        Err(e) => {
            println!("❌ Failed to query semantic memory: {}", e);
        }
    }

    println!("\n🔍 Cross-Memory Search for Architecture Terms");
    println!("{}", "-".repeat(40));

    let architecture_terms = vec![
        "PocketFlow", "agent", "workflow", "orchestration", "framework"
    ];

    for term in architecture_terms {
        match memory_service.query_all_memories(term).await {
            Ok(results) => {
                let total_results = results.working_results.len() + 
                                  results.episodic_results.len() + 
                                  results.semantic_results.len();
                
                if total_results > 0 {
                    println!("\n🎯 Cross-memory search for '{}' found {} results:", term, total_results);
                    
                    for (i, item) in results.working_results.iter().enumerate() {
                        let preview = if item.content.len() > 100 {
                            format!("{}...", &item.content[..100])
                        } else {
                            item.content.clone()
                        };
                        println!("   Working {}: {}", i + 1, preview);
                    }
                    
                    for (i, event) in results.episodic_results.iter().enumerate() {
                        let preview = if event.content.len() > 100 {
                            format!("{}...", &event.content[..100])
                        } else {
                            event.content.clone()
                        };
                        println!("   Episodic {}: {}", i + 1, preview);
                    }
                    
                    for (i, concept) in results.semantic_results.iter().enumerate() {
                        println!("   Semantic {}: {} - {}", i + 1, concept.name, concept.description);
                    }
                }
            }
            Err(e) => {
                println!("❌ Cross-memory search for '{}' failed: {}", term, e);
            }
        }
    }

    println!("\n📊 Memory Summary");
    println!("{}", "-".repeat(40));
    
    // Get a summary of what's in memory
    let all_query = WorkingMemoryQuery::default();
    match memory_service.query_working(&all_query).await {
        Ok(items) => {
            let total_items = items.len();
            let total_size: usize = items.iter().map(|item| item.content.len()).sum();
            println!("Working Memory Summary:");
            println!("  • Total items: {}", total_items);
            println!("  • Total content size: {} bytes", total_size);
            if total_items > 0 {
                let avg_size = total_size / total_items;
                println!("  • Average item size: {} bytes", avg_size);
            }
        }
        Err(e) => {
            println!("Failed to get memory summary: {}", e);
        }
    }

    println!("\n✅ Memory Content Debug Complete!");
    println!("This should help identify what's being stored and why insights might not be generated.");

    Ok(())
} 