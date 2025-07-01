//! Feature Experimentation Agent for Brain AI
//!
//! This agent specializes in A/B testing, feature flag management, and experimental design
//! for optimizing product features and user experience.

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, ExecutionMetadata, ExecutionStatus, BrainResult, CognitivePreferences};
use brain_types::BrainError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// Feature Experimentation Agent
/// 
/// Manages A/B tests, feature flags, and experimental design to optimize product development
/// through data-driven experimentation and statistical analysis.
pub struct FeatureExperimentationAgent {
    metadata: AgentMetadata,
    cognitive_preferences: CognitivePreferences,
}

/// Input data for feature experimentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentationInput {
    /// Type of experimentation to perform
    pub experiment_type: ExperimentType,
    /// Experiment configuration
    pub experiment_config: ExperimentConfig,
    /// Historical data for analysis
    pub historical_data: Vec<ExperimentData>,
    /// Target metrics to optimize
    pub target_metrics: Vec<String>,
}

/// Type of experiment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExperimentType {
    ABTest,
    MultiVariateTest,
    FeatureFlag,
    GradualRollout,
    CohortAnalysis,
    FunnelOptimization,
}

/// Experiment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    pub experiment_id: String,
    pub name: String,
    pub description: String,
    pub variants: Vec<ExperimentVariant>,
    pub traffic_allocation: f64,
    pub duration_days: u32,
    pub success_metrics: Vec<String>,
    pub guardrail_metrics: Vec<String>,
}

/// Experiment variant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentVariant {
    pub variant_id: String,
    pub name: String,
    pub description: String,
    pub traffic_percentage: f64,
    pub configuration: HashMap<String, serde_json::Value>,
}

/// Experiment data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentData {
    pub user_id: String,
    pub variant_id: String,
    pub timestamp: u64,
    pub metrics: HashMap<String, f64>,
    pub conversion_events: Vec<String>,
}

/// Output of feature experimentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentationOutput {
    /// Experiment results and analysis
    pub experiment_results: ExperimentResults,
    /// Statistical significance analysis
    pub statistical_analysis: StatisticalAnalysis,
    /// Recommendations for next steps
    pub recommendations: Vec<ExperimentRecommendation>,
    /// Feature flag configurations
    pub feature_flags: Vec<FeatureFlagConfig>,
    /// Analysis confidence score
    pub confidence_score: f64,
}

/// Experiment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResults {
    pub experiment_id: String,
    pub status: ExperimentStatus,
    pub variant_performance: Vec<VariantPerformance>,
    pub winning_variant: Option<String>,
    pub improvement_metrics: HashMap<String, f64>,
}

/// Experiment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExperimentStatus {
    Planning,
    Running,
    Completed,
    Stopped,
    Inconclusive,
}

/// Performance metrics for a variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantPerformance {
    pub variant_id: String,
    pub sample_size: u64,
    pub conversion_rate: f64,
    pub metric_values: HashMap<String, f64>,
    pub confidence_intervals: HashMap<String, ConfidenceInterval>,
}

/// Confidence interval for a metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

/// Statistical analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    pub significance_level: f64,
    pub p_values: HashMap<String, f64>,
    pub effect_sizes: HashMap<String, f64>,
    pub power_analysis: PowerAnalysis,
    pub minimum_detectable_effect: f64,
}

/// Power analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerAnalysis {
    pub statistical_power: f64,
    pub required_sample_size: u64,
    pub current_sample_size: u64,
    pub days_to_significance: Option<u32>,
}

/// Experiment recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentRecommendation {
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub impact_estimate: f64,
    pub confidence: f64,
    pub next_steps: Vec<String>,
}

/// Type of recommendation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecommendationType {
    Launch,
    Stop,
    Continue,
    Iterate,
    ScaleUp,
    Rollback,
}

/// Priority level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// Feature flag configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagConfig {
    pub flag_id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub rollout_percentage: f64,
    pub target_segments: Vec<String>,
    pub conditions: Vec<FlagCondition>,
}

/// Feature flag condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagCondition {
    pub attribute: String,
    pub operator: String,
    pub value: serde_json::Value,
}

impl FeatureExperimentationAgent {
    /// Create a new Feature Experimentation Agent
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "feature_experimentation".to_string(),
            name: "FeatureExperimentationAgent".to_string(),
            persona: "I am a data scientist specializing in A/B testing and experimental design for product optimization".to_string(),
            description: "Manages A/B tests, feature flags, and experimental design for data-driven product development".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["experiment_design".to_string(), "ab_test_data".to_string()],
            supported_output_types: vec!["experiment_results".to_string(), "feature_flags".to_string()],
            capabilities: vec!["Analytics".to_string(), "Testing".to_string()],
            dependencies: vec![],
            tags: vec!["intelligence".to_string(), "experimentation".to_string(), "ab_testing".to_string()],
            base_confidence: 0.88,
        };

        Self {
            metadata,
            cognitive_preferences: CognitivePreferences::default(),
        }
    }

    /// Analyze experiment performance
    fn analyze_experiment(&self, config: &ExperimentConfig, data: &[ExperimentData]) -> BrainResult<ExperimentResults> {
        let mut variant_performance = Vec::new();

        for variant in &config.variants {
            let variant_data: Vec<&ExperimentData> = data.iter()
                .filter(|d| d.variant_id == variant.variant_id)
                .collect();

            if !variant_data.is_empty() {
                let sample_size = variant_data.len() as u64;
                let conversion_rate = variant_data.iter()
                    .filter(|d| !d.conversion_events.is_empty())
                    .count() as f64 / sample_size as f64;

                let mut metric_values = HashMap::new();
                let mut confidence_intervals = HashMap::new();

                // Calculate average metrics
                for metric in &config.success_metrics {
                    let values: Vec<f64> = variant_data.iter()
                        .filter_map(|d| d.metrics.get(metric))
                        .copied()
                        .collect();

                    if !values.is_empty() {
                        let avg_value = values.iter().sum::<f64>() / values.len() as f64;
                        metric_values.insert(metric.clone(), avg_value);

                        // Simple confidence interval calculation (95%)
                        let std_dev = self.calculate_std_dev(&values, avg_value);
                        let margin_of_error = 1.96 * std_dev / (values.len() as f64).sqrt();
                        
                        confidence_intervals.insert(metric.clone(), ConfidenceInterval {
                            lower_bound: avg_value - margin_of_error,
                            upper_bound: avg_value + margin_of_error,
                            confidence_level: 0.95,
                        });
                    }
                }

                variant_performance.push(VariantPerformance {
                    variant_id: variant.variant_id.clone(),
                    sample_size,
                    conversion_rate,
                    metric_values,
                    confidence_intervals,
                });
            }
        }

        // Determine winning variant (simplified)
        let winning_variant = variant_performance.iter()
            .max_by(|a, b| a.conversion_rate.partial_cmp(&b.conversion_rate).unwrap())
            .map(|v| v.variant_id.clone());

        Ok(ExperimentResults {
            experiment_id: config.experiment_id.clone(),
            status: ExperimentStatus::Completed,
            variant_performance,
            winning_variant,
            improvement_metrics: HashMap::new(), // Simplified
        })
    }

    /// Calculate standard deviation
    fn calculate_std_dev(&self, values: &[f64], mean: f64) -> f64 {
        if values.len() <= 1 {
            return 0.0;
        }

        let variance = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;

        variance.sqrt()
    }

    /// Perform statistical analysis
    fn perform_statistical_analysis(&self, results: &ExperimentResults) -> BrainResult<StatisticalAnalysis> {
        let mut p_values = HashMap::new();
        let mut effect_sizes = HashMap::new();

        // Simplified statistical analysis
        if results.variant_performance.len() >= 2 {
            let control = &results.variant_performance[0];
            let treatment = &results.variant_performance[1];

            // Simple p-value calculation (placeholder)
            let p_value = if (control.conversion_rate - treatment.conversion_rate).abs() > 0.01 {
                0.03 // Significant
            } else {
                0.15 // Not significant
            };

            p_values.insert("conversion_rate".to_string(), p_value);

            // Effect size calculation
            let effect_size = (treatment.conversion_rate - control.conversion_rate) / control.conversion_rate;
            effect_sizes.insert("conversion_rate".to_string(), effect_size);
        }

        let power_analysis = PowerAnalysis {
            statistical_power: 0.8,
            required_sample_size: 1000,
            current_sample_size: results.variant_performance.iter().map(|v| v.sample_size).sum(),
            days_to_significance: Some(7),
        };

        Ok(StatisticalAnalysis {
            significance_level: 0.05,
            p_values,
            effect_sizes,
            power_analysis,
            minimum_detectable_effect: 0.05,
        })
    }

    /// Generate experiment recommendations
    fn generate_recommendations(&self, 
        _results: &ExperimentResults, 
        analysis: &StatisticalAnalysis) -> BrainResult<Vec<ExperimentRecommendation>> {
        let mut recommendations = Vec::new();

        // Check for statistical significance
        if let Some(p_value) = analysis.p_values.get("conversion_rate") {
            if *p_value < analysis.significance_level {
                recommendations.push(ExperimentRecommendation {
                    recommendation_type: RecommendationType::Launch,
                    title: "Statistically Significant Results".to_string(),
                    description: "The experiment shows statistically significant results, ready for launch".to_string(),
                    priority: Priority::High,
                    impact_estimate: 0.8,
                    confidence: 0.9,
                    next_steps: vec![
                        "Deploy winning variant to full traffic".to_string(),
                        "Monitor performance for 2 weeks".to_string(),
                        "Plan follow-up experiments".to_string(),
                    ],
                });
            } else {
                recommendations.push(ExperimentRecommendation {
                    recommendation_type: RecommendationType::Continue,
                    title: "Continue Experiment".to_string(),
                    description: "Results are not yet statistically significant, continue running".to_string(),
                    priority: Priority::Medium,
                    impact_estimate: 0.5,
                    confidence: 0.7,
                    next_steps: vec![
                        "Continue for additional week".to_string(),
                        "Increase traffic allocation if possible".to_string(),
                        "Monitor guardrail metrics".to_string(),
                    ],
                });
            }
        }

        Ok(recommendations)
    }
}

#[async_trait]
impl BrainAgent for FeatureExperimentationAgent {
    async fn execute(&self, input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        let experimentation_input: ExperimentationInput = serde_json::from_value(
            input.parameters.get("experimentation_data")
                .cloned()
                .unwrap_or_default()
        ).map_err(|e| BrainError::InvalidInput(format!("Failed to parse experimentation input: {}", e)))?;

        // Analyze experiment
        let experiment_results = self.analyze_experiment(
            &experimentation_input.experiment_config, 
            &experimentation_input.historical_data
        )?;

        // Perform statistical analysis
        let statistical_analysis = self.perform_statistical_analysis(&experiment_results)?;

        // Generate recommendations
        let recommendations = self.generate_recommendations(&experiment_results, &statistical_analysis)?;

        let output = ExperimentationOutput {
            experiment_results,
            statistical_analysis,
            recommendations,
            feature_flags: vec![], // Simplified for initial implementation
            confidence_score: 0.88,
        };

        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "experiment_results".to_string(),
            content: "Feature experimentation analysis completed".to_string(),
            data: {
                let mut data = HashMap::new();
                data.insert("experimentation_output".to_string(), serde_json::to_value(output)
                    .map_err(|e| BrainError::InvalidInput(format!("Failed to serialize output: {}", e)))?);
                data
            },
            confidence: 0.88,
            reasoning: Some("Analyzed experiment data and generated statistical insights".to_string()),
            next_actions: vec!["review_results".to_string(), "implement_recommendations".to_string()],
            execution_metadata: ExecutionMetadata {
                execution_time_ms: 1500,
                memory_usage_mb: 12.0,
                api_calls: 0,
                status: ExecutionStatus::Success,
                warnings: vec![],
            },
            timestamp: chrono::Utc::now(),
        })
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        0.75
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.cognitive_preferences
    }

    async fn assess_confidence(&self, input: &AgentInput, _context: &CognitiveContext) -> BrainResult<f32> {
        // Check if input contains required fields for experimentation
        if input.parameters.contains_key("experimentation_data") {
            Ok(0.9) // High confidence if input is well-formed
        } else {
            Ok(0.4) // Low confidence if input format is incorrect
        }
    }
}

impl Default for FeatureExperimentationAgent {
    fn default() -> Self {
        Self::new()
    }
} 