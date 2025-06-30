//! Branching Simulation Demo - Task 6.3 Showcase
//!
//! This demo showcases the advanced branching simulation capabilities
//! implemented in Task 6.3, including:
//! - Tree-based branching structure
//! - Confidence scoring algorithms
//! - Pruning mechanisms for complexity management
//! - Constraint injection for guided exploration
//! - Comprehensive result analysis

use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

use brain::simulation_engine::{
    SimulationEngine, SimulationState, StateProperty, PropertyType,
    Action, ActionPriority, Effect, EffectType, Condition, ConditionType, ComparisonOperator,
    BranchingConfig, SimulationConstraint, ConstraintType, BranchingResult,
};
use brain::SimulationConfidenceConfig as ConfidenceConfig;
use brain::concept_graph::{ConceptGraphManager, ConceptNode, ConceptType};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Brain AI - Branching Simulation Demo (Task 6.3)");
    println!("==================================================\n");

    // Initialize the simulation engine
    let config = brain::ConceptGraphConfig::default();
    let concept_graph = ConceptGraphManager::new(config).await?;
    let mut engine = SimulationEngine::new(concept_graph);

    // Configure branching parameters for demonstration
    let branching_config = BranchingConfig {
        max_branches_per_step: 3,
        max_branching_depth: 4,
        min_branch_confidence: 0.3,
        max_active_branches: 15,
        pruning_threshold: 0.2,
        enable_aggressive_pruning: true,
        max_simulation_time_seconds: 60,
    };
    engine.set_branching_config(branching_config);

    // Configure confidence scoring
    let confidence_config = ConfidenceConfig {
        rule_confidence_weight: 0.4,
        path_likelihood_weight: 0.3,
        state_consistency_weight: 0.2,
        historical_accuracy_weight: 0.1,
        confidence_decay_factor: 0.95,
        constraint_satisfaction_bonus: 0.1,
    };
    engine.set_confidence_config(confidence_config);

    // Demo 1: Basic Branching Simulation
    println!("📊 Demo 1: Basic Branching Simulation");
    println!("-------------------------------------");
    
    let _initial_state = create_demo_scenario().await?;
    engine.reset();
    
    // Set initial state (simulating initialization from text)
    let state_id = engine.initialize_from_text("A person stands in a room with a door and a window").await?;
    println!("✅ Initialized simulation with state ID: {}", state_id);

    // Add some demo actions
    add_demo_actions(&mut engine)?;
    println!("✅ Added {} demo actions", engine.get_available_actions().len());

    // Run branching simulation
    println!("\n🌳 Running branching simulation...");
    let result = engine.run_branching_simulation(5).await?;
    
    println!("📈 Simulation Results:");
    println!("  • Total branches explored: {}", result.total_branches_explored);
    println!("  • Total branches pruned: {}", result.total_branches_pruned);
    println!("  • Overall confidence: {:.3}", result.overall_confidence);
    println!("  • Execution time: {}ms", result.execution_time_ms);
    println!("  • Most likely outcomes: {} branches", result.most_likely_outcomes.len());

    // Demo 2: Constraint-Guided Simulation
    println!("\n📊 Demo 2: Constraint-Guided Simulation");
    println!("---------------------------------------");
    
    // Add constraints to guide the simulation
    add_demo_constraints(&mut engine)?;
    println!("✅ Added {} simulation constraints", engine.get_constraints().len());

    let constrained_result = engine.run_branching_simulation(4).await?;
    
    println!("📈 Constrained Simulation Results:");
    println!("  • Total branches explored: {}", constrained_result.total_branches_explored);
    println!("  • Total branches pruned: {}", constrained_result.total_branches_pruned);
    println!("  • Overall confidence: {:.3}", constrained_result.overall_confidence);
    println!("  • Execution time: {}ms", constrained_result.execution_time_ms);

    // Demo 3: Detailed Branch Analysis
    println!("\n📊 Demo 3: Detailed Branch Analysis");
    println!("-----------------------------------");
    
    analyze_branching_results(&constrained_result)?;

    // Demo 4: Confidence Scoring Analysis
    println!("\n📊 Demo 4: Confidence Scoring Analysis");
    println!("--------------------------------------");
    
    analyze_confidence_scoring(&constrained_result)?;

    // Demo 5: Pruning Mechanism Demonstration
    println!("\n📊 Demo 5: Pruning Mechanism Analysis");
    println!("-------------------------------------");
    
    demonstrate_pruning_mechanisms(&constrained_result)?;

    println!("\n🎉 Branching Simulation Demo Complete!");
    println!("Task 6.3 successfully demonstrates:");
    println!("  ✅ Tree-based branching structure");
    println!("  ✅ Advanced confidence scoring");
    println!("  ✅ Intelligent pruning mechanisms");
    println!("  ✅ Constraint-guided exploration");
    println!("  ✅ Comprehensive result analysis");

    Ok(())
}

/// Create a demo scenario for simulation
async fn create_demo_scenario() -> Result<SimulationState> {
    let mut state = SimulationState::new();
    state.set_description("Demo scenario: Person in a room with multiple interaction possibilities".to_string());
    
    // Add demo entities (these would normally come from concept graph)
    let person_concept = ConceptNode::new(
        ConceptType::Entity,
        "person".to_string(),
        0.95,
        Some("demo_scenario".to_string()),
    );
    
    let person_properties = vec![
        StateProperty {
            name: "position".to_string(),
            value: "center".to_string(),
            property_type: PropertyType::Location,
            confidence: 0.9,
            source: "initial_state".to_string(),
        },
        StateProperty {
            name: "energy".to_string(),
            value: "high".to_string(),
            property_type: PropertyType::State,
            confidence: 0.8,
            source: "initial_state".to_string(),
        },
    ];
    
    state.add_entity(person_concept, person_properties);
    
    Ok(state)
}

/// Add demonstration actions to the simulation engine
fn add_demo_actions(engine: &mut SimulationEngine) -> Result<()> {
    // Action 1: Move to door
    let move_to_door = Action {
        id: Uuid::new_v4(),
        name: "move_to_door".to_string(),
        description: "Move towards the door".to_string(),
        preconditions: vec![
            Condition {
                condition_type: ConditionType::PropertyEquals,
                entity_id: None,
                property_name: Some("position".to_string()),
                expected_value: "center".to_string(),
                operator: ComparisonOperator::Equals,
                required_confidence: 0.7,
            }
        ],
        effects: vec![
            Effect {
                effect_type: EffectType::SetProperty,
                entity_id: None,
                property_name: Some("position".to_string()),
                new_value: Some("near_door".to_string()),
                probability: 0.9,
                delay_ms: 1000,
            }
        ],
        confidence: 0.85,
        duration_ms: 2000,
        priority: ActionPriority::Medium,
        context: HashMap::new(),
    };
    
    // Action 2: Move to window
    let move_to_window = Action {
        id: Uuid::new_v4(),
        name: "move_to_window".to_string(),
        description: "Move towards the window".to_string(),
        preconditions: vec![
            Condition {
                condition_type: ConditionType::PropertyEquals,
                entity_id: None,
                property_name: Some("position".to_string()),
                expected_value: "center".to_string(),
                operator: ComparisonOperator::Equals,
                required_confidence: 0.7,
            }
        ],
        effects: vec![
            Effect {
                effect_type: EffectType::SetProperty,
                entity_id: None,
                property_name: Some("position".to_string()),
                new_value: Some("near_window".to_string()),
                probability: 0.8,
                delay_ms: 1500,
            }
        ],
        confidence: 0.75,
        duration_ms: 2500,
        priority: ActionPriority::Low,
        context: HashMap::new(),
    };
    
    // Action 3: Rest (available from any position)
    let rest_action = Action {
        id: Uuid::new_v4(),
        name: "rest".to_string(),
        description: "Rest and recover energy".to_string(),
        preconditions: vec![
            Condition {
                condition_type: ConditionType::PropertyEquals,
                entity_id: None,
                property_name: Some("energy".to_string()),
                expected_value: "low".to_string(),
                operator: ComparisonOperator::Equals,
                required_confidence: 0.6,
            }
        ],
        effects: vec![
            Effect {
                effect_type: EffectType::SetProperty,
                entity_id: None,
                property_name: Some("energy".to_string()),
                new_value: Some("high".to_string()),
                probability: 0.95,
                delay_ms: 500,
            }
        ],
        confidence: 0.9,
        duration_ms: 3000,
        priority: ActionPriority::High,
        context: HashMap::new(),
    };

    engine.add_action(move_to_door);
    engine.add_action(move_to_window);
    engine.add_action(rest_action);
    
    Ok(())
}

/// Add demonstration constraints
fn add_demo_constraints(engine: &mut SimulationEngine) -> Result<()> {
    // Constraint 1: Avoid staying in center too long
    let avoid_center = SimulationConstraint {
        id: Uuid::new_v4(),
        constraint_type: ConstraintType::Avoidance,
        target_entity: None,
        target_property: Some("position".to_string()),
        target_value: Some("center".to_string()),
        weight: 0.7,
        priority: ActionPriority::Medium,
        description: "Avoid staying in center position for too long".to_string(),
    };
    
    // Constraint 2: Maintain high energy
    let maintain_energy = SimulationConstraint {
        id: Uuid::new_v4(),
        constraint_type: ConstraintType::Maintenance,
        target_entity: None,
        target_property: Some("energy".to_string()),
        target_value: Some("high".to_string()),
        weight: 0.8,
        priority: ActionPriority::High,
        description: "Try to maintain high energy levels".to_string(),
    };

    engine.add_constraint(avoid_center);
    engine.add_constraint(maintain_energy);
    
    Ok(())
}

/// Analyze branching results in detail
fn analyze_branching_results(result: &BranchingResult) -> Result<()> {
    println!("🌳 Branch Tree Analysis:");
    println!("  • Total branches explored: {}", result.total_branches_explored);
    println!("  • Total branches pruned: {}", result.total_branches_pruned);
    println!("  • Final states: {}", result.final_states.len());
    
    // Analyze branch depths from most likely outcomes
    let depths: Vec<usize> = result.most_likely_outcomes.iter().map(|b| b.depth).collect();
    let max_depth = depths.iter().max().unwrap_or(&0);
    let avg_depth = if !depths.is_empty() { 
        depths.iter().sum::<usize>() as f64 / depths.len() as f64 
    } else { 0.0 };
    
    println!("  • Maximum depth reached: {}", max_depth);
    println!("  • Average branch depth: {:.2}", avg_depth);
    
    // Analyze pruning statistics
    println!("  • Pruning breakdown:");
    println!("    - Low confidence: {}", result.pruning_statistics.low_confidence_pruned);
    println!("    - Resource limit: {}", result.pruning_statistics.resource_limit_pruned);
    println!("    - Constraint violation: {}", result.pruning_statistics.constraint_violation_pruned);
    println!("    - Time limit: {}", result.pruning_statistics.time_limit_pruned);
    println!("    - Aggressive pruning: {}", result.pruning_statistics.aggressive_pruned);
    
    Ok(())
}

/// Analyze confidence scoring in detail
fn analyze_confidence_scoring(result: &BranchingResult) -> Result<()> {
    println!("📊 Confidence Scoring Analysis:");
    
    // Collect confidence scores from most likely outcomes
    let confidences: Vec<f64> = result.most_likely_outcomes
        .iter()
        .map(|b| b.confidence)
        .collect();
    
    if !confidences.is_empty() {
        let max_conf = confidences.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min_conf = confidences.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let avg_conf = confidences.iter().sum::<f64>() / confidences.len() as f64;
        
        println!("  • Confidence range: {:.3} - {:.3}", min_conf, max_conf);
        println!("  • Average confidence: {:.3}", avg_conf);
        println!("  • Overall simulation confidence: {:.3}", result.overall_confidence);
        println!("  • Constraint satisfaction: {:.3}", result.constraint_satisfaction_score);
    }
    
    // Analyze most likely outcomes
    println!("  • Most likely outcomes: {} branches", result.most_likely_outcomes.len());
    for (i, branch) in result.most_likely_outcomes.iter().take(3).enumerate() {
        println!("    {}. Branch {:?} (confidence: {:.3}, depth: {})", 
            i + 1, branch.id, branch.confidence, branch.depth);
    }
    
    Ok(())
}

/// Demonstrate pruning mechanisms
fn demonstrate_pruning_mechanisms(result: &BranchingResult) -> Result<()> {
    println!("✂️ Pruning Mechanism Analysis:");
    
    let pruning_ratio = if result.total_branches_explored > 0 {
        result.total_branches_pruned as f64 / result.total_branches_explored as f64
    } else {
        0.0
    };
    
    println!("  • Pruning efficiency: {:.1}% ({}/{} branches pruned)", 
        pruning_ratio * 100.0, result.total_branches_pruned, result.total_branches_explored);
    
    // Analyze available statistics
    println!("  • Available metrics:");
    println!("    - Overall confidence: {:.3}", result.overall_confidence);
    println!("    - Constraint satisfaction: {:.3}", result.constraint_satisfaction_score);
    println!("    - Most likely outcomes: {} branches", result.most_likely_outcomes.len());
    println!("    - Final states: {} states", result.final_states.len());
    println!("    - Execution time: {}ms", result.execution_time_ms);
    
    // Detailed pruning breakdown
    let stats = &result.pruning_statistics;
    println!("  • Detailed pruning breakdown:");
    println!("    - Low confidence pruned: {}", stats.low_confidence_pruned);
    println!("    - Resource limit pruned: {}", stats.resource_limit_pruned);
    println!("    - Constraint violation pruned: {}", stats.constraint_violation_pruned);
    println!("    - Time limit pruned: {}", stats.time_limit_pruned);
    println!("    - Aggressive pruned: {}", stats.aggressive_pruned);
    
    println!("\n💡 Pruning helps manage computational complexity while preserving");
    println!("   the most promising simulation paths for exploration.");
    
    Ok(())
} 