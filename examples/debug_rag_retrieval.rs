#!/usr/bin/env cargo run --example debug_rag_retrieval
//! Debug RAG Retrieval
//!
//! This debug example tests RAG retrieval directly to understand why
//! the stored architectural knowledge isn't being found by the Brain AI.

use brain::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🔍 Brain AI RAG Retrieval Debug");
    println!("{}", "=".repeat(50));

    // Initialize Brain AI components
    let mut memory_system = MemorySystem::new(2000);
    
    println!("\n🧠 Loading Test Knowledge");
    println!("{}", "-".repeat(30));

    // Load some test architectural knowledge
    let test_knowledge = vec![
        "PocketFlow implements three unique architecture patterns: Node-Flow Architecture, Async Parallel Processing, and Batch Optimization Framework.",
        "The Node-Flow pattern in PocketFlow separates processing logic (Nodes) from execution orchestration (Flows). BaseNode is the fundamental abstraction.",
        "PocketFlow uses BatchNode and ParallelBatchNode for optimizing LLM API costs by grouping multiple requests together.",
        "PocketFlow enables agent-based workflows through its 'Agents build Agents' design philosophy.",
        "PocketFlow is a 100-line framework that provides essential LLM orchestration capabilities in a compact codebase.",
    ];

    for (i, knowledge) in test_knowledge.iter().enumerate() {
        match memory_system.learn(knowledge.to_string(), Priority::High) {
            Ok(_) => println!("✅ Stored knowledge {}", i + 1),
            Err(e) => println!("❌ Failed to store knowledge {}: {}", i + 1, e),
        }
    }

    println!("\n🔍 Testing Direct Memory Queries");
    println!("{}", "-".repeat(30));

    // Test direct memory queries
    let test_queries = vec![
        "architecture patterns",
        "Node-Flow",
        "BatchNode",
        "agent-based",
        "100-line framework",
        "PocketFlow",
    ];

    for query in test_queries {
        println!("\n🎯 Testing query: '{}'", query);
        
        // Test working memory query
        let working_query = WorkingMemoryQuery {
            content_pattern: Some(query.to_string()),
            priority: None,
            min_importance: None,
            created_after: None,
            limit: Some(5),
        };

        match memory_system.query_working(&working_query) {
            Ok(items) => {
                println!("   Working memory: {} items found", items.len());
                for (i, item) in items.iter().take(2).enumerate() {
                    let preview = if item.content.len() > 80 {
                        format!("{}...", &item.content[..80])
                    } else {
                        item.content.clone()
                    };
                    println!("     {}. {} (importance: {:.3})", i + 1, preview, item.importance_score());
                }
            }
            Err(e) => {
                println!("   Working memory query failed: {}", e);
            }
        }

        // Test cross-memory search
        match memory_system.find_related_memories(query, 3) {
            Ok(results) => {
                let total = results.working_results.len() + results.episodic_results.len() + results.semantic_results.len();
                println!("   Cross-memory search: {} total results", total);
                for (i, item) in results.working_results.iter().take(2).enumerate() {
                    let preview = if item.content.len() > 80 {
                        format!("{}...", &item.content[..80])
                    } else {
                        item.content.clone()
                    };
                    println!("     {}. {}", i + 1, preview);
                }
            }
            Err(e) => {
                println!("   Cross-memory search failed: {}", e);
            }
        }
    }

    println!("\n🤖 Testing Similarity Calculations");
    println!("{}", "-".repeat(30));

    // Test RAG retrieval with different thresholds
    let test_questions = vec![
        "What are the 3 unique architecture patterns in PocketFlow?",
        "How does the Node-Flow pattern work?",
        "What is BatchNode used for?",
    ];

    for question in test_questions {
        println!("\n📝 Question: '{}'", question);
        
        // Test simple text similarity
        for (i, knowledge) in test_knowledge.iter().enumerate() {
            let similarity = calculate_simple_similarity(question, knowledge);
            println!("     Knowledge {}: similarity {:.3}", i + 1, similarity);
            if similarity > 0.1 {
                println!("       ✅ Would be retrieved (above threshold)");
            } else {
                println!("       ❌ Below threshold");
            }
        }
    }

    println!("\n📊 Memory System State");
    println!("{}", "-".repeat(30));
    
    let stats = memory_system.get_stats();
    for (memory_type, memory_stats) in stats.iter() {
        println!("{}: {} items, {} bytes", 
                memory_type, 
                memory_stats.total_items, 
                memory_stats.size_bytes);
    }

    println!("\n✅ RAG Retrieval Debug Complete!");
    println!("This should help identify why the RAG system isn't finding the stored knowledge.");

    Ok(())
}

// Simple similarity calculation for debugging
fn calculate_simple_similarity(query: &str, content: &str) -> f64 {
    let query_lower = query.to_lowercase();
    let content_lower = content.to_lowercase();
    
    let query_words: std::collections::HashSet<&str> = query_lower
        .split_whitespace()
        .collect();
    
    let content_words: std::collections::HashSet<&str> = content_lower
        .split_whitespace()
        .collect();
    
    let intersection: std::collections::HashSet<_> = query_words
        .intersection(&content_words)
        .collect();
    
    let union: std::collections::HashSet<_> = query_words
        .union(&content_words)
        .collect();
    
    if union.is_empty() {
        0.0
    } else {
        intersection.len() as f64 / union.len() as f64
    }
} 