//! Specialized Model Training Demo
//! 
//! Demonstrates specialized training capabilities using Brain AI
//! with the new MemoryService and ConceptGraphService architecture.

use brain::*;
use brain::services::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎓 Specialized Model Training Demo");
    println!("==================================");
    
    // Check for OpenAI API key
    let _openai_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("⚠️  OPENAI_API_KEY not set. Please set it to use this demo.");
        std::process::exit(1);
    });
    
    println!("✅ OpenAI API key found");
    
    // Initialize Brain AI components using new service architecture
    println!("\n🔧 Initializing Brain AI Training System...");
    let mut memory_system = create_memory_service_with_capacity(1000).await?;
    let mut concept_graph = create_concept_graph_service_default().await?;
    
    println!("✅ MemoryService initialized for training");
    println!("✅ ConceptGraphService initialized");
    
    // Load specialized training data
    println!("\n📚 Loading Specialized Training Data...");
    let training_data = vec![
        "Machine learning models improve through iterative training and validation",
        "Specialized training focuses on domain-specific knowledge and patterns",
        "Transfer learning enables knowledge sharing between related domains",
        "Active learning optimizes training efficiency by selecting informative examples",
        "Meta-learning enables models to learn how to learn new tasks quickly",
        "Multi-task learning allows models to benefit from related task knowledge",
        "Few-shot learning enables rapid adaptation to new tasks with minimal data",
        "Continual learning prevents catastrophic forgetting in sequential training",
        "Self-supervised learning leverages unlabeled data for representation learning",
        "Reinforcement learning optimizes behavior through reward-based feedback",
    ];
    
    for (i, data) in training_data.iter().enumerate() {
        memory_system.learn(data.to_string(), Priority::High).await?;
        println!("✅ Loaded training data {}", i + 1);
    }
    
    // Create training orchestrator
    println!("\n🤖 Initializing Training Orchestrator...");
    let mut rag_orchestrator = RagOrchestrator::new()?;
    
    // Training-focused questions
    let training_questions = vec![
        "What are the key principles of specialized model training?",
        "How does transfer learning improve training efficiency?",
        "What is the role of active learning in model training?",
        "How does meta-learning enable rapid task adaptation?",
        "What are the benefits of multi-task learning?",
        "How does few-shot learning work with minimal data?",
        "What strategies prevent catastrophic forgetting?",
        "How does self-supervised learning utilize unlabeled data?",
    ];
    
    println!("\n🎓 Training Knowledge Assessment");
    println!("===============================");
    
    let mut training_results = Vec::new();
    
    for (i, question) in training_questions.iter().enumerate() {
        println!("\n📝 Training Assessment {}: {}", i + 1, question);
        
        let request = RagRequest {
            message: question.to_string(),
            conversation_id: Some("training_session".to_string()),
            context_limit: Some(6),
            retrieval_threshold: Some(0.3),
        };
        
        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
        ).await {
            Ok(response) => {
                println!("🎯 Training Response:");
                println!("   {}", response.response);
                println!("   📊 Confidence: {:.1}%", response.confidence_score * 100.0);
                println!("   📚 Knowledge sources: {}", response.context_used.len());
                
                // Evaluate training effectiveness
                let effective = response.confidence_score > 0.5 && response.context_used.len() > 0;
                training_results.push((question.to_string(), effective, response.confidence_score));
                
                if effective {
                    println!("   ✅ Training EFFECTIVE");
                } else {
                    println!("   ⚠️  Training needs improvement");
                }
                
                // Store training interaction for learning
                let training_interaction = format!("Training Q: {} | A: {}", question, response.response);
                memory_system.learn(training_interaction, Priority::Medium).await?;
            }
            Err(e) => {
                println!("   ❌ Training error: {}", e);
                training_results.push((question.to_string(), false, 0.0));
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;
    }
    
    // Simulate specialized training iteration
    println!("\n🔄 Specialized Training Iteration");
    println!("=================================");
    
    // Add domain-specific knowledge
    let specialized_knowledge = vec![
        "Neural architecture search automates the discovery of optimal model structures",
        "Gradient accumulation enables large batch training on limited hardware",
        "Learning rate scheduling optimizes convergence during training",
        "Data augmentation increases training data diversity and model robustness",
        "Regularization techniques prevent overfitting in complex models",
    ];
    
    for (i, knowledge) in specialized_knowledge.iter().enumerate() {
        memory_system.learn(knowledge.to_string(), Priority::High).await?;
        println!("✅ Added specialized knowledge {}", i + 1);
    }
    
    // Test improvement after specialized training
    let post_training_question = "What advanced techniques optimize neural network training?";
    println!("\n🔍 Post-Training Assessment: {}", post_training_question);
    
    let request = RagRequest {
        message: post_training_question.to_string(),
        conversation_id: Some("post_training_session".to_string()),
        context_limit: Some(8),
        retrieval_threshold: Some(0.25),
    };
    
    match rag_orchestrator.process_conversation(
        request,
        &mut memory_system,
        &mut concept_graph,
    ).await {
        Ok(response) => {
            println!("🎯 Post-Training Response:");
            println!("   {}", response.response);
            println!("   📊 Confidence: {:.1}%", response.confidence_score * 100.0);
            println!("   📚 Knowledge sources: {}", response.context_used.len());
            
            if response.confidence_score > 0.6 {
                println!("   🏆 Excellent improvement after specialized training!");
            } else {
                println!("   📈 Some improvement observed");
            }
        }
        Err(e) => {
            println!("   ❌ Post-training assessment error: {}", e);
        }
    }
    
    // Generate training report
    println!("\n📋 Training Effectiveness Report");
    println!("================================");
    
    let effective_training: Vec<_> = training_results.iter()
        .filter(|(_, effective, _)| *effective)
        .collect();
    
    let total_training = training_results.len();
    let effectiveness_rate = (effective_training.len() as f64 / total_training as f64) * 100.0;
    
    println!("✅ Effective training sessions: {}/{} ({:.1}%)", 
        effective_training.len(), total_training, effectiveness_rate);
    
    let avg_confidence = training_results.iter()
        .map(|(_, _, c)| c)
        .sum::<f64>() / total_training as f64;
    println!("📊 Average training confidence: {:.1}%", avg_confidence * 100.0);
    
    // Training consolidation
    println!("\n🧠 Training Memory Consolidation...");
    match memory_system.consolidate().await {
        Ok(result) => {
            println!("✅ Training consolidation complete:");
            println!("   Training data consolidated: {} items", result.working_to_episodic);
            println!("   Specialized concepts formed: {} items", result.episodic_to_semantic);
        }
        Err(e) => {
            println!("⚠️  Consolidation warning: {}", e);
        }
    }
    
    // Display session statistics
    println!("\n📊 Training Session Statistics");
    println!("==============================");
    let stats = rag_orchestrator.get_conversation_stats();
    for (key, value) in stats {
        println!("   {}: {}", key, value);
    }
    
    println!("\n✅ Specialized Model Training Demo Complete!");
    println!("   Training capabilities demonstrated successfully with new service architecture.");
    
    Ok(())
} 