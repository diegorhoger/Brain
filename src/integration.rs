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
}

/// Type of input used for prediction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputType {
    Character,
    Segment,
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
}

/// Metrics for a specific input type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeMetrics {
    pub total: usize,
    pub correct: usize,
    pub accuracy: f64,
    pub average_confidence: f64,
    pub average_time_ms: f64,
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
        }
    }
}

impl PerformanceMetrics {
    /// Create new empty metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Add feedback to update metrics
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
        
        // Update input type specific metrics
        let type_metrics = self.by_input_type.entry(feedback.input_type.clone()).or_insert(TypeMetrics {
            total: 0,
            correct: 0,
            accuracy: 0.0,
            average_confidence: 0.0,
            average_time_ms: 0.0,
        });
        
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
    }
    
    /// Get accuracy for a specific input type
    pub fn get_accuracy_for_type(&self, input_type: &InputType) -> f64 {
        self.by_input_type.get(input_type)
            .map(|metrics| metrics.accuracy)
            .unwrap_or(0.0)
    }
    
    /// Compare performance between character and segment predictions
    pub fn compare_performance(&self) -> PerformanceComparison {
        let char_metrics = self.by_input_type.get(&InputType::Character);
        let segment_metrics = self.by_input_type.get(&InputType::Segment);
        
        PerformanceComparison {
            character_accuracy: char_metrics.map(|m| m.accuracy).unwrap_or(0.0),
            segment_accuracy: segment_metrics.map(|m| m.accuracy).unwrap_or(0.0),
            character_avg_time: char_metrics.map(|m| m.average_time_ms).unwrap_or(0.0),
            segment_avg_time: segment_metrics.map(|m| m.average_time_ms).unwrap_or(0.0),
            character_count: char_metrics.map(|m| m.total).unwrap_or(0),
            segment_count: segment_metrics.map(|m| m.total).unwrap_or(0),
            segment_advantage: segment_metrics.map(|m| m.accuracy).unwrap_or(0.0) 
                - char_metrics.map(|m| m.accuracy).unwrap_or(0.0),
        }
    }
}

/// Comparison between character and segment prediction performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub character_accuracy: f64,
    pub segment_accuracy: f64,
    pub character_avg_time: f64,
    pub segment_avg_time: f64,
    pub character_count: usize,
    pub segment_count: usize,
    /// Positive if segments perform better, negative if characters perform better
    pub segment_advantage: f64,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub trait SegmentProvider {
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

/// Adaptive segment selector that chooses optimal segments based on performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveSegmentSelector {
    /// Performance metrics for different segments
    segment_performance: HashMap<String, PerformanceMetrics>,
    /// Minimum number of samples required before trusting performance metrics
    min_samples: usize,
    /// Performance threshold for considering a segment "good"
    performance_threshold: f64,
}

impl AdaptiveSegmentSelector {
    /// Create a new adaptive segment selector
    pub fn new(min_samples: usize, performance_threshold: f64) -> Self {
        Self {
            segment_performance: HashMap::new(),
            min_samples,
            performance_threshold,
        }
    }
    
    /// Update performance metrics for a segment
    pub fn update_segment_performance(&mut self, segment: &str, feedback: &crate::integration::PredictionFeedback) {
        let metrics = self.segment_performance.entry(segment.to_string()).or_insert_with(PerformanceMetrics::new);
        metrics.add_feedback(feedback);
    }
    
    /// Get the best performing segments
    pub fn get_best_segments(&self, max_count: usize) -> Vec<String> {
        let mut segments: Vec<_> = self.segment_performance
            .iter()
            .filter(|(_, metrics)| metrics.total_predictions >= self.min_samples)
            .filter(|(_, metrics)| metrics.accuracy >= self.performance_threshold)
            .collect();
        
        segments.sort_by(|a, b| b.1.accuracy.partial_cmp(&a.1.accuracy).unwrap_or(std::cmp::Ordering::Equal));
        
        segments.into_iter()
            .take(max_count)
            .map(|(segment, _)| segment.clone())
            .collect()
    }
    
    /// Check if a segment should be used based on its performance
    pub fn should_use_segment(&self, segment: &str) -> bool {
        self.segment_performance.get(segment)
            .map(|metrics| {
                metrics.total_predictions >= self.min_samples && 
                metrics.accuracy >= self.performance_threshold
            })
            .unwrap_or(false)
    }
    
    /// Get performance metrics for a specific segment
    pub fn get_segment_metrics(&self, segment: &str) -> Option<&PerformanceMetrics> {
        self.segment_performance.get(segment)
    }
}

/// Integration manager that coordinates between segmenter and predictor
pub struct IntegrationManager {
    /// Performance tracker for the integrated system
    performance_tracker: PerformanceMetrics,
    /// Adaptive segment selector
    segment_selector: AdaptiveSegmentSelector,
    /// Current prediction mode
    prediction_mode: PredictionMode,
}

impl IntegrationManager {
    /// Create a new integration manager
    pub fn new() -> Self {
        Self {
            performance_tracker: PerformanceMetrics::new(),
            segment_selector: AdaptiveSegmentSelector::new(10, 75.0), // 10 samples, 75% accuracy threshold
            prediction_mode: PredictionMode::Adaptive,
        }
    }
    
    /// Update the integration with new prediction feedback
    pub fn update_with_feedback(&mut self, feedback: crate::integration::PredictionFeedback) -> Result<()> {
        // Update overall performance tracking
        self.performance_tracker.add_feedback(&feedback);
        
        // Update segment-specific performance if it's a segment prediction
        if feedback.input_type == InputType::Segment {
            self.segment_selector.update_segment_performance(&feedback.input, &feedback);
        }
        
        // Adapt prediction mode based on performance
        self.adapt_prediction_mode();
        
        Ok(())
    }
    
    /// Determine the best prediction mode based on current performance
    fn adapt_prediction_mode(&mut self) {
        if self.prediction_mode != PredictionMode::Adaptive {
            return;
        }
        
        let comparison = self.performance_tracker.compare_performance();
        
        // Switch to the better performing mode if there's a significant difference
        if comparison.character_count >= 10 && comparison.segment_count >= 10 {
            if comparison.segment_advantage > 5.0 {
                self.prediction_mode = PredictionMode::SegmentOnly;
            } else if comparison.segment_advantage < -5.0 {
                self.prediction_mode = PredictionMode::CharacterOnly;
            } else {
                self.prediction_mode = PredictionMode::Hybrid;
            }
        }
    }
    
    /// Get the current optimal prediction mode
    pub fn get_optimal_prediction_mode(&self) -> PredictionMode {
        self.prediction_mode.clone()
    }
    
    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_tracker
    }
    
    /// Get the segment selector
    pub fn get_segment_selector(&self) -> &AdaptiveSegmentSelector {
        &self.segment_selector
    }
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
        };
        
        manager.update_with_feedback(char_feedback).unwrap();
        
        let metrics = manager.get_performance_metrics();
        assert_eq!(metrics.total_predictions, 1);
        assert_eq!(metrics.accuracy, 100.0);
    }
} 