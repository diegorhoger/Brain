//! Insight Extraction Engine
//! 
//! This module implements the insight extraction system that generalizes rules and causal patterns
//! from experiences stored in memory and the concept graph.
//! 
//! ## Task 5.1: Pattern Detection System
//! 
//! Creates the core system that monitors memory stores and identifies recurring patterns and relationships.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::memory::{MemorySystem, EpisodicEvent, SemanticConcept, WorkingMemoryItem};
use crate::concept_graph::{ConceptGraphManager, ConceptRelationship, RelationshipType};

/// Configuration for pattern detection algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDetectionConfig {
    /// Minimum frequency for a pattern to be considered significant
    pub min_pattern_frequency: usize,
    /// Time window for temporal pattern detection (in hours)
    pub temporal_window_hours: i64,
    /// Minimum confidence threshold for pattern significance
    pub min_confidence_threshold: f64,
    /// Maximum number of patterns to detect in a single operation
    pub max_patterns_per_batch: usize,
    /// Minimum co-occurrence count for relationship patterns
    pub min_co_occurrence_count: usize,
    /// Statistical significance threshold (p-value)
    pub significance_threshold: f64,
    /// Enable incremental pattern detection
    pub incremental_detection: bool,
    /// Batch size for processing memory items
    pub batch_size: usize,
}

impl Default for PatternDetectionConfig {
    fn default() -> Self {
        Self {
            min_pattern_frequency: 3,
            temporal_window_hours: 24,
            min_confidence_threshold: 0.6,
            max_patterns_per_batch: 100,
            min_co_occurrence_count: 2,
            significance_threshold: 0.05,
            incremental_detection: true,
            batch_size: 50,
        }
    }
}

/// Types of patterns that can be detected
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternType {
    /// Temporal sequence pattern (A happens before B)
    TemporalSequence,
    /// Co-occurrence pattern (A and B happen together)
    CoOccurrence,
    /// Causal pattern (A causes B)
    Causal,
    /// Correlation pattern (A correlates with B)
    Correlation,
    /// Frequency pattern (A happens frequently)
    Frequency,
    /// Hierarchical pattern (A is part of B)
    Hierarchical,
    /// Similarity pattern (A is similar to B)
    Similarity,
    /// Negation pattern (A prevents B)
    Negation,
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternType::TemporalSequence => write!(f, "TemporalSequence"),
            PatternType::CoOccurrence => write!(f, "CoOccurrence"),
            PatternType::Causal => write!(f, "Causal"),
            PatternType::Correlation => write!(f, "Correlation"),
            PatternType::Frequency => write!(f, "Frequency"),
            PatternType::Hierarchical => write!(f, "Hierarchical"),
            PatternType::Similarity => write!(f, "Similarity"),
            PatternType::Negation => write!(f, "Negation"),
        }
    }
}

/// A detected pattern with statistical metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    /// Unique identifier for the pattern
    pub id: Uuid,
    /// Type of pattern detected
    pub pattern_type: PatternType,
    /// Elements involved in the pattern
    pub elements: Vec<String>,
    /// Frequency of occurrence
    pub frequency: usize,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Statistical significance (p-value)
    pub significance: f64,
    /// Supporting evidence (memory item IDs)
    pub evidence: Vec<Uuid>,
    /// Timestamp when pattern was detected
    pub detected_at: DateTime<Utc>,
    /// Context information
    pub context: HashMap<String, String>,
    /// Strength of the pattern (0.0 to 1.0)
    pub strength: f64,
    /// Temporal information for sequence patterns
    pub temporal_info: Option<TemporalInfo>,
}

/// Temporal information for sequence patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalInfo {
    /// Average time delay between elements (in minutes)
    pub average_delay_minutes: f64,
    /// Standard deviation of delays
    pub delay_std_dev: f64,
    /// Minimum observed delay
    pub min_delay_minutes: f64,
    /// Maximum observed delay
    pub max_delay_minutes: f64,
}

impl DetectedPattern {
    /// Create a new detected pattern
    pub fn new(
        pattern_type: PatternType,
        elements: Vec<String>,
        frequency: usize,
        confidence: f64,
        evidence: Vec<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            pattern_type,
            elements,
            frequency,
            confidence,
            significance: 0.0, // Will be calculated
            evidence,
            detected_at: Utc::now(),
            context: HashMap::new(),
            strength: confidence, // Default to confidence
            temporal_info: None,
        }
    }

    /// Update pattern statistics with new evidence
    pub fn update_with_evidence(&mut self, new_evidence: Vec<Uuid>, new_frequency: usize) {
        self.evidence.extend(new_evidence);
        self.frequency = new_frequency;
        // Recalculate confidence based on new evidence
        self.confidence = (self.frequency as f64 / (self.evidence.len() as f64 + 1.0)).min(1.0);
        self.strength = self.confidence;
    }

    /// Check if pattern meets significance thresholds
    pub fn is_significant(&self, config: &PatternDetectionConfig) -> bool {
        self.frequency >= config.min_pattern_frequency
            && self.confidence >= config.min_confidence_threshold
            && self.significance <= config.significance_threshold
    }
}

/// Results from pattern detection operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDetectionResult {
    /// Patterns detected in this operation
    pub detected_patterns: Vec<DetectedPattern>,
    /// Number of memory items processed
    pub items_processed: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Patterns filtered out (didn't meet thresholds)
    pub filtered_patterns: usize,
    /// Statistics by pattern type
    pub pattern_type_counts: HashMap<PatternType, usize>,
}

/// Pattern detection system that monitors memory and identifies patterns
pub struct PatternDetector {
    /// Configuration for pattern detection
    config: PatternDetectionConfig,
    /// Cache of previously detected patterns
    pattern_cache: HashMap<String, DetectedPattern>,
    /// Statistics tracking
    detection_stats: DetectionStats,
    /// Last processing timestamp for incremental detection
    last_processed_at: Option<DateTime<Utc>>,
}

/// Statistics for pattern detection operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionStats {
    /// Total patterns detected
    pub total_patterns_detected: usize,
    /// Total memory items processed
    pub total_items_processed: usize,
    /// Total processing time in milliseconds
    pub total_processing_time_ms: u64,
    /// Number of detection operations performed
    pub detection_operations: usize,
    /// Average patterns per operation
    pub average_patterns_per_operation: f64,
    /// Pattern detection by type
    pub patterns_by_type: HashMap<PatternType, usize>,
}

impl Default for DetectionStats {
    fn default() -> Self {
        Self {
            total_patterns_detected: 0,
            total_items_processed: 0,
            total_processing_time_ms: 0,
            detection_operations: 0,
            average_patterns_per_operation: 0.0,
            patterns_by_type: HashMap::new(),
        }
    }
}

impl PatternDetector {
    /// Create a new pattern detector with default configuration
    pub fn new() -> Self {
        Self::with_config(PatternDetectionConfig::default())
    }

    /// Create a new pattern detector with custom configuration
    pub fn with_config(config: PatternDetectionConfig) -> Self {
        Self {
            config,
            pattern_cache: HashMap::new(),
            detection_stats: DetectionStats::default(),
            last_processed_at: None,
        }
    }

    /// Detect patterns from memory system
    pub async fn detect_patterns_from_memory(
        &mut self,
        memory_system: &MemorySystem,
    ) -> Result<PatternDetectionResult> {
        let start_time = std::time::Instant::now();
        let mut detected_patterns = Vec::new();
        let mut items_processed = 0;

        // Get memory items to process
        let (working_items, episodic_events, semantic_concepts) = 
            self.get_memory_items_for_processing(memory_system).await?;

        items_processed += working_items.len() + episodic_events.len() + semantic_concepts.len();

        // Detect temporal sequence patterns from episodic memory
        let temporal_patterns = self.detect_temporal_patterns(&episodic_events)?;
        detected_patterns.extend(temporal_patterns);

        // Detect co-occurrence patterns across all memory types
        let cooccurrence_patterns = self.detect_cooccurrence_patterns(
            &working_items, &episodic_events, &semantic_concepts
        )?;
        detected_patterns.extend(cooccurrence_patterns);

        // Detect frequency patterns
        let frequency_patterns = self.detect_frequency_patterns(&episodic_events)?;
        detected_patterns.extend(frequency_patterns);

        // Store the original count before filtering
        let total_detected = detected_patterns.len();

        // Filter patterns based on significance
        let significant_patterns: Vec<DetectedPattern> = detected_patterns
            .into_iter()
            .filter(|p| p.is_significant(&self.config))
            .collect();

        let filtered_count = total_detected - significant_patterns.len();

        // Update cache and statistics
        self.update_pattern_cache(&significant_patterns);
        self.update_detection_stats(&significant_patterns, items_processed, start_time.elapsed().as_millis() as u64);

        // Create pattern type counts
        let mut pattern_type_counts = HashMap::new();
        for pattern in &significant_patterns {
            *pattern_type_counts.entry(pattern.pattern_type.clone()).or_insert(0) += 1;
        }

        Ok(PatternDetectionResult {
            detected_patterns: significant_patterns,
            items_processed,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            filtered_patterns: filtered_count,
            pattern_type_counts,
        })
    }

    /// Detect patterns from concept graph
    pub async fn detect_patterns_from_concept_graph(
        &mut self,
        concept_graph: &ConceptGraphManager,
    ) -> Result<PatternDetectionResult> {
        let start_time = std::time::Instant::now();
        let mut detected_patterns = Vec::new();

        // Get all concepts and relationships
        let concepts = concept_graph.query_concepts(&Default::default()).await?;
        let relationships = concept_graph.query_relationships(&Default::default()).await?;

        let items_processed = concepts.len() + relationships.len();

        // Detect hierarchical patterns from IS_A relationships
        let hierarchical_patterns = self.detect_hierarchical_patterns(&relationships)?;
        detected_patterns.extend(hierarchical_patterns);

        // Detect similarity patterns from SIMILAR_TO relationships
        let similarity_patterns = self.detect_similarity_patterns(&relationships)?;
        detected_patterns.extend(similarity_patterns);

        // Detect causal patterns from CAUSES relationships
        let causal_patterns = self.detect_causal_patterns(&relationships)?;
        detected_patterns.extend(causal_patterns);

        // Store the original count before filtering
        let total_detected = detected_patterns.len();

        // Filter significant patterns
        let significant_patterns: Vec<DetectedPattern> = detected_patterns
            .into_iter()
            .filter(|p| p.is_significant(&self.config))
            .collect();

        let filtered_count = total_detected - significant_patterns.len();

        // Update statistics
        self.update_pattern_cache(&significant_patterns);
        self.update_detection_stats(&significant_patterns, items_processed, start_time.elapsed().as_millis() as u64);

        // Create pattern type counts
        let mut pattern_type_counts = HashMap::new();
        for pattern in &significant_patterns {
            *pattern_type_counts.entry(pattern.pattern_type.clone()).or_insert(0) += 1;
        }

        Ok(PatternDetectionResult {
            detected_patterns: significant_patterns,
            items_processed,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            filtered_patterns: filtered_count,
            pattern_type_counts,
        })
    }

    /// Get memory items for processing based on incremental detection settings
    async fn get_memory_items_for_processing(
        &self,
        memory_system: &MemorySystem,
    ) -> Result<(Vec<WorkingMemoryItem>, Vec<EpisodicEvent>, Vec<SemanticConcept>)> {
        // Get working memory items
        let working_query = crate::memory::WorkingMemoryQuery {
            created_after: if self.config.incremental_detection {
                self.last_processed_at
            } else {
                None
            },
            limit: Some(self.config.batch_size),
            ..Default::default()
        };
        let working_items = memory_system.query_working(&working_query)?;

        // Get episodic events
        let episodic_query = crate::memory::EpisodicQuery {
            time_range: if self.config.incremental_detection && self.last_processed_at.is_some() {
                Some((self.last_processed_at.unwrap(), Utc::now()))
            } else {
                None
            },
            limit: Some(self.config.batch_size),
            ..Default::default()
        };
        let episodic_events = memory_system.query_episodic(&episodic_query)?;

        // Get semantic concepts
        let semantic_query = crate::memory::SemanticQuery {
            limit: Some(self.config.batch_size),
            ..Default::default()
        };
        let semantic_concepts = memory_system.query_semantic(&semantic_query)?;

        Ok((working_items, episodic_events, semantic_concepts))
    }

    /// Detect temporal sequence patterns from episodic events
    fn detect_temporal_patterns(&self, events: &[EpisodicEvent]) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();
        let mut sequence_counts: HashMap<(String, String), Vec<(DateTime<Utc>, DateTime<Utc>)>> = HashMap::new();

        // Sort events by timestamp
        let mut sorted_events = events.to_vec();
        sorted_events.sort_by_key(|e| e.timestamp);

        // Look for sequences within the temporal window
        for i in 0..sorted_events.len() {
            for j in (i + 1)..sorted_events.len() {
                let event1 = &sorted_events[i];
                let event2 = &sorted_events[j];
                
                let time_diff = event2.timestamp.signed_duration_since(event1.timestamp);
                if time_diff.num_hours() <= self.config.temporal_window_hours {
                    let sequence_key = (
                        self.extract_content_pattern(&event1.content),
                        self.extract_content_pattern(&event2.content)
                    );
                    
                    sequence_counts.entry(sequence_key)
                        .or_insert_with(Vec::new)
                        .push((event1.timestamp, event2.timestamp));
                }
            }
        }

        // Create patterns from frequent sequences
        for ((pattern1, pattern2), timestamps) in sequence_counts {
            if timestamps.len() >= self.config.min_pattern_frequency {
                let confidence = timestamps.len() as f64 / events.len() as f64;
                
                // Calculate temporal information
                let delays: Vec<f64> = timestamps.iter()
                    .map(|(t1, t2)| t2.signed_duration_since(*t1).num_minutes() as f64)
                    .collect();
                
                let avg_delay = delays.iter().sum::<f64>() / delays.len() as f64;
                let variance = delays.iter()
                    .map(|d| (d - avg_delay).powi(2))
                    .sum::<f64>() / delays.len() as f64;
                let std_dev = variance.sqrt();
                
                let temporal_info = TemporalInfo {
                    average_delay_minutes: avg_delay,
                    delay_std_dev: std_dev,
                    min_delay_minutes: delays.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                    max_delay_minutes: delays.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
                };

                let mut pattern = DetectedPattern::new(
                    PatternType::TemporalSequence,
                    vec![pattern1, pattern2],
                    timestamps.len(),
                    confidence,
                    events.iter().map(|e| e.id).collect(),
                );
                pattern.temporal_info = Some(temporal_info);
                patterns.push(pattern);
            }
        }

        Ok(patterns)
    }

    /// Detect co-occurrence patterns across memory types
    fn detect_cooccurrence_patterns(
        &self,
        working_items: &[WorkingMemoryItem],
        episodic_events: &[EpisodicEvent],
        semantic_concepts: &[SemanticConcept],
    ) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();
        let mut cooccurrence_counts: HashMap<(String, String), usize> = HashMap::new();

        // Extract content patterns from all memory types
        let mut all_contents = Vec::new();
        
        for item in working_items {
            all_contents.push(self.extract_content_pattern(&item.content));
        }
        
        for event in episodic_events {
            all_contents.push(self.extract_content_pattern(&event.content));
        }
        
        for concept in semantic_concepts {
            all_contents.push(self.extract_content_pattern(&concept.name));
        }

        // Count co-occurrences
        for i in 0..all_contents.len() {
            for j in (i + 1)..all_contents.len() {
                let pattern1 = &all_contents[i];
                let pattern2 = &all_contents[j];
                
                if pattern1 != pattern2 {
                    let key = if pattern1 < pattern2 {
                        (pattern1.clone(), pattern2.clone())
                    } else {
                        (pattern2.clone(), pattern1.clone())
                    };
                    
                    *cooccurrence_counts.entry(key).or_insert(0) += 1;
                }
            }
        }

        // Create patterns from frequent co-occurrences
        for ((pattern1, pattern2), count) in cooccurrence_counts {
            if count >= self.config.min_co_occurrence_count {
                let confidence = count as f64 / all_contents.len() as f64;
                
                let pattern = DetectedPattern::new(
                    PatternType::CoOccurrence,
                    vec![pattern1, pattern2],
                    count,
                    confidence,
                    working_items.iter().map(|i| i.id)
                        .chain(episodic_events.iter().map(|e| e.id))
                        .chain(semantic_concepts.iter().map(|c| c.id))
                        .collect(),
                );
                patterns.push(pattern);
            }
        }

        Ok(patterns)
    }

    /// Detect frequency patterns from episodic events
    fn detect_frequency_patterns(&self, events: &[EpisodicEvent]) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();
        let mut frequency_counts: HashMap<String, usize> = HashMap::new();

        // Count frequency of content patterns
        for event in events {
            let pattern = self.extract_content_pattern(&event.content);
            *frequency_counts.entry(pattern).or_insert(0) += 1;
        }

        // Create patterns from frequent items
        for (pattern, count) in frequency_counts {
            if count >= self.config.min_pattern_frequency {
                let confidence = count as f64 / events.len() as f64;
                
                let detected_pattern = DetectedPattern::new(
                    PatternType::Frequency,
                    vec![pattern],
                    count,
                    confidence,
                    events.iter().map(|e| e.id).collect(),
                );
                patterns.push(detected_pattern);
            }
        }

        Ok(patterns)
    }

    /// Detect hierarchical patterns from IS_A relationships
    fn detect_hierarchical_patterns(&self, relationships: &[ConceptRelationship]) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();
        let mut hierarchy_counts: HashMap<String, usize> = HashMap::new();

        for relationship in relationships {
            if relationship.relationship_type == RelationshipType::IsA {
                let pattern = format!("{}->IS_A->{}", 
                    relationship.source_id, relationship.target_id);
                *hierarchy_counts.entry(pattern).or_insert(0) += 1;
            }
        }

        for (pattern, count) in hierarchy_counts {
            if count >= self.config.min_pattern_frequency {
                // Find the relationship to get its weight
                let rel = relationships.iter()
                    .find(|r| r.relationship_type == RelationshipType::IsA)
                    .unwrap();
                let confidence = rel.weight;
                
                let detected_pattern = DetectedPattern::new(
                    PatternType::Hierarchical,
                    vec![pattern],
                    count,
                    confidence,
                    vec![rel.id],
                );
                patterns.push(detected_pattern);
            }
        }

        Ok(patterns)
    }

    /// Detect similarity patterns from SIMILAR_TO relationships
    fn detect_similarity_patterns(&self, relationships: &[ConceptRelationship]) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();

        for relationship in relationships {
            if relationship.relationship_type == RelationshipType::SimilarTo 
                && relationship.weight >= self.config.min_confidence_threshold {
                
                let pattern = format!("{}->SIMILAR_TO->{}", 
                    relationship.source_id, relationship.target_id);
                
                let detected_pattern = DetectedPattern::new(
                    PatternType::Similarity,
                    vec![pattern],
                    relationship.activation_count as usize,
                    relationship.weight,
                    vec![relationship.id],
                );
                patterns.push(detected_pattern);
            }
        }

        Ok(patterns)
    }

    /// Detect causal patterns from CAUSES relationships
    fn detect_causal_patterns(&self, relationships: &[ConceptRelationship]) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();

        for relationship in relationships {
            if relationship.relationship_type == RelationshipType::Causes 
                && relationship.weight >= self.config.min_confidence_threshold {
                
                let pattern = format!("{}->CAUSES->{}", 
                    relationship.source_id, relationship.target_id);
                
                let detected_pattern = DetectedPattern::new(
                    PatternType::Causal,
                    vec![pattern],
                    relationship.activation_count as usize,
                    relationship.weight,
                    vec![relationship.id],
                );
                patterns.push(detected_pattern);
            }
        }

        Ok(patterns)
    }

    /// Extract a simplified content pattern from text
    fn extract_content_pattern(&self, content: &str) -> String {
        // Simple pattern extraction - in a real implementation, this could use NLP
        content.to_lowercase()
            .split_whitespace()
            .take(3) // Take first 3 words as pattern
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Update the pattern cache with new patterns
    fn update_pattern_cache(&mut self, patterns: &[DetectedPattern]) {
        for pattern in patterns {
            let cache_key = format!("{:?}:{}", pattern.pattern_type, pattern.elements.join("|"));
            self.pattern_cache.insert(cache_key, pattern.clone());
        }
    }

    /// Update detection statistics
    fn update_detection_stats(&mut self, patterns: &[DetectedPattern], items_processed: usize, processing_time_ms: u64) {
        self.detection_stats.total_patterns_detected += patterns.len();
        self.detection_stats.total_items_processed += items_processed;
        self.detection_stats.total_processing_time_ms += processing_time_ms;
        self.detection_stats.detection_operations += 1;
        
        self.detection_stats.average_patterns_per_operation = 
            self.detection_stats.total_patterns_detected as f64 / self.detection_stats.detection_operations as f64;

        for pattern in patterns {
            *self.detection_stats.patterns_by_type
                .entry(pattern.pattern_type.clone())
                .or_insert(0) += 1;
        }

        self.last_processed_at = Some(Utc::now());
    }

    /// Get cached patterns
    pub fn get_cached_patterns(&self) -> Vec<&DetectedPattern> {
        self.pattern_cache.values().collect()
    }

    /// Get detection statistics
    pub fn get_detection_stats(&self) -> &DetectionStats {
        &self.detection_stats
    }

    /// Get configuration
    pub fn get_config(&self) -> &PatternDetectionConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: PatternDetectionConfig) {
        self.config = config;
    }

    /// Clear pattern cache
    pub fn clear_cache(&mut self) {
        self.pattern_cache.clear();
    }

    /// Reset detection statistics
    pub fn reset_stats(&mut self) {
        self.detection_stats = DetectionStats::default();
        self.last_processed_at = None;
    }
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detection_config_creation() {
        let config = PatternDetectionConfig::default();
        assert_eq!(config.min_pattern_frequency, 3);
        assert_eq!(config.temporal_window_hours, 24);
        assert_eq!(config.min_confidence_threshold, 0.6);
        assert!(config.incremental_detection);
    }

    #[test]
    fn test_detected_pattern_creation() {
        let pattern = DetectedPattern::new(
            PatternType::TemporalSequence,
            vec!["event1".to_string(), "event2".to_string()],
            5,
            0.8,
            vec![Uuid::new_v4(), Uuid::new_v4()],
        );

        assert_eq!(pattern.pattern_type, PatternType::TemporalSequence);
        assert_eq!(pattern.elements.len(), 2);
        assert_eq!(pattern.frequency, 5);
        assert_eq!(pattern.confidence, 0.8);
        assert_eq!(pattern.evidence.len(), 2);
    }

    #[test]
    fn test_pattern_significance_check() {
        let config = PatternDetectionConfig::default();
        
        let significant_pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["frequent_event".to_string()],
            5, // Above min_pattern_frequency (3)
            0.7, // Above min_confidence_threshold (0.6)
            vec![Uuid::new_v4()],
        );

        assert!(significant_pattern.is_significant(&config));

        let insignificant_pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["rare_event".to_string()],
            2, // Below min_pattern_frequency (3)
            0.5, // Below min_confidence_threshold (0.6)
            vec![Uuid::new_v4()],
        );

        assert!(!insignificant_pattern.is_significant(&config));
    }

    #[test]
    fn test_pattern_detector_creation() {
        let detector = PatternDetector::new();
        assert_eq!(detector.config.min_pattern_frequency, 3);
        assert_eq!(detector.pattern_cache.len(), 0);
        assert_eq!(detector.detection_stats.total_patterns_detected, 0);
    }

    #[test]
    fn test_pattern_detector_with_custom_config() {
        let config = PatternDetectionConfig {
            min_pattern_frequency: 5,
            temporal_window_hours: 48,
            min_confidence_threshold: 0.8,
            ..Default::default()
        };

        let detector = PatternDetector::with_config(config.clone());
        assert_eq!(detector.config.min_pattern_frequency, 5);
        assert_eq!(detector.config.temporal_window_hours, 48);
        assert_eq!(detector.config.min_confidence_threshold, 0.8);
    }

    #[test]
    fn test_content_pattern_extraction() {
        let detector = PatternDetector::new();
        
        let content = "The quick brown fox jumps over the lazy dog";
        let pattern = detector.extract_content_pattern(content);
        assert_eq!(pattern, "the quick brown");

        let short_content = "Hello world";
        let short_pattern = detector.extract_content_pattern(short_content);
        assert_eq!(short_pattern, "hello world");
    }

    #[test]
    fn test_pattern_type_display() {
        assert_eq!(PatternType::TemporalSequence.to_string(), "TemporalSequence");
        assert_eq!(PatternType::CoOccurrence.to_string(), "CoOccurrence");
        assert_eq!(PatternType::Causal.to_string(), "Causal");
        assert_eq!(PatternType::Frequency.to_string(), "Frequency");
    }

    #[test]
    fn test_detection_stats_default() {
        let stats = DetectionStats::default();
        assert_eq!(stats.total_patterns_detected, 0);
        assert_eq!(stats.total_items_processed, 0);
        assert_eq!(stats.detection_operations, 0);
        assert_eq!(stats.average_patterns_per_operation, 0.0);
    }

    #[test]
    fn test_temporal_info_creation() {
        let temporal_info = TemporalInfo {
            average_delay_minutes: 30.0,
            delay_std_dev: 5.0,
            min_delay_minutes: 20.0,
            max_delay_minutes: 45.0,
        };

        assert_eq!(temporal_info.average_delay_minutes, 30.0);
        assert_eq!(temporal_info.delay_std_dev, 5.0);
        assert_eq!(temporal_info.min_delay_minutes, 20.0);
        assert_eq!(temporal_info.max_delay_minutes, 45.0);
    }

    #[test]
    fn test_pattern_cache_operations() {
        let mut detector = PatternDetector::new();
        
        let pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["test_pattern".to_string()],
            3,
            0.7,
            vec![Uuid::new_v4()],
        );

        detector.update_pattern_cache(&[pattern.clone()]);
        assert_eq!(detector.get_cached_patterns().len(), 1);

        detector.clear_cache();
        assert_eq!(detector.get_cached_patterns().len(), 0);
    }

    #[test]
    fn test_stats_reset() {
        let mut detector = PatternDetector::new();
        
        // Simulate some detection activity
        detector.detection_stats.total_patterns_detected = 10;
        detector.detection_stats.detection_operations = 2;
        detector.last_processed_at = Some(Utc::now());

        detector.reset_stats();
        assert_eq!(detector.detection_stats.total_patterns_detected, 0);
        assert_eq!(detector.detection_stats.detection_operations, 0);
        assert!(detector.last_processed_at.is_none());
    }
} 