//! Novelty Detection System Demonstration
//! 
//! This example demonstrates the capabilities of Task 9.2:
//! - Statistical novelty detection comparing inputs against knowledge distributions
//! - Surprise metrics quantifying deviation from expected patterns
//! - Anomaly detection for identifying outlier inputs
//! - Context-based novelty assessment considering task context
//! - Novelty scoring system (0-1 range) combining multiple detection methods
//! - Integration with meta-memory system for confidence-based assessments

use anyhow::Result;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use brain::{
    MetaMemorySystem, MetaMemoryConfig, MetaMemoryItem, KnowledgeType,
    NoveltyDetectionEngine, NoveltyDetectionConfig, NoveltyContext, NoveltyLevel
};

fn main() -> Result<()> {
    println!("🔍 Novelty Detection System Demonstration - Task 9.2");
    println!("{}", "=".repeat(70));

    // Phase 1: Initialize Systems
    println!("\n🚀 Phase 1: Initialize Meta-Memory and Novelty Detection Systems");
    println!("{}", "-".repeat(50));
    
    // Initialize meta-memory system
    let meta_memory_config = MetaMemoryConfig {
        database_path: "novelty_demo.db".to_string(),
        high_confidence_threshold: 0.8,
        low_confidence_threshold: 0.3,
        ..Default::default()
    };
    
    let meta_memory_arc = Arc::new(Mutex::new(
        MetaMemorySystem::with_config(meta_memory_config)?
    ));
    
    println!("✅ Meta-memory system initialized");
    
    // Initialize novelty detection system
    let novelty_config = NoveltyDetectionConfig {
        high_novelty_threshold: 0.7,
        low_novelty_threshold: 0.3,
        statistical_weight: 0.4,
        confidence_weight: 0.3,
        context_weight: 0.3,
        min_sample_size: 5,
        context_window_size: 5,
        enable_logging: true,
        max_novelty_records: 1000,
    };
    
    let mut novelty_engine = NoveltyDetectionEngine::new(
        novelty_config,
        meta_memory_arc.clone()
    )?;
    
    println!("✅ Novelty detection engine initialized");

    // Phase 2: Populate Meta-Memory with Known Patterns
    println!("\n📚 Phase 2: Populate Meta-Memory with Known Patterns");
    println!("{}", "-".repeat(50));
    
    // Add various knowledge components to establish baseline distributions
    let known_patterns = [
        (KnowledgeType::Segment, 0.9, "Common segment: 'the'"),
        (KnowledgeType::Segment, 0.85, "Frequent pattern: 'ing'"),
        (KnowledgeType::Segment, 0.8, "Regular occurrence: 'tion'"),
        (KnowledgeType::Segment, 0.7, "Standard segment: 'er'"),
        (KnowledgeType::Segment, 0.6, "Typical pattern: 'ly'"),
        (KnowledgeType::ConceptNode, 0.95, "Well-established concept: 'animal'"),
        (KnowledgeType::ConceptNode, 0.9, "Clear concept: 'food'"),
        (KnowledgeType::ConceptNode, 0.85, "Solid concept: 'transportation'"),
        (KnowledgeType::Rule, 0.8, "Reliable rule: if hungry then eat"),
        (KnowledgeType::Rule, 0.75, "Good rule: if raining then umbrella"),
        (KnowledgeType::Rule, 0.7, "Standard rule: if tired then sleep"),
        (KnowledgeType::SemanticConcept, 0.9, "Core concept: 'learning'"),
        (KnowledgeType::SemanticConcept, 0.85, "Basic concept: 'communication'"),
        (KnowledgeType::WorkingMemory, 0.6, "Current task: reading email"),
        (KnowledgeType::EpisodicMemory, 0.8, "Yesterday: went to store"),
        (KnowledgeType::Pattern, 0.7, "Common pattern: greeting->conversation"),
    ];
    
    {
        let mut meta_memory = meta_memory_arc.lock().unwrap();
        for (knowledge_type, confidence, description) in known_patterns.iter() {
            let component_id = Uuid::new_v4();
            let mut item = MetaMemoryItem::new(
                component_id,
                knowledge_type.clone(),
                *confidence,
                description.to_string(),
            );
            
            // Simulate some validation history
            for i in 0..10 {
                let success = i < (confidence * 10.0) as usize; // Success rate matches confidence
                item.update_confidence(success);
            }
            
            meta_memory.store_item(item)?;
            println!("📝 Added {}: {} (confidence: {:.2})", 
                knowledge_type, description, confidence);
        }
    }
    
    println!("✅ {} known patterns added to meta-memory", known_patterns.len());

    // Phase 3: Test Novelty Detection with Various Inputs
    println!("\n🎯 Phase 3: Novelty Detection Testing");
    println!("{}", "-".repeat(50));
    
    // Test inputs with expected novelty levels
    let test_inputs = [
        // Low novelty (familiar patterns)
        ("Hello, how are you?", "general", "Familiar greeting pattern"),
        ("I need food", "general", "Common need expression"),
        ("The weather is nice", "general", "Standard weather comment"),
        
        // Medium novelty (somewhat unexpected)
        ("Quantum entanglement in cooking", "science", "Unusual domain mixing"),
        ("Purple elephants dance silently", "creative", "Creative but comprehensible"),
        ("Algorithm learns to paint emotions", "technology", "Novel AI application"),
        
        // High novelty (very unexpected)
        ("Zxqwtyuiop asdfghjkl vbnm", "random", "Random character sequence"),
        ("The table dreams of algebraic poetry while singing numerical lullabies", "surreal", "Highly abstract concept"),
        ("!@#$%^&*()_+{}|:<>?", "symbols", "Pure symbol input"),
        
        // Anomalous inputs
        ("aaaaaaaaaaaaaaaaaaaaaaaaa", "repetitive", "Highly repetitive content"),
        ("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz", "long", "Very long repetitive sequence"),
    ];

    println!("\n🔍 Testing Novelty Detection on Various Inputs:\n");

    for (i, (input, task_context, description)) in test_inputs.iter().enumerate() {
        // Create context for this test
        let context = NoveltyContext {
            task_context: task_context.to_string(),
            recent_inputs: if i > 0 {
                test_inputs[..i.min(3)]
                    .iter()
                    .map(|(inp, _, _)| inp.to_string())
                    .collect()
            } else {
                Vec::new()
            },
            active_components: Vec::new(),
            temporal_context: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };

        // Assess novelty
        let assessment = novelty_engine.assess_novelty(input, Some(context))?;
        let level = assessment.get_novelty_level(novelty_engine.get_config());
        
        println!("{}. Input: \"{}\"", i + 1, input);
        println!("   Description: {}", description);
        println!("   Context: {}", task_context);
        println!("   ┌─ Novelty Score: {:.3} ({})", assessment.novelty_score, level);
        println!("   ├─ Assessment Confidence: {:.3}", assessment.assessment_confidence);
        println!("   └─ Method Breakdown:");
        
        // Display method scores
        for (method, score) in &assessment.method_scores {
            println!("      • {:?}: {:.3}", method, score);
        }
        
        // Display key explanations
        if !assessment.explanation.is_empty() {
            println!("   📋 Key Findings:");
            for explanation in assessment.explanation.iter().take(2) {
                println!("      • {}", explanation);
            }
        }
        
        // Display recommendations
        if !assessment.recommendations.is_empty() {
            println!("   💡 Recommendations:");
            for rec in assessment.recommendations.iter().take(2) {
                println!("      • {}", rec);
            }
        }
        
        println!();
    }

    // Phase 4: Context-Aware Novelty Testing
    println!("\n🌍 Phase 4: Context-Aware Novelty Testing");
    println!("{}", "-".repeat(50));
    
    println!("Testing how context affects novelty assessment:\n");
    
    let context_test_input = "Machine learning model";
    let contexts = [
        ("technology", "Technology discussion context"),
        ("cooking", "Cooking/culinary context"),
        ("poetry", "Creative writing context"),
        ("general", "General conversation context"),
    ];
    
    for (context_name, context_desc) in contexts.iter() {
        let context = NoveltyContext {
            task_context: context_name.to_string(),
            recent_inputs: vec![format!("Previous discussion about {}", context_name)],
            ..Default::default()
        };
        
        let assessment = novelty_engine.assess_novelty(context_test_input, Some(context))?;
        let level = assessment.get_novelty_level(novelty_engine.get_config());
        
        println!("Input: \"{}\" in {} context", context_test_input, context_name);
        println!("  Novelty: {:.3} ({}) - {}", 
            assessment.novelty_score, level, context_desc);
        
        if let Some(context_score) = assessment.method_scores.get(&brain::NoveltyMethod::ContextBased) {
            println!("  Context-specific score: {:.3}", context_score);
        }
        println!();
    }

    // Phase 5: Integration API Demonstration
    println!("\n🔗 Phase 5: Integration API Demonstration");
    println!("{}", "-".repeat(50));
    
    println!("Demonstrating API capabilities for other system components:\n");
    
    // Example API usage for other Brain components
    let api_test_inputs = [
        "New learned segment pattern",
        "Discovered rule relationship", 
        "Novel concept formation",
    ];
    
    for input in api_test_inputs.iter() {
        let assessment = novelty_engine.assess_novelty(input, None)?;
        let level = assessment.get_novelty_level(novelty_engine.get_config());
        
        println!("API Query: \"{}\"", input);
        println!("  Response: Novelty {:.3} ({})", assessment.novelty_score, level);
        
        // Show how other components might use this information
        match level {
            NoveltyLevel::High => println!("  → System Action: Prioritize for learning and exploration"),
            NoveltyLevel::Medium => println!("  → System Action: Schedule for additional validation"),
            NoveltyLevel::Low => println!("  → System Action: Process with standard confidence"),
        }
        println!();
    }

    // Phase 6: System Analytics and Performance
    println!("\n📊 Phase 6: System Analytics and Performance");
    println!("{}", "-".repeat(50));
    
    let stats = novelty_engine.get_stats();
    println!("📈 Novelty Detection Statistics:");
    println!("  • Total assessments performed: {}", stats.total_assessments);
    println!("  • Average novelty score: {:.3}", stats.average_novelty_score);
    println!("  • Average assessment confidence: {:.3}", stats.average_assessment_confidence);
    
    println!("\n🎭 Novelty Level Distribution:");
    let total = stats.total_assessments;
    for (level, count) in &stats.novelty_distribution {
        let percentage = if total > 0 { *count as f64 / total as f64 * 100.0 } else { 0.0 };
        println!("  • {}: {} assessments ({:.1}%)", level, count, percentage);
    }
    
    println!("\n🔧 Method Usage Statistics:");
    for (method, count) in &stats.method_usage {
        println!("  • {:?}: {} times", method, count);
    }
    
    if !stats.common_contexts.is_empty() {
        println!("\n🌐 Common Contexts:");
        for (context, count) in &stats.common_contexts {
            println!("  • '{}': {} assessments", context, count);
        }
    }

    // Phase 7: Novelty Level Analysis
    println!("\n🎯 Phase 7: Novelty Level Analysis");
    println!("{}", "-".repeat(50));
    
    println!("High Novelty Assessments:");
    let high_novelty = novelty_engine.get_assessments_by_level(NoveltyLevel::High);
    for (i, assessment) in high_novelty.iter().enumerate().take(3) {
        println!("  {}. \"{}\" (score: {:.3})", 
            i + 1, assessment.input, assessment.novelty_score);
        if !assessment.recommendations.is_empty() {
            println!("     → {}", assessment.recommendations[0]);
        }
    }
    
    println!("\nLow Novelty Assessments:");
    let low_novelty = novelty_engine.get_assessments_by_level(NoveltyLevel::Low);
    for (i, assessment) in low_novelty.iter().enumerate().take(3) {
        println!("  {}. \"{}\" (score: {:.3})", 
            i + 1, assessment.input, assessment.novelty_score);
    }

    // Phase 8: Export and Analysis
    println!("\n💾 Phase 8: Export and Analysis Capabilities");
    println!("{}", "-".repeat(50));
    
    println!("Recent assessment history (last 5 assessments):");
    let recent = novelty_engine.get_recent_assessments(5);
    for (i, assessment) in recent.iter().enumerate() {
        println!("  {}. \"{}...\" - Novelty: {:.3} ({})",
            i + 1,
            assessment.input.chars().take(20).collect::<String>(),
            assessment.novelty_score,
            assessment.get_novelty_level(novelty_engine.get_config())
        );
    }
    
    // Export assessments (truncated for demo)
    println!("\n📤 Assessment export capability available");
    println!("   (JSON export with {} total assessments)", stats.total_assessments);

    // Final Summary
    println!("\n🎉 Task 9.2 Novelty Detection - DEMONSTRATION COMPLETE!");
    println!("{}", "=".repeat(70));
    println!("✅ Statistical novelty detection operational");
    println!("✅ Confidence-based assessment using meta-memory");
    println!("✅ Context-aware novelty evaluation");
    println!("✅ Anomaly detection for outlier identification");
    println!("✅ Composite novelty scoring (0-1 range)");
    println!("✅ Comprehensive logging and analytics");
    println!("✅ API integration for other Brain components");
    println!("✅ Export capabilities for analysis and visualization");
    println!("\n🎯 Novelty detection system ready for Task 9.3: Curiosity-Driven Learning!");

    Ok(())
} 