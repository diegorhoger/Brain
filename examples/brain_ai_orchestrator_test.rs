use brain::conversation::{RagOrchestrator, RagRequest};
use brain::memory::MemorySystem;
use brain::concept_graph::{ConceptGraphManager, ConceptGraphConfig};
use brain::insight_extraction::PatternDetector;
use brain::error::BrainError;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), BrainError> {
    println!("🧠 Brain AI Orchestrator Test - True AI Delegation");
    println!("=================================================");
    
    // Set up environment for testing
    if env::var("OPENAI_API_KEY").is_err() {
        println!("❌ OPENAI_API_KEY not set. Please set it in your .env file.");
        return Ok(());
    }
    
    // Ensure Brain AI delegation is enabled
    env::set_var("ENABLE_BRAIN_AI_DELEGATION", "true");
    
    println!("1. Initializing Brain AI systems...");
    
    // Initialize core systems
    let mut memory_system = MemorySystem::new(1000);
    let mut concept_graph = ConceptGraphManager::new(ConceptGraphConfig::default()).await?;
    let mut pattern_detector = PatternDetector::new();
    
    println!("2. Initializing RAG Orchestrator with Brain AI delegation...");
    
    // Initialize RAG orchestrator (should automatically create Brain AI Orchestrator)
    let mut rag_orchestrator = RagOrchestrator::new()?;
    
    println!("3. Testing Brain AI Orchestrator with various queries...");
    println!();
    
    // Test 1: GitHub repository analysis
    println!("🔍 Test 1: GitHub Repository Analysis");
    println!("Query: Tell me about PocketFlow");
    
    let github_request = RagRequest {
        message: "Tell me about PocketFlow".to_string(),
        conversation_id: Some("test-github-1".to_string()),
        context_limit: Some(10),
        retrieval_threshold: Some(0.3),
    };
    
    match rag_orchestrator.process_conversation(
        github_request,
        &mut memory_system,
        &mut concept_graph,
        &mut pattern_detector,
    ).await {
        Ok(response) => {
            println!("✅ Response received:");
            println!("  - Confidence: {:.3}", response.confidence_score);
            println!("  - Knowledge sources: {}", response.context_used.len());
            println!("  - Response preview: {}...", 
                     response.response.chars().take(200).collect::<String>());
            println!();
            
            // Show knowledge sources
            for (i, knowledge) in response.context_used.iter().enumerate() {
                println!("  📚 Knowledge Source {}: {} ({})", 
                         i + 1, knowledge.knowledge_type, knowledge.source);
                println!("      Relevance: {:.3}", knowledge.relevance_score);
                println!("      Content: {}...", 
                         knowledge.content.chars().take(100).collect::<String>());
                println!();
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    println!("=====================================");
    
    // Test 2: Pattern analysis
    println!("🔍 Test 2: Pattern Analysis");
    println!("Query: What patterns do you see in software development?");
    
    let pattern_request = RagRequest {
        message: "What patterns do you see in software development?".to_string(),
        conversation_id: Some("test-pattern-1".to_string()),
        context_limit: Some(10),
        retrieval_threshold: Some(0.3),
    };
    
    match rag_orchestrator.process_conversation(
        pattern_request,
        &mut memory_system,
        &mut concept_graph,
        &mut pattern_detector,
    ).await {
        Ok(response) => {
            println!("✅ Response received:");
            println!("  - Confidence: {:.3}", response.confidence_score);
            println!("  - Knowledge sources: {}", response.context_used.len());
            println!("  - Response preview: {}...", 
                     response.response.chars().take(200).collect::<String>());
            println!();
        },
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    println!("=====================================");
    
    // Test 3: Concept analysis
    println!("🔍 Test 3: Concept Analysis");
    println!("Query: Explain the concept of machine learning");
    
    let concept_request = RagRequest {
        message: "Explain the concept of machine learning".to_string(),
        conversation_id: Some("test-concept-1".to_string()),
        context_limit: Some(10),
        retrieval_threshold: Some(0.3),
    };
    
    match rag_orchestrator.process_conversation(
        concept_request,
        &mut memory_system,
        &mut concept_graph,
        &mut pattern_detector,
    ).await {
        Ok(response) => {
            println!("✅ Response received:");
            println!("  - Confidence: {:.3}", response.confidence_score);
            println!("  - Knowledge sources: {}", response.context_used.len());
            println!("  - Response preview: {}...", 
                     response.response.chars().take(200).collect::<String>());
            println!();
        },
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    println!("=====================================");
    
    // Test 4: Disable Brain AI delegation and compare
    println!("🔍 Test 4: Comparison with Traditional Retrieval");
    println!("Temporarily disabling Brain AI delegation...");
    
    env::set_var("ENABLE_BRAIN_AI_DELEGATION", "false");
    let mut traditional_rag = RagOrchestrator::new()?;
    
    let traditional_request = RagRequest {
        message: "Tell me about PocketFlow".to_string(),
        conversation_id: Some("test-traditional-1".to_string()),
        context_limit: Some(10),
        retrieval_threshold: Some(0.3),
    };
    
    match traditional_rag.process_conversation(
        traditional_request,
        &mut memory_system,
        &mut concept_graph,
        &mut pattern_detector,
    ).await {
        Ok(response) => {
            println!("✅ Traditional retrieval response:");
            println!("  - Confidence: {:.3}", response.confidence_score);
            println!("  - Knowledge sources: {}", response.context_used.len());
            println!("  - Response preview: {}...", 
                     response.response.chars().take(200).collect::<String>());
            println!();
        },
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    println!("=====================================");
    println!("🎯 Brain AI Orchestrator Test Complete!");
    println!("   The new orchestrator delegates to Brain AI's actual capabilities:");
    println!("   - GitHub Learning Engine for repository analysis");
    println!("   - Pattern Detector for insight extraction");
    println!("   - Concept Graph Manager for relationship discovery");
    println!("   - Semantic Memory for knowledge retrieval");
    println!("   This provides much richer, more detailed analysis than the");
    println!("   simplified fallback system used previously.");
    
    Ok(())
} 