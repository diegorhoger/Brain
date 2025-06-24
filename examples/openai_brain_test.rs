//! OpenAI Brain AI Integration Test
//! 
//! This example demonstrates the OpenAI ChatGPT integration with Brain AI impersonation.
//! It shows how Brain AI maintains its persona while using OpenAI as the underlying LLM.

use brain::{
    RagOrchestrator, RagRequest, MemorySystem, ConceptGraphManager, PatternDetector,
    BrainImpersonationHandler
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 Brain AI OpenAI Integration Test");
    println!("=====================================");

    // Check for OpenAI API key
    if env::var("OPENAI_API_KEY").is_err() {
        println!("❌ OPENAI_API_KEY environment variable not set");
        println!("Please set your OpenAI API key in the .env file or environment");
        return Ok(());
    }

    // Test Brain AI impersonation handler
    println!("\n🎭 Testing Brain AI Impersonation Handler");
    println!("-----------------------------------------");
    
    let impersonation_handler = BrainImpersonationHandler::default();
    
    // Test various responses that might come from OpenAI
    let test_responses = vec![
        "I am an AI assistant created by OpenAI to be helpful and harmless.",
        "As an AI language model, I don't have access to real-time information.",
        "I'm ChatGPT, and I cannot browse the internet or access external databases.",
        "I don't have the ability to remember our previous conversations.",
    ];
    
    for (i, response) in test_responses.iter().enumerate() {
        println!("\nTest {}: Original Response:", i + 1);
        println!("  \"{}\"", response);
        
        let processed = impersonation_handler.process_response(response);
        println!("  Processed Response:");
        println!("  \"{}\"", processed);
    }
    
    // Test system prompt generation
    println!("\n🤖 Brain AI System Prompt");
    println!("--------------------------");
    let system_prompt = impersonation_handler.get_brain_system_prompt();
    println!("{}", system_prompt);

    // Initialize Brain AI components
    println!("\n🧠 Initializing Brain AI Components");
    println!("------------------------------------");
    
    let mut memory_system = MemorySystem::new(1000);
    let mut concept_graph = ConceptGraphManager::new(Default::default()).await?;
    let mut pattern_detector = PatternDetector::new();
    
    println!("✅ Memory System initialized");
    println!("✅ Concept Graph initialized");
    println!("✅ Pattern Detector initialized");

    // Initialize RAG Orchestrator with OpenAI
    println!("\n🔗 Initializing RAG Orchestrator with OpenAI");
    println!("----------------------------------------------");
    
    match RagOrchestrator::new() {
        Ok(mut rag_orchestrator) => {
            println!("✅ RAG Orchestrator initialized with OpenAI");
            
            // Test conversation
            println!("\n💬 Testing Brain AI Conversation");
            println!("---------------------------------");
            
            let test_messages = vec![
                "Hello, what are you?",
                "Can you tell me about your capabilities?",
                "How do you access information?",
                "What makes you different from other AI systems?",
            ];
            
            for message in test_messages {
                println!("\n👤 User: {}", message);
                
                let request = RagRequest {
                    message: message.to_string(),
                    conversation_id: None,
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
                        println!("🧠 Brain AI: {}", response.response);
                        println!("   Confidence: {:.3}", response.confidence_score);
                        println!("   Knowledge sources used: {}", response.context_used.len());
                    },
                    Err(e) => {
                        println!("❌ Error: {}", e);
                    }
                }
            }
            
            // Display RAG orchestrator stats
            println!("\n📊 RAG Orchestrator Statistics");
            println!("-------------------------------");
            let stats = rag_orchestrator.get_stats();
            for (key, value) in stats {
                println!("  {}: {}", key, value);
            }
            
        },
        Err(e) => {
            println!("❌ Failed to initialize RAG Orchestrator: {}", e);
            println!("   Make sure OPENAI_API_KEY is set in your environment");
        }
    }

    println!("\n🎯 Test Complete!");
    println!("==================");
    println!("Brain AI is now configured to use OpenAI ChatGPT as the underlying LLM");
    println!("while maintaining the Brain AI persona and never mentioning external providers.");

    Ok(())
} 