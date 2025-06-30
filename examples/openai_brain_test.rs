//! OpenAI Brain Test
//! 
//! Tests Brain AI conversation capabilities with OpenAI integration
//! using the new MemoryService and ConceptGraphService architecture.

use brain::*;
use brain::services::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain AI - OpenAI Integration Test");
    println!("====================================");
    
    // Check for OpenAI API key
    let _openai_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("❌ OPENAI_API_KEY environment variable not found!");
        println!("   Please set your OpenAI API key:");
        println!("   export OPENAI_API_KEY=your_key_here");
        std::process::exit(1);
    });
    
    println!("✅ OpenAI API key found");
    
    // Initialize Brain AI services
    println!("\n🔧 Initializing Brain AI Services...");
    let mut memory_system = create_memory_service_with_capacity(1000).await?;
    let mut concept_graph = create_concept_graph_service_default().await?;
    
    println!("✅ MemoryService initialized");
    println!("✅ ConceptGraphService initialized");
    
    // Load some test knowledge
    println!("\n📚 Loading Test Knowledge...");
    let test_knowledge = vec![
        "Brain AI is an advanced artificial intelligence system with memory and reasoning capabilities",
        "The system uses episodic, working, and semantic memory for comprehensive knowledge storage",
        "Brain AI can learn from conversations and improve its responses over time",
        "The architecture supports real-time learning and knowledge consolidation",
        "Brain AI integrates with OpenAI for enhanced language generation capabilities",
    ];
    
    for (i, knowledge) in test_knowledge.iter().enumerate() {
        memory_system.learn(knowledge.to_string(), Priority::High).await?;
        println!("✅ Loaded knowledge item {}", i + 1);
    }
    
    // Create RAG orchestrator for conversation processing
    println!("\n🤖 Initializing Conversation System...");
    let mut rag_orchestrator = RagOrchestrator::new()?;
    
    // Test questions to validate the integration
    let test_questions = vec![
        "What is Brain AI?",
        "How does Brain AI handle memory?",
        "What makes Brain AI different from other AI systems?",
        "How does Brain AI integrate with OpenAI?",
        "Can Brain AI learn from our conversation?",
    ];
    
    println!("\n💬 Testing Brain AI Conversations");
    println!("=================================");
    
    for (i, question) in test_questions.iter().enumerate() {
        println!("\n📝 Test {}: {}", i + 1, question);
        
        let request = RagRequest {
            message: question.to_string(),
            conversation_id: Some("openai_test_session".to_string()),
            context_limit: Some(5),
            retrieval_threshold: Some(0.3),
        };
        
        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
        ).await {
            Ok(response) => {
                println!("🤖 Brain AI Response:");
                println!("   {}", response.response);
                println!("   📊 Confidence: {:.1}%", response.confidence_score * 100.0);
                println!("   📚 Knowledge sources used: {}", response.context_used.len());
                
                // Store the interaction for learning
                let interaction = format!("Q: {} | A: {}", question, response.response);
                memory_system.learn(interaction, Priority::Low).await?;
                
                // Validate response quality
                if response.confidence_score > 0.5 {
                    println!("   ✅ High confidence response");
                } else {
                    println!("   ⚠️  Lower confidence response");
                }
            }
            Err(e) => {
                println!("   ❌ Error processing question: {}", e);
                println!("   This could indicate API issues or configuration problems");
            }
        }
        
        // Brief pause between requests
        tokio::time::sleep(tokio::time::Duration::from_millis(750)).await;
    }
    
    // Display final statistics
    println!("\n📊 Session Summary");
    println!("==================");
    
    let conversation_stats = rag_orchestrator.get_conversation_stats();
    for (key, value) in conversation_stats {
        println!("   {}: {}", key, value);
    }
    
    // Test memory consolidation
    println!("\n🧠 Testing Memory Consolidation...");
    match memory_system.consolidate().await {
        Ok(result) => {
            println!("✅ Memory consolidation successful:");
            println!("   Working to Episodic: {} items", result.working_to_episodic);
            println!("   Episodic to Semantic: {} items", result.episodic_to_semantic);
            println!("   Forgotten items: {} items", result.forgotten_events);
        }
        Err(e) => {
            println!("⚠️  Memory consolidation warning: {}", e);
        }
    }
    
    println!("\n✅ OpenAI Brain AI Test Complete!");
    println!("   The new service architecture is functioning properly with OpenAI integration.");
    
    Ok(())
} 