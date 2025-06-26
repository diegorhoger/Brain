//! Memory Domain Logic and Abstractions
//! 
//! This module defines the core memory abstractions and domain logic
//! without any I/O dependencies. Infrastructure implementations
//! are provided through trait implementations.

use brain_types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Generic memory trait for all memory types
#[allow(async_fn_in_trait)]
pub trait Memory {
    type Item;
    type Query;
    type Result;

    /// Store an item in memory
    async fn store(&mut self, item: Self::Item) -> Result<Uuid>;
    
    /// Retrieve items based on a query
    async fn retrieve(&self, query: &Self::Query) -> Result<Vec<Self::Result>>;
    
    /// Update an existing item
    async fn update(&mut self, id: Uuid, item: Self::Item) -> Result<()>;
    
    /// Remove an item from memory
    async fn remove(&mut self, id: Uuid) -> Result<()>;
    
    /// Get memory statistics
    async fn stats(&self) -> Result<MemoryStats>;
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

/// Working memory query structure
#[derive(Debug, Clone)]
pub struct WorkingMemoryQuery {
    pub content_pattern: Option<String>,
    pub priority: Option<Priority>,
    pub min_importance: Option<f64>,
    pub created_after: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
}

impl Default for WorkingMemoryQuery {
    fn default() -> Self {
        Self {
            content_pattern: None,
            priority: None,
            min_importance: None,
            created_after: None,
            limit: None,
        }
    }
}

/// Episodic event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicEvent {
    pub id: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub context: HashMap<String, String>,
    pub importance: f64,
    pub tags: Vec<String>,
    pub source: String,
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

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

/// Episodic memory query structure
#[derive(Debug, Clone)]
pub struct EpisodicQuery {
    pub content_pattern: Option<String>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub min_importance: Option<f64>,
    pub tags: Vec<String>,
    pub context_filters: HashMap<String, String>,
    pub limit: Option<usize>,
}

impl Default for EpisodicQuery {
    fn default() -> Self {
        Self {
            content_pattern: None,
            time_range: None,
            min_importance: None,
            tags: Vec::new(),
            context_filters: HashMap::new(),
            limit: None,
        }
    }
}

/// Semantic concept structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConcept {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub embedding: Vec<f32>,
    pub frequency: u32,
    pub confidence: f64,
    pub last_updated: DateTime<Utc>,
    pub source_events: Vec<Uuid>,
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

    pub fn similarity(&self, other: &SemanticConcept) -> f64 {
        cosine_similarity(&self.embedding, &other.embedding)
    }

    pub fn update_confidence(&mut self, positive_feedback: bool) {
        if positive_feedback {
            self.confidence = (self.confidence + 0.1).min(1.0);
        } else {
            self.confidence = (self.confidence - 0.1).max(0.0);
        }
        self.last_updated = Utc::now();
    }
}

/// Semantic memory query structure
#[derive(Debug, Clone)]
pub struct SemanticQuery {
    pub name_pattern: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub min_confidence: Option<f64>,
    pub min_similarity: Option<f64>,
    pub limit: Option<usize>,
}

impl Default for SemanticQuery {
    fn default() -> Self {
        Self {
            name_pattern: None,
            embedding: None,
            min_confidence: None,
            min_similarity: None,
            limit: None,
        }
    }
}

/// Repository trait for working memory
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait WorkingMemoryRepository: Send + Sync {
    async fn store_item(&mut self, item: WorkingMemoryItem) -> Result<Uuid>;
    async fn get_item(&self, id: Uuid) -> Result<Option<WorkingMemoryItem>>;
    async fn update_item(&mut self, item: &WorkingMemoryItem) -> Result<()>;
    async fn remove_item(&mut self, id: Uuid) -> Result<()>;
    async fn query_items(&self, query: &WorkingMemoryQuery) -> Result<Vec<WorkingMemoryItem>>;
    async fn get_consolidation_candidates(&self, age_threshold_hours: i64) -> Result<Vec<WorkingMemoryItem>>;
    async fn prune_low_importance(&mut self, threshold: f64) -> Result<Vec<Uuid>>;
    async fn stats(&self) -> Result<MemoryStats>;
}

/// Repository trait for episodic memory
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait EpisodicMemoryRepository: Send + Sync {
    async fn store_event(&mut self, event: EpisodicEvent) -> Result<Uuid>;
    async fn get_event(&self, id: Uuid) -> Result<Option<EpisodicEvent>>;
    async fn update_event(&mut self, event: &EpisodicEvent) -> Result<()>;
    async fn remove_event(&mut self, id: Uuid) -> Result<()>;
    async fn query_events(&self, query: &EpisodicQuery) -> Result<Vec<EpisodicEvent>>;
    async fn get_events_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<EpisodicEvent>>;
    async fn apply_forgetting(&mut self, decay_rate: f64, min_importance: f64) -> Result<usize>;
    async fn stats(&self) -> Result<MemoryStats>;
}

/// Repository trait for semantic memory
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait SemanticMemoryRepository: Send + Sync {
    async fn store_concept(&mut self, concept: SemanticConcept) -> Result<Uuid>;
    async fn get_concept(&self, id: Uuid) -> Result<Option<SemanticConcept>>;
    async fn update_concept(&mut self, concept: &SemanticConcept) -> Result<()>;
    async fn remove_concept(&mut self, id: Uuid) -> Result<()>;
    async fn query_concepts(&self, query: &SemanticQuery) -> Result<Vec<SemanticConcept>>;
    async fn find_similar(&self, embedding: &[f32], threshold: f64, limit: usize) -> Result<Vec<(Uuid, f64)>>;
    async fn merge_concepts(&mut self, id1: Uuid, id2: Uuid) -> Result<Uuid>;
    async fn stats(&self) -> Result<MemoryStats>;
}

/// Consolidation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationConfig {
    pub working_to_episodic_hours: i64,
    pub min_access_count: u32,
    pub importance_threshold: f64,
    pub max_episodic_events: usize,
    pub semantic_extraction_threshold: f64,
    pub decay_rate: f64,
    pub forgetting_threshold: f64,
}

impl Default for ConsolidationConfig {
    fn default() -> Self {
        Self {
            working_to_episodic_hours: 24,
            min_access_count: 3,
            importance_threshold: 0.5,
            max_episodic_events: 10000,
            semantic_extraction_threshold: 0.7,
            decay_rate: 0.1,
            forgetting_threshold: 0.2,
        }
    }
}

/// Consolidation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub working_to_episodic: usize,
    pub episodic_to_semantic: usize,
    pub forgotten_events: usize,
}

/// Cross-memory query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossMemoryResults {
    pub working_results: Vec<WorkingMemoryItem>,
    pub episodic_results: Vec<EpisodicEvent>,
    pub semantic_results: Vec<SemanticConcept>,
}

/// Memory system service coordinating all memory types
pub struct MemoryService {
    working_repo: Box<dyn WorkingMemoryRepository>,
    episodic_repo: Box<dyn EpisodicMemoryRepository>,
    semantic_repo: Box<dyn SemanticMemoryRepository>,
    consolidation_config: ConsolidationConfig,
}

impl MemoryService {
    pub fn new(
        working_repo: Box<dyn WorkingMemoryRepository>,
        episodic_repo: Box<dyn EpisodicMemoryRepository>,
        semantic_repo: Box<dyn SemanticMemoryRepository>,
    ) -> Self {
        Self {
            working_repo,
            episodic_repo,
            semantic_repo,
            consolidation_config: ConsolidationConfig::default(),
        }
    }

    pub async fn learn(&mut self, content: String, priority: Priority) -> Result<Uuid> {
        let item = WorkingMemoryItem::new(content, priority);
        let id = item.id;
        self.working_repo.store_item(item).await?;
        Ok(id)
    }

    pub async fn recall_working(&self, id: Uuid) -> Result<Option<WorkingMemoryItem>> {
        self.working_repo.get_item(id).await
    }

    pub async fn query_working(&self, query: &WorkingMemoryQuery) -> Result<Vec<WorkingMemoryItem>> {
        self.working_repo.query_items(query).await
    }

    pub async fn query_episodic(&self, query: &EpisodicQuery) -> Result<Vec<EpisodicEvent>> {
        self.episodic_repo.query_events(query).await
    }

    pub async fn query_semantic(&self, query: &SemanticQuery) -> Result<Vec<SemanticConcept>> {
        self.semantic_repo.query_concepts(query).await
    }

    pub async fn store_concept(&mut self, concept: SemanticConcept) -> Result<Uuid> {
        self.semantic_repo.store_concept(concept).await
    }

    pub async fn consolidate(&mut self) -> Result<ConsolidationResult> {
        // Get consolidation candidates from working memory
        let candidates = self.working_repo
            .get_consolidation_candidates(self.consolidation_config.working_to_episodic_hours)
            .await?;

        let mut working_to_episodic = 0;
        let mut episodic_to_semantic = 0;

        // Move working memory items to episodic memory
        for item in candidates {
            if item.access_count >= self.consolidation_config.min_access_count
                && item.importance_score() >= self.consolidation_config.importance_threshold
            {
                let event = EpisodicEvent::new(
                    item.content.clone(),
                    HashMap::new(),
                    item.importance_score(),
                    "working_memory".to_string(),
                );
                
                self.episodic_repo.store_event(event).await?;
                self.working_repo.remove_item(item.id).await?;
                working_to_episodic += 1;
            }
        }

        // Extract semantic patterns from episodic memory
        episodic_to_semantic += self.extract_semantic_patterns().await?;

        // Apply forgetting to episodic memory
        let forgotten_events = self.episodic_repo
            .apply_forgetting(
                self.consolidation_config.decay_rate,
                self.consolidation_config.forgetting_threshold,
            )
            .await?;

        Ok(ConsolidationResult {
            working_to_episodic,
            episodic_to_semantic,
            forgotten_events,
        })
    }

    async fn extract_semantic_patterns(&mut self) -> Result<usize> {
        // This would implement pattern extraction logic
        // For now, return 0 as placeholder
        Ok(0)
    }

    pub async fn query_all_memories(&self, content_pattern: &str) -> Result<CrossMemoryResults> {
        let working_query = WorkingMemoryQuery {
            content_pattern: Some(content_pattern.to_string()),
            ..Default::default()
        };

        let episodic_query = EpisodicQuery {
            content_pattern: Some(content_pattern.to_string()),
            ..Default::default()
        };

        let semantic_query = SemanticQuery {
            name_pattern: Some(content_pattern.to_string()),
            ..Default::default()
        };

        let working_results = self.working_repo.query_items(&working_query).await?;
        let episodic_results = self.episodic_repo.query_events(&episodic_query).await?;
        let semantic_results = self.semantic_repo.query_concepts(&semantic_query).await?;

        Ok(CrossMemoryResults {
            working_results,
            episodic_results,
            semantic_results,
        })
    }

    pub fn configure_consolidation(&mut self, config: ConsolidationConfig) {
        self.consolidation_config = config;
    }

    pub fn get_consolidation_config(&self) -> &ConsolidationConfig {
        &self.consolidation_config
    }
}

/// Calculate cosine similarity between two vectors
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        (dot_product / (norm_a * norm_b)) as f64
    }
}
