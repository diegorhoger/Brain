#!/usr/bin/env cargo run --example pocketflow_analysis_demo
//! PocketFlow Analysis Demo
//! 
//! Demonstrates advanced analysis capabilities using Brain AI
//! with the new MemoryService and ConceptGraphService architecture.

use brain::*;
use brain::services::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 PocketFlow Analysis Demo");
    println!("===========================");
    
    // Check for OpenAI API key
    let _openai_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("⚠️  OPENAI_API_KEY not set. Please set it to use this demo.");
        std::process::exit(1);
    });
    
    println!("✅ OpenAI API key found");
    
    // Initialize Brain AI components using new service architecture
    println!("\n🔧 Initializing Brain AI Services...");
    let mut memory_system = create_memory_service_with_capacity(2000).await?;
    let mut concept_graph = create_concept_graph_service_default().await?;
    
    println!("✅ MemoryService initialized for detailed analysis");
    println!("✅ ConceptGraphService initialized");
    
    // Load comprehensive PocketFlow analysis data
    println!("\n📊 Loading PocketFlow Analysis Data...");
    let analysis_data = vec![
        "PocketFlow analysis reveals three core architectural patterns for LLM orchestration",
        "Performance metrics show 60% cost reduction through batch processing optimization",
        "Code analysis indicates 85% reduction in boilerplate through Node-Flow abstraction",
        "The 100-line philosophy maintains simplicity while supporting complex workflows",
        "Async parallel processing reduces latency by 40% in multi-LLM scenarios",
        "Agent-based design enables recursive and self-improving AI system architectures",
        "Batch optimization framework shows 3x improvement in API cost efficiency",
        "Node-Flow pattern separates concerns between logic and orchestration effectively",
        "Framework demonstrates high extensibility with minimal core complexity",
        "Real-world usage shows significant developer productivity improvements",
    ];
    
    for (i, data) in analysis_data.iter().enumerate() {
        memory_system.learn(data.to_string(), Priority::High).await?;
        println!("✅ Loaded analysis data {}", i + 1);
    }
    
    // Create RAG orchestrator for analysis processing
    println!("\n🤖 Initializing Analysis System...");
    let mut rag_orchestrator = RagOrchestrator::new()?;
    
    // Comprehensive analysis questions
    let analysis_questions = vec![
        "What are the key performance improvements shown by PocketFlow?",
        "How does the Node-Flow pattern improve code organization?",
        "What cost optimizations does PocketFlow provide?",
        "How does PocketFlow handle parallel processing?",
        "What makes PocketFlow suitable for production environments?",
        "How does the framework balance simplicity with functionality?",
        "What are the measurable benefits of using PocketFlow?",
        "How does PocketFlow support different types of AI workflows?",
    ];
    
    println!("\n📈 Running Comprehensive PocketFlow Analysis");
    println!("============================================");
    
    let mut analysis_results = Vec::new();
    
    for (i, question) in analysis_questions.iter().enumerate() {
        println!("\n🔍 Analysis {}: {}", i + 1, question);
        
        let request = RagRequest {
            message: question.to_string(),
            conversation_id: Some("analysis_session".to_string()),
            context_limit: Some(7),
            retrieval_threshold: Some(0.25),
        };
        
        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
        ).await {
            Ok(response) => {
                println!("📊 Analysis Result:");
                println!("   {}", response.response);
                println!("   🎯 Confidence: {:.1}%", response.confidence_score * 100.0);
                println!("   📚 Data sources: {}", response.context_used.len());
                
                // Store analysis result
                analysis_results.push((question.to_string(), response.response.clone(), response.confidence_score));
                
                // Learn from analysis for future insights
                let insight = format!("Analysis insight: {} -> {}", question, response.response);
                memory_system.learn(insight, Priority::Medium).await?;
            }
            Err(e) => {
                println!("   ❌ Analysis error: {}", e);
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;
    }
    
    // Generate summary report
    println!("\n📋 Analysis Summary Report");
    println!("==========================");
    
    let high_confidence_analyses: Vec<_> = analysis_results.iter()
        .filter(|(_, _, confidence)| *confidence > 0.6)
        .collect();
    
    println!("✅ High confidence analyses: {}/{}", high_confidence_analyses.len(), analysis_results.len());
    println!("📊 Average confidence: {:.1}%", 
        analysis_results.iter().map(|(_, _, c)| c).sum::<f64>() / analysis_results.len() as f64 * 100.0);
    
    if !high_confidence_analyses.is_empty() {
        println!("\n🏆 Key Insights (High Confidence):");
        for (i, (question, answer, confidence)) in high_confidence_analyses.iter().enumerate() {
            println!("{}. {} ({:.1}%)", i + 1, question, confidence * 100.0);
            println!("   💡 {}", answer.chars().take(100).collect::<String>());
            if answer.len() > 100 {
                println!("   ...");
            }
        }
    }
    
    // Display session statistics
    println!("\n📊 Session Statistics");
    println!("=====================");
    let stats = rag_orchestrator.get_conversation_stats();
    for (key, value) in stats {
        println!("   {}: {}", key, value);
    }
    
    // Memory consolidation
    println!("\n🧠 Consolidating Analysis Results...");
    match memory_system.consolidate().await {
        Ok(result) => {
            println!("✅ Consolidation complete:");
            println!("   Promoted to episodic: {} items", result.working_to_episodic);
            println!("   Extracted semantic concepts: {} items", result.episodic_to_semantic);
        }
        Err(e) => {
            println!("⚠️  Consolidation warning: {}", e);
        }
    }
    
    println!("\n✅ PocketFlow Analysis Complete!");
    println!("   Advanced analysis completed successfully with new service architecture.");
    
    Ok(())
} 