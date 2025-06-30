#!/usr/bin/env cargo run --example simple_pocketflow_chat
//! Simple PocketFlow Chat Demo
//! 
//! Demonstrates basic conversation capabilities using the Brain AI orchestrator
//! with proper MemoryService and ConceptGraphService architecture.

use brain::*;
use brain::services::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Simple PocketFlow Chat Demo");
    println!("===============================");
    
    // Check for OpenAI API key
    let _openai_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("⚠️  OPENAI_API_KEY not set. Please set it to use this demo.");
        println!("   export OPENAI_API_KEY=your_key_here");
        std::process::exit(1);
    });
    
    println!("✅ OpenAI API key found");
    
    // Initialize Brain AI components using new service architecture
    println!("\n🔧 Initializing Brain AI Services...");
    let mut memory_system = create_memory_service_with_capacity(2000).await?;
    let mut concept_graph = create_concept_graph_service_default().await?;
    
    println!("✅ MemoryService initialized with SQLite persistence");
    println!("✅ ConceptGraphService initialized with in-memory storage");
    
    println!("\n🧠 Loading PocketFlow Knowledge Base");
    
    // Create RAG orchestrator
    let mut rag_orchestrator = RagOrchestrator::new()?;
    
    // Add some PocketFlow-specific knowledge to memory
    let pocketflow_knowledge = vec![
        "PocketFlow is a streamlined development framework for building AI applications",
        "It emphasizes simplicity, modularity, and rapid prototyping",
        "PocketFlow supports multiple AI models and provides unified interfaces",
        "The framework includes built-in conversation management and context handling",
        "PocketFlow can integrate with various databases and external APIs",
        "PocketFlow uses Node-Flow Architecture for modular processing",
        "The framework implements async parallel processing for concurrent operations",
        "PocketFlow follows a minimalist 100-line framework philosophy",
        "BatchNode and ParallelBatchNode enable cost-effective LLM operations",
        "PocketFlow enables agent-based workflows through autonomous agents",
    ];
    
    for (i, knowledge) in pocketflow_knowledge.iter().enumerate() {
        memory_system.learn(knowledge.to_string(), Priority::Medium).await?;
        println!("✅ Loaded knowledge chunk {}", i + 1);
    }
    
    println!("✅ Knowledge base loaded with {} items", pocketflow_knowledge.len());
    
    // Demo questions
    let demo_questions = vec![
        "What is PocketFlow?",
        "What are the key features of PocketFlow?",
        "How does PocketFlow handle AI models?",
        "What is the Node-Flow Architecture in PocketFlow?",
        "How does PocketFlow optimize costs?",
    ];
    
    println!("\n💬 PocketFlow Chat Demo - Automated Q&A");
    println!("==========================================");
    
    for (i, question) in demo_questions.iter().enumerate() {
        println!("\n📝 Question {}: {}", i + 1, question);
        
        // Create request
        let request = RagRequest {
            message: question.to_string(),
            conversation_id: Some("demo_session".to_string()),
            context_limit: Some(5),
            retrieval_threshold: Some(0.3),
        };
        
        // Process with Brain AI
        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
        ).await {
            Ok(response) => {
                println!("🤖 Brain AI: {}", response.response);
                println!("   📊 Confidence: {:.1}%", response.confidence_score * 100.0);
                println!("   📚 Knowledge sources: {}", response.context_used.len());
                
                // Learn from this interaction
                let interaction = format!("User asked: '{}' | AI responded: '{}'", question, response.response);
                memory_system.learn(interaction, Priority::Low).await?;
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                println!("   This might be due to missing OpenAI API key or network issues.");
            }
        }
        
        // Brief pause between questions
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Show final statistics
    println!("\n📈 Session Statistics:");
    let stats = rag_orchestrator.get_conversation_stats();
    for (key, value) in stats {
        println!("   {}: {}", key, value);
    }
    
    println!("\n✅ Demo Complete! The new service architecture is working properly.");
    
    Ok(())
} 