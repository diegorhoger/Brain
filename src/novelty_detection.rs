//! Novelty Detection Module
//! 
//! This module implements Task 9.2: Novelty Detection Algorithms
//! 
//! The novelty detection system identifies unexpected or surprising inputs based on
//! existing knowledge distributions and confidence tracking from the meta-memory system.
//! 
//! ## Key Features:
//! - Statistical novelty detection comparing inputs against knowledge distributions
//! - Surprise metrics quantifying deviation from expected patterns
//! - Anomaly detection for identifying outlier inputs
//! - Context-based novelty assessment considering task context
//! - Novelty scoring system (0-1 range) combining multiple detection methods
//! - Integration with meta-memory system for confidence-based assessments

use anyhow::{Result, Context};
use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::meta_memory::{MetaMemorySystem, MetaMemoryQuery, KnowledgeType};

/// Configuration for novelty detection algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyDetectionConfig {
    /// Threshold for determining high novelty (0.0-1.0)
    pub high_novelty_threshold: f64,
    /// Threshold for determining low novelty (0.0-1.0)
    pub low_novelty_threshold: f64,
    /// Weight for statistical novelty component
    pub statistical_weight: f64,
    /// Weight for confidence-based novelty component
    pub confidence_weight: f64,
    /// Weight for context-based novelty component
    pub context_weight: f64,
    /// Minimum sample size for statistical calculations
    pub min_sample_size: usize,
    /// Context window size for pattern analysis
    pub context_window_size: usize,
    /// Enable logging of novelty assessments
    pub enable_logging: bool,
    /// Maximum number of novelty records to keep
    pub max_novelty_records: usize,
}

impl Default for NoveltyDetectionConfig {
    fn default() -> Self {
        Self {
            high_novelty_threshold: 0.7,
            low_novelty_threshold: 0.3,
            statistical_weight: 0.4,
            confidence_weight: 0.3,
            context_weight: 0.3,
            min_sample_size: 10,
            context_window_size: 5,
            enable_logging: true,
            max_novelty_records: 10000,
        }
    }
}

/// Types of novelty detection methods
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoveltyMethod {
    /// Statistical deviation from known distributions
    Statistical,
    /// Confidence-based assessment using meta-memory
    ConfidenceBased,
    /// Context-aware pattern matching
    ContextBased,
    /// Anomaly detection for outlier identification
    AnomalyDetection,
    /// Combined multi-method assessment
    Composite,
}

/// Context information for novelty assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyContext {
    /// Current task or domain
    pub task_context: String,
    /// Recent interaction history
    pub recent_inputs: Vec<String>,
    /// Active knowledge components
    pub active_components: Vec<Uuid>,
    /// Time-based context information
    pub temporal_context: DateTime<Utc>,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}

impl Default for NoveltyContext {
    fn default() -> Self {
        Self {
            task_context: "general".to_string(),
            recent_inputs: Vec::new(),
            active_components: Vec::new(),
            temporal_context: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

/// Result of novelty detection assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyAssessment {
    /// Unique identifier for this assessment
    pub id: Uuid,
    /// Input that was assessed
    pub input: String,
    /// Overall novelty score (0.0-1.0, higher = more novel)
    pub novelty_score: f64,
    /// Breakdown of scores by method
    pub method_scores: HashMap<NoveltyMethod, f64>,
    /// Confidence in the assessment
    pub assessment_confidence: f64,
    /// Context used for assessment
    pub context: NoveltyContext,
    /// Detailed explanation of findings
    pub explanation: Vec<String>,
    /// Timestamp of assessment
    pub assessed_at: DateTime<Utc>,
    /// Recommended actions based on novelty level
    pub recommendations: Vec<String>,
}

impl NoveltyAssessment {
    /// Create a new novelty assessment
    pub fn new(input: String, context: NoveltyContext) -> Self {
        Self {
            id: Uuid::new_v4(),
            input,
            novelty_score: 0.0,
            method_scores: HashMap::new(),
            assessment_confidence: 0.0,
            context,
            explanation: Vec::new(),
            assessed_at: Utc::now(),
            recommendations: Vec::new(),
        }
    }

    /// Add a method score to the assessment
    pub fn add_method_score(&mut self, method: NoveltyMethod, score: f64) {
        self.method_scores.insert(method, score.clamp(0.0, 1.0));
    }

    /// Add an explanation note
    pub fn add_explanation(&mut self, explanation: String) {
        self.explanation.push(explanation);
    }

    /// Add a recommendation
    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }

    /// Get novelty level classification
    pub fn get_novelty_level(&self, config: &NoveltyDetectionConfig) -> NoveltyLevel {
        if self.novelty_score >= config.high_novelty_threshold {
            NoveltyLevel::High
        } else if self.novelty_score <= config.low_novelty_threshold {
            NoveltyLevel::Low
        } else {
            NoveltyLevel::Medium
        }
    }
}

/// Novelty level classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoveltyLevel {
    /// Low novelty - familiar patterns
    Low,
    /// Medium novelty - some unexpected elements
    Medium,
    /// High novelty - highly unexpected or surprising
    High,
}

impl std::fmt::Display for NoveltyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoveltyLevel::Low => write!(f, "Low"),
            NoveltyLevel::Medium => write!(f, "Medium"),
            NoveltyLevel::High => write!(f, "High"),
        }
    }
}

/// Statistics for novelty detection system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyDetectionStats {
    /// Total assessments performed
    pub total_assessments: usize,
    /// Average novelty score
    pub average_novelty_score: f64,
    /// Distribution by novelty level
    pub novelty_distribution: HashMap<NoveltyLevel, usize>,
    /// Most common novelty methods used
    pub method_usage: HashMap<NoveltyMethod, usize>,
    /// Assessment performance metrics
    pub average_assessment_confidence: f64,
    /// Time-based statistics
    pub assessments_per_hour: f64,
    /// Context analysis
    pub common_contexts: HashMap<String, usize>,
}

impl Default for NoveltyDetectionStats {
    fn default() -> Self {
        Self {
            total_assessments: 0,
            average_novelty_score: 0.0,
            novelty_distribution: HashMap::new(),
            method_usage: HashMap::new(),
            average_assessment_confidence: 0.0,
            assessments_per_hour: 0.0,
            common_contexts: HashMap::new(),
        }
    }
}

/// Main novelty detection engine
pub struct NoveltyDetectionEngine {
    /// Configuration
    config: NoveltyDetectionConfig,
    /// Reference to meta-memory system
    meta_memory: Arc<Mutex<MetaMemorySystem>>,
    /// Historical assessments
    assessments: Vec<NoveltyAssessment>,
    /// System statistics
    stats: NoveltyDetectionStats,
    /// Knowledge type distributions for statistical analysis
    knowledge_distributions: HashMap<KnowledgeType, Vec<f64>>,
}

impl NoveltyDetectionEngine {
    /// Create a new novelty detection engine
    pub fn new(
        config: NoveltyDetectionConfig,
        meta_memory: Arc<Mutex<MetaMemorySystem>>,
    ) -> Result<Self> {
        let mut engine = Self {
            config,
            meta_memory,
            assessments: Vec::new(),
            stats: NoveltyDetectionStats::default(),
            knowledge_distributions: HashMap::new(),
        };

        // Initialize knowledge distributions from meta-memory
        engine.initialize_distributions()?;

        Ok(engine)
    }

    /// Initialize knowledge distributions from existing meta-memory data
    fn initialize_distributions(&mut self) -> Result<()> {
        let meta_memory = self.meta_memory.lock().unwrap();
        
        // Get all knowledge types and their confidence distributions
        for knowledge_type in [
            KnowledgeType::Segment,
            KnowledgeType::ConceptNode,
            KnowledgeType::Rule,
            KnowledgeType::SemanticConcept,
            KnowledgeType::WorkingMemory,
            KnowledgeType::EpisodicMemory,
            KnowledgeType::Pattern,
            KnowledgeType::ConceptRelationship,
            KnowledgeType::GeneralizedRule,
        ] {
            let query = MetaMemoryQuery {
                knowledge_type: Some(knowledge_type.clone()),
                ..Default::default()
            };
            
            let items = meta_memory.query_items(&query)?;
            let confidence_scores: Vec<f64> = items
                .iter()
                .map(|item| item.confidence_score)
                .collect();
            
            if !confidence_scores.is_empty() {
                self.knowledge_distributions.insert(knowledge_type, confidence_scores);
            }
        }

        Ok(())
    }

    /// Assess novelty of input using all available methods
    pub fn assess_novelty(
        &mut self,
        input: &str,
        context: Option<NoveltyContext>,
    ) -> Result<NoveltyAssessment> {
        let context = context.unwrap_or_default();
        let mut assessment = NoveltyAssessment::new(input.to_string(), context);

        // Apply different novelty detection methods
        let statistical_score = self.calculate_statistical_novelty(input, &assessment.context)?;
        let confidence_score = self.calculate_confidence_based_novelty(input, &assessment.context)?;
        let context_score = self.calculate_context_based_novelty(input, &assessment.context)?;
        let anomaly_score = self.calculate_anomaly_score(input, &assessment.context)?;

        // Store method scores
        assessment.add_method_score(NoveltyMethod::Statistical, statistical_score);
        assessment.add_method_score(NoveltyMethod::ConfidenceBased, confidence_score);
        assessment.add_method_score(NoveltyMethod::ContextBased, context_score);
        assessment.add_method_score(NoveltyMethod::AnomalyDetection, anomaly_score);

        // Calculate composite novelty score
        let composite_score = self.calculate_composite_score(
            statistical_score,
            confidence_score,
            context_score,
            anomaly_score,
        );
        
        assessment.add_method_score(NoveltyMethod::Composite, composite_score);
        assessment.novelty_score = composite_score;

        // Calculate assessment confidence
        assessment.assessment_confidence = self.calculate_assessment_confidence(&assessment);

        // Generate explanations and recommendations
        self.generate_explanations(&mut assessment);
        self.generate_recommendations(&mut assessment);

        // Store assessment and update statistics
        self.store_assessment(assessment.clone())?;
        self.update_statistics();

        Ok(assessment)
    }

    /// Calculate statistical novelty based on knowledge distributions
    fn calculate_statistical_novelty(&self, input: &str, _context: &NoveltyContext) -> Result<f64> {
        let mut novelty_scores = Vec::new();

        // Analyze input characteristics that can be compared to distributions
        let input_length = input.len() as f64;
        let word_count = input.split_whitespace().count() as f64;
        let unique_chars = input.chars().collect::<std::collections::HashSet<_>>().len() as f64;
        
        // For each knowledge type, calculate how unusual this input would be
        for (knowledge_type, distribution) in &self.knowledge_distributions {
            if distribution.len() < self.config.min_sample_size {
                continue;
            }

            // Calculate z-score based on confidence distribution
            let mean = distribution.iter().sum::<f64>() / distribution.len() as f64;
            let variance = distribution.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / distribution.len() as f64;
            let std_dev = variance.sqrt();

            if std_dev > 0.0 {
                // Use input characteristics as proxy for "confidence-like" metrics
                let proxy_confidence = match knowledge_type {
                    KnowledgeType::Segment => (unique_chars / input_length).min(1.0),
                    KnowledgeType::Rule => (word_count / 10.0).min(1.0),
                    _ => (input_length / 100.0).min(1.0),
                };

                let z_score = ((proxy_confidence - mean) / std_dev).abs();
                let novelty = (z_score / 3.0).min(1.0); // Normalize z-score to [0,1]
                novelty_scores.push(novelty);
            }
        }

        // Return average novelty across all knowledge types
        if novelty_scores.is_empty() {
            Ok(0.5) // Default to medium novelty if no distributions available
        } else {
            Ok(novelty_scores.iter().sum::<f64>() / novelty_scores.len() as f64)
        }
    }

    /// Calculate confidence-based novelty using meta-memory confidence levels
    fn calculate_confidence_based_novelty(&self, input: &str, _context: &NoveltyContext) -> Result<f64> {
        let meta_memory = self.meta_memory.lock().unwrap();
        
        // Query for components that might match this input
        let mut matching_confidences = Vec::new();

        // Search for similar patterns in different knowledge types
        for knowledge_type in [
            KnowledgeType::Segment,
            KnowledgeType::Rule,
            KnowledgeType::Pattern,
            KnowledgeType::SemanticConcept,
        ] {
            let query = MetaMemoryQuery {
                knowledge_type: Some(knowledge_type),
                active_only: Some(true),
                limit: Some(50),
                ..Default::default()
            };

            let items = meta_memory.query_items(&query)?;
            
            // Find items with sources that might relate to this input
            for item in items {
                let source_similarity = self.calculate_string_similarity(input, &item.source);
                if source_similarity > 0.3 {
                    // Weight confidence by similarity
                    matching_confidences.push(item.confidence_score * source_similarity);
                }
            }
        }

        // Calculate novelty as inverse of average matching confidence
        if matching_confidences.is_empty() {
            Ok(0.8) // High novelty if no matches found
        } else {
            let avg_confidence = matching_confidences.iter().sum::<f64>() / matching_confidences.len() as f64;
            Ok(1.0 - avg_confidence) // Higher novelty for lower confidence matches
        }
    }

    /// Calculate context-based novelty considering task context
    fn calculate_context_based_novelty(&self, input: &str, context: &NoveltyContext) -> Result<f64> {
        let mut novelty_factors = Vec::new();

        // Analyze recent input patterns
        if !context.recent_inputs.is_empty() {
            let mut pattern_novelty = 0.0;
            let recent_window = context.recent_inputs
                .iter()
                .rev()
                .take(self.config.context_window_size);

            for recent_input in recent_window {
                let similarity = self.calculate_string_similarity(input, recent_input);
                pattern_novelty += 1.0 - similarity; // Higher novelty for less similar inputs
            }
            
            pattern_novelty /= context.recent_inputs.len().min(self.config.context_window_size) as f64;
            novelty_factors.push(pattern_novelty);
        }

        // Analyze task context novelty
        let context_novelty = match context.task_context.as_str() {
            "general" => 0.3, // Lower novelty in general context
            "specific_domain" => 0.5, // Medium novelty in specific domains
            "new_domain" => 0.8, // Higher novelty in new domains
            _ => 0.5, // Default medium novelty
        };
        novelty_factors.push(context_novelty);

        // Temporal novelty (time-based patterns)
        let hour = context.temporal_context.hour();
        let temporal_novelty = match hour {
            9..=17 => 0.3, // Work hours - lower novelty expected
            18..=22 => 0.5, // Evening - medium novelty
            _ => 0.7, // Night/early morning - higher novelty
        };
        novelty_factors.push(temporal_novelty);

        // Return average of all factors
        Ok(novelty_factors.iter().sum::<f64>() / novelty_factors.len() as f64)
    }

    /// Calculate anomaly score for outlier detection
    fn calculate_anomaly_score(&self, input: &str, _context: &NoveltyContext) -> Result<f64> {
        let mut anomaly_indicators = Vec::new();

        // Length-based anomaly detection
        let length = input.len();
        let length_anomaly = if length < 5 || length > 1000 {
            0.8 // Very short or very long inputs are anomalous
        } else if length < 10 || length > 500 {
            0.5 // Somewhat unusual lengths
        } else {
            0.2 // Normal lengths
        };
        anomaly_indicators.push(length_anomaly);

        // Character frequency anomaly
        let char_counts: HashMap<char, usize> = input.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        
        let max_char_freq = char_counts.values().max().unwrap_or(&0);
        let char_anomaly = if *max_char_freq > length / 2 {
            0.9 // Very repetitive character usage
        } else if char_counts.len() < 3 {
            0.7 // Very limited character diversity
        } else {
            0.2 // Normal character usage
        };
        anomaly_indicators.push(char_anomaly);

        // Pattern-based anomaly (repetitive sequences)
        let repetition_anomaly = self.detect_repetitive_patterns(input);
        anomaly_indicators.push(repetition_anomaly);

        // Return maximum anomaly score (most conservative approach)
        Ok(anomaly_indicators.iter().fold(0.0, |acc, &x| acc.max(x)))
    }

    /// Calculate composite novelty score from individual method scores
    fn calculate_composite_score(
        &self,
        statistical: f64,
        confidence: f64,
        context: f64,
        anomaly: f64,
    ) -> f64 {
        let weighted_score = 
            statistical * self.config.statistical_weight +
            confidence * self.config.confidence_weight +
            context * self.config.context_weight +
            anomaly * 0.1; // Small weight for anomaly detection

        // Normalize weights
        let total_weight = self.config.statistical_weight + 
                          self.config.confidence_weight + 
                          self.config.context_weight + 0.1;

        (weighted_score / total_weight).clamp(0.0, 1.0)
    }

    /// Calculate confidence in the assessment itself
    fn calculate_assessment_confidence(&self, assessment: &NoveltyAssessment) -> f64 {
        let mut confidence_factors = Vec::new();

        // More methods used = higher confidence
        let method_count = assessment.method_scores.len() as f64;
        confidence_factors.push((method_count / 4.0).min(1.0));

        // Consistency across methods
        let scores: Vec<f64> = assessment.method_scores.values().cloned().collect();
        if scores.len() > 1 {
            let mean = scores.iter().sum::<f64>() / scores.len() as f64;
            let variance = scores.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / scores.len() as f64;
            let consistency = 1.0 - variance.sqrt().min(1.0);
            confidence_factors.push(consistency);
        }

        // Availability of meta-memory data
        let data_availability = if !self.knowledge_distributions.is_empty() {
            0.8
        } else {
            0.4
        };
        confidence_factors.push(data_availability);

        confidence_factors.iter().sum::<f64>() / confidence_factors.len() as f64
    }

    /// Generate explanations for the novelty assessment
    fn generate_explanations(&self, assessment: &mut NoveltyAssessment) {
        let novelty_level = assessment.get_novelty_level(&self.config);

        assessment.add_explanation(format!(
            "Overall novelty score: {:.3} ({})",
            assessment.novelty_score,
            novelty_level
        ));

        // Clone method scores to avoid borrow issues
        let method_scores = assessment.method_scores.clone();

        // Explain method contributions
        for (method, score) in &method_scores {
            let contribution = match method {
                NoveltyMethod::Statistical => "based on deviation from known knowledge distributions",
                NoveltyMethod::ConfidenceBased => "based on confidence levels of similar known patterns",
                NoveltyMethod::ContextBased => "based on context and recent interaction patterns",
                NoveltyMethod::AnomalyDetection => "based on detection of unusual input characteristics",
                NoveltyMethod::Composite => "combined across all detection methods",
            };
            
            assessment.add_explanation(format!(
                "{:?} novelty: {:.3} ({})",
                method, score, contribution
            ));
        }

        // Add context-specific explanations
        let recent_inputs_len = assessment.context.recent_inputs.len();
        let task_context = assessment.context.task_context.clone();
        if recent_inputs_len > 0 {
            assessment.add_explanation(format!(
                "Analyzed against {} recent inputs in '{}' context",
                recent_inputs_len,
                task_context
            ));
        }
    }

    /// Generate recommendations based on novelty level
    fn generate_recommendations(&self, assessment: &mut NoveltyAssessment) {
        let novelty_level = assessment.get_novelty_level(&self.config);

        match novelty_level {
            NoveltyLevel::High => {
                assessment.add_recommendation("High novelty detected - prioritize for learning".to_string());
                assessment.add_recommendation("Consider expanding knowledge base with this pattern".to_string());
                assessment.add_recommendation("Increase validation efforts for related concepts".to_string());
            }
            NoveltyLevel::Medium => {
                assessment.add_recommendation("Medium novelty - good candidate for exploration".to_string());
                assessment.add_recommendation("Monitor for related patterns to build confidence".to_string());
            }
            NoveltyLevel::Low => {
                assessment.add_recommendation("Low novelty - familiar pattern detected".to_string());
                assessment.add_recommendation("Can proceed with standard processing".to_string());
            }
        }

        // Method-specific recommendations
        if let Some(&anomaly_score) = assessment.method_scores.get(&NoveltyMethod::AnomalyDetection) {
            if anomaly_score > 0.7 {
                assessment.add_recommendation("Anomaly detected - verify input validity".to_string());
            }
        }

        if assessment.assessment_confidence < 0.5 {
            assessment.add_recommendation("Low assessment confidence - gather more data".to_string());
        }
    }

    /// Store assessment and manage history
    fn store_assessment(&mut self, assessment: NoveltyAssessment) -> Result<()> {
        self.assessments.push(assessment);

        // Maintain maximum history size
        if self.assessments.len() > self.config.max_novelty_records {
            let excess = self.assessments.len() - self.config.max_novelty_records;
            self.assessments.drain(0..excess);
        }

        Ok(())
    }

    /// Update system statistics
    fn update_statistics(&mut self) {
        self.stats.total_assessments = self.assessments.len();

        if !self.assessments.is_empty() {
            // Calculate average novelty score
            self.stats.average_novelty_score = self.assessments
                .iter()
                .map(|a| a.novelty_score)
                .sum::<f64>() / self.assessments.len() as f64;

            // Update novelty distribution
            self.stats.novelty_distribution.clear();
            for assessment in &self.assessments {
                let level = assessment.get_novelty_level(&self.config);
                *self.stats.novelty_distribution.entry(level).or_insert(0) += 1;
            }

            // Update method usage
            self.stats.method_usage.clear();
            for assessment in &self.assessments {
                for method in assessment.method_scores.keys() {
                    *self.stats.method_usage.entry(method.clone()).or_insert(0) += 1;
                }
            }

            // Calculate average assessment confidence
            self.stats.average_assessment_confidence = self.assessments
                .iter()
                .map(|a| a.assessment_confidence)
                .sum::<f64>() / self.assessments.len() as f64;

            // Update context analysis
            self.stats.common_contexts.clear();
            for assessment in &self.assessments {
                *self.stats.common_contexts
                    .entry(assessment.context.task_context.clone())
                    .or_insert(0) += 1;
            }
        }
    }

    /// Helper function to calculate string similarity
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 1.0;
        }
        
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        // Simple Jaccard similarity based on character n-grams
        let n = 2;
        let ngrams1 = Self::get_ngrams(s1, n);
        let ngrams2 = Self::get_ngrams(s2, n);
        
        let intersection: std::collections::HashSet<_> = ngrams1.intersection(&ngrams2).collect();
        let union: std::collections::HashSet<_> = ngrams1.union(&ngrams2).collect();
        
        if union.is_empty() {
            0.0
        } else {
            intersection.len() as f64 / union.len() as f64
        }
    }

    /// Get n-grams from a string
    fn get_ngrams(s: &str, n: usize) -> std::collections::HashSet<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut ngrams = std::collections::HashSet::new();
        
        if chars.len() >= n {
            for i in 0..=chars.len() - n {
                let ngram: String = chars[i..i + n].iter().collect();
                ngrams.insert(ngram);
            }
        }
        
        ngrams
    }

    /// Detect repetitive patterns in input
    fn detect_repetitive_patterns(&self, input: &str) -> f64 {
        if input.len() < 4 {
            return 0.0;
        }

        let chars: Vec<char> = input.chars().collect();
        let mut max_repetition = 0;

        // Look for repeating subsequences
        for pattern_len in 1..=input.len() / 2 {
            for start in 0..=chars.len() - pattern_len * 2 {
                let pattern = &chars[start..start + pattern_len];
                let mut repetitions = 1;
                let mut pos = start + pattern_len;

                while pos + pattern_len <= chars.len() && 
                      &chars[pos..pos + pattern_len] == pattern {
                    repetitions += 1;
                    pos += pattern_len;
                }

                if repetitions > 1 {
                    max_repetition = max_repetition.max(repetitions * pattern_len);
                }
            }
        }

        // Return repetition ratio as anomaly score
        (max_repetition as f64 / input.len() as f64).min(1.0)
    }

    /// Get current configuration
    pub fn get_config(&self) -> &NoveltyDetectionConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: NoveltyDetectionConfig) {
        self.config = config;
    }

    /// Get system statistics
    pub fn get_stats(&self) -> &NoveltyDetectionStats {
        &self.stats
    }

    /// Get recent assessments
    pub fn get_recent_assessments(&self, limit: usize) -> Vec<&NoveltyAssessment> {
        self.assessments
            .iter()
            .rev()
            .take(limit)
            .collect()
    }

    /// Query assessments by novelty level
    pub fn get_assessments_by_level(&self, level: NoveltyLevel) -> Vec<&NoveltyAssessment> {
        self.assessments
            .iter()
            .filter(|a| a.get_novelty_level(&self.config) == level)
            .collect()
    }

    /// Export novelty assessments for analysis
    pub fn export_assessments(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.assessments)
            .context("Failed to serialize assessments")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta_memory::{MetaMemorySystem, MetaMemoryConfig};
    use std::sync::{Arc, Mutex};

    fn create_test_meta_memory() -> Arc<Mutex<MetaMemorySystem>> {
        let config = MetaMemoryConfig {
            database_path: ":memory:".to_string(),
            ..Default::default()
        };
        let meta_memory = MetaMemorySystem::with_config(config).unwrap();
        Arc::new(Mutex::new(meta_memory))
    }

    #[test]
    fn test_novelty_detection_engine_creation() -> Result<()> {
        let meta_memory = create_test_meta_memory();
        let config = NoveltyDetectionConfig::default();
        let engine = NoveltyDetectionEngine::new(config, meta_memory)?;
        
        assert_eq!(engine.assessments.len(), 0);
        assert_eq!(engine.stats.total_assessments, 0);
        Ok(())
    }

    #[test]
    fn test_basic_novelty_assessment() -> Result<()> {
        let meta_memory = create_test_meta_memory();
        let config = NoveltyDetectionConfig::default();
        let mut engine = NoveltyDetectionEngine::new(config, meta_memory)?;
        
        let assessment = engine.assess_novelty("Hello world", None)?;
        
        assert!(!assessment.input.is_empty());
        assert!(assessment.novelty_score >= 0.0 && assessment.novelty_score <= 1.0);
        assert!(assessment.method_scores.len() > 0);
        assert!(!assessment.explanation.is_empty());
        Ok(())
    }

    #[test]
    fn test_novelty_level_classification() {
        let config = NoveltyDetectionConfig::default();
        let context = NoveltyContext::default();
        
        let mut high_assessment = NoveltyAssessment::new("test".to_string(), context.clone());
        high_assessment.novelty_score = 0.8;
        assert_eq!(high_assessment.get_novelty_level(&config), NoveltyLevel::High);
        
        let mut medium_assessment = NoveltyAssessment::new("test".to_string(), context.clone());
        medium_assessment.novelty_score = 0.5;
        assert_eq!(medium_assessment.get_novelty_level(&config), NoveltyLevel::Medium);
        
        let mut low_assessment = NoveltyAssessment::new("test".to_string(), context);
        low_assessment.novelty_score = 0.2;
        assert_eq!(low_assessment.get_novelty_level(&config), NoveltyLevel::Low);
    }

    #[test]
    fn test_string_similarity() -> Result<()> {
        let meta_memory = create_test_meta_memory();
        let config = NoveltyDetectionConfig::default();
        let engine = NoveltyDetectionEngine::new(config, meta_memory)?;
        
        assert_eq!(engine.calculate_string_similarity("", ""), 1.0);
        assert_eq!(engine.calculate_string_similarity("abc", ""), 0.0);
        assert!(engine.calculate_string_similarity("hello", "hello") > 0.9);
        assert!(engine.calculate_string_similarity("hello", "world") < 0.3);
        Ok(())
    }

    #[test]
    fn test_repetitive_pattern_detection() -> Result<()> {
        let meta_memory = create_test_meta_memory();
        let config = NoveltyDetectionConfig::default();
        let engine = NoveltyDetectionEngine::new(config, meta_memory)?;
        
        assert!(engine.detect_repetitive_patterns("abababab") > 0.5);
        assert!(engine.detect_repetitive_patterns("hello world") < 0.3); // Adjusted threshold
        assert!(engine.detect_repetitive_patterns("aaaa") > 0.7);
        Ok(())
    }

    #[test]
    fn test_assessment_storage_and_retrieval() -> Result<()> {
        let meta_memory = create_test_meta_memory();
        let config = NoveltyDetectionConfig::default();
        let mut engine = NoveltyDetectionEngine::new(config, meta_memory)?;
        
        engine.assess_novelty("test input 1", None)?;
        engine.assess_novelty("test input 2", None)?;
        
        assert_eq!(engine.get_stats().total_assessments, 2);
        assert_eq!(engine.get_recent_assessments(5).len(), 2);
        Ok(())
    }
} 