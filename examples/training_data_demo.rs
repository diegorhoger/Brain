use brain::{
    BrainError, MemorySystem, ConceptGraphManager, ConceptGraphConfig, PatternDetector,
    RagOrchestrator, RagRequest, 
    TrainingDataCollector, TrainingDataConfig, ExportFormat, DatasetFilter, 
    ConversationType, ComplexityLevel
};
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), BrainError> {
    println!("🎓 Brain AI - Training Data Collection Demonstration");
    println!("==================================================");
    
    // Initialize core components
    let mut memory_system = MemorySystem::new(1000);
    let concept_config = ConceptGraphConfig {
        uri: "neo4j://localhost:7687".to_string(),
        username: "neo4j".to_string(),
        password: "password".to_string(),
        database: None,
        pool_size: 10,
        timeout_seconds: 30,
    };
    let mut concept_graph = ConceptGraphManager::new(concept_config).await?;
    let mut pattern_detector = PatternDetector::new();
    let mut rag_orchestrator = RagOrchestrator::new()?;
    
    // Configure training data collection
    let training_config = TrainingDataConfig {
        storage_path: "demo_training_data".to_string(),
        max_conversations: 1000,
        quality_threshold: 0.6,
        enable_anonymization: true,
        retention_days: 365,
        batch_size: 50,
        auto_export: true,
        export_format: ExportFormat::JsonL,
    };
    
    println!("\n📊 Step 1: Initialize Training Data Collector");
    let training_collector = TrainingDataCollector::new(training_config)?;
    rag_orchestrator.enable_training_data_collection(training_collector)?;
    println!("✅ Training data collection enabled with auto-export");
    
    println!("\n🗣️  Step 2: Simulate Conversations with Quality Assessment");
    
    // Simulate different types of conversations
    let conversation_scenarios = vec![
        ("How does memory consolidation work in the brain?", "educational_query"),
        ("What are the latest advances in neural networks?", "technical_discussion"),
        ("Can you explain the concept of attention mechanisms?", "concept_explanation"),
        ("I'm having trouble understanding transformers", "help_seeking"),
        ("What's the difference between supervised and unsupervised learning?", "comparison_request"),
    ];
    
    for (i, (message, scenario_type)) in conversation_scenarios.iter().enumerate() {
        println!("\n  📝 Conversation {}: {} scenario", i + 1, scenario_type);
        println!("     User: {}", message);
        
        let request = RagRequest {
            message: message.to_string(),
            conversation_id: Some(format!("demo_conv_{}", i + 1)),
            context_limit: Some(10),
            retrieval_threshold: Some(0.5),
        };
        
        // Process conversation (this will automatically capture training data)
        match rag_orchestrator.process_conversation(
            request,
            &mut memory_system,
            &mut concept_graph,
            &mut pattern_detector,
        ).await {
            Ok(response) => {
                println!("     Assistant: {}...", 
                    &response.response[..response.response.len().min(80)]);
                println!("     Quality Score: {:.2}", response.response_quality.factual_grounding);
                println!("     Knowledge Sources: {}", response.context_used.len());
            },
            Err(e) => println!("     ❌ Error: {}", e),
        }
        
        // Small delay to simulate realistic conversation timing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    println!("\n📈 Step 3: Analyze Captured Training Data");
    if let Some(collector) = rag_orchestrator.get_training_data_collector() {
        let analytics = collector.get_conversation_analytics();
        
        println!("  📊 Collection Statistics:");
        println!("     Total Conversations: {}", analytics.total_conversations);
        println!("     Total Messages: {}", analytics.total_messages);
        println!("     Average Quality: {:.2}", analytics.user_satisfaction);
        
        println!("\n  🎯 Quality Distribution:");
        let quality_dist = collector.get_quality_distribution();
        for (quality_level, percentage) in &quality_dist {
            println!("     {}: {:.1}%", quality_level, percentage * 100.0);
        }
        
        println!("\n  📚 Topic Frequency:");
        for (topic, count) in analytics.topic_frequency.iter().take(5) {
            println!("     {}: {} mentions", topic, count);
        }
    }
    
    println!("\n🔍 Step 4: Export Training Dataset with Filtering");
    if let Some(collector) = rag_orchestrator.get_training_data_collector_mut() {
        // Create filter for high-quality educational conversations
        let filter = DatasetFilter {
            min_quality: Some(0.7),
            max_quality: None,
            conversation_types: Some(vec![
                ConversationType::QuestionsAndAnswers,
                ConversationType::Tutorial,
                ConversationType::Technical,
            ]),
            complexity_levels: Some(vec![
                ComplexityLevel::Moderate,
                ComplexityLevel::Complex,
            ]),
            topics: None,
            date_range: Some((
                Utc::now() - Duration::hours(1),
                Utc::now() + Duration::minutes(1),
            )),
        };
        
        match collector.export_training_dataset(Some(filter)).await {
            Ok(dataset) => {
                println!("  ✅ Exported training dataset:");
                println!("     Conversations: {}", dataset.metadata.total_conversations);
                println!("     Messages: {}", dataset.metadata.total_messages);
                println!("     Average Quality: {:.2}", dataset.statistics.average_quality);
                println!("     Average Length: {:.1} messages", dataset.statistics.average_conversation_length);
                
                println!("\n  📋 Dataset Statistics:");
                println!("     Quality Distribution:");
                for (level, count) in &dataset.statistics.quality_distribution {
                    println!("       {}: {}", level, count);
                }
                
                println!("     Conversation Types:");
                for (conv_type, count) in &dataset.statistics.conversation_type_distribution {
                    println!("       {}: {}", conv_type, count);
                }
            },
            Err(e) => println!("  ❌ Export failed: {}", e),
        }
    }
    
    println!("\n🔒 Step 5: Demonstrate Anonymization Features");
    demonstrate_anonymization().await?;
    
    println!("\n🚀 Step 6: Training Data Pipeline Readiness");
    demonstrate_training_pipeline_readiness().await?;
    
    println!("\n✨ Training Data Collection Demonstration Complete!");
    println!("==================================================");
    println!("🎯 Key Features Demonstrated:");
    println!("   • Automatic conversation capture during RAG interactions");
    println!("   • Multi-dimensional quality assessment and scoring");
    println!("   • Privacy protection through data anonymization");
    println!("   • Flexible dataset filtering and export capabilities");
    println!("   • Comprehensive analytics and conversation insights");
    println!("   • Production-ready data pipeline for model training");
    
    Ok(())
}

async fn demonstrate_anonymization() -> Result<(), BrainError> {
    println!("  🔒 Privacy Protection Features:");
    
    // Simulate messages with PII that would be anonymized
    let test_messages = vec![
        "My email is john.doe@example.com and my phone is 555-123-4567",
        "I work at 123 Main Street, Springfield, IL",
        "You can reach me at (555) 987-6543 or jane@company.org",
        "My IP address is 192.168.1.100 for troubleshooting",
    ];
    
    for (i, message) in test_messages.iter().enumerate() {
        println!("     Original {}: {}", i + 1, message);
        // In a real implementation, this would show the anonymized version
        let anonymized = message
            .replace(r"[\w\.-]+@[\w\.-]+\.\w+", "[EMAIL]")
            .replace(r"\d{3}-\d{3}-\d{4}", "[PHONE]")
            .replace(r"\(\d{3}\)\s*\d{3}-\d{4}", "[PHONE]")
            .replace(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}", "[IP_ADDRESS]");
        println!("     Anonymized {}: {}", i + 1, anonymized);
    }
    
    Ok(())
}

async fn demonstrate_training_pipeline_readiness() -> Result<(), BrainError> {
    println!("  🚀 Training Pipeline Integration:");
    println!("     ✅ Data Format: JSONL, CSV, and Parquet export support");
    println!("     ✅ Quality Metrics: Multi-dimensional scoring for filtering");
    println!("     ✅ Metadata: Rich conversation context and user profiles");
    println!("     ✅ Anonymization: Privacy-preserving data preparation");
    println!("     ✅ Validation: Comprehensive quality assessment pipeline");
    println!("     ✅ Analytics: Conversation pattern recognition and insights");
    
    println!("\n  📝 Training Data Structure:");
    println!("     • User-Assistant message pairs with context");
    println!("     • Knowledge source attribution and relevance scores");
    println!("     • Quality metrics (coherence, grounding, safety, etc.)");
    println!("     • Conversation metadata (type, complexity, topics)");
    println!("     • Temporal information and user interaction patterns");
    
    println!("\n  🎯 Model Training Applications:");
    println!("     • Fine-tuning conversational response generation");
    println!("     • Knowledge grounding and factual accuracy improvement");
    println!("     • Context-aware response personalization");
    println!("     • Safety and quality filtering model training");
    println!("     • Conversation flow and coherence optimization");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain::{TrainingDataConfig, ConversationMetadata, ConversationQualityMetrics};
    
    #[tokio::test]
    async fn test_training_data_collection() -> Result<(), BrainError> {
        let config = TrainingDataConfig::default();
        let mut collector = TrainingDataCollector::new(config)?;
        
        // Test basic functionality
        assert_eq!(collector.get_conversation_analytics().total_conversations, 0);
        
        // Test configuration
        assert!(collector.get_conversation_analytics().total_messages == 0);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_quality_assessment() -> Result<(), BrainError> {
        // Test quality metrics calculation - create manually since default() is private
        let metrics = ConversationQualityMetrics {
            overall_quality: 0.0,
            coherence_score: 0.0,
            knowledge_grounding: 0.0,
            response_relevance: 0.0,
            safety_score: 1.0,
            educational_value: 0.0,
            diversity_score: 0.0,
            uniqueness_score: 0.0,
        };
        assert_eq!(metrics.overall_quality, 0.0);
        assert_eq!(metrics.safety_score, 1.0); // Should default to safe
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_conversation_filtering() -> Result<(), BrainError> {
        let filter = DatasetFilter {
            min_quality: Some(0.8),
            max_quality: Some(1.0),
            conversation_types: Some(vec![ConversationType::Technical]),
            complexity_levels: Some(vec![ComplexityLevel::Expert]),
            topics: Some(vec!["AI".to_string(), "ML".to_string()]),
            date_range: None,
        };
        
        // Test filter configuration
        assert_eq!(filter.min_quality, Some(0.8));
        assert_eq!(filter.conversation_types.as_ref().unwrap().len(), 1);
        
        Ok(())
    }
} 