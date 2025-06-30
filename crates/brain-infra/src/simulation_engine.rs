//! Simulation Engine with Branching Logic
//!
//! This module provides advanced simulation capabilities including:
//! - Complex state management with entities and properties
//! - Action-based state transitions with preconditions and effects
//! - Tree-based branching exploration with confidence scoring
//! - Intelligent pruning mechanisms for optimization
//! - Constraint-guided simulation for targeted exploration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::Result;
use uuid::Uuid;
use brain_types::BrainError;
use brain_core::{ConceptNode, ConceptType};
use tokio::time::Instant;

/// Property types for simulation state entities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    Location,
    State,
    Attribute,
    Relationship,
    Numeric,
    Boolean,
    Temporal,
    Resource,
}

/// Individual property of a simulation entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateProperty {
    pub name: String,
    pub value: String,
    pub property_type: PropertyType,
    pub confidence: f64,
    pub source: String,
}

/// Priority levels for actions
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ActionPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of effects actions can have
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    SetProperty,
    ModifyProperty,
    AddEntity,
    RemoveEntity,
    CreateRelationship,
    RemoveRelationship,
    TriggerEvent,
}

/// Effect of an action on the simulation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub effect_type: EffectType,
    pub entity_id: Option<Uuid>,
    pub property_name: Option<String>,
    pub new_value: Option<String>,
    pub probability: f64,
    pub delay_ms: u64,
}

/// Types of conditions for action preconditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    PropertyEquals,
    PropertyNotEquals,
    PropertyGreaterThan,
    PropertyLessThan,
    EntityExists,
    EntityNotExists,
    RelationshipExists,
    CustomPredicate,
}

/// Comparison operators for conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
}

/// Condition that must be met for an action to be applicable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub entity_id: Option<Uuid>,
    pub property_name: Option<String>,
    pub expected_value: String,
    pub operator: ComparisonOperator,
    pub required_confidence: f64,
}

/// Action that can be taken in the simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub preconditions: Vec<Condition>,
    pub effects: Vec<Effect>,
    pub confidence: f64,
    pub duration_ms: u64,
    pub priority: ActionPriority,
    pub context: HashMap<String, String>,
}

/// Configuration for branching simulation behavior
#[derive(Debug, Clone)]
pub struct BranchingConfig {
    pub max_branches_per_step: usize,
    pub max_branching_depth: usize,
    pub min_branch_confidence: f64,
    pub max_active_branches: usize,
    pub pruning_threshold: f64,
    pub enable_aggressive_pruning: bool,
    pub max_simulation_time_seconds: u64,
}

impl Default for BranchingConfig {
    fn default() -> Self {
        Self {
            max_branches_per_step: 3,
            max_branching_depth: 5,
            min_branch_confidence: 0.3,
            max_active_branches: 20,
            pruning_threshold: 0.2,
            enable_aggressive_pruning: false,
            max_simulation_time_seconds: 120,
        }
    }
}

/// Configuration for confidence scoring
#[derive(Debug, Clone)]
pub struct ConfidenceConfig {
    pub rule_confidence_weight: f64,
    pub path_likelihood_weight: f64,
    pub state_consistency_weight: f64,
    pub historical_accuracy_weight: f64,
    pub confidence_decay_factor: f64,
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

/// Types of simulation constraints
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    MustReach,
    MustAvoid,
    Avoidance,
    Maintenance,
    PreferPath,
    MinimizeSteps,
    MaximizeConfidence,
    ResourceLimit,
    TimeLimit,
    CustomGoal,
}

/// Constraint to guide simulation exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConstraint {
    pub id: Uuid,
    pub constraint_type: ConstraintType,
    pub target_entity: Option<Uuid>,
    pub target_property: Option<String>,
    pub target_value: Option<String>,
    pub weight: f64,
    pub priority: ActionPriority,
    pub description: String,
}

/// Branch in the simulation tree
#[derive(Debug, Clone)]
pub struct SimulationBranch {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub state: SimulationState,
    pub action_taken: Option<Action>,
    pub confidence: f64,
    pub depth: usize,
    pub created_at: DateTime<Utc>,
    pub is_pruned: bool,
    pub constraint_satisfaction: f64,
}

/// Result of a branching simulation
#[derive(Debug, Clone)]
pub struct BranchingResult {
    pub total_branches_explored: usize,
    pub total_branches_pruned: usize,
    pub overall_confidence: f64,
    pub execution_time_ms: u64,
    pub most_likely_outcomes: Vec<SimulationBranch>,
    pub constraint_satisfaction_score: f64,
    pub final_states: Vec<SimulationState>,
    pub pruning_statistics: PruningStatistics,
}

/// Statistics about pruning operations
#[derive(Debug, Clone)]
pub struct PruningStatistics {
    pub low_confidence_pruned: usize,
    pub resource_limit_pruned: usize,
    pub constraint_violation_pruned: usize,
    pub time_limit_pruned: usize,
    pub aggressive_pruned: usize,
}

/// Simulation state containing entities and their properties
#[derive(Debug, Clone)]
pub struct SimulationState {
    pub id: Uuid,
    pub description: String,
    pub entities: HashMap<Uuid, ConceptNode>,
    pub entity_properties: HashMap<Uuid, Vec<StateProperty>>,
    pub global_properties: Vec<StateProperty>,
    pub timestamp: DateTime<Utc>,
    pub step_number: usize,
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            description: String::new(),
            entities: HashMap::new(),
            entity_properties: HashMap::new(),
            global_properties: Vec::new(),
            timestamp: Utc::now(),
            step_number: 0,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn add_entity(&mut self, entity: ConceptNode, properties: Vec<StateProperty>) {
        let entity_id = entity.id;
        self.entities.insert(entity_id, entity);
        self.entity_properties.insert(entity_id, properties);
    }

    pub fn remove_entity(&mut self, entity_id: Uuid) {
        self.entities.remove(&entity_id);
        self.entity_properties.remove(&entity_id);
    }

    pub fn get_entity_property(&self, entity_id: Uuid, property_name: &str) -> Option<&StateProperty> {
        self.entity_properties.get(&entity_id)?
            .iter()
            .find(|prop| prop.name == property_name)
    }

    pub fn set_entity_property(&mut self, entity_id: Uuid, property: StateProperty) {
        if let Some(properties) = self.entity_properties.get_mut(&entity_id) {
            // Remove existing property with same name
            properties.retain(|p| p.name != property.name);
            properties.push(property);
        }
    }

    pub fn add_global_property(&mut self, property: StateProperty) {
        self.global_properties.retain(|p| p.name != property.name);
        self.global_properties.push(property);
    }

    /// Calculate consistency score of the state
    pub fn calculate_consistency(&self) -> f64 {
        // Simple consistency check - could be enhanced with domain knowledge
        let mut consistency_score: f64 = 1.0;
        
        // Check for conflicting properties
        for properties in self.entity_properties.values() {
            for prop in properties {
                if prop.confidence < 0.5 {
                    consistency_score *= 0.9;
                }
            }
        }

        consistency_score.max(0.0)
    }

    /// Clone state with new ID and timestamp
    pub fn clone_for_branch(&self) -> Self {
        let mut new_state = self.clone();
        new_state.id = Uuid::new_v4();
        new_state.timestamp = Utc::now();
        new_state.step_number += 1;
        new_state
    }
}

/// Main simulation engine with branching capabilities
pub struct SimulationEngine {
    #[allow(dead_code)]
    concept_graph: crate::ConceptGraphManager,
    current_state: Option<SimulationState>,
    available_actions: Vec<Action>,
    constraints: Vec<SimulationConstraint>,
    branching_config: BranchingConfig,
    confidence_config: ConfidenceConfig,
    active_branches: Vec<SimulationBranch>,
    simulation_history: Vec<SimulationBranch>,
}

impl SimulationEngine {
    pub fn new(concept_graph: crate::ConceptGraphManager) -> Self {
        Self {
            concept_graph,
            current_state: None,
            available_actions: Vec::new(),
            constraints: Vec::new(),
            branching_config: BranchingConfig::default(),
            confidence_config: ConfidenceConfig::default(),
            active_branches: Vec::new(),
            simulation_history: Vec::new(),
        }
    }

    pub fn set_branching_config(&mut self, config: BranchingConfig) {
        self.branching_config = config;
    }

    pub fn set_confidence_config(&mut self, config: ConfidenceConfig) {
        self.confidence_config = config;
    }

    pub fn reset(&mut self) {
        self.current_state = None;
        self.active_branches.clear();
        self.simulation_history.clear();
    }

    /// Initialize simulation from text description
    pub async fn initialize_from_text(&mut self, description: &str) -> Result<Uuid> {
        let mut state = SimulationState::new();
        state.set_description(description.to_string());

        // Parse description to extract entities and properties
        // This is a simplified implementation - in practice would use NLP
        self.parse_initial_state(&mut state, description).await?;

        let state_id = state.id;
        self.current_state = Some(state);

        Ok(state_id)
    }

    /// Parse text description to populate initial state
    async fn parse_initial_state(&mut self, state: &mut SimulationState, description: &str) -> Result<()> {
        // Simple keyword-based parsing for demo
        if description.contains("person") {
            let _person_id = Uuid::new_v4();
            let person_concept = ConceptNode::new(
                ConceptType::Entity,
                "person".to_string(),
                0.95,
                Some("simulation_entity".to_string()),
            );

            let mut properties = Vec::new();
            
            if description.contains("room") {
                properties.push(StateProperty {
                    name: "location".to_string(),
                    value: "room".to_string(),
                    property_type: PropertyType::Location,
                    confidence: 0.9,
                    source: "text_parsing".to_string(),
                });
            }

            if description.contains("stands") {
                properties.push(StateProperty {
                    name: "position".to_string(),
                    value: "center".to_string(),
                    property_type: PropertyType::State,
                    confidence: 0.8,
                    source: "text_parsing".to_string(),
                });
            }

            state.add_entity(person_concept, properties);
        }

        // Add room entities if mentioned
        if description.contains("door") {
            let _door_id = Uuid::new_v4();
            let door_concept = ConceptNode::new(
                ConceptType::Entity,
                "door".to_string(),
                0.9,
                Some("simulation_entity".to_string()),
            );
            state.add_entity(door_concept, vec![
                StateProperty {
                    name: "state".to_string(),
                    value: "closed".to_string(),
                    property_type: PropertyType::State,
                    confidence: 0.7,
                    source: "text_parsing".to_string(),
                }
            ]);
        }

        if description.contains("window") {
            let _window_id = Uuid::new_v4();
            let window_concept = ConceptNode::new(
                ConceptType::Entity,
                "window".to_string(),
                0.9,
                Some("simulation_entity".to_string()),
            );
            state.add_entity(window_concept, vec![
                StateProperty {
                    name: "state".to_string(),
                    value: "closed".to_string(),
                    property_type: PropertyType::State,
                    confidence: 0.7,
                    source: "text_parsing".to_string(),
                }
            ]);
        }

        Ok(())
    }

    pub fn add_action(&mut self, action: Action) {
        self.available_actions.push(action);
    }

    pub fn get_available_actions(&self) -> &Vec<Action> {
        &self.available_actions
    }

    pub fn add_constraint(&mut self, constraint: SimulationConstraint) {
        self.constraints.push(constraint);
    }

    pub fn get_constraints(&self) -> &Vec<SimulationConstraint> {
        &self.constraints
    }

    /// Run branching simulation for specified number of steps
    pub async fn run_branching_simulation(&mut self, max_steps: usize) -> Result<BranchingResult> {
        let start_time = Instant::now();
        
        if self.current_state.is_none() {
            return Err(BrainError::Other("No initial state set".to_string()).into());
        }

        // Initialize with current state as root branch
        let initial_state = self.current_state.as_ref().unwrap().clone();
        let root_branch = SimulationBranch {
            id: Uuid::new_v4(),
            parent_id: None,
            state: initial_state,
            action_taken: None,
            confidence: 1.0,
            depth: 0,
            created_at: Utc::now(),
            is_pruned: false,
            constraint_satisfaction: self.calculate_constraint_satisfaction(&self.current_state.as_ref().unwrap()),
        };

        self.active_branches = vec![root_branch];
        self.simulation_history.clear();

        let mut total_branches_explored = 1;
        let mut pruning_stats = PruningStatistics {
            low_confidence_pruned: 0,
            resource_limit_pruned: 0,
            constraint_violation_pruned: 0,
            time_limit_pruned: 0,
            aggressive_pruned: 0,
        };

        // Main simulation loop
        for _step in 0..max_steps {
            if self.active_branches.is_empty() {
                break;
            }

            let mut new_branches = Vec::new();
            
            // Extract branches to process to avoid borrowing conflicts
            let branches_to_process: Vec<SimulationBranch> = self.active_branches.drain(..).collect();

            // Process each active branch
            for branch in branches_to_process {
                if branch.depth >= self.branching_config.max_branching_depth {
                    self.simulation_history.push(branch);
                    continue;
                }

                // Find applicable actions for this branch
                let applicable_actions = self.find_applicable_actions(&branch.state);

                // Create new branches for each applicable action
                let mut branch_count = 0;
                for action in applicable_actions {
                    if branch_count >= self.branching_config.max_branches_per_step {
                        break;
                    }

                    // Apply action to create new state
                    let mut new_state = branch.state.clone_for_branch();
                    self.apply_action(&mut new_state, &action).await?;

                    // Calculate confidence for new branch
                    let confidence = self.calculate_branch_confidence(&new_state, &action, &branch);
                    
                    if confidence < self.branching_config.min_branch_confidence {
                        pruning_stats.low_confidence_pruned += 1;
                        continue;
                    }

                    let constraint_satisfaction = self.calculate_constraint_satisfaction(&new_state);

                    let new_branch = SimulationBranch {
                        id: Uuid::new_v4(),
                        parent_id: Some(branch.id),
                        state: new_state,
                        action_taken: Some(action),
                        confidence,
                        depth: branch.depth + 1,
                        created_at: Utc::now(),
                        is_pruned: false,
                        constraint_satisfaction,
                    };

                    new_branches.push(new_branch);
                    total_branches_explored += 1;
                    branch_count += 1;
                }

                self.simulation_history.push(branch);
            }

            // Apply pruning
            self.prune_branches(&mut new_branches, &mut pruning_stats);

            // Check branch limits
            if new_branches.len() > self.branching_config.max_active_branches {
                new_branches.sort_by(|a, b| {
                    b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal)
                });
                new_branches.truncate(self.branching_config.max_active_branches);
                pruning_stats.resource_limit_pruned += total_branches_explored - new_branches.len() - self.simulation_history.len();
            }

            self.active_branches = new_branches;

            // Check time limit
            if start_time.elapsed().as_secs() > self.branching_config.max_simulation_time_seconds {
                pruning_stats.time_limit_pruned += self.active_branches.len();
                break;
            }
        }

        // Move remaining active branches to history
        self.simulation_history.extend(self.active_branches.drain(..));

        // Calculate results
        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        let total_branches_pruned = pruning_stats.low_confidence_pruned 
            + pruning_stats.resource_limit_pruned 
            + pruning_stats.constraint_violation_pruned 
            + pruning_stats.time_limit_pruned 
            + pruning_stats.aggressive_pruned;

        let overall_confidence = if self.simulation_history.is_empty() {
            0.0
        } else {
            self.simulation_history.iter().map(|b| b.confidence).sum::<f64>() / self.simulation_history.len() as f64
        };

        // Find most likely outcomes (top 5 by confidence)
        let mut most_likely_outcomes = self.simulation_history.clone();
        most_likely_outcomes.sort_by(|a, b| {
            b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal)
        });
        most_likely_outcomes.truncate(5);

        let final_states = self.simulation_history
            .iter()
            .filter(|b| b.depth == self.branching_config.max_branching_depth || 
                      self.simulation_history.iter().all(|other| other.parent_id != Some(b.id)))
            .map(|b| b.state.clone())
            .collect();

        let constraint_satisfaction_score = if self.simulation_history.is_empty() {
            0.0
        } else {
            self.simulation_history.iter().map(|b| b.constraint_satisfaction).sum::<f64>() / self.simulation_history.len() as f64
        };

        Ok(BranchingResult {
            total_branches_explored,
            total_branches_pruned,
            overall_confidence,
            execution_time_ms,
            most_likely_outcomes,
            constraint_satisfaction_score,
            final_states,
            pruning_statistics: pruning_stats,
        })
    }

    /// Find actions applicable to the current state
    fn find_applicable_actions(&self, state: &SimulationState) -> Vec<Action> {
        self.available_actions
            .iter()
            .filter(|action| self.check_preconditions(state, &action.preconditions))
            .cloned()
            .collect()
    }

    /// Check if all preconditions are satisfied
    fn check_preconditions(&self, state: &SimulationState, preconditions: &[Condition]) -> bool {
        preconditions.iter().all(|condition| self.evaluate_condition(state, condition))
    }

    /// Evaluate a single condition
    fn evaluate_condition(&self, state: &SimulationState, condition: &Condition) -> bool {
        match condition.condition_type {
            ConditionType::PropertyEquals => {
                if let Some(property_name) = &condition.property_name {
                    if let Some(entity_id) = condition.entity_id {
                        if let Some(property) = state.get_entity_property(entity_id, property_name) {
                            return property.confidence >= condition.required_confidence &&
                                   self.compare_values(&property.value, &condition.expected_value, &condition.operator);
                        }
                    } else {
                        // Check global properties
                        if let Some(property) = state.global_properties.iter().find(|p| p.name == *property_name) {
                            return property.confidence >= condition.required_confidence &&
                                   self.compare_values(&property.value, &condition.expected_value, &condition.operator);
                        }
                    }
                }
            }
            ConditionType::EntityExists => {
                if let Some(entity_id) = condition.entity_id {
                    return state.entities.contains_key(&entity_id);
                }
            }
            // Add more condition types as needed
            _ => {}
        }

        false
    }

    /// Compare values using the specified operator
    fn compare_values(&self, actual: &str, expected: &str, operator: &ComparisonOperator) -> bool {
        match operator {
            ComparisonOperator::Equals => actual == expected,
            ComparisonOperator::NotEquals => actual != expected,
            ComparisonOperator::Contains => actual.contains(expected),
            ComparisonOperator::StartsWith => actual.starts_with(expected),
            ComparisonOperator::EndsWith => actual.ends_with(expected),
            // Add numeric comparisons if needed
            _ => false,
        }
    }

    /// Apply an action to a state
    async fn apply_action(&mut self, state: &mut SimulationState, action: &Action) -> Result<()> {
        for effect in &action.effects {
            match effect.effect_type {
                EffectType::SetProperty => {
                    if let (Some(property_name), Some(new_value)) = (&effect.property_name, &effect.new_value) {
                        if let Some(entity_id) = effect.entity_id {
                            let property = StateProperty {
                                name: property_name.clone(),
                                value: new_value.clone(),
                                property_type: PropertyType::State,
                                confidence: effect.probability,
                                source: format!("action:{}", action.name),
                            };
                            state.set_entity_property(entity_id, property);
                        } else {
                            let property = StateProperty {
                                name: property_name.clone(),
                                value: new_value.clone(),
                                property_type: PropertyType::State,
                                confidence: effect.probability,
                                source: format!("action:{}", action.name),
                            };
                            state.add_global_property(property);
                        }
                    }
                }
                // Add more effect types as needed
                _ => {}
            }
        }

        Ok(())
    }

    /// Calculate confidence for a new branch
    fn calculate_branch_confidence(&self, state: &SimulationState, action: &Action, parent_branch: &SimulationBranch) -> f64 {
        let rule_confidence = action.confidence * self.confidence_config.rule_confidence_weight;
        let path_likelihood = parent_branch.confidence * self.confidence_config.path_likelihood_weight;
        let state_consistency = state.calculate_consistency() * self.confidence_config.state_consistency_weight;
        let constraint_bonus = self.calculate_constraint_satisfaction(state) * self.confidence_config.constraint_satisfaction_bonus;

        let base_confidence = rule_confidence + path_likelihood + state_consistency;
        let decay = self.confidence_config.confidence_decay_factor.powf(parent_branch.depth as f64);
        
        ((base_confidence + constraint_bonus) * decay).min(1.0).max(0.0)
    }

    /// Calculate constraint satisfaction score for a state
    fn calculate_constraint_satisfaction(&self, state: &SimulationState) -> f64 {
        if self.constraints.is_empty() {
            return 1.0;
        }

        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for constraint in &self.constraints {
            let satisfaction = self.evaluate_constraint_satisfaction(state, constraint);
            total_score += satisfaction * constraint.weight;
            total_weight += constraint.weight;
        }

        if total_weight > 0.0 {
            total_score / total_weight
        } else {
            1.0
        }
    }

    /// Evaluate how well a single constraint is satisfied
    fn evaluate_constraint_satisfaction(&self, state: &SimulationState, constraint: &SimulationConstraint) -> f64 {
        match constraint.constraint_type {
            ConstraintType::MustReach => {
                // Check if target state/property is reached
                if let (Some(target_property), Some(target_value)) = (&constraint.target_property, &constraint.target_value) {
                    if let Some(entity_id) = constraint.target_entity {
                        if let Some(property) = state.get_entity_property(entity_id, target_property) {
                            return if property.value == *target_value { 1.0 } else { 0.0 };
                        }
                    }
                }
                0.0
            }
            ConstraintType::MustAvoid | ConstraintType::Avoidance => {
                // Check if forbidden state is avoided
                if let (Some(target_property), Some(target_value)) = (&constraint.target_property, &constraint.target_value) {
                    if let Some(entity_id) = constraint.target_entity {
                        if let Some(property) = state.get_entity_property(entity_id, target_property) {
                            return if property.value != *target_value { 1.0 } else { 0.0 };
                        }
                    }
                }
                1.0 // If property doesn't exist, it's avoided
            }
            ConstraintType::Maintenance => {
                // Check if required state is maintained
                if let (Some(target_property), Some(target_value)) = (&constraint.target_property, &constraint.target_value) {
                    if let Some(entity_id) = constraint.target_entity {
                        if let Some(property) = state.get_entity_property(entity_id, target_property) {
                            return if property.value == *target_value { 1.0 } else { 0.0 };
                        }
                    }
                }
                0.0
            }
            ConstraintType::MaximizeConfidence => {
                state.calculate_consistency()
            }
            // Add more constraint types as needed
            _ => 0.5, // Neutral satisfaction for unimplemented constraints
        }
    }

    /// Prune branches based on various criteria
    fn prune_branches(&self, branches: &mut Vec<SimulationBranch>, stats: &mut PruningStatistics) {
        // Low confidence pruning
        branches.retain(|branch| {
            if branch.confidence < self.branching_config.pruning_threshold {
                stats.low_confidence_pruned += 1;
                false
            } else {
                true
            }
        });

        // Aggressive pruning if enabled
        if self.branching_config.enable_aggressive_pruning && branches.len() > self.branching_config.max_active_branches {
            let target_size = self.branching_config.max_active_branches / 2;
            branches.sort_by(|a, b| {
                b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal)
            });
            if branches.len() > target_size {
                stats.aggressive_pruned += branches.len() - target_size;
                branches.truncate(target_size);
            }
        }
    }
} 