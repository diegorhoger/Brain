//! Meta-Memory System Demonstration
//! 
//! This example demonstrates the core capabilities of Task 9.1:
//! - Meta-memory structure with confidence tracking
//! - Unified tracking across different knowledge types
//! - Confidence updates based on validation outcomes
//! - Querying by confidence levels and knowledge types
//! - Analytics for knowledge quality assessment

use anyhow::Result;
use uuid::Uuid;
use brain::{
    MetaMemorySystem, MetaMemoryItem, MetaMemoryConfig, MetaMemoryQuery,
    KnowledgeType
};

fn main() -> Result<()> {
    println!("🧠 Meta-Memory System Demonstration - Task 9.1");
    println!("{}", "=".repeat(60));

    // Phase 1: Initialize Meta-Memory System
    println!("\n📊 Phase 1: Initialize Meta-Memory System");
    println!("{}", "-".repeat(40));
    
    let config = MetaMemoryConfig {
        database_path: "meta_memory_demo.db".to_string(),
        high_confidence_threshold: 0.8,
        low_confidence_threshold: 0.3,
        min_validation_count: 3,
        ..Default::default()
    };
    
    let mut meta_memory = MetaMemorySystem::with_config(config)?;
    println!("✅ Meta-memory system initialized with SQLite persistence");
    
    // Phase 2: Create Knowledge Components with Different Types
    println!("\n🏗️  Phase 2: Create Knowledge Components");
    println!("{}", "-".repeat(40));
    
    let mut components = Vec::new();
    
    // Create various knowledge components
    let knowledge_samples = [
        (KnowledgeType::Segment, 0.7, "BPE discovered frequent pattern 'th'"),
        (KnowledgeType::ConceptNode, 0.8, "Graph node representing 'animal' concept"),
        (KnowledgeType::Rule, 0.6, "If weather=rain then carry=umbrella"),
        (KnowledgeType::SemanticConcept, 0.9, "High-level concept 'transportation'"),
        (KnowledgeType::WorkingMemory, 0.5, "Active task information"),
        (KnowledgeType::EpisodicMemory, 0.7, "User visited park yesterday"),
        (KnowledgeType::Pattern, 0.4, "Temporal pattern: morning->coffee"),
        (KnowledgeType::ConceptRelationship, 0.6, "Cat IS_A Animal relationship"),
    ];
    
    for (knowledge_type, initial_confidence, description) in knowledge_samples.iter() {
        let component_id = Uuid::new_v4();
        let mut item = MetaMemoryItem::new(
            component_id,
            knowledge_type.clone(),
            *initial_confidence,
            description.to_string(),
        );
        
        // Add some metadata
        item.set_metadata("description".to_string(), description.to_string());
        item.set_metadata("created_by".to_string(), "demo_system".to_string());
        
        let meta_id = meta_memory.store_item(item)?;
        components.push((component_id, meta_id, knowledge_type.clone()));
        
        println!("📝 Created {} component: {}", knowledge_type, description);
    }
    
    // Phase 3: Demonstrate Confidence Updates
    println!("\n🎯 Phase 3: Confidence Updates & Validation");
    println!("{}", "-".repeat(40));
    
    println!("Simulating validation outcomes for knowledge components...\n");
    
    // Simulate various validation scenarios
    let validation_scenarios = [
        (0, true, "Segment 'th' successfully used in prediction"),
        (0, true, "Segment confirmed by frequency analysis"),
        (0, false, "Segment failed in specific context"),
        (1, true, "Concept node correctly retrieved"),
        (1, true, "Concept relationships validated"),
        (2, false, "Rule failed: umbrella not needed indoors"),
        (2, true, "Rule successful: umbrella used outside"),
        (2, true, "Rule pattern confirmed by user behavior"),
        (3, true, "Semantic concept correctly categorized"),
        (4, false, "Working memory item became irrelevant"),
        (5, true, "Episodic memory accurately recalled"),
        (6, false, "Temporal pattern broken by user behavior"),
        (7, true, "Concept relationship logically consistent"),
    ];
    
    for (component_idx, success, description) in validation_scenarios.iter() {
        if *component_idx < components.len() {
            let (component_id, _, knowledge_type) = &components[*component_idx];
            meta_memory.update_confidence(*component_id, *success)?;
            
            let status = if *success { "✅ SUCCESS" } else { "❌ FAILURE" };
            println!("{} {}: {}", status, knowledge_type, description);
        }
    }
    
    // Phase 4: Access Tracking
    println!("\n👆 Phase 4: Usage and Access Tracking");
    println!("{}", "-".repeat(40));
    
    // Simulate usage of different components
    for (i, (component_id, _, knowledge_type)) in components.iter().enumerate() {
        // Simulate different usage frequencies
        let access_count = match i % 3 {
            0 => 5, // High usage
            1 => 2, // Medium usage
            _ => 1, // Low usage
        };
        
        for _ in 0..access_count {
            meta_memory.mark_accessed(*component_id)?;
        }
        
        println!("📊 {} accessed {} times", knowledge_type, access_count);
    }
    
    // Phase 5: Query and Analysis
    println!("\n🔍 Phase 5: Query and Analysis");
    println!("{}", "-".repeat(40));
    
    // Query high-confidence components
    println!("\n🏆 High-Confidence Components (>= 0.8):");
    let high_confidence = meta_memory.get_high_confidence_components()?;
    for item in &high_confidence {
        println!("  • {} [{}]: {:.3} confidence ({} validations, {:.1}% success)",
            item.knowledge_type,
            item.source,
            item.confidence_score,
            item.validation_count,
            item.success_rate() * 100.0
        );
    }
    
    // Query low-confidence components
    println!("\n⚠️  Low-Confidence Components (< 0.3):");
    let low_confidence = meta_memory.get_low_confidence_components()?;
    for item in &low_confidence {
        println!("  • {} [{}]: {:.3} confidence ({} validations, {:.1}% success)",
            item.knowledge_type,
            item.source,
            item.confidence_score,
            item.validation_count,
            item.success_rate() * 100.0
        );
    }
    
    // Custom query examples
    println!("\n🎯 Custom Query Examples:");
    
    // Query by knowledge type
    let concept_query = MetaMemoryQuery {
        knowledge_type: Some(KnowledgeType::ConceptNode),
        ..Default::default()
    };
    let concept_items = meta_memory.query_items(&concept_query)?;
    println!("  • Found {} ConceptNode items", concept_items.len());
    
    // Query by confidence range
    let medium_confidence_query = MetaMemoryQuery {
        min_confidence: Some(0.4),
        max_confidence: Some(0.7),
        sort_by: Some("confidence_score".to_string()),
        descending: true,
        ..Default::default()
    };
    let medium_items = meta_memory.query_items(&medium_confidence_query)?;
    println!("  • Found {} medium-confidence items (0.4-0.7)", medium_items.len());
    
    // Query by usage count
    let high_usage_query = MetaMemoryQuery {
        min_usage_count: Some(3),
        sort_by: Some("usage_count".to_string()),
        descending: true,
        ..Default::default()
    };
    let high_usage_items = meta_memory.query_items(&high_usage_query)?;
    println!("  • Found {} frequently used items (>= 3 accesses)", high_usage_items.len());
    
    // Phase 6: System Analytics
    println!("\n📈 Phase 6: System Analytics & Quality Metrics");
    println!("{}", "-".repeat(40));
    
    let stats = meta_memory.get_stats();
    println!("\n📊 Overall Meta-Memory Statistics:");
    println!("  • Total Knowledge Components: {}", stats.total_components);
    println!("  • Average Confidence Score: {:.3}", stats.average_confidence);
    println!("  • High-Confidence Count: {} ({:.1}%)", 
        stats.high_confidence_count,
        stats.high_confidence_count as f64 / stats.total_components as f64 * 100.0
    );
    println!("  • Low-Confidence Count: {} ({:.1}%)", 
        stats.low_confidence_count,
        stats.low_confidence_count as f64 / stats.total_components as f64 * 100.0
    );
    println!("  • Total Validations: {}", stats.total_validations);
    println!("  • Overall Success Rate: {:.1}%", stats.overall_success_rate * 100.0);
    
    // Components by type breakdown
    println!("\n🗂️  Knowledge Components by Type:");
    for (knowledge_type, count) in &stats.components_by_type {
        println!("  • {}: {} components", knowledge_type, count);
    }
    
    // Confidence distribution
    println!("\n📊 Confidence Distribution:");
    let distribution = meta_memory.get_confidence_distribution()?;
    for (range, count) in &distribution {
        let percentage = *count as f64 / stats.total_components as f64 * 100.0;
        println!("  • {}: {} components ({:.1}%)", range, count, percentage);
    }
    
    // Phase 7: Knowledge Quality Assessment
    println!("\n🎯 Phase 7: Knowledge Quality Assessment");
    println!("{}", "-".repeat(40));
    
    let quality_score = calculate_knowledge_quality_score(&stats);
    println!("\n⭐ Overall Knowledge Quality Score: {:.2}/10.0", quality_score);
    
    let quality_analysis = analyze_knowledge_quality(&stats);
    println!("\n📋 Quality Analysis:");
    for insight in quality_analysis {
        println!("  • {}", insight);
    }
    
    // Phase 8: Recommendations
    println!("\n💡 Phase 8: Improvement Recommendations");
    println!("{}", "-".repeat(40));
    
    let recommendations = generate_recommendations(&stats, &meta_memory)?;
    println!("\n🔧 System Recommendations:");
    for rec in recommendations {
        println!("  • {}", rec);
    }
    
    println!("\n✨ Meta-Memory System demonstration completed successfully!");
    println!("🎉 Task 9.1 - Meta-Memory Structure with Confidence Tracking - COMPLETE");
    
    Ok(())
}

/// Calculate an overall knowledge quality score (0-10)
fn calculate_knowledge_quality_score(stats: &brain::MetaMemoryStats) -> f64 {
    if stats.total_components == 0 {
        return 0.0;
    }
    
    // Factors contributing to quality score
    let confidence_factor = stats.average_confidence * 3.0; // 0-3 points
    let coverage_factor = (stats.total_components.min(100) as f64 / 100.0) * 2.0; // 0-2 points
    let reliability_factor = if stats.total_validations > 0 {
        stats.overall_success_rate * 2.0 // 0-2 points
    } else {
        0.0
    };
    let diversity_factor = (stats.components_by_type.len().min(9) as f64 / 9.0) * 2.0; // 0-2 points
    let balance_factor = if stats.high_confidence_count > 0 && stats.low_confidence_count < stats.total_components / 2 {
        1.0 // 1 point for good balance
    } else {
        0.0
    };
    
    confidence_factor + coverage_factor + reliability_factor + diversity_factor + balance_factor
}

/// Analyze knowledge quality and provide insights
fn analyze_knowledge_quality(stats: &brain::MetaMemoryStats) -> Vec<String> {
    let mut insights = Vec::new();
    
    if stats.average_confidence > 0.7 {
        insights.push("Strong overall confidence in knowledge components".to_string());
    } else if stats.average_confidence < 0.4 {
        insights.push("Low average confidence - need more validation".to_string());
    }
    
    if stats.high_confidence_count > stats.total_components / 2 {
        insights.push("Good proportion of high-confidence knowledge".to_string());
    }
    
    if stats.low_confidence_count > stats.total_components / 3 {
        insights.push("High number of low-confidence components - consider cleanup".to_string());
    }
    
    if stats.total_validations == 0 {
        insights.push("No validations performed - confidence scores are initial estimates".to_string());
    } else if stats.overall_success_rate > 0.8 {
        insights.push("Excellent validation success rate".to_string());
    } else if stats.overall_success_rate < 0.5 {
        insights.push("Low validation success rate - review knowledge accuracy".to_string());
    }
    
    if stats.components_by_type.len() >= 6 {
        insights.push("Good diversity across knowledge types".to_string());
    } else {
        insights.push("Limited knowledge type diversity".to_string());
    }
    
    insights
}

/// Generate improvement recommendations
fn generate_recommendations(stats: &brain::MetaMemoryStats, meta_memory: &MetaMemorySystem) -> Result<Vec<String>> {
    let mut recommendations = Vec::new();
    
    if stats.average_confidence < 0.6 {
        recommendations.push("Increase validation frequency to improve confidence scores".to_string());
    }
    
    if stats.low_confidence_count > stats.total_components / 4 {
        recommendations.push("Consider removing or revalidating low-confidence components".to_string());
    }
    
    if stats.total_validations < stats.total_components as u64 * 2 {
        recommendations.push("Increase validation coverage - aim for 2+ validations per component".to_string());
    }
    
    let stale_components = meta_memory.get_stale_components()?;
    if !stale_components.is_empty() {
        recommendations.push(format!("Review {} stale components for cleanup", stale_components.len()));
    }
    
    if stats.components_by_type.len() < 5 {
        recommendations.push("Expand knowledge tracking to more component types".to_string());
    }
    
    if stats.overall_success_rate < 0.7 && stats.total_validations > 10 {
        recommendations.push("Investigate causes of low validation success rate".to_string());
    }
    
    Ok(recommendations)
} 