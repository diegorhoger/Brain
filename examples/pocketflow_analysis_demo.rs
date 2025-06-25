#!/usr/bin/env cargo run --example pocketflow_analysis_demo
//! PocketFlow Architecture Analysis Demo
//!
//! This demo specifically analyzes the PocketFlow repository to extract
//! architectural patterns and provide detailed insights about the codebase.
//!
//! Usage:
//!     cargo run --example pocketflow_analysis_demo

use brain::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🧠 Brain AI - PocketFlow Architecture Analysis Demo");
    println!("{}", "=".repeat(60));

    // Initialize Brain AI components
    let mut memory_system = MemorySystem::new(2000); // Larger capacity for detailed analysis
    
    // Create concept graph with default configuration
    let config = ConceptGraphConfig::default();
    let mut concept_graph = ConceptGraphManager::new(config).await?;
    let mut pattern_detector = PatternDetector::new();
    
    // Get GitHub token
    let github_token = env::var("GITHUB_TOKEN").ok();
    
    if github_token.is_some() {
        println!("✅ GitHub token found - using authenticated API");
    } else {
        println!("⚠️  No GitHub token - using public API (rate limited)");
        println!("   Set GITHUB_TOKEN environment variable for better performance");
    }

    // Create specialized GitHub learning configuration for architectural analysis
    let config = GitHubLearningConfig {
        max_files: 100,           // Analyze more files
        max_file_size: 100_000,   // Larger files for comprehensive analysis
        include_code: true,       // Include Python code
        include_docs: true,       // Include documentation
        include_config: true,     // Include configuration files
        ..Default::default()
    };

    let github_engine = GitHubLearningEngine::new(github_token.clone(), Some(config));

    println!("\n🚀 Learning from PocketFlow Repository");
    println!("{}", "-".repeat(40));

    // Learn from PocketFlow repository
    let pocketflow_url = "https://github.com/The-Pocket/PocketFlow";
    match github_engine.learn_from_repository(&mut memory_system, pocketflow_url).await {
        Ok(result) => {
            println!("✅ Learning completed!");
            println!("   Repository: {}", result.repository);
            println!("   Files processed: {}", result.files_processed);
            println!("   Content size: {} bytes", result.total_content_size);
            println!("   Concepts discovered: {}", result.concepts_discovered);
            println!("   Memory entries: {}", result.memory_entries_created);
            println!("   Learning time: {}ms", result.learning_time_ms);
            println!("   Summary: {}", result.summary);
            
            println!("\n🔍 Key Insights:");
            for (i, insight) in result.key_insights.iter().enumerate() {
                println!("   {}. {}", i + 1, insight);
            }
        }
        Err(e) => {
            println!("❌ Learning failed: {}", e);
            return Err(e);
        }
    }

    println!("\n🔍 Analyzing Architecture Patterns");
    println!("{}", "-".repeat(40));

    // Create RAG orchestrator for querying
    let mut rag_orchestrator = RagOrchestrator::new()?;

    // Queries to extract architectural insights
    let architecture_queries = vec![
        "What are the 3 unique architecture patterns in PocketFlow?",
        "How does PocketFlow implement agent-based workflows?",
        "What is the flow-based programming pattern in PocketFlow?",
        "How does PocketFlow handle LLM integration and orchestration?",
        "What are the key components of PocketFlow's architecture?",
        "How does PocketFlow implement the 100-line framework concept?",
        "What design patterns are used for agent communication?",
        "How does PocketFlow handle workflow orchestration?",
        "What are the core abstractions in PocketFlow's design?",
        "How does PocketFlow implement retrieval-augmented generation?",
    ];

    for (i, query) in architecture_queries.iter().enumerate() {
        println!("\n📝 Query {}: {}", i + 1, query);
        
        let request = RagRequest {
            message: query.to_string(),
            conversation_id: Some("pocketflow_analysis".to_string()),
            context_limit: Some(10),
            retrieval_threshold: Some(0.3),
        };

        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
            &mut pattern_detector,
        ).await {
            Ok(response) => {
                println!("💡 Response:");
                println!("   {}", response.response);
                println!("   Confidence: {:.3}", response.confidence_score);
                println!("   Sources used: {}", response.context_used.len());
                
                if !response.context_used.is_empty() {
                    println!("   📚 Knowledge sources:");
                    for (j, source) in response.context_used.iter().take(3).enumerate() {
                        println!("     {}. {} (relevance: {:.3})", 
                                j + 1, 
                                source.source, 
                                source.relevance_score);
                    }
                }
            }
            Err(e) => {
                println!("❌ Query failed: {}", e);
            }
        }
        
        // Small delay between queries
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("\n📊 Memory System Statistics");
    println!("{}", "-".repeat(40));
    
    // Get memory statistics
    let stats = memory_system.get_stats();
    
    // Display stats for each memory type
    for (memory_type, memory_stats) in stats.iter() {
        println!("{} memory:", memory_type);
        println!("   Total items: {}", memory_stats.total_items);
        println!("   Size: {} bytes", memory_stats.size_bytes);
        println!("   Access count: {}", memory_stats.access_count);
    }

    println!("\n🎯 Concept Graph Analysis");
    println!("{}", "-".repeat(40));
    
    // Analyze key concepts
    let key_concepts = vec![
        "architecture",
        "agent",
        "workflow",
        "llm",
        "framework",
        "pocketflow",
        "pattern",
        "orchestration",
    ];

    for concept in key_concepts {
        let query = ConceptQuery {
            content_pattern: Some(concept.to_string()),
            concept_type: None,
            min_confidence: Some(0.3),
            limit: Some(5),
            ..Default::default()
        };

        match concept_graph.query_concepts(&query).await {
            Ok(concepts) => {
                if !concepts.is_empty() {
                    println!("🔗 Concept '{}' found {} related concepts:", concept, concepts.len());
                    for (i, c) in concepts.iter().take(3).enumerate() {
                        println!("   {}. {} (confidence: {:.3})", 
                                i + 1, 
                                c.content, 
                                c.confidence_score);
                    }
                }
            }
            Err(e) => {
                println!("⚠️  Failed to query concept '{}': {}", concept, e);
            }
        }
    }

    println!("\n✅ PocketFlow Architecture Analysis Complete!");
    println!("The Brain AI has learned about PocketFlow's architecture and can now");
    println!("answer detailed questions about its design patterns and implementation.");

    Ok(())
} 