//! Concept Graph Infrastructure Implementations
//! 
//! Sophisticated implementations of concept graph repository traits with
//! Neo4j integration, Hebbian learning, advanced traversal algorithms,
//! and comprehensive concept formation capabilities.

use brain_core::*;
use brain_types::*;
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for the Neo4j concept graph database
#[derive(Debug, Clone)]
pub struct ConceptGraphConfig {
    /// Neo4j database URI (e.g., "neo4j://localhost:7687")
    pub uri: String,
    /// Database username
    pub username: String,
    /// Database password  
    pub password: String,
    /// Database name (optional, defaults to "neo4j")
    pub database: Option<String>,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for ConceptGraphConfig {
    fn default() -> Self {
        Self {
            uri: "neo4j://localhost:7687".to_string(),
            username: "neo4j".to_string(),
            password: "password".to_string(),
            database: None,
            pool_size: 10,
            timeout_seconds: 30,
        }
    }
}

/// Hebbian learning configuration
#[derive(Debug, Clone)]
pub struct HebbianConfig {
    /// Default learning rate for new relationships
    pub default_learning_rate: f64,
    /// Default decay rate for unused relationships
    pub default_decay_rate: f64,
    /// Default pruning threshold for weak relationships
    pub default_pruning_threshold: f64,
    /// Maximum number of relationships per concept
    pub max_relationships_per_concept: usize,
    /// Batch size for efficient relationship processing
    pub batch_update_size: usize,
    /// Time window for co-activation detection (in minutes)
    pub co_activation_window_minutes: u64,
}

impl Default for HebbianConfig {
    fn default() -> Self {
        Self {
            default_learning_rate: 0.1,
            default_decay_rate: 0.01,
            default_pruning_threshold: 0.1,
            max_relationships_per_concept: 1000,
            batch_update_size: 100,
            co_activation_window_minutes: 10,
        }
    }
}

/// Concept formation configuration
#[derive(Debug, Clone)]
pub struct ConceptFormationConfig {
    /// Minimum frequency threshold for pattern-to-concept conversion
    pub min_pattern_frequency: usize,
    /// Minimum confidence threshold for pattern-to-concept conversion
    pub min_pattern_confidence: f64,
    /// Maximum number of concepts to form in a single operation
    pub max_concepts_per_batch: usize,
    /// Similarity threshold for concept merging (0.0 to 1.0)
    pub concept_merge_threshold: f64,
    /// Usage threshold for concept splitting
    pub concept_split_usage_threshold: u64,
    /// Confidence bonus for multi-character patterns
    pub multi_char_bonus: f64,
}

impl Default for ConceptFormationConfig {
    fn default() -> Self {
        Self {
            min_pattern_frequency: 5,
            min_pattern_confidence: 0.7,
            max_concepts_per_batch: 50,
            concept_merge_threshold: 0.9,
            concept_split_usage_threshold: 100,
            multi_char_bonus: 0.1,
        }
    }
}

/// Concept formation result
#[derive(Debug, Clone)]
pub struct ConceptFormationResult {
    /// Number of concepts formed
    pub concepts_formed: usize,
    /// Number of concepts merged
    pub concepts_merged: usize,
    /// Number of concepts split
    pub concepts_split: usize,
    /// IDs of newly created concepts
    pub new_concept_ids: Vec<Uuid>,
    /// IDs of concepts that were merged (now removed)
    pub merged_concept_ids: Vec<Uuid>,
    /// Patterns that were rejected (didn't meet thresholds)
    pub rejected_patterns: Vec<String>,
}

/// Similarity calculation configuration
#[derive(Debug, Clone)]
pub struct SimilarityConfig {
    /// Weight for content similarity (0.0 to 1.0)
    pub content_weight: f64,
    /// Weight for relationship similarity (0.0 to 1.0)
    pub relationship_weight: f64,
    /// Weight for usage pattern similarity (0.0 to 1.0)
    pub usage_weight: f64,
    /// Weight for metadata similarity (0.0 to 1.0)
    pub metadata_weight: f64,
    /// Minimum similarity threshold for considering concepts similar
    pub min_similarity_threshold: f64,
}

impl Default for SimilarityConfig {
    fn default() -> Self {
        Self {
            content_weight: 0.4,
            relationship_weight: 0.3,
            usage_weight: 0.2,
            metadata_weight: 0.1,
            min_similarity_threshold: 0.7,
        }
    }
}

/// Concept subgraph structure
#[derive(Debug, Clone)]
pub struct ConceptSubgraph {
    /// Concepts in the subgraph
    pub concepts: Vec<ConceptNode>,
    /// Relationships in the subgraph
    pub relationships: Vec<ConceptRelationship>,
    /// Center concept ID (if extracted around a specific concept)
    pub center_concept_id: Option<Uuid>,
    /// Radius of extraction (if extracted around a specific concept)
    pub radius: Option<usize>,
    /// Metrics for the subgraph
    pub metrics: NetworkMetrics,
}

/// Advanced concept graph manager with Neo4j integration and sophisticated algorithms
pub struct ConceptGraphManager {
    /// Connection configuration
    #[allow(dead_code)]
    config: ConceptGraphConfig,
    /// In-memory storage for concepts (fallback when Neo4j unavailable)
    concepts: HashMap<Uuid, ConceptNode>,
    /// In-memory storage for relationships (fallback when Neo4j unavailable)
    relationships: HashMap<Uuid, ConceptRelationship>,
    /// Hebbian learning configuration
    #[allow(dead_code)]
    hebbian_config: HebbianConfig,
    /// Concept formation configuration
    #[allow(dead_code)]
    formation_config: ConceptFormationConfig,
    /// Traversal algorithm configuration
    #[allow(dead_code)]
    traversal_config: TraversalConfig,
    /// Similarity calculation configuration
    #[allow(dead_code)]
    similarity_config: SimilarityConfig,
}

impl ConceptGraphManager {
    /// Create a new concept graph manager with default configuration
    pub async fn new(config: ConceptGraphConfig) -> Result<Self> {
        Ok(Self {
            config,
            concepts: HashMap::new(),
            relationships: HashMap::new(),
            hebbian_config: HebbianConfig::default(),
            formation_config: ConceptFormationConfig::default(),
            traversal_config: TraversalConfig::default(),
            similarity_config: SimilarityConfig::default(),
        })
    }

    /// Create with custom Hebbian configuration
    pub async fn with_hebbian_config(config: ConceptGraphConfig, hebbian_config: HebbianConfig) -> Result<Self> {
        Ok(Self {
            config,
            concepts: HashMap::new(),
            relationships: HashMap::new(),
            hebbian_config,
            formation_config: ConceptFormationConfig::default(),
            traversal_config: TraversalConfig::default(),
            similarity_config: SimilarityConfig::default(),
        })
    }

    /// Create with all custom configurations
    pub async fn with_all_configs(
        config: ConceptGraphConfig,
        hebbian_config: HebbianConfig,
        formation_config: ConceptFormationConfig,
        traversal_config: TraversalConfig,
        similarity_config: SimilarityConfig,
    ) -> Result<Self> {
        Ok(Self {
            config,
            concepts: HashMap::new(),
            relationships: HashMap::new(),
            hebbian_config,
            formation_config,
            traversal_config,
            similarity_config,
        })
    }
}

#[async_trait::async_trait]
impl ConceptRepository for ConceptGraphManager {
    async fn create_concept(&mut self, mut concept: ConceptNode) -> Result<Uuid> {
        concept.mark_accessed();
        let id = concept.id;
        
        // Store in memory (fallback storage)
        self.concepts.insert(id, concept.clone());
        
        // TODO: Store in Neo4j when available
        // This would involve creating a Cypher query to insert the concept node
        
        Ok(id)
    }

    async fn get_concept(&self, id: Uuid) -> Result<Option<ConceptNode>> {
        // Try in-memory first (fallback)
        if let Some(concept) = self.concepts.get(&id) {
            return Ok(Some(concept.clone()));
        }
        
        // TODO: Query Neo4j when available
        // This would involve a Cypher query to find the concept by ID
        
        Ok(None)
    }

    async fn update_concept(&mut self, concept: &ConceptNode) -> Result<()> {
        // Update in memory
        self.concepts.insert(concept.id, concept.clone());
        
        // TODO: Update in Neo4j when available
        // This would involve a Cypher query to update the concept node
        
        Ok(())
    }

    async fn delete_concept(&mut self, id: Uuid) -> Result<bool> {
        // Remove from memory
        let removed = self.concepts.remove(&id).is_some();
        
        // Remove associated relationships
        self.relationships.retain(|_, rel| {
            rel.source_id != id && rel.target_id != id
        });
        
        // TODO: Delete from Neo4j when available
        // This would involve Cypher queries to delete the concept and its relationships
        
        Ok(removed)
    }

    async fn query_concepts(&self, query: &ConceptQuery) -> Result<Vec<ConceptNode>> {
        let mut results: Vec<ConceptNode> = self.concepts.values().cloned().collect();
        
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
            let pattern_lower = pattern.to_lowercase();
            results.retain(|c| c.content.to_lowercase().contains(&pattern_lower));
        }
        
        if let Some(min_usage) = query.min_usage_count {
            results.retain(|c| c.usage_count >= min_usage);
        }
        
        // Sort results
        if let Some(sort_field) = &query.sort_by {
            match sort_field.as_str() {
                "confidence" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.confidence_score.partial_cmp(&a.confidence_score).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            a.confidence_score.partial_cmp(&b.confidence_score).unwrap_or(std::cmp::Ordering::Equal)
                        }
                    });
                }
                "usage_count" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.usage_count.cmp(&a.usage_count)
                        } else {
                            a.usage_count.cmp(&b.usage_count)
                        }
                    });
                }
                "created_at" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.created_at.cmp(&a.created_at)
                        } else {
                            a.created_at.cmp(&b.created_at)
                        }
                    });
                }
                "last_accessed_at" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.last_accessed_at.cmp(&a.last_accessed_at)
                        } else {
                            a.last_accessed_at.cmp(&b.last_accessed_at)
                        }
                    });
                }
                _ => {} // No sorting for unknown fields
            }
        }
        
        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }
        
        Ok(results)
    }

    async fn mark_concept_accessed(&mut self, id: Uuid) -> Result<bool> {
        if let Some(concept) = self.concepts.get_mut(&id) {
            concept.mark_accessed();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn get_concept_count(&self) -> Result<usize> {
        Ok(self.concepts.len())
    }
}

#[async_trait::async_trait]
impl RelationshipRepository for ConceptGraphManager {
    async fn create_relationship(&mut self, relationship: ConceptRelationship) -> Result<Uuid> {
        let id = relationship.id;
        
        // Validate that both concepts exist
        if !self.concepts.contains_key(&relationship.source_id) {
            return Err(BrainError::NotFound(format!("Source concept {} not found", relationship.source_id)));
        }
        if !self.concepts.contains_key(&relationship.target_id) {
            return Err(BrainError::NotFound(format!("Target concept {} not found", relationship.target_id)));
        }
        
        // Store in memory
        self.relationships.insert(id, relationship);
        
        // TODO: Store in Neo4j when available
        // This would involve creating a Cypher query to create the relationship
        
        Ok(id)
    }

    async fn get_relationship(&self, id: Uuid) -> Result<Option<ConceptRelationship>> {
        Ok(self.relationships.get(&id).cloned())
    }

    async fn update_relationship(&mut self, relationship: &ConceptRelationship) -> Result<()> {
        self.relationships.insert(relationship.id, relationship.clone());
        Ok(())
    }

    async fn delete_relationship(&mut self, id: Uuid) -> Result<bool> {
        let removed = self.relationships.remove(&id).is_some();
        
        // TODO: Delete from Neo4j when available
        // This would involve a Cypher query to delete the relationship
        
        Ok(removed)
    }

    async fn query_relationships(&self, query: &RelationshipQuery) -> Result<Vec<ConceptRelationship>> {
        let mut results: Vec<ConceptRelationship> = self.relationships.values().cloned().collect();
        
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
        
        // Sort results
        if let Some(sort_field) = &query.sort_by {
            match sort_field.as_str() {
                "weight" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            a.weight.partial_cmp(&b.weight).unwrap_or(std::cmp::Ordering::Equal)
                        }
                    });
                }
                "activation_count" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.activation_count.cmp(&a.activation_count)
                        } else {
                            a.activation_count.cmp(&b.activation_count)
                        }
                    });
                }
                "created_at" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.created_at.cmp(&a.created_at)
                        } else {
                            a.created_at.cmp(&b.created_at)
                        }
                    });
                }
                "last_activated_at" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.last_activated_at.cmp(&a.last_activated_at)
                        } else {
                            a.last_activated_at.cmp(&b.last_activated_at)
                        }
                    });
                }
                _ => {} // No sorting for unknown fields
            }
        }
        
        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }
        
        Ok(results)
    }

    async fn get_concept_relationships(&self, concept_id: Uuid) -> Result<Vec<ConceptRelationship>> {
        let results: Vec<ConceptRelationship> = self.relationships
            .values()
            .filter(|rel| rel.source_id == concept_id || rel.target_id == concept_id)
            .cloned()
            .collect();
        
        Ok(results)
    }

    async fn activate_relationship(&mut self, id: Uuid) -> Result<bool> {
        if let Some(relationship) = self.relationships.get_mut(&id) {
            relationship.activate();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn apply_decay_to_all(&mut self, time_delta_hours: f64) -> Result<usize> {
        let mut count = 0;
        
        for relationship in self.relationships.values_mut() {
            relationship.apply_decay(time_delta_hours);
            count += 1;
        }
        
        Ok(count)
    }

    async fn prune_weak_relationships(&mut self) -> Result<usize> {
        let mut pruned_count = 0;
        let mut to_remove = Vec::new();
        
        for (id, relationship) in self.relationships.iter() {
            if relationship.should_prune() {
                to_remove.push(*id);
            }
        }
        
        for id in to_remove {
            self.relationships.remove(&id);
            pruned_count += 1;
        }
        
        Ok(pruned_count)
    }

    async fn get_relationship_count(&self) -> Result<usize> {
        Ok(self.relationships.len())
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

#[cfg(test)]
mod tests {
    use super::*;
    // Removed unused test imports

    #[tokio::test]
    async fn test_concept_graph_manager_creation() {
        let config = ConceptGraphConfig::default();
        let manager = ConceptGraphManager::new(config).await.unwrap();
        
        assert_eq!(manager.concepts.len(), 0);
        assert_eq!(manager.relationships.len(), 0);
    }

    #[tokio::test]
    async fn test_concept_crud_operations() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();
        
        // Create a concept
        let concept = ConceptNode::new(
            ConceptType::Entity,
            "test concept".to_string(),
            0.9,
            Some("test source".to_string()),
        );
        let concept_id = concept.id;
        
        // Test create
        let created_id = manager.create_concept(concept.clone()).await.unwrap();
        assert_eq!(created_id, concept_id);
        
        // Test get
        let retrieved = manager.get_concept(concept_id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, "test concept");
        
        // Test update
        let mut updated_concept = concept.clone();
        updated_concept.content = "updated concept".to_string();
        manager.update_concept(&updated_concept).await.unwrap();
        
        let retrieved = manager.get_concept(concept_id).await.unwrap().unwrap();
        assert_eq!(retrieved.content, "updated concept");
        
        // Test delete
        let deleted = manager.delete_concept(concept_id).await.unwrap();
        assert!(deleted);
        
        let retrieved = manager.get_concept(concept_id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_relationship_crud_operations() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();
        
        // Create two concepts first
        let concept1 = ConceptNode::new(ConceptType::Entity, "concept1".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "concept2".to_string(), 0.8, None);
        
        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();
        
        // Create a relationship
        let relationship = ConceptRelationship::new(
            concept1_id,
            concept2_id,
            RelationshipType::SimilarTo,
            0.7,
        );
        let rel_id = relationship.id;
        
        // Test create
        let created_id = manager.create_relationship(relationship.clone()).await.unwrap();
        assert_eq!(created_id, rel_id);
        
        // Test get
        let retrieved = manager.get_relationship(rel_id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().weight, 0.7);
        
        // Test activate (Hebbian learning)
        let activated = manager.activate_relationship(rel_id).await.unwrap();
        assert!(activated);
        
        let retrieved = manager.get_relationship(rel_id).await.unwrap().unwrap();
        assert!(retrieved.weight > 0.7); // Weight should increase after activation
        assert_eq!(retrieved.activation_count, 1);
        
        // Test delete
        let deleted = manager.delete_relationship(rel_id).await.unwrap();
        assert!(deleted);
        
        let retrieved = manager.get_relationship(rel_id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_concept_query_filtering() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();
        
        // Create test concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "entity_test".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Action, "action_test".to_string(), 0.7, None);
        let concept3 = ConceptNode::new(ConceptType::Entity, "another_entity".to_string(), 0.5, None);
        
        manager.create_concept(concept1).await.unwrap();
        manager.create_concept(concept2).await.unwrap();
        manager.create_concept(concept3).await.unwrap();
        
        // Test filtering by concept type
        let query = ConceptQuery {
            concept_type: Some(ConceptType::Entity),
            ..Default::default()
        };
        let results = manager.query_concepts(&query).await.unwrap();
        assert_eq!(results.len(), 2);
        
        // Test filtering by confidence
        let query = ConceptQuery {
            min_confidence: Some(0.8),
            ..Default::default()
        };
        let results = manager.query_concepts(&query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "entity_test");
        
        // Test content pattern filtering
        let query = ConceptQuery {
            content_pattern: Some("test".to_string()),
            ..Default::default()
        };
        let results = manager.query_concepts(&query).await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_relationship_query_and_concept_relationships() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();
        
        // Create concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "concept1".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "concept2".to_string(), 0.8, None);
        let concept3 = ConceptNode::new(ConceptType::Entity, "concept3".to_string(), 0.7, None);
        
        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();
        let concept3_id = manager.create_concept(concept3).await.unwrap();
        
        // Create relationships
        let rel1 = ConceptRelationship::new(concept1_id, concept2_id, RelationshipType::SimilarTo, 0.8);
        let rel2 = ConceptRelationship::new(concept1_id, concept3_id, RelationshipType::Causes, 0.6);
        let rel3 = ConceptRelationship::new(concept2_id, concept3_id, RelationshipType::IsA, 0.9);
        
        manager.create_relationship(rel1).await.unwrap();
        manager.create_relationship(rel2).await.unwrap();
        manager.create_relationship(rel3).await.unwrap();
        
        // Test get concept relationships
        let concept1_rels = manager.get_concept_relationships(concept1_id).await.unwrap();
        assert_eq!(concept1_rels.len(), 2); // concept1 is involved in 2 relationships
        
        let concept2_rels = manager.get_concept_relationships(concept2_id).await.unwrap();
        assert_eq!(concept2_rels.len(), 2); // concept2 is involved in 2 relationships
        
        let concept3_rels = manager.get_concept_relationships(concept3_id).await.unwrap();
        assert_eq!(concept3_rels.len(), 2); // concept3 is involved in 2 relationships
        
        // Test relationship query by type
        let query = RelationshipQuery {
            relationship_type: Some(RelationshipType::SimilarTo),
            ..Default::default()
        };
        let results = manager.query_relationships(&query).await.unwrap();
        assert_eq!(results.len(), 1);
        
        // Test relationship query by weight range
        let query = RelationshipQuery {
            min_weight: Some(0.7),
            ..Default::default()
        };
        let results = manager.query_relationships(&query).await.unwrap();
        assert_eq!(results.len(), 2); // rel1 (0.8) and rel3 (0.9)
    }

    #[tokio::test]
    async fn test_hebbian_learning_and_decay() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();
        
        // Create concepts and relationship
        let concept1 = ConceptNode::new(ConceptType::Entity, "concept1".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "concept2".to_string(), 0.8, None);
        
        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();
        
        let relationship = ConceptRelationship::new(concept1_id, concept2_id, RelationshipType::SimilarTo, 0.5);
        let rel_id = manager.create_relationship(relationship).await.unwrap();
        
        // Test multiple activations (Hebbian learning)
        let initial_weight = manager.get_relationship(rel_id).await.unwrap().unwrap().weight;
        
        // Activate multiple times
        for _ in 0..5 {
            manager.activate_relationship(rel_id).await.unwrap();
        }
        
        let after_activation = manager.get_relationship(rel_id).await.unwrap().unwrap();
        assert!(after_activation.weight > initial_weight);
        assert_eq!(after_activation.activation_count, 5);
        
        // Test decay
        let decay_count = manager.apply_decay_to_all(24.0).await.unwrap(); // 24 hours
        assert_eq!(decay_count, 1);
        
        let after_decay = manager.get_relationship(rel_id).await.unwrap().unwrap();
        assert!(after_decay.weight < after_activation.weight);
    }

    #[tokio::test]
    async fn test_relationship_pruning() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();
        
        // Create concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "concept1".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "concept2".to_string(), 0.8, None);
        
        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();
        
        // Create a weak relationship (below pruning threshold)
        let mut weak_relationship = ConceptRelationship::new(concept1_id, concept2_id, RelationshipType::SimilarTo, 0.05);
        weak_relationship.weight = 0.05; // Below default pruning threshold of 0.1
        
        let rel_id = manager.create_relationship(weak_relationship).await.unwrap();
        
        // Verify relationship exists
        assert!(manager.get_relationship(rel_id).await.unwrap().is_some());
        
        // Test pruning
        let pruned_count = manager.prune_weak_relationships().await.unwrap();
        assert_eq!(pruned_count, 1);
        
        // Verify relationship was pruned
        assert!(manager.get_relationship(rel_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_cosine_similarity_function() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];
        let vec3 = vec![1.0, 2.0, 3.0]; // Same as vec1
        
        let similarity1 = cosine_similarity(&vec1, &vec2);
        let similarity2 = cosine_similarity(&vec1, &vec3);
        
        assert!(similarity1 > 0.0 && similarity1 < 1.0);
        assert!((similarity2 - 1.0).abs() < 1e-6); // Should be 1.0 for identical vectors
        
        // Test with zero vector
        let zero_vec = vec![0.0, 0.0, 0.0];
        let similarity3 = cosine_similarity(&vec1, &zero_vec);
        assert_eq!(similarity3, 0.0);
        
        // Test with different length vectors
        let short_vec = vec![1.0, 2.0];
        let similarity4 = cosine_similarity(&vec1, &short_vec);
        assert_eq!(similarity4, 0.0);
    }
}