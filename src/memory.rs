//! # Memory Module Foundation
//! 
//! This module implements the core memory architecture with three types of memory:
//! - Working Memory: Short-term reasoning and active information processing
//! - Episodic Memory: Event recall with temporal indexing using DuckDB
//! - Semantic Memory: Abstract knowledge using vector embeddings and FAISS
//!
//! ## Architecture Overview
//!
//! The memory system follows a hierarchical approach where information flows
//! from working memory to episodic memory through consolidation processes,
//! and patterns are extracted from episodic memory into semantic memory
//! for long-term abstract knowledge storage.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BinaryHeap};
use uuid::Uuid;

/// Common interface for all memory types
pub trait Memory {
    type Item;
    type Query;
    type Result;

    /// Store an item in memory
    fn store(&mut self, item: Self::Item) -> Result<Uuid>;
    
    /// Retrieve items based on a query
    fn retrieve(&self, query: &Self::Query) -> Result<Vec<Self::Result>>;
    
    /// Update an existing item
    fn update(&mut self, id: Uuid, item: Self::Item) -> Result<()>;
    
    /// Remove an item from memory
    fn remove(&mut self, id: Uuid) -> Result<()>;
    
    /// Get memory statistics
    fn stats(&self) -> MemoryStats;
}

/// Memory statistics for monitoring and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_items: usize,
    pub size_bytes: usize,
    pub last_access: DateTime<Utc>,
    pub access_count: u64,
    pub consolidation_count: u64,
}

/// Priority levels for working memory items
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Working memory item with priority and temporal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryItem {
    pub id: Uuid,
    pub content: String,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u32,
    pub decay_factor: f64,
}

impl WorkingMemoryItem {
    pub fn new(content: String, priority: Priority) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content,
            priority,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            decay_factor: 1.0,
        }
    }

    /// Update decay factor based on time and access patterns
    pub fn update_decay(&mut self) {
        let time_since_access = Utc::now().signed_duration_since(self.last_accessed);
        let hours_since_access = time_since_access.num_hours() as f64;
        
        // Exponential decay with half-life of 24 hours
        self.decay_factor = 0.5_f64.powf(hours_since_access / 24.0);
        
        // Boost factor based on access frequency
        let access_boost = 1.0 + (self.access_count as f64 * 0.1);
        self.decay_factor *= access_boost;
        
        // Clamp between 0.01 and 1.0
        self.decay_factor = self.decay_factor.max(0.01).min(1.0);
    }

    /// Calculate current importance score
    pub fn importance_score(&self) -> f64 {
        let priority_weight = self.priority as u8 as f64;
        priority_weight * self.decay_factor
    }
}

/// Working memory implementation using priority queues
#[derive(Debug)]
pub struct WorkingMemory {
    items: HashMap<Uuid, WorkingMemoryItem>,
    priority_queue: BinaryHeap<(u64, Uuid)>, // (score * 1000 as int, id)
    max_capacity: usize,
    stats: MemoryStats,
}

impl WorkingMemory {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            items: HashMap::new(),
            priority_queue: BinaryHeap::new(),
            max_capacity,
            stats: MemoryStats {
                total_items: 0,
                size_bytes: 0,
                last_access: Utc::now(),
                access_count: 0,
                consolidation_count: 0,
            },
        }
    }

    /// Add item to working memory, potentially evicting low-priority items
    pub fn add_item(&mut self, content: String, priority: Priority) -> Result<Uuid> {
        let item = WorkingMemoryItem::new(content, priority);
        let id = item.id;
        
        // Check capacity and evict if necessary
        if self.items.len() >= self.max_capacity {
            self.evict_lowest_priority()?;
        }
        
        let score = (item.importance_score() * 1000.0) as u64;
        self.priority_queue.push((score, id));
        self.items.insert(id, item);
        
        self.update_stats();
        Ok(id)
    }

    /// Access an item, updating its access statistics
    pub fn access_item(&mut self, id: Uuid) -> Option<&WorkingMemoryItem> {
        if let Some(item) = self.items.get_mut(&id) {
            item.last_accessed = Utc::now();
            item.access_count += 1;
            item.update_decay();
            
            self.stats.access_count += 1;
            self.stats.last_access = Utc::now();
            
            Some(&*item)
        } else {
            None
        }
    }

    /// Get items by priority level
    pub fn get_by_priority(&self, priority: Priority) -> Vec<&WorkingMemoryItem> {
        self.items.values()
            .filter(|item| item.priority == priority)
            .collect()
    }

    /// Get items that should be consolidated to episodic memory
    pub fn get_consolidation_candidates(&self, age_threshold_hours: i64) -> Vec<&WorkingMemoryItem> {
        let threshold = Utc::now() - chrono::Duration::hours(age_threshold_hours);
        
        self.items.values()
            .filter(|item| item.created_at < threshold && item.access_count > 2)
            .collect()
    }

    /// Remove items that have been consolidated
    pub fn remove_consolidated(&mut self, ids: &[Uuid]) -> Result<()> {
        for id in ids {
            self.items.remove(id);
        }
        
        // Rebuild priority queue (inefficient but simple for now)
        self.rebuild_priority_queue();
        self.update_stats();
        
        Ok(())
    }

    fn evict_lowest_priority(&mut self) -> Result<()> {
        // Find item with lowest importance score
        if let Some(lowest_id) = self.items.iter()
            .min_by(|a, b| a.1.importance_score().partial_cmp(&b.1.importance_score()).unwrap())
            .map(|(id, _)| *id) {
            
            self.items.remove(&lowest_id);
            self.rebuild_priority_queue();
        }
        
        Ok(())
    }

    fn rebuild_priority_queue(&mut self) {
        self.priority_queue.clear();
        for (id, item) in &self.items {
            let score = (item.importance_score() * 1000.0) as u64;
            self.priority_queue.push((score, *id));
        }
    }

    fn update_stats(&mut self) {
        self.stats.total_items = self.items.len();
        self.stats.size_bytes = self.items.len() * std::mem::size_of::<WorkingMemoryItem>();
        self.stats.last_access = Utc::now();
    }
}

/// Episodic memory event with contextual information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicEvent {
    pub id: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub context: HashMap<String, String>,
    pub importance: f64,
    pub tags: Vec<String>,
    pub source: String, // working memory, external input, etc.
}

impl EpisodicEvent {
    pub fn new(content: String, context: HashMap<String, String>, importance: f64, source: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            timestamp: Utc::now(),
            context,
            importance,
            tags: Vec::new(),
            source,
        }
    }
}

/// Query structure for episodic memory
#[derive(Debug, Clone)]
pub struct EpisodicQuery {
    pub content_pattern: Option<String>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub min_importance: Option<f64>,
    pub tags: Vec<String>,
    pub context_filters: HashMap<String, String>,
    pub limit: Option<usize>,
}

/// Semantic memory concept with vector embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConcept {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub embedding: Vec<f32>, // Vector representation
    pub frequency: u32,
    pub confidence: f64,
    pub last_updated: DateTime<Utc>,
    pub source_events: Vec<Uuid>, // References to episodic events
}

impl SemanticConcept {
    pub fn new(name: String, description: String, embedding: Vec<f32>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            embedding,
            frequency: 1,
            confidence: 0.5,
            last_updated: Utc::now(),
            source_events: Vec::new(),
        }
    }
}

/// Configuration for memory consolidation processes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationConfig {
    pub working_to_episodic_hours: i64,
    pub min_access_count: u32,
    pub importance_threshold: f64,
    pub max_episodic_events: usize,
    pub semantic_extraction_threshold: f64,
}

impl Default for ConsolidationConfig {
    fn default() -> Self {
        Self {
            working_to_episodic_hours: 24,
            min_access_count: 3,
            importance_threshold: 0.5,
            max_episodic_events: 10000,
            semantic_extraction_threshold: 0.7,
        }
    }
}

/// Main memory system orchestrating all memory types
#[derive(Debug)]
pub struct MemorySystem {
    working_memory: WorkingMemory,
    consolidation_config: ConsolidationConfig,
    
    // Database connections will be added in subtasks
    // episodic_db: Option<Connection>,
    // semantic_index: Option<FaissIndex>,
}

impl MemorySystem {
    pub fn new(working_memory_capacity: usize) -> Self {
        Self {
            working_memory: WorkingMemory::new(working_memory_capacity),
            consolidation_config: ConsolidationConfig::default(),
        }
    }

    /// Add information to working memory
    pub fn learn(&mut self, content: String, priority: Priority) -> Result<Uuid> {
        self.working_memory.add_item(content, priority)
    }

    /// Access information from working memory
    pub fn recall_working(&mut self, id: Uuid) -> Option<&WorkingMemoryItem> {
        self.working_memory.access_item(id)
    }

    /// Get system-wide memory statistics
    pub fn get_stats(&self) -> HashMap<String, MemoryStats> {
        let mut stats = HashMap::new();
        stats.insert("working".to_string(), self.working_memory.stats.clone());
        stats
    }

    /// Run consolidation process (placeholder - will be implemented in subtasks)
    pub fn consolidate(&mut self) -> Result<usize> {
        let candidates = self.working_memory.get_consolidation_candidates(
            self.consolidation_config.working_to_episodic_hours
        );
        
        // TODO: Implement actual consolidation to episodic memory
        // This will be implemented in subtask 3.2
        
        Ok(candidates.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_memory_item_creation() {
        let item = WorkingMemoryItem::new("Test content".to_string(), Priority::High);
        assert_eq!(item.content, "Test content");
        assert_eq!(item.priority, Priority::High);
        assert_eq!(item.access_count, 0);
        assert_eq!(item.decay_factor, 1.0);
    }

    #[test]
    fn test_working_memory_basic_operations() {
        let mut memory = WorkingMemory::new(5);
        
        let id = memory.add_item("Test item".to_string(), Priority::Medium).unwrap();
        assert_eq!(memory.items.len(), 1);
        
        let item = memory.access_item(id).unwrap();
        assert_eq!(item.content, "Test item");
        assert_eq!(item.access_count, 1);
    }

    #[test]
    fn test_working_memory_capacity_management() {
        let mut memory = WorkingMemory::new(2);
        
        // Add items up to capacity
        memory.add_item("Item 1".to_string(), Priority::Low).unwrap();
        memory.add_item("Item 2".to_string(), Priority::High).unwrap();
        
        // Adding third item should evict lowest priority
        memory.add_item("Item 3".to_string(), Priority::Medium).unwrap();
        
        assert_eq!(memory.items.len(), 2);
        
        // Check that high priority item is still there
        let high_priority_items = memory.get_by_priority(Priority::High);
        assert_eq!(high_priority_items.len(), 1);
    }

    #[test]
    fn test_consolidation_candidates() {
        let mut memory = WorkingMemory::new(10);
        
        let id = memory.add_item("Old item".to_string(), Priority::Medium).unwrap();
        
        // Simulate multiple accesses
        memory.access_item(id);
        memory.access_item(id);
        memory.access_item(id);
        
        // Should be a candidate for consolidation (age threshold will be 0 for testing)
        let candidates = memory.get_consolidation_candidates(0);
        assert_eq!(candidates.len(), 1);
    }

    #[test]
    fn test_memory_system_integration() {
        let mut system = MemorySystem::new(10);
        
        let id = system.learn("Test learning".to_string(), Priority::High).unwrap();
        
        let recalled = system.recall_working(id).unwrap();
        assert_eq!(recalled.content, "Test learning");
        
        let stats = system.get_stats();
        assert!(stats.contains_key("working"));
    }

    #[test]
    fn test_episodic_event_creation() {
        let mut context = HashMap::new();
        context.insert("location".to_string(), "home".to_string());
        
        let event = EpisodicEvent::new(
            "User asked about weather".to_string(),
            context,
            0.8,
            "user_input".to_string()
        );
        
        assert_eq!(event.content, "User asked about weather");
        assert_eq!(event.importance, 0.8);
        assert_eq!(event.context.get("location"), Some(&"home".to_string()));
    }

    #[test]
    fn test_semantic_concept_creation() {
        let embedding = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let concept = SemanticConcept::new(
            "weather".to_string(),
            "Information about atmospheric conditions".to_string(),
            embedding.clone()
        );
        
        assert_eq!(concept.name, "weather");
        assert_eq!(concept.embedding, embedding);
        assert_eq!(concept.frequency, 1);
        assert_eq!(concept.confidence, 0.5);
    }
} 