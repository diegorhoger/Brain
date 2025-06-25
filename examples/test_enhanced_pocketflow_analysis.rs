#!/usr/bin/env cargo run --example test_enhanced_pocketflow_analysis
//! Enhanced PocketFlow Analysis Test
//!
//! This test loads architectural insights and then queries the Brain AI
//! to verify it can now answer detailed questions about PocketFlow's architecture.

use brain::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🧠 Enhanced Brain AI - PocketFlow Architecture Analysis Test");
    println!("{}", "=".repeat(60));

    // Initialize Brain AI components
    let mut memory_system = MemorySystem::new(2000);
    let config = ConceptGraphConfig::default();
    let mut concept_graph = ConceptGraphManager::new(config).await?;
    let mut pattern_detector = PatternDetector::new();
    
    println!("\n🧠 Loading Enhanced Architectural Knowledge");
    println!("{}", "-".repeat(40));

    // Load comprehensive architectural insights about PocketFlow
    let detailed_insights = vec![
        ("PocketFlow Core Architecture", 
         "PocketFlow implements a Node-Flow architecture with three key patterns: 1) Node-based Components (BaseNode, BatchNode, ParallelBatchNode), 2) Flow-based Orchestration (Flow, AsyncFlow), and 3) LLM Integration Layer. The framework follows a minimalist 100-line philosophy while providing powerful agent orchestration capabilities."),
        
        ("PocketFlow Pattern 1: Node-Flow Architecture", 
         "The Node-Flow pattern separates processing logic (Nodes) from execution orchestration (Flows). BaseNode is the fundamental abstraction, with specialized nodes like BatchNode for optimization and ParallelBatchNode for concurrency. This enables modular, composable AI workflows where nodes can be chained and reused."),
        
        ("PocketFlow Pattern 2: Async Parallel Processing", 
         "PocketFlow implements asynchronous execution through AsyncFlow and parallel processing via ParallelBatchNode. This pattern enables efficient concurrent LLM operations, reducing latency and improving throughput. The async/await paradigm allows non-blocking operations while maintaining clean code structure."),
        
        ("PocketFlow Pattern 3: Batch Optimization Framework", 
         "The Batch Optimization pattern uses BatchNode and related classes to group multiple LLM requests together, reducing API costs and improving efficiency. This pattern is crucial for production LLM applications where cost optimization and rate limiting are important considerations."),
        
        ("PocketFlow Agent-Based Design", 
         "PocketFlow enables 'Agents build Agents' through its agent-based framework design. The system provides abstractions for autonomous agents that can create and orchestrate other agents, enabling recursive and self-improving AI systems. This supports agentic workflows and multi-agent coordination."),
        
        ("PocketFlow Implementation Details", 
         "Key classes include: BaseNode (base processing unit), Flow (synchronous orchestrator), AsyncFlow (asynchronous orchestrator), BatchNode (batch processor), ParallelBatchNode (parallel batch processor). The framework supports fallback mechanisms, error handling, and flexible configuration. All core functionality is implemented in approximately 100 lines of Python code."),
        
        ("PocketFlow Use Cases and Applications", 
         "PocketFlow is designed for: LLM workflow orchestration, agent-based AI systems, batch processing of AI tasks, parallel LLM operations, cost-optimized AI pipelines, and rapid prototyping of AI agents. The minimalist design makes it ideal for both research and production environments."),
    ];

    for (title, description) in detailed_insights {
        match memory_system.learn(format!("{}: {}", title, description), Priority::High) {
            Ok(_) => println!("✅ Loaded: {}", title),
            Err(e) => println!("❌ Failed to load {}: {}", title, e),
        }
    }

    // Also load the GitHub repository information
    let github_token = env::var("GITHUB_TOKEN").ok();
    if github_token.is_some() {
        let config = GitHubLearningConfig {
            max_files: 20,
            max_file_size: 50_000,
            include_code: true,
            include_docs: true,
            include_config: false,
            ..Default::default()
        };

        let github_engine = GitHubLearningEngine::new(github_token.clone(), Some(config));
        
        println!("\n📚 Loading Repository Context");
        match github_engine.learn_from_repository(&mut memory_system, "https://github.com/The-Pocket/PocketFlow").await {
            Ok(result) => {
                println!("✅ Repository context loaded: {} files processed", result.files_processed);
            }
            Err(e) => {
                println!("⚠️ Repository loading failed: {}", e);
            }
        }
    }

    println!("\n🔍 Testing Architectural Knowledge");
    println!("{}", "-".repeat(40));

    // Create RAG orchestrator for querying
    let mut rag_orchestrator = RagOrchestrator::new()?;

    // Test queries about PocketFlow architecture
    let test_queries = vec![
        "What are the 3 unique architecture patterns in PocketFlow?",
        "How does PocketFlow implement the Node-Flow architecture pattern?",
        "What is the purpose of BatchNode and ParallelBatchNode in PocketFlow?",
        "How does PocketFlow enable agent-based workflows?",
        "What makes PocketFlow a 100-line framework?",
        "How does PocketFlow optimize LLM API costs?",
        "What are the key classes and components in PocketFlow?",
    ];

    for (i, query) in test_queries.iter().enumerate() {
        println!("\n📝 Query {}: {}", i + 1, query);
        
        let request = RagRequest {
            message: query.to_string(),
            conversation_id: Some("enhanced_pocketflow_test".to_string()),
            context_limit: Some(8),
            retrieval_threshold: Some(0.2), // Lower threshold to find more matches
        };

        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
            &mut pattern_detector,
        ).await {
            Ok(response) => {
                println!("💡 Response (Confidence: {:.3}):", response.confidence_score);
                println!("   {}", response.response);
                
                if response.context_used.len() > 0 {
                    println!("   📚 Used {} knowledge sources:", response.context_used.len());
                    for (j, source) in response.context_used.iter().take(2).enumerate() {
                        println!("     {}. {} (relevance: {:.3})", 
                                j + 1, 
                                source.source, 
                                source.relevance_score);
                    }
                } else {
                    println!("   ⚠️ No knowledge sources found");
                }
            }
            Err(e) => {
                println!("❌ Query failed: {}", e);
            }
        }
        
        // Small delay between queries
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }

    println!("\n📊 Memory System Analysis");
    println!("{}", "-".repeat(40));
    
    // Check memory statistics
    let stats = memory_system.get_stats();
    for (memory_type, memory_stats) in stats.iter() {
        println!("{} memory: {} items, {} bytes", 
                memory_type, 
                memory_stats.total_items, 
                memory_stats.size_bytes);
    }

    // Test specific memory searches
    println!("\n🔍 Testing Memory Retrieval");
    let test_searches = vec!["Node-Flow", "BatchNode", "AsyncFlow", "agent", "100-line"];
    
    for search_term in test_searches {
        match memory_system.find_related_memories(search_term, 3) {
            Ok(results) => {
                let total = results.working_results.len() + results.episodic_results.len() + results.semantic_results.len();
                if total > 0 {
                    println!("🎯 '{}': {} related memories found", search_term, total);
                }
            }
            Err(e) => {
                println!("❌ Search for '{}' failed: {}", search_term, e);
            }
        }
    }

    println!("\n✅ Enhanced PocketFlow Analysis Test Complete!");
    println!("The Brain AI should now have comprehensive knowledge about PocketFlow's");
    println!("architecture patterns and be able to provide detailed insights.");

    Ok(())
} 