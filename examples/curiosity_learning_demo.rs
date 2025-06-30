//! Curiosity-Driven Learning Demo
//! 
//! This example demonstrates the complete curiosity-driven learning system,
//! showcasing how it integrates with meta-memory and novelty detection to
//! create intelligent learning priorities and adaptive exploration behavior.

use anyhow::Result;
use brain_cognitive::learning::{
    CuriosityLearningEngine, CuriosityConfig, LearningEvent, 
    CuriosityDrive, NoveltyDetector, NoveltyAssessment, 
    NoveltyLevel, CuriosityLearningService,
};
use brain_cognitive::meta::{
    MetaMemoryService, MetaMemoryRepository, MetaMemoryAnalytics, 
    MetaMemoryMaintenance, MetaMemoryConfig, KnowledgeType,
};
use brain_types::BrainError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use rand::Rng;

/// Simple novelty detector implementation for demo
pub struct SimpleNoveltyDetector {
    known_patterns: Arc<RwLock<HashMap<String, f64>>>,
}

impl SimpleNoveltyDetector {
    pub fn new() -> Self {
        Self {
            known_patterns: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add some known patterns to reduce novelty scores
    pub async fn seed_with_patterns(&self, patterns: &[&str]) {
        let mut known = self.known_patterns.write().await;
        for pattern in patterns {
            known.insert(pattern.to_string(), 0.9); // High familiarity
        }
    }
}

#[async_trait::async_trait]
impl NoveltyDetector for SimpleNoveltyDetector {
    async fn assess_novelty(&self, input: &str) -> Result<NoveltyAssessment, BrainError> {
        let known = self.known_patterns.read().await;
        
        // Simple novelty assessment based on pattern familiarity
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut novelty_score = 1.0; // Start with high novelty
        let mut familiar_count = 0;
        
        // Check for familiar patterns
        for word in &words {
            if known.contains_key(&word.to_lowercase()) {
                familiar_count += 1;
            }
        }
        
        if !words.is_empty() {
            let familiarity = familiar_count as f64 / words.len() as f64;
            novelty_score = 1.0 - familiarity; // High familiarity = low novelty
        }
        
        // Add some randomness for interesting results
        let mut rng = rand::thread_rng();
        novelty_score = (novelty_score + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
        
        let novelty_level = if novelty_score > 0.8 {
            NoveltyLevel::VeryHigh
        } else if novelty_score > 0.6 {
            NoveltyLevel::High
        } else if novelty_score > 0.4 {
            NoveltyLevel::Medium
        } else if novelty_score > 0.2 {
            NoveltyLevel::Low
        } else {
            NoveltyLevel::VeryLow
        };
        
        Ok(NoveltyAssessment {
            novelty_score,
            novelty_level,
            novelty_factors: vec![
                format!("Word familiarity: {:.2}", 1.0 - novelty_score),
                format!("Pattern complexity: {:.2}", words.len() as f64 / 10.0),
            ],
            assessment_confidence: 0.8,
        })
    }
    
    async fn update_models(&mut self, input: &str) -> Result<(), BrainError> {
        let mut known = self.known_patterns.write().await;
        let words: Vec<&str> = input.split_whitespace().collect();
        
        // Add new words to known patterns with moderate familiarity
        for word in words {
            let current_familiarity = known.get(&word.to_lowercase()).unwrap_or(&0.0);
            let new_familiarity = (current_familiarity + 0.1).min(1.0);
            known.insert(word.to_lowercase(), new_familiarity);
        }
        
        Ok(())
    }
}

/// Simple meta-memory repository implementation for demo
pub struct SimpleMetaMemoryRepository {
    items: Arc<RwLock<HashMap<Uuid, brain_cognitive::meta::MetaMemoryItem>>>,
    component_to_meta: Arc<RwLock<HashMap<Uuid, Uuid>>>,
}

impl SimpleMetaMemoryRepository {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            component_to_meta: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl MetaMemoryRepository for SimpleMetaMemoryRepository {
    async fn store_item(&mut self, item: brain_cognitive::meta::MetaMemoryItem) -> brain_cognitive::meta::MetaMemoryResult<Uuid> {
        let mut items = self.items.write().await;
        let mut component_map = self.component_to_meta.write().await;
        
        let item_id = item.id;
        let component_id = item.component_id;
        
        items.insert(item_id, item);
        component_map.insert(component_id, item_id);
        
        Ok(item_id)
    }
    
    async fn get_item(&self, id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<Option<brain_cognitive::meta::MetaMemoryItem>> {
        let items = self.items.read().await;
        Ok(items.get(&id).cloned())
    }
    
    async fn get_item_by_component(&self, component_id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<Option<brain_cognitive::meta::MetaMemoryItem>> {
        let component_map = self.component_to_meta.read().await;
        if let Some(&meta_id) = component_map.get(&component_id) {
            self.get_item(meta_id).await
        } else {
            Ok(None)
        }
    }
    
    async fn query_items(&self, _query: &brain_cognitive::meta::MetaMemoryQuery) -> brain_cognitive::meta::MetaMemoryResult<Vec<brain_cognitive::meta::MetaMemoryItem>> {
        let items = self.items.read().await;
        Ok(items.values().cloned().collect())
    }
    
    async fn remove_item(&mut self, id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<bool> {
        let mut items = self.items.write().await;
        Ok(items.remove(&id).is_some())
    }
    
    async fn batch_update(&mut self, items_to_update: Vec<brain_cognitive::meta::MetaMemoryItem>) -> brain_cognitive::meta::MetaMemoryResult<Vec<Uuid>> {
        let mut ids = Vec::new();
        for item in items_to_update {
            let id = self.store_item(item).await?;
            ids.push(id);
        }
        Ok(ids)
    }
    
    async fn count_items(&self) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        let items = self.items.read().await;
        Ok(items.len())
    }
    
    async fn clear_all(&mut self) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        let mut items = self.items.write().await;
        let mut component_map = self.component_to_meta.write().await;
        let count = items.len();
        items.clear();
        component_map.clear();
        Ok(count)
    }
}

// Simple implementations for analytics and maintenance
pub struct SimpleMetaMemoryAnalytics;

#[async_trait::async_trait]
impl MetaMemoryAnalytics for SimpleMetaMemoryAnalytics {
    async fn calculate_stats(&self) -> brain_cognitive::meta::MetaMemoryResult<brain_cognitive::meta::MetaMemoryStats> {
        Ok(brain_cognitive::meta::MetaMemoryStats::default())
    }
    
    async fn get_confidence_distribution(&self) -> brain_cognitive::meta::MetaMemoryResult<HashMap<String, usize>> {
        Ok(HashMap::new())
    }
    
    async fn get_quality_distribution(&self) -> brain_cognitive::meta::MetaMemoryResult<HashMap<String, usize>> {
        Ok(HashMap::new())
    }
    
    async fn get_knowledge_type_distribution(&self) -> brain_cognitive::meta::MetaMemoryResult<HashMap<brain_cognitive::meta::KnowledgeType, usize>> {
        Ok(HashMap::new())
    }
    
    async fn get_trending_components(&self, _limit: usize) -> brain_cognitive::meta::MetaMemoryResult<Vec<brain_cognitive::meta::MetaMemoryItem>> {
        Ok(Vec::new())
    }
    
    async fn get_performance_metrics(&self, _hours_back: f64) -> brain_cognitive::meta::MetaMemoryResult<brain_cognitive::meta::PerformanceMetrics> {
        Ok(brain_cognitive::meta::PerformanceMetrics {
            time_period_hours: 24.0,
            items_added: 0,
            items_updated: 0,
            items_accessed: 0,
            avg_confidence_change: 0.0,
            avg_quality_improvement: 0.0,
            validation_success_rate: 0.8,
            storage_efficiency: 0.9,
        })
    }
}

pub struct SimpleMetaMemoryMaintenance;

#[async_trait::async_trait]
impl MetaMemoryMaintenance for SimpleMetaMemoryMaintenance {
    async fn cleanup_stale_components(&mut self, _config: &MetaMemoryConfig) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        Ok(0)
    }
    
    async fn optimize_storage(&mut self) -> brain_cognitive::meta::MetaMemoryResult<()> {
        Ok(())
    }
    
    async fn backup_data(&self, _backup_path: &str) -> brain_cognitive::meta::MetaMemoryResult<()> {
        Ok(())
    }
    
    async fn restore_data(&mut self, _backup_path: &str) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        Ok(0)
    }
    
    async fn validate_integrity(&self) -> brain_cognitive::meta::MetaMemoryResult<brain_cognitive::meta::IntegrityReport> {
        Ok(brain_cognitive::meta::IntegrityReport {
            total_items: 0,
            corrupted_items: 0,
            missing_metadata: 0,
            invalid_confidence: 0,
            timestamp_issues: 0,
            integrity_score: 1.0,
            issues: Vec::new(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("🧠 Brain AI - Curiosity-Driven Learning System Demo");
    println!("===================================================");
    println!();

    // Phase 1: System Initialization
    println!("📋 Phase 1: System Initialization");
    println!("----------------------------------");
    
    // Create novelty detector
    let novelty_detector = Arc::new(SimpleNoveltyDetector::new());
    
    // Seed with some common patterns to make results more interesting
    novelty_detector.seed_with_patterns(&[
        "the", "and", "a", "to", "of", "in", "is", "for", "with", "on",
        "machine", "learning", "artificial", "intelligence", "computer", "data"
    ]).await;
    
    println!("✅ Novelty detector initialized with basic patterns");
    
    // Create meta-memory service
    let meta_memory_repo = Arc::new(SimpleMetaMemoryRepository::new());
    let meta_memory_analytics = Arc::new(SimpleMetaMemoryAnalytics);
    let meta_memory_maintenance = Arc::new(SimpleMetaMemoryMaintenance);
    let meta_memory_config = MetaMemoryConfig::default();
    
    let meta_memory = Arc::new(MetaMemoryService::new(
        meta_memory_repo,
        meta_memory_analytics,
        meta_memory_maintenance,
        meta_memory_config,
    ));
    
    println!("✅ Meta-memory system initialized");
    
    // Create curiosity learning engine
    let curiosity_config = CuriosityConfig {
        novelty_weight: 0.4,
        uncertainty_weight: 0.3,
        progress_weight: 0.3,
        learning_threshold: 0.25,
        exploration_rate: 0.7,
        ..Default::default()
    };
    
    let mut curiosity_engine = CuriosityLearningEngine::new(
        curiosity_config.clone(),
        meta_memory.clone(),
        novelty_detector.clone(),
    );
    
    println!("✅ Curiosity-driven learning engine initialized");
    println!();

    // Phase 2: Populate Meta-Memory with Sample Knowledge
    println!("📊 Phase 2: Populating Meta-Memory with Sample Knowledge");
    println!("--------------------------------------------------------");
    
         // Add some sample knowledge components
     for (i, (knowledge_type, confidence, source)) in [
         (KnowledgeType::ConceptNode, 0.9, "Core concept: 'learning'"),
         (KnowledgeType::ConceptNode, 0.85, "Important concept: 'intelligence'"),
         (KnowledgeType::ConceptNode, 0.8, "Key concept: 'knowledge'"),
         (KnowledgeType::Rule, 0.7, "Rule: if curious then explore"),
         (KnowledgeType::Rule, 0.75, "Rule: if learning then remember"),
         (KnowledgeType::Pattern, 0.65, "Pattern: question -> research -> answer"),
     ].iter().enumerate() {
         let component_id = Uuid::new_v4();
         match meta_memory.track_component(
             component_id,
             knowledge_type.clone(),
             *confidence,
             source.to_string(),
         ).await {
             Ok(_) => println!("   ✅ Added knowledge component {}: {}", i + 1, source),
             Err(e) => println!("   ❌ Failed to add component: {}", e),
         }
     }
    
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
        
        match curiosity_engine.assess_curiosity(input).await {
            Ok(curiosity_score) => {
                curiosity_scores.push(curiosity_score);
                
                println!("   Input: \"{}\"", input);
                println!("   Curiosity Score: {:.3}", curiosity_score);
                
                if curiosity_score >= curiosity_config.learning_threshold {
                    println!("   🎯 Learning priority created!");
                } else {
                    println!("   ℹ️  Below learning threshold");
                }
            }
            Err(e) => {
                println!("   ❌ Error assessing curiosity: {}", e);
            }
        }
        println!();
    }
    
    let avg_curiosity = if !curiosity_scores.is_empty() {
        curiosity_scores.iter().sum::<f64>() / curiosity_scores.len() as f64
    } else {
        0.0
    };
    println!("📊 Average curiosity score across all tests: {:.3}", avg_curiosity);
    println!();

    // Phase 4: Learning Priorities Analysis
    println!("🎯 Phase 4: Learning Priorities Analysis");
    println!("----------------------------------------");
    
    match curiosity_engine.get_top_priorities(5).await {
        Ok(top_priorities) => {
            println!("🏆 Top {} Learning Priorities:", top_priorities.len());
            
            for (i, priority) in top_priorities.iter().enumerate() {
                println!("   {}. ID: {}", i + 1, priority.id);
                println!("      Content: \"{}\"", priority.content);
                println!("      Curiosity Score: {:.3}", priority.curiosity_score);
                println!("      Drive: {:?}", priority.primary_drive);
                println!("      Expected Value: {:.3}", priority.expected_value);
                println!("      Knowledge Gaps: {}", priority.knowledge_gaps.len());
                println!();
            }
        }
        Err(e) => {
            println!("❌ Error getting learning priorities: {}", e);
        }
    }

    // Phase 5: Simulate Learning Events
    println!("📚 Phase 5: Simulating Learning Events");
    println!("--------------------------------------");
    
    // Simulate some learning events
    for i in 0..3 {
        let priority_id = Uuid::new_v4();
        let content = format!("Learning topic {}: Advanced concepts in curiosity-driven systems", i + 1);
        
        println!("🎓 Simulating learning event for: \"{}\"", content);
        
                 let mut event = LearningEvent::new(
             priority_id,
             content.clone(),
             CuriosityDrive::NoveltySeeker,
             KnowledgeType::ConceptNode,
         );
        
        // Simulate learning outcomes
        let mut rng = rand::thread_rng();
        let success = rng.gen_bool(0.7); // 70% success rate
        
        event.success = success;
        event.progress_gained = if success { 
            rng.gen_range(0.3..0.7) // 30-70% progress
        } else { 
            rng.gen_range(0.1..0.3) // 10-30% progress
        };
        event.duration_minutes = rng.gen_range(15.0..45.0); // 15-45 minutes
        event.satisfaction = if success { 
            rng.gen_range(0.6..1.0) // 60-100% satisfaction
        } else { 
            rng.gen_range(0.2..0.6) // 20-60% satisfaction
        };
        
        // Store event details before moving
        let progress_gained = event.progress_gained;
        let duration_minutes = event.duration_minutes;
        let satisfaction = event.satisfaction;
        
        match curiosity_engine.record_learning_event(event).await {
            Ok(_) => {
                println!("   ✅ Success: {}", if success { "Yes" } else { "No" });
                println!("   📈 Progress Gained: {:.1}%", progress_gained * 100.0);
                println!("   ⏱️  Duration: {:.1} minutes", duration_minutes);
                println!("   😊 Satisfaction: {:.1}%", satisfaction * 100.0);
            }
            Err(e) => {
                println!("   ❌ Error recording learning event: {}", e);
            }
        }
        println!();
    }
    
    // Get updated statistics
    match curiosity_engine.get_stats().await {
        Ok(updated_stats) => {
            println!("📈 Updated curiosity system statistics:");
            println!("   • Total priorities: {}", updated_stats.total_priorities);
            println!("   • Active priorities: {}", updated_stats.active_priorities);
            println!("   • Completed priorities: {}", updated_stats.completed_priorities);
            println!("   • Overall success rate: {:.3}", updated_stats.overall_success_rate);
            println!("   • Average progress: {:.3}", updated_stats.average_progress);
            println!();
            
            println!("🎭 Drive Distribution:");
            for (drive, count) in &updated_stats.drive_distribution {
                println!("   • {:?}: {} priorities", drive, count);
            }
        }
        Err(e) => {
            println!("❌ Error getting statistics: {}", e);
        }
    }
    println!();

    // Phase 6: Summary
    println!("🎉 Phase 6: Demonstration Summary");
    println!("=================================");
    println!("✅ Curiosity-driven learning system operational");
    println!("✅ Novelty detection for curiosity assessment");
    println!("✅ Meta-memory integration for knowledge tracking");
    println!("✅ Learning priority creation and management");
    println!("✅ Learning event simulation and recording");
    println!("✅ Statistical analysis and reporting");
    println!();
    
    println!("🎯 The curiosity learning system successfully demonstrated:");
    println!("   • Adaptive novelty assessment");
    println!("   • Knowledge gap identification");
    println!("   • Learning priority generation");
    println!("   • Progress tracking and statistics");
    println!("   • Integration with meta-memory system");
    println!();
    
    println!("💡 Ready for integration with other Brain AI components!");

    Ok(())
} 