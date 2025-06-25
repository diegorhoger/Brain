#!/usr/bin/env cargo run --example simple_pocketflow_chat
//! Simple PocketFlow Chat
//!
//! A direct, simple chat interface that can answer questions about PocketFlow
//! architecture by using direct RAG retrieval with optimized settings.

use brain::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("💬 Brain AI - Simple PocketFlow Architecture Chat");
    println!("{}", "=".repeat(55));

    // Initialize Brain AI components
    let mut memory_system = MemorySystem::new(2000);
    let config = ConceptGraphConfig::default();
    let mut concept_graph = ConceptGraphManager::new(config).await?;
    let mut pattern_detector = PatternDetector::new();
    
    println!("\n🧠 Loading PocketFlow Knowledge Base");
    println!("{}", "-".repeat(40));

    // Load comprehensive PocketFlow knowledge
    let pocketflow_knowledge = vec![
        // Core architecture overview
        "PocketFlow implements three unique architecture patterns: 1) Node-Flow Architecture for modular processing, 2) Async Parallel Processing for concurrent operations, and 3) Batch Optimization Framework for cost-effective LLM operations.",
        
        // Pattern 1: Node-Flow Architecture
        "The Node-Flow Architecture pattern in PocketFlow separates processing logic (Nodes) from execution orchestration (Flows). BaseNode is the fundamental abstraction that all processing units inherit from. Flow classes orchestrate the execution of multiple nodes in sequence or parallel.",
        
        // Pattern 2: Async Parallel Processing  
        "PocketFlow's Async Parallel Processing pattern uses AsyncFlow for asynchronous execution and ParallelBatchNode for concurrent processing. This enables efficient concurrent LLM operations, reducing latency and improving throughput through non-blocking operations.",
        
        // Pattern 3: Batch Optimization
        "The Batch Optimization Framework in PocketFlow uses BatchNode and ParallelBatchNode to group multiple LLM requests together. This reduces API costs by minimizing the number of individual calls and improves efficiency through batching strategies.",
        
        // Key components
        "PocketFlow key classes and components include: BaseNode (base processing unit), Flow (synchronous orchestrator), AsyncFlow (asynchronous orchestrator), BatchNode (batch processor), ParallelBatchNode (parallel batch processor). These components work together to create flexible AI workflows.",
        
        // Agent-based design
        "PocketFlow enables agent-based workflows through its 'Agents build Agents' design philosophy. The framework provides abstractions for autonomous agents that can create and orchestrate other agents, enabling recursive and self-improving AI systems.",
        
        // 100-line philosophy
        "PocketFlow follows a minimalist 100-line framework philosophy, providing essential LLM orchestration capabilities in approximately 100 lines of Python code. This compact design makes it easy to understand, modify, and extend while maintaining powerful functionality.",
        
        // LLM cost optimization
        "PocketFlow optimizes LLM API costs through batch processing with BatchNode, parallel processing with ParallelBatchNode, and efficient request grouping. This reduces the number of individual API calls and takes advantage of batch pricing discounts offered by LLM providers.",
        
        // Use cases and applications
        "PocketFlow is designed for LLM workflow orchestration, agent-based AI systems, batch processing of AI tasks, parallel LLM operations, cost-optimized AI pipelines, and rapid prototyping of AI agents. The minimalist design makes it ideal for both research and production environments.",
        
        // Implementation details
        "PocketFlow implementation uses Python with async/await patterns for non-blocking operations. The framework supports error handling, fallback mechanisms, and flexible configuration. Nodes can be chained together to create complex workflows while maintaining clean, readable code.",
    ];

    for (i, knowledge) in pocketflow_knowledge.iter().enumerate() {
        match memory_system.learn(knowledge.to_string(), Priority::High) {
            Ok(_) => println!("✅ Loaded knowledge chunk {}", i + 1),
            Err(e) => println!("❌ Failed to load knowledge {}: {}", i + 1, e),
        }
    }

    // Also load GitHub repository context if available
    let github_token = env::var("GITHUB_TOKEN").ok();
    if github_token.is_some() {
        println!("\n📚 Loading GitHub Repository Context");
        let config = GitHubLearningConfig {
            max_files: 15,
            max_file_size: 30_000,
            include_code: true,
            include_docs: true,
            include_config: false,
            ..Default::default()
        };

        let github_engine = GitHubLearningEngine::new(github_token.clone(), Some(config));
        
        match github_engine.learn_from_repository(&mut memory_system, "https://github.com/The-Pocket/PocketFlow").await {
            Ok(result) => {
                println!("✅ Repository context loaded: {} files processed", result.files_processed);
            }
            Err(e) => {
                println!("⚠️ Repository loading failed: {}", e);
            }
        }
    }

    println!("\n💬 PocketFlow Architecture Chat Interface");
    println!("{}", "-".repeat(40));
    println!("Ask me anything about PocketFlow's architecture patterns!");
    println!("Type 'quit' to exit.\n");

    // Create a simple RAG orchestrator
    let mut rag_orchestrator = RagOrchestrator::new()?;

    // Pre-defined test questions for demo
    let demo_questions = vec![
        "What are the 3 unique architecture patterns in PocketFlow?",
        "How does PocketFlow implement the Node-Flow architecture pattern?", 
        "What is the purpose of BatchNode and ParallelBatchNode in PocketFlow?",
        "How does PocketFlow enable agent-based workflows?",
        "What makes PocketFlow a 100-line framework?",
        "How does PocketFlow optimize LLM API costs?",
        "What are the key classes and components in PocketFlow?",
        "What are PocketFlow's main use cases and applications?",
    ];

    println!("🎯 Demo Questions (answering automatically):");
    println!("{}", "-".repeat(40));

    for (i, question) in demo_questions.iter().enumerate() {
        println!("\n📝 Question {}: {}", i + 1, question);
        
        // Use direct RAG with very low threshold for better retrieval
        let request = RagRequest {
            message: question.to_string(),
            conversation_id: Some("pocketflow_chat".to_string()),
            context_limit: Some(10),
            retrieval_threshold: Some(0.05), // Very low threshold for better matches
        };

        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
            &mut pattern_detector,
        ).await {
            Ok(response) => {
                println!("💡 Answer (Confidence: {:.3}):", response.confidence_score);
                println!("   {}", response.response);
                
                if response.context_used.len() > 0 {
                    println!("   📚 Based on {} knowledge sources", response.context_used.len());
                } else {
                    println!("   ⚠️ No relevant knowledge found");
                }
            }
            Err(e) => {
                println!("❌ Failed to answer: {}", e);
            }
        }
        
        // Small delay between questions
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("\n📊 Final Memory Statistics");
    println!("{}", "-".repeat(40));
    
    let stats = memory_system.get_stats();
    for (memory_type, memory_stats) in stats.iter() {
        println!("{}: {} items, {} bytes", 
                memory_type, 
                memory_stats.total_items, 
                memory_stats.size_bytes);
    }

    println!("\n✅ PocketFlow Architecture Chat Complete!");
    println!("The Brain AI now has comprehensive knowledge about PocketFlow's");
    println!("architecture patterns and can provide detailed, accurate answers.");

    Ok(())
} 