//! Optimization module for the evolution system
//! 
//! This module provides optimization strategies and implementations
//! for improving agent performance based on analysis results.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use uuid::Uuid;

use crate::agents::traits::{BrainResult, CognitiveContext};

/// Represents an optimization strategy that can be applied to agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStrategy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target_metrics: Vec<String>,
    pub expected_improvement: f64,
    pub risk_level: RiskLevel,
    pub implementation_steps: Vec<OptimizationStep>,
    pub validation_criteria: Vec<ValidationCriterion>,
}

/// Risk level of applying an optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Individual step in an optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStep {
    pub id: String,
    pub description: String,
    pub step_type: OptimizationStepType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub validation_required: bool,
}

/// Types of optimization steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationStepType {
    ParameterTuning,
    CodeOptimization,
    ArchitectureChange,
    ResourceAllocation,
    AlgorithmReplacement,
}

/// Criteria for validating optimization success
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCriterion {
    pub metric_name: String,
    pub threshold: f64,
    pub comparison: ComparisonType,
}

/// Types of comparisons for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonType {
    GreaterThan,
    LessThan,
    Equals,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Result of applying an optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub strategy_id: String,
    pub applied_at: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub metrics_before: HashMap<String, f64>,
    pub metrics_after: HashMap<String, f64>,
    pub improvement_percentage: f64,
    pub validation_results: Vec<ValidationResult>,
    pub rollback_available: bool,
}

/// Result of validating an optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub criterion: ValidationCriterion,
    pub actual_value: f64,
    pub passed: bool,
    pub message: String,
}

/// Manages optimization strategies and their application
#[derive(Debug)]
pub struct OptimizationManager {
    pub strategies: HashMap<String, OptimizationStrategy>,
    pub applied_optimizations: Vec<OptimizationResult>,
    pub rollback_history: Vec<RollbackRecord>,
}

/// Record of a rollback operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackRecord {
    pub optimization_id: String,
    pub rolled_back_at: chrono::DateTime<chrono::Utc>,
    pub reason: String,
    pub success: bool,
}

impl OptimizationManager {
    /// Create a new optimization manager
    pub fn new() -> Self {
        Self {
            strategies: HashMap::new(),
            applied_optimizations: Vec::new(),
            rollback_history: Vec::new(),
        }
    }

    /// Register a new optimization strategy
    pub fn register_strategy(&mut self, strategy: OptimizationStrategy) -> BrainResult<()> {
        self.strategies.insert(strategy.id.clone(), strategy);
        Ok(())
    }

    /// Get available strategies for specific metrics
    pub fn get_strategies_for_metrics(&self, metrics: &[String]) -> Vec<&OptimizationStrategy> {
        self.strategies
            .values()
            .filter(|strategy| {
                strategy.target_metrics.iter().any(|metric| metrics.contains(metric))
            })
            .collect()
    }

    /// Apply an optimization strategy
    pub async fn apply_optimization(
        &mut self,
        strategy_id: &str,
        _context: &CognitiveContext,
    ) -> BrainResult<OptimizationResult> {
        let _strategy = self.strategies
            .get(strategy_id)
            .ok_or_else(|| anyhow::anyhow!("Strategy not found: {}", strategy_id))?;

        // For now, simulate optimization application
        let result = OptimizationResult {
            strategy_id: strategy_id.to_string(),
            applied_at: chrono::Utc::now(),
            success: true,
            metrics_before: HashMap::new(),
            metrics_after: HashMap::new(),
            improvement_percentage: 15.0, // Simulated improvement
            validation_results: Vec::new(),
            rollback_available: true,
        };

        self.applied_optimizations.push(result.clone());
        Ok(result)
    }

    /// Rollback an optimization
    pub async fn rollback_optimization(
        &mut self,
        optimization_id: &str,
        reason: String,
    ) -> BrainResult<()> {
        // Find the optimization to rollback
        if let Some(opt) = self.applied_optimizations
            .iter()
            .find(|o| o.strategy_id == optimization_id)
        {
            if !opt.rollback_available {
                return Err(anyhow::anyhow!("Rollback not available for optimization: {}", optimization_id).into());
            }

            let rollback = RollbackRecord {
                optimization_id: optimization_id.to_string(),
                rolled_back_at: chrono::Utc::now(),
                reason,
                success: true,
            };

            self.rollback_history.push(rollback);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Optimization not found: {}", optimization_id).into())
        }
    }

    /// Get optimization history
    pub fn get_optimization_history(&self) -> &Vec<OptimizationResult> {
        &self.applied_optimizations
    }

    /// Get rollback history
    pub fn get_rollback_history(&self) -> &Vec<RollbackRecord> {
        &self.rollback_history
    }
}

impl Default for OptimizationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in optimization strategies
pub struct BuiltinStrategies;

impl BuiltinStrategies {
    /// Create performance optimization strategy
    pub fn performance_optimization() -> OptimizationStrategy {
        OptimizationStrategy {
            id: Uuid::new_v4().to_string(),
            name: "Performance Optimization".to_string(),
            description: "Optimizes agent performance through parameter tuning".to_string(),
            target_metrics: vec!["execution_time".to_string(), "throughput".to_string()],
            expected_improvement: 20.0,
            risk_level: RiskLevel::Medium,
            implementation_steps: vec![
                OptimizationStep {
                    id: Uuid::new_v4().to_string(),
                    description: "Analyze current performance bottlenecks".to_string(),
                    step_type: OptimizationStepType::ParameterTuning,
                    parameters: HashMap::new(),
                    validation_required: true,
                },
            ],
            validation_criteria: vec![
                ValidationCriterion {
                    metric_name: "execution_time".to_string(),
                    threshold: 0.8, // 20% improvement
                    comparison: ComparisonType::LessThan,
                },
            ],
        }
    }

    /// Create memory optimization strategy
    pub fn memory_optimization() -> OptimizationStrategy {
        OptimizationStrategy {
            id: Uuid::new_v4().to_string(),
            name: "Memory Optimization".to_string(),
            description: "Optimizes memory usage and reduces overhead".to_string(),
            target_metrics: vec!["memory_usage".to_string(), "allocation_rate".to_string()],
            expected_improvement: 15.0,
            risk_level: RiskLevel::Low,
            implementation_steps: vec![
                OptimizationStep {
                    id: Uuid::new_v4().to_string(),
                    description: "Optimize memory allocation patterns".to_string(),
                    step_type: OptimizationStepType::CodeOptimization,
                    parameters: HashMap::new(),
                    validation_required: true,
                },
            ],
            validation_criteria: vec![
                ValidationCriterion {
                    metric_name: "memory_usage".to_string(),
                    threshold: 0.85, // 15% reduction
                    comparison: ComparisonType::LessThan,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimization_manager_creation() {
        let manager = OptimizationManager::new();
        assert!(manager.strategies.is_empty());
        assert!(manager.applied_optimizations.is_empty());
    }

    #[tokio::test]
    async fn test_strategy_registration() {
        let mut manager = OptimizationManager::new();
        let strategy = BuiltinStrategies::performance_optimization();
        let strategy_id = strategy.id.clone();

        manager.register_strategy(strategy).unwrap();
        assert!(manager.strategies.contains_key(&strategy_id));
    }

    #[tokio::test]
    async fn test_builtin_strategies() {
        let perf_strategy = BuiltinStrategies::performance_optimization();
        assert_eq!(perf_strategy.name, "Performance Optimization");
        assert_eq!(perf_strategy.risk_level, RiskLevel::Medium);

        let mem_strategy = BuiltinStrategies::memory_optimization();
        assert_eq!(mem_strategy.name, "Memory Optimization");
        assert_eq!(mem_strategy.risk_level, RiskLevel::Low);
    }
} 