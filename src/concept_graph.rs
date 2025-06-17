//! Concept Graph Engine for Neo4j-based concept management and relationships
//! 
//! This module implements:
//! - Task 4.1: Set up Neo4j database and core concept node structure
//! - Task 4.2: Implement relationship management and Hebbian learning mechanism  
//! - Task 4.3: Develop concept formation and graph traversal algorithms
//! following the same architectural patterns as the memory module.

use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::segment_discovery::{BpeSegmenter, SegmentStats};

/// Configuration for the Neo4j concept graph database
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Types of concept nodes in the graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConceptType {
    /// Represents entities (nouns, objects, people, places)
    Entity,
    /// Represents actions (verbs, processes, behaviors)
    Action,
    /// Represents attributes (adjectives, properties, qualities)
    Attribute,
    /// Represents abstract concepts (ideas, emotions, states)
    Abstract,
    /// Represents relationships or connections between concepts
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
    /// Hierarchical relationship: A is a type of B
    IsA,
    /// Part-whole relationship: A is part of B
    PartOf,
    /// Causal relationship: A causes B
    Causes,
    /// Similarity relationship: A is similar to B
    SimilarTo,
    /// Temporal relationship: A happens before B
    Before,
    /// Temporal relationship: A happens after B
    After,
    /// Spatial relationship: A is located in/at B
    LocatedAt,
    /// Ownership relationship: A owns/has B
    Has,
    /// Usage relationship: A uses B
    Uses,
    /// Opposite relationship: A is opposite to B
    OppositeOf,
    /// Associated relationship: A is associated with B
    AssociatedWith,
    /// Custom relationship type for extensibility
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

/// Relationship between two concept nodes with Hebbian learning properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelationship {
    /// Unique identifier for the relationship
    pub id: Uuid,
    /// Source concept ID
    pub source_id: Uuid,
    /// Target concept ID
    pub target_id: Uuid,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Connection strength (0.0 to 1.0) - increases with co-activation
    pub weight: f64,
    /// Number of times this relationship has been activated
    pub activation_count: u64,
    /// Timestamp when the relationship was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the relationship was last activated
    pub last_activated_at: DateTime<Utc>,
    /// Base weight before Hebbian learning adjustments
    pub base_weight: f64,
    /// Learning rate for Hebbian updates (0.0 to 1.0)
    pub learning_rate: f64,
    /// Decay rate for unused connections (0.0 to 1.0)
    pub decay_rate: f64,
    /// Minimum threshold below which relationship is considered weak
    pub pruning_threshold: f64,
    /// Additional metadata for the relationship
    pub metadata: HashMap<String, String>,
}

impl ConceptRelationship {
    /// Create a new relationship with Hebbian learning properties
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
            learning_rate: 0.1, // Default learning rate
            decay_rate: 0.01,   // Default decay rate
            pruning_threshold: 0.1, // Default pruning threshold
            metadata: HashMap::new(),
        }
    }

    /// Activate the relationship using Hebbian learning
    /// Increases weight based on co-activation frequency
    pub fn activate(&mut self) {
        self.activation_count += 1;
        self.last_activated_at = Utc::now();
        
        // Hebbian learning: weight increases with activation
        // New weight = old weight + learning_rate * (1 - old weight)
        self.weight = (self.weight + self.learning_rate * (1.0 - self.weight)).clamp(0.0, 1.0);
    }

    /// Apply time-based decay to the relationship weight
    pub fn apply_decay(&mut self, time_delta_hours: f64) {
        if time_delta_hours > 0.0 {
            // Exponential decay: weight *= exp(-decay_rate * time)
            let decay_factor = (-self.decay_rate * time_delta_hours).exp();
            self.weight = (self.weight * decay_factor).max(self.base_weight * 0.1);
        }
    }

    /// Check if the relationship is below the pruning threshold
    pub fn should_prune(&self) -> bool {
        self.weight < self.pruning_threshold
    }

    /// Update the learning parameters
    pub fn configure_learning(&mut self, learning_rate: f64, decay_rate: f64, pruning_threshold: f64) {
        self.learning_rate = learning_rate.clamp(0.001, 1.0);
        self.decay_rate = decay_rate.clamp(0.0, 1.0);
        self.pruning_threshold = pruning_threshold.clamp(0.0, 1.0);
    }

    /// Get the inverse relationship type (if applicable)
    pub fn get_inverse_type(&self) -> Option<RelationshipType> {
        match &self.relationship_type {
            RelationshipType::Before => Some(RelationshipType::After),
            RelationshipType::After => Some(RelationshipType::Before),
            RelationshipType::Has => Some(RelationshipType::PartOf),
            RelationshipType::PartOf => Some(RelationshipType::Has),
            RelationshipType::Causes => None, // Causes is not typically bidirectional
            RelationshipType::OppositeOf => Some(RelationshipType::OppositeOf),
            _ => None, // Most relationships don't have clear inverses
        }
    }

    /// Add or update metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Configuration for Hebbian learning and relationship management
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            co_activation_window_minutes: 5,
        }
    }
}

/// Query parameters for filtering relationships
#[derive(Debug, Clone, Default)]
pub struct RelationshipQuery {
    /// Filter by source concept ID
    pub source_id: Option<Uuid>,
    /// Filter by target concept ID
    pub target_id: Option<Uuid>,
    /// Filter by relationship type
    pub relationship_type: Option<RelationshipType>,
    /// Filter by minimum weight
    pub min_weight: Option<f64>,
    /// Filter by maximum weight
    pub max_weight: Option<f64>,
    /// Filter by minimum activation count
    pub min_activation_count: Option<u64>,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Sort by field ("weight", "activation_count", "created_at", "last_activated_at")
    pub sort_by: Option<String>,
    /// Sort in descending order (default: ascending)
    pub descending: bool,
}

/// Network connectivity and relationship strength metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Total number of relationships in the network
    pub total_relationships: usize,
    /// Number of relationships by type
    pub relationships_by_type: HashMap<RelationshipType, usize>,
    /// Average relationship weight across all relationships
    pub average_weight: f64,
    /// Number of strong relationships (weight >= 0.7)
    pub strong_relationships: usize,
    /// Number of weak relationships (weight < 0.3)
    pub weak_relationships: usize,
    /// Number of relationships eligible for pruning
    pub prunable_relationships: usize,
    /// Average degree (number of connections per concept)
    pub average_degree: f64,
    /// Number of isolated concepts (no relationships)
    pub isolated_concepts: usize,
    /// Clustering coefficient of the network
    pub clustering_coefficient: f64,
    /// Most connected concepts (concept ID, degree)
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

/// Core concept node structure with essential properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    /// Unique identifier for the concept
    pub id: Uuid,
    /// Type of concept (entity, action, attribute, etc.)
    pub concept_type: ConceptType,
    /// Primary content or label of the concept
    pub content: String,
    /// Optional description providing more context
    pub description: Option<String>,
    /// Timestamp when the concept was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the concept was last accessed
    pub last_accessed_at: DateTime<Utc>,
    /// Number of times this concept has been accessed or used
    pub usage_count: u64,
    /// Confidence score (0.0 to 1.0) indicating certainty about this concept
    pub confidence_score: f64,
    /// Reference to the source (memory, segment, etc.) that generated this concept
    pub source_reference: Option<String>,
    /// Additional metadata as key-value pairs
    pub metadata: HashMap<String, String>,
}

impl ConceptNode {
    /// Create a new concept node with essential properties
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

    /// Update the last accessed timestamp and increment usage count
    pub fn mark_accessed(&mut self) {
        self.last_accessed_at = Utc::now();
        self.usage_count += 1;
    }

    /// Update the confidence score (clamped to 0.0-1.0 range)
    pub fn update_confidence(&mut self, score: f64) {
        self.confidence_score = score.clamp(0.0, 1.0);
    }

    /// Add or update metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Statistics for the concept graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptGraphStats {
    /// Total number of concept nodes
    pub total_concepts: usize,
    /// Number of concepts by type
    pub concepts_by_type: HashMap<ConceptType, usize>,
    /// Average confidence score across all concepts
    pub average_confidence: f64,
    /// Total number of relationships
    pub total_relationships: usize,
    /// Number of relationships by type
    pub relationships_by_type: HashMap<RelationshipType, usize>,
    /// Average relationship weight
    pub average_relationship_weight: f64,
    /// Number of high-confidence concepts (confidence >= 0.8)
    pub high_confidence_concepts: usize,
    /// Number of strong relationships (weight >= 0.7)
    pub strong_relationships: usize,
    /// Most recently created concept age in seconds
    pub newest_concept_age_seconds: Option<i64>,
    /// Most recently accessed concept age in seconds
    pub last_access_age_seconds: Option<i64>,
}

impl Default for ConceptGraphStats {
    fn default() -> Self {
        Self {
            total_concepts: 0,
            concepts_by_type: HashMap::new(),
            average_confidence: 0.0,
            total_relationships: 0,
            relationships_by_type: HashMap::new(),
            average_relationship_weight: 0.0,
            high_confidence_concepts: 0,
            strong_relationships: 0,
            newest_concept_age_seconds: None,
            last_access_age_seconds: None,
        }
    }
}

/// Query parameters for filtering concept nodes
#[derive(Debug, Clone, Default)]
pub struct ConceptQuery {
    /// Filter by concept type
    pub concept_type: Option<ConceptType>,
    /// Filter by minimum confidence score
    pub min_confidence: Option<f64>,
    /// Filter by maximum confidence score
    pub max_confidence: Option<f64>,
    /// Filter by content pattern (case-insensitive contains)
    pub content_pattern: Option<String>,
    /// Filter by minimum usage count
    pub min_usage_count: Option<u64>,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Sort by field ("confidence", "usage_count", "created_at", "last_accessed_at")
    pub sort_by: Option<String>,
    /// Sort in descending order (default: ascending)
    pub descending: bool,
}



/// Configuration for concept formation from segment patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            min_pattern_confidence: 0.6,
            max_concepts_per_batch: 100,
            concept_merge_threshold: 0.8,
            concept_split_usage_threshold: 1000,
            multi_char_bonus: 0.1,
        }
    }
}

/// Result from concept formation process
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Traversal algorithm types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraversalAlgorithm {
    /// Breadth-first search
    BreadthFirst,
    /// Depth-first search
    DepthFirst,
    /// Spreading activation algorithm
    SpreadingActivation,
    /// Dijkstra's shortest path
    ShortestPath,
}

/// Configuration for graph traversal algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalConfig {
    /// Maximum depth for traversal
    pub max_depth: usize,
    /// Maximum number of nodes to visit
    pub max_nodes: usize,
    /// Minimum relationship weight to follow
    pub min_relationship_weight: f64,
    /// Activation spread factor for spreading activation
    pub activation_spread_factor: f64,
    /// Decay factor for spreading activation
    pub activation_decay_factor: f64,
    /// Relationship types to follow (empty = all types)
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

/// Result from graph traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalResult {
    /// Starting concept ID
    pub start_concept_id: Uuid,
    /// Algorithm used for traversal
    pub algorithm: TraversalAlgorithm,
    /// Concepts visited in order
    pub visited_concepts: Vec<Uuid>,
    /// Relationships traversed
    pub traversed_relationships: Vec<Uuid>,
    /// Activation scores for each concept (for spreading activation)
    pub activation_scores: HashMap<Uuid, f64>,
    /// Path lengths from start node
    pub distances: HashMap<Uuid, usize>,
    /// Total nodes visited
    pub total_nodes_visited: usize,
    /// Maximum depth reached
    pub max_depth_reached: usize,
}

/// Configuration for concept similarity calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Path between two concepts in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptPath {
    /// Source concept ID
    pub source_id: Uuid,
    /// Target concept ID
    pub target_id: Uuid,
    /// Concept IDs in the path (including source and target)
    pub concept_path: Vec<Uuid>,
    /// Relationship IDs in the path
    pub relationship_path: Vec<Uuid>,
    /// Total path length
    pub path_length: usize,
    /// Combined weight of all relationships in the path
    pub total_weight: f64,
    /// Average weight of relationships in the path
    pub average_weight: f64,
}

/// Subgraph extracted from the main concept graph
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Main concept graph manager with Neo4j database operations
/// 
/// Note: This implementation provides the foundation for Neo4j integration.
/// Full Neo4j functionality will be implemented in future iterations.
#[derive(Debug)]
pub struct ConceptGraphManager {
    /// Connection configuration
    config: ConceptGraphConfig,
    /// In-memory storage for concepts (will be replaced with Neo4j in full implementation)
    concepts: HashMap<Uuid, ConceptNode>,
    /// In-memory storage for relationships (will be replaced with Neo4j in full implementation)
    relationships: HashMap<Uuid, ConceptRelationship>,
    /// Hebbian learning configuration
    hebbian_config: HebbianConfig,
    /// Concept formation configuration
    formation_config: ConceptFormationConfig,
    /// Traversal algorithm configuration
    traversal_config: TraversalConfig,
    /// Similarity calculation configuration
    similarity_config: SimilarityConfig,
}

impl ConceptGraphManager {
    /// Create a new concept graph manager with default configuration
    /// This initializes the in-memory storage for concepts and relationships
    /// In a full implementation, this would establish Neo4j connection
    pub async fn new(config: ConceptGraphConfig) -> Result<Self> {
        // In a full implementation, we would connect to Neo4j here
        // For now, we use in-memory HashMaps for educational purposes
        
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

    /// Create a new concept graph manager with custom Hebbian configuration
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

    /// Create a new concept graph manager with all custom configurations
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

    /// Create a new concept node in the database
    pub async fn create_concept(&mut self, mut concept: ConceptNode) -> Result<Uuid> {
        // Ensure the concept has a valid ID
        if concept.id.is_nil() {
            concept.id = Uuid::new_v4();
        }

        let concept_id = concept.id;
        self.concepts.insert(concept_id, concept);

        Ok(concept_id)
    }

    /// Get a concept node by its ID
    pub async fn get_concept(&self, id: Uuid) -> Result<Option<ConceptNode>> {
        Ok(self.concepts.get(&id).cloned())
    }

    /// Update a concept node
    pub async fn update_concept(&mut self, concept: &ConceptNode) -> Result<()> {
        self.concepts.insert(concept.id, concept.clone());
        Ok(())
    }

    /// Delete a concept node
    pub async fn delete_concept(&mut self, id: Uuid) -> Result<bool> {
        Ok(self.concepts.remove(&id).is_some())
    }

    /// Mark a concept as accessed (updates usage count and last accessed time)
    pub async fn mark_concept_accessed(&mut self, id: Uuid) -> Result<bool> {
        if let Some(concept) = self.concepts.get_mut(&id) {
            concept.mark_accessed();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Query concepts with filtering and sorting
    pub async fn query_concepts(&self, query_params: &ConceptQuery) -> Result<Vec<ConceptNode>> {
        let mut concepts: Vec<ConceptNode> = self.concepts.values()
            .filter(|concept| {
                // Filter by concept type
                if let Some(ref expected_type) = query_params.concept_type {
                    if &concept.concept_type != expected_type {
                        return false;
                    }
                }

                // Filter by minimum confidence
                if let Some(min_confidence) = query_params.min_confidence {
                    if concept.confidence_score < min_confidence {
                        return false;
                    }
                }

                // Filter by maximum confidence
                if let Some(max_confidence) = query_params.max_confidence {
                    if concept.confidence_score > max_confidence {
                        return false;
                    }
                }

                // Filter by content pattern
                if let Some(ref pattern) = query_params.content_pattern {
                    if !concept.content.to_lowercase().contains(&pattern.to_lowercase()) {
                        return false;
                    }
                }

                // Filter by minimum usage count
                if let Some(min_usage) = query_params.min_usage_count {
                    if concept.usage_count < min_usage {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        // Sort results
        if let Some(ref sort_field) = query_params.sort_by {
            concepts.sort_by(|a, b| {
                let ordering = match sort_field.as_str() {
                    "confidence" | "confidence_score" => a.confidence_score.partial_cmp(&b.confidence_score).unwrap_or(std::cmp::Ordering::Equal),
                    "usage_count" => a.usage_count.cmp(&b.usage_count),
                    "created_at" => a.created_at.cmp(&b.created_at),
                    "last_accessed_at" => a.last_accessed_at.cmp(&b.last_accessed_at),
                    _ => std::cmp::Ordering::Equal,
                };

                if query_params.descending {
                    ordering.reverse()
                } else {
                    ordering
                }
            });
        }

        // Apply limit
        if let Some(limit) = query_params.limit {
            concepts.truncate(limit);
        }

        Ok(concepts)
    }

    /// Get comprehensive statistics about the concept graph
    pub async fn get_statistics(&self) -> Result<ConceptGraphStats> {
        let mut stats = ConceptGraphStats::default();
        
        stats.total_concepts = self.concepts.len();
        stats.total_relationships = self.relationships.len();
        
        // Calculate concept metrics
        let mut total_confidence = 0.0;
        let now = Utc::now();
        let mut newest_created: Option<DateTime<Utc>> = None;
        let mut last_accessed: Option<DateTime<Utc>> = None;

        for concept in self.concepts.values() {
            // Type distribution
            *stats.concepts_by_type.entry(concept.concept_type.clone()).or_insert(0) += 1;
            
            // Confidence metrics
            total_confidence += concept.confidence_score;
            if concept.confidence_score >= 0.8 {
                stats.high_confidence_concepts += 1;
            }
            
            // Temporal metrics
            if newest_created.is_none() || concept.created_at > newest_created.unwrap() {
                newest_created = Some(concept.created_at);
            }
            
            if last_accessed.is_none() || concept.last_accessed_at > last_accessed.unwrap() {
                last_accessed = Some(concept.last_accessed_at);
            }
        }

        if stats.total_concepts > 0 {
            stats.average_confidence = total_confidence / stats.total_concepts as f64;
        }

        // Calculate relationship metrics
        let mut total_relationship_weight = 0.0;
        for relationship in self.relationships.values() {
            // Type distribution
            *stats.relationships_by_type.entry(relationship.relationship_type.clone()).or_insert(0) += 1;
            
            // Weight metrics
            total_relationship_weight += relationship.weight;
            if relationship.weight >= 0.7 {
                stats.strong_relationships += 1;
            }
        }

        if stats.total_relationships > 0 {
            stats.average_relationship_weight = total_relationship_weight / stats.total_relationships as f64;
        }

        if let Some(newest) = newest_created {
            stats.newest_concept_age_seconds = Some((now - newest).num_seconds());
        }

        if let Some(last_access) = last_accessed {
            stats.last_access_age_seconds = Some((now - last_access).num_seconds());
        }

        Ok(stats)
    }

    /// Get the current configuration
    pub fn config(&self) -> &ConceptGraphConfig {
        &self.config
    }

    /// Get the number of concepts currently stored
    pub fn concept_count(&self) -> usize {
        self.concepts.len()
    }

    // ==================== RELATIONSHIP MANAGEMENT ====================

    /// Create a new relationship between two concepts
    pub async fn create_relationship(
        &mut self,
        source_id: Uuid,
        target_id: Uuid,
        relationship_type: RelationshipType,
        initial_weight: f64,
    ) -> Result<Uuid> {
        // Validate that both concepts exist
        if !self.concepts.contains_key(&source_id) {
            return Err(anyhow::anyhow!("Source concept {} not found", source_id));
        }
        if !self.concepts.contains_key(&target_id) {
            return Err(anyhow::anyhow!("Target concept {} not found", target_id));
        }

        // Check if relationship already exists
        for relationship in self.relationships.values() {
            if relationship.source_id == source_id 
                && relationship.target_id == target_id 
                && relationship.relationship_type == relationship_type {
                return Err(anyhow::anyhow!("Relationship already exists"));
            }
        }

        let mut relationship = ConceptRelationship::new(source_id, target_id, relationship_type, initial_weight);
        
        // Apply Hebbian configuration
        relationship.configure_learning(
            self.hebbian_config.default_learning_rate,
            self.hebbian_config.default_decay_rate,
            self.hebbian_config.default_pruning_threshold,
        );

        let relationship_id = relationship.id;
        self.relationships.insert(relationship_id, relationship);

        Ok(relationship_id)
    }

    /// Get a relationship by its ID
    pub async fn get_relationship(&self, id: Uuid) -> Result<Option<ConceptRelationship>> {
        Ok(self.relationships.get(&id).cloned())
    }

    /// Update a relationship
    pub async fn update_relationship(&mut self, relationship: &ConceptRelationship) -> Result<()> {
        self.relationships.insert(relationship.id, relationship.clone());
        Ok(())
    }

    /// Delete a relationship
    pub async fn delete_relationship(&mut self, id: Uuid) -> Result<bool> {
        Ok(self.relationships.remove(&id).is_some())
    }

    /// Activate a relationship (Hebbian learning)
    pub async fn activate_relationship(&mut self, id: Uuid) -> Result<bool> {
        if let Some(relationship) = self.relationships.get_mut(&id) {
            relationship.activate();
            
            // Store concept IDs to avoid multiple mutable borrows
            let source_id = relationship.source_id;
            let target_id = relationship.target_id;
            
            // Also mark the connected concepts as accessed
            self.mark_concept_accessed(source_id).await?;
            self.mark_concept_accessed(target_id).await?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Activate relationships between two concepts (co-activation)
    pub async fn co_activate_concepts(&mut self, concept_id1: Uuid, concept_id2: Uuid) -> Result<usize> {
        let mut activated_count = 0;
        let relationship_ids: Vec<Uuid> = self.relationships.keys().cloned().collect();
        
        for relationship_id in relationship_ids {
            if let Some(relationship) = self.relationships.get(&relationship_id) {
                // Check if this relationship connects the two concepts
                if (relationship.source_id == concept_id1 && relationship.target_id == concept_id2) ||
                   (relationship.source_id == concept_id2 && relationship.target_id == concept_id1) {
                    self.activate_relationship(relationship_id).await?;
                    activated_count += 1;
                }
            }
        }
        
        Ok(activated_count)
    }

    /// Query relationships with filtering and sorting
    pub async fn query_relationships(&self, query_params: &RelationshipQuery) -> Result<Vec<ConceptRelationship>> {
        let mut relationships: Vec<ConceptRelationship> = self.relationships.values()
            .filter(|relationship| {
                // Filter by source concept ID
                if let Some(source_id) = query_params.source_id {
                    if relationship.source_id != source_id {
                        return false;
                    }
                }

                // Filter by target concept ID
                if let Some(target_id) = query_params.target_id {
                    if relationship.target_id != target_id {
                        return false;
                    }
                }

                // Filter by relationship type
                if let Some(ref expected_type) = query_params.relationship_type {
                    if &relationship.relationship_type != expected_type {
                        return false;
                    }
                }

                // Filter by minimum weight
                if let Some(min_weight) = query_params.min_weight {
                    if relationship.weight < min_weight {
                        return false;
                    }
                }

                // Filter by maximum weight
                if let Some(max_weight) = query_params.max_weight {
                    if relationship.weight > max_weight {
                        return false;
                    }
                }

                // Filter by minimum activation count
                if let Some(min_activation) = query_params.min_activation_count {
                    if relationship.activation_count < min_activation {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        // Sort results
        if let Some(ref sort_field) = query_params.sort_by {
            relationships.sort_by(|a, b| {
                let ordering = match sort_field.as_str() {
                    "weight" => a.weight.partial_cmp(&b.weight).unwrap_or(std::cmp::Ordering::Equal),
                    "activation_count" => a.activation_count.cmp(&b.activation_count),
                    "created_at" => a.created_at.cmp(&b.created_at),
                    "last_activated_at" => a.last_activated_at.cmp(&b.last_activated_at),
                    _ => std::cmp::Ordering::Equal,
                };

                if query_params.descending {
                    ordering.reverse()
                } else {
                    ordering
                }
            });
        }

        // Apply limit
        if let Some(limit) = query_params.limit {
            relationships.truncate(limit);
        }

        Ok(relationships)
    }

    /// Get all relationships for a specific concept
    pub async fn get_concept_relationships(&self, concept_id: Uuid) -> Result<Vec<ConceptRelationship>> {
        let relationships = self.relationships.values()
            .filter(|r| r.source_id == concept_id || r.target_id == concept_id)
            .cloned()
            .collect();
        Ok(relationships)
    }

    /// Apply decay to all relationships based on time elapsed
    pub async fn apply_decay_to_all_relationships(&mut self, time_delta_hours: f64) -> Result<usize> {
        let mut updated_count = 0;
        
        for relationship in self.relationships.values_mut() {
            let old_weight = relationship.weight;
            relationship.apply_decay(time_delta_hours);
            
            if relationship.weight != old_weight {
                updated_count += 1;
            }
        }
        
        Ok(updated_count)
    }

    /// Prune weak relationships below threshold
    pub async fn prune_weak_relationships(&mut self) -> Result<usize> {
        let mut to_remove = Vec::new();
        
        for (id, relationship) in &self.relationships {
            if relationship.should_prune() {
                to_remove.push(*id);
            }
        }
        
        let removed_count = to_remove.len();
        for id in to_remove {
            self.relationships.remove(&id);
        }
        
        Ok(removed_count)
    }

    /// Batch update multiple relationships efficiently
    pub async fn batch_update_relationships(&mut self, updates: Vec<ConceptRelationship>) -> Result<usize> {
        let mut updated_count = 0;
        
        for relationship in updates {
            if self.relationships.contains_key(&relationship.id) {
                self.relationships.insert(relationship.id, relationship);
                updated_count += 1;
            }
        }
        
        Ok(updated_count)
    }

    /// Get network connectivity metrics
    pub async fn get_network_metrics(&self) -> Result<NetworkMetrics> {
        let mut metrics = NetworkMetrics::default();
        
        metrics.total_relationships = self.relationships.len();
        
        // Calculate relationship statistics
        let mut total_weight = 0.0;
        let mut degree_map: HashMap<Uuid, usize> = HashMap::new();
        
        for relationship in self.relationships.values() {
            // Type distribution
            *metrics.relationships_by_type.entry(relationship.relationship_type.clone()).or_insert(0) += 1;
            
            // Weight statistics
            total_weight += relationship.weight;
            
            if relationship.weight >= 0.7 {
                metrics.strong_relationships += 1;
            } else if relationship.weight < 0.3 {
                metrics.weak_relationships += 1;
            }
            
            if relationship.should_prune() {
                metrics.prunable_relationships += 1;
            }
            
            // Degree calculation
            *degree_map.entry(relationship.source_id).or_insert(0) += 1;
            *degree_map.entry(relationship.target_id).or_insert(0) += 1;
        }
        
        if metrics.total_relationships > 0 {
            metrics.average_weight = total_weight / metrics.total_relationships as f64;
        }
        
        // Calculate average degree and isolated concepts
        if !degree_map.is_empty() {
            let total_degree: usize = degree_map.values().sum();
            metrics.average_degree = total_degree as f64 / degree_map.len() as f64;
        }
        
        metrics.isolated_concepts = self.concepts.len() - degree_map.len();
        
        // Find most connected concepts
        let mut concept_degrees: Vec<(Uuid, usize)> = degree_map.into_iter().collect();
        concept_degrees.sort_by(|a, b| b.1.cmp(&a.1));
        metrics.most_connected_concepts = concept_degrees.into_iter().take(10).collect();
        
        // Calculate clustering coefficient (simplified version)
        metrics.clustering_coefficient = self.calculate_clustering_coefficient().await?;
        
        Ok(metrics)
    }

    /// Calculate the clustering coefficient of the network
    async fn calculate_clustering_coefficient(&self) -> Result<f64> {
        if self.concepts.len() < 3 {
            return Ok(0.0);
        }
        
        let mut total_clustering = 0.0;
        let mut node_count = 0;
        
        // For each concept, calculate its local clustering coefficient
        for concept_id in self.concepts.keys() {
            let neighbors = self.get_neighbors(*concept_id).await?;
            
            if neighbors.len() < 2 {
                continue; // Need at least 2 neighbors for clustering
            }
            
            let mut triangle_count = 0;
            let possible_triangles = neighbors.len() * (neighbors.len() - 1) / 2;
            
            // Count triangles (connections between neighbors)
            for i in 0..neighbors.len() {
                for j in (i + 1)..neighbors.len() {
                    if self.has_relationship(neighbors[i], neighbors[j]).await? {
                        triangle_count += 1;
                    }
                }
            }
            
            if possible_triangles > 0 {
                total_clustering += triangle_count as f64 / possible_triangles as f64;
                node_count += 1;
            }
        }
        
        if node_count > 0 {
            Ok(total_clustering / node_count as f64)
        } else {
            Ok(0.0)
        }
    }

    /// Get neighboring concepts (directly connected)
    async fn get_neighbors(&self, concept_id: Uuid) -> Result<Vec<Uuid>> {
        let mut neighbors = Vec::new();
        
        for relationship in self.relationships.values() {
            if relationship.source_id == concept_id {
                neighbors.push(relationship.target_id);
            } else if relationship.target_id == concept_id {
                neighbors.push(relationship.source_id);
            }
        }
        
        Ok(neighbors)
    }

    /// Check if a relationship exists between two concepts
    async fn has_relationship(&self, concept_id1: Uuid, concept_id2: Uuid) -> Result<bool> {
        for relationship in self.relationships.values() {
            if (relationship.source_id == concept_id1 && relationship.target_id == concept_id2) ||
               (relationship.source_id == concept_id2 && relationship.target_id == concept_id1) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Get the Hebbian configuration
    pub fn hebbian_config(&self) -> &HebbianConfig {
        &self.hebbian_config
    }

    /// Update the Hebbian configuration
    pub fn set_hebbian_config(&mut self, config: HebbianConfig) {
        self.hebbian_config = config;
    }

    /// Get the number of relationships currently stored
    pub fn relationship_count(&self) -> usize {
        self.relationships.len()
    }

    // ===== TASK 4.3: CONCEPT FORMATION AND GRAPH TRAVERSAL ALGORITHMS =====

    /// Form concepts from BPE segment patterns
    /// This is the core concept formation algorithm that identifies high-frequency patterns
    /// from the segment discovery module and converts them into concept nodes
    pub async fn form_concepts_from_patterns(
        &mut self,
        segmenter: &BpeSegmenter,
    ) -> Result<ConceptFormationResult> {
        let mut result = ConceptFormationResult {
            concepts_formed: 0,
            concepts_merged: 0,
            concepts_split: 0,
            new_concept_ids: Vec::new(),
            merged_concept_ids: Vec::new(),
            rejected_patterns: Vec::new(),
        };

        // Get high-confidence segments from the segmenter
        let segments = segmenter.get_high_confidence_segments();
        let mut patterns_to_process = Vec::new();

        // Filter segments based on formation criteria
        for segment in segments {
            if let Some(stats) = segmenter.get_segment_stats_by_string(&segment.segment) {
                if stats.frequency >= self.formation_config.min_pattern_frequency
                    && stats.confidence >= self.formation_config.min_pattern_confidence
                {
                    patterns_to_process.push((segment.segment.clone(), stats.clone()));
                } else {
                    result.rejected_patterns.push(segment.segment.clone());
                }
            }
        }

        // Limit batch size
        patterns_to_process.truncate(self.formation_config.max_concepts_per_batch);

        // Create concepts from qualified patterns
        for (pattern, stats) in patterns_to_process {
            // Determine concept type based on pattern characteristics
            let concept_type = self.infer_concept_type(&pattern, &stats);
            
            // Calculate confidence score with bonuses
            let mut confidence = stats.confidence;
            if pattern.len() > 1 {
                confidence += self.formation_config.multi_char_bonus;
            }
            confidence = confidence.min(1.0);

            // Create the concept
            let concept = ConceptNode::new(
                concept_type,
                pattern.clone(),
                confidence,
                Some(format!("BPE_SEGMENT:{}", pattern)),
            );

            match self.create_concept(concept).await {
                Ok(concept_id) => {
                    result.new_concept_ids.push(concept_id);
                    result.concepts_formed += 1;
                }
                Err(_) => {
                    result.rejected_patterns.push(pattern);
                }
            }
        }

        // Auto-merge similar concepts if enabled
        if self.formation_config.concept_merge_threshold > 0.0 {
            let merge_result = self.merge_similar_concepts().await?;
            result.concepts_merged += merge_result.concepts_merged;
            result.merged_concept_ids.extend(merge_result.merged_concept_ids);
        }

        Ok(result)
    }

    /// Infer concept type from pattern characteristics
    fn infer_concept_type(&self, pattern: &str, _stats: &SegmentStats) -> ConceptType {
        // Simple heuristics for concept type inference
        // In a full implementation, this would use more sophisticated NLP
        
        if pattern.chars().all(|c| c.is_alphabetic()) {
            if pattern.len() == 1 {
                ConceptType::Attribute // Single characters often represent attributes
            } else if pattern.chars().all(|c| c.is_lowercase()) {
                ConceptType::Entity // Common words are often entities
            } else {
                ConceptType::Entity // Capitalized words are often proper nouns (entities)
            }
        } else if pattern.contains(' ') {
            ConceptType::Action // Multi-word patterns often represent actions or relations
        } else if pattern.chars().any(|c| c.is_numeric()) {
            ConceptType::Attribute // Patterns with numbers often represent quantities
        } else {
            ConceptType::Abstract // Default for complex patterns
        }
    }

    /// Merge concepts that are similar based on configuration thresholds
    pub async fn merge_similar_concepts(&mut self) -> Result<ConceptFormationResult> {
        let mut result = ConceptFormationResult {
            concepts_formed: 0,
            concepts_merged: 0,
            concepts_split: 0,
            new_concept_ids: Vec::new(),
            merged_concept_ids: Vec::new(),
            rejected_patterns: Vec::new(),
        };

        let concept_ids: Vec<Uuid> = self.concepts.keys().cloned().collect();
        let mut merged_ids = HashSet::new();

        // Compare all pairs of concepts
        for i in 0..concept_ids.len() {
            if merged_ids.contains(&concept_ids[i]) {
                continue;
            }

            for j in (i + 1)..concept_ids.len() {
                if merged_ids.contains(&concept_ids[j]) {
                    continue;
                }

                let concept1_id = concept_ids[i];
                let concept2_id = concept_ids[j];

                // Calculate similarity
                let similarity = self.calculate_concept_similarity(concept1_id, concept2_id).await?;
                
                if similarity >= self.formation_config.concept_merge_threshold {
                    // Merge the concepts
                    self.merge_concepts(concept1_id, concept2_id).await?;
                    merged_ids.insert(concept2_id);
                    result.merged_concept_ids.push(concept2_id);
                    result.concepts_merged += 1;
                }
            }
        }

        Ok(result)
    }

    /// Calculate similarity between two concepts
    pub async fn calculate_concept_similarity(&self, concept1_id: Uuid, concept2_id: Uuid) -> Result<f64> {
        let concept1 = self.concepts.get(&concept1_id)
            .ok_or_else(|| anyhow::anyhow!("Concept 1 not found"))?;
        let concept2 = self.concepts.get(&concept2_id)
            .ok_or_else(|| anyhow::anyhow!("Concept 2 not found"))?;

        let mut total_similarity = 0.0;
        let mut total_weight = 0.0;

        // Content similarity (string similarity)
        if self.similarity_config.content_weight > 0.0 {
            let content_sim = self.calculate_string_similarity(&concept1.content, &concept2.content);
            total_similarity += content_sim * self.similarity_config.content_weight;
            total_weight += self.similarity_config.content_weight;
        }

        // Relationship similarity (common neighbors)
        if self.similarity_config.relationship_weight > 0.0 {
            let relationship_sim = self.calculate_relationship_similarity(concept1_id, concept2_id).await?;
            total_similarity += relationship_sim * self.similarity_config.relationship_weight;
            total_weight += self.similarity_config.relationship_weight;
        }

        // Usage pattern similarity
        if self.similarity_config.usage_weight > 0.0 {
            let usage_sim = self.calculate_usage_similarity(concept1, concept2);
            total_similarity += usage_sim * self.similarity_config.usage_weight;
            total_weight += self.similarity_config.usage_weight;
        }

        // Metadata similarity
        if self.similarity_config.metadata_weight > 0.0 {
            let metadata_sim = self.calculate_metadata_similarity(concept1, concept2);
            total_similarity += metadata_sim * self.similarity_config.metadata_weight;
            total_weight += self.similarity_config.metadata_weight;
        }

        // Return weighted average
        if total_weight > 0.0 {
            Ok(total_similarity / total_weight)
        } else {
            Ok(0.0)
        }
    }

    /// Calculate string similarity using Levenshtein distance
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }

        let len1 = s1.chars().count();
        let len2 = s2.chars().count();
        
        if len1 == 0 || len2 == 0 {
            return 0.0;
        }

        // Simple Levenshtein distance implementation
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                    matrix[i - 1][j - 1] + cost,
                );
            }
        }
        
        let distance = matrix[len1][len2];
        let max_len = std::cmp::max(len1, len2);
        
        1.0 - (distance as f64 / max_len as f64)
    }

    /// Calculate relationship similarity based on common neighbors
    async fn calculate_relationship_similarity(&self, concept1_id: Uuid, concept2_id: Uuid) -> Result<f64> {
        let neighbors1 = self.get_neighbors(concept1_id).await?;
        let neighbors2 = self.get_neighbors(concept2_id).await?;

        if neighbors1.is_empty() && neighbors2.is_empty() {
            return Ok(1.0); // Both have no connections
        }

        if neighbors1.is_empty() || neighbors2.is_empty() {
            return Ok(0.0); // One has connections, other doesn't
        }

        let set1: HashSet<_> = neighbors1.into_iter().collect();
        let set2: HashSet<_> = neighbors2.into_iter().collect();

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        if union == 0 {
            Ok(0.0)
        } else {
            Ok(intersection as f64 / union as f64)
        }
    }

    /// Calculate usage pattern similarity
    fn calculate_usage_similarity(&self, concept1: &ConceptNode, concept2: &ConceptNode) -> f64 {
        // Compare usage counts, confidence scores, and access patterns
        let usage_diff = (concept1.usage_count as f64 - concept2.usage_count as f64).abs();
        let confidence_diff = (concept1.confidence_score - concept2.confidence_score).abs();
        
        // Normalize differences to 0-1 range and invert to get similarity
        let usage_sim = 1.0 - (usage_diff / (concept1.usage_count.max(concept2.usage_count).max(1) as f64));
        let confidence_sim = 1.0 - confidence_diff;
        
        (usage_sim + confidence_sim) / 2.0
    }

    /// Calculate metadata similarity
    fn calculate_metadata_similarity(&self, concept1: &ConceptNode, concept2: &ConceptNode) -> f64 {
        if concept1.metadata.is_empty() && concept2.metadata.is_empty() {
            return 1.0;
        }

        let keys1: HashSet<&String> = concept1.metadata.keys().collect();
        let keys2: HashSet<&String> = concept2.metadata.keys().collect();
        
        let common_keys: HashSet<&String> = keys1.intersection(&keys2).cloned().collect();
        let total_keys = keys1.union(&keys2).count();

        if total_keys == 0 {
            return 1.0;
        }

        let mut matching_values = 0;
        for key in &common_keys {
            if concept1.metadata.get(key.as_str()) == concept2.metadata.get(key.as_str()) {
                matching_values += 1;
            }
        }

        (common_keys.len() as f64 + matching_values as f64) / (total_keys as f64 * 2.0)
    }

    /// Merge two concepts by combining their properties and relationships
    async fn merge_concepts(&mut self, concept1_id: Uuid, concept2_id: Uuid) -> Result<()> {
        let concept2 = self.concepts.remove(&concept2_id)
            .ok_or_else(|| anyhow::anyhow!("Concept 2 not found"))?;

        // Update concept1 with merged properties
        if let Some(concept1) = self.concepts.get_mut(&concept1_id) {
            // Merge usage counts
            concept1.usage_count += concept2.usage_count;
            
            // Take higher confidence score
            concept1.confidence_score = concept1.confidence_score.max(concept2.confidence_score);
            
            // Update access time to most recent
            if concept2.last_accessed_at > concept1.last_accessed_at {
                concept1.last_accessed_at = concept2.last_accessed_at;
            }
            
            // Merge metadata
            for (key, value) in concept2.metadata {
                concept1.metadata.entry(key).or_insert(value);
            }
        }

        // Transfer all relationships from concept2 to concept1
        let relationships_to_update: Vec<_> = self.relationships
            .iter()
            .filter(|(_, rel)| rel.source_id == concept2_id || rel.target_id == concept2_id)
            .map(|(id, _)| *id)
            .collect();

        for rel_id in relationships_to_update {
            if let Some(relationship) = self.relationships.get_mut(&rel_id) {
                if relationship.source_id == concept2_id {
                    relationship.source_id = concept1_id;
                }
                if relationship.target_id == concept2_id {
                    relationship.target_id = concept1_id;
                }
            }
        }

        Ok(())
    }

    /// Perform graph traversal using specified algorithm
    pub async fn traverse_graph(
        &self,
        start_concept_id: Uuid,
        algorithm: TraversalAlgorithm,
        config: Option<TraversalConfig>,
    ) -> Result<TraversalResult> {
        let config = config.unwrap_or_else(|| self.traversal_config.clone());
        
        // Verify start concept exists
        if !self.concepts.contains_key(&start_concept_id) {
            return Err(anyhow::anyhow!("Start concept not found"));
        }

        match algorithm {
            TraversalAlgorithm::BreadthFirst => {
                self.breadth_first_search(start_concept_id, &config).await
            }
            TraversalAlgorithm::DepthFirst => {
                self.depth_first_search(start_concept_id, &config).await
            }
            TraversalAlgorithm::SpreadingActivation => {
                self.spreading_activation_search(start_concept_id, &config).await
            }
            TraversalAlgorithm::ShortestPath => {
                // For shortest path, we need a target - using BFS as fallback
                self.breadth_first_search(start_concept_id, &config).await
            }
        }
    }

    /// Breadth-First Search traversal
    async fn breadth_first_search(
        &self,
        start_concept_id: Uuid,
        config: &TraversalConfig,
    ) -> Result<TraversalResult> {
        let mut result = TraversalResult {
            start_concept_id,
            algorithm: TraversalAlgorithm::BreadthFirst,
            visited_concepts: Vec::new(),
            traversed_relationships: Vec::new(),
            activation_scores: HashMap::new(),
            distances: HashMap::new(),
            total_nodes_visited: 0,
            max_depth_reached: 0,
        };

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        queue.push_back((start_concept_id, 0));
        visited.insert(start_concept_id);
        result.distances.insert(start_concept_id, 0);

        while let Some((current_id, depth)) = queue.pop_front() {
            if result.total_nodes_visited >= config.max_nodes || depth > config.max_depth {
                break;
            }

            result.visited_concepts.push(current_id);
            result.total_nodes_visited += 1;
            result.max_depth_reached = result.max_depth_reached.max(depth);

            // Get neighbors through relationships
            let neighbors = self.get_neighbors(current_id).await?;
            
            for neighbor_id in neighbors {
                if !visited.contains(&neighbor_id) {
                    // Check if we should follow this relationship
                    if let Some(rel) = self.find_relationship(current_id, neighbor_id).await? {
                        if rel.weight >= config.min_relationship_weight {
                            // Check relationship type filter
                            if config.follow_relationship_types.is_empty() 
                                || config.follow_relationship_types.contains(&rel.relationship_type) {
                                
                                visited.insert(neighbor_id);
                                queue.push_back((neighbor_id, depth + 1));
                                result.distances.insert(neighbor_id, depth + 1);
                                result.traversed_relationships.push(rel.id);
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Depth-First Search traversal
    async fn depth_first_search(
        &self,
        start_concept_id: Uuid,
        config: &TraversalConfig,
    ) -> Result<TraversalResult> {
        let mut result = TraversalResult {
            start_concept_id,
            algorithm: TraversalAlgorithm::DepthFirst,
            visited_concepts: Vec::new(),
            traversed_relationships: Vec::new(),
            activation_scores: HashMap::new(),
            distances: HashMap::new(),
            total_nodes_visited: 0,
            max_depth_reached: 0,
        };

        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        
        stack.push((start_concept_id, 0));
        result.distances.insert(start_concept_id, 0);

        while let Some((current_id, depth)) = stack.pop() {
            if result.total_nodes_visited >= config.max_nodes || depth > config.max_depth {
                continue;
            }

            if visited.contains(&current_id) {
                continue;
            }

            visited.insert(current_id);
            result.visited_concepts.push(current_id);
            result.total_nodes_visited += 1;
            result.max_depth_reached = result.max_depth_reached.max(depth);

            // Get neighbors through relationships
            let neighbors = self.get_neighbors(current_id).await?;
            
            for neighbor_id in neighbors {
                if !visited.contains(&neighbor_id) {
                    // Check if we should follow this relationship
                    if let Some(rel) = self.find_relationship(current_id, neighbor_id).await? {
                        if rel.weight >= config.min_relationship_weight {
                            // Check relationship type filter
                            if config.follow_relationship_types.is_empty() 
                                || config.follow_relationship_types.contains(&rel.relationship_type) {
                                
                                stack.push((neighbor_id, depth + 1));
                                if !result.distances.contains_key(&neighbor_id) {
                                    result.distances.insert(neighbor_id, depth + 1);
                                }
                                result.traversed_relationships.push(rel.id);
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Spreading Activation search - simulates neural activation spreading
    async fn spreading_activation_search(
        &self,
        start_concept_id: Uuid,
        config: &TraversalConfig,
    ) -> Result<TraversalResult> {
        let mut result = TraversalResult {
            start_concept_id,
            algorithm: TraversalAlgorithm::SpreadingActivation,
            visited_concepts: Vec::new(),
            traversed_relationships: Vec::new(),
            activation_scores: HashMap::new(),
            distances: HashMap::new(),
            total_nodes_visited: 0,
            max_depth_reached: 0,
        };

        // Initialize activation
        let mut current_activations = HashMap::new();
        current_activations.insert(start_concept_id, 1.0);
        result.activation_scores.insert(start_concept_id, 1.0);
        result.distances.insert(start_concept_id, 0);

        for depth in 0..config.max_depth {
            let mut next_activations = HashMap::new();
            let mut activated_any = false;

            for (&concept_id, &activation) in &current_activations {
                if activation < 0.01 {
                    continue; // Skip very low activations
                }

                result.visited_concepts.push(concept_id);
                result.total_nodes_visited += 1;
                result.max_depth_reached = depth;

                // Spread activation to neighbors
                let neighbors = self.get_neighbors(concept_id).await?;
                
                for neighbor_id in neighbors {
                    if let Some(rel) = self.find_relationship(concept_id, neighbor_id).await? {
                        if rel.weight >= config.min_relationship_weight {
                            // Check relationship type filter
                            if config.follow_relationship_types.is_empty() 
                                || config.follow_relationship_types.contains(&rel.relationship_type) {
                                
                                // Calculate activation to spread
                                let spread_activation = activation 
                                    * rel.weight 
                                    * config.activation_spread_factor
                                    * config.activation_decay_factor.powi(depth as i32);

                                if spread_activation > 0.01 {
                                    let current_neighbor_activation = next_activations.get(&neighbor_id).unwrap_or(&0.0);
                                    next_activations.insert(neighbor_id, current_neighbor_activation + spread_activation);
                                    
                                    if !result.distances.contains_key(&neighbor_id) {
                                        result.distances.insert(neighbor_id, depth + 1);
                                    }
                                    result.traversed_relationships.push(rel.id);
                                    activated_any = true;
                                }
                            }
                        }
                    }
                }
            }

            // Update activation scores in result
            for (&concept_id, &activation) in &next_activations {
                let current_score = result.activation_scores.get(&concept_id).unwrap_or(&0.0);
                result.activation_scores.insert(concept_id, current_score.max(activation));
            }

            if !activated_any || result.total_nodes_visited >= config.max_nodes {
                break;
            }

            current_activations = next_activations;
        }

        Ok(result)
    }

    /// Find a relationship between two concepts
    async fn find_relationship(&self, concept1_id: Uuid, concept2_id: Uuid) -> Result<Option<ConceptRelationship>> {
        for relationship in self.relationships.values() {
            if (relationship.source_id == concept1_id && relationship.target_id == concept2_id) ||
               (relationship.source_id == concept2_id && relationship.target_id == concept1_id) {
                return Ok(Some(relationship.clone()));
            }
        }
        Ok(None)
    }

    /// Find shortest path between two concepts
    pub async fn find_shortest_path(
        &self,
        source_id: Uuid,
        target_id: Uuid,
    ) -> Result<Option<ConceptPath>> {
        // Verify both concepts exist
        if !self.concepts.contains_key(&source_id) || !self.concepts.contains_key(&target_id) {
            return Err(anyhow::anyhow!("Source or target concept not found"));
        }

        // Use Dijkstra's algorithm for shortest path
        let mut distances: HashMap<Uuid, f64> = HashMap::new();
        let mut previous: HashMap<Uuid, (Uuid, Uuid)> = HashMap::new(); // (previous_concept, relationship_id)
        let mut unvisited: HashSet<Uuid> = self.concepts.keys().cloned().collect();
        
        // Initialize distances
        for &concept_id in &unvisited {
            distances.insert(concept_id, f64::INFINITY);
        }
        distances.insert(source_id, 0.0);

        while !unvisited.is_empty() {
            // Find unvisited node with minimum distance
            let current = *unvisited.iter()
                .min_by(|&&a, &&b| {
                    distances.get(&a).unwrap_or(&f64::INFINITY).partial_cmp(
                        distances.get(&b).unwrap_or(&f64::INFINITY)
                    ).unwrap()
                })
                .ok_or_else(|| anyhow::anyhow!("No reachable nodes"))?;

            unvisited.remove(&current);

            // If we reached the target, break
            if current == target_id {
                break;
            }

            let current_distance = *distances.get(&current).unwrap_or(&f64::INFINITY);
            if current_distance == f64::INFINITY {
                break; // No more reachable nodes
            }

            // Check all neighbors
            let neighbors = self.get_neighbors(current).await?;
            for neighbor in neighbors {
                if unvisited.contains(&neighbor) {
                    if let Some(rel) = self.find_relationship(current, neighbor).await? {
                        // Use inverse of weight as distance (higher weight = shorter distance)
                        let edge_weight = 1.0 / rel.weight.max(0.01);
                        let alt_distance = current_distance + edge_weight;
                        
                        let neighbor_distance = *distances.get(&neighbor).unwrap_or(&f64::INFINITY);
                        if alt_distance < neighbor_distance {
                            distances.insert(neighbor, alt_distance);
                            previous.insert(neighbor, (current, rel.id));
                        }
                    }
                }
            }
        }

        // Reconstruct path if target was reached
        if distances.get(&target_id).unwrap_or(&f64::INFINITY) == &f64::INFINITY {
            return Ok(None); // No path found
        }

        let mut path_concepts = Vec::new();
        let mut path_relationships = Vec::new();
        let mut current = target_id;
        let mut total_weight = 0.0;

        // Build path backwards
        while current != source_id {
            path_concepts.push(current);
            
            if let Some((prev_concept, rel_id)) = previous.get(&current) {
                path_relationships.push(*rel_id);
                if let Some(rel) = self.relationships.get(rel_id) {
                    total_weight += rel.weight;
                }
                current = *prev_concept;
            } else {
                return Err(anyhow::anyhow!("Path reconstruction failed"));
            }
        }
        
        path_concepts.push(source_id);
        
        // Reverse to get path from source to target
        path_concepts.reverse();
        path_relationships.reverse();

        let path_length = path_concepts.len() - 1;
        let average_weight = if path_length > 0 { total_weight / path_length as f64 } else { 0.0 };

        Ok(Some(ConceptPath {
            source_id,
            target_id,
            concept_path: path_concepts,
            relationship_path: path_relationships,
            path_length,
            total_weight,
            average_weight,
        }))
    }

    /// Extract a subgraph around a specific concept
    pub async fn extract_subgraph(
        &self,
        center_concept_id: Uuid,
        radius: usize,
    ) -> Result<ConceptSubgraph> {
        let mut subgraph_concepts = HashMap::new();
        let mut subgraph_relationships = HashMap::new();
        let mut concepts_to_visit = VecDeque::new();
        let mut visited = HashSet::new();

        // Start with center concept
        if let Some(center_concept) = self.concepts.get(&center_concept_id) {
            subgraph_concepts.insert(center_concept_id, center_concept.clone());
            concepts_to_visit.push_back((center_concept_id, 0));
            visited.insert(center_concept_id);
        } else {
            return Err(anyhow::anyhow!("Center concept not found"));
        }

        // BFS to gather concepts within radius
        while let Some((current_id, depth)) = concepts_to_visit.pop_front() {
            if depth >= radius {
                continue;
            }

            let neighbors = self.get_neighbors(current_id).await?;
            for neighbor_id in neighbors {
                // Add neighbor concept if not already added
                if !visited.contains(&neighbor_id) {
                    if let Some(neighbor_concept) = self.concepts.get(&neighbor_id) {
                        subgraph_concepts.insert(neighbor_id, neighbor_concept.clone());
                        concepts_to_visit.push_back((neighbor_id, depth + 1));
                        visited.insert(neighbor_id);
                    }
                }

                // Add relationship if both concepts are in subgraph
                if subgraph_concepts.contains_key(&neighbor_id) {
                    if let Some(rel) = self.find_relationship(current_id, neighbor_id).await? {
                        subgraph_relationships.insert(rel.id, rel);
                    }
                }
            }
        }

        // Calculate metrics for the subgraph
        let concepts: Vec<ConceptNode> = subgraph_concepts.into_values().collect();
        let relationships: Vec<ConceptRelationship> = subgraph_relationships.into_values().collect();
        
        // Create simplified metrics for the subgraph
        let metrics = NetworkMetrics {
            total_relationships: relationships.len(),
            relationships_by_type: relationships.iter().fold(HashMap::new(), |mut acc, rel| {
                *acc.entry(rel.relationship_type.clone()).or_insert(0) += 1;
                acc
            }),
            average_weight: if relationships.is_empty() {
                0.0
            } else {
                relationships.iter().map(|r| r.weight).sum::<f64>() / relationships.len() as f64
            },
            strong_relationships: relationships.iter().filter(|r| r.weight >= 0.7).count(),
            weak_relationships: relationships.iter().filter(|r| r.weight < 0.3).count(),
            prunable_relationships: relationships.iter().filter(|r| r.should_prune()).count(),
            average_degree: if concepts.is_empty() {
                0.0
            } else {
                (relationships.len() * 2) as f64 / concepts.len() as f64
            },
            isolated_concepts: 0, // All concepts in subgraph have connections by definition
            clustering_coefficient: 0.0, // Would need more complex calculation
            most_connected_concepts: Vec::new(), // Simplified for now
        };

        Ok(ConceptSubgraph {
            concepts,
            relationships,
            center_concept_id: Some(center_concept_id),
            radius: Some(radius),
            metrics,
        })
    }

    /// Get configuration objects
    pub fn formation_config(&self) -> &ConceptFormationConfig {
        &self.formation_config
    }

    pub fn traversal_config(&self) -> &TraversalConfig {
        &self.traversal_config
    }

    pub fn similarity_config(&self) -> &SimilarityConfig {
        &self.similarity_config
    }

    /// Set configuration objects
    pub fn set_formation_config(&mut self, config: ConceptFormationConfig) {
        self.formation_config = config;
    }

    pub fn set_traversal_config(&mut self, config: TraversalConfig) {
        self.traversal_config = config;
    }

    pub fn set_similarity_config(&mut self, config: SimilarityConfig) {
        self.similarity_config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_concept_node_creation() {
        let concept = ConceptNode::new(
            ConceptType::Entity,
            "test_entity".to_string(),
            0.85,
            Some("test_source".to_string()),
        );

        assert_eq!(concept.concept_type, ConceptType::Entity);
        assert_eq!(concept.content, "test_entity");
        assert_eq!(concept.confidence_score, 0.85);
        assert_eq!(concept.source_reference, Some("test_source".to_string()));
        assert_eq!(concept.usage_count, 0);
        assert!(!concept.id.is_nil());
    }

    #[tokio::test]
    async fn test_concept_node_access_tracking() {
        let mut concept = ConceptNode::new(
            ConceptType::Action,
            "test_action".to_string(),
            0.75,
            None,
        );

        let initial_usage = concept.usage_count;
        let initial_access_time = concept.last_accessed_at;

        // Wait a bit to ensure timestamp difference
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        concept.mark_accessed();

        assert_eq!(concept.usage_count, initial_usage + 1);
        assert!(concept.last_accessed_at > initial_access_time);
    }

    #[tokio::test]
    async fn test_concept_node_confidence_update() {
        let mut concept = ConceptNode::new(
            ConceptType::Attribute,
            "test_attribute".to_string(),
            0.5,
            None,
        );

        concept.update_confidence(0.95);
        assert_eq!(concept.confidence_score, 0.95);

        // Test clamping
        concept.update_confidence(1.5);
        assert_eq!(concept.confidence_score, 1.0);

        concept.update_confidence(-0.1);
        assert_eq!(concept.confidence_score, 0.0);
    }

    #[tokio::test]
    async fn test_concept_node_metadata() {
        let mut concept = ConceptNode::new(
            ConceptType::Abstract,
            "test_abstract".to_string(),
            0.6,
            None,
        );

        concept.set_metadata("key1".to_string(), "value1".to_string());
        concept.set_metadata("key2".to_string(), "value2".to_string());

        assert_eq!(concept.get_metadata("key1"), Some(&"value1".to_string()));
        assert_eq!(concept.get_metadata("key2"), Some(&"value2".to_string()));
        assert_eq!(concept.get_metadata("nonexistent"), None);
    }

    #[tokio::test]
    async fn test_concept_type_display() {
        assert_eq!(ConceptType::Entity.to_string(), "Entity");
        assert_eq!(ConceptType::Action.to_string(), "Action");
        assert_eq!(ConceptType::Attribute.to_string(), "Attribute");
        assert_eq!(ConceptType::Abstract.to_string(), "Abstract");
        assert_eq!(ConceptType::Relation.to_string(), "Relation");
    }

    #[tokio::test]
    async fn test_concept_graph_manager_crud() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Test create
        let concept = ConceptNode::new(
            ConceptType::Entity,
            "test_crud_entity".to_string(),
            0.9,
            Some("test_source".to_string()),
        );
        let concept_id = concept.id;

        let result_id = manager.create_concept(concept.clone()).await.unwrap();
        assert_eq!(result_id, concept_id);

        // Test read
        let retrieved = manager.get_concept(concept_id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, "test_crud_entity");

        // Test update (mark as accessed)
        let access_result = manager.mark_concept_accessed(concept_id).await.unwrap();
        assert!(access_result);

        let updated_concept = manager.get_concept(concept_id).await.unwrap().unwrap();
        assert_eq!(updated_concept.usage_count, 1);

        // Test delete
        let delete_result = manager.delete_concept(concept_id).await.unwrap();
        assert!(delete_result);

        // Verify deletion
        let deleted_check = manager.get_concept(concept_id).await.unwrap();
        assert!(deleted_check.is_none());
    }

    #[tokio::test]
    async fn test_concept_query_filtering() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create test concepts
        let concepts = vec![
            ConceptNode::new(ConceptType::Entity, "user".to_string(), 0.95, None),
            ConceptNode::new(ConceptType::Action, "runs".to_string(), 0.85, None),
            ConceptNode::new(ConceptType::Entity, "system".to_string(), 0.75, None),
        ];

        for concept in concepts {
            manager.create_concept(concept).await.unwrap();
        }

        // Test type filtering
        let entity_query = ConceptQuery {
            concept_type: Some(ConceptType::Entity),
            ..Default::default()
        };

        let entities = manager.query_concepts(&entity_query).await.unwrap();
        assert_eq!(entities.len(), 2);
        assert!(entities.iter().all(|c| c.concept_type == ConceptType::Entity));

        // Test confidence filtering
        let high_confidence_query = ConceptQuery {
            min_confidence: Some(0.9),
            ..Default::default()
        };

        let high_confidence = manager.query_concepts(&high_confidence_query).await.unwrap();
        assert_eq!(high_confidence.len(), 1);
        assert_eq!(high_confidence[0].content, "user");
    }

    #[tokio::test]
    async fn test_concept_graph_statistics() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create test concepts
        let concepts = vec![
            ConceptNode::new(ConceptType::Entity, "entity1".to_string(), 0.9, None),
            ConceptNode::new(ConceptType::Entity, "entity2".to_string(), 0.8, None),
            ConceptNode::new(ConceptType::Action, "action1".to_string(), 0.7, None),
        ];

        let mut concept_ids: Vec<Uuid> = Vec::new();
        for concept in concepts {
            let id = manager.create_concept(concept).await.unwrap();
            concept_ids.push(id);
        }

        // Add a relationship to test relationship statistics
        if concept_ids.len() >= 2 {
            manager.create_relationship(concept_ids[0], concept_ids[1], RelationshipType::IsA, 0.8).await.unwrap();
        }

        let stats = manager.get_statistics().await.unwrap();
        
        assert_eq!(stats.total_concepts, 3);
        assert_eq!(stats.total_relationships, 1);
        assert_eq!(stats.high_confidence_concepts, 2); // >= 0.8
        assert_eq!(stats.strong_relationships, 1); // >= 0.7
        assert_eq!(stats.concepts_by_type.get(&ConceptType::Entity), Some(&2));
        assert_eq!(stats.concepts_by_type.get(&ConceptType::Action), Some(&1));
        assert!((stats.average_confidence - 0.8).abs() < 0.01); // (0.9 + 0.8 + 0.7) / 3
        assert!((stats.average_relationship_weight - 0.8).abs() < 0.01);
    }

    // ==================== RELATIONSHIP TESTS ====================

    #[tokio::test]
    async fn test_concept_relationship_creation() {
        let relationship = ConceptRelationship::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RelationshipType::IsA,
            0.5,
        );

        assert_eq!(relationship.relationship_type, RelationshipType::IsA);
        assert_eq!(relationship.weight, 0.5);
        assert_eq!(relationship.base_weight, 0.5);
        assert_eq!(relationship.activation_count, 0);
        assert_eq!(relationship.learning_rate, 0.1);
        assert_eq!(relationship.decay_rate, 0.01);
        assert_eq!(relationship.pruning_threshold, 0.1);
    }

    #[tokio::test]
    async fn test_hebbian_learning_activation() {
        let mut relationship = ConceptRelationship::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RelationshipType::SimilarTo,
            0.5,
        );

        let initial_weight = relationship.weight;
        relationship.activate();

        assert_eq!(relationship.activation_count, 1);
        assert!(relationship.weight > initial_weight); // Hebbian learning should increase weight
        assert!(relationship.weight <= 1.0); // Should be clamped
    }

    #[tokio::test]
    async fn test_relationship_decay() {
        let mut relationship = ConceptRelationship::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RelationshipType::Causes,
            0.8,
        );

        let initial_weight = relationship.weight;
        relationship.apply_decay(24.0); // 24 hours

        assert!(relationship.weight < initial_weight); // Decay should reduce weight
        assert!(relationship.weight >= relationship.base_weight * 0.1); // Should not decay below minimum
    }

    #[tokio::test]
    async fn test_relationship_pruning() {
        let mut relationship = ConceptRelationship::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RelationshipType::PartOf,
            0.05, // Below default pruning threshold
        );

        assert!(relationship.should_prune());

        relationship.weight = 0.2;
        assert!(!relationship.should_prune());
    }

    #[tokio::test]
    async fn test_relationship_inverse_types() {
        let relationship = ConceptRelationship::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RelationshipType::Before,
            0.5,
        );

        assert_eq!(relationship.get_inverse_type(), Some(RelationshipType::After));

        let relationship2 = ConceptRelationship::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RelationshipType::Has,
            0.5,
        );

        assert_eq!(relationship2.get_inverse_type(), Some(RelationshipType::PartOf));
    }

    #[tokio::test]
    async fn test_relationship_type_display() {
        assert_eq!(RelationshipType::IsA.to_string(), "IS_A");
        assert_eq!(RelationshipType::PartOf.to_string(), "PART_OF");
        assert_eq!(RelationshipType::Causes.to_string(), "CAUSES");
        assert_eq!(RelationshipType::SimilarTo.to_string(), "SIMILAR_TO");
        assert_eq!(RelationshipType::Custom("CUSTOM_REL".to_string()).to_string(), "CUSTOM_REL");
    }

    #[tokio::test]
    async fn test_concept_graph_relationship_crud() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create test concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "dog".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "animal".to_string(), 0.8, None);

        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();

        // Create relationship
        let rel_id = manager.create_relationship(
            concept1_id,
            concept2_id,
            RelationshipType::IsA,
            0.7,
        ).await.unwrap();

        assert_eq!(manager.relationship_count(), 1);

        // Get relationship
        let relationship = manager.get_relationship(rel_id).await.unwrap();
        assert!(relationship.is_some());
        let rel = relationship.unwrap();
        assert_eq!(rel.source_id, concept1_id);
        assert_eq!(rel.target_id, concept2_id);
        assert_eq!(rel.relationship_type, RelationshipType::IsA);

        // Update relationship
        let mut updated_rel = rel.clone();
        updated_rel.weight = 0.9;
        manager.update_relationship(&updated_rel).await.unwrap();

        let updated = manager.get_relationship(rel_id).await.unwrap().unwrap();
        assert_eq!(updated.weight, 0.9);

        // Delete relationship
        let deleted = manager.delete_relationship(rel_id).await.unwrap();
        assert!(deleted);
        assert_eq!(manager.relationship_count(), 0);
    }

    #[tokio::test]
    async fn test_relationship_activation_and_co_activation() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create test concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "cat".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "pet".to_string(), 0.8, None);

        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();

        // Create relationship
        let rel_id = manager.create_relationship(
            concept1_id,
            concept2_id,
            RelationshipType::IsA,
            0.5,
        ).await.unwrap();

        // Activate relationship
        let activated = manager.activate_relationship(rel_id).await.unwrap();
        assert!(activated);

        // Check that relationship weight increased
        let relationship = manager.get_relationship(rel_id).await.unwrap().unwrap();
        assert!(relationship.weight > 0.5);
        assert_eq!(relationship.activation_count, 1);

        // Test co-activation
        let co_activated = manager.co_activate_concepts(concept1_id, concept2_id).await.unwrap();
        assert_eq!(co_activated, 1); // One relationship was activated

        let relationship = manager.get_relationship(rel_id).await.unwrap().unwrap();
        assert_eq!(relationship.activation_count, 2);
    }

    #[tokio::test]
    async fn test_relationship_query_filtering() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create test concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "car".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "vehicle".to_string(), 0.8, None);
        let concept3 = ConceptNode::new(ConceptType::Entity, "wheel".to_string(), 0.7, None);

        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();
        let concept3_id = manager.create_concept(concept3).await.unwrap();

        // Create relationships
        let rel1_id = manager.create_relationship(concept1_id, concept2_id, RelationshipType::IsA, 0.8).await.unwrap();
        manager.create_relationship(concept1_id, concept3_id, RelationshipType::Has, 0.6).await.unwrap();
        manager.create_relationship(concept2_id, concept3_id, RelationshipType::Has, 0.9).await.unwrap();

        // Test filtering by relationship type
        let mut query = RelationshipQuery::default();
        query.relationship_type = Some(RelationshipType::IsA);
        let is_a_rels = manager.query_relationships(&query).await.unwrap();
        assert_eq!(is_a_rels.len(), 1);
        assert_eq!(is_a_rels[0].id, rel1_id);

        // Test filtering by source concept
        query = RelationshipQuery::default();
        query.source_id = Some(concept1_id);
        let source_rels = manager.query_relationships(&query).await.unwrap();
        assert_eq!(source_rels.len(), 2);

        // Test filtering by weight range
        query = RelationshipQuery::default();
        query.min_weight = Some(0.7);
        let strong_rels = manager.query_relationships(&query).await.unwrap();
        assert_eq!(strong_rels.len(), 2); // Two relationships with weight >= 0.7

        // Test sorting by weight descending
        query = RelationshipQuery::default();
        query.sort_by = Some("weight".to_string());
        query.descending = true;
        let sorted_rels = manager.query_relationships(&query).await.unwrap();
        assert!(sorted_rels[0].weight >= sorted_rels[1].weight);
        assert!(sorted_rels[1].weight >= sorted_rels[2].weight);

        // Test limit
        query = RelationshipQuery::default();
        query.limit = Some(2);
        let limited_rels = manager.query_relationships(&query).await.unwrap();
        assert_eq!(limited_rels.len(), 2);
    }

    #[tokio::test]
    async fn test_network_metrics_and_pruning() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create test concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "node1".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "node2".to_string(), 0.8, None);
        let concept3 = ConceptNode::new(ConceptType::Entity, "node3".to_string(), 0.7, None);
        let concept4 = ConceptNode::new(ConceptType::Entity, "isolated".to_string(), 0.6, None);

        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();
        let concept3_id = manager.create_concept(concept3).await.unwrap();
        manager.create_concept(concept4).await.unwrap(); // Isolated

        // Create relationships with different weights
        manager.create_relationship(concept1_id, concept2_id, RelationshipType::IsA, 0.8).await.unwrap(); // Strong
        manager.create_relationship(concept1_id, concept3_id, RelationshipType::Has, 0.2).await.unwrap(); // Weak (but above pruning threshold)
        manager.create_relationship(concept2_id, concept3_id, RelationshipType::SimilarTo, 0.05).await.unwrap(); // Very weak (below pruning threshold)

        // Get network metrics
        let metrics = manager.get_network_metrics().await.unwrap();
        assert_eq!(metrics.total_relationships, 3);
        assert_eq!(metrics.strong_relationships, 1); // One relationship with weight >= 0.7
        assert_eq!(metrics.weak_relationships, 2); // Two relationships with weight < 0.3 (0.2 and 0.05)
        assert_eq!(metrics.isolated_concepts, 1); // concept4 has no relationships
        assert!(metrics.average_degree > 0.0);
        assert_eq!(metrics.prunable_relationships, 1); // One relationship below pruning threshold (0.05 < 0.1)

        // Test pruning
        let pruned = manager.prune_weak_relationships().await.unwrap();
        assert_eq!(pruned, 1); // One relationship should be pruned
        assert_eq!(manager.relationship_count(), 2);

        // Test decay
        let decayed = manager.apply_decay_to_all_relationships(48.0).await.unwrap(); // 48 hours
        assert!(decayed > 0); // Some relationships should have decayed
    }

    // ===== TASK 4.3 TESTS: CONCEPT FORMATION AND GRAPH TRAVERSAL =====

    #[tokio::test]
    async fn test_concept_formation_config() {
        let config = ConceptFormationConfig::default();
        assert_eq!(config.min_pattern_frequency, 5);
        assert_eq!(config.min_pattern_confidence, 0.6);
        assert_eq!(config.max_concepts_per_batch, 100);
        assert_eq!(config.concept_merge_threshold, 0.8);
        assert_eq!(config.concept_split_usage_threshold, 1000);
        assert_eq!(config.multi_char_bonus, 0.1);
    }

    #[tokio::test]
    async fn test_traversal_config() {
        let config = TraversalConfig::default();
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.max_nodes, 100);
        assert_eq!(config.min_relationship_weight, 0.1);
        assert_eq!(config.activation_spread_factor, 0.8);
        assert_eq!(config.activation_decay_factor, 0.9);
        assert!(config.follow_relationship_types.is_empty());
    }

    #[tokio::test]
    async fn test_similarity_config() {
        let config = SimilarityConfig::default();
        assert_eq!(config.content_weight, 0.4);
        assert_eq!(config.relationship_weight, 0.3);
        assert_eq!(config.usage_weight, 0.2);
        assert_eq!(config.metadata_weight, 0.1);
        assert_eq!(config.min_similarity_threshold, 0.7);
    }

    #[tokio::test]
    async fn test_concept_type_inference() {
        let config = ConceptGraphConfig::default();
        let manager = ConceptGraphManager::new(config).await.unwrap();
        
        // Mock SegmentStats for testing
        let stats = crate::segment_discovery::SegmentStats {
            segment: "test".to_string(),
            frequency: 10,
            length: 4,
            formed_from: None,
            merge_step: None,
            confidence: 0.8,
            entropy: 0.5,
            context_stability: 0.7,
            created_at: 0,
            last_accessed: 0,
            last_modified: 0,
            access_count: 5,
            is_archived: false,
        };

        // Test different pattern types
        assert_eq!(manager.infer_concept_type("a", &stats), ConceptType::Attribute); // Single character
        assert_eq!(manager.infer_concept_type("hello", &stats), ConceptType::Entity); // Lowercase word
        assert_eq!(manager.infer_concept_type("Hello", &stats), ConceptType::Entity); // Capitalized word
        assert_eq!(manager.infer_concept_type("hello world", &stats), ConceptType::Action); // Multi-word
        assert_eq!(manager.infer_concept_type("test123", &stats), ConceptType::Attribute); // Contains numbers
        assert_eq!(manager.infer_concept_type("@#$", &stats), ConceptType::Abstract); // Complex pattern
    }

    #[tokio::test]
    async fn test_string_similarity_calculation() {
        let config = ConceptGraphConfig::default();
        let manager = ConceptGraphManager::new(config).await.unwrap();

        // Test exact match
        assert_eq!(manager.calculate_string_similarity("hello", "hello"), 1.0);
        
        // Test empty strings
        assert_eq!(manager.calculate_string_similarity("", ""), 1.0);
        assert_eq!(manager.calculate_string_similarity("hello", ""), 0.0);
        assert_eq!(manager.calculate_string_similarity("", "world"), 0.0);
        
        // Test partial similarity
        let sim1 = manager.calculate_string_similarity("hello", "hallo");
        assert!(sim1 > 0.5 && sim1 < 1.0);
        
        let sim2 = manager.calculate_string_similarity("cat", "dog");
        assert!(sim2 < 0.5);
        
        // Test different lengths
        let sim3 = manager.calculate_string_similarity("hi", "hello");
        assert!(sim3 > 0.0 && sim3 < 0.5);
    }

    #[tokio::test]
    async fn test_concept_similarity_calculation() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create similar concepts
        let concept1 = ConceptNode::new(ConceptType::Entity, "cat".to_string(), 0.9, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "cats".to_string(), 0.8, None);
        let concept3 = ConceptNode::new(ConceptType::Action, "dog".to_string(), 0.7, None);

        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();
        let concept3_id = manager.create_concept(concept3).await.unwrap();

        // Calculate similarity between similar concepts
        let similarity_1_2 = manager.calculate_concept_similarity(concept1_id, concept2_id).await.unwrap();
        let similarity_1_3 = manager.calculate_concept_similarity(concept1_id, concept3_id).await.unwrap();
        
        // cat vs cats should be more similar than cat vs dog
        assert!(similarity_1_2 > similarity_1_3);
        assert!(similarity_1_2 > 0.3); // Should have reasonable similarity
        assert!(similarity_1_3 < 0.5); // Should be less similar
    }

    #[tokio::test]
    async fn test_concept_merging() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create concepts to merge
        let mut concept1 = ConceptNode::new(ConceptType::Entity, "cat".to_string(), 0.9, None);
        concept1.usage_count = 10;
        concept1.set_metadata("category".to_string(), "animal".to_string());

        let mut concept2 = ConceptNode::new(ConceptType::Entity, "feline".to_string(), 0.8, None);
        concept2.usage_count = 5;
        concept2.set_metadata("color".to_string(), "orange".to_string());

        let concept1_id = manager.create_concept(concept1).await.unwrap();
        let concept2_id = manager.create_concept(concept2).await.unwrap();

        // Create a third concept to relate to
        let concept3 = ConceptNode::new(ConceptType::Entity, "pet".to_string(), 0.7, None);
        let concept3_id = manager.create_concept(concept3).await.unwrap();

        // Create relationships
        manager.create_relationship(concept1_id, concept3_id, RelationshipType::IsA, 0.8).await.unwrap();
        manager.create_relationship(concept2_id, concept3_id, RelationshipType::IsA, 0.7).await.unwrap();

        assert_eq!(manager.concept_count(), 3);
        assert_eq!(manager.relationship_count(), 2);

        // Merge concepts
        manager.merge_concepts(concept1_id, concept2_id).await.unwrap();

        // Verify merge results
        assert_eq!(manager.concept_count(), 2); // One concept should be removed
        assert!(manager.concepts.contains_key(&concept1_id)); // concept1 should remain
        assert!(!manager.concepts.contains_key(&concept2_id)); // concept2 should be removed

        // Check merged properties
        let merged_concept = manager.get_concept(concept1_id).await.unwrap().unwrap();
        assert_eq!(merged_concept.usage_count, 15); // 10 + 5
        assert_eq!(merged_concept.confidence_score, 0.9); // Should take higher confidence
        assert_eq!(merged_concept.metadata.len(), 2); // Should have both metadata entries

        // Check that relationships were transferred
        let concept1_rels = manager.get_concept_relationships(concept1_id).await.unwrap();
        assert_eq!(concept1_rels.len(), 2); // Should have both relationships
    }

    #[tokio::test]
    async fn test_breadth_first_search() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create a small graph: A -> B -> C
        //                       A -> D
        let concept_a = ConceptNode::new(ConceptType::Entity, "A".to_string(), 0.9, None);
        let concept_b = ConceptNode::new(ConceptType::Entity, "B".to_string(), 0.8, None);
        let concept_c = ConceptNode::new(ConceptType::Entity, "C".to_string(), 0.7, None);
        let concept_d = ConceptNode::new(ConceptType::Entity, "D".to_string(), 0.6, None);

        let id_a = manager.create_concept(concept_a).await.unwrap();
        let id_b = manager.create_concept(concept_b).await.unwrap();
        let id_c = manager.create_concept(concept_c).await.unwrap();
        let id_d = manager.create_concept(concept_d).await.unwrap();

        // Create relationships
        manager.create_relationship(id_a, id_b, RelationshipType::IsA, 0.8).await.unwrap();
        manager.create_relationship(id_b, id_c, RelationshipType::IsA, 0.7).await.unwrap();
        manager.create_relationship(id_a, id_d, RelationshipType::Has, 0.6).await.unwrap();

        // Perform BFS from A
        let result = manager.traverse_graph(id_a, TraversalAlgorithm::BreadthFirst, None).await.unwrap();

        assert_eq!(result.algorithm, TraversalAlgorithm::BreadthFirst);
        assert_eq!(result.start_concept_id, id_a);
        assert!(result.visited_concepts.contains(&id_a));
        assert!(result.visited_concepts.contains(&id_b));
        assert!(result.visited_concepts.len() >= 2); // Should visit at least A and its neighbors
        assert!(result.max_depth_reached <= 5); // Default max depth
        assert!(!result.distances.is_empty());
        assert_eq!(*result.distances.get(&id_a).unwrap(), 0); // Start node has distance 0
    }

    #[tokio::test]
    async fn test_depth_first_search() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create concepts and relationships
        let concept_a = ConceptNode::new(ConceptType::Entity, "A".to_string(), 0.9, None);
        let concept_b = ConceptNode::new(ConceptType::Entity, "B".to_string(), 0.8, None);
        let concept_c = ConceptNode::new(ConceptType::Entity, "C".to_string(), 0.7, None);

        let id_a = manager.create_concept(concept_a).await.unwrap();
        let id_b = manager.create_concept(concept_b).await.unwrap();
        let id_c = manager.create_concept(concept_c).await.unwrap();

        manager.create_relationship(id_a, id_b, RelationshipType::IsA, 0.8).await.unwrap();
        manager.create_relationship(id_b, id_c, RelationshipType::IsA, 0.7).await.unwrap();

        // Perform DFS from A
        let result = manager.traverse_graph(id_a, TraversalAlgorithm::DepthFirst, None).await.unwrap();

        assert_eq!(result.algorithm, TraversalAlgorithm::DepthFirst);
        assert_eq!(result.start_concept_id, id_a);
        assert!(result.visited_concepts.contains(&id_a));
        assert!(!result.visited_concepts.is_empty());
        assert_eq!(*result.distances.get(&id_a).unwrap(), 0);
    }

    #[tokio::test]
    async fn test_spreading_activation_search() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create concepts and relationships
        let concept_a = ConceptNode::new(ConceptType::Entity, "A".to_string(), 0.9, None);
        let concept_b = ConceptNode::new(ConceptType::Entity, "B".to_string(), 0.8, None);
        let concept_c = ConceptNode::new(ConceptType::Entity, "C".to_string(), 0.7, None);

        let id_a = manager.create_concept(concept_a).await.unwrap();
        let id_b = manager.create_concept(concept_b).await.unwrap();
        let id_c = manager.create_concept(concept_c).await.unwrap();

        manager.create_relationship(id_a, id_b, RelationshipType::IsA, 0.9).await.unwrap();
        manager.create_relationship(id_b, id_c, RelationshipType::IsA, 0.8).await.unwrap();

        // Perform spreading activation from A
        let result = manager.traverse_graph(id_a, TraversalAlgorithm::SpreadingActivation, None).await.unwrap();

        assert_eq!(result.algorithm, TraversalAlgorithm::SpreadingActivation);
        assert_eq!(result.start_concept_id, id_a);
        assert!(result.visited_concepts.contains(&id_a));
        
        // Check activation scores
        assert!(!result.activation_scores.is_empty());
        assert_eq!(*result.activation_scores.get(&id_a).unwrap(), 1.0); // Start node has full activation
        
        // Check that activation spreads
        if result.activation_scores.len() > 1 {
            // If activation spread, other nodes should have lower activation
            for (concept_id, activation) in &result.activation_scores {
                if *concept_id != id_a {
                    assert!(*activation < 1.0 && *activation > 0.0);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_shortest_path_finding() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create a path: A -> B -> C
        let concept_a = ConceptNode::new(ConceptType::Entity, "A".to_string(), 0.9, None);
        let concept_b = ConceptNode::new(ConceptType::Entity, "B".to_string(), 0.8, None);
        let concept_c = ConceptNode::new(ConceptType::Entity, "C".to_string(), 0.7, None);
        let concept_d = ConceptNode::new(ConceptType::Entity, "D".to_string(), 0.6, None); // Isolated

        let id_a = manager.create_concept(concept_a).await.unwrap();
        let id_b = manager.create_concept(concept_b).await.unwrap();
        let id_c = manager.create_concept(concept_c).await.unwrap();
        let id_d = manager.create_concept(concept_d).await.unwrap();

        manager.create_relationship(id_a, id_b, RelationshipType::IsA, 0.8).await.unwrap();
        manager.create_relationship(id_b, id_c, RelationshipType::IsA, 0.7).await.unwrap();

        // Find path from A to C
        let path = manager.find_shortest_path(id_a, id_c).await.unwrap();
        assert!(path.is_some());
        
        let path = path.unwrap();
        assert_eq!(path.source_id, id_a);
        assert_eq!(path.target_id, id_c);
        assert_eq!(path.path_length, 2); // A -> B -> C (2 edges)
        assert_eq!(path.concept_path.len(), 3); // A, B, C
        assert_eq!(path.relationship_path.len(), 2); // Two relationships
        assert!(path.average_weight > 0.0);

        // Find path to isolated node (should return None)
        let no_path = manager.find_shortest_path(id_a, id_d).await.unwrap();
        assert!(no_path.is_none());
    }

    #[tokio::test]
    async fn test_subgraph_extraction() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create a small network
        let concept_center = ConceptNode::new(ConceptType::Entity, "center".to_string(), 0.9, None);
        let concept1 = ConceptNode::new(ConceptType::Entity, "node1".to_string(), 0.8, None);
        let concept2 = ConceptNode::new(ConceptType::Entity, "node2".to_string(), 0.7, None);
        let concept_far = ConceptNode::new(ConceptType::Entity, "far".to_string(), 0.6, None);

        let id_center = manager.create_concept(concept_center).await.unwrap();
        let id_1 = manager.create_concept(concept1).await.unwrap();
        let id_2 = manager.create_concept(concept2).await.unwrap();
        let id_far = manager.create_concept(concept_far).await.unwrap();

        // Create relationships: center -> node1 -> node2 -> far
        manager.create_relationship(id_center, id_1, RelationshipType::IsA, 0.8).await.unwrap();
        manager.create_relationship(id_1, id_2, RelationshipType::IsA, 0.7).await.unwrap();
        manager.create_relationship(id_2, id_far, RelationshipType::IsA, 0.6).await.unwrap();

        // Extract subgraph with radius 1 around center
        let subgraph = manager.extract_subgraph(id_center, 1).await.unwrap();

        assert_eq!(subgraph.center_concept_id, Some(id_center));
        assert_eq!(subgraph.radius, Some(1));
        assert_eq!(subgraph.concepts.len(), 2); // center + node1
        assert_eq!(subgraph.relationships.len(), 1); // One relationship within radius
        assert!(subgraph.metrics.total_relationships > 0);

        // Extract larger subgraph with radius 2
        let larger_subgraph = manager.extract_subgraph(id_center, 2).await.unwrap();
        assert!(larger_subgraph.concepts.len() >= subgraph.concepts.len());
        assert!(larger_subgraph.relationships.len() >= subgraph.relationships.len());
    }

    #[tokio::test]
    async fn test_traversal_with_custom_config() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Create concepts
        let concept_a = ConceptNode::new(ConceptType::Entity, "A".to_string(), 0.9, None);
        let concept_b = ConceptNode::new(ConceptType::Entity, "B".to_string(), 0.8, None);

        let id_a = manager.create_concept(concept_a).await.unwrap();
        let id_b = manager.create_concept(concept_b).await.unwrap();

        // Create weak relationship
        manager.create_relationship(id_a, id_b, RelationshipType::IsA, 0.05).await.unwrap();

        // Test with default config (should follow weak relationship)
        let result1 = manager.traverse_graph(id_a, TraversalAlgorithm::BreadthFirst, None).await.unwrap();
        assert!(result1.visited_concepts.len() >= 2); // Should visit both nodes

        // Test with custom config that filters out weak relationships
        let mut custom_config = TraversalConfig::default();
        custom_config.min_relationship_weight = 0.5; // Higher threshold

        let result2 = manager.traverse_graph(id_a, TraversalAlgorithm::BreadthFirst, Some(custom_config)).await.unwrap();
        assert_eq!(result2.visited_concepts.len(), 1); // Should only visit starting node
    }

    #[tokio::test]
    async fn test_configuration_getters_and_setters() {
        let config = ConceptGraphConfig::default();
        let mut manager = ConceptGraphManager::new(config).await.unwrap();

        // Test initial configs
        assert_eq!(manager.formation_config().min_pattern_frequency, 5);
        assert_eq!(manager.traversal_config().max_depth, 5);
        assert_eq!(manager.similarity_config().content_weight, 0.4);

        // Test setters
        let mut new_formation_config = ConceptFormationConfig::default();
        new_formation_config.min_pattern_frequency = 10;
        manager.set_formation_config(new_formation_config);

        let mut new_traversal_config = TraversalConfig::default();
        new_traversal_config.max_depth = 10;
        manager.set_traversal_config(new_traversal_config);

        let mut new_similarity_config = SimilarityConfig::default();
        new_similarity_config.content_weight = 0.5;
        manager.set_similarity_config(new_similarity_config);

        // Verify changes
        assert_eq!(manager.formation_config().min_pattern_frequency, 10);
        assert_eq!(manager.traversal_config().max_depth, 10);
        assert_eq!(manager.similarity_config().content_weight, 0.5);
    }
} 