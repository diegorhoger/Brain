//! Curiosity-Driven Learning Demo
//! 
//! This example demonstrates the complete curiosity-driven learning system,
//! showcasing how it integrates with meta-memory and novelty detection to
//! create intelligent learning priorities and adaptive exploration behavior.

use anyhow::Result;
use brain::{
    CuriosityLearningEngine, CuriosityConfig, LearningEvent,
    MetaMemorySystem, MetaMemoryItem, KnowledgeType,
    NoveltyDetectionEngine, NoveltyDetectionConfig, NoveltyContext,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tempfile::tempdir;
use uuid::Uuid;

fn main() -> Result<()> {
    println!("🧠 Brain AI - Curiosity-Driven Learning System Demo");
    println!("===================================================");
    println!();

    // Phase 1: System Initialization
    println!("📋 Phase 1: System Initialization");
    println!("----------------------------------");
    
    let temp_dir = tempdir()?;
    let meta_memory_path = temp_dir.path().join("curiosity_meta.db");
    let _novelty_db_path = temp_dir.path().join("curiosity_novelty.db");
    let curiosity_db_path = temp_dir.path().join("curiosity_learning.db");
    
    // Initialize meta-memory system
    let meta_memory = Arc::new(Mutex::new(MetaMemorySystem::new(&meta_memory_path)?));
    println!("✅ Meta-memory system initialized");
    
    // Initialize novelty detection engine
    let novelty_config = NoveltyDetectionConfig::default();
    let novelty_engine = Arc::new(Mutex::new(
        NoveltyDetectionEngine::new(novelty_config, meta_memory.clone())?
    ));
    println!("✅ Novelty detection engine initialized");
    
    // Initialize curiosity-driven learning engine
    let curiosity_config = CuriosityConfig {
        novelty_weight: 0.4,
        uncertainty_weight: 0.3,
        progress_weight: 0.3,
        learning_threshold: 0.25,
        exploration_rate: 0.7,
        database_path: curiosity_db_path.to_string_lossy().to_string(),
        ..Default::default()
    };
    
    let mut curiosity_engine = CuriosityLearningEngine::new(
        curiosity_config.clone(),
        meta_memory.clone(),
        novelty_engine.clone(),
    )?;
    println!("✅ Curiosity-driven learning engine initialized");
    println!();

    // Phase 2: Populate Meta-Memory with Sample Knowledge
    println!("📊 Phase 2: Populating Meta-Memory with Sample Knowledge");
    println!("--------------------------------------------------------");
    
    populate_sample_knowledge(&meta_memory)?;
    println!("✅ Sample knowledge components added to meta-memory");
    
    let stats = {
        let meta_memory_guard = meta_memory.lock().unwrap();
        meta_memory_guard.get_stats().clone()
    };
    println!("📈 Meta-memory statistics:");
    println!("   • Total components: {}", stats.total_components);
    println!("   • Average confidence: {:.3}", stats.average_confidence);
    println!("   • High confidence components: {}", stats.high_confidence_count);
    println!("   • Low confidence components: {}", stats.low_confidence_count);
    println!();

    // Phase 3: Test Curiosity Assessment
    println!("🔍 Phase 3: Curiosity Assessment Tests");
    println!("--------------------------------------");
    
    let test_inputs = vec![
        ("The quantum nature of reality suggests that observation affects outcome", "physics"),
        ("Machine learning algorithms can exhibit emergent behavior", "AI"),
        ("Economic systems show fractal patterns at multiple scales", "economics"),
        ("Biological networks demonstrate small-world properties", "biology"),
        ("Language evolution follows power law distributions", "linguistics"),
        ("Consciousness may emerge from complex information integration", "neuroscience"),
        ("Social networks exhibit preferential attachment dynamics", "sociology"),
        ("Climate systems display chaotic behavior patterns", "climate"),
        ("Mathematical proofs can be verified automatically", "mathematics"),
        ("Artistic creativity involves pattern recognition and innovation", "art"),
    ];
    
    let mut curiosity_scores = Vec::new();
    
    for (i, (input, domain)) in test_inputs.iter().enumerate() {
        println!("🧪 Test {}: {} Domain", i + 1, domain.to_uppercase());
        
        let context = NoveltyContext {
            task_context: domain.to_string(),
            recent_inputs: vec![input.to_string()],
            temporal_context: Utc::now(),
            metadata: {
                let mut map = HashMap::new();
                map.insert("domain".to_string(), domain.to_string());
                map.insert("test_id".to_string(), (i + 1).to_string());
                map
            },
            ..Default::default()
        };
        
        let curiosity_score = curiosity_engine.assess_curiosity(input, Some(context))?;
        curiosity_scores.push(curiosity_score);
        
        println!("   Input: \"{}\"", input);
        println!("   Curiosity Score: {:.3}", curiosity_score);
        
        if curiosity_score >= curiosity_config.learning_threshold {
            println!("   🎯 Learning priority created!");
        } else {
            println!("   ℹ️  Below learning threshold");
        }
        println!();
    }
    
    let avg_curiosity = curiosity_scores.iter().sum::<f64>() / curiosity_scores.len() as f64;
    println!("📊 Average curiosity score across all tests: {:.3}", avg_curiosity);
    println!();

    // Phase 4: Learning Priorities Analysis
    println!("🎯 Phase 4: Learning Priorities Analysis");
    println!("----------------------------------------");
    
    let top_priorities = curiosity_engine.get_top_learning_priorities(5);
    println!("🏆 Top {} Learning Priorities:", top_priorities.len());
    
    for (i, priority) in top_priorities.iter().enumerate() {
        println!("   {}. ID: {}", i + 1, priority.id);
        println!("      Content: \"{}\"", priority.content);
        println!("      Curiosity Score: {:.3}", priority.curiosity_score);
        println!("      Primary Drive: {}", priority.primary_drive);
        println!("      Expected Value: {:.3}", priority.expected_value);
        println!("      Knowledge Gaps: {}", priority.knowledge_gaps.len());
        
        if let Some(novelty) = &priority.novelty_assessment {
            println!("      Novelty Score: {:.3}", novelty.novelty_score);
        }
        
        println!("      Drive Breakdown:");
        for (drive, score) in &priority.drive_scores {
            println!("        • {}: {:.3}", drive, score);
        }
        println!();
    }

    // Phase 5: Simulate Learning Events
    println!("📚 Phase 5: Simulating Learning Events");
    println!("--------------------------------------");
    
    simulate_learning_events(&mut curiosity_engine)?;
    
    let updated_stats = curiosity_engine.get_stats();
    println!("📈 Updated curiosity system statistics:");
    println!("   • Total priorities: {}", updated_stats.total_priorities);
    println!("   • Active priorities: {}", updated_stats.active_priorities);
    println!("   • Completed priorities: {}", updated_stats.completed_priorities);
    println!("   • Overall success rate: {:.3}", updated_stats.overall_success_rate);
    println!("   • Average progress: {:.3}", updated_stats.average_progress);
    println!();
    
    println!("🎭 Drive Distribution:");
    for (drive, count) in &updated_stats.drive_distribution {
        println!("   • {}: {} priorities", drive, count);
    }
    println!();

    // Phase 6: Interest Model Analysis
    println!("🧭 Phase 6: Interest Model Analysis");
    println!("-----------------------------------");
    
    let interest_model = curiosity_engine.get_interest_model();
    println!("🎨 Learning Preferences (Drive Preferences):");
    for (drive, preference) in &interest_model.drive_preferences {
        println!("   • {}: {:.3}", drive, preference);
    }
    println!();
    
    println!("📊 Knowledge Type Success Rates:");
    for (knowledge_type, success_rate) in &interest_model.type_success_rates {
        println!("   • {}: {:.3}", knowledge_type, success_rate);
    }
    println!();
    
    println!("🕒 Recent Learning Events: {}", interest_model.recent_learning.len());
    println!("📅 Last Model Update: {}", interest_model.last_updated.format("%Y-%m-%d %H:%M:%S"));
    println!();

    // Phase 7: Adaptive Behavior Demonstration
    println!("🔄 Phase 7: Adaptive Behavior Demonstration");
    println!("--------------------------------------------");
    
    demonstrate_adaptive_behavior(&mut curiosity_engine)?;
    println!();

    // Phase 8: System Integration Summary
    println!("🔗 Phase 8: System Integration Summary");
    println!("--------------------------------------");
    
    print_integration_summary(&curiosity_engine, &meta_memory, &novelty_engine)?;
    
    println!("🎉 Curiosity-Driven Learning Demo Complete!");
    println!("============================================");
    println!();
    println!("The curiosity-driven learning system has successfully demonstrated:");
    println!("✅ Integration with meta-memory for knowledge gap detection");
    println!("✅ Integration with novelty detection for interesting input identification");
    println!("✅ Intelligent learning priority creation and management");
    println!("✅ Adaptive exploration strategies based on success patterns");
    println!("✅ Interest modeling and preference learning");
    println!("✅ Comprehensive learning event tracking and analysis");
    println!("✅ Production-ready architecture with persistence capabilities");
    
    Ok(())
}

/// Populate meta-memory with sample knowledge components
fn populate_sample_knowledge(meta_memory: &Arc<Mutex<MetaMemorySystem>>) -> Result<()> {
    let knowledge_items = vec![
        (KnowledgeType::ConceptNode, "quantum_physics", 0.85, "physics_textbook"),
        (KnowledgeType::ConceptNode, "machine_learning", 0.72, "ai_research"),
        (KnowledgeType::Rule, "conservation_of_energy", 0.95, "physics_laws"),
        (KnowledgeType::Rule, "gradient_descent", 0.68, "optimization_theory"),
        (KnowledgeType::Segment, "fractal_pattern", 0.45, "pattern_recognition"),
        (KnowledgeType::Segment, "network_topology", 0.38, "graph_theory"),
        (KnowledgeType::EpisodicMemory, "learning_experience_1", 0.60, "educational_session"),
        (KnowledgeType::EpisodicMemory, "problem_solving_event", 0.55, "cognitive_task"),
        (KnowledgeType::SemanticConcept, "emergence", 0.40, "complexity_science"),
        (KnowledgeType::SemanticConcept, "consciousness", 0.25, "neuroscience_research"),
        (KnowledgeType::Pattern, "power_law_distribution", 0.78, "statistical_analysis"),
        (KnowledgeType::Pattern, "small_world_network", 0.52, "network_science"),
    ];
    
    let mut meta_memory_guard = meta_memory.lock().unwrap();
    
    for (knowledge_type, _concept, confidence, source) in knowledge_items {
        let item = MetaMemoryItem::new(
            Uuid::new_v4(),
            knowledge_type,
            confidence,
            source.to_string(),
        );
        meta_memory_guard.store_item(item)?;
    }
    
    // Simulate some validation events to create realistic confidence patterns
    let items = meta_memory_guard.query_items(&Default::default())?;
    for item in items.iter().take(6) {
        // Simulate successful validations for some items
        meta_memory_guard.update_confidence(item.component_id, true)?;
        meta_memory_guard.update_confidence(item.component_id, true)?;
        if item.confidence_score > 0.6 {
            meta_memory_guard.update_confidence(item.component_id, true)?;
        } else {
            meta_memory_guard.update_confidence(item.component_id, false)?;
        }
    }
    
    meta_memory_guard.update_stats()?;
    Ok(())
}

/// Simulate learning events to demonstrate system adaptation
fn simulate_learning_events(curiosity_engine: &mut CuriosityLearningEngine) -> Result<()> {
    // Collect priority data first to avoid borrow checker issues
    let priority_data: Vec<_> = curiosity_engine.get_top_learning_priorities(3)
        .iter()
        .map(|p| (p.id, p.content.clone(), p.primary_drive.clone(), p.curiosity_score))
        .collect();
    
    for (priority_id, content, primary_drive, curiosity_score) in priority_data {
        println!("🎓 Simulating learning event for priority: {}", priority_id);
        println!("   Content: \"{}\"", content);
        
        // Create learning event
        let mut event = LearningEvent::new(
            priority_id,
            content.clone(),
            primary_drive.clone(),
            KnowledgeType::ConceptNode,
        );
        
        // Simulate learning outcomes based on curiosity score
        let success_probability = (curiosity_score * 0.8) + 0.2; // 20-100% based on curiosity
        let success = rand::random::<f64>() < success_probability;
        
        event.success = success;
        event.progress_gained = if success { 
            0.3 + (rand::random::<f64>() * 0.4) // 30-70% progress
        } else { 
            0.1 + (rand::random::<f64>() * 0.2) // 10-30% progress
        };
        event.duration_minutes = 15.0 + (rand::random::<f64>() * 30.0); // 15-45 minutes
        event.satisfaction = if success { 
            0.6 + (rand::random::<f64>() * 0.4) // 60-100% satisfaction
        } else { 
            0.2 + (rand::random::<f64>() * 0.4) // 20-60% satisfaction
        };
        
        // Store values before moving the event
        let progress_gained = event.progress_gained;
        let duration_minutes = event.duration_minutes;
        let satisfaction = event.satisfaction;
        
        curiosity_engine.record_learning_event(event)?;
        
        println!("   ✅ Success: {}", if success { "Yes" } else { "No" });
        println!("   📈 Progress Gained: {:.1}%", progress_gained * 100.0);
        println!("   ⏱️  Duration: {:.1} minutes", duration_minutes);
        println!("   😊 Satisfaction: {:.1}%", satisfaction * 100.0);
        println!();
    }
    
    Ok(())
}

/// Demonstrate adaptive behavior based on learning outcomes
fn demonstrate_adaptive_behavior(curiosity_engine: &mut CuriosityLearningEngine) -> Result<()> {
    println!("🤖 Testing adaptive behavior with follow-up assessments...");
    
    // Test similar content to see how the system adapts
    let follow_up_inputs = vec![
        "Quantum entanglement creates spooky action at a distance",
        "Deep neural networks exhibit representational learning",
        "Economic bubbles follow predictable boom-bust cycles",
    ];
    
    for (i, input) in follow_up_inputs.iter().enumerate() {
        println!("   Follow-up Test {}: \"{}\"", i + 1, input);
        
        let curiosity_score = curiosity_engine.assess_curiosity(input, None)?;
        println!("   Curiosity Score: {:.3}", curiosity_score);
        
        // The system should show different curiosity levels based on:
        // 1. Previous learning success in similar domains
        // 2. Interest model preferences
        // 3. Knowledge gap status
        
        if curiosity_score > 0.5 {
            println!("   🔥 High curiosity - system wants to explore this further!");
        } else if curiosity_score > 0.3 {
            println!("   🤔 Moderate curiosity - might be worth investigating");
        } else {
            println!("   😴 Low curiosity - system has sufficient knowledge or low interest");
        }
        println!();
    }
    
    Ok(())
}

/// Print comprehensive integration summary
fn print_integration_summary(
    curiosity_engine: &CuriosityLearningEngine,
    meta_memory: &Arc<Mutex<MetaMemorySystem>>,
    novelty_engine: &Arc<Mutex<NoveltyDetectionEngine>>,
) -> Result<()> {
    println!("🔍 Integration Status Summary:");
    println!();
    
    // Meta-memory integration
    let meta_stats = {
        let meta_memory_guard = meta_memory.lock().unwrap();
        meta_memory_guard.get_stats().clone()
    };
    println!("🧠 Meta-Memory Integration:");
    println!("   • Components tracked: {}", meta_stats.total_components);
    println!("   • Knowledge gaps identified: {}", meta_stats.low_confidence_count);
    println!("   • Confidence-based curiosity: Active");
    println!();
    
    // Novelty detection integration
    let novelty_stats = {
        let novelty_engine_guard = novelty_engine.lock().unwrap();
        novelty_engine_guard.get_stats().clone()
    };
    println!("✨ Novelty Detection Integration:");
    println!("   • Novelty assessments: {}", novelty_stats.total_assessments);
    println!("   • Average novelty score: {:.3}", novelty_stats.average_novelty_score);
    println!("   • Novelty-based curiosity: Active");
    println!();
    
    // Curiosity system status
    let curiosity_stats = curiosity_engine.get_stats();
    println!("🎯 Curiosity System Status:");
    println!("   • Learning priorities: {}", curiosity_stats.total_priorities);
    println!("   • Active explorations: {}", curiosity_stats.active_priorities);
    println!("   • Learning success rate: {:.3}", curiosity_stats.overall_success_rate);
    println!("   • Adaptive behavior: Enabled");
    println!();
    
    // Configuration summary
    let config = curiosity_engine.get_config();
    println!("⚙️  System Configuration:");
    println!("   • Novelty weight: {:.1}%", config.novelty_weight * 100.0);
    println!("   • Uncertainty weight: {:.1}%", config.uncertainty_weight * 100.0);
    println!("   • Progress weight: {:.1}%", config.progress_weight * 100.0);
    println!("   • Exploration rate: {:.1}%", config.exploration_rate * 100.0);
    println!("   • Learning threshold: {:.3}", config.learning_threshold);
    println!();
    
    Ok(())
} 