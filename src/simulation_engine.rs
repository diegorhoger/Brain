//! Simulation Engine Module
//! 
//! This module implements the simulation engine that converts text to state-action graphs
//! and simulates temporal transitions using concept nodes from the concept graph.
//! 
//! ## Task 6.1: State Representation and Text-to-Graph Conversion
//! 
//! Creates foundational components for representing simulation states using concept nodes
//! and converting text inputs into structured state graphs.
//!
//! ## Task 6.3: Branching Simulations and Confidence Scoring
//!
//! Implements branching simulation capabilities with tree-based branch tracking,
//! pruning mechanisms, and enhanced confidence scoring algorithms.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::concept_graph::{ConceptGraphManager, ConceptNode, RelationshipType};
use crate::insight_extraction::{Rule, RuleDatabase, RulePattern, RuleOutcome};

/// Configuration for the simulation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Maximum number of entities to extract from text
    pub max_entities_per_state: usize,
    /// Maximum depth for relationship traversal
    pub max_relationship_depth: usize,
    /// Minimum confidence threshold for concepts to include in state
    pub min_concept_confidence: f64,
    /// Enable detailed parsing logs
    pub enable_parsing_logs: bool,
    /// Timeout for text parsing operations (seconds)
    pub parsing_timeout_seconds: u64,
    /// Maximum state complexity before simplification
    pub max_state_complexity: usize,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_entities_per_state: 50,
            max_relationship_depth: 3,
            min_concept_confidence: 0.3,
            enable_parsing_logs: false,
            parsing_timeout_seconds: 30,
            max_state_complexity: 100,
        }
    }
}

/// Configuration for branching simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingConfig {
    /// Maximum number of branches to explore per simulation step
    pub max_branches_per_step: usize,
    /// Maximum depth of branching tree
    pub max_branching_depth: usize,
    /// Minimum confidence threshold for creating new branches
    pub min_branch_confidence: f64,
    /// Maximum number of active branches at any time
    pub max_active_branches: usize,
    /// Threshold for pruning low-confidence branches
    pub pruning_threshold: f64,
    /// Enable aggressive pruning to manage computational complexity
    pub enable_aggressive_pruning: bool,
    /// Maximum simulation time per branch (seconds)
    pub max_simulation_time_seconds: u64,
}

impl Default for BranchingConfig {
    fn default() -> Self {
        Self {
            max_branches_per_step: 5,
            max_branching_depth: 10,
            min_branch_confidence: 0.4,
            max_active_branches: 50,
            pruning_threshold: 0.3,
            enable_aggressive_pruning: true,
            max_simulation_time_seconds: 300,
        }
    }
}

/// Property of a simulation state entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateProperty {
    /// Property name/key
    pub name: String,
    /// Property value
    pub value: String,
    /// Property type
    pub property_type: PropertyType,
    /// Confidence in this property (0.0 to 1.0)
    pub confidence: f64,
    /// Source of this property (text position, rule, etc.)
    pub source: String,
}

/// Types of properties that can be extracted
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// Physical attribute (color, size, shape)
    Physical,
    /// Location or position
    Location,
    /// Temporal property (time, duration)
    Temporal,
    /// State or condition
    State,
    /// Relationship to other entities
    Relationship,
    /// Action or behavior
    Action,
    /// Quantity or amount
    Quantity,
    /// Abstract property
    Abstract,
}

/// Simulation state representing a scenario at a specific point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    /// Unique identifier for this state
    pub id: Uuid,
    /// Timestamp when this state was created
    pub created_at: DateTime<Utc>,
    /// Entities present in this state (concept IDs)
    pub entities: HashMap<Uuid, ConceptNode>,
    /// Properties of each entity
    pub entity_properties: HashMap<Uuid, Vec<StateProperty>>,
    /// Relationships between entities
    pub relationships: HashMap<(Uuid, Uuid), RelationshipInfo>,
    /// Global state properties (weather, time of day, etc.)
    pub global_properties: Vec<StateProperty>,
    /// Textual description of this state
    pub description: String,
    /// Confidence score for the entire state (0.0 to 1.0)
    pub confidence: f64,
    /// Source text that generated this state
    pub source_text: Option<String>,
    /// Validation status
    pub is_valid: bool,
    /// Validation errors if any
    pub validation_errors: Vec<String>,
}

/// Information about a relationship in the simulation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipInfo {
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Strength/weight of the relationship (0.0 to 1.0)
    pub strength: f64,
    /// Properties of this relationship
    pub properties: Vec<StateProperty>,
    /// Confidence in this relationship (0.0 to 1.0)
    pub confidence: f64,
}

/// State transition representing change from one state to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    /// Unique identifier for this transition
    pub id: Uuid,
    /// Source state ID
    pub from_state_id: Uuid,
    /// Target state ID
    pub to_state_id: Uuid,
    /// Rules that triggered this transition
    pub applied_rules: Vec<Uuid>,
    /// Changes made during transition
    pub changes: Vec<StateChange>,
    /// Confidence in this transition (0.0 to 1.0)
    pub confidence: f64,
    /// Timestamp when transition occurred
    pub timestamp: DateTime<Utc>,
    /// Duration of the transition
    pub duration_ms: u64,
}

/// Individual change within a state transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    /// Type of change
    pub change_type: ChangeType,
    /// Entity affected by the change
    pub entity_id: Option<Uuid>,
    /// Property affected by the change
    pub property_name: Option<String>,
    /// Old value (if applicable)
    pub old_value: Option<String>,
    /// New value (if applicable)
    pub new_value: Option<String>,
    /// Confidence in this change (0.0 to 1.0)
    pub confidence: f64,
}

/// Types of changes that can occur in state transitions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeType {
    /// Entity was added to the state
    EntityAdded,
    /// Entity was removed from the state
    EntityRemoved,
    /// Property value was modified
    PropertyChanged,
    /// Relationship was added
    RelationshipAdded,
    /// Relationship was removed
    RelationshipRemoved,
    /// Relationship strength changed
    RelationshipModified,
    /// Global property changed
    GlobalPropertyChanged,
}

/// Action that can be applied to a simulation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Unique identifier for the action
    pub id: Uuid,
    /// Name/type of the action
    pub name: String,
    /// Description of what the action does
    pub description: String,
    /// Preconditions that must be met for the action to be applicable
    pub preconditions: Vec<Condition>,
    /// Effects that the action will have on the state
    pub effects: Vec<Effect>,
    /// Confidence in the action's applicability (0.0 to 1.0)
    pub confidence: f64,
    /// Duration the action takes to complete (in milliseconds)
    pub duration_ms: u64,
    /// Priority of the action when multiple actions are applicable
    pub priority: ActionPriority,
    /// Context in which the action is applicable
    pub context: HashMap<String, String>,
}

/// Priority levels for actions
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ActionPriority {
    /// Low priority action
    Low,
    /// Medium priority action
    Medium,
    /// High priority action
    High,
    /// Critical priority action
    Critical,
}

/// Condition that must be met for an action to be applicable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Type of condition
    pub condition_type: ConditionType,
    /// Entity the condition applies to (if applicable)
    pub entity_id: Option<Uuid>,
    /// Property name the condition checks (if applicable)
    pub property_name: Option<String>,
    /// Expected value for the condition
    pub expected_value: String,
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Confidence required for the condition to be considered met
    pub required_confidence: f64,
}

/// Types of conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    /// Entity must exist in the state
    EntityExists,
    /// Entity must not exist in the state
    EntityNotExists,
    /// Property must have a specific value
    PropertyEquals,
    /// Property must not have a specific value
    PropertyNotEquals,
    /// Relationship must exist between entities
    RelationshipExists,
    /// Relationship must not exist between entities
    RelationshipNotExists,
    /// Global property condition
    GlobalProperty,
    /// Custom condition based on rule pattern
    CustomPattern,
}

/// Comparison operators for conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Equal to
    Equals,
    /// Not equal to
    NotEquals,
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Contains substring
    Contains,
    /// Does not contain substring
    NotContains,
    /// Matches regex pattern
    Matches,
}

/// Effect that an action has on the simulation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    /// Type of effect
    pub effect_type: EffectType,
    /// Entity the effect applies to (if applicable)
    pub entity_id: Option<Uuid>,
    /// Property name the effect modifies (if applicable)
    pub property_name: Option<String>,
    /// New value to set
    pub new_value: Option<String>,
    /// Probability that this effect occurs (0.0 to 1.0)
    pub probability: f64,
    /// Delay before the effect takes place (in milliseconds)
    pub delay_ms: u64,
}

/// Types of effects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    /// Add a new entity to the state
    AddEntity,
    /// Remove an entity from the state
    RemoveEntity,
    /// Set a property value
    SetProperty,
    /// Modify a property value
    ModifyProperty,
    /// Add a relationship between entities
    AddRelationship,
    /// Remove a relationship between entities
    RemoveRelationship,
    /// Modify relationship strength
    ModifyRelationship,
    /// Set a global property
    SetGlobalProperty,
    /// Trigger another action
    TriggerAction,
}

/// Configuration for action modeling and transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    /// Maximum number of actions to consider per step
    pub max_actions_per_step: usize,
    /// Minimum confidence threshold for applying actions
    pub min_action_confidence: f64,
    /// Maximum number of concurrent actions
    pub max_concurrent_actions: usize,
    /// Enable conflict resolution between actions
    pub enable_conflict_resolution: bool,
    /// Timeout for action application (milliseconds)
    pub action_timeout_ms: u64,
    /// Enable temporal logic for action sequencing
    pub enable_temporal_logic: bool,
    /// Maximum depth for action chaining
    pub max_action_chain_depth: usize,
}

impl Default for ActionConfig {
    fn default() -> Self {
        Self {
            max_actions_per_step: 10,
            min_action_confidence: 0.5,
            max_concurrent_actions: 3,
            enable_conflict_resolution: true,
            action_timeout_ms: 5000,
            enable_temporal_logic: true,
            max_action_chain_depth: 5,
        }
    }
}

/// Result of applying an action to a state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// The action that was applied
    pub action_id: Uuid,
    /// Whether the action was successfully applied
    pub success: bool,
    /// Changes made to the state
    pub changes: Vec<StateChange>,
    /// Confidence in the result (0.0 to 1.0)
    pub confidence: f64,
    /// Time taken to apply the action (milliseconds)
    pub execution_time_ms: u64,
    /// Any errors that occurred during application
    pub errors: Vec<String>,
    /// Side effects that occurred
    pub side_effects: Vec<Effect>,
}

/// Conflict between multiple applicable actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConflict {
    /// Actions that are in conflict
    pub conflicting_actions: Vec<Uuid>,
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Severity of the conflict (0.0 to 1.0)
    pub severity: f64,
    /// Suggested resolution
    pub resolution: ConflictResolution,
}

/// Types of conflicts between actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Actions modify the same property with different values
    PropertyConflict,
    /// Actions have contradictory effects
    EffectContradiction,
    /// Actions compete for the same resource
    ResourceConflict,
    /// Actions have incompatible preconditions
    PreconditionConflict,
    /// Actions create temporal inconsistencies
    TemporalConflict,
}

/// Resolution strategies for action conflicts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Apply the action with higher priority
    HigherPriority,
    /// Apply the action with higher confidence
    HigherConfidence,
    /// Apply actions in sequence
    Sequential,
    /// Apply both actions and merge effects
    Merge,
    /// Skip all conflicting actions
    Skip,
    /// Require manual resolution
    Manual,
}

/// A single simulation branch in the branching tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationBranch {
    /// Unique identifier for this branch
    pub id: Uuid,
    /// Parent branch ID (None for root)
    pub parent_id: Option<Uuid>,
    /// Child branch IDs
    pub child_ids: Vec<Uuid>,
    /// Current state of this branch
    pub current_state: SimulationState,
    /// Sequence of transitions taken to reach this state
    pub transition_history: Vec<StateTransition>,
    /// Accumulated confidence score for this branch
    pub accumulated_confidence: f64,
    /// Depth in the branching tree
    pub depth: usize,
    /// Whether this branch is still active
    pub is_active: bool,
    /// Timestamp when branch was created
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
    /// Reason for pruning (if inactive)
    pub pruning_reason: Option<String>,
    /// Custom metadata for this branch
    pub metadata: HashMap<String, String>,
}

/// Result of a branching simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingResult {
    /// All branches created during simulation
    pub branches: HashMap<Uuid, SimulationBranch>,
    /// Root branch ID
    pub root_branch_id: Uuid,
    /// Most likely outcome branches (sorted by confidence)
    pub most_likely_outcomes: Vec<Uuid>,
    /// Total number of branches explored
    pub total_branches_explored: usize,
    /// Total number of branches pruned
    pub total_branches_pruned: usize,
    /// Overall confidence in the simulation results
    pub overall_confidence: f64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Branching statistics
    pub branching_stats: BranchingStats,
}

/// Statistics about the branching simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingStats {
    /// Average confidence across all branches
    pub average_confidence: f64,
    /// Maximum depth reached
    pub max_depth_reached: usize,
    /// Average depth of active branches
    pub average_depth: f64,
    /// Number of terminal branches (no further expansion)
    pub terminal_branches: usize,
    /// Branch diversity score (0.0 to 1.0)
    pub diversity_score: f64,
    /// Computational complexity score
    pub complexity_score: f64,
}

/// Constraints that can be injected into simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConstraint {
    /// Unique identifier for the constraint
    pub id: Uuid,
    /// Type of constraint
    pub constraint_type: ConstraintType,
    /// Description of the constraint
    pub description: String,
    /// Condition that must be satisfied
    pub condition: Condition,
    /// Weight/importance of this constraint (0.0 to 1.0)
    pub weight: f64,
    /// Whether the constraint is mandatory or optional
    pub is_mandatory: bool,
}

/// Types of simulation constraints
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstraintType {
    /// Must avoid certain states or conditions
    Avoidance,
    /// Must achieve certain states or conditions
    Achievement,
    /// Must maintain certain properties throughout simulation
    Maintenance,
    /// Must follow specific sequences or patterns
    Sequence,
    /// Resource or time limitations
    Resource,
    /// Probabilistic constraints
    Probabilistic,
}

/// Confidence scoring algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceConfig {
    /// Weight for rule confidence in overall scoring
    pub rule_confidence_weight: f64,
    /// Weight for path likelihood in overall scoring
    pub path_likelihood_weight: f64,
    /// Weight for state consistency in overall scoring
    pub state_consistency_weight: f64,
    /// Weight for historical accuracy in overall scoring
    pub historical_accuracy_weight: f64,
    /// Decay factor for accumulated confidence over time
    pub confidence_decay_factor: f64,
    /// Bonus for branches that satisfy constraints
    pub constraint_satisfaction_bonus: f64,
}

impl Default for ConfidenceConfig {
    fn default() -> Self {
        Self {
            rule_confidence_weight: 0.4,
            path_likelihood_weight: 0.3,
            state_consistency_weight: 0.2,
            historical_accuracy_weight: 0.1,
            confidence_decay_factor: 0.95,
            constraint_satisfaction_bonus: 0.1,
        }
    }
}

impl SimulationState {
    /// Create a new empty simulation state
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            entities: HashMap::new(),
            entity_properties: HashMap::new(),
            relationships: HashMap::new(),
            global_properties: Vec::new(),
            description: String::new(),
            confidence: 1.0,
            source_text: None,
            is_valid: true,
            validation_errors: Vec::new(),
        }
    }

    /// Add an entity to the state
    pub fn add_entity(&mut self, concept: ConceptNode, properties: Vec<StateProperty>) -> Uuid {
        let entity_id = concept.id;
        self.entities.insert(entity_id, concept);
        self.entity_properties.insert(entity_id, properties);
        entity_id
    }

    /// Add a relationship between two entities
    pub fn add_relationship(
        &mut self,
        entity1_id: Uuid,
        entity2_id: Uuid,
        relationship_info: RelationshipInfo,
    ) -> Result<()> {
        if !self.entities.contains_key(&entity1_id) {
            return Err(anyhow::anyhow!("Entity {} not found in state", entity1_id));
        }
        if !self.entities.contains_key(&entity2_id) {
            return Err(anyhow::anyhow!("Entity {} not found in state", entity2_id));
        }

        self.relationships.insert((entity1_id, entity2_id), relationship_info);
        Ok(())
    }

    /// Get entity by ID
    pub fn get_entity(&self, entity_id: Uuid) -> Option<&ConceptNode> {
        self.entities.get(&entity_id)
    }

    /// Get properties of an entity
    pub fn get_entity_properties(&self, entity_id: Uuid) -> Option<&Vec<StateProperty>> {
        self.entity_properties.get(&entity_id)
    }

    /// Get relationship between two entities
    pub fn get_relationship(&self, entity1_id: Uuid, entity2_id: Uuid) -> Option<&RelationshipInfo> {
        self.relationships.get(&(entity1_id, entity2_id))
            .or_else(|| self.relationships.get(&(entity2_id, entity1_id)))
    }

    /// Calculate the complexity of this state
    pub fn complexity(&self) -> usize {
        self.entities.len() + self.relationships.len() + self.global_properties.len()
    }

    /// Update state description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Set source text
    pub fn set_source_text(&mut self, source_text: String) {
        self.source_text = Some(source_text);
    }

    /// Mark state as invalid with errors
    pub fn invalidate(&mut self, errors: Vec<String>) {
        self.is_valid = false;
        self.validation_errors = errors;
    }
}

/// Text-to-state parser for converting text descriptions into simulation states
pub struct TextToStateParser {
    /// Configuration for parsing
    config: SimulationConfig,
    /// Concept graph manager for entity identification
    concept_graph: ConceptGraphManager,
}

impl TextToStateParser {
    /// Create a new parser with the concept graph
    pub fn new(concept_graph: ConceptGraphManager) -> Self {
        Self {
            config: SimulationConfig::default(),
            concept_graph,
        }
    }

    /// Create a new parser with custom configuration
    pub fn with_config(concept_graph: ConceptGraphManager, config: SimulationConfig) -> Self {
        Self {
            config,
            concept_graph,
        }
    }

    /// Parse text into a simulation state
    pub async fn parse_text_to_state(&mut self, text: &str) -> Result<SimulationState> {
        let start_time = std::time::Instant::now();
        
        // Create new state
        let mut state = SimulationState::new();
        state.set_source_text(text.to_string());
        state.set_description(format!("State parsed from: {}", text.chars().take(50).collect::<String>()));

        // Extract entities from text
        let entities = self.extract_entities_from_text(text).await
            .context("Failed to extract entities from text")?;

        // Add entities to state
        for (concept, properties) in entities {
            state.add_entity(concept, properties);
        }

        // Extract relationships
        let relationships = self.extract_relationships_from_text(text, &state).await
            .context("Failed to extract relationships from text")?;

        // Add relationships to state
        for ((entity1_id, entity2_id), relationship_info) in relationships {
            if let Err(e) = state.add_relationship(entity1_id, entity2_id, relationship_info) {
                if self.config.enable_parsing_logs {
                    println!("Warning: Failed to add relationship: {}", e);
                }
            }
        }

        // Extract global properties
        let global_properties = self.extract_global_properties(text).await
            .context("Failed to extract global properties")?;
        state.global_properties = global_properties;

        // Calculate overall confidence
        state.confidence = self.calculate_state_confidence(&state);

        let elapsed = start_time.elapsed();
        if self.config.enable_parsing_logs {
            println!("Parsing completed in {:.2}ms", elapsed.as_secs_f64() * 1000.0);
        }

        Ok(state)
    }

    /// Extract entities from text using concept graph
    async fn extract_entities_from_text(&mut self, text: &str) -> Result<Vec<(ConceptNode, Vec<StateProperty>)>> {
        let mut entities = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for word in words.iter().take(self.config.max_entities_per_state) {
            // Simple word matching - could be enhanced with NLP
            if let Some(concept) = self.find_concept_for_word(word).await? {
                if concept.confidence_score >= self.config.min_concept_confidence {
                    let properties = self.extract_properties_for_word(word, text).await?;
                    entities.push((concept, properties));
                }
            }
        }

        Ok(entities)
    }

    /// Find concept in the concept graph that matches a word
    async fn find_concept_for_word(&mut self, word: &str) -> Result<Option<ConceptNode>> {
        // Query concepts by content pattern
        let query = crate::concept_graph::ConceptQuery {
            content_pattern: Some(word.to_lowercase()),
            min_confidence: Some(self.config.min_concept_confidence),
            limit: Some(1),
            ..Default::default()
        };

        let concepts = self.concept_graph.query_concepts(&query).await?;
        Ok(concepts.into_iter().next())
    }

    /// Extract properties for a word from the text context
    async fn extract_properties_for_word(&self, word: &str, text: &str) -> Result<Vec<StateProperty>> {
        let mut properties = Vec::new();

        // Simple pattern matching for properties
        // In a full implementation, this would use NLP and the concept graph

        // Look for adjectives before the word
        if let Some(adjective) = self.find_adjective_before_word(word, text) {
            properties.push(StateProperty {
                name: "description".to_string(),
                value: adjective,
                property_type: PropertyType::Physical,
                confidence: 0.7,
                source: "text_analysis".to_string(),
            });
        }

        // Look for location indicators
        if let Some(location) = self.find_location_for_word(word, text) {
            properties.push(StateProperty {
                name: "location".to_string(),
                value: location,
                property_type: PropertyType::Location,
                confidence: 0.6,
                source: "text_analysis".to_string(),
            });
        }

        Ok(properties)
    }

    /// Find adjective before a word (simple implementation)
    fn find_adjective_before_word(&self, word: &str, text: &str) -> Option<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        
        for (i, w) in words.iter().enumerate() {
            if w.to_lowercase() == word.to_lowercase() && i > 0 {
                let prev_word = words[i - 1];
                // Simple heuristic: if previous word ends with certain suffixes, treat as adjective
                if prev_word.ends_with("ed") || prev_word.ends_with("ing") || 
                   prev_word.ends_with("ly") || prev_word.len() > 3 {
                    return Some(prev_word.to_string());
                }
            }
        }
        None
    }

    /// Find location indicators for a word
    fn find_location_for_word(&self, _word: &str, text: &str) -> Option<String> {
        // Look for location prepositions
        let location_indicators = ["in", "at", "on", "near", "under", "over", "beside"];
        
        for indicator in &location_indicators {
            if let Some(pos) = text.to_lowercase().find(indicator) {
                // Extract text after the location indicator
                let after_indicator = &text[pos + indicator.len()..];
                let location_words: Vec<&str> = after_indicator.split_whitespace().take(3).collect();
                if !location_words.is_empty() {
                    return Some(location_words.join(" "));
                }
            }
        }
        None
    }

    /// Extract relationships from text based on state entities
    async fn extract_relationships_from_text(
        &self,
        text: &str,
        state: &SimulationState,
    ) -> Result<HashMap<(Uuid, Uuid), RelationshipInfo>> {
        let mut relationships = HashMap::new();

        // Simple relationship extraction based on proximity and common patterns
        let entities: Vec<Uuid> = state.entities.keys().cloned().collect();

        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let entity1_id = entities[i];
                let entity2_id = entities[j];

                if let (Some(entity1), Some(entity2)) = (
                    state.entities.get(&entity1_id),
                    state.entities.get(&entity2_id),
                ) {
                    // Check if entities appear near each other in text
                    if self.entities_are_related_in_text(&entity1.content, &entity2.content, text) {
                        let relationship_info = RelationshipInfo {
                            relationship_type: RelationshipType::AssociatedWith,
                            strength: 0.5,
                            properties: Vec::new(),
                            confidence: 0.5,
                        };
                        relationships.insert((entity1_id, entity2_id), relationship_info);
                    }
                }
            }
        }

        Ok(relationships)
    }

    /// Check if two entities are related in text based on proximity
    fn entities_are_related_in_text(&self, entity1: &str, entity2: &str, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        let entity1_lower = entity1.to_lowercase();
        let entity2_lower = entity2.to_lowercase();

        if let (Some(pos1), Some(pos2)) = (text_lower.find(&entity1_lower), text_lower.find(&entity2_lower)) {
            // Consider entities related if they appear within 50 characters of each other
            (pos1 as i32 - pos2 as i32).abs() < 50
        } else {
            false
        }
    }

    /// Extract global properties from text
    async fn extract_global_properties(&self, text: &str) -> Result<Vec<StateProperty>> {
        let mut properties = Vec::new();

        // Look for temporal indicators
        let time_indicators = ["morning", "afternoon", "evening", "night", "dawn", "dusk"];
        for indicator in &time_indicators {
            if text.to_lowercase().contains(indicator) {
                properties.push(StateProperty {
                    name: "time_of_day".to_string(),
                    value: indicator.to_string(),
                    property_type: PropertyType::Temporal,
                    confidence: 0.8,
                    source: "text_analysis".to_string(),
                });
                break;
            }
        }

        // Look for weather indicators
        let weather_indicators = ["sunny", "rainy", "cloudy", "stormy", "clear", "foggy"];
        for indicator in &weather_indicators {
            if text.to_lowercase().contains(indicator) {
                properties.push(StateProperty {
                    name: "weather".to_string(),
                    value: indicator.to_string(),
                    property_type: PropertyType::Physical,
                    confidence: 0.7,
                    source: "text_analysis".to_string(),
                });
                break;
            }
        }

        Ok(properties)
    }

    /// Calculate overall confidence for the state
    fn calculate_state_confidence(&self, state: &SimulationState) -> f64 {
        if state.entities.is_empty() {
            return 0.0;
        }

        let entity_confidence: f64 = state.entities.values()
            .map(|e| e.confidence_score)
            .sum::<f64>() / state.entities.len() as f64;

        let relationship_confidence: f64 = if state.relationships.is_empty() {
            0.5 // Neutral if no relationships
        } else {
            state.relationships.values()
                .map(|r| r.confidence)
                .sum::<f64>() / state.relationships.len() as f64
        };

        let property_confidence: f64 = if state.global_properties.is_empty() {
            0.5
        } else {
            state.global_properties.iter()
                .map(|p| p.confidence)
                .sum::<f64>() / state.global_properties.len() as f64
        };

        // Weighted average
        entity_confidence * 0.5 + relationship_confidence * 0.3 + property_confidence * 0.2
    }

    /// Get configuration
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: SimulationConfig) {
        self.config = config;
    }
}

/// State validator for ensuring simulation state consistency
pub struct StateValidator {
    /// Validation configuration
    config: SimulationConfig,
}

impl StateValidator {
    /// Create a new state validator
    pub fn new() -> Self {
        Self {
            config: SimulationConfig::default(),
        }
    }

    /// Create validator with custom configuration
    pub fn with_config(config: SimulationConfig) -> Self {
        Self { config }
    }

    /// Validate a simulation state
    pub fn validate_state(&self, state: &mut SimulationState) -> Result<bool> {
        let mut errors = Vec::new();

        // Check complexity
        if state.complexity() > self.config.max_state_complexity {
            errors.push(format!(
                "State complexity {} exceeds maximum {}", 
                state.complexity(), 
                self.config.max_state_complexity
            ));
        }

        // Validate entities
        for (entity_id, entity) in &state.entities {
            if entity.confidence_score < self.config.min_concept_confidence {
                errors.push(format!(
                    "Entity {} has confidence {} below threshold {}", 
                    entity_id, 
                    entity.confidence_score, 
                    self.config.min_concept_confidence
                ));
            }
        }

        // Validate relationships
        for ((entity1_id, entity2_id), relationship) in &state.relationships {
            if !state.entities.contains_key(entity1_id) {
                errors.push(format!("Relationship references non-existent entity {}", entity1_id));
            }
            if !state.entities.contains_key(entity2_id) {
                errors.push(format!("Relationship references non-existent entity {}", entity2_id));
            }
            if relationship.confidence < 0.1 {
                errors.push(format!("Relationship has very low confidence: {}", relationship.confidence));
            }
        }

        // Update state validation status
        if errors.is_empty() {
            state.is_valid = true;
            state.validation_errors.clear();
            Ok(true)
        } else {
            state.invalidate(errors);
            Ok(false)
        }
    }

    /// Validate state transition
    pub fn validate_transition(&self, transition: &StateTransition) -> Result<Vec<String>> {
        let mut errors = Vec::new();

        if transition.confidence < 0.1 {
            errors.push("Transition has very low confidence".to_string());
        }

        if transition.applied_rules.is_empty() && !transition.changes.is_empty() {
            errors.push("Transition has changes but no applied rules".to_string());
        }

        Ok(errors)
    }

    /// Get configuration
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }
}

/// Main simulation engine that orchestrates state management and transitions
pub struct SimulationEngine {
    /// Configuration
    config: SimulationConfig,
    /// Text-to-state parser
    parser: TextToStateParser,
    /// State validator
    validator: StateValidator,
    /// Current simulation state
    current_state: Option<SimulationState>,
    /// History of states
    state_history: Vec<SimulationState>,
    /// History of transitions
    transition_history: Vec<StateTransition>,
    /// Rule database for action modeling
    rule_database: RuleDatabase,
    /// Action configuration
    action_config: ActionConfig,
    /// Available actions
    available_actions: Vec<Action>,
    /// Branching simulation configuration
    branching_config: BranchingConfig,
    /// Confidence scoring configuration
    confidence_config: ConfidenceConfig,
    /// Current simulation constraints
    constraints: Vec<SimulationConstraint>,
}

impl SimulationEngine {
    /// Create a new simulation engine
    pub fn new(concept_graph: ConceptGraphManager) -> Self {
        let config = SimulationConfig::default();
        let parser = TextToStateParser::with_config(concept_graph, config.clone());
        let validator = StateValidator::with_config(config.clone());

        Self {
            config,
            parser,
            validator,
            current_state: None,
            state_history: Vec::new(),
            transition_history: Vec::new(),
            rule_database: RuleDatabase::new(),
            action_config: ActionConfig::default(),
            available_actions: Vec::new(),
            branching_config: BranchingConfig::default(),
            confidence_config: ConfidenceConfig::default(),
            constraints: Vec::new(),
        }
    }

    /// Initialize simulation with text input
    pub async fn initialize_from_text(&mut self, text: &str) -> Result<Uuid> {
        let mut state = self.parser.parse_text_to_state(text).await?;
        
        // Validate the state
        self.validator.validate_state(&mut state)?;

        let state_id = state.id;
        self.current_state = Some(state.clone());
        self.state_history.push(state);

        Ok(state_id)
    }

    /// Get current simulation state
    pub fn get_current_state(&self) -> Option<&SimulationState> {
        self.current_state.as_ref()
    }

    /// Get state history
    pub fn get_state_history(&self) -> &[SimulationState] {
        &self.state_history
    }

    /// Get transition history
    pub fn get_transition_history(&self) -> &[StateTransition] {
        &self.transition_history
    }

    /// Reset simulation
    pub fn reset(&mut self) {
        self.current_state = None;
        self.state_history.clear();
        self.transition_history.clear();
    }

    /// Get configuration
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: SimulationConfig) {
        self.config = config.clone();
        self.parser.set_config(config.clone());
        self.validator = StateValidator::with_config(config);
    }

    /// Add an action to the available actions
    pub fn add_action(&mut self, action: Action) {
        self.available_actions.push(action);
    }

    /// Create action from rule
    pub fn create_action_from_rule(&self, rule: &Rule) -> Result<Action> {
        let action_id = Uuid::new_v4();
        let mut preconditions = Vec::new();
        let mut effects = Vec::new();

        // Convert rule pattern to preconditions
        match &rule.pattern {
            RulePattern::Single(element) => {
                preconditions.push(Condition {
                    condition_type: ConditionType::CustomPattern,
                    entity_id: None,
                    property_name: None,
                    expected_value: element.clone(),
                    operator: ComparisonOperator::Contains,
                    required_confidence: rule.metrics.confidence * 0.8,
                });
            }
            RulePattern::Sequence(elements) => {
                for element in elements {
                    preconditions.push(Condition {
                        condition_type: ConditionType::CustomPattern,
                        entity_id: None,
                        property_name: None,
                        expected_value: element.clone(),
                        operator: ComparisonOperator::Contains,
                        required_confidence: rule.metrics.confidence * 0.8,
                    });
                }
            }
            RulePattern::CoOccurrence(elements) => {
                for element in elements {
                    preconditions.push(Condition {
                        condition_type: ConditionType::CustomPattern,
                        entity_id: None,
                        property_name: None,
                        expected_value: element.clone(),
                        operator: ComparisonOperator::Contains,
                        required_confidence: rule.metrics.confidence * 0.7,
                    });
                }
            }
            _ => {
                // For other pattern types, create a generic condition
                preconditions.push(Condition {
                    condition_type: ConditionType::CustomPattern,
                    entity_id: None,
                    property_name: None,
                    expected_value: "pattern_match".to_string(),
                    operator: ComparisonOperator::Contains,
                    required_confidence: rule.metrics.confidence,
                });
            }
        }

        // Convert rule outcome to effects
        match &rule.outcome {
            RuleOutcome::Single(outcome) => {
                effects.push(Effect {
                    effect_type: EffectType::SetGlobalProperty,
                    entity_id: None,
                    property_name: Some("outcome".to_string()),
                    new_value: Some(outcome.clone()),
                    probability: rule.metrics.confidence,
                    delay_ms: 0,
                });
            }
            RuleOutcome::StateChange { from_state: _, to_state } => {
                effects.push(Effect {
                    effect_type: EffectType::SetGlobalProperty,
                    entity_id: None,
                    property_name: Some("state".to_string()),
                    new_value: Some(to_state.clone()),
                    probability: rule.metrics.confidence,
                    delay_ms: 100,
                });
            }
            RuleOutcome::Action(action_name) => {
                effects.push(Effect {
                    effect_type: EffectType::TriggerAction,
                    entity_id: None,
                    property_name: Some("action".to_string()),
                    new_value: Some(action_name.clone()),
                    probability: rule.metrics.confidence,
                    delay_ms: 50,
                });
            }
            _ => {
                // Default effect for other outcome types
                effects.push(Effect {
                    effect_type: EffectType::SetGlobalProperty,
                    entity_id: None,
                    property_name: Some("rule_applied".to_string()),
                    new_value: Some(rule.id.to_string()),
                    probability: rule.metrics.confidence,
                    delay_ms: 0,
                });
            }
        }

        let priority = if rule.metrics.confidence > 0.8 {
            ActionPriority::High
        } else if rule.metrics.confidence > 0.6 {
            ActionPriority::Medium
        } else {
            ActionPriority::Low
        };

        Ok(Action {
            id: action_id,
            name: format!("rule_action_{}", rule.id),
            description: format!("Action derived from rule pattern: {:?}", rule.pattern),
            preconditions,
            effects,
            confidence: rule.metrics.confidence,
            duration_ms: 1000,
            priority,
            context: rule.context.clone(),
        })
    }

    /// Find applicable actions for the current state
    pub fn find_applicable_actions(&self) -> Result<Vec<Uuid>> {
        let current_state = match &self.current_state {
            Some(state) => state,
            None => return Ok(Vec::new()),
        };

        let mut applicable_actions = Vec::new();

        for action in &self.available_actions {
            if self.is_action_applicable(action, current_state)? {
                applicable_actions.push(action.id);
            }
        }

        // Also check rules for applicable actions
        let active_rules = self.rule_database.get_active_rules();
        for rule in active_rules {
            if self.is_rule_applicable(rule, current_state)? {
                // Create action from rule if it doesn't exist
                if let Ok(rule_action) = self.create_action_from_rule(rule) {
                    applicable_actions.push(rule_action.id);
                }
            }
        }

        Ok(applicable_actions)
    }

    /// Check if an action is applicable to the current state
    fn is_action_applicable(&self, action: &Action, state: &SimulationState) -> Result<bool> {
        if action.confidence < self.action_config.min_action_confidence {
            return Ok(false);
        }

        for condition in &action.preconditions {
            if !self.evaluate_condition(condition, state)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check if a rule is applicable to the current state
    fn is_rule_applicable(&self, rule: &Rule, state: &SimulationState) -> Result<bool> {
        if !rule.is_active || rule.metrics.confidence < self.action_config.min_action_confidence {
            return Ok(false);
        }

        // Check if rule pattern matches current state
        match &rule.pattern {
            RulePattern::Single(element) => {
                self.state_contains_element(state, element)
            }
            RulePattern::Sequence(elements) => {
                for element in elements {
                    if !self.state_contains_element(state, element)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            RulePattern::CoOccurrence(elements) => {
                let mut found_count = 0;
                for element in elements {
                    if self.state_contains_element(state, element)? {
                        found_count += 1;
                    }
                }
                Ok(found_count >= elements.len() / 2) // At least half must be present
            }
            _ => Ok(true), // Default to applicable for other pattern types
        }
    }

    /// Check if state contains a specific element
    fn state_contains_element(&self, state: &SimulationState, element: &str) -> Result<bool> {
        // Check entity content
        for concept in state.entities.values() {
            if concept.content.to_lowercase().contains(&element.to_lowercase()) {
                return Ok(true);
            }
        }

        // Check property values
        for properties in state.entity_properties.values() {
            for property in properties {
                if property.value.to_lowercase().contains(&element.to_lowercase()) {
                    return Ok(true);
                }
            }
        }

        // Check global properties
        for property in &state.global_properties {
            if property.value.to_lowercase().contains(&element.to_lowercase()) {
                return Ok(true);
            }
        }

        // Check description
        if state.description.to_lowercase().contains(&element.to_lowercase()) {
            return Ok(true);
        }

        Ok(false)
    }

    /// Evaluate a condition against the current state
    fn evaluate_condition(&self, condition: &Condition, state: &SimulationState) -> Result<bool> {
        match condition.condition_type {
            ConditionType::EntityExists => {
                if let Some(entity_id) = condition.entity_id {
                    Ok(state.entities.contains_key(&entity_id))
                } else {
                    Ok(false)
                }
            }
            ConditionType::EntityNotExists => {
                if let Some(entity_id) = condition.entity_id {
                    Ok(!state.entities.contains_key(&entity_id))
                } else {
                    Ok(true)
                }
            }
            ConditionType::PropertyEquals => {
                if let (Some(entity_id), Some(property_name)) = (condition.entity_id, &condition.property_name) {
                    if let Some(properties) = state.entity_properties.get(&entity_id) {
                        for property in properties {
                            if property.name == *property_name {
                                return Ok(self.compare_values(&property.value, &condition.expected_value, &condition.operator));
                            }
                        }
                    }
                }
                Ok(false)
            }
            ConditionType::GlobalProperty => {
                if let Some(property_name) = &condition.property_name {
                    for property in &state.global_properties {
                        if property.name == *property_name {
                            return Ok(self.compare_values(&property.value, &condition.expected_value, &condition.operator));
                        }
                    }
                }
                Ok(false)
            }
            ConditionType::CustomPattern => {
                self.state_contains_element(state, &condition.expected_value)
            }
            _ => Ok(true), // Default to true for other condition types
        }
    }

    /// Compare values using the specified operator
    fn compare_values(&self, actual: &str, expected: &str, operator: &ComparisonOperator) -> bool {
        match operator {
            ComparisonOperator::Equals => actual == expected,
            ComparisonOperator::NotEquals => actual != expected,
            ComparisonOperator::Contains => actual.to_lowercase().contains(&expected.to_lowercase()),
            ComparisonOperator::NotContains => !actual.to_lowercase().contains(&expected.to_lowercase()),
            ComparisonOperator::GreaterThan => {
                if let (Ok(a), Ok(e)) = (actual.parse::<f64>(), expected.parse::<f64>()) {
                    a > e
                } else {
                    actual > expected
                }
            }
            ComparisonOperator::LessThan => {
                if let (Ok(a), Ok(e)) = (actual.parse::<f64>(), expected.parse::<f64>()) {
                    a < e
                } else {
                    actual < expected
                }
            }
            ComparisonOperator::Matches => {
                // Simple regex matching - in production, use proper regex
                actual.contains(expected)
            }
        }
    }

    /// Apply an action to the current state
    pub async fn apply_action(&mut self, action_id: Uuid) -> Result<ActionResult> {
        let start_time = std::time::Instant::now();
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let side_effects = Vec::new();

        // Find the action
        let action = self.available_actions.iter()
            .find(|a| a.id == action_id)
            .ok_or_else(|| anyhow::anyhow!("Action not found: {}", action_id))?
            .clone();

        // Check if action is still applicable
        if let Some(current_state) = &self.current_state {
            if !self.is_action_applicable(&action, current_state)? {
                return Ok(ActionResult {
                    action_id,
                    success: false,
                    changes: Vec::new(),
                    confidence: 0.0,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                    errors: vec!["Action is no longer applicable".to_string()],
                    side_effects: Vec::new(),
                });
            }
        } else {
            return Ok(ActionResult {
                action_id,
                success: false,
                changes: Vec::new(),
                confidence: 0.0,
                execution_time_ms: start_time.elapsed().as_millis() as u64,
                errors: vec!["No current state available".to_string()],
                side_effects: Vec::new(),
            });
        }

        // Apply effects
        let mut new_state = self.current_state.as_ref().unwrap().clone();
        let mut total_confidence = 0.0;
        let mut effect_count = 0;

        for effect in &action.effects {
            if rand::random::<f64>() <= effect.probability {
                match self.apply_effect(effect, &mut new_state) {
                    Ok(change) => {
                        changes.push(change);
                        total_confidence += effect.probability;
                        effect_count += 1;
                    }
                    Err(e) => {
                        errors.push(format!("Failed to apply effect: {}", e));
                    }
                }
            }
        }

        let success = errors.is_empty() && !changes.is_empty();
        let confidence = if effect_count > 0 {
            (total_confidence / effect_count as f64) * action.confidence
        } else {
            0.0
        };

        // Update current state if successful
        if success {
            let new_state_id = new_state.id;
            self.current_state = Some(new_state.clone());
            self.state_history.push(new_state);

            // Create transition record
            let transition = StateTransition {
                id: Uuid::new_v4(),
                from_state_id: self.state_history[self.state_history.len() - 2].id,
                to_state_id: new_state_id,
                applied_rules: vec![], // Could be populated if action came from rule
                changes: changes.clone(),
                confidence,
                timestamp: Utc::now(),
                duration_ms: start_time.elapsed().as_millis() as u64,
            };
            self.transition_history.push(transition);
        }

        Ok(ActionResult {
            action_id,
            success,
            changes,
            confidence,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            errors,
            side_effects,
        })
    }

    /// Apply a single effect to the state
    fn apply_effect(&self, effect: &Effect, state: &mut SimulationState) -> Result<StateChange> {
        match effect.effect_type {
            EffectType::SetProperty => {
                if let (Some(entity_id), Some(property_name), Some(new_value)) = 
                    (effect.entity_id, &effect.property_name, &effect.new_value) {
                    
                    let old_value = if let Some(properties) = state.entity_properties.get_mut(&entity_id) {
                        let mut old_val = None;
                        for property in properties.iter_mut() {
                            if property.name == *property_name {
                                old_val = Some(property.value.clone());
                                property.value = new_value.clone();
                                break;
                            }
                        }
                        old_val
                    } else {
                        None
                    };

                    Ok(StateChange {
                        change_type: ChangeType::PropertyChanged,
                        entity_id: Some(entity_id),
                        property_name: Some(property_name.clone()),
                        old_value,
                        new_value: Some(new_value.clone()),
                        confidence: effect.probability,
                    })
                } else {
                    Err(anyhow::anyhow!("Invalid property effect parameters"))
                }
            }
            EffectType::SetGlobalProperty => {
                if let (Some(property_name), Some(new_value)) = (&effect.property_name, &effect.new_value) {
                    let old_value = state.global_properties.iter()
                        .find(|p| p.name == *property_name)
                        .map(|p| p.value.clone());

                    // Update or add global property
                    if let Some(property) = state.global_properties.iter_mut()
                        .find(|p| p.name == *property_name) {
                        property.value = new_value.clone();
                    } else {
                        state.global_properties.push(StateProperty {
                            name: property_name.clone(),
                            value: new_value.clone(),
                            property_type: PropertyType::Abstract,
                            confidence: effect.probability,
                            source: "action_effect".to_string(),
                        });
                    }

                    Ok(StateChange {
                        change_type: ChangeType::GlobalPropertyChanged,
                        entity_id: None,
                        property_name: Some(property_name.clone()),
                        old_value,
                        new_value: Some(new_value.clone()),
                        confidence: effect.probability,
                    })
                } else {
                    Err(anyhow::anyhow!("Invalid global property effect parameters"))
                }
            }
            _ => {
                // For other effect types, create a generic change
                Ok(StateChange {
                    change_type: ChangeType::GlobalPropertyChanged,
                    entity_id: effect.entity_id,
                    property_name: effect.property_name.clone(),
                    old_value: None,
                    new_value: effect.new_value.clone(),
                    confidence: effect.probability,
                })
            }
        }
    }

    /// Advance simulation by one step
    pub async fn step(&mut self) -> Result<Vec<ActionResult>> {
        let applicable_actions = self.find_applicable_actions()?;
        let mut results = Vec::new();

        if applicable_actions.is_empty() {
            return Ok(results);
        }

        // Detect conflicts
        let conflicts = self.detect_action_conflicts(&applicable_actions)?;
        
        // Resolve conflicts and select actions to apply
        let selected_actions = self.resolve_conflicts(&applicable_actions, &conflicts)?;

        // Apply selected actions
        for action_id in selected_actions.iter().take(self.action_config.max_concurrent_actions) {
            match self.apply_action(*action_id).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    results.push(ActionResult {
                        action_id: *action_id,
                        success: false,
                        changes: Vec::new(),
                        confidence: 0.0,
                        execution_time_ms: 0,
                        errors: vec![e.to_string()],
                        side_effects: Vec::new(),
                    });
                }
            }
        }

        Ok(results)
    }

    /// Detect conflicts between actions
    fn detect_action_conflicts(&self, action_ids: &[Uuid]) -> Result<Vec<ActionConflict>> {
        let mut conflicts = Vec::new();

        for i in 0..action_ids.len() {
            for j in (i + 1)..action_ids.len() {
                let action1 = self.available_actions.iter().find(|a| a.id == action_ids[i]);
                let action2 = self.available_actions.iter().find(|a| a.id == action_ids[j]);

                if let (Some(a1), Some(a2)) = (action1, action2) {
                    if let Some(conflict) = self.check_action_conflict(a1, a2)? {
                        conflicts.push(conflict);
                    }
                }
            }
        }

        Ok(conflicts)
    }

    /// Check if two actions conflict
    fn check_action_conflict(&self, action1: &Action, action2: &Action) -> Result<Option<ActionConflict>> {
        // Check for property conflicts
        for effect1 in &action1.effects {
            for effect2 in &action2.effects {
                if effect1.entity_id == effect2.entity_id && 
                   effect1.property_name == effect2.property_name &&
                   effect1.new_value != effect2.new_value {
                    return Ok(Some(ActionConflict {
                        conflicting_actions: vec![action1.id, action2.id],
                        conflict_type: ConflictType::PropertyConflict,
                        severity: 0.8,
                        resolution: ConflictResolution::HigherConfidence,
                    }));
                }
            }
        }

        // Check for precondition conflicts
        for precond1 in &action1.preconditions {
            for precond2 in &action2.preconditions {
                if precond1.entity_id == precond2.entity_id &&
                   precond1.property_name == precond2.property_name &&
                   precond1.expected_value != precond2.expected_value {
                    return Ok(Some(ActionConflict {
                        conflicting_actions: vec![action1.id, action2.id],
                        conflict_type: ConflictType::PreconditionConflict,
                        severity: 0.6,
                        resolution: ConflictResolution::Sequential,
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Resolve conflicts and select actions to apply
    fn resolve_conflicts(&self, action_ids: &[Uuid], conflicts: &[ActionConflict]) -> Result<Vec<Uuid>> {
        if conflicts.is_empty() {
            return Ok(action_ids.to_vec());
        }

        let mut selected_actions = Vec::new();
        let mut conflicted_actions = std::collections::HashSet::new();

        // Collect all conflicted action IDs
        for conflict in conflicts {
            for action_id in &conflict.conflicting_actions {
                conflicted_actions.insert(*action_id);
            }
        }

        // Add non-conflicted actions
        for action_id in action_ids {
            if !conflicted_actions.contains(action_id) {
                selected_actions.push(*action_id);
            }
        }

        // Resolve conflicts based on resolution strategy
        for conflict in conflicts {
            match conflict.resolution {
                ConflictResolution::HigherConfidence => {
                    if let Some(best_action) = self.select_highest_confidence_action(&conflict.conflicting_actions)? {
                        if !selected_actions.contains(&best_action) {
                            selected_actions.push(best_action);
                        }
                    }
                }
                ConflictResolution::HigherPriority => {
                    if let Some(best_action) = self.select_highest_priority_action(&conflict.conflicting_actions)? {
                        if !selected_actions.contains(&best_action) {
                            selected_actions.push(best_action);
                        }
                    }
                }
                ConflictResolution::Sequential => {
                    // For sequential resolution, just pick the first one for now
                    if let Some(first_action) = conflict.conflicting_actions.first() {
                        if !selected_actions.contains(first_action) {
                            selected_actions.push(*first_action);
                        }
                    }
                }
                _ => {
                    // For other resolution types, skip the conflicted actions
                    continue;
                }
            }
        }

        Ok(selected_actions)
    }

    /// Select action with highest confidence from a list
    fn select_highest_confidence_action(&self, action_ids: &[Uuid]) -> Result<Option<Uuid>> {
        let mut best_action = None;
        let mut best_confidence = 0.0;

        for action_id in action_ids {
            if let Some(action) = self.available_actions.iter().find(|a| a.id == *action_id) {
                if action.confidence > best_confidence {
                    best_confidence = action.confidence;
                    best_action = Some(*action_id);
                }
            }
        }

        Ok(best_action)
    }

    /// Select action with highest priority from a list
    fn select_highest_priority_action(&self, action_ids: &[Uuid]) -> Result<Option<Uuid>> {
        let mut best_action = None;
        let mut best_priority = ActionPriority::Low;

        for action_id in action_ids {
            if let Some(action) = self.available_actions.iter().find(|a| a.id == *action_id) {
                if action.priority > best_priority {
                    best_priority = action.priority.clone();
                    best_action = Some(*action_id);
                }
            }
        }

        Ok(best_action)
    }

    /// Get rule database
    pub fn get_rule_database(&self) -> &RuleDatabase {
        &self.rule_database
    }

    /// Get mutable rule database
    pub fn get_rule_database_mut(&mut self) -> &mut RuleDatabase {
        &mut self.rule_database
    }

    /// Get action configuration
    pub fn get_action_config(&self) -> &ActionConfig {
        &self.action_config
    }

    /// Set action configuration
    pub fn set_action_config(&mut self, config: ActionConfig) {
        self.action_config = config;
    }

    /// Get available actions
    pub fn get_available_actions(&self) -> &[Action] {
        &self.available_actions
    }

    // === Task 6.3: Branching Simulations and Confidence Scoring ===

    /// Get branching configuration
    pub fn get_branching_config(&self) -> &BranchingConfig {
        &self.branching_config
    }

    /// Set branching configuration
    pub fn set_branching_config(&mut self, config: BranchingConfig) {
        self.branching_config = config;
    }

    /// Get confidence configuration
    pub fn get_confidence_config(&self) -> &ConfidenceConfig {
        &self.confidence_config
    }

    /// Set confidence configuration
    pub fn set_confidence_config(&mut self, config: ConfidenceConfig) {
        self.confidence_config = config;
    }

    /// Add a simulation constraint
    pub fn add_constraint(&mut self, constraint: SimulationConstraint) {
        self.constraints.push(constraint);
    }

    /// Remove a constraint by ID
    pub fn remove_constraint(&mut self, constraint_id: Uuid) -> bool {
        let initial_len = self.constraints.len();
        self.constraints.retain(|c| c.id != constraint_id);
        self.constraints.len() != initial_len
    }

    /// Get all constraints
    pub fn get_constraints(&self) -> &[SimulationConstraint] {
        &self.constraints
    }

    /// Clear all constraints
    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }

    /// Run a branching simulation from the current state
    pub async fn run_branching_simulation(&mut self, max_steps: usize) -> Result<BranchingResult> {
        let start_time = std::time::Instant::now();
        
        let current_state = self.current_state.clone()
            .ok_or_else(|| anyhow::anyhow!("No current state available for branching simulation"))?;

        // Create root branch
        let root_branch = self.create_root_branch(current_state)?;
        let root_id = root_branch.id;
        
        let mut branches = HashMap::new();
        branches.insert(root_id, root_branch);
        
        let mut active_branches = vec![root_id];
        let mut total_branches_explored = 1;
        let mut total_branches_pruned = 0;

        // Run simulation steps
        for step in 0..max_steps {
            if active_branches.is_empty() {
                break;
            }

            // Process each active branch
            let mut new_branches = Vec::new();
            let mut branches_to_deactivate = Vec::new();

            for &branch_id in &active_branches {
                let branch = branches.get_mut(&branch_id).unwrap();
                
                // Check if branch should continue
                if !self.should_continue_branch(branch, step)? {
                    branches_to_deactivate.push(branch_id);
                    continue;
                }

                // Find applicable actions for this branch
                let applicable_actions = self.find_applicable_actions_for_state(&branch.current_state)?;
                
                if applicable_actions.is_empty() {
                    // Terminal branch - no more actions possible
                    branch.is_active = false;
                    branch.pruning_reason = Some("No applicable actions".to_string());
                    continue;
                }

                // Create new branches for different actions (up to max_branches_per_step)
                let max_new_branches = self.branching_config.max_branches_per_step.min(applicable_actions.len());
                let selected_actions = &applicable_actions[..max_new_branches];
                
                for &action_id in selected_actions {
                    if let Some(action) = self.available_actions.iter().find(|a| a.id == action_id) {
                        if action.confidence >= self.branching_config.min_branch_confidence {
                            let new_branch = self.create_child_branch(branch, action.clone()).await?;
                            new_branches.push(new_branch);
                            total_branches_explored += 1;
                        }
                    }
                }

                // Deactivate parent branch if it has children
                if !new_branches.is_empty() {
                    branches_to_deactivate.push(branch_id);
                }
            }

            // Add new branches
            for new_branch in new_branches {
                let new_id = new_branch.id;
                branches.insert(new_id, new_branch);
                active_branches.push(new_id);
            }

            // Deactivate processed branches
            for branch_id in branches_to_deactivate {
                if let Some(branch) = branches.get_mut(&branch_id) {
                    branch.is_active = false;
                    branch.pruning_reason = Some("Expanded into child branches".to_string());
                }
                active_branches.retain(|&id| id != branch_id);
            }

            // Prune branches if needed
            if active_branches.len() > self.branching_config.max_active_branches {
                let pruned_count = self.prune_branches(&mut branches, &mut active_branches)?;
                total_branches_pruned += pruned_count;
            }
        }

        // Calculate final results
        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        let most_likely_outcomes = self.identify_most_likely_outcomes(&branches)?;
        let overall_confidence = self.calculate_overall_confidence(&branches, &most_likely_outcomes)?;
        let branching_stats = self.calculate_branching_stats(&branches)?;

        Ok(BranchingResult {
            branches,
            root_branch_id: root_id,
            most_likely_outcomes,
            total_branches_explored,
            total_branches_pruned,
            overall_confidence,
            execution_time_ms,
            branching_stats,
        })
    }

    /// Create root branch from current state
    fn create_root_branch(&self, state: SimulationState) -> Result<SimulationBranch> {
        let now = Utc::now();
        Ok(SimulationBranch {
            id: Uuid::new_v4(),
            parent_id: None,
            child_ids: Vec::new(),
            current_state: state,
            transition_history: Vec::new(),
            accumulated_confidence: 1.0, // Root starts with full confidence
            depth: 0,
            is_active: true,
            created_at: now,
            last_updated: now,
            pruning_reason: None,
            metadata: HashMap::new(),
        })
    }

    /// Create a child branch by applying an action to the parent branch
    async fn create_child_branch(&mut self, parent: &SimulationBranch, action: Action) -> Result<SimulationBranch> {
        let mut new_state = parent.current_state.clone();
        new_state.id = Uuid::new_v4(); // New state ID
        
        // Apply action to create new state
        let action_result = self.simulate_action_on_state(&action, &mut new_state).await?;
        
        // Create transition record
        let transition = StateTransition {
            id: Uuid::new_v4(),
            from_state_id: parent.current_state.id,
            to_state_id: new_state.id,
            applied_rules: Vec::new(), // Could be populated from action context
            changes: action_result.changes,
            confidence: action_result.confidence,
            timestamp: Utc::now(),
            duration_ms: action_result.execution_time_ms,
        };

        // Calculate accumulated confidence with decay
        let accumulated_confidence = parent.accumulated_confidence * 
            self.confidence_config.confidence_decay_factor * 
            action_result.confidence;

        // Add constraint satisfaction bonus
        let constraint_bonus = self.calculate_constraint_satisfaction_bonus(&new_state)?;
        let final_confidence = (accumulated_confidence + constraint_bonus).min(1.0);

        let now = Utc::now();
        let mut transition_history = parent.transition_history.clone();
        transition_history.push(transition);

        Ok(SimulationBranch {
            id: Uuid::new_v4(),
            parent_id: Some(parent.id),
            child_ids: Vec::new(),
            current_state: new_state,
            transition_history,
            accumulated_confidence: final_confidence,
            depth: parent.depth + 1,
            is_active: true,
            created_at: now,
            last_updated: now,
            pruning_reason: None,
            metadata: HashMap::new(),
        })
    }

    /// Check if a branch should continue simulation
    fn should_continue_branch(&self, branch: &SimulationBranch, _step: usize) -> Result<bool> {
        // Check depth limit
        if branch.depth >= self.branching_config.max_branching_depth {
            return Ok(false);
        }

        // Check confidence threshold
        if branch.accumulated_confidence < self.branching_config.pruning_threshold {
            return Ok(false);
        }

        // Check if branch violates mandatory constraints
        for constraint in &self.constraints {
            if constraint.is_mandatory && !self.evaluate_constraint(constraint, &branch.current_state)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Find applicable actions for a specific state
    fn find_applicable_actions_for_state(&self, state: &SimulationState) -> Result<Vec<Uuid>> {
        let mut applicable_actions = Vec::new();

        for action in &self.available_actions {
            if self.is_action_applicable(action, state)? {
                applicable_actions.push(action.id);
            }
        }

        // Sort by confidence (descending)
        applicable_actions.sort_by(|a, b| {
            let conf_a = self.available_actions.iter().find(|act| act.id == *a).map(|act| act.confidence).unwrap_or(0.0);
            let conf_b = self.available_actions.iter().find(|act| act.id == *b).map(|act| act.confidence).unwrap_or(0.0);
            conf_b.partial_cmp(&conf_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(applicable_actions)
    }

    /// Simulate applying an action to a state without modifying the engine state
    async fn simulate_action_on_state(&self, action: &Action, state: &mut SimulationState) -> Result<ActionResult> {
        let start_time = std::time::Instant::now();
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let mut success = true;

        // Apply each effect
        for effect in &action.effects {
            if let Ok(change) = self.apply_effect_to_state(effect, state) {
                changes.push(change);
            } else {
                success = false;
                errors.push(format!("Failed to apply effect: {:?}", effect.effect_type));
            }
        }

        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        let confidence = if success { action.confidence } else { action.confidence * 0.5 };

        Ok(ActionResult {
            action_id: action.id,
            success,
            changes,
            confidence,
            execution_time_ms,
            errors,
            side_effects: Vec::new(),
        })
    }

    /// Apply an effect to a specific state
    fn apply_effect_to_state(&self, effect: &Effect, state: &mut SimulationState) -> Result<StateChange> {
        match effect.effect_type {
            EffectType::SetGlobalProperty => {
                if let (Some(prop_name), Some(new_value)) = (&effect.property_name, &effect.new_value) {
                    // Find existing property or create new one
                    for prop in &mut state.global_properties {
                        if prop.name == *prop_name {
                            let old_value = prop.value.clone();
                            prop.value = new_value.clone();
                            return Ok(StateChange {
                                change_type: ChangeType::GlobalPropertyChanged,
                                entity_id: None,
                                property_name: Some(prop_name.clone()),
                                old_value: Some(old_value),
                                new_value: Some(new_value.clone()),
                                confidence: effect.probability,
                            });
                        }
                    }
                    
                    // Property not found, create new one
                    state.global_properties.push(StateProperty {
                        name: prop_name.clone(),
                        value: new_value.clone(),
                        property_type: PropertyType::Abstract,
                        confidence: effect.probability,
                        source: "effect".to_string(),
                    });
                    
                    return Ok(StateChange {
                        change_type: ChangeType::GlobalPropertyChanged,
                        entity_id: None,
                        property_name: Some(prop_name.clone()),
                        old_value: None,
                        new_value: Some(new_value.clone()),
                        confidence: effect.probability,
                    });
                }
            }
            _ => {
                // Other effect types can be implemented as needed
                return Ok(StateChange {
                    change_type: ChangeType::GlobalPropertyChanged,
                    entity_id: effect.entity_id,
                    property_name: effect.property_name.clone(),
                    old_value: None,
                    new_value: effect.new_value.clone(),
                    confidence: effect.probability,
                });
            }
        }

        Err(anyhow::anyhow!("Failed to apply effect"))
    }

    /// Prune low-confidence branches
    fn prune_branches(&self, branches: &mut HashMap<Uuid, SimulationBranch>, active_branches: &mut Vec<Uuid>) -> Result<usize> {
        let mut branches_to_prune = Vec::new();
        
        // Sort active branches by confidence (ascending, so we prune the lowest first)
        active_branches.sort_by(|a, b| {
            let conf_a = branches.get(a).map(|b| b.accumulated_confidence).unwrap_or(0.0);
            let conf_b = branches.get(b).map(|b| b.accumulated_confidence).unwrap_or(0.0);
            conf_a.partial_cmp(&conf_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Calculate how many branches to prune
        let target_active = if self.branching_config.enable_aggressive_pruning {
            self.branching_config.max_active_branches / 2
        } else {
            self.branching_config.max_active_branches
        };

        let prune_count = active_branches.len().saturating_sub(target_active);
        
        for i in 0..prune_count {
            if let Some(&branch_id) = active_branches.get(i) {
                if let Some(branch) = branches.get_mut(&branch_id) {
                    branch.is_active = false;
                    branch.pruning_reason = Some("Low confidence pruning".to_string());
                    branches_to_prune.push(branch_id);
                }
            }
        }

        // Remove pruned branches from active list
        for branch_id in &branches_to_prune {
            active_branches.retain(|&id| id != *branch_id);
        }

        Ok(branches_to_prune.len())
    }

    /// Identify the most likely outcomes from all branches
    fn identify_most_likely_outcomes(&self, branches: &HashMap<Uuid, SimulationBranch>) -> Result<Vec<Uuid>> {
        let mut branch_scores: Vec<(Uuid, f64)> = branches.iter()
            .map(|(id, branch)| (*id, branch.accumulated_confidence))
            .collect();

        // Sort by confidence (descending)
        branch_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return top 5 most likely outcomes
        Ok(branch_scores.into_iter().take(5).map(|(id, _)| id).collect())
    }

    /// Calculate overall confidence in simulation results
    fn calculate_overall_confidence(&self, branches: &HashMap<Uuid, SimulationBranch>, most_likely: &[Uuid]) -> Result<f64> {
        if most_likely.is_empty() {
            return Ok(0.0);
        }

        let total_confidence: f64 = most_likely.iter()
            .filter_map(|id| branches.get(id))
            .map(|branch| branch.accumulated_confidence)
            .sum();

        Ok(total_confidence / most_likely.len() as f64)
    }

    /// Calculate comprehensive branching statistics
    fn calculate_branching_stats(&self, branches: &HashMap<Uuid, SimulationBranch>) -> Result<BranchingStats> {
        let active_branches: Vec<&SimulationBranch> = branches.values().filter(|b| b.is_active).collect();
        let all_branches: Vec<&SimulationBranch> = branches.values().collect();

        let average_confidence = if all_branches.is_empty() {
            0.0
        } else {
            all_branches.iter().map(|b| b.accumulated_confidence).sum::<f64>() / all_branches.len() as f64
        };

        let max_depth_reached = all_branches.iter().map(|b| b.depth).max().unwrap_or(0);
        
        let average_depth = if active_branches.is_empty() {
            0.0
        } else {
            active_branches.iter().map(|b| b.depth as f64).sum::<f64>() / active_branches.len() as f64
        };

        let terminal_branches = all_branches.iter().filter(|b| !b.is_active).count();

        // Calculate diversity score based on variation in confidence scores
        let confidence_variance = if all_branches.len() > 1 {
            let mean = average_confidence;
            let variance = all_branches.iter()
                .map(|b| (b.accumulated_confidence - mean).powi(2))
                .sum::<f64>() / all_branches.len() as f64;
            variance.sqrt() // Standard deviation as diversity measure
        } else {
            0.0
        };

        let diversity_score = confidence_variance.min(1.0);

        // Calculate computational complexity score
        let complexity_score = (all_branches.len() as f64 / 100.0).min(1.0); // Normalize to 0-1

        Ok(BranchingStats {
            average_confidence,
            max_depth_reached,
            average_depth,
            terminal_branches,
            diversity_score,
            complexity_score,
        })
    }

    /// Evaluate if a constraint is satisfied by a state
    fn evaluate_constraint(&self, constraint: &SimulationConstraint, state: &SimulationState) -> Result<bool> {
        self.evaluate_condition(&constraint.condition, state)
    }

    /// Calculate bonus confidence for satisfying constraints
    fn calculate_constraint_satisfaction_bonus(&self, state: &SimulationState) -> Result<f64> {
        let mut bonus = 0.0;
        let mut total_weight = 0.0;

        for constraint in &self.constraints {
            total_weight += constraint.weight;
            if self.evaluate_constraint(constraint, state)? {
                bonus += constraint.weight * self.confidence_config.constraint_satisfaction_bonus;
            }
        }

        // Normalize bonus by total constraint weight
        Ok(if total_weight > 0.0 { bonus / total_weight } else { 0.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concept_graph::{ConceptGraphConfig, ConceptNode, ConceptType};

    #[tokio::test]
    async fn test_simulation_config_creation() {
        let config = SimulationConfig::default();
        assert_eq!(config.max_entities_per_state, 50);
        assert_eq!(config.max_relationship_depth, 3);
        assert!(config.min_concept_confidence > 0.0);
    }

    #[tokio::test]
    async fn test_simulation_state_creation() {
        let mut state = SimulationState::new();
        assert!(state.entities.is_empty());
        assert!(state.relationships.is_empty());
        assert!(state.is_valid);
        assert_eq!(state.complexity(), 0);

        // Add an entity
        let concept = ConceptNode::new(
            ConceptType::Entity,
            "test_entity".to_string(),
            0.8,
            Some("test".to_string())
        );
        let properties = vec![StateProperty {
            name: "color".to_string(),
            value: "red".to_string(),
            property_type: PropertyType::Physical,
            confidence: 0.9,
            source: "test".to_string(),
        }];

        let entity_id = state.add_entity(concept, properties);
        assert_eq!(state.entities.len(), 1);
        assert_eq!(state.complexity(), 1);
        assert!(state.get_entity(entity_id).is_some());
    }

    #[tokio::test]
    async fn test_text_to_state_parser() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let mut parser = TextToStateParser::new(concept_graph);

        let text = "The red car is in the garage";
        let result = parser.parse_text_to_state(text).await;
        
        // Should not fail even with empty concept graph
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.source_text, Some(text.to_string()));
    }

    #[tokio::test]
    async fn test_state_validator() {
        let validator = StateValidator::new();
        let mut state = SimulationState::new();

        // Valid empty state
        let result = validator.validate_state(&mut state);
        assert!(result.is_ok());
        assert!(state.is_valid);

        // Add entity with low confidence
        let low_confidence_concept = ConceptNode::new(
            ConceptType::Entity,
            "test".to_string(),
            0.1,  // Below default threshold
            None
        );
        state.add_entity(low_confidence_concept, Vec::new());

        let result = validator.validate_state(&mut state);
        assert!(result.is_ok());
        assert!(!state.is_valid);  // Should be invalid due to low confidence
        assert!(!state.validation_errors.is_empty());
    }

    #[tokio::test]
    async fn test_simulation_engine_initialization() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let mut engine = SimulationEngine::new(concept_graph);

        assert!(engine.get_current_state().is_none());
        assert!(engine.get_state_history().is_empty());

        let text = "A simple test scenario";
        let result = engine.initialize_from_text(text).await;
        
        assert!(result.is_ok());
        assert!(engine.get_current_state().is_some());
        assert_eq!(engine.get_state_history().len(), 1);
    }

    #[tokio::test]
    async fn test_property_types() {
        let property = StateProperty {
            name: "test".to_string(),
            value: "value".to_string(),
            property_type: PropertyType::Physical,
            confidence: 0.8,
            source: "test".to_string(),
        };

        assert_eq!(property.property_type, PropertyType::Physical);
        assert_eq!(property.confidence, 0.8);
    }

    #[tokio::test]
    async fn test_relationship_info() {
        let relationship = RelationshipInfo {
            relationship_type: RelationshipType::IsA,
            strength: 0.9,
            properties: Vec::new(),
            confidence: 0.8,
        };

        assert_eq!(relationship.relationship_type, RelationshipType::IsA);
        assert_eq!(relationship.strength, 0.9);
    }

    #[tokio::test]
    async fn test_state_transition() {
        let transition = StateTransition {
            id: Uuid::new_v4(),
            from_state_id: Uuid::new_v4(),
            to_state_id: Uuid::new_v4(),
            applied_rules: Vec::new(),
            changes: Vec::new(),
            confidence: 0.7,
            timestamp: Utc::now(),
            duration_ms: 1000,
        };

        assert_eq!(transition.confidence, 0.7);
        assert_eq!(transition.duration_ms, 1000);
    }

    #[tokio::test]
    async fn test_action_creation() {
        let action = Action {
            id: Uuid::new_v4(),
            name: "test_action".to_string(),
            description: "A test action".to_string(),
            preconditions: vec![Condition {
                condition_type: ConditionType::EntityExists,
                entity_id: Some(Uuid::new_v4()),
                property_name: None,
                expected_value: "test".to_string(),
                operator: ComparisonOperator::Equals,
                required_confidence: 0.8,
            }],
            effects: vec![Effect {
                effect_type: EffectType::SetProperty,
                entity_id: Some(Uuid::new_v4()),
                property_name: Some("status".to_string()),
                new_value: Some("active".to_string()),
                probability: 0.9,
                delay_ms: 0,
            }],
            confidence: 0.8,
            duration_ms: 1000,
            priority: ActionPriority::Medium,
            context: HashMap::new(),
        };

        assert_eq!(action.name, "test_action");
        assert_eq!(action.priority, ActionPriority::Medium);
        assert_eq!(action.preconditions.len(), 1);
        assert_eq!(action.effects.len(), 1);
    }

    #[tokio::test]
    async fn test_action_config() {
        let config = ActionConfig::default();
        assert_eq!(config.max_actions_per_step, 10);
        assert_eq!(config.min_action_confidence, 0.5);
        assert!(config.enable_conflict_resolution);
    }

    #[tokio::test]
    async fn test_condition_evaluation() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let engine = SimulationEngine::new(concept_graph);

        let mut state = SimulationState::new();
        
        // Add entity to state
        let concept = ConceptNode::new(
            ConceptType::Entity,
            "test_entity".to_string(),
            0.8,
            None
        );
        let entity_id = state.add_entity(concept, vec![StateProperty {
            name: "color".to_string(),
            value: "red".to_string(),
            property_type: PropertyType::Physical,
            confidence: 0.9,
            source: "test".to_string(),
        }]);

        // Test EntityExists condition
        let condition = Condition {
            condition_type: ConditionType::EntityExists,
            entity_id: Some(entity_id),
            property_name: None,
            expected_value: "".to_string(),
            operator: ComparisonOperator::Equals,
            required_confidence: 0.5,
        };

        let result = engine.evaluate_condition(&condition, &state);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_effect_application() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let engine = SimulationEngine::new(concept_graph);

        let mut state = SimulationState::new();
        
        let effect = Effect {
            effect_type: EffectType::SetGlobalProperty,
            entity_id: None,
            property_name: Some("weather".to_string()),
            new_value: Some("sunny".to_string()),
            probability: 1.0,
            delay_ms: 0,
        };

        let result = engine.apply_effect(&effect, &mut state);
        assert!(result.is_ok());

        let change = result.unwrap();
        assert_eq!(change.change_type, ChangeType::GlobalPropertyChanged);
        assert_eq!(change.new_value, Some("sunny".to_string()));
        
        // Check that global property was added
        assert_eq!(state.global_properties.len(), 1);
        assert_eq!(state.global_properties[0].name, "weather");
        assert_eq!(state.global_properties[0].value, "sunny");
    }

    #[tokio::test]
    async fn test_action_conflict_detection() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let engine = SimulationEngine::new(concept_graph);

        let entity_id = Uuid::new_v4();

        let action1 = Action {
            id: Uuid::new_v4(),
            name: "action1".to_string(),
            description: "First action".to_string(),
            preconditions: Vec::new(),
            effects: vec![Effect {
                effect_type: EffectType::SetProperty,
                entity_id: Some(entity_id),
                property_name: Some("status".to_string()),
                new_value: Some("active".to_string()),
                probability: 1.0,
                delay_ms: 0,
            }],
            confidence: 0.8,
            duration_ms: 1000,
            priority: ActionPriority::Medium,
            context: HashMap::new(),
        };

        let action2 = Action {
            id: Uuid::new_v4(),
            name: "action2".to_string(),
            description: "Second action".to_string(),
            preconditions: Vec::new(),
            effects: vec![Effect {
                effect_type: EffectType::SetProperty,
                entity_id: Some(entity_id),
                property_name: Some("status".to_string()),
                new_value: Some("inactive".to_string()),
                probability: 1.0,
                delay_ms: 0,
            }],
            confidence: 0.7,
            duration_ms: 1000,
            priority: ActionPriority::Low,
            context: HashMap::new(),
        };

        let conflict = engine.check_action_conflict(&action1, &action2);
        assert!(conflict.is_ok());
        
        let conflict_result = conflict.unwrap();
        assert!(conflict_result.is_some());
        
        let conflict_info = conflict_result.unwrap();
        assert_eq!(conflict_info.conflict_type, ConflictType::PropertyConflict);
        assert_eq!(conflict_info.conflicting_actions.len(), 2);
    }

    #[tokio::test]
    async fn test_action_priority_comparison() {
        assert!(ActionPriority::High > ActionPriority::Medium);
        assert!(ActionPriority::Medium > ActionPriority::Low);
        assert!(ActionPriority::Critical > ActionPriority::High);
    }

    #[tokio::test]
    async fn test_comparison_operators() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let engine = SimulationEngine::new(concept_graph);

        assert!(engine.compare_values("hello", "hello", &ComparisonOperator::Equals));
        assert!(!engine.compare_values("hello", "world", &ComparisonOperator::Equals));
        assert!(engine.compare_values("hello world", "world", &ComparisonOperator::Contains));
        assert!(!engine.compare_values("hello", "world", &ComparisonOperator::Contains));
        assert!(engine.compare_values("10", "5", &ComparisonOperator::GreaterThan));
        assert!(engine.compare_values("5", "10", &ComparisonOperator::LessThan));
    }

    #[tokio::test]
    async fn test_state_element_detection() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let engine = SimulationEngine::new(concept_graph);

        let mut state = SimulationState::new();
        state.set_description("A red car in the garage".to_string());

        let result = engine.state_contains_element(&state, "car");
        assert!(result.is_ok());
        assert!(result.unwrap());

        let result = engine.state_contains_element(&state, "bicycle");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    // === Task 6.3: Branching Simulations Tests ===

    #[tokio::test]
    async fn test_branching_config_creation() {
        let config = BranchingConfig::default();
        assert_eq!(config.max_branches_per_step, 5);
        assert_eq!(config.max_branching_depth, 10);
        assert_eq!(config.min_branch_confidence, 0.4);
        assert_eq!(config.max_active_branches, 50);
        assert_eq!(config.pruning_threshold, 0.3);
        assert!(config.enable_aggressive_pruning);
        assert_eq!(config.max_simulation_time_seconds, 300);
    }

    #[tokio::test]
    async fn test_confidence_config_creation() {
        let config = ConfidenceConfig::default();
        assert_eq!(config.rule_confidence_weight, 0.4);
        assert_eq!(config.path_likelihood_weight, 0.3);
        assert_eq!(config.state_consistency_weight, 0.2);
        assert_eq!(config.historical_accuracy_weight, 0.1);
        assert_eq!(config.confidence_decay_factor, 0.95);
        assert_eq!(config.constraint_satisfaction_bonus, 0.1);
    }

    #[tokio::test]
    async fn test_simulation_branch_creation() {
        let state = SimulationState::new();
        let branch = SimulationBranch {
            id: Uuid::new_v4(),
            parent_id: None,
            child_ids: Vec::new(),
            current_state: state,
            transition_history: Vec::new(),
            accumulated_confidence: 1.0,
            depth: 0,
            is_active: true,
            created_at: Utc::now(),
            last_updated: Utc::now(),
            pruning_reason: None,
            metadata: HashMap::new(),
        };
        
        assert!(branch.is_active);
        assert_eq!(branch.depth, 0);
        assert_eq!(branch.accumulated_confidence, 1.0);
        assert!(branch.parent_id.is_none());
    }

    #[tokio::test]
    async fn test_constraint_creation() {
        let constraint = SimulationConstraint {
            id: Uuid::new_v4(),
            constraint_type: ConstraintType::Achievement,
            description: "Test constraint".to_string(),
            condition: Condition {
                condition_type: ConditionType::PropertyEquals,
                entity_id: None,
                property_name: Some("test".to_string()),
                expected_value: "value".to_string(),
                operator: ComparisonOperator::Equals,
                required_confidence: 0.5,
            },
            weight: 0.8,
            is_mandatory: true,
        };

        assert_eq!(constraint.constraint_type, ConstraintType::Achievement);
        assert!(constraint.is_mandatory);
        assert_eq!(constraint.weight, 0.8);
    }

    #[tokio::test]
    async fn test_branching_engine_configuration() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let mut engine = SimulationEngine::new(concept_graph);
        
        let branching_config = BranchingConfig {
            max_branches_per_step: 3,
            max_branching_depth: 5,
            min_branch_confidence: 0.6,
            max_active_branches: 20,
            pruning_threshold: 0.4,
            enable_aggressive_pruning: false,
            max_simulation_time_seconds: 120,
        };

        engine.set_branching_config(branching_config.clone());
        assert_eq!(engine.get_branching_config().max_branches_per_step, 3);
        assert_eq!(engine.get_branching_config().max_branching_depth, 5);
        assert_eq!(engine.get_branching_config().min_branch_confidence, 0.6);
    }

    #[tokio::test]
    async fn test_confidence_scoring_configuration() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let mut engine = SimulationEngine::new(concept_graph);
        
        let confidence_config = ConfidenceConfig {
            rule_confidence_weight: 0.5,
            path_likelihood_weight: 0.25,
            state_consistency_weight: 0.15,
            historical_accuracy_weight: 0.1,
            confidence_decay_factor: 0.9,
            constraint_satisfaction_bonus: 0.15,
        };

        engine.set_confidence_config(confidence_config.clone());
        assert_eq!(engine.get_confidence_config().rule_confidence_weight, 0.5);
        assert_eq!(engine.get_confidence_config().confidence_decay_factor, 0.9);
    }

    #[tokio::test]
    async fn test_constraint_management() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let mut engine = SimulationEngine::new(concept_graph);
        
        let constraint_id = Uuid::new_v4();
        let constraint = SimulationConstraint {
            id: constraint_id,
            constraint_type: ConstraintType::Avoidance,
            description: "Test avoidance constraint".to_string(),
            condition: Condition {
                condition_type: ConditionType::GlobalProperty,
                entity_id: None,
                property_name: Some("danger".to_string()),
                expected_value: "high".to_string(),
                operator: ComparisonOperator::Equals,
                required_confidence: 0.7,
            },
            weight: 0.9,
            is_mandatory: true,
        };

        // Test adding constraint
        engine.add_constraint(constraint.clone());
        assert_eq!(engine.get_constraints().len(), 1);

        // Test removing constraint
        assert!(engine.remove_constraint(constraint_id));
        assert_eq!(engine.get_constraints().len(), 0);

        // Test removing non-existent constraint
        assert!(!engine.remove_constraint(Uuid::new_v4()));
    }

    #[tokio::test]
    async fn test_root_branch_creation() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let engine = SimulationEngine::new(concept_graph);
        
        let state = SimulationState::new();
        let root_branch = engine.create_root_branch(state.clone()).unwrap();
        
        assert!(root_branch.parent_id.is_none());
        assert_eq!(root_branch.depth, 0);
        assert_eq!(root_branch.accumulated_confidence, 1.0);
        assert!(root_branch.is_active);
        assert!(root_branch.transition_history.is_empty());
        assert_eq!(root_branch.current_state.id, state.id);
    }

    #[tokio::test]
    async fn test_branching_stats_calculation() {
        let concept_graph_config = ConceptGraphConfig::default();
        let concept_graph = ConceptGraphManager::new(concept_graph_config).await.unwrap();
        let engine = SimulationEngine::new(concept_graph);
        
        let mut branches = HashMap::new();
        
        // Create test branches with varying confidence
        for i in 0..5 {
            let branch = SimulationBranch {
                id: Uuid::new_v4(),
                parent_id: None,
                child_ids: Vec::new(),
                current_state: SimulationState::new(),
                transition_history: Vec::new(),
                accumulated_confidence: 0.5 + (i as f64 * 0.1),
                depth: i,
                is_active: i < 3, // First 3 are active
                created_at: Utc::now(),
                last_updated: Utc::now(),
                pruning_reason: if i >= 3 { Some("Test pruning".to_string()) } else { None },
                metadata: HashMap::new(),
            };
            branches.insert(branch.id, branch);
        }
        
        let stats = engine.calculate_branching_stats(&branches).unwrap();
        
        assert_eq!(stats.terminal_branches, 2); // 2 inactive branches
        assert_eq!(stats.max_depth_reached, 4);
        assert!(stats.average_confidence > 0.0);
        assert!(stats.complexity_score >= 0.0 && stats.complexity_score <= 1.0);
    }

    #[tokio::test]
    async fn test_constraint_types() {
        // Test all constraint types
        let constraint_types = vec![
            ConstraintType::Avoidance,
            ConstraintType::Achievement,
            ConstraintType::Maintenance,
            ConstraintType::Sequence,
            ConstraintType::Resource,
            ConstraintType::Probabilistic,
        ];

        for constraint_type in constraint_types {
            let constraint = SimulationConstraint {
                id: Uuid::new_v4(),
                constraint_type: constraint_type.clone(),
                description: format!("Test {:?} constraint", constraint_type),
                condition: Condition {
                    condition_type: ConditionType::PropertyEquals,
                    entity_id: None,
                    property_name: Some("test".to_string()),
                    expected_value: "value".to_string(),
                    operator: ComparisonOperator::Equals,
                    required_confidence: 0.5,
                },
                weight: 0.5,
                is_mandatory: false,
            };
            
            assert_eq!(constraint.constraint_type, constraint_type);
        }
    }
} 