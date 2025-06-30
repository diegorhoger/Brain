#!/usr/bin/env cargo run --example test_enhanced_pocketflow_analysis
//! Test Enhanced PocketFlow Analysis
//! 
//! Enhanced testing of PocketFlow analysis capabilities
//! with the new MemoryService and ConceptGraphService architecture.

use brain::*;
use brain::services::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🔬 Enhanced PocketFlow Analysis Test");
    println!("===================================");
    
    // Check for OpenAI API key
    let _openai_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("⚠️  OPENAI_API_KEY not set. Please set it to use this demo.");
        std::process::exit(1);
    });
    
    println!("✅ OpenAI API key found");
    
    // Initialize Brain AI components using new service architecture
    println!("\n🔧 Initializing Enhanced Brain AI Services...");
    let mut memory_system = create_memory_service_with_capacity(2000).await?;
    let mut concept_graph = create_concept_graph_service_default().await?;
    
    println!("✅ MemoryService initialized with enhanced capacity");
    println!("✅ ConceptGraphService initialized");
    
    // Load enhanced analysis dataset
    println!("\n📊 Loading Enhanced Analysis Dataset...");
    let enhanced_data = vec![
        "PocketFlow enhanced analysis shows 70% improvement in development velocity",
        "Advanced metrics indicate 50% reduction in debugging time with Node-Flow pattern",
        "Enhanced batch processing achieves 4x cost efficiency compared to sequential processing",
        "Advanced agent orchestration enables complex multi-step reasoning workflows",
        "Enhanced error handling provides 90% improvement in system reliability",
        "Advanced monitoring shows 99.5% uptime in production environments",
        "Enhanced scalability supports 1000+ concurrent agent operations",
        "Advanced integration patterns enable seamless third-party API connections",
        "Enhanced security features provide enterprise-grade protection mechanisms",
        "Advanced analytics provide real-time insights into AI workflow performance",
        "Enhanced testing framework ensures 95% code coverage with automated validation",
        "Advanced deployment strategies support zero-downtime production updates",
    ];
    
    for (i, data) in enhanced_data.iter().enumerate() {
        memory_system.learn(data.to_string(), Priority::High).await?;
        println!("✅ Loaded enhanced data {}", i + 1);
    }
    
    // Create RAG orchestrator for enhanced analysis
    println!("\n🤖 Initializing Enhanced Analysis System...");
    let mut rag_orchestrator = RagOrchestrator::new()?;
    
    // Enhanced analysis test cases
    let test_cases = vec![
        ("Performance", "What performance improvements does PocketFlow provide?"),
        ("Reliability", "How does PocketFlow ensure system reliability?"),
        ("Scalability", "What scalability features does PocketFlow offer?"),
        ("Security", "What security measures does PocketFlow implement?"),
        ("Integration", "How does PocketFlow handle third-party integrations?"),
        ("Monitoring", "What monitoring capabilities does PocketFlow provide?"),
        ("Testing", "How does PocketFlow ensure code quality?"),
        ("Deployment", "What deployment strategies does PocketFlow support?"),
    ];
    
    println!("\n🧪 Running Enhanced Analysis Test Suite");
    println!("=======================================");
    
    let mut test_results = Vec::new();
    
    for (i, (category, question)) in test_cases.iter().enumerate() {
        println!("\n🔬 Test {}: {} Analysis", i + 1, category);
        println!("   Question: {}", question);
        
        let request = RagRequest {
            message: question.to_string(),
            conversation_id: Some("enhanced_test_session".to_string()),
            context_limit: Some(8),
            retrieval_threshold: Some(0.2),
        };
        
        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
        ).await {
            Ok(response) => {
                println!("   📊 Result: {}", response.response);
                println!("   🎯 Confidence: {:.1}%", response.confidence_score * 100.0);
                println!("   📚 Sources: {}", response.context_used.len());
                
                // Evaluate test result
                let passed = response.confidence_score > 0.4 && response.context_used.len() > 0;
                test_results.push((category.to_string(), passed, response.confidence_score));
                
                if passed {
                    println!("   ✅ Test PASSED");
                } else {
                    println!("   ❌ Test FAILED (low confidence or no sources)");
                }
                
                // Learn from test result
                let test_insight = format!("Enhanced test {}: {} -> {}", category, question, response.response);
                memory_system.learn(test_insight, Priority::Medium).await?;
            }
            Err(e) => {
                println!("   ❌ Test ERROR: {}", e);
                test_results.push((category.to_string(), false, 0.0));
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(700)).await;
    }
    
    // Generate enhanced test report
    println!("\n📋 Enhanced Test Report");
    println!("=======================");
    
    let passed_tests: Vec<_> = test_results.iter().filter(|(_, passed, _)| *passed).collect();
    let total_tests = test_results.len();
    let pass_rate = (passed_tests.len() as f64 / total_tests as f64) * 100.0;
    
    println!("✅ Tests passed: {}/{} ({:.1}%)", passed_tests.len(), total_tests, pass_rate);
    
    if pass_rate >= 75.0 {
        println!("🏆 EXCELLENT: Enhanced analysis system performing exceptionally well");
    } else if pass_rate >= 50.0 {
        println!("✅ GOOD: Enhanced analysis system performing adequately");
    } else {
        println!("⚠️  NEEDS IMPROVEMENT: Enhanced analysis system requires optimization");
    }
    
    println!("\n📊 Detailed Test Results:");
    for (category, passed, confidence) in &test_results {
        let status = if *passed { "✅ PASS" } else { "❌ FAIL" };
        println!("   {}: {} (confidence: {:.1}%)", category, status, confidence * 100.0);
    }
    
    // Calculate average confidence
    let avg_confidence = test_results.iter().map(|(_, _, c)| c).sum::<f64>() / total_tests as f64;
    println!("\n📈 Average confidence: {:.1}%", avg_confidence * 100.0);
    
    // Display session statistics
    println!("\n📊 Session Statistics");
    println!("=====================");
    let stats = rag_orchestrator.get_conversation_stats();
    for (key, value) in stats {
        println!("   {}: {}", key, value);
    }
    
    // Enhanced memory consolidation
    println!("\n🧠 Enhanced Memory Consolidation...");
    match memory_system.consolidate().await {
        Ok(result) => {
            println!("✅ Enhanced consolidation complete:");
            println!("   Working to episodic: {} items", result.working_to_episodic);
            println!("   Episodic to semantic: {} items", result.episodic_to_semantic);
            println!("   Forgotten items: {} items", result.forgotten_events);
        }
        Err(e) => {
            println!("⚠️  Consolidation warning: {}", e);
        }
    }
    
    println!("\n✅ Enhanced PocketFlow Analysis Test Complete!");
    println!("   Enhanced testing completed successfully with new service architecture.");
    
    Ok(())
} 