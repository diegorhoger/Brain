//! Memory Infrastructure Implementations
//! 
//! Concrete implementations of memory repository traits using
//! in-memory storage, file system, and database backends.

use brain_core::*;
use brain_types::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// In-memory implementation of WorkingMemoryRepository
pub struct InMemoryWorkingMemoryRepository {
    items: Arc<RwLock<HashMap<Uuid, WorkingMemoryItem>>>,
    access_count: Arc<RwLock<u64>>,
}

impl InMemoryWorkingMemoryRepository {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            access_count: Arc::new(RwLock::new(0)),
        }
    }
}

impl Default for InMemoryWorkingMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
impl WorkingMemoryRepository for InMemoryWorkingMemoryRepository {
    async fn store_item(&mut self, item: WorkingMemoryItem) -> Result<Uuid> {
        let id = item.id;
        let mut items = self.items.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        items.insert(id, item);
        Ok(id)
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<WorkingMemoryItem>> {
        let items = self.items.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(items.get(&id).cloned())
    }

    async fn update_item(&mut self, item: &WorkingMemoryItem) -> Result<()> {
        let mut items = self.items.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        items.insert(item.id, item.clone());
        Ok(())
    }

    async fn remove_item(&mut self, id: Uuid) -> Result<()> {
        let mut items = self.items.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        items.remove(&id);
        Ok(())
    }

    async fn query_items(&self, query: &WorkingMemoryQuery) -> Result<Vec<WorkingMemoryItem>> {
        let items = self.items.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let mut results: Vec<WorkingMemoryItem> = items.values().cloned().collect();

        // Apply filters
        if let Some(ref pattern) = query.content_pattern {
            results.retain(|item| item.content.contains(pattern));
        }

        if let Some(priority) = query.priority {
            results.retain(|item| item.priority == priority);
        }

        if let Some(min_importance) = query.min_importance {
            results.retain(|item| item.importance_score() >= min_importance);
        }

        if let Some(created_after) = query.created_after {
            results.retain(|item| item.created_at >= created_after);
        }

        // Sort by importance score (descending)
        results.sort_by(|a, b| b.importance_score().partial_cmp(&a.importance_score()).unwrap_or(std::cmp::Ordering::Equal));

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn get_consolidation_candidates(&self, age_threshold_hours: i64) -> Result<Vec<WorkingMemoryItem>> {
        let items = self.items.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let threshold_time = Utc::now() - chrono::Duration::hours(age_threshold_hours);
        
        let candidates: Vec<WorkingMemoryItem> = items
            .values()
            .filter(|item| item.created_at <= threshold_time)
            .cloned()
            .collect();

        Ok(candidates)
    }

    async fn prune_low_importance(&mut self, threshold: f64) -> Result<Vec<Uuid>> {
        let mut items = self.items.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        let mut pruned_ids = Vec::new();

        items.retain(|&id, item| {
            if item.importance_score() < threshold {
                pruned_ids.push(id);
                false
            } else {
                true
            }
        });

        Ok(pruned_ids)
    }

    async fn stats(&self) -> Result<MemoryStats> {
        let items = self.items.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let access_count = *self.access_count.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;

        let total_items = items.len();
        let size_bytes = total_items * std::mem::size_of::<WorkingMemoryItem>();
        let last_access = items.values()
            .map(|item| item.last_accessed)
            .max()
            .unwrap_or_else(Utc::now);

        Ok(MemoryStats {
            total_items,
            size_bytes,
            last_access,
            access_count,
            consolidation_count: 0, // Would be tracked separately
        })
    }
}

/// In-memory implementation of EpisodicMemoryRepository
pub struct InMemoryEpisodicMemoryRepository {
    events: Arc<RwLock<HashMap<Uuid, EpisodicEvent>>>,
}

impl InMemoryEpisodicMemoryRepository {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryEpisodicMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
impl EpisodicMemoryRepository for InMemoryEpisodicMemoryRepository {
    async fn store_event(&mut self, event: EpisodicEvent) -> Result<Uuid> {
        let id = event.id;
        let mut events = self.events.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        events.insert(id, event);
        Ok(id)
    }

    async fn get_event(&self, id: Uuid) -> Result<Option<EpisodicEvent>> {
        let events = self.events.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(events.get(&id).cloned())
    }

    async fn update_event(&mut self, event: &EpisodicEvent) -> Result<()> {
        let mut events = self.events.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        events.insert(event.id, event.clone());
        Ok(())
    }

    async fn remove_event(&mut self, id: Uuid) -> Result<()> {
        let mut events = self.events.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        events.remove(&id);
        Ok(())
    }

    async fn query_events(&self, query: &EpisodicQuery) -> Result<Vec<EpisodicEvent>> {
        let events = self.events.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let mut results: Vec<EpisodicEvent> = events.values().cloned().collect();

        // Apply filters
        if let Some(ref pattern) = query.content_pattern {
            results.retain(|event| event.content.contains(pattern));
        }

        if let Some(min_importance) = query.min_importance {
            results.retain(|event| event.importance >= min_importance);
        }

        if let Some((start, end)) = query.time_range {
            results.retain(|event| event.timestamp >= start && event.timestamp <= end);
        }

        if !query.tags.is_empty() {
            results.retain(|event| {
                query.tags.iter().any(|tag| event.tags.contains(tag))
            });
        }

        for (key, value) in &query.context_filters {
            results.retain(|event| {
                event.context.get(key).map_or(false, |v| v == value)
            });
        }

        // Sort by timestamp (descending)
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn get_events_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<EpisodicEvent>> {
        let query = EpisodicQuery {
            time_range: Some((start, end)),
            ..Default::default()
        };
        self.query_events(&query).await
    }

    async fn apply_forgetting(&mut self, decay_rate: f64, min_importance: f64) -> Result<usize> {
        let mut events = self.events.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        let mut forgotten_count = 0;

        events.retain(|_, event| {
            // Apply time-based decay
            let hours_since = Utc::now().signed_duration_since(event.timestamp).num_hours() as f64;
            let decayed_importance = event.importance * (1.0 - decay_rate).powf(hours_since / 24.0);
            
            if decayed_importance < min_importance {
                forgotten_count += 1;
                false
            } else {
                true
            }
        });

        Ok(forgotten_count)
    }

    async fn stats(&self) -> Result<MemoryStats> {
        let events = self.events.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let total_items = events.len();
        let size_bytes = total_items * std::mem::size_of::<EpisodicEvent>();
        let last_access = events.values()
            .map(|event| event.timestamp)
            .max()
            .unwrap_or_else(Utc::now);

        Ok(MemoryStats {
            total_items,
            size_bytes,
            last_access,
            access_count: 0, // Would be tracked separately
            consolidation_count: 0,
        })
    }
}

/// In-memory implementation of SemanticMemoryRepository
pub struct InMemorySemanticMemoryRepository {
    concepts: Arc<RwLock<HashMap<Uuid, SemanticConcept>>>,
}

impl InMemorySemanticMemoryRepository {
    pub fn new() -> Self {
        Self {
            concepts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemorySemanticMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
impl SemanticMemoryRepository for InMemorySemanticMemoryRepository {
    async fn store_concept(&mut self, concept: SemanticConcept) -> Result<Uuid> {
        let id = concept.id;
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        concepts.insert(id, concept);
        Ok(id)
    }

    async fn get_concept(&self, id: Uuid) -> Result<Option<SemanticConcept>> {
        let concepts = self.concepts.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(concepts.get(&id).cloned())
    }

    async fn update_concept(&mut self, concept: &SemanticConcept) -> Result<()> {
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        concepts.insert(concept.id, concept.clone());
        Ok(())
    }

    async fn remove_concept(&mut self, id: Uuid) -> Result<()> {
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        concepts.remove(&id);
        Ok(())
    }

    async fn query_concepts(&self, query: &SemanticQuery) -> Result<Vec<SemanticConcept>> {
        let concepts = self.concepts.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let mut results: Vec<SemanticConcept> = concepts.values().cloned().collect();

        // Apply filters
        if let Some(ref pattern) = query.name_pattern {
            results.retain(|concept| concept.name.contains(pattern));
        }

        if let Some(min_confidence) = query.min_confidence {
            results.retain(|concept| concept.confidence >= min_confidence);
        }

        if let Some(ref embedding) = query.embedding {
            if let Some(min_similarity) = query.min_similarity {
                results.retain(|concept| {
                    cosine_similarity(&concept.embedding, embedding) >= min_similarity
                });
            }
        }

        // Sort by confidence (descending)
        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn find_similar(&self, embedding: &[f32], threshold: f64, limit: usize) -> Result<Vec<(Uuid, f64)>> {
        let concepts = self.concepts.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let mut similarities: Vec<(Uuid, f64)> = concepts
            .iter()
            .map(|(&id, concept)| {
                let similarity = cosine_similarity(&concept.embedding, embedding);
                (id, similarity)
            })
            .filter(|(_, similarity)| *similarity >= threshold)
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Apply limit
        similarities.truncate(limit);
        
        Ok(similarities)
    }

    async fn merge_concepts(&mut self, id1: Uuid, id2: Uuid) -> Result<Uuid> {
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        let concept1 = concepts.remove(&id1).ok_or_else(|| BrainError::NotFound("Concept 1 not found".to_string()))?;
        let concept2 = concepts.remove(&id2).ok_or_else(|| BrainError::NotFound("Concept 2 not found".to_string()))?;

        // Merge the concepts
        let merged_embedding: Vec<f32> = concept1.embedding
            .iter()
            .zip(concept2.embedding.iter())
            .map(|(a, b)| (a + b) / 2.0)
            .collect();

        let mut merged_concept = SemanticConcept::new(
            format!("{} + {}", concept1.name, concept2.name),
            format!("{} | {}", concept1.description, concept2.description),
            merged_embedding,
        );

        merged_concept.frequency = concept1.frequency + concept2.frequency;
        merged_concept.confidence = (concept1.confidence + concept2.confidence) / 2.0;
        
        // Combine source events
        merged_concept.source_events.extend(concept1.source_events);
        merged_concept.source_events.extend(concept2.source_events);

        let merged_id = merged_concept.id;
        concepts.insert(merged_id, merged_concept);
        
        Ok(merged_id)
    }

    async fn stats(&self) -> Result<MemoryStats> {
        let concepts = self.concepts.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let total_items = concepts.len();
        let size_bytes = total_items * std::mem::size_of::<SemanticConcept>();
        let last_access = concepts.values()
            .map(|concept| concept.last_updated)
            .max()
            .unwrap_or_else(Utc::now);

        Ok(MemoryStats {
            total_items,
            size_bytes,
            last_access,
            access_count: 0,
            consolidation_count: 0,
        })
    }
} 