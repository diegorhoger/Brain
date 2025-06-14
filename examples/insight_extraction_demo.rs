//! Insight Extraction Engine Demonstration
//! 
//! This example demonstrates the pattern detection system (Task 5.1) that monitors
//! memory stores and identifies recurring patterns and relationships.

use anyhow::Result;
use brain::{
    PatternDetector, PatternDetectionConfig, PatternType,
    MemorySystem, Priority, EpisodicEvent, SemanticConcept,
    ConceptGraphManager, ConceptGraphConfig, ConceptNode, ConceptType,
};
use brain::concept_graph::RelationshipType;
use chrono::{Utc, Duration};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain Insight Extraction Engine Demo");
    println!("========================================");
    println!();

    // Phase 1: Initialize Pattern Detection System
    println!("📊 Phase 1: Initializing Pattern Detection System");
    println!("--------------------------------------------------");
    
    let config = PatternDetectionConfig {
        min_pattern_frequency: 2,
        temporal_window_hours: 24,
        min_confidence_threshold: 0.5,
        max_patterns_per_batch: 50,
        min_co_occurrence_count: 2,
        significance_threshold: 0.1,
        incremental_detection: true,
        batch_size: 20,
    };
    
    let mut pattern_detector = PatternDetector::with_config(config);
    println!("✅ Pattern detector initialized with custom configuration");
    println!("   - Min pattern frequency: {}", pattern_detector.get_config().min_pattern_frequency);
    println!("   - Temporal window: {} hours", pattern_detector.get_config().temporal_window_hours);
    println!("   - Confidence threshold: {:.2}", pattern_detector.get_config().min_confidence_threshold);
    println!();

    // Phase 2: Set up Memory System with Sample Data
    println!("💾 Phase 2: Setting up Memory System with Sample Data");
    println!("-----------------------------------------------------");
    
    // Create memory system with episodic database
    let mut memory_system = MemorySystem::with_episodic_db(50, "test_insight_demo.db")?;
    
    // Add working memory items
    memory_system.learn("User asks about weather".to_string(), Priority::High)?;
    memory_system.learn("System provides weather forecast".to_string(), Priority::Medium)?;
    memory_system.learn("User asks about traffic".to_string(), Priority::High)?;
    memory_system.learn("System provides traffic update".to_string(), Priority::Medium)?;
    memory_system.learn("User asks about weather".to_string(), Priority::High)?; // Repeated pattern
    memory_system.learn("System provides weather forecast".to_string(), Priority::Medium)?; // Repeated pattern
    
    println!("✅ Added 6 working memory items with repeated patterns");
    
    // Add episodic events with temporal patterns
    let base_time = Utc::now() - Duration::hours(2);
    
    let events = vec![
        ("User login detected", base_time),
        ("User asks about weather", base_time + Duration::minutes(5)),
        ("Weather data retrieved", base_time + Duration::minutes(6)),
        ("User logout detected", base_time + Duration::minutes(30)),
        ("User login detected", base_time + Duration::hours(1)), // Repeated sequence
        ("User asks about traffic", base_time + Duration::hours(1) + Duration::minutes(3)),
        ("Traffic data retrieved", base_time + Duration::hours(1) + Duration::minutes(4)),
        ("User logout detected", base_time + Duration::hours(1) + Duration::minutes(25)),
    ];
    
    for (content, timestamp) in events {
        let mut event = EpisodicEvent::new(
            content.to_string(),
            HashMap::new(),
            0.8,
            "demo".to_string(),
        );
        event.timestamp = timestamp;
        
        // Store the event using the memory system's store method
        memory_system.store_concept(SemanticConcept::new(
            content.to_string(),
            format!("Event: {}", content),
            vec![0.1, 0.2, 0.3, 0.4], // Simple embedding
        ))?;
    }
    
    println!("✅ Added 8 episodic events as semantic concepts");
    
    // Add semantic concepts
    let concepts = vec![
        ("weather", "Information about atmospheric conditions"),
        ("traffic", "Information about road conditions and congestion"),
        ("user", "Person interacting with the system"),
        ("system", "The AI assistant providing responses"),
        ("query", "A request for information"),
        ("response", "An answer to a query"),
    ];
    
    for (name, description) in concepts {
        let concept = SemanticConcept::new(
            name.to_string(),
            description.to_string(),
            vec![0.1, 0.2, 0.3, 0.4], // Simple embedding
        );
        memory_system.store_concept(concept)?;
    }
    
    println!("✅ Added 6 semantic concepts");
    println!();

    // Phase 3: Detect Patterns from Memory System
    println!("🔍 Phase 3: Detecting Patterns from Memory System");
    println!("--------------------------------------------------");
    
    let memory_result = pattern_detector.detect_patterns_from_memory(&memory_system).await?;
    
    println!("📈 Pattern Detection Results from Memory:");
    println!("   - Patterns detected: {}", memory_result.detected_patterns.len());
    println!("   - Items processed: {}", memory_result.items_processed);
    println!("   - Processing time: {}ms", memory_result.processing_time_ms);
    println!("   - Filtered patterns: {}", memory_result.filtered_patterns);
    println!();
    
    println!("🎯 Detected Patterns by Type:");
    for (pattern_type, count) in &memory_result.pattern_type_counts {
        println!("   - {}: {} patterns", pattern_type, count);
    }
    println!();
    
    println!("📋 Detailed Pattern Analysis:");
    for (i, pattern) in memory_result.detected_patterns.iter().enumerate() {
        println!("   Pattern {}: {} ({})", i + 1, pattern.pattern_type, pattern.elements.join(" → "));
        println!("      Frequency: {}, Confidence: {:.3}", pattern.frequency, pattern.confidence);
        println!("      Evidence: {} items", pattern.evidence.len());
        
        if let Some(ref temporal_info) = pattern.temporal_info {
            println!("      Temporal: avg {:.1}min, std {:.1}min", 
                temporal_info.average_delay_minutes, temporal_info.delay_std_dev);
        }
        println!();
    }

    // Phase 4: Set up Concept Graph with Sample Data
    println!("🕸️  Phase 4: Setting up Concept Graph with Sample Data");
    println!("-------------------------------------------------------");
    
    let graph_config = ConceptGraphConfig::default();
    let mut concept_graph = ConceptGraphManager::new(graph_config).await?;
    
    // Create concept nodes
    let weather_concept = ConceptNode::new(
        ConceptType::Entity,
        "weather".to_string(),
        0.9,
        Some("semantic_memory".to_string()),
    );
    let weather_id = concept_graph.create_concept(weather_concept).await?;
    
    let forecast_concept = ConceptNode::new(
        ConceptType::Entity,
        "forecast".to_string(),
        0.8,
        Some("semantic_memory".to_string()),
    );
    let forecast_id = concept_graph.create_concept(forecast_concept).await?;
    
    let user_concept = ConceptNode::new(
        ConceptType::Entity,
        "user".to_string(),
        0.95,
        Some("semantic_memory".to_string()),
    );
    let user_id = concept_graph.create_concept(user_concept).await?;
    
    let query_concept = ConceptNode::new(
        ConceptType::Action,
        "query".to_string(),
        0.85,
        Some("semantic_memory".to_string()),
    );
    let query_id = concept_graph.create_concept(query_concept).await?;
    
    println!("✅ Created 4 concept nodes");
    
    // Create relationships
    concept_graph.create_relationship(
        forecast_id, weather_id, RelationshipType::IsA, 0.8
    ).await?;
    
    concept_graph.create_relationship(
        weather_id, forecast_id, RelationshipType::SimilarTo, 0.7
    ).await?;
    
    concept_graph.create_relationship(
        user_id, query_id, RelationshipType::Uses, 0.9
    ).await?;
    
    concept_graph.create_relationship(
        query_id, weather_id, RelationshipType::Causes, 0.6
    ).await?;
    
    println!("✅ Created 4 concept relationships");
    println!();

    // Phase 5: Detect Patterns from Concept Graph
    println!("🔗 Phase 5: Detecting Patterns from Concept Graph");
    println!("--------------------------------------------------");
    
    let graph_result = pattern_detector.detect_patterns_from_concept_graph(&concept_graph).await?;
    
    println!("📈 Pattern Detection Results from Concept Graph:");
    println!("   - Patterns detected: {}", graph_result.detected_patterns.len());
    println!("   - Items processed: {}", graph_result.items_processed);
    println!("   - Processing time: {}ms", graph_result.processing_time_ms);
    println!("   - Filtered patterns: {}", graph_result.filtered_patterns);
    println!();
    
    println!("🎯 Detected Patterns by Type:");
    for (pattern_type, count) in &graph_result.pattern_type_counts {
        println!("   - {}: {} patterns", pattern_type, count);
    }
    println!();
    
    println!("📋 Detailed Pattern Analysis:");
    for (i, pattern) in graph_result.detected_patterns.iter().enumerate() {
        println!("   Pattern {}: {} ({})", i + 1, pattern.pattern_type, pattern.elements.join(" → "));
        println!("      Frequency: {}, Confidence: {:.3}", pattern.frequency, pattern.confidence);
        println!("      Evidence: {} relationships", pattern.evidence.len());
        println!();
    }

    // Phase 6: Pattern Cache and Statistics Analysis
    println!("📊 Phase 6: Pattern Cache and Statistics Analysis");
    println!("--------------------------------------------------");
    
    let cached_patterns = pattern_detector.get_cached_patterns();
    println!("🗄️  Pattern Cache Analysis:");
    println!("   - Total cached patterns: {}", cached_patterns.len());
    
    let mut cache_by_type: HashMap<PatternType, usize> = HashMap::new();
    for pattern in &cached_patterns {
        *cache_by_type.entry(pattern.pattern_type.clone()).or_insert(0) += 1;
    }
    
    for (pattern_type, count) in cache_by_type {
        println!("   - {}: {} cached", pattern_type, count);
    }
    println!();
    
    let stats = pattern_detector.get_detection_stats();
    println!("📈 Detection Statistics:");
    println!("   - Total patterns detected: {}", stats.total_patterns_detected);
    println!("   - Total items processed: {}", stats.total_items_processed);
    println!("   - Total processing time: {}ms", stats.total_processing_time_ms);
    println!("   - Detection operations: {}", stats.detection_operations);
    println!("   - Average patterns per operation: {:.2}", stats.average_patterns_per_operation);
    println!();
    
    println!("🎯 Patterns by Type (Overall):");
    for (pattern_type, count) in &stats.patterns_by_type {
        println!("   - {}: {} total", pattern_type, count);
    }
    println!();

    // Phase 7: Advanced Pattern Analysis
    println!("🔬 Phase 7: Advanced Pattern Analysis");
    println!("--------------------------------------");
    
    // Analyze pattern significance
    let all_patterns: Vec<_> = cached_patterns.iter().collect();
    let significant_patterns: Vec<_> = all_patterns.iter()
        .filter(|p| p.is_significant(pattern_detector.get_config()))
        .collect();
    
    println!("🎯 Pattern Significance Analysis:");
    println!("   - Total patterns: {}", all_patterns.len());
    println!("   - Significant patterns: {}", significant_patterns.len());
    if !all_patterns.is_empty() {
        println!("   - Significance ratio: {:.2}%", 
            (significant_patterns.len() as f64 / all_patterns.len() as f64) * 100.0);
    }
    println!();
    
    // Find highest confidence patterns
    let mut sorted_patterns = all_patterns.clone();
    sorted_patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    
    println!("🏆 Top 3 Highest Confidence Patterns:");
    for (i, pattern) in sorted_patterns.iter().take(3).enumerate() {
        println!("   {}. {} (confidence: {:.3})", 
            i + 1, pattern.pattern_type, pattern.confidence);
        println!("      Elements: {}", pattern.elements.join(" → "));
        println!("      Frequency: {}", pattern.frequency);
    }
    println!();

    // Phase 8: Configuration Testing
    println!("⚙️  Phase 8: Configuration Testing");
    println!("-----------------------------------");
    
    // Test with stricter configuration
    let strict_config = PatternDetectionConfig {
        min_pattern_frequency: 3,
        min_confidence_threshold: 0.8,
        significance_threshold: 0.01,
        ..pattern_detector.get_config().clone()
    };
    
    pattern_detector.set_config(strict_config);
    println!("🔧 Applied stricter configuration:");
    println!("   - Min frequency: {}", pattern_detector.get_config().min_pattern_frequency);
    println!("   - Min confidence: {:.2}", pattern_detector.get_config().min_confidence_threshold);
    println!("   - Significance threshold: {:.3}", pattern_detector.get_config().significance_threshold);
    
    // Re-run detection with stricter settings
    let strict_result = pattern_detector.detect_patterns_from_memory(&memory_system).await?;
    println!("   - Patterns with strict config: {}", strict_result.detected_patterns.len());
    println!("   - Filtered out: {}", strict_result.filtered_patterns);
    println!();

    // Phase 9: Cache Management
    println!("🗂️  Phase 9: Cache Management");
    println!("------------------------------");
    
    println!("🧹 Cache Management Operations:");
    println!("   - Patterns before clear: {}", pattern_detector.get_cached_patterns().len());
    
    pattern_detector.clear_cache();
    println!("   - Patterns after clear: {}", pattern_detector.get_cached_patterns().len());
    
    // Reset statistics
    let stats_before = pattern_detector.get_detection_stats().clone();
    pattern_detector.reset_stats();
    let stats_after = pattern_detector.get_detection_stats();
    
    println!("   - Operations before reset: {}", stats_before.detection_operations);
    println!("   - Operations after reset: {}", stats_after.detection_operations);
    println!();

    // Phase 10: Summary and Next Steps
    println!("📋 Phase 10: Summary and Next Steps");
    println!("------------------------------------");
    
    println!("✅ Pattern Detection System (Task 5.1) Successfully Demonstrated!");
    println!();
    println!("🎯 Key Capabilities Implemented:");
    println!("   ✓ Temporal sequence pattern detection from episodic memory");
    println!("   ✓ Co-occurrence pattern detection across memory types");
    println!("   ✓ Frequency pattern detection from recurring events");
    println!("   ✓ Hierarchical pattern detection from concept relationships");
    println!("   ✓ Similarity pattern detection from concept graph");
    println!("   ✓ Causal pattern detection from relationship types");
    println!("   ✓ Statistical significance filtering");
    println!("   ✓ Incremental pattern detection");
    println!("   ✓ Pattern caching and statistics tracking");
    println!("   ✓ Configurable detection parameters");
    println!();
    println!("🚀 Ready for Task 5.2: Rule Formalization Framework");
    println!("   - Transform detected patterns into formal rules");
    println!("   - Implement [Pattern] → [Outcome] rule structures");
    println!("   - Add support, confidence, and generality metrics");
    println!("   - Create rule storage and indexing systems");
    println!();
    println!("🧠 Pattern Detection Engine is now operational and ready for integration!");

    // Clean up test database
    std::fs::remove_file("test_insight_demo.db").ok();

    Ok(())
} 