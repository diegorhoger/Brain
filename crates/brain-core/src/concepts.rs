//! Concept Graph Domain Logic and Abstractions
//! 
//! This module defines core concept graph abstractions and domain logic
//! without any I/O dependencies. Infrastructure implementations are
//! provided through trait implementations.

use brain_types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Types of concept nodes in the graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConceptType {
    Entity,
    Action,
    Attribute,
    Abstract,
    Relation,
}

impl std::fmt::Display for ConceptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConceptType::Entity => write!(f, "Entity"),
            ConceptType::Action => write!(f, "Action"),
            ConceptType::Attribute => write!(f, "Attribute"),
            ConceptType::Abstract => write!(f, "Abstract"),
            ConceptType::Relation => write!(f, "Relation"),
        }
    }
}

/// Types of relationships between concept nodes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    IsA,
    PartOf,
    Causes,
    SimilarTo,
    Before,
    After,
    LocatedAt,
    Has,
    Uses,
    OppositeOf,
    AssociatedWith,
    Custom(String),
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationshipType::IsA => write!(f, "IS_A"),
            RelationshipType::PartOf => write!(f, "PART_OF"),
            RelationshipType::Causes => write!(f, "CAUSES"),
            RelationshipType::SimilarTo => write!(f, "SIMILAR_TO"),
            RelationshipType::Before => write!(f, "BEFORE"),
            RelationshipType::After => write!(f, "AFTER"),
            RelationshipType::LocatedAt => write!(f, "LOCATED_AT"),
            RelationshipType::Has => write!(f, "HAS"),
            RelationshipType::Uses => write!(f, "USES"),
            RelationshipType::OppositeOf => write!(f, "OPPOSITE_OF"),
            RelationshipType::AssociatedWith => write!(f, "ASSOCIATED_WITH"),
            RelationshipType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Concept node structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: Uuid,
    pub concept_type: ConceptType,
    pub content: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub usage_count: u64,
    pub confidence_score: f64,
    pub source_reference: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl ConceptNode {
    pub fn new(
        concept_type: ConceptType,
        content: String,
        confidence_score: f64,
        source_reference: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            concept_type,
            content,
            description: None,
            created_at: now,
            last_accessed_at: now,
            usage_count: 0,
            confidence_score: confidence_score.clamp(0.0, 1.0),
            source_reference,
            metadata: HashMap::new(),
        }
    }

    pub fn mark_accessed(&mut self) {
        self.last_accessed_at = Utc::now();
        self.usage_count += 1;
    }

    pub fn update_confidence(&mut self, score: f64) {
        self.confidence_score = score.clamp(0.0, 1.0);
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Relationship between concept nodes with Hebbian learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelationship {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub relationship_type: RelationshipType,
    pub weight: f64,
    pub activation_count: u64,
    pub created_at: DateTime<Utc>,
    pub last_activated_at: DateTime<Utc>,
    pub base_weight: f64,
    pub learning_rate: f64,
    pub decay_rate: f64,
    pub pruning_threshold: f64,
    pub metadata: HashMap<String, String>,
}

impl ConceptRelationship {
    pub fn new(
        source_id: Uuid,
        target_id: Uuid,
        relationship_type: RelationshipType,
        initial_weight: f64,
    ) -> Self {
        let now = Utc::now();
        let weight = initial_weight.clamp(0.0, 1.0);
        
        Self {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            relationship_type,
            weight,
            activation_count: 0,
            created_at: now,
            last_activated_at: now,
            base_weight: weight,
            learning_rate: 0.1,
            decay_rate: 0.01,
            pruning_threshold: 0.1,
            metadata: HashMap::new(),
        }
    }

    /// Activate relationship using Hebbian learning
    pub fn activate(&mut self) {
        self.activation_count += 1;
        self.last_activated_at = Utc::now();
        
        // Hebbian learning: weight increases with activation
        self.weight = (self.weight + self.learning_rate * (1.0 - self.weight)).clamp(0.0, 1.0);
    }

    /// Apply time-based decay to relationship weight
    pub fn apply_decay(&mut self, time_delta_hours: f64) {
        if time_delta_hours > 0.0 {
            let decay_factor = (-self.decay_rate * time_delta_hours).exp();
            self.weight = (self.weight * decay_factor).max(self.base_weight * 0.1);
        }
    }

    /// Check if relationship should be pruned
    pub fn should_prune(&self) -> bool {
        self.weight < self.pruning_threshold
    }

    pub fn configure_learning(&mut self, learning_rate: f64, decay_rate: f64, pruning_threshold: f64) {
        self.learning_rate = learning_rate.clamp(0.0, 1.0);
        self.decay_rate = decay_rate.clamp(0.0, 1.0);
        self.pruning_threshold = pruning_threshold.clamp(0.0, 1.0);
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Query structure for concepts
#[derive(Debug, Clone)]
pub struct ConceptQuery {
    pub concept_type: Option<ConceptType>,
    pub min_confidence: Option<f64>,
    pub max_confidence: Option<f64>,
    pub content_pattern: Option<String>,
    pub min_usage_count: Option<u64>,
    pub limit: Option<usize>,
    pub sort_by: Option<String>,
    pub descending: bool,
}

impl Default for ConceptQuery {
    fn default() -> Self {
        Self {
            concept_type: None,
            min_confidence: None,
            max_confidence: None,
            content_pattern: None,
            min_usage_count: None,
            limit: None,
            sort_by: None,
            descending: false,
        }
    }
}

/// Query structure for relationships
#[derive(Debug, Clone)]
pub struct RelationshipQuery {
    pub source_id: Option<Uuid>,
    pub target_id: Option<Uuid>,
    pub relationship_type: Option<RelationshipType>,
    pub min_weight: Option<f64>,
    pub max_weight: Option<f64>,
    pub min_activation_count: Option<u64>,
    pub limit: Option<usize>,
    pub sort_by: Option<String>,
    pub descending: bool,
}

impl Default for RelationshipQuery {
    fn default() -> Self {
        Self {
            source_id: None,
            target_id: None,
            relationship_type: None,
            min_weight: None,
            max_weight: None,
            min_activation_count: None,
            limit: None,
            sort_by: None,
            descending: false,
        }
    }
}

/// Traversal algorithms for graph navigation
#[derive(Debug, Clone)]
pub enum TraversalAlgorithm {
    BreadthFirst,
    DepthFirst,
    SpreadingActivation,
    ShortestPath,
}

/// Configuration for graph traversal
#[derive(Debug, Clone)]
pub struct TraversalConfig {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub min_relationship_weight: f64,
    pub activation_spread_factor: f64,
    pub activation_decay_factor: f64,
    pub follow_relationship_types: Vec<RelationshipType>,
}

impl Default for TraversalConfig {
    fn default() -> Self {
        Self {
            max_depth: 5,
            max_nodes: 100,
            min_relationship_weight: 0.1,
            activation_spread_factor: 0.8,
            activation_decay_factor: 0.9,
            follow_relationship_types: Vec::new(),
        }
    }
}

/// Result of graph traversal
#[derive(Debug, Clone)]
pub struct TraversalResult {
    pub start_concept_id: Uuid,
    pub algorithm: TraversalAlgorithm,
    pub visited_concepts: Vec<Uuid>,
    pub traversed_relationships: Vec<Uuid>,
    pub activation_scores: HashMap<Uuid, f64>,
    pub distances: HashMap<Uuid, usize>,
    pub total_nodes_visited: usize,
    pub max_depth_reached: usize,
}

/// Path between two concepts
#[derive(Debug, Clone)]
pub struct ConceptPath {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub concept_path: Vec<Uuid>,
    pub relationship_path: Vec<Uuid>,
    pub path_length: usize,
    pub total_weight: f64,
    pub average_weight: f64,
}

/// Network metrics for analysis
#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    pub total_relationships: usize,
    pub relationships_by_type: HashMap<RelationshipType, usize>,
    pub average_weight: f64,
    pub strong_relationships: usize,
    pub weak_relationships: usize,
    pub prunable_relationships: usize,
    pub average_degree: f64,
    pub isolated_concepts: usize,
    pub clustering_coefficient: f64,
    pub most_connected_concepts: Vec<(Uuid, usize)>,
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            total_relationships: 0,
            relationships_by_type: HashMap::new(),
            average_weight: 0.0,
            strong_relationships: 0,
            weak_relationships: 0,
            prunable_relationships: 0,
            average_degree: 0.0,
            isolated_concepts: 0,
            clustering_coefficient: 0.0,
            most_connected_concepts: Vec::new(),
        }
    }
}

/// Repository trait for concept nodes
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait ConceptRepository: Send + Sync {
    async fn create_concept(&mut self, concept: ConceptNode) -> Result<Uuid>;
    async fn get_concept(&self, id: Uuid) -> Result<Option<ConceptNode>>;
    async fn update_concept(&mut self, concept: &ConceptNode) -> Result<()>;
    async fn delete_concept(&mut self, id: Uuid) -> Result<bool>;
    async fn query_concepts(&self, query: &ConceptQuery) -> Result<Vec<ConceptNode>>;
    async fn mark_concept_accessed(&mut self, id: Uuid) -> Result<bool>;
    async fn get_concept_count(&self) -> Result<usize>;
}

/// Repository trait for concept relationships
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait RelationshipRepository: Send + Sync {
    async fn create_relationship(&mut self, relationship: ConceptRelationship) -> Result<Uuid>;
    async fn get_relationship(&self, id: Uuid) -> Result<Option<ConceptRelationship>>;
    async fn update_relationship(&mut self, relationship: &ConceptRelationship) -> Result<()>;
    async fn delete_relationship(&mut self, id: Uuid) -> Result<bool>;
    async fn query_relationships(&self, query: &RelationshipQuery) -> Result<Vec<ConceptRelationship>>;
    async fn get_concept_relationships(&self, concept_id: Uuid) -> Result<Vec<ConceptRelationship>>;
    async fn activate_relationship(&mut self, id: Uuid) -> Result<bool>;
    async fn apply_decay_to_all(&mut self, time_delta_hours: f64) -> Result<usize>;
    async fn prune_weak_relationships(&mut self) -> Result<usize>;
    async fn get_relationship_count(&self) -> Result<usize>;
}

/// Concept graph service coordinating concepts and relationships
pub struct ConceptGraphService {
    concept_repo: Box<dyn ConceptRepository>,
    relationship_repo: Box<dyn RelationshipRepository>,
}

impl ConceptGraphService {
    pub fn new(
        concept_repo: Box<dyn ConceptRepository>,
        relationship_repo: Box<dyn RelationshipRepository>,
    ) -> Self {
        Self {
            concept_repo,
            relationship_repo,
        }
    }

    pub async fn create_concept(&mut self, concept: ConceptNode) -> Result<Uuid> {
        self.concept_repo.create_concept(concept).await
    }

    pub async fn get_concept(&self, id: Uuid) -> Result<Option<ConceptNode>> {
        self.concept_repo.get_concept(id).await
    }

    pub async fn create_relationship(
        &mut self,
        source_id: Uuid,
        target_id: Uuid,
        relationship_type: RelationshipType,
        initial_weight: f64,
    ) -> Result<Uuid> {
        // Verify both concepts exist
        let source_exists = self.concept_repo.get_concept(source_id).await?.is_some();
        let target_exists = self.concept_repo.get_concept(target_id).await?.is_some();

        if !source_exists || !target_exists {
            return Err(BrainError::NotFound("One or both concepts not found".to_string()));
        }

        let relationship = ConceptRelationship::new(source_id, target_id, relationship_type, initial_weight);
        self.relationship_repo.create_relationship(relationship).await
    }

    pub async fn activate_relationship(&mut self, id: Uuid) -> Result<bool> {
        self.relationship_repo.activate_relationship(id).await
    }

    pub async fn co_activate_concepts(&mut self, concept_id1: Uuid, concept_id2: Uuid) -> Result<usize> {
        // Find relationships between the concepts
        let query = RelationshipQuery {
            source_id: Some(concept_id1),
            target_id: Some(concept_id2),
            ..Default::default()
        };

        let relationships = self.relationship_repo.query_relationships(&query).await?;
        let mut activated_count = 0;

        for relationship in relationships {
            if self.relationship_repo.activate_relationship(relationship.id).await? {
                activated_count += 1;
            }
        }

        // Also check reverse direction
        let reverse_query = RelationshipQuery {
            source_id: Some(concept_id2),
            target_id: Some(concept_id1),
            ..Default::default()
        };

        let reverse_relationships = self.relationship_repo.query_relationships(&reverse_query).await?;
        for relationship in reverse_relationships {
            if self.relationship_repo.activate_relationship(relationship.id).await? {
                activated_count += 1;
            }
        }

        Ok(activated_count)
    }

    pub async fn find_shortest_path(&self, _source_id: Uuid, _target_id: Uuid) -> Result<Option<ConceptPath>> {
        // This would implement Dijkstra's algorithm or similar
        // For now, return None as placeholder
        Ok(None)
    }

    pub async fn traverse_graph(
        &self,
        start_concept_id: Uuid,
        algorithm: TraversalAlgorithm,
        config: Option<TraversalConfig>,
    ) -> Result<TraversalResult> {
        let config = config.unwrap_or_default();

        match algorithm {
            TraversalAlgorithm::BreadthFirst => self.breadth_first_search(start_concept_id, &config).await,
            TraversalAlgorithm::DepthFirst => self.depth_first_search(start_concept_id, &config).await,
            TraversalAlgorithm::SpreadingActivation => self.spreading_activation_search(start_concept_id, &config).await,
            TraversalAlgorithm::ShortestPath => {
                // For shortest path, we need a target - use BFS as fallback
                self.breadth_first_search(start_concept_id, &config).await
            }
        }
    }

    async fn breadth_first_search(&self, start_concept_id: Uuid, _config: &TraversalConfig) -> Result<TraversalResult> {
        // Placeholder implementation
        Ok(TraversalResult {
            start_concept_id,
            algorithm: TraversalAlgorithm::BreadthFirst,
            visited_concepts: vec![start_concept_id],
            traversed_relationships: Vec::new(),
            activation_scores: HashMap::new(),
            distances: HashMap::new(),
            total_nodes_visited: 1,
            max_depth_reached: 0,
        })
    }

    async fn depth_first_search(&self, start_concept_id: Uuid, _config: &TraversalConfig) -> Result<TraversalResult> {
        // Placeholder implementation
        Ok(TraversalResult {
            start_concept_id,
            algorithm: TraversalAlgorithm::DepthFirst,
            visited_concepts: vec![start_concept_id],
            traversed_relationships: Vec::new(),
            activation_scores: HashMap::new(),
            distances: HashMap::new(),
            total_nodes_visited: 1,
            max_depth_reached: 0,
        })
    }

    async fn spreading_activation_search(&self, start_concept_id: Uuid, _config: &TraversalConfig) -> Result<TraversalResult> {
        // Placeholder implementation
        let mut activation_scores = HashMap::new();
        activation_scores.insert(start_concept_id, 1.0);

        Ok(TraversalResult {
            start_concept_id,
            algorithm: TraversalAlgorithm::SpreadingActivation,
            visited_concepts: vec![start_concept_id],
            traversed_relationships: Vec::new(),
            activation_scores,
            distances: HashMap::new(),
            total_nodes_visited: 1,
            max_depth_reached: 0,
        })
    }

    pub async fn get_network_metrics(&self) -> Result<NetworkMetrics> {
        // This would calculate comprehensive network metrics
        // For now, return default metrics
        Ok(NetworkMetrics::default())
    }

    pub async fn calculate_concept_similarity(&self, concept1_id: Uuid, concept2_id: Uuid) -> Result<f64> {
        let concept1 = self.concept_repo.get_concept(concept1_id).await?;
        let concept2 = self.concept_repo.get_concept(concept2_id).await?;

        match (concept1, concept2) {
            (Some(c1), Some(c2)) => {
                // Simple string similarity for now
                Ok(self.calculate_string_similarity(&c1.content, &c2.content))
            }
            _ => Ok(0.0),
        }
    }

    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        if s1_chars.is_empty() || s2_chars.is_empty() {
            return 0.0;
        }

        // Simple Jaccard similarity on character bigrams
        let s1_bigrams: std::collections::HashSet<_> = s1_chars.windows(2).collect();
        let s2_bigrams: std::collections::HashSet<_> = s2_chars.windows(2).collect();

        let intersection_size = s1_bigrams.intersection(&s2_bigrams).count();
        let union_size = s1_bigrams.union(&s2_bigrams).count();

        if union_size == 0 {
            0.0
        } else {
            intersection_size as f64 / union_size as f64
        }
    }
}
