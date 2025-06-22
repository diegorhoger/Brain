//! Integration API for Character Predictor and Segment Discovery
//! 
//! This module provides trait-based interfaces for seamless integration between
//! the BpeSegmenter and CharacterPredictor, including feedback mechanisms,
//! performance tracking, and adaptive segment selection.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::Result;

/// Feedback information for a prediction attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionFeedback {
    /// The input that was used for prediction (character or segment)
    pub input: String,
    /// Whether the input was character-level or segment-level
    pub input_type: InputType,
    /// The predicted output
    pub predicted: String,
    /// The actual expected output
    pub actual: String,
    /// Whether the prediction was correct
    pub is_correct: bool,
    /// Confidence score of the prediction (0.0-1.0)
    pub confidence: f64,
    /// Time taken for the prediction in milliseconds
    pub prediction_time_ms: u64,
    /// Timestamp when the feedback was generated
    pub timestamp: u64,
    /// Context length used for the prediction
    pub context_length: usize,
    /// Segment quality score if applicable (0.0-1.0)
    pub segment_quality: Option<f64>,
}

/// Type of input used for prediction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputType {
    Character,
    Segment,
    Hybrid,
}

/// Advanced segment quality metrics for adaptive learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentQualityMetrics {
    /// Prediction accuracy when using this segment
    pub prediction_accuracy: f64,
    /// Average confidence when using this segment
    pub average_confidence: f64,
    /// Speed improvement over character-level prediction
    pub speed_improvement: f64,
    /// Frequency of usage in successful predictions
    pub usage_frequency: usize,
    /// Stability score across different contexts
    pub context_stability: f64,
    /// Semantic coherence score
    pub semantic_coherence: f64,
    /// Last updated timestamp
    pub last_updated: u64,
}

impl Default for SegmentQualityMetrics {
    fn default() -> Self {
        Self {
            prediction_accuracy: 0.5, // Start with neutral value instead of 0.0
            average_confidence: 0.5,   // Start with neutral value instead of 0.0
            speed_improvement: 0.0,
            usage_frequency: 0,
            context_stability: 0.5,    // Start with neutral value instead of 0.0
            semantic_coherence: 0.5,   // Start with neutral value instead of 0.0
            last_updated: 0,
        }
    }
}

/// Performance metrics for comparing prediction approaches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total number of predictions made
    pub total_predictions: usize,
    /// Number of correct predictions
    pub correct_predictions: usize,
    /// Accuracy percentage (0.0-100.0)
    pub accuracy: f64,
    /// Average confidence score
    pub average_confidence: f64,
    /// Average prediction time in milliseconds
    pub average_time_ms: f64,
    /// Number of unique inputs processed
    pub unique_inputs: usize,
    /// Breakdown by input type
    pub by_input_type: HashMap<InputType, TypeMetrics>,
    /// Recent performance trend (last 100 predictions)
    pub recent_accuracy: f64,
    /// Performance improvement rate
    pub improvement_rate: f64,
    /// Adaptive learning effectiveness score
    pub learning_effectiveness: f64,
}

/// Metrics for a specific input type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeMetrics {
    pub total: usize,
    pub correct: usize,
    pub accuracy: f64,
    pub average_confidence: f64,
    pub average_time_ms: f64,
    /// Recent performance buffer for trend analysis
    pub recent_results: Vec<bool>,
    /// Context-specific performance tracking
    pub context_performance: HashMap<usize, f64>,
}

impl Default for TypeMetrics {
    fn default() -> Self {
        Self {
            total: 0,
            correct: 0,
            accuracy: 0.0,
            average_confidence: 0.0,
            average_time_ms: 0.0,
            recent_results: Vec::new(),
            context_performance: HashMap::new(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_predictions: 0,
            correct_predictions: 0,
            accuracy: 0.0,
            average_confidence: 0.0,
            average_time_ms: 0.0,
            unique_inputs: 0,
            by_input_type: HashMap::new(),
            recent_accuracy: 0.0,
            improvement_rate: 0.0,
            learning_effectiveness: 0.0,
        }
    }
}

impl PerformanceMetrics {
    /// Create new empty metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Add feedback to update metrics with advanced analytics
    pub fn add_feedback(&mut self, feedback: &PredictionFeedback) {
        self.total_predictions += 1;
        if feedback.is_correct {
            self.correct_predictions += 1;
        }
        
        // Update overall accuracy
        self.accuracy = if self.total_predictions > 0 {
            (self.correct_predictions as f64 / self.total_predictions as f64) * 100.0
        } else {
            0.0
        };
        
        // Update average confidence (running average)
        self.average_confidence = if self.total_predictions == 1 {
            feedback.confidence
        } else {
            (self.average_confidence * (self.total_predictions - 1) as f64 + feedback.confidence) 
                / self.total_predictions as f64
        };
        
        // Update average time (running average)
        self.average_time_ms = if self.total_predictions == 1 {
            feedback.prediction_time_ms as f64
        } else {
            (self.average_time_ms * (self.total_predictions - 1) as f64 + feedback.prediction_time_ms as f64) 
                / self.total_predictions as f64
        };
        
        // Update input type specific metrics with advanced tracking
        let type_metrics = self.by_input_type.entry(feedback.input_type.clone()).or_insert(TypeMetrics::default());
        
        type_metrics.total += 1;
        if feedback.is_correct {
            type_metrics.correct += 1;
        }
        
        type_metrics.accuracy = if type_metrics.total > 0 {
            (type_metrics.correct as f64 / type_metrics.total as f64) * 100.0
        } else {
            0.0
        };
        
        type_metrics.average_confidence = if type_metrics.total == 1 {
            feedback.confidence
        } else {
            (type_metrics.average_confidence * (type_metrics.total - 1) as f64 + feedback.confidence) 
                / type_metrics.total as f64
        };
        
        type_metrics.average_time_ms = if type_metrics.total == 1 {
            feedback.prediction_time_ms as f64
        } else {
            (type_metrics.average_time_ms * (type_metrics.total - 1) as f64 + feedback.prediction_time_ms as f64) 
                / type_metrics.total as f64
        };
        
        // Update recent results buffer for trend analysis
        type_metrics.recent_results.push(feedback.is_correct);
        if type_metrics.recent_results.len() > 100 {
            type_metrics.recent_results.remove(0);
        }
        
        // Update context-specific performance
        let context_accuracy = type_metrics.context_performance
            .entry(feedback.context_length)
            .or_insert(0.0);
        *context_accuracy = (*context_accuracy + if feedback.is_correct { 1.0 } else { 0.0 }) / 2.0;
        
        // Update recent accuracy trend
        self.update_recent_accuracy();
        
        // Calculate improvement rate
        self.calculate_improvement_rate();
        
        // Update learning effectiveness
        self.update_learning_effectiveness();
    }
    
    /// Update recent accuracy based on last 100 predictions
    fn update_recent_accuracy(&mut self) {
        let mut recent_correct = 0;
        let mut recent_total = 0;
        
        for metrics in self.by_input_type.values() {
            let correct_count = metrics.recent_results.iter().filter(|&&x| x).count();
            recent_correct += correct_count;
            recent_total += metrics.recent_results.len();
        }
        
        self.recent_accuracy = if recent_total > 0 {
            (recent_correct as f64 / recent_total as f64) * 100.0
        } else {
            0.0
        };
    }
    
    /// Calculate improvement rate over time
    fn calculate_improvement_rate(&mut self) {
        if self.total_predictions < 50 {
            self.improvement_rate = 0.0;
            return;
        }
        
        // Compare recent performance to early performance
        let early_accuracy = if self.total_predictions > 100 {
            // Calculate accuracy of first 50 predictions
            let mut early_correct = 0;
            for metrics in self.by_input_type.values() {
                if metrics.recent_results.len() >= 50 {
                    early_correct += metrics.recent_results[0..50].iter().filter(|&&x| x).count();
                }
            }
            (early_correct as f64 / 50.0) * 100.0
        } else {
            self.accuracy
        };
        
        self.improvement_rate = self.recent_accuracy - early_accuracy;
    }
    
    /// Update learning effectiveness score
    fn update_learning_effectiveness(&mut self) {
        // Learning effectiveness combines improvement rate and recent accuracy
        let accuracy_factor = self.recent_accuracy / 100.0;
        let improvement_factor = (self.improvement_rate + 50.0) / 100.0; // Normalize to 0-1
        
        self.learning_effectiveness = (accuracy_factor * 0.7 + improvement_factor * 0.3).max(0.0).min(1.0);
    }
    
    /// Get accuracy for a specific input type
    pub fn get_accuracy_for_type(&self, input_type: &InputType) -> f64 {
        self.by_input_type.get(input_type)
            .map(|metrics| metrics.accuracy)
            .unwrap_or(0.0)
    }
    
    /// Get recent accuracy for a specific input type
    pub fn get_recent_accuracy_for_type(&self, input_type: &InputType) -> f64 {
        self.by_input_type.get(input_type)
            .map(|metrics| {
                if metrics.recent_results.is_empty() {
                    return 0.0;
                }
                let correct = metrics.recent_results.iter().filter(|&&x| x).count();
                (correct as f64 / metrics.recent_results.len() as f64) * 100.0
            })
            .unwrap_or(0.0)
    }
    
    /// Get optimal context length for an input type
    pub fn get_optimal_context_length(&self, input_type: &InputType) -> Option<usize> {
        self.by_input_type.get(input_type)
            .and_then(|metrics| {
                metrics.context_performance
                    .iter()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(context_len, _)| *context_len)
            })
    }
    
    /// Compare performance between character and segment predictions
    pub fn compare_performance(&self) -> PerformanceComparison {
        let char_metrics = self.by_input_type.get(&InputType::Character);
        let segment_metrics = self.by_input_type.get(&InputType::Segment);
        let hybrid_metrics = self.by_input_type.get(&InputType::Hybrid);
        
        let character_accuracy = char_metrics.map(|m| m.accuracy).unwrap_or(0.0);
        let segment_accuracy = segment_metrics.map(|m| m.accuracy).unwrap_or(0.0);
        let hybrid_accuracy = hybrid_metrics.map(|m| m.accuracy).unwrap_or(0.0);
        
        // Calculate recommended mode directly to avoid circular dependency
        let recommended_mode = if self.total_predictions < 20 {
            PredictionMode::Adaptive
        } else if hybrid_accuracy > character_accuracy && hybrid_accuracy > segment_accuracy {
            PredictionMode::Hybrid
        } else if segment_accuracy > character_accuracy + 5.0 {
            PredictionMode::SegmentOnly
        } else if character_accuracy > segment_accuracy + 5.0 {
            PredictionMode::CharacterOnly
        } else {
            PredictionMode::Adaptive
        };
        
        PerformanceComparison {
            character_accuracy,
            segment_accuracy,
            hybrid_accuracy,
            character_avg_time: char_metrics.map(|m| m.average_time_ms).unwrap_or(0.0),
            segment_avg_time: segment_metrics.map(|m| m.average_time_ms).unwrap_or(0.0),
            hybrid_avg_time: hybrid_metrics.map(|m| m.average_time_ms).unwrap_or(0.0),
            character_count: char_metrics.map(|m| m.total).unwrap_or(0),
            segment_count: segment_metrics.map(|m| m.total).unwrap_or(0),
            hybrid_count: hybrid_metrics.map(|m| m.total).unwrap_or(0),
            segment_advantage: segment_accuracy - character_accuracy,
            hybrid_advantage: hybrid_accuracy - character_accuracy,
            recommended_mode,
        }
    }
    
    /// Get recommended prediction mode based on performance
    pub fn get_recommended_prediction_mode(&self) -> PredictionMode {
        let comparison = self.compare_performance();
        
        // Need minimum samples to make reliable recommendations
        if self.total_predictions < 20 {
            return PredictionMode::Adaptive;
        }
        
        // Choose the mode with highest accuracy
        if comparison.hybrid_accuracy > comparison.character_accuracy && 
           comparison.hybrid_accuracy > comparison.segment_accuracy && 
           comparison.hybrid_count >= 10 {
            PredictionMode::Hybrid
        } else if comparison.segment_accuracy > comparison.character_accuracy && 
                  comparison.segment_count >= 10 {
            PredictionMode::SegmentOnly
        } else if comparison.character_count >= 10 {
            PredictionMode::CharacterOnly
        } else {
            PredictionMode::Adaptive
        }
    }
}

/// Comparison between different prediction modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub character_accuracy: f64,
    pub segment_accuracy: f64,
    pub hybrid_accuracy: f64,
    pub character_avg_time: f64,
    pub segment_avg_time: f64,
    pub hybrid_avg_time: f64,
    pub character_count: usize,
    pub segment_count: usize,
    pub hybrid_count: usize,
    /// Positive if segments perform better, negative if characters perform better
    pub segment_advantage: f64,
    /// Positive if hybrid performs better, negative if characters perform better
    pub hybrid_advantage: f64,
    /// AI-recommended prediction mode based on performance
    pub recommended_mode: PredictionMode,
}

/// Trait for providing prediction feedback to improve segmentation
pub trait PredictionFeedbackTrait {
    /// Report the accuracy of a prediction for a given input
    fn report_prediction(&mut self, feedback: crate::integration::PredictionFeedback) -> Result<()>;
    
    /// Get performance metrics for analysis
    fn get_performance_metrics(&self) -> &PerformanceMetrics;
    
    /// Reset performance tracking
    fn reset_metrics(&mut self);
    
    /// Get segments that are performing well for predictions
    fn get_high_performing_segments(&self, min_accuracy: f64) -> Vec<String>;
    
    /// Get segments that are performing poorly for predictions
    fn get_low_performing_segments(&self, max_accuracy: f64) -> Vec<String>;
}

/// Trait for segment-aware prediction capabilities
pub trait SegmentAwarePredictor {
    /// Predict the next character given character-level input
    fn predict_next_char(&mut self, input: &str) -> Result<(char, f64)>;
    
    /// Predict the next segment given segment-level input
    fn predict_next_segment(&mut self, segments: &[String]) -> Result<(String, f64)>;
    
    /// Predict using both character and segment context
    fn predict_hybrid(&mut self, char_context: &str, segment_context: &[String]) -> Result<(String, f64)>;
    
    /// Set the segmenter to use for segment-aware predictions
    fn set_segmenter(&mut self, segmenter: Box<dyn SegmentProvider>);
    
    /// Get the current prediction mode
    fn get_prediction_mode(&self) -> PredictionMode;
    
    /// Set the prediction mode
    fn set_prediction_mode(&mut self, mode: PredictionMode);
}

/// Prediction mode for the integrated system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionMode {
    /// Use only character-level predictions
    CharacterOnly,
    /// Use only segment-level predictions
    SegmentOnly,
    /// Use hybrid approach with both character and segment context
    Hybrid,
    /// Adaptive mode that switches based on performance
    Adaptive,
}

/// Trait for providing segments to the predictor
pub trait SegmentProvider: std::fmt::Debug + Send + Sync {
    /// Get the current vocabulary of segments
    fn get_segments(&self) -> Vec<String>;
    
    /// Segment a given text into segments
    fn segment_text(&self, text: &str) -> Vec<String>;
    
    /// Get statistics for a specific segment
    fn get_segment_stats(&self, segment: &str) -> Option<crate::segment_discovery::SegmentStats>;
    
    /// Get high-confidence segments
    fn get_high_confidence_segments(&self) -> Vec<String>;
}

/// Trait for tracking and analyzing performance metrics
pub trait PerformanceTracker {
    /// Record a prediction attempt and its outcome
    fn track_prediction(&mut self, feedback: crate::integration::PredictionFeedback) -> Result<()>;
    
    /// Get current performance metrics
    fn get_metrics(&self) -> &PerformanceMetrics;
    
    /// Get performance comparison between different input types
    fn get_performance_comparison(&self) -> PerformanceComparison;
    
    /// Export metrics to JSON for analysis
    fn export_metrics(&self) -> Result<String>;
    
    /// Import metrics from JSON
    fn import_metrics(&mut self, json_data: &str) -> Result<()>;
}

/// Enhanced adaptive segment selector with machine learning capabilities
pub struct AdaptiveSegmentSelector {
    /// Performance metrics for different segments
    segment_performance: HashMap<String, PerformanceMetrics>,
    /// Advanced segment quality tracking
    segment_quality: HashMap<String, SegmentQualityMetrics>,
    /// Minimum number of samples required before trusting performance metrics
    min_samples: usize,
    /// Performance threshold for considering a segment "good"
    performance_threshold: f64,
    /// Learning rate for adaptive adjustments
    learning_rate: f64,
    /// Context-aware segment recommendations
    context_segment_map: HashMap<String, Vec<String>>,
    /// Segment usage frequency tracking
    usage_frequency: HashMap<String, usize>,
    /// Last update timestamp
    last_updated: u64,
}

impl AdaptiveSegmentSelector {
    /// Create a new adaptive segment selector with enhanced learning capabilities
    pub fn new(min_samples: usize, performance_threshold: f64) -> Self {
        Self {
            segment_performance: HashMap::new(),
            segment_quality: HashMap::new(),
            min_samples,
            performance_threshold,
            learning_rate: 0.1,
            context_segment_map: HashMap::new(),
            usage_frequency: HashMap::new(),
            last_updated: current_timestamp(),
        }
    }
    
    /// Update segment performance with advanced quality assessment
    pub fn update_segment_performance(&mut self, segment: &str, feedback: &crate::integration::PredictionFeedback) {
        // Update basic performance metrics
        let metrics = self.segment_performance.entry(segment.to_string()).or_insert(PerformanceMetrics::new());
        metrics.add_feedback(feedback);
        
        // Update advanced quality metrics
        {
            let quality = self.segment_quality.entry(segment.to_string()).or_insert(SegmentQualityMetrics::default());
            Self::update_segment_quality_static(quality, feedback, self.learning_rate);
        }
        
        // Update usage frequency
        *self.usage_frequency.entry(segment.to_string()).or_insert(0) += 1;
        
        // Update context mapping
        self.update_context_mapping(segment, feedback);
        
        self.last_updated = current_timestamp();
    }
    
    /// Update advanced segment quality metrics
    fn update_segment_quality_static(quality: &mut SegmentQualityMetrics, feedback: &PredictionFeedback, learning_rate: f64) {
        // Update prediction accuracy with learning rate
        let new_accuracy = if feedback.is_correct { 1.0 } else { 0.0 };
        quality.prediction_accuracy = quality.prediction_accuracy * (1.0 - learning_rate) + 
                                    new_accuracy * learning_rate;
        
        // Update average confidence
        quality.average_confidence = quality.average_confidence * (1.0 - learning_rate) + 
                                   feedback.confidence * learning_rate;
        
        // Update speed improvement (comparison to character-level baseline)
        let baseline_time = 50.0; // Assumed character-level baseline in ms
        let speed_improvement = (baseline_time - feedback.prediction_time_ms as f64) / baseline_time;
        quality.speed_improvement = quality.speed_improvement * (1.0 - learning_rate) + 
                                  speed_improvement * learning_rate;
        
        // Update usage frequency
        quality.usage_frequency += 1;
        
        // Update semantic coherence based on segment quality score
        if let Some(seg_quality) = feedback.segment_quality {
            quality.semantic_coherence = quality.semantic_coherence * (1.0 - learning_rate) + 
                                       seg_quality * learning_rate;
        }
        
        quality.last_updated = current_timestamp();
    }
    
    /// Update context-aware segment mapping
    fn update_context_mapping(&mut self, segment: &str, feedback: &PredictionFeedback) {
        let context_key = if feedback.context_length > 0 {
            format!("ctx_{}", feedback.context_length)
        } else {
            "default".to_string()
        };
        
        let segments = self.context_segment_map.entry(context_key).or_insert(Vec::new());
        
        // Add segment if it performed well and isn't already in the list
        if feedback.is_correct && feedback.confidence > 0.7 && !segments.contains(&segment.to_string()) {
            segments.push(segment.to_string());
            
            // Keep only top performing segments (limit to 20 per context)
            if segments.len() > 20 {
                segments.truncate(20);
            }
        }
    }
    
    /// Get best segments with advanced scoring and threshold filtering
    pub fn get_best_segments(&self, max_count: usize) -> Vec<String> {
        let mut scored_segments: Vec<(String, f64)> = self.segment_quality
            .iter()
            .filter(|(segment, quality)| {
                let metrics = self.segment_performance.get(*segment);
                metrics.map_or(false, |m| m.total_predictions >= self.min_samples) &&
                quality.usage_frequency >= self.min_samples &&
                quality.prediction_accuracy >= self.performance_threshold / 100.0
            })
            .map(|(segment, quality)| {
                let composite_score = self.calculate_composite_score(segment, quality);
                (segment.clone(), composite_score)
            })
            .collect();
        
        scored_segments.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored_segments.into_iter()
            .take(max_count)
            .map(|(segment, _)| segment)
            .collect()
    }
    
    /// Calculate composite score for segment ranking
    fn calculate_composite_score(&self, segment: &str, quality: &SegmentQualityMetrics) -> f64 {
        let _base_metrics = self.segment_performance.get(segment)
            .map(|m| m.accuracy / 100.0)
            .unwrap_or(0.0);
        
        // Weighted composite score
        let accuracy_weight = 0.4;
        let confidence_weight = 0.2;
        let speed_weight = 0.15;
        let usage_weight = 0.15;
        let coherence_weight = 0.1;
        
        let usage_score = (quality.usage_frequency as f64).ln().max(0.0) / 10.0;
        
        accuracy_weight * quality.prediction_accuracy +
        confidence_weight * quality.average_confidence +
        speed_weight * quality.speed_improvement.max(0.0) +
        usage_weight * usage_score.min(1.0) +
        coherence_weight * quality.semantic_coherence
    }
    
    /// Get context-aware segment recommendations
    pub fn get_context_segments(&self, context_length: usize) -> Vec<String> {
        let context_key = format!("ctx_{}", context_length);
        self.context_segment_map.get(&context_key)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to default context
                self.context_segment_map.get("default").cloned().unwrap_or_default()
            })
    }
    
    /// Enhanced segment recommendation based on multiple factors
    pub fn should_use_segment(&self, segment: &str) -> bool {
        if let Some(quality) = self.segment_quality.get(segment) {
            if let Some(metrics) = self.segment_performance.get(segment) {
                return metrics.total_predictions >= self.min_samples &&
                       quality.prediction_accuracy >= self.performance_threshold / 100.0 &&
                       quality.usage_frequency >= self.min_samples;
            }
        }
        false
    }
    
    /// Get detailed segment analysis
    pub fn get_segment_analysis(&self, segment: &str) -> Option<SegmentAnalysis> {
        let metrics = self.segment_performance.get(segment)?;
        let quality = self.segment_quality.get(segment)?;
        
        Some(SegmentAnalysis {
            segment: segment.to_string(),
            overall_score: self.calculate_composite_score(segment, quality),
            performance_metrics: metrics.clone(),
            quality_metrics: quality.clone(),
            recommendation: self.should_use_segment(segment),
            confidence_level: if metrics.total_predictions >= self.min_samples * 2 {
                "High"
            } else if metrics.total_predictions >= self.min_samples {
                "Medium"
            } else {
                "Low"
            }.to_string(),
        })
    }
    
    /// Get segment metrics for inspection
    pub fn get_segment_metrics(&self, segment: &str) -> Option<&PerformanceMetrics> {
        self.segment_performance.get(segment)
    }
    
    /// Reset all metrics (for debugging/testing)
    pub fn reset_all_metrics(&mut self) {
        self.segment_performance.clear();
        self.segment_quality.clear();
        self.context_segment_map.clear();
        self.usage_frequency.clear();
        self.last_updated = current_timestamp();
    }
    
    /// Export analytics data for analysis
    pub fn export_analytics(&self) -> Result<String> {
        let analytics = SegmentAnalytics {
            total_segments_tracked: self.segment_performance.len(),
            high_performing_segments: self.get_best_segments(10),
            context_mappings: self.context_segment_map.clone(),
            average_performance: self.calculate_average_performance(),
            last_updated: self.last_updated,
        };
        
        serde_json::to_string_pretty(&analytics).map_err(|e| crate::error::BrainError::from(e))
    }
    
    /// Calculate average performance across all segments
    fn calculate_average_performance(&self) -> f64 {
        if self.segment_performance.is_empty() {
            return 0.0;
        }
        
        let total_accuracy: f64 = self.segment_performance.values()
            .map(|m| m.accuracy)
            .sum();
        
        total_accuracy / self.segment_performance.len() as f64
    }
}

/// Detailed analysis of a specific segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentAnalysis {
    pub segment: String,
    pub overall_score: f64,
    pub performance_metrics: PerformanceMetrics,
    pub quality_metrics: SegmentQualityMetrics,
    pub recommendation: bool,
    pub confidence_level: String,
}

/// Analytics data for segment performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentAnalytics {
    pub total_segments_tracked: usize,
    pub high_performing_segments: Vec<String>,
    pub context_mappings: HashMap<String, Vec<String>>,
    pub average_performance: f64,
    pub last_updated: u64,
}

/// Get current timestamp in seconds since Unix epoch
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Advanced integration manager with intelligent adaptive learning
pub struct IntegrationManager {
    /// Performance tracker for the integrated system
    performance_tracker: PerformanceMetrics,
    /// Adaptive segment selector with machine learning capabilities
    segment_selector: AdaptiveSegmentSelector,
    /// Current prediction mode
    prediction_mode: PredictionMode,
    /// Mode switching configuration
    mode_switching_config: ModeSwitchingConfig,
    /// Historical performance data for trend analysis
    performance_history: Vec<PerformanceSnapshot>,
    /// Adaptive learning parameters
    learning_config: AdaptiveLearningConfig,
    /// Integration statistics
    integration_stats: IntegrationStats,
}

/// Configuration for intelligent mode switching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSwitchingConfig {
    /// Minimum predictions before considering mode switch
    pub min_predictions_for_switch: usize,
    /// Accuracy threshold difference to trigger mode switch
    pub accuracy_threshold_diff: f64,
    /// Confidence threshold for mode recommendations
    pub confidence_threshold: f64,
    /// Performance degradation tolerance before switching
    pub degradation_tolerance: f64,
    /// Enable automatic mode switching
    pub enable_auto_switching: bool,
}

impl Default for ModeSwitchingConfig {
    fn default() -> Self {
        Self {
            min_predictions_for_switch: 50,
            accuracy_threshold_diff: 5.0, // 5% difference
            confidence_threshold: 0.7,
            degradation_tolerance: 10.0, // 10% degradation
            enable_auto_switching: true,
        }
    }
}

/// Configuration for adaptive learning system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveLearningConfig {
    /// Learning rate for performance adaptation
    pub learning_rate: f64,
    /// Memory size for performance history
    pub history_size: usize,
    /// Threshold for significant performance change
    pub significance_threshold: f64,
    /// Enable context-aware learning
    pub enable_context_learning: bool,
    /// Enable segment quality assessment
    pub enable_quality_assessment: bool,
}

impl Default for AdaptiveLearningConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.1,
            history_size: 1000,
            significance_threshold: 3.0, // 3% change
            enable_context_learning: true,
            enable_quality_assessment: true,
        }
    }
}

/// Performance snapshot for historical tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub timestamp: u64,
    pub accuracy: f64,
    pub prediction_mode: PredictionMode,
    pub average_confidence: f64,
    pub average_time_ms: f64,
    pub total_predictions: usize,
}

/// Integration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStats {
    pub total_mode_switches: usize,
    pub successful_adaptations: usize,
    pub failed_adaptations: usize,
    pub best_performing_mode: PredictionMode,
    pub adaptive_learning_score: f64,
    pub last_optimization: u64,
}

impl Default for IntegrationStats {
    fn default() -> Self {
        Self {
            total_mode_switches: 0,
            successful_adaptations: 0,
            failed_adaptations: 0,
            best_performing_mode: PredictionMode::Adaptive,
            adaptive_learning_score: 0.0,
            last_optimization: current_timestamp(),
        }
    }
}

impl IntegrationManager {
    /// Create a new integration manager with advanced capabilities
    pub fn new() -> Self {
        Self {
            performance_tracker: PerformanceMetrics::new(),
            segment_selector: AdaptiveSegmentSelector::new(10, 0.75),
            prediction_mode: PredictionMode::Adaptive,
            mode_switching_config: ModeSwitchingConfig::default(),
            performance_history: Vec::new(),
            learning_config: AdaptiveLearningConfig::default(),
            integration_stats: IntegrationStats::default(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(
        mode_config: ModeSwitchingConfig,
        learning_config: AdaptiveLearningConfig,
    ) -> Self {
        Self {
            performance_tracker: PerformanceMetrics::new(),
            segment_selector: AdaptiveSegmentSelector::new(10, 0.75),
            prediction_mode: PredictionMode::Adaptive,
            mode_switching_config: mode_config,
            performance_history: Vec::new(),
            learning_config,
            integration_stats: IntegrationStats::default(),
        }
    }

    /// Update system with prediction feedback and perform adaptive learning
    pub fn update_with_feedback(&mut self, feedback: crate::integration::PredictionFeedback) -> Result<()> {
        // Update performance tracking
        self.performance_tracker.add_feedback(&feedback);
        
        // Update segment-specific performance if applicable
        if matches!(feedback.input_type, InputType::Segment | InputType::Hybrid) {
            self.segment_selector.update_segment_performance(&feedback.input, &feedback);
        }
        
        // Create performance snapshot
        self.create_performance_snapshot();
        
        // Perform adaptive learning
        self.perform_adaptive_learning(&feedback)?;
        
        // Check for mode switching opportunities
        if self.mode_switching_config.enable_auto_switching {
            self.consider_mode_switch()?;
        }
        
        // Update integration statistics
        self.update_integration_stats();
        
        Ok(())
    }
    
    /// Create a performance snapshot for historical tracking
    fn create_performance_snapshot(&mut self) {
        let snapshot = PerformanceSnapshot {
            timestamp: current_timestamp(),
            accuracy: self.performance_tracker.accuracy,
            prediction_mode: self.prediction_mode.clone(),
            average_confidence: self.performance_tracker.average_confidence,
            average_time_ms: self.performance_tracker.average_time_ms,
            total_predictions: self.performance_tracker.total_predictions,
        };
        
        self.performance_history.push(snapshot);
        
        // Maintain history size limit
        if self.performance_history.len() > self.learning_config.history_size {
            self.performance_history.remove(0);
        }
    }
    
    /// Perform adaptive learning based on recent performance
    fn perform_adaptive_learning(&mut self, feedback: &PredictionFeedback) -> Result<()> {
        // Context-aware learning
        if self.learning_config.enable_context_learning {
            self.update_context_learning(feedback);
        }
        
        // Quality assessment learning
        if self.learning_config.enable_quality_assessment {
            self.update_quality_assessment(feedback);
        }
        
        // Performance trend analysis
        self.analyze_performance_trends()?;
        
        Ok(())
    }
    
    /// Update context-aware learning
    fn update_context_learning(&mut self, feedback: &PredictionFeedback) {
        // Analyze context patterns and update recommendations
        let _context_key = format!("ctx_{}", feedback.context_length);
        
        // This would integrate with the segment selector's context mapping
        // Already handled in segment_selector.update_context_mapping()
    }
    
    /// Update quality assessment learning
    fn update_quality_assessment(&mut self, feedback: &PredictionFeedback) {
        // Calculate segment quality score if not provided
        if feedback.segment_quality.is_none() && matches!(feedback.input_type, InputType::Segment) {
            let _estimated_quality = self.estimate_segment_quality(feedback);
            // Would need to create a new feedback with quality score
            // This is a simplified approach - in practice, would maintain internal quality tracking
        }
    }
    
    /// Estimate segment quality based on performance indicators
    fn estimate_segment_quality(&self, feedback: &PredictionFeedback) -> f64 {
        let base_score = if feedback.is_correct { 0.8 } else { 0.2 };
        let confidence_bonus = feedback.confidence * 0.2;
        let speed_bonus = if feedback.prediction_time_ms < 30 { 0.1 } else { 0.0 };
        
        (base_score + confidence_bonus + speed_bonus).min(1.0)
    }
    
    /// Analyze performance trends for adaptive adjustments
    fn analyze_performance_trends(&mut self) -> Result<()> {
        if self.performance_history.len() < 10 {
            return Ok(());
        }
        
        let recent_performance = self.calculate_recent_trend();
        let historical_performance = self.calculate_historical_average();
        
        let performance_change = recent_performance - historical_performance;
        
        // Significant improvement or degradation
        if performance_change.abs() > self.learning_config.significance_threshold {
            if performance_change > 0.0 {
                self.integration_stats.successful_adaptations += 1;
            } else {
                self.integration_stats.failed_adaptations += 1;
            }
            
            // Update adaptive learning score
            self.integration_stats.adaptive_learning_score = 
                (self.integration_stats.successful_adaptations as f64) / 
                (self.integration_stats.successful_adaptations + self.integration_stats.failed_adaptations).max(1) as f64;
        }
        
        Ok(())
    }
    
    /// Calculate recent performance trend
    fn calculate_recent_trend(&self) -> f64 {
        if self.performance_history.len() < 5 {
            return 0.0;
        }
        
        let recent_count = (self.performance_history.len() / 4).max(5);
        let recent_snapshots = &self.performance_history[self.performance_history.len() - recent_count..];
        
        recent_snapshots.iter().map(|s| s.accuracy).sum::<f64>() / recent_count as f64
    }
    
    /// Calculate historical average performance
    fn calculate_historical_average(&self) -> f64 {
        if self.performance_history.is_empty() {
            return 0.0;
        }
        
        self.performance_history.iter().map(|s| s.accuracy).sum::<f64>() / self.performance_history.len() as f64
    }
    
    /// Consider switching prediction mode based on performance
    fn consider_mode_switch(&mut self) -> Result<()> {
        if self.performance_tracker.total_predictions < self.mode_switching_config.min_predictions_for_switch {
            return Ok(());
        }
        
        let comparison = self.performance_tracker.compare_performance();
        let recommended_mode = comparison.recommended_mode;
        
        // Check if we should switch modes
        if recommended_mode != self.prediction_mode {
            let current_accuracy = match self.prediction_mode {
                PredictionMode::CharacterOnly => comparison.character_accuracy,
                PredictionMode::SegmentOnly => comparison.segment_accuracy,
                PredictionMode::Hybrid => comparison.hybrid_accuracy,
                PredictionMode::Adaptive => comparison.character_accuracy.max(comparison.segment_accuracy).max(comparison.hybrid_accuracy),
            };
            
            let recommended_accuracy = match recommended_mode {
                PredictionMode::CharacterOnly => comparison.character_accuracy,
                PredictionMode::SegmentOnly => comparison.segment_accuracy,
                PredictionMode::Hybrid => comparison.hybrid_accuracy,
                PredictionMode::Adaptive => comparison.character_accuracy.max(comparison.segment_accuracy).max(comparison.hybrid_accuracy),
            };
            
            // Switch if recommended mode shows significant improvement
            if recommended_accuracy - current_accuracy > self.mode_switching_config.accuracy_threshold_diff {
                self.prediction_mode = recommended_mode;
                self.integration_stats.total_mode_switches += 1;
                
                // Update best performing mode
                if recommended_accuracy > match self.integration_stats.best_performing_mode {
                    PredictionMode::CharacterOnly => comparison.character_accuracy,
                    PredictionMode::SegmentOnly => comparison.segment_accuracy,
                    PredictionMode::Hybrid => comparison.hybrid_accuracy,
                    PredictionMode::Adaptive => comparison.character_accuracy.max(comparison.segment_accuracy).max(comparison.hybrid_accuracy),
                } {
                    self.integration_stats.best_performing_mode = recommended_mode;
                }
            }
        }
        
        Ok(())
    }
    
    /// Update integration statistics
    fn update_integration_stats(&mut self) {
        self.integration_stats.last_optimization = current_timestamp();
    }

    /// Get optimal prediction mode with advanced analytics
    pub fn get_optimal_prediction_mode(&self) -> PredictionMode {
        self.prediction_mode.clone()
    }
    
    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_tracker
    }
    
    /// Get segment selector
    pub fn get_segment_selector(&self) -> &AdaptiveSegmentSelector {
        &self.segment_selector
    }
    
    /// Get detailed integration analysis
    pub fn get_integration_analysis(&self) -> IntegrationAnalysis {
        let comparison = self.performance_tracker.compare_performance();
        let recommended_mode = comparison.recommended_mode;
        
        IntegrationAnalysis {
            current_mode: self.prediction_mode.clone(),
            recommended_mode,
            performance_comparison: comparison,
            integration_stats: self.integration_stats.clone(),
            best_segments: self.segment_selector.get_best_segments(10),
            learning_effectiveness: self.performance_tracker.learning_effectiveness,
            total_adaptations: self.integration_stats.successful_adaptations + self.integration_stats.failed_adaptations,
            adaptation_success_rate: if self.integration_stats.successful_adaptations + self.integration_stats.failed_adaptations > 0 {
                self.integration_stats.successful_adaptations as f64 / 
                (self.integration_stats.successful_adaptations + self.integration_stats.failed_adaptations) as f64
            } else {
                0.0
            },
        }
    }
    
    /// Export comprehensive analytics
    pub fn export_full_analytics(&self) -> Result<String> {
        let analytics = ComprehensiveAnalytics {
            integration_analysis: self.get_integration_analysis(),
            performance_history: self.performance_history.clone(),
            segment_analytics: self.segment_selector.export_analytics()?,
            configuration: AnalyticsConfiguration {
                mode_switching: self.mode_switching_config.clone(),
                learning: self.learning_config.clone(),
            },
            timestamp: current_timestamp(),
        };
        
        serde_json::to_string_pretty(&analytics).map_err(|e| crate::error::BrainError::from(e))
    }
    
    /// Reset all metrics and learning state
    pub fn reset_learning_state(&mut self) {
        self.performance_tracker = PerformanceMetrics::new();
        self.segment_selector.reset_all_metrics();
        self.performance_history.clear();
        self.integration_stats = IntegrationStats::default();
        self.prediction_mode = PredictionMode::Adaptive;
    }
}

/// Comprehensive integration analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAnalysis {
    pub current_mode: PredictionMode,
    pub recommended_mode: PredictionMode,
    pub performance_comparison: PerformanceComparison,
    pub integration_stats: IntegrationStats,
    pub best_segments: Vec<String>,
    pub learning_effectiveness: f64,
    pub total_adaptations: usize,
    pub adaptation_success_rate: f64,
}

/// Comprehensive analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveAnalytics {
    pub integration_analysis: IntegrationAnalysis,
    pub performance_history: Vec<PerformanceSnapshot>,
    pub segment_analytics: String, // JSON string from segment selector
    pub configuration: AnalyticsConfiguration,
    pub timestamp: u64,
}

/// Analytics configuration snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfiguration {
    pub mode_switching: ModeSwitchingConfig,
    pub learning: AdaptiveLearningConfig,
}

impl Default for IntegrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();
        
        let feedback1 = crate::integration::PredictionFeedback {
            input: "a".to_string(),
            input_type: InputType::Character,
            predicted: "b".to_string(),
            actual: "b".to_string(),
            is_correct: true,
            confidence: 0.8,
            prediction_time_ms: 10,
            timestamp: 1000,
            context_length: 0,
            segment_quality: None,
        };
        
        metrics.add_feedback(&feedback1);
        assert_eq!(metrics.total_predictions, 1);
        assert_eq!(metrics.correct_predictions, 1);
        assert_eq!(metrics.accuracy, 100.0);
        assert_eq!(metrics.average_confidence, 0.8);
    }
    
    #[test]
    fn test_adaptive_segment_selector() {
        let mut selector = AdaptiveSegmentSelector::new(2, 50.0);
        
        let good_feedback = crate::integration::PredictionFeedback {
            input: "the".to_string(),
            input_type: InputType::Segment,
            predicted: "cat".to_string(),
            actual: "cat".to_string(),
            is_correct: true,
            confidence: 0.9,
            prediction_time_ms: 5,
            timestamp: 1000,
            context_length: 0,
            segment_quality: None,
        };
        
        // Add multiple good feedbacks
        selector.update_segment_performance("the", &good_feedback);
        selector.update_segment_performance("the", &good_feedback);
        
        assert!(selector.should_use_segment("the"));
        
        let best_segments = selector.get_best_segments(5);
        assert!(best_segments.contains(&"the".to_string()));
    }
    
    #[test]
    fn test_integration_manager() {
        let mut manager = IntegrationManager::new();
        
        let char_feedback = crate::integration::PredictionFeedback {
            input: "a".to_string(),
            input_type: InputType::Character,
            predicted: "b".to_string(),
            actual: "b".to_string(),
            is_correct: true,
            confidence: 0.7,
            prediction_time_ms: 15,
            timestamp: 1000,
            context_length: 0,
            segment_quality: None,
        };
        
        manager.update_with_feedback(char_feedback).unwrap();
        
        let metrics = manager.get_performance_metrics();
        assert_eq!(metrics.total_predictions, 1);
        assert_eq!(metrics.accuracy, 100.0);
    }
} 