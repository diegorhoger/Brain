//! Simulation Engine Domain Layer
//!
//! This module defines the core domain abstractions for the simulation engine that converts
//! text to state-action graphs and simulates temporal transitions using concept nodes.

use brain_types::*;
use crate::concepts::{ConceptNode, RelationshipType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

/// Types of conditions that can be checked
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Effect that an action will have on the simulation state
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

/// Types of effects that can be applied
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Result of applying an action to a simulation state
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

/// Simulation branch for branching simulations
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

/// Statistics for branching simulations
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

/// Constraint for simulation behavior
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

// Domain service traits

/// Service for parsing text into simulation states
#[async_trait::async_trait]
pub trait TextToStateParser: Send + Sync {
    /// Parse text into a simulation state
    async fn parse_text_to_state(&mut self, text: &str) -> Result<SimulationState>;
    
    /// Get current configuration
    fn config(&self) -> &SimulationConfig;
    
    /// Set new configuration
    fn set_config(&mut self, config: SimulationConfig);
}

/// Service for validating simulation states and transitions
pub trait StateValidator: Send + Sync {
    /// Validate a simulation state
    fn validate_state(&self, state: &mut SimulationState) -> Result<bool>;
    
    /// Validate a state transition
    fn validate_transition(&self, transition: &StateTransition) -> Result<Vec<String>>;
    
    /// Get current configuration
    fn config(&self) -> &SimulationConfig;
}

/// Service for managing simulation execution
#[async_trait::async_trait]
pub trait SimulationEngine: Send + Sync {
    /// Initialize simulation from text
    async fn initialize_from_text(&mut self, text: &str) -> Result<Uuid>;
    
    /// Get current simulation state
    fn get_current_state(&self) -> Option<&SimulationState>;
    
    /// Apply an action to the current state
    async fn apply_action(&mut self, action_id: Uuid) -> Result<ActionResult>;
    
    /// Execute one simulation step
    async fn step(&mut self) -> Result<Vec<ActionResult>>;
    
    /// Run a branching simulation
    async fn run_branching_simulation(&mut self, max_steps: usize) -> Result<BranchingResult>;
    
    /// Find applicable actions for the current state
    fn find_applicable_actions(&self) -> Result<Vec<Uuid>>;
    
    /// Add an action to the simulation
    fn add_action(&mut self, action: Action);
    
    /// Add a constraint to the simulation
    fn add_constraint(&mut self, constraint: SimulationConstraint);
    
    /// Reset the simulation
    fn reset(&mut self);
    
    /// Get simulation configuration
    fn config(&self) -> &SimulationConfig;
    
    /// Set simulation configuration
    fn set_config(&mut self, config: SimulationConfig);
}

/// Service for managing simulation branches
#[async_trait::async_trait]
pub trait BranchingSimulation: Send + Sync {
    /// Create a new branch from a parent state
    async fn create_branch(&mut self, parent_id: Option<Uuid>, state: SimulationState) -> Result<Uuid>;
    
    /// Prune branches based on confidence thresholds
    fn prune_branches(&mut self, threshold: f64) -> Result<usize>;
    
    /// Get most likely outcomes
    fn get_most_likely_outcomes(&self, limit: usize) -> Result<Vec<Uuid>>;
    
    /// Calculate branching statistics
    fn calculate_stats(&self) -> Result<BranchingStats>;
    
    /// Get branch by ID
    fn get_branch(&self, branch_id: Uuid) -> Option<&SimulationBranch>;
    
    /// Get all active branches
    fn get_active_branches(&self) -> Vec<Uuid>;
}

// Utility implementations for domain types

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
            confidence: 0.0,
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

    /// Add a relationship between entities
    pub fn add_relationship(
        &mut self,
        entity1_id: Uuid,
        entity2_id: Uuid,
        relationship_info: RelationshipInfo,
    ) -> Result<()> {
        if !self.entities.contains_key(&entity1_id) || !self.entities.contains_key(&entity2_id) {
            return Err(BrainError::InvalidInput(
                "Both entities must exist in the state to create a relationship".to_string(),
            ));
        }
        self.relationships.insert((entity1_id, entity2_id), relationship_info);
        Ok(())
    }

    /// Get entity by ID
    pub fn get_entity(&self, entity_id: Uuid) -> Option<&ConceptNode> {
        self.entities.get(&entity_id)
    }

    /// Get entity properties
    pub fn get_entity_properties(&self, entity_id: Uuid) -> Option<&Vec<StateProperty>> {
        self.entity_properties.get(&entity_id)
    }

    /// Get relationship between entities
    pub fn get_relationship(&self, entity1_id: Uuid, entity2_id: Uuid) -> Option<&RelationshipInfo> {
        self.relationships.get(&(entity1_id, entity2_id))
            .or_else(|| self.relationships.get(&(entity2_id, entity1_id)))
    }

    /// Calculate state complexity
    pub fn complexity(&self) -> usize {
        self.entities.len() + self.relationships.len() + self.global_properties.len()
    }

    /// Set state description
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

impl Default for SimulationState {
    fn default() -> Self {
        Self::new()
    }
}

impl Action {
    /// Create a new action
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            preconditions: Vec::new(),
            effects: Vec::new(),
            confidence: 1.0,
            duration_ms: 0,
            priority: ActionPriority::Medium,
            context: HashMap::new(),
        }
    }

    /// Add a precondition to the action
    pub fn add_precondition(&mut self, condition: Condition) {
        self.preconditions.push(condition);
    }

    /// Add an effect to the action
    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    /// Set action priority
    pub fn set_priority(&mut self, priority: ActionPriority) {
        self.priority = priority;
    }

    /// Set action confidence
    pub fn set_confidence(&mut self, confidence: f64) {
        self.confidence = confidence.clamp(0.0, 1.0);
    }
}

impl SimulationBranch {
    /// Create a new simulation branch
    pub fn new(state: SimulationState, parent_id: Option<Uuid>) -> Self {
        let depth = if parent_id.is_some() { 1 } else { 0 };
        Self {
            id: Uuid::new_v4(),
            parent_id,
            child_ids: Vec::new(),
            current_state: state,
            transition_history: Vec::new(),
            accumulated_confidence: 1.0,
            depth,
            is_active: true,
            created_at: Utc::now(),
            last_updated: Utc::now(),
            pruning_reason: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a child branch
    pub fn add_child(&mut self, child_id: Uuid) {
        self.child_ids.push(child_id);
    }

    /// Mark branch as pruned
    pub fn prune(&mut self, reason: String) {
        self.is_active = false;
        self.pruning_reason = Some(reason);
    }

    /// Update accumulated confidence
    pub fn update_confidence(&mut self, new_confidence: f64) {
        self.accumulated_confidence = new_confidence.clamp(0.0, 1.0);
        self.last_updated = Utc::now();
    }
} 