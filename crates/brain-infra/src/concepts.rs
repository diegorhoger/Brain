//! Concept Infrastructure Implementations
//! 
//! This module provides in-memory implementations of concept-related repositories
//! for development and testing purposes.

use brain_core::*;
use brain_types::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// In-memory implementation of ConceptRepository
pub struct InMemoryConceptRepository {
    concepts: Arc<RwLock<HashMap<Uuid, ConceptNode>>>,
    relationships: Arc<RwLock<HashMap<Uuid, ConceptRelationship>>>,
    concept_relationships: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,
}

impl InMemoryConceptRepository {
    pub fn new() -> Self {
        Self {
            concepts: Arc::new(RwLock::new(HashMap::new())),
            relationships: Arc::new(RwLock::new(HashMap::new())),
            concept_relationships: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryConceptRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ConceptRepository for InMemoryConceptRepository {
    async fn create_concept(&mut self, concept: ConceptNode) -> Result<Uuid> {
        let id = concept.id;
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        concepts.insert(id, concept);
        Ok(id)
    }

    async fn get_concept(&self, id: Uuid) -> Result<Option<ConceptNode>> {
        let concepts = self.concepts.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(concepts.get(&id).cloned())
    }

    async fn update_concept(&mut self, concept: &ConceptNode) -> Result<()> {
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        concepts.insert(concept.id, concept.clone());
        Ok(())
    }

    async fn delete_concept(&mut self, id: Uuid) -> Result<bool> {
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        let mut concept_relationships = self.concept_relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        // Remove the concept
        let removed = concepts.remove(&id).is_some();
        
        // Clean up relationships
        concept_relationships.remove(&id);
        
        Ok(removed)
    }

    async fn query_concepts(&self, query: &ConceptQuery) -> Result<Vec<ConceptNode>> {
        let concepts = self.concepts.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let mut results: Vec<ConceptNode> = concepts.values().cloned().collect();
        
        // Apply filters
        if let Some(concept_type) = &query.concept_type {
            results.retain(|c| &c.concept_type == concept_type);
        }
        
        if let Some(min_confidence) = query.min_confidence {
            results.retain(|c| c.confidence_score >= min_confidence);
        }
        
        if let Some(max_confidence) = query.max_confidence {
            results.retain(|c| c.confidence_score <= max_confidence);
        }
        
        if let Some(pattern) = &query.content_pattern {
            results.retain(|c| c.content.contains(pattern));
        }
        
        if let Some(min_usage) = query.min_usage_count {
            results.retain(|c| c.usage_count >= min_usage);
        }
        
        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }
        
        Ok(results)
    }

    async fn mark_concept_accessed(&mut self, id: Uuid) -> Result<bool> {
        let mut concepts = self.concepts.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(concept) = concepts.get_mut(&id) {
            concept.mark_accessed();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn get_concept_count(&self) -> Result<usize> {
        let concepts = self.concepts.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(concepts.len())
    }
}

#[async_trait::async_trait]
impl RelationshipRepository for InMemoryConceptRepository {
    async fn create_relationship(&mut self, relationship: ConceptRelationship) -> Result<Uuid> {
        let id = relationship.id;
        let source_id = relationship.source_id;
        let target_id = relationship.target_id;
        
        let mut relationships = self.relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        let mut concept_relationships = self.concept_relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        relationships.insert(id, relationship);
        
        // Update concept relationship mappings
        concept_relationships.entry(source_id).or_insert_with(Vec::new).push(id);
        concept_relationships.entry(target_id).or_insert_with(Vec::new).push(id);
        
        Ok(id)
    }

    async fn get_relationship(&self, id: Uuid) -> Result<Option<ConceptRelationship>> {
        let relationships = self.relationships.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(relationships.get(&id).cloned())
    }

    async fn update_relationship(&mut self, relationship: &ConceptRelationship) -> Result<()> {
        let mut relationships = self.relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        relationships.insert(relationship.id, relationship.clone());
        Ok(())
    }

    async fn delete_relationship(&mut self, id: Uuid) -> Result<bool> {
        let mut relationships = self.relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        let mut concept_relationships = self.concept_relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(relationship) = relationships.remove(&id) {
            // Clean up concept relationship mappings
            if let Some(source_rels) = concept_relationships.get_mut(&relationship.source_id) {
                source_rels.retain(|&rel_id| rel_id != id);
            }
            if let Some(target_rels) = concept_relationships.get_mut(&relationship.target_id) {
                target_rels.retain(|&rel_id| rel_id != id);
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn query_relationships(&self, query: &RelationshipQuery) -> Result<Vec<ConceptRelationship>> {
        let relationships = self.relationships.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let mut results: Vec<ConceptRelationship> = relationships.values().cloned().collect();
        
        // Apply filters
        if let Some(source_id) = query.source_id {
            results.retain(|r| r.source_id == source_id);
        }
        
        if let Some(target_id) = query.target_id {
            results.retain(|r| r.target_id == target_id);
        }
        
        if let Some(rel_type) = &query.relationship_type {
            results.retain(|r| &r.relationship_type == rel_type);
        }
        
        if let Some(min_weight) = query.min_weight {
            results.retain(|r| r.weight >= min_weight);
        }
        
        if let Some(max_weight) = query.max_weight {
            results.retain(|r| r.weight <= max_weight);
        }
        
        if let Some(min_activation) = query.min_activation_count {
            results.retain(|r| r.activation_count >= min_activation);
        }
        
        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }
        
        Ok(results)
    }

    async fn get_concept_relationships(&self, concept_id: Uuid) -> Result<Vec<ConceptRelationship>> {
        let relationships = self.relationships.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let concept_relationships = self.concept_relationships.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let mut results = Vec::new();
        
        if let Some(relationship_ids) = concept_relationships.get(&concept_id) {
            for &rel_id in relationship_ids {
                if let Some(relationship) = relationships.get(&rel_id) {
                    results.push(relationship.clone());
                }
            }
        }
        
        Ok(results)
    }

    async fn activate_relationship(&mut self, id: Uuid) -> Result<bool> {
        let mut relationships = self.relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(relationship) = relationships.get_mut(&id) {
            relationship.activate();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn apply_decay_to_all(&mut self, time_delta_hours: f64) -> Result<usize> {
        let mut relationships = self.relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        let mut count = 0;
        
        for relationship in relationships.values_mut() {
            relationship.apply_decay(time_delta_hours);
            count += 1;
        }
        
        Ok(count)
    }

    async fn prune_weak_relationships(&mut self) -> Result<usize> {
        let mut relationships = self.relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        let mut concept_relationships = self.concept_relationships.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        let mut pruned_count = 0;
        let mut to_remove = Vec::new();
        
        for (id, relationship) in relationships.iter() {
            if relationship.should_prune() {
                to_remove.push((*id, relationship.source_id, relationship.target_id));
            }
        }
        
        for (id, source_id, target_id) in to_remove {
            relationships.remove(&id);
            
            // Clean up concept relationship mappings
            if let Some(source_rels) = concept_relationships.get_mut(&source_id) {
                source_rels.retain(|&rel_id| rel_id != id);
            }
            if let Some(target_rels) = concept_relationships.get_mut(&target_id) {
                target_rels.retain(|&rel_id| rel_id != id);
            }
            
            pruned_count += 1;
        }
        
        Ok(pruned_count)
    }

    async fn get_relationship_count(&self) -> Result<usize> {
        let relationships = self.relationships.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(relationships.len())
    }
}

// Helper function for calculating cosine similarity between embeddings
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() {
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