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

// ============================================================================
// Task 5.2: Rule Formalization Framework
// ============================================================================

/// Configuration for rule formalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleFormalizationConfig {
    /// Minimum pattern frequency to create a rule
    pub min_pattern_frequency_for_rule: usize,
    /// Minimum confidence threshold for rule creation
    pub min_rule_confidence: f64,
    /// Maximum number of rules to create in a single operation
    pub max_rules_per_batch: usize,
    /// Minimum support threshold (frequency of observation)
    pub min_support_threshold: f64,
    /// Minimum generality threshold for rule applicability
    pub min_generality_threshold: f64,
    /// Rule validation window (hours) for historical testing
    pub validation_window_hours: i64,
    /// Enable rule contradiction detection
    pub enable_contradiction_detection: bool,
    /// Rule deprecation threshold (confidence below which rules are removed)
    pub deprecation_threshold: f64,
}

impl Default for RuleFormalizationConfig {
    fn default() -> Self {
        Self {
            min_pattern_frequency_for_rule: 3,
            min_rule_confidence: 0.7,
            max_rules_per_batch: 50,
            min_support_threshold: 0.6,
            min_generality_threshold: 0.5,
            validation_window_hours: 168, // 1 week
            enable_contradiction_detection: true,
            deprecation_threshold: 0.3,
        }
    }
}

/// Types of rule patterns (antecedents)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RulePattern {
    /// Single element pattern
    Single(String),
    /// Sequence pattern (ordered elements)
    Sequence(Vec<String>),
    /// Co-occurrence pattern (unordered elements)
    CoOccurrence(Vec<String>),
    /// Conditional pattern with context
    Conditional {
        condition: String,
        context: HashMap<String, String>,
    },
    /// Temporal pattern with timing constraints
    Temporal {
        elements: Vec<String>,
        timing_constraints: TemporalConstraints,
    },
    /// Hierarchical pattern with relationships
    Hierarchical {
        parent: String,
        children: Vec<String>,
    },
}

/// Temporal constraints for rule patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalConstraints {
    /// Maximum time between elements (in minutes)
    pub max_delay_minutes: f64,
    /// Minimum time between elements (in minutes)
    pub min_delay_minutes: f64,
    /// Required order of elements
    pub strict_order: bool,
}

impl Eq for TemporalConstraints {}

impl std::hash::Hash for TemporalConstraints {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the float values as their bit representation
        self.max_delay_minutes.to_bits().hash(state);
        self.min_delay_minutes.to_bits().hash(state);
        self.strict_order.hash(state);
    }
}

/// Types of rule outcomes (consequents)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuleOutcome {
    /// Single outcome element
    Single(String),
    /// Multiple possible outcomes with probabilities
    Multiple(Vec<(String, f64)>),
    /// State change outcome
    StateChange {
        from_state: String,
        to_state: String,
    },
    /// Action outcome
    Action(String),
    /// Negation outcome (something doesn't happen)
    Negation(String),
}

/// Metrics for rule quality and applicability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetrics {
    /// Support: frequency of observation (0.0 to 1.0)
    pub support: f64,
    /// Confidence: consistency of outcome (0.0 to 1.0)
    pub confidence: f64,
    /// Generality: applicability across contexts (0.0 to 1.0)
    pub generality: f64,
    /// Reusability: how often the rule is successfully applied (0.0 to 1.0)
    pub reusability: f64,
    /// Lift: how much more likely the outcome is given the pattern
    pub lift: f64,
    /// Conviction: strength of implication
    pub conviction: f64,
}

impl RuleMetrics {
    /// Create new rule metrics with basic values
    pub fn new(support: f64, confidence: f64) -> Self {
        Self {
            support: support.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
            generality: 0.5, // Default middle value
            reusability: 0.0, // Will be updated with usage
            lift: 1.0, // Default no lift
            conviction: 1.0, // Default no conviction
        }
    }

    /// Update metrics based on new observations
    pub fn update(&mut self, new_support: f64, new_confidence: f64, usage_count: u64) {
        self.support = new_support.clamp(0.0, 1.0);
        self.confidence = new_confidence.clamp(0.0, 1.0);
        
        // Update reusability based on usage
        self.reusability = (usage_count as f64 / (usage_count as f64 + 1.0)).min(1.0);
        
        // Calculate lift (confidence / baseline probability)
        // Simplified calculation - in practice would need baseline data
        self.lift = if self.support > 0.0 { self.confidence / self.support } else { 1.0 };
        
        // Calculate conviction
        self.conviction = if self.confidence < 1.0 {
            (1.0 - self.support) / (1.0 - self.confidence)
        } else {
            f64::INFINITY
        };
    }

    /// Check if rule meets quality thresholds
    pub fn meets_thresholds(&self, config: &RuleFormalizationConfig) -> bool {
        self.support >= config.min_support_threshold
            && self.confidence >= config.min_rule_confidence
            && self.generality >= config.min_generality_threshold
    }
}

/// A formalized rule in [Pattern] → [Outcome] format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Unique identifier for the rule
    pub id: Uuid,
    /// Pattern (antecedent) of the rule
    pub pattern: RulePattern,
    /// Outcome (consequent) of the rule
    pub outcome: RuleOutcome,
    /// Quality metrics for the rule
    pub metrics: RuleMetrics,
    /// Source patterns that generated this rule
    pub source_patterns: Vec<Uuid>,
    /// Timestamp when rule was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when rule was last updated
    pub last_updated_at: DateTime<Utc>,
    /// Number of times rule has been applied
    pub usage_count: u64,
    /// Number of successful applications
    pub success_count: u64,
    /// Context in which the rule applies
    pub context: HashMap<String, String>,
    /// Rule version for tracking evolution
    pub version: u32,
    /// Whether the rule is active or deprecated
    pub is_active: bool,
}

impl Rule {
    /// Create a new rule from a detected pattern
    pub fn from_pattern(pattern: &DetectedPattern, outcome: RuleOutcome) -> Self {
        let rule_pattern = Self::convert_pattern_to_rule_pattern(pattern);
        let metrics = RuleMetrics::new(
            (pattern.frequency as f64 / 100.0).min(1.0).max(0.0), // Normalize frequency to support, ensure it's reasonable
            pattern.confidence,
        );

        Self {
            id: Uuid::new_v4(),
            pattern: rule_pattern,
            outcome,
            metrics,
            source_patterns: vec![pattern.id],
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            usage_count: 0,
            success_count: 0,
            context: pattern.context.clone(),
            version: 1,
            is_active: true,
        }
    }

    /// Convert DetectedPattern to RulePattern
    fn convert_pattern_to_rule_pattern(pattern: &DetectedPattern) -> RulePattern {
        match pattern.pattern_type {
            PatternType::TemporalSequence => {
                if let Some(temporal_info) = &pattern.temporal_info {
                    RulePattern::Temporal {
                        elements: pattern.elements.clone(),
                        timing_constraints: TemporalConstraints {
                            max_delay_minutes: temporal_info.max_delay_minutes,
                            min_delay_minutes: temporal_info.min_delay_minutes,
                            strict_order: true,
                        },
                    }
                } else {
                    RulePattern::Sequence(pattern.elements.clone())
                }
            }
            PatternType::CoOccurrence => RulePattern::CoOccurrence(pattern.elements.clone()),
            PatternType::Hierarchical => {
                if pattern.elements.len() >= 2 {
                    RulePattern::Hierarchical {
                        parent: pattern.elements[0].clone(),
                        children: pattern.elements[1..].to_vec(),
                    }
                } else {
                    RulePattern::Single(pattern.elements.first().unwrap_or(&"unknown".to_string()).clone())
                }
            }
            _ => {
                if pattern.elements.len() == 1 {
                    RulePattern::Single(pattern.elements[0].clone())
                } else {
                    RulePattern::Sequence(pattern.elements.clone())
                }
            }
        }
    }

    /// Update rule with new evidence
    pub fn update_with_evidence(&mut self, success: bool, new_support: f64, new_confidence: f64) {
        self.usage_count += 1;
        if success {
            self.success_count += 1;
        }
        
        self.metrics.update(new_support, new_confidence, self.usage_count);
        self.last_updated_at = Utc::now();
        self.version += 1;
    }

    /// Check if rule should be deprecated
    pub fn should_deprecate(&self, config: &RuleFormalizationConfig) -> bool {
        !self.is_active || self.metrics.confidence < config.deprecation_threshold
    }

    /// Get rule success rate
    pub fn success_rate(&self) -> f64 {
        if self.usage_count > 0 {
            self.success_count as f64 / self.usage_count as f64
        } else {
            0.0
        }
    }
}

/// Query parameters for rule retrieval
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RuleQuery {
    /// Filter by pattern type
    pub pattern_type: Option<String>,
    /// Filter by minimum confidence
    pub min_confidence: Option<f64>,
    /// Filter by minimum support
    pub min_support: Option<f64>,
    /// Filter by context key-value pairs
    pub context_filters: HashMap<String, String>,
    /// Filter by active status
    pub active_only: bool,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Sort by field
    pub sort_by: Option<String>,
    /// Sort in descending order
    pub descending: bool,
}

/// Results from rule validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleValidationResult {
    /// Rule being validated
    pub rule_id: Uuid,
    /// Number of test cases
    pub test_cases: usize,
    /// Number of successful predictions
    pub successful_predictions: usize,
    /// Validation accuracy
    pub accuracy: f64,
    /// Precision score
    pub precision: f64,
    /// Recall score
    pub recall: f64,
    /// F1 score
    pub f1_score: f64,
}

/// Rule comparison result for detecting contradictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleComparison {
    /// First rule ID
    pub rule1_id: Uuid,
    /// Second rule ID
    pub rule2_id: Uuid,
    /// Similarity score between patterns (0.0 to 1.0)
    pub pattern_similarity: f64,
    /// Whether outcomes contradict each other
    pub outcomes_contradict: bool,
    /// Overlap in context
    pub context_overlap: f64,
    /// Recommendation for handling the comparison
    pub recommendation: ComparisonRecommendation,
}

/// Recommendations for handling rule comparisons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonRecommendation {
    /// Rules are compatible, no action needed
    Compatible,
    /// Rules should be merged due to high similarity
    Merge,
    /// Rules contradict and need resolution
    ResolveContradiction,
    /// Rules overlap but serve different contexts
    KeepBoth,
    /// One rule should be deprecated in favor of the other
    DeprecateWeaker,
    /// Keep the rule with higher confidence
    KeepHigherConfidence,
    /// Keep the rule with more evidence
    KeepMoreEvidence,
    /// Manual review required
    ManualReview,
}

/// Database for storing and managing rules
#[derive(Debug)]
pub struct RuleDatabase {
    /// Storage for rules indexed by ID
    rules: HashMap<Uuid, Rule>,
    /// Index by pattern type for efficient retrieval
    pattern_type_index: HashMap<String, Vec<Uuid>>,
    /// Index by confidence ranges
    confidence_index: HashMap<String, Vec<Uuid>>, // "high", "medium", "low"
    /// Index by context keys
    context_index: HashMap<String, Vec<Uuid>>,
    /// Configuration for rule management
    config: RuleFormalizationConfig,
    /// Statistics tracking
    stats: RuleDatabaseStats,
}

/// Statistics for rule database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDatabaseStats {
    /// Total number of rules
    pub total_rules: usize,
    /// Number of active rules
    pub active_rules: usize,
    /// Number of deprecated rules
    pub deprecated_rules: usize,
    /// Average rule confidence
    pub average_confidence: f64,
    /// Average rule support
    pub average_support: f64,
    /// Rules by pattern type
    pub rules_by_pattern_type: HashMap<String, usize>,
    /// Total rule applications
    pub total_applications: u64,
    /// Total successful applications
    pub total_successes: u64,
    /// Overall success rate
    pub overall_success_rate: f64,
}

impl Default for RuleDatabaseStats {
    fn default() -> Self {
        Self {
            total_rules: 0,
            active_rules: 0,
            deprecated_rules: 0,
            average_confidence: 0.0,
            average_support: 0.0,
            rules_by_pattern_type: HashMap::new(),
            total_applications: 0,
            total_successes: 0,
            overall_success_rate: 0.0,
        }
    }
}

impl RuleDatabase {
    /// Create a new rule database
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            pattern_type_index: HashMap::new(),
            confidence_index: HashMap::new(),
            context_index: HashMap::new(),
            config: RuleFormalizationConfig::default(),
            stats: RuleDatabaseStats::default(),
        }
    }

    /// Create rule database with custom configuration
    pub fn with_config(config: RuleFormalizationConfig) -> Self {
        Self {
            rules: HashMap::new(),
            pattern_type_index: HashMap::new(),
            confidence_index: HashMap::new(),
            context_index: HashMap::new(),
            config,
            stats: RuleDatabaseStats::default(),
        }
    }

    /// Store a rule in the database
    pub fn store_rule(&mut self, rule: Rule) -> Result<Uuid> {
        let rule_id = rule.id;
        
        // Update indexes
        self.update_indexes(&rule);
        
        // Store the rule
        self.rules.insert(rule_id, rule);
        
        // Update statistics
        self.update_stats();
        
        Ok(rule_id)
    }

    /// Retrieve a rule by ID
    pub fn get_rule(&self, id: Uuid) -> Option<&Rule> {
        self.rules.get(&id)
    }

    /// Update an existing rule
    pub fn update_rule(&mut self, rule: Rule) -> Result<()> {
        let rule_id = rule.id;
        
        if self.rules.contains_key(&rule_id) {
            // Remove old indexes by cloning the old rule first
            if let Some(old_rule) = self.rules.get(&rule_id).cloned() {
                self.remove_from_indexes(&old_rule);
            }
            
            // Update indexes with new rule
            self.update_indexes(&rule);
            
            // Store updated rule
            self.rules.insert(rule_id, rule);
            
            // Update statistics
            self.update_stats();
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Rule with ID {} not found", rule_id))
        }
    }

    /// Remove a rule from the database
    pub fn remove_rule(&mut self, id: Uuid) -> Result<bool> {
        if let Some(rule) = self.rules.remove(&id) {
            self.remove_from_indexes(&rule);
            self.update_stats();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Query rules based on criteria
    pub fn query_rules(&self, query: &RuleQuery) -> Vec<&Rule> {
        let mut results: Vec<&Rule> = self.rules.values().collect();

        // Apply filters
        if query.active_only {
            results.retain(|rule| rule.is_active);
        }

        if let Some(min_confidence) = query.min_confidence {
            results.retain(|rule| rule.metrics.confidence >= min_confidence);
        }

        if let Some(min_support) = query.min_support {
            results.retain(|rule| rule.metrics.support >= min_support);
        }

        // Filter by context
        for (key, value) in &query.context_filters {
            results.retain(|rule| {
                rule.context.get(key).map_or(false, |v| v == value)
            });
        }

        // Sort results
        if let Some(sort_field) = &query.sort_by {
            match sort_field.as_str() {
                "confidence" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.metrics.confidence.partial_cmp(&a.metrics.confidence).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            a.metrics.confidence.partial_cmp(&b.metrics.confidence).unwrap_or(std::cmp::Ordering::Equal)
                        }
                    });
                }
                "support" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.metrics.support.partial_cmp(&a.metrics.support).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            a.metrics.support.partial_cmp(&b.metrics.support).unwrap_or(std::cmp::Ordering::Equal)
                        }
                    });
                }
                "usage_count" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.usage_count.cmp(&a.usage_count)
                        } else {
                            a.usage_count.cmp(&b.usage_count)
                        }
                    });
                }
                _ => {} // No sorting for unknown fields
            }
        }

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        results
    }

    /// Get all active rules
    pub fn get_active_rules(&self) -> Vec<&Rule> {
        self.rules.values().filter(|rule| rule.is_active).collect()
    }

    /// Get rules that should be deprecated
    pub fn get_deprecated_candidates(&self) -> Vec<&Rule> {
        self.rules.values()
            .filter(|rule| rule.should_deprecate(&self.config))
            .collect()
    }

    /// Update indexes for a rule
    fn update_indexes(&mut self, rule: &Rule) {
        // Pattern type index
        let pattern_type = self.get_pattern_type_string(&rule.pattern);
        self.pattern_type_index
            .entry(pattern_type)
            .or_insert_with(Vec::new)
            .push(rule.id);

        // Confidence index
        let confidence_category = if rule.metrics.confidence >= 0.8 {
            "high"
        } else if rule.metrics.confidence >= 0.5 {
            "medium"
        } else {
            "low"
        };
        self.confidence_index
            .entry(confidence_category.to_string())
            .or_insert_with(Vec::new)
            .push(rule.id);

        // Context index
        for key in rule.context.keys() {
            self.context_index
                .entry(key.clone())
                .or_insert_with(Vec::new)
                .push(rule.id);
        }
    }

    /// Remove rule from indexes
    fn remove_from_indexes(&mut self, rule: &Rule) {
        // Remove from pattern type index
        let pattern_type = self.get_pattern_type_string(&rule.pattern);
        if let Some(ids) = self.pattern_type_index.get_mut(&pattern_type) {
            ids.retain(|&id| id != rule.id);
        }

        // Remove from confidence index
        let confidence_category = if rule.metrics.confidence >= 0.8 {
            "high"
        } else if rule.metrics.confidence >= 0.5 {
            "medium"
        } else {
            "low"
        };
        if let Some(ids) = self.confidence_index.get_mut(confidence_category) {
            ids.retain(|&id| id != rule.id);
        }

        // Remove from context index
        for key in rule.context.keys() {
            if let Some(ids) = self.context_index.get_mut(key) {
                ids.retain(|&id| id != rule.id);
            }
        }
    }

    /// Get pattern type as string for indexing
    fn get_pattern_type_string(&self, pattern: &RulePattern) -> String {
        match pattern {
            RulePattern::Single(_) => "single".to_string(),
            RulePattern::Sequence(_) => "sequence".to_string(),
            RulePattern::CoOccurrence(_) => "co_occurrence".to_string(),
            RulePattern::Conditional { .. } => "conditional".to_string(),
            RulePattern::Temporal { .. } => "temporal".to_string(),
            RulePattern::Hierarchical { .. } => "hierarchical".to_string(),
        }
    }

    /// Update database statistics
    fn update_stats(&mut self) {
        let total_rules = self.rules.len();
        let active_rules = self.rules.values().filter(|r| r.is_active).count();
        let deprecated_rules = total_rules - active_rules;

        let total_confidence: f64 = self.rules.values().map(|r| r.metrics.confidence).sum();
        let total_support: f64 = self.rules.values().map(|r| r.metrics.support).sum();
        let average_confidence = if total_rules > 0 { total_confidence / total_rules as f64 } else { 0.0 };
        let average_support = if total_rules > 0 { total_support / total_rules as f64 } else { 0.0 };

        let total_applications: u64 = self.rules.values().map(|r| r.usage_count).sum();
        let total_successes: u64 = self.rules.values().map(|r| r.success_count).sum();
        let overall_success_rate = if total_applications > 0 {
            total_successes as f64 / total_applications as f64
        } else {
            0.0
        };

        let mut rules_by_pattern_type = HashMap::new();
        for rule in self.rules.values() {
            let pattern_type = self.get_pattern_type_string(&rule.pattern);
            *rules_by_pattern_type.entry(pattern_type).or_insert(0) += 1;
        }

        self.stats = RuleDatabaseStats {
            total_rules,
            active_rules,
            deprecated_rules,
            average_confidence,
            average_support,
            rules_by_pattern_type,
            total_applications,
            total_successes,
            overall_success_rate,
        };
    }

    /// Get database statistics
    pub fn get_stats(&self) -> &RuleDatabaseStats {
        &self.stats
    }

    /// Get configuration
    pub fn get_config(&self) -> &RuleFormalizationConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: RuleFormalizationConfig) {
        self.config = config;
    }
}

/// Main rule formalization engine that transforms patterns into rules
pub struct RuleFormalizationEngine {
    /// Rule database for storage and retrieval
    rule_database: RuleDatabase,
    /// Configuration for rule formalization
    config: RuleFormalizationConfig,
}

impl RuleFormalizationEngine {
    /// Create a new rule formalization engine
    pub fn new() -> Self {
        Self {
            rule_database: RuleDatabase::new(),
            config: RuleFormalizationConfig::default(),
        }
    }

    /// Create engine with custom configuration
    pub fn with_config(config: RuleFormalizationConfig) -> Self {
        Self {
            rule_database: RuleDatabase::with_config(config.clone()),
            config,
        }
    }

    /// Transform detected patterns into formal rules
    pub fn formalize_patterns(&mut self, patterns: &[DetectedPattern]) -> Result<Vec<Uuid>> {
        let mut rule_ids = Vec::new();
        let mut processed = 0;

        for pattern in patterns {
            if processed >= self.config.max_rules_per_batch {
                break;
            }

            // Check if pattern meets minimum requirements for rule creation
            if pattern.frequency < self.config.min_pattern_frequency_for_rule
                || pattern.confidence < self.config.min_rule_confidence
            {
                continue;
            }

            // Create rule outcome based on pattern type
            let outcome = self.create_outcome_from_pattern(pattern);
            
            // Create rule from pattern
            let rule = Rule::from_pattern(pattern, outcome);
            
            // Check if rule meets quality thresholds
            if rule.metrics.meets_thresholds(&self.config) {
                let rule_id = self.rule_database.store_rule(rule)?;
                rule_ids.push(rule_id);
                processed += 1;
            }
        }

        Ok(rule_ids)
    }

    /// Create appropriate outcome from detected pattern
    fn create_outcome_from_pattern(&self, pattern: &DetectedPattern) -> RuleOutcome {
        match pattern.pattern_type {
            PatternType::TemporalSequence => {
                if pattern.elements.len() >= 2 {
                    // Last element is the outcome
                    RuleOutcome::Single(pattern.elements.last().unwrap().clone())
                } else {
                    RuleOutcome::Action("sequence_completion".to_string())
                }
            }
            PatternType::Causal => {
                if pattern.elements.len() >= 2 {
                    RuleOutcome::Single(pattern.elements.last().unwrap().clone())
                } else {
                    RuleOutcome::Action("causal_effect".to_string())
                }
            }
            PatternType::CoOccurrence => {
                RuleOutcome::Multiple(
                    pattern.elements.iter()
                        .map(|e| (e.clone(), pattern.confidence))
                        .collect()
                )
            }
            PatternType::Frequency => {
                RuleOutcome::Single(format!("frequent_{}", pattern.elements.first().unwrap_or(&"event".to_string())))
            }
            PatternType::Hierarchical => {
                if pattern.elements.len() >= 2 {
                    RuleOutcome::StateChange {
                        from_state: pattern.elements[0].clone(),
                        to_state: pattern.elements[1].clone(),
                    }
                } else {
                    RuleOutcome::Action("hierarchical_relation".to_string())
                }
            }
            PatternType::Negation => {
                RuleOutcome::Negation(pattern.elements.first().unwrap_or(&"unknown".to_string()).clone())
            }
            _ => {
                // Default outcome for other pattern types
                RuleOutcome::Action(format!("{}_outcome", pattern.pattern_type.to_string().to_lowercase()))
            }
        }
    }

    /// Validate rules against historical data
    pub async fn validate_rules(&self, memory_system: &MemorySystem) -> Result<Vec<RuleValidationResult>> {
        let mut validation_results = Vec::new();
        let active_rules = self.rule_database.get_active_rules();

        for rule in active_rules {
            let validation_result = self.validate_single_rule(rule, memory_system).await?;
            validation_results.push(validation_result);
        }

        Ok(validation_results)
    }

    /// Validate a single rule against historical data
    async fn validate_single_rule(&self, rule: &Rule, memory_system: &MemorySystem) -> Result<RuleValidationResult> {
        // Get historical data for validation
        let end_time = Utc::now();
        let start_time = end_time - chrono::Duration::hours(self.config.validation_window_hours);
        
        let episodic_query = crate::memory::EpisodicQuery {
            time_range: Some((start_time, end_time)),
            ..Default::default()
        };
        
        let historical_events = memory_system.query_episodic(&episodic_query)?;
        
        // Test rule predictions against historical data
        let mut test_cases = 0;
        let mut successful_predictions = 0;
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for event in &historical_events {
            if self.rule_pattern_matches(&rule.pattern, &event.content) {
                test_cases += 1;
                
                // Check if the rule's outcome occurred
                let outcome_occurred = self.check_outcome_occurrence(&rule.outcome, &historical_events, event);
                
                if outcome_occurred {
                    successful_predictions += 1;
                    true_positives += 1;
                } else {
                    false_positives += 1;
                }
            } else {
                // Check for false negatives (outcome occurred without pattern)
                let outcome_occurred = self.check_outcome_occurrence(&rule.outcome, &historical_events, event);
                if outcome_occurred {
                    false_negatives += 1;
                }
            }
        }

        let accuracy = if test_cases > 0 {
            successful_predictions as f64 / test_cases as f64
        } else {
            0.0
        };

        let precision = if (true_positives + false_positives) > 0 {
            true_positives as f64 / (true_positives + false_positives) as f64
        } else {
            0.0
        };

        let recall = if (true_positives + false_negatives) > 0 {
            true_positives as f64 / (true_positives + false_negatives) as f64
        } else {
            0.0
        };

        let f1_score = if (precision + recall) > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        Ok(RuleValidationResult {
            rule_id: rule.id,
            test_cases,
            successful_predictions,
            accuracy,
            precision,
            recall,
            f1_score,
        })
    }

    /// Check if rule pattern matches event content
    fn rule_pattern_matches(&self, pattern: &RulePattern, content: &str) -> bool {
        match pattern {
            RulePattern::Single(element) => content.contains(element),
            RulePattern::Sequence(elements) => {
                elements.iter().all(|element| content.contains(element))
            }
            RulePattern::CoOccurrence(elements) => {
                elements.iter().all(|element| content.contains(element))
            }
            RulePattern::Conditional { condition, .. } => content.contains(condition),
            RulePattern::Temporal { elements, .. } => {
                elements.iter().all(|element| content.contains(element))
            }
            RulePattern::Hierarchical { parent, children } => {
                content.contains(parent) && children.iter().any(|child| content.contains(child))
            }
        }
    }

    /// Check if rule outcome occurred in historical data
    fn check_outcome_occurrence(&self, outcome: &RuleOutcome, events: &[crate::memory::EpisodicEvent], _trigger_event: &crate::memory::EpisodicEvent) -> bool {
        match outcome {
            RuleOutcome::Single(outcome_text) => {
                events.iter().any(|event| event.content.contains(outcome_text))
            }
            RuleOutcome::Multiple(outcomes) => {
                outcomes.iter().any(|(outcome_text, _prob)| {
                    events.iter().any(|event| event.content.contains(outcome_text))
                })
            }
            RuleOutcome::StateChange { to_state, .. } => {
                events.iter().any(|event| event.content.contains(to_state))
            }
            RuleOutcome::Action(action) => {
                events.iter().any(|event| event.content.contains(action))
            }
            RuleOutcome::Negation(negated_outcome) => {
                !events.iter().any(|event| event.content.contains(negated_outcome))
            }
        }
    }

    /// Compare rules to detect contradictions and overlaps
    pub fn compare_rules(&self, rule1_id: Uuid, rule2_id: Uuid) -> Result<RuleComparison> {
        let rule1 = self.rule_database.get_rule(rule1_id)
            .ok_or_else(|| anyhow::anyhow!("Rule {} not found", rule1_id))?;
        let rule2 = self.rule_database.get_rule(rule2_id)
            .ok_or_else(|| anyhow::anyhow!("Rule {} not found", rule2_id))?;

        let pattern_similarity = self.calculate_pattern_similarity(&rule1.pattern, &rule2.pattern);
        let outcomes_contradict = self.check_outcome_contradiction(&rule1.outcome, &rule2.outcome);
        let context_overlap = self.calculate_context_overlap(&rule1.context, &rule2.context);

        let recommendation = if pattern_similarity > 0.9 && !outcomes_contradict {
            ComparisonRecommendation::Merge
        } else if pattern_similarity > 0.7 && outcomes_contradict {
            ComparisonRecommendation::ResolveContradiction
        } else if pattern_similarity > 0.5 && context_overlap < 0.3 {
            ComparisonRecommendation::KeepBoth
        } else if outcomes_contradict && rule1.metrics.confidence > rule2.metrics.confidence {
            ComparisonRecommendation::DeprecateWeaker
        } else {
            ComparisonRecommendation::Compatible
        };

        Ok(RuleComparison {
            rule1_id,
            rule2_id,
            pattern_similarity,
            outcomes_contradict,
            context_overlap,
            recommendation,
        })
    }

    /// Calculate similarity between two rule patterns
    fn calculate_pattern_similarity(&self, pattern1: &RulePattern, pattern2: &RulePattern) -> f64 {
        match (pattern1, pattern2) {
            (RulePattern::Single(e1), RulePattern::Single(e2)) => {
                if e1 == e2 { 1.0 } else { 0.0 }
            }
            (RulePattern::Sequence(seq1), RulePattern::Sequence(seq2)) => {
                self.calculate_sequence_similarity(seq1, seq2)
            }
            (RulePattern::CoOccurrence(co1), RulePattern::CoOccurrence(co2)) => {
                self.calculate_set_similarity(co1, co2)
            }
            _ => 0.0 // Different pattern types have no similarity
        }
    }

    /// Calculate similarity between two sequences
    fn calculate_sequence_similarity(&self, seq1: &[String], seq2: &[String]) -> f64 {
        if seq1.is_empty() && seq2.is_empty() {
            return 1.0;
        }
        if seq1.is_empty() || seq2.is_empty() {
            return 0.0;
        }

        let common_elements = seq1.iter()
            .filter(|e1| seq2.contains(e1))
            .count();
        
        let total_elements = seq1.len().max(seq2.len());
        common_elements as f64 / total_elements as f64
    }

    /// Calculate similarity between two sets
    fn calculate_set_similarity(&self, set1: &[String], set2: &[String]) -> f64 {
        if set1.is_empty() && set2.is_empty() {
            return 1.0;
        }
        if set1.is_empty() || set2.is_empty() {
            return 0.0;
        }

        let set1_set: std::collections::HashSet<_> = set1.iter().collect();
        let set2_set: std::collections::HashSet<_> = set2.iter().collect();
        
        let intersection = set1_set.intersection(&set2_set).count();
        let union = set1_set.union(&set2_set).count();
        
        if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        }
    }

    /// Check if two outcomes contradict each other
    fn check_outcome_contradiction(&self, outcome1: &RuleOutcome, outcome2: &RuleOutcome) -> bool {
        match (outcome1, outcome2) {
            (RuleOutcome::Single(o1), RuleOutcome::Negation(o2)) => o1 == o2,
            (RuleOutcome::Negation(o1), RuleOutcome::Single(o2)) => o1 == o2,
            (RuleOutcome::StateChange { to_state: to1, .. }, RuleOutcome::StateChange { to_state: to2, .. }) => {
                to1 != to2
            }
            _ => false // Most outcomes don't directly contradict
        }
    }

    /// Calculate overlap between two context maps
    fn calculate_context_overlap(&self, context1: &HashMap<String, String>, context2: &HashMap<String, String>) -> f64 {
        if context1.is_empty() && context2.is_empty() {
            return 1.0;
        }
        if context1.is_empty() || context2.is_empty() {
            return 0.0;
        }

        let common_keys = context1.keys()
            .filter(|key| context2.contains_key(*key) && context1[*key] == context2[*key])
            .count();
        
        let total_keys = context1.len().max(context2.len());
        common_keys as f64 / total_keys as f64
    }

    /// Get rule database reference
    pub fn get_rule_database(&self) -> &RuleDatabase {
        &self.rule_database
    }

    /// Get mutable rule database reference
    pub fn get_rule_database_mut(&mut self) -> &mut RuleDatabase {
        &mut self.rule_database
    }

    /// Get configuration
    pub fn get_config(&self) -> &RuleFormalizationConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: RuleFormalizationConfig) {
        self.config = config.clone();
        self.rule_database.set_config(config);
    }
}

impl Default for RuleFormalizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// ## Task 5.3: Rule Generalization and Maintenance System
/// 
/// Creates mechanisms to generalize specific rules into broader patterns and update rules based on new evidence.

/// Configuration for rule generalization and maintenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleGeneralizationConfig {
    /// Minimum similarity threshold for rule merging (0.0 to 1.0)
    pub min_similarity_for_merge: f64,
    /// Maximum abstraction level for generalization
    pub max_abstraction_levels: usize,
    /// Minimum rule count for creating generalizations
    pub min_rules_for_generalization: usize,
    /// Confidence threshold for maintaining generalized rules
    pub generalized_rule_confidence_threshold: f64,
    /// Maximum number of versions to track per rule
    pub max_rule_versions: u32,
    /// Time window for rule update aggregation (hours)
    pub update_aggregation_window_hours: i64,
    /// Minimum evidence strength for rule updates
    pub min_evidence_strength: f64,
    /// Enable automatic contradiction resolution
    pub auto_resolve_contradictions: bool,
    /// Threshold for deprecating contradictory rules
    pub contradiction_deprecation_threshold: f64,
}

impl Default for RuleGeneralizationConfig {
    fn default() -> Self {
        Self {
            min_similarity_for_merge: 0.8,
            max_abstraction_levels: 3,
            min_rules_for_generalization: 3,
            generalized_rule_confidence_threshold: 0.7,
            max_rule_versions: 10,
            update_aggregation_window_hours: 24,
            min_evidence_strength: 0.6,
            auto_resolve_contradictions: true,
            contradiction_deprecation_threshold: 0.3,
        }
    }
}

/// Abstraction level for rule generalization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbstractionLevel {
    /// Specific instances (no abstraction)
    Specific,
    /// Category-level abstraction (e.g., "dog" -> "animal")
    Category,
    /// Type-level abstraction (e.g., "John" -> "person")
    Type,
    /// Concept-level abstraction (e.g., "eat" -> "consume")
    Concept,
}

/// Generalized rule created from multiple specific rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralizedRule {
    /// Unique identifier for the generalized rule
    pub id: Uuid,
    /// Pattern with abstracted elements
    pub generalized_pattern: RulePattern,
    /// Outcome with abstracted elements
    pub generalized_outcome: RuleOutcome,
    /// Quality metrics aggregated from source rules
    pub aggregated_metrics: RuleMetrics,
    /// Source rules that were generalized
    pub source_rules: Vec<Uuid>,
    /// Abstraction level applied
    pub abstraction_level: AbstractionLevel,
    /// Mapping from abstract elements to concrete instances
    pub abstraction_mapping: HashMap<String, Vec<String>>,
    /// Timestamp when generalization was created
    pub created_at: DateTime<Utc>,
    /// Confidence in the generalization (0.0 to 1.0)
    pub generalization_confidence: f64,
    /// Number of concrete instances covered
    pub coverage_count: usize,
}

/// Rule update information for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleUpdate {
    /// Unique identifier for the update
    pub id: Uuid,
    /// Rule being updated
    pub rule_id: Uuid,
    /// Type of update applied
    pub update_type: UpdateType,
    /// New evidence that triggered the update
    pub evidence: Vec<Uuid>,
    /// Previous metrics before update
    pub previous_metrics: RuleMetrics,
    /// New metrics after update
    pub updated_metrics: RuleMetrics,
    /// Timestamp of the update
    pub timestamp: DateTime<Utc>,
    /// Confidence in the update (0.0 to 1.0)
    pub update_confidence: f64,
    /// Human-readable description of the change
    pub description: String,
}

/// Types of rule updates
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateType {
    /// Confidence score adjustment
    ConfidenceUpdate,
    /// Support frequency update
    SupportUpdate,
    /// Pattern refinement
    PatternRefinement,
    /// Outcome modification
    OutcomeModification,
    /// Context expansion
    ContextExpansion,
    /// Deprecation due to low performance
    Deprecation,
    /// Reactivation after improvement
    Reactivation,
}

/// Rule version for tracking evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleVersion {
    /// Version number
    pub version: u32,
    /// Rule state at this version
    pub rule_snapshot: Rule,
    /// Update that created this version
    pub update_info: RuleUpdate,
    /// Timestamp of version creation
    pub created_at: DateTime<Utc>,
    /// Whether this version is deprecated
    pub is_deprecated: bool,
}

/// Contradiction detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContradictionResult {
    /// Pairs of rules that contradict each other
    pub contradictory_pairs: Vec<(Uuid, Uuid)>,
    /// Severity of each contradiction (0.0 to 1.0)
    pub contradiction_severity: HashMap<(Uuid, Uuid), f64>,
    /// Recommendations for resolving contradictions
    pub resolution_recommendations: HashMap<(Uuid, Uuid), ContradictionResolution>,
    /// Total contradictions found
    pub total_contradictions: usize,
    /// High-priority contradictions requiring immediate attention
    pub high_priority_contradictions: Vec<(Uuid, Uuid)>,
}

/// Methods for resolving rule contradictions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContradictionResolution {
    /// Keep the rule with higher confidence
    KeepHigherConfidence,
    /// Keep the rule with more evidence
    KeepMoreEvidence,
    /// Create a conditional rule that handles both cases
    CreateConditionalRule,
    /// Mark both rules for manual review
    ManualReview,
    /// Deprecate both conflicting rules
    DeprecateBoth,
    /// Attempt to find a generalization that covers both
    GeneralizeToResolve,
    /// Keep both rules as they serve different contexts
    KeepBoth,
}

/// Query interface for external system components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleQueryInterface {
    /// Query parameters
    pub query: RuleQuery,
    /// Include confidence scores in results
    pub include_confidence: bool,
    /// Include rule history/versions
    pub include_history: bool,
    /// Maximum staleness of cached results (minutes)
    pub max_cache_age_minutes: u64,
    /// Enable smart caching based on query patterns
    pub smart_caching: bool,
}

/// Enhanced query result with confidence and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedRuleResult {
    /// The matching rule
    pub rule: Rule,
    /// Query match confidence (0.0 to 1.0)
    pub match_confidence: f64,
    /// Related rules that might be relevant
    pub related_rules: Vec<Uuid>,
    /// Explanation of why this rule was selected
    pub selection_reasoning: String,
    /// Cached result metadata
    pub cache_metadata: Option<CacheMetadata>,
}

/// Metadata for cached query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    /// When the result was cached
    pub cached_at: DateTime<Utc>,
    /// Cache key used
    pub cache_key: String,
    /// Number of times this result was served from cache
    pub hit_count: u64,
}

/// Rule Generalization and Maintenance System
/// 
/// This system provides advanced capabilities for rule abstraction, maintenance,
/// and evolution based on new evidence and patterns.
pub struct RuleGeneralizationSystem {
    /// Core rule database
    rule_database: RuleDatabase,
    /// Configuration for generalization
    config: RuleGeneralizationConfig,
    /// Storage for generalized rules
    generalized_rules: HashMap<Uuid, GeneralizedRule>,
    /// Rule version history
    rule_versions: HashMap<Uuid, Vec<RuleVersion>>,
    /// Update history for tracking changes
    update_history: Vec<RuleUpdate>,
    /// Query result cache
    query_cache: HashMap<String, (EnhancedRuleResult, DateTime<Utc>)>,
    /// Abstraction hierarchy mapping
    abstraction_hierarchy: HashMap<String, Vec<String>>,
    /// Statistics for system performance
    maintenance_stats: MaintenanceStats,
}

/// Statistics for rule maintenance operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceStats {
    /// Total rules generalized
    pub total_generalizations: usize,
    /// Total rule updates processed
    pub total_updates: usize,
    /// Total contradictions resolved
    pub contradictions_resolved: usize,
    /// Total rules deprecated
    pub rules_deprecated: usize,
    /// Total rules reactivated
    pub rules_reactivated: usize,
    /// Average generalization confidence
    pub avg_generalization_confidence: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Update success rate
    pub update_success_rate: f64,
}

impl Default for MaintenanceStats {
    fn default() -> Self {
        Self {
            total_generalizations: 0,
            total_updates: 0,
            contradictions_resolved: 0,
            rules_deprecated: 0,
            rules_reactivated: 0,
            avg_generalization_confidence: 0.0,
            cache_hit_rate: 0.0,
            update_success_rate: 0.0,
        }
    }
}

impl RuleGeneralizationSystem {
    /// Create a new rule generalization system
    pub fn new() -> Self {
        Self {
            rule_database: RuleDatabase::new(),
            config: RuleGeneralizationConfig::default(),
            generalized_rules: HashMap::new(),
            rule_versions: HashMap::new(),
            update_history: Vec::new(),
            query_cache: HashMap::new(),
            abstraction_hierarchy: HashMap::new(),
            maintenance_stats: MaintenanceStats::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: RuleGeneralizationConfig) -> Self {
        let mut system = Self::new();
        system.config = config;
        system
    }

    /// Initialize abstraction hierarchy from concept graph
    pub async fn initialize_abstraction_hierarchy(
        &mut self,
        concept_graph: &ConceptGraphManager,
    ) -> Result<()> {
        // Query concept graph for hierarchical relationships
        let query = crate::concept_graph::RelationshipQuery {
            relationship_type: Some(RelationshipType::IsA),
            ..Default::default()
        };
        let hierarchical_relationships = concept_graph.query_relationships(&query).await?;
        
        for relationship in hierarchical_relationships {
            let child = relationship.source_id;
            let parent = relationship.target_id;
            
            // Convert UUIDs to strings for the abstraction hierarchy
            let child_str = child.to_string();
            let parent_str = parent.to_string();
            
            self.abstraction_hierarchy
                .entry(parent_str)
                .or_insert_with(Vec::new)
                .push(child_str);
        }
        
        Ok(())
    }

    /// Generate generalizations from a set of similar rules
    pub fn generate_generalizations(&mut self, rule_ids: &[Uuid]) -> Result<Vec<Uuid>> {
        if rule_ids.len() < self.config.min_rules_for_generalization {
            return Ok(Vec::new());
        }

        let mut generated_ids = Vec::new();
        
        // Clone the rules to avoid borrowing issues
        let rules: Vec<Rule> = rule_ids.iter()
            .filter_map(|id| self.rule_database.get_rule(*id).cloned())
            .collect();

        if rules.is_empty() {
            return Ok(Vec::new());
        }

        // Convert to references for grouping
        let rule_refs: Vec<&Rule> = rules.iter().collect();
        
        // Group rules by pattern similarity
        let rule_groups = self.group_rules_by_similarity(&rule_refs)?;
        
        // Process each group that meets the minimum size requirement
        for group in rule_groups {
            if group.len() >= self.config.min_rules_for_generalization {
                if let Some(generalized_id) = self.create_generalization(&group)? {
                    generated_ids.push(generalized_id);
                }
            }
        }

        self.maintenance_stats.total_generalizations += generated_ids.len();
        Ok(generated_ids)
    }

    /// Create a generalization from a group of similar rules
    fn create_generalization(&mut self, rules: &[&Rule]) -> Result<Option<Uuid>> {
        if rules.is_empty() {
            return Ok(None);
        }

        // Find common patterns and abstract them
        let generalized_pattern = self.abstract_patterns(rules)?;
        let generalized_outcome = self.abstract_outcomes(rules)?;
        
        // Aggregate metrics from source rules
        let aggregated_metrics = self.aggregate_metrics(rules)?;
        
        // Create abstraction mapping
        let abstraction_mapping = self.create_abstraction_mapping(rules)?;
        
        // Determine abstraction level
        let abstraction_level = self.determine_abstraction_level(&abstraction_mapping)?;
        
        // Calculate generalization confidence
        let generalization_confidence = self.calculate_generalization_confidence(rules, &aggregated_metrics)?;
        
        if generalization_confidence < self.config.generalized_rule_confidence_threshold {
            return Ok(None);
        }

        let generalized_rule = GeneralizedRule {
            id: Uuid::new_v4(),
            generalized_pattern,
            generalized_outcome,
            aggregated_metrics,
            source_rules: rules.iter().map(|r| r.id).collect(),
            abstraction_level,
            abstraction_mapping,
            created_at: Utc::now(),
            generalization_confidence,
            coverage_count: rules.len(),
        };

        let id = generalized_rule.id;
        self.generalized_rules.insert(id, generalized_rule);
        
        Ok(Some(id))
    }

    /// Group rules by pattern similarity
    fn group_rules_by_similarity<'a>(&self, rules: &[&'a Rule]) -> Result<Vec<Vec<&'a Rule>>> {
        let mut groups = Vec::new();
        let mut ungrouped: Vec<&Rule> = rules.to_vec();

        while !ungrouped.is_empty() {
            let base_rule = ungrouped.remove(0);
            let mut current_group = vec![base_rule];
            
            let mut i = 0;
            while i < ungrouped.len() {
                let similarity = self.calculate_rule_similarity(base_rule, ungrouped[i])?;
                if similarity >= self.config.min_similarity_for_merge {
                    current_group.push(ungrouped.remove(i));
                } else {
                    i += 1;
                }
            }
            
            if current_group.len() >= self.config.min_rules_for_generalization {
                groups.push(current_group);
            }
        }

        Ok(groups)
    }

    /// Calculate similarity between two rules
    fn calculate_rule_similarity(&self, rule1: &Rule, rule2: &Rule) -> Result<f64> {
        let pattern_similarity = self.calculate_pattern_similarity_detailed(&rule1.pattern, &rule2.pattern)?;
        let outcome_similarity = self.calculate_outcome_similarity(&rule1.outcome, &rule2.outcome)?;
        let context_similarity = self.calculate_context_similarity(&rule1.context, &rule2.context)?;
        
        // Weighted average of similarity scores
        let similarity = (pattern_similarity * 0.4) + (outcome_similarity * 0.4) + (context_similarity * 0.2);
        Ok(similarity)
    }

    /// Calculate detailed pattern similarity
    fn calculate_pattern_similarity_detailed(&self, pattern1: &RulePattern, pattern2: &RulePattern) -> Result<f64> {
        match (pattern1, pattern2) {
            (RulePattern::Single(s1), RulePattern::Single(s2)) => {
                Ok(if s1 == s2 { 1.0 } else { self.calculate_semantic_similarity(s1, s2)? })
            },
            (RulePattern::Sequence(seq1), RulePattern::Sequence(seq2)) => {
                Ok(self.calculate_string_similarity(&seq1.join(" "), &seq2.join(" ")))
            },
            (RulePattern::CoOccurrence(co1), RulePattern::CoOccurrence(co2)) => {
                Ok(self.calculate_string_similarity(&co1.join(" "), &co2.join(" ")))
            },
            (RulePattern::Temporal { elements: e1, .. }, RulePattern::Temporal { elements: e2, .. }) => {
                Ok(self.calculate_string_similarity(&e1.join(" "), &e2.join(" ")))
            },
            (RulePattern::Hierarchical { parent: p1, children: c1 }, RulePattern::Hierarchical { parent: p2, children: c2 }) => {
                let parent_sim = if p1 == p2 { 1.0 } else { self.calculate_semantic_similarity(p1, p2)? };
                let children_sim = self.calculate_string_similarity(&c1.join(" "), &c2.join(" "));
                Ok((parent_sim + children_sim) / 2.0)
            },
            _ => Ok(0.0), // Different pattern types
        }
    }

    /// Calculate semantic similarity between two strings
    fn calculate_semantic_similarity(&self, s1: &str, s2: &str) -> Result<f64> {
        // Check if one is an abstraction of the other
        if let Some(abstractions) = self.abstraction_hierarchy.get(s1) {
            if abstractions.contains(&s2.to_string()) {
                return Ok(0.8); // High similarity for abstraction relationships
            }
        }
        if let Some(abstractions) = self.abstraction_hierarchy.get(s2) {
            if abstractions.contains(&s1.to_string()) {
                return Ok(0.8);
            }
        }
        
        // Basic string similarity as fallback
        let similarity = self.calculate_string_similarity(s1, s2);
        Ok(similarity)
    }

    /// Calculate basic string similarity using Levenshtein distance
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.len().max(s2.len());
        if max_len == 0 {
            return 1.0;
        }
        
        let distance = levenshtein_distance(s1, s2);
        1.0 - (distance as f64 / max_len as f64)
    }

    /// Calculate outcome similarity
    fn calculate_outcome_similarity(&self, outcome1: &RuleOutcome, outcome2: &RuleOutcome) -> Result<f64> {
        match (outcome1, outcome2) {
            (RuleOutcome::Single(s1), RuleOutcome::Single(s2)) => {
                Ok(if s1 == s2 { 1.0 } else { self.calculate_semantic_similarity(s1, s2)? })
            },
            (RuleOutcome::Action(a1), RuleOutcome::Action(a2)) => {
                Ok(if a1 == a2 { 1.0 } else { self.calculate_semantic_similarity(a1, a2)? })
            },
            (RuleOutcome::StateChange { from_state: f1, to_state: t1 }, 
             RuleOutcome::StateChange { from_state: f2, to_state: t2 }) => {
                let from_sim = self.calculate_semantic_similarity(f1, f2)?;
                let to_sim = self.calculate_semantic_similarity(t1, t2)?;
                Ok((from_sim + to_sim) / 2.0)
            },
            (RuleOutcome::Multiple(m1), RuleOutcome::Multiple(m2)) => {
                // Compare probability distributions
                let mut total_similarity = 0.0;
                let mut count = 0;
                
                for (outcome1, prob1) in m1 {
                    for (outcome2, prob2) in m2 {
                        if outcome1 == outcome2 {
                            total_similarity += (prob1 * prob2).sqrt();
                            count += 1;
                        }
                    }
                }
                
                Ok(if count > 0 { total_similarity / count as f64 } else { 0.0 })
            },
            _ => Ok(0.0), // Different outcome types
        }
    }

    /// Calculate context similarity
    fn calculate_context_similarity(&self, context1: &HashMap<String, String>, context2: &HashMap<String, String>) -> Result<f64> {
        if context1.is_empty() && context2.is_empty() {
            return Ok(1.0);
        }
        
        let all_keys: std::collections::HashSet<_> = context1.keys().chain(context2.keys()).collect();
        let mut matching_keys = 0;
        let mut total_similarity = 0.0;
        
        for key in all_keys {
            match (context1.get(key), context2.get(key)) {
                (Some(v1), Some(v2)) => {
                    if v1 == v2 {
                        total_similarity += 1.0;
                    } else {
                        total_similarity += self.calculate_semantic_similarity(v1, v2)?;
                    }
                    matching_keys += 1;
                },
                (Some(_), None) | (None, Some(_)) => {
                    total_similarity += 0.0; // No similarity for missing keys
                    matching_keys += 1;
                },
                (None, None) => unreachable!(), // Key must exist in at least one map
            }
        }
        
        Ok(if matching_keys > 0 { total_similarity / matching_keys as f64 } else { 0.0 })
    }

    /// Abstract patterns from a group of rules
    fn abstract_patterns(&self, rules: &[&Rule]) -> Result<RulePattern> {
        if rules.is_empty() {
            return Err(anyhow::anyhow!("Cannot abstract patterns from empty rule set"));
        }
        
        // For now, use the first rule's pattern as base and abstract specific elements
        let base_pattern = &rules[0].pattern;
        self.abstract_single_pattern(base_pattern, rules)
    }

    /// Abstract a single pattern based on rule group
    fn abstract_single_pattern(&self, pattern: &RulePattern, rules: &[&Rule]) -> Result<RulePattern> {
        match pattern {
            RulePattern::Single(element) => {
                let abstracted = self.find_abstraction(element, rules)?;
                Ok(RulePattern::Single(abstracted))
            },
            RulePattern::Sequence(elements) => {
                let abstracted: Result<Vec<_>> = elements.iter()
                    .map(|e| self.find_abstraction(e, rules))
                    .collect();
                Ok(RulePattern::Sequence(abstracted?))
            },
            RulePattern::CoOccurrence(elements) => {
                let abstracted: Result<Vec<_>> = elements.iter()
                    .map(|e| self.find_abstraction(e, rules))
                    .collect();
                Ok(RulePattern::CoOccurrence(abstracted?))
            },
            RulePattern::Temporal { elements, timing_constraints } => {
                let abstracted: Result<Vec<_>> = elements.iter()
                    .map(|e| self.find_abstraction(e, rules))
                    .collect();
                Ok(RulePattern::Temporal {
                    elements: abstracted?,
                    timing_constraints: timing_constraints.clone(),
                })
            },
            RulePattern::Hierarchical { parent, children } => {
                let abstracted_parent = self.find_abstraction(parent, rules)?;
                let abstracted_children: Result<Vec<_>> = children.iter()
                    .map(|c| self.find_abstraction(c, rules))
                    .collect();
                Ok(RulePattern::Hierarchical {
                    parent: abstracted_parent,
                    children: abstracted_children?,
                })
            },
            RulePattern::Conditional { condition, context } => {
                let abstracted_condition = self.find_abstraction(condition, rules)?;
                // Context abstraction could be more complex, but for now keep it simple
                Ok(RulePattern::Conditional {
                    condition: abstracted_condition,
                    context: context.clone(),
                })
            },
        }
    }

    /// Find appropriate abstraction for an element
    fn find_abstraction(&self, element: &str, rules: &[&Rule]) -> Result<String> {
        // Collect all variants of this element across rules
        let mut variants = Vec::new();
        
        for rule in rules {
            if let Some(rule_variants) = self.extract_element_variants(&rule.pattern, element) {
                variants.extend(rule_variants);
            }
        }
        
        // If we have multiple variants, find their common abstraction
        if variants.len() > 1 {
            if let Some(abstraction) = self.find_common_abstraction(&variants) {
                return Ok(abstraction);
            }
        }
        
        // If no abstraction found, return the original element
        Ok(element.to_string())
    }

    /// Extract variants of an element from a pattern
    fn extract_element_variants(&self, pattern: &RulePattern, target_element: &str) -> Option<Vec<String>> {
        match pattern {
            RulePattern::Single(element) => {
                if element == target_element {
                    Some(vec![element.clone()])
                } else {
                    None
                }
            },
            RulePattern::Sequence(elements) | 
            RulePattern::CoOccurrence(elements) => {
                let mut variants = Vec::new();
                for element in elements {
                    if self.calculate_string_similarity(element, target_element) > 0.7 {
                        variants.push(element.clone());
                    }
                }
                if variants.is_empty() { None } else { Some(variants) }
            },
            RulePattern::Temporal { elements, .. } => {
                let mut variants = Vec::new();
                for element in elements {
                    if self.calculate_string_similarity(element, target_element) > 0.7 {
                        variants.push(element.clone());
                    }
                }
                if variants.is_empty() { None } else { Some(variants) }
            },
            RulePattern::Hierarchical { parent, children } => {
                let mut variants = Vec::new();
                if self.calculate_string_similarity(parent, target_element) > 0.7 {
                    variants.push(parent.clone());
                }
                for child in children {
                    if self.calculate_string_similarity(child, target_element) > 0.7 {
                        variants.push(child.clone());
                    }
                }
                if variants.is_empty() { None } else { Some(variants) }
            },
            RulePattern::Conditional { condition, .. } => {
                if self.calculate_string_similarity(condition, target_element) > 0.7 {
                    Some(vec![condition.clone()])
                } else {
                    None
                }
            },
        }
    }

    /// Find common abstraction for a set of variants
    fn find_common_abstraction(&self, variants: &[String]) -> Option<String> {
        // Look for a common parent in the abstraction hierarchy
        for variant in variants {
            if let Some(abstractions) = self.abstraction_hierarchy.get(variant) {
                for abstraction in abstractions {
                    // Check if this abstraction covers most variants
                    let coverage = self.calculate_abstraction_coverage(abstraction, variants);
                    if coverage >= 0.7 {
                        return Some(abstraction.clone());
                    }
                }
            }
        }
        
        // If no hierarchy-based abstraction, create a generic one
        if variants.len() >= 3 {
            Some(format!("{}*", self.find_common_prefix(variants)))
        } else {
            None
        }
    }

    /// Calculate how well an abstraction covers a set of variants
    fn calculate_abstraction_coverage(&self, abstraction: &str, variants: &[String]) -> f64 {
        let mut covered = 0;
        
        for variant in variants {
            if let Some(hierarchy) = self.abstraction_hierarchy.get(abstraction) {
                if hierarchy.contains(variant) {
                    covered += 1;
                }
            }
        }
        
        covered as f64 / variants.len() as f64
    }

    /// Find common prefix of strings
    fn find_common_prefix(&self, strings: &[String]) -> String {
        if strings.is_empty() {
            return String::new();
        }
        
        let mut prefix = strings[0].clone();
        
        for string in strings.iter().skip(1) {
            let mut common_len = 0;
            for (c1, c2) in prefix.chars().zip(string.chars()) {
                if c1 == c2 {
                    common_len += c1.len_utf8();
                } else {
                    break;
                }
            }
            prefix.truncate(common_len);
        }
        
        prefix
    }

    /// Abstract outcomes from a group of rules
    fn abstract_outcomes(&self, rules: &[&Rule]) -> Result<RuleOutcome> {
        if rules.is_empty() {
            return Err(anyhow::anyhow!("Cannot abstract outcomes from empty rule set"));
        }
        
        // Collect all outcomes
        let outcomes: Vec<&RuleOutcome> = rules.iter().map(|r| &r.outcome).collect();
        
        // If all outcomes are the same type, try to abstract them
        if let Some(first_outcome) = outcomes.first() {
            match first_outcome {
                RuleOutcome::Single(_) => {
                    let singles: Vec<&str> = outcomes.iter().filter_map(|o| {
                        if let RuleOutcome::Single(s) = o { Some(s.as_str()) } else { None }
                    }).collect();
                    
                    if singles.len() == outcomes.len() {
                        if let Some(abstraction) = self.find_common_abstraction(&singles.iter().map(|s| s.to_string()).collect::<Vec<_>>()) {
                            return Ok(RuleOutcome::Single(abstraction));
                        }
                    }
                },
                RuleOutcome::Action(_) => {
                    let actions: Vec<&str> = outcomes.iter().filter_map(|o| {
                        if let RuleOutcome::Action(a) = o { Some(a.as_str()) } else { None }
                    }).collect();
                    
                    if actions.len() == outcomes.len() {
                        if let Some(abstraction) = self.find_common_abstraction(&actions.iter().map(|s| s.to_string()).collect::<Vec<_>>()) {
                            return Ok(RuleOutcome::Action(abstraction));
                        }
                    }
                },
                _ => {
                    // For complex outcomes, use the first one as template
                    return Ok((*first_outcome).clone());
                }
            }
        }
        
        // If no abstraction possible, create a multiple outcome
        if rules.len() > 1 {
            let mut outcome_map = HashMap::new();
            
            for rule in rules {
                let outcome_key = match &rule.outcome {
                    RuleOutcome::Single(s) => s.clone(),
                    RuleOutcome::Action(a) => a.clone(),
                    RuleOutcome::StateChange { to_state, .. } => to_state.clone(),
                    RuleOutcome::Negation(n) => format!("NOT_{}", n),
                    RuleOutcome::Multiple(m) => {
                        if let Some((first_outcome, _)) = m.first() {
                            first_outcome.clone()
                        } else {
                            "unknown".to_string()
                        }
                    }
                };
                
                *outcome_map.entry(outcome_key).or_insert(0.0) += 1.0 / rules.len() as f64;
            }
            
            let multiple_outcomes: Vec<(String, f64)> = outcome_map.into_iter().collect();
            Ok(RuleOutcome::Multiple(multiple_outcomes))
        } else {
            Ok(rules[0].outcome.clone())
        }
    }

    /// Aggregate metrics from multiple rules
    fn aggregate_metrics(&self, rules: &[&Rule]) -> Result<RuleMetrics> {
        if rules.is_empty() {
            return Err(anyhow::anyhow!("Cannot aggregate metrics from empty rule set"));
        }
        
        let mut total_support = 0.0;
        let mut total_confidence = 0.0;
        let mut total_generality = 0.0;
        let mut total_reusability = 0.0;
        let mut total_lift = 0.0;
        let mut total_conviction = 0.0;
        
        for rule in rules {
            total_support += rule.metrics.support;
            total_confidence += rule.metrics.confidence;
            total_generality += rule.metrics.generality;
            total_reusability += rule.metrics.reusability;
            total_lift += rule.metrics.lift;
            total_conviction += rule.metrics.conviction;
        }
        
        let count = rules.len() as f64;
        
        Ok(RuleMetrics {
            support: total_support / count,
            confidence: total_confidence / count,
            generality: total_generality / count,
            reusability: total_reusability / count,
            lift: total_lift / count,
            conviction: total_conviction / count,
        })
    }

    /// Create abstraction mapping
    fn create_abstraction_mapping(&self, rules: &[&Rule]) -> Result<HashMap<String, Vec<String>>> {
        let mut mapping = HashMap::new();
        
        for rule in rules {
            let concrete_elements = self.extract_concrete_elements(&rule.pattern);
            for element in concrete_elements {
                if let Some(abstractions) = self.abstraction_hierarchy.get(&element) {
                    for abstraction in abstractions {
                        mapping.entry(abstraction.clone())
                            .or_insert_with(Vec::new)
                            .push(element.clone());
                    }
                }
            }
        }
        
        Ok(mapping)
    }

    /// Extract concrete elements from a pattern
    fn extract_concrete_elements(&self, pattern: &RulePattern) -> Vec<String> {
        match pattern {
            RulePattern::Single(element) => vec![element.clone()],
            RulePattern::Sequence(elements) | 
            RulePattern::CoOccurrence(elements) => elements.clone(),
            RulePattern::Temporal { elements, .. } => elements.clone(),
            RulePattern::Hierarchical { parent, children } => {
                let mut elements = vec![parent.clone()];
                elements.extend(children.clone());
                elements
            },
            RulePattern::Conditional { condition, .. } => vec![condition.clone()],
        }
    }

    /// Determine abstraction level
    fn determine_abstraction_level(&self, mapping: &HashMap<String, Vec<String>>) -> Result<AbstractionLevel> {
        let max_abstractions = mapping.values().map(|v| v.len()).max().unwrap_or(0);
        
        match max_abstractions {
            0..=1 => Ok(AbstractionLevel::Specific),
            2..=3 => Ok(AbstractionLevel::Category),
            4..=7 => Ok(AbstractionLevel::Type),
            _ => Ok(AbstractionLevel::Concept),
        }
    }

    /// Calculate confidence in generalization
    fn calculate_generalization_confidence(&self, rules: &[&Rule], metrics: &RuleMetrics) -> Result<f64> {
        if rules.is_empty() {
            return Ok(0.0);
        }
        
        // Base confidence on aggregated metrics
        let metric_confidence = (metrics.support + metrics.confidence + metrics.generality) / 3.0;
        
        // Adjust based on rule count (more rules = higher confidence)
        let count_bonus = (rules.len() as f64).ln() / 10.0; // Logarithmic scaling
        
        // Adjust based on rule quality variance
        let confidence_variance = self.calculate_confidence_variance(rules);
        let variance_penalty = confidence_variance * 0.5; // Penalize high variance
        
        let final_confidence = (metric_confidence + count_bonus - variance_penalty).max(0.0).min(1.0);
        
        Ok(final_confidence)
    }

    /// Calculate variance in rule confidence scores
    fn calculate_confidence_variance(&self, rules: &[&Rule]) -> f64 {
        if rules.len() <= 1 {
            return 0.0;
        }
        
        let mean: f64 = rules.iter().map(|r| r.metrics.confidence).sum::<f64>() / rules.len() as f64;
        let variance: f64 = rules.iter()
            .map(|r| (r.metrics.confidence - mean).powi(2))
            .sum::<f64>() / rules.len() as f64;
        
                 variance.sqrt()
     }

    /// Update a rule with new evidence
    pub fn update_rule_with_evidence(
        &mut self,
        rule_id: Uuid,
        evidence: Vec<Uuid>,
        success: bool,
        new_support: f64,
        new_confidence: f64,
    ) -> Result<()> {
        // Get the current rule
        let rule = self.rule_database.get_rule(rule_id)
            .ok_or_else(|| anyhow::anyhow!("Rule not found: {}", rule_id))?
            .clone();

        // Create version snapshot before update
        let version = RuleVersion {
            version: rule.version,
            rule_snapshot: rule.clone(),
            update_info: RuleUpdate {
                id: Uuid::new_v4(),
                rule_id,
                update_type: if success { UpdateType::ConfidenceUpdate } else { UpdateType::SupportUpdate },
                evidence: evidence.clone(),
                previous_metrics: rule.metrics.clone(),
                updated_metrics: RuleMetrics::new(new_support, new_confidence), // Will be updated below
                timestamp: Utc::now(),
                update_confidence: if success { 0.8 } else { 0.6 },
                description: format!("Evidence update: success={}, support={:.3}, confidence={:.3}", 
                                   success, new_support, new_confidence),
            },
            created_at: Utc::now(),
            is_deprecated: false,
        };

        // Store version history
        self.rule_versions.entry(rule_id)
            .or_insert_with(Vec::new)
            .push(version.clone());

        // Trim version history if too long
        if let Some(versions) = self.rule_versions.get_mut(&rule_id) {
            if versions.len() > self.config.max_rule_versions as usize {
                versions.remove(0); // Remove oldest version
            }
        }

        // Update the rule
        let mut updated_rule = rule;
        updated_rule.update_with_evidence(success, new_support, new_confidence);
        updated_rule.version += 1;

        // Check if rule should be deprecated
        if updated_rule.should_deprecate(&self.rule_database.get_config()) {
            updated_rule.is_active = false;
            self.maintenance_stats.rules_deprecated += 1;
        }

        // Update in database
        self.rule_database.update_rule(updated_rule)?;

        // Add to update history
        self.update_history.push(version.update_info);
        self.maintenance_stats.total_updates += 1;

        Ok(())
    }

    /// Detect contradictions between rules
    pub fn detect_contradictions(&self) -> Result<ContradictionResult> {
        let active_rules: Vec<&Rule> = self.rule_database.get_active_rules();
        let mut contradictory_pairs = Vec::new();
        let mut contradiction_severity = HashMap::new();
        let mut resolution_recommendations = HashMap::new();

        // Compare each pair of rules
        for i in 0..active_rules.len() {
            for j in i + 1..active_rules.len() {
                let rule1 = active_rules[i];
                let rule2 = active_rules[j];

                if let Ok(comparison) = self.compare_rules_for_contradiction(rule1, rule2) {
                    if comparison.outcomes_contradict {
                        let pair = (rule1.id, rule2.id);
                        contradictory_pairs.push(pair);
                        
                        // Calculate severity based on rule confidence and overlap
                        let severity = self.calculate_contradiction_severity(rule1, rule2, &comparison)?;
                        contradiction_severity.insert(pair, severity);

                        // Determine resolution recommendation
                        let recommendation = self.determine_contradiction_resolution(rule1, rule2, &comparison)?;
                        resolution_recommendations.insert(pair, recommendation);
                    }
                }
            }
        }

        // Identify high-priority contradictions
        let high_priority_contradictions: Vec<(Uuid, Uuid)> = contradictory_pairs.iter()
            .filter(|pair| {
                *contradiction_severity.get(pair).unwrap_or(&0.0) > 0.7
            })
            .copied()
            .collect();

        Ok(ContradictionResult {
            total_contradictions: contradictory_pairs.len(),
            contradictory_pairs,
            contradiction_severity,
            resolution_recommendations,
            high_priority_contradictions,
        })
    }

    /// Compare two rules for contradiction
    fn compare_rules_for_contradiction(&self, rule1: &Rule, rule2: &Rule) -> Result<RuleComparison> {
        let pattern_similarity = self.calculate_pattern_similarity_detailed(&rule1.pattern, &rule2.pattern)?;
        let outcomes_contradict = self.check_outcome_contradiction(&rule1.outcome, &rule2.outcome);
        let context_overlap = self.calculate_context_similarity(&rule1.context, &rule2.context)?;

        let recommendation = if outcomes_contradict && pattern_similarity > 0.7 {
            if rule1.metrics.confidence > rule2.metrics.confidence {
                ComparisonRecommendation::KeepHigherConfidence
            } else if rule1.evidence_count() > rule2.evidence_count() {
                ComparisonRecommendation::KeepMoreEvidence
            } else {
                ComparisonRecommendation::ManualReview
            }
        } else {
            ComparisonRecommendation::Compatible
        };

        Ok(RuleComparison {
            rule1_id: rule1.id,
            rule2_id: rule2.id,
            pattern_similarity,
            outcomes_contradict,
            context_overlap,
            recommendation,
        })
    }

    /// Calculate severity of contradiction
    fn calculate_contradiction_severity(&self, rule1: &Rule, rule2: &Rule, comparison: &RuleComparison) -> Result<f64> {
        if !comparison.outcomes_contradict {
            return Ok(0.0);
        }

        // Base severity on pattern similarity (more similar patterns = more severe contradiction)
        let pattern_severity = comparison.pattern_similarity;
        
        // Adjust for rule confidence (higher confidence rules create higher severity)
        let confidence_factor = (rule1.metrics.confidence + rule2.metrics.confidence) / 2.0;
        
        // Adjust for context overlap (overlapping contexts make contradiction more severe)
        let context_factor = comparison.context_overlap;
        
        // Adjust for rule usage (frequently used contradictory rules are more severe)
        let usage_factor = ((rule1.usage_count + rule2.usage_count) as f64).ln() / 10.0;

        let severity = (pattern_severity * 0.4 + confidence_factor * 0.3 + context_factor * 0.2 + usage_factor * 0.1)
            .max(0.0).min(1.0);

        Ok(severity)
    }

    /// Determine how to resolve a contradiction
    fn determine_contradiction_resolution(&self, rule1: &Rule, rule2: &Rule, comparison: &RuleComparison) -> Result<ContradictionResolution> {
        if !comparison.outcomes_contradict {
            return Ok(ContradictionResolution::KeepBoth);
        }

        // High pattern similarity with contradictory outcomes
        if comparison.pattern_similarity > 0.9 {
            if (rule1.metrics.confidence - rule2.metrics.confidence).abs() > 0.2 {
                return Ok(ContradictionResolution::KeepHigherConfidence);
            } else if rule1.evidence_count() != rule2.evidence_count() {
                return Ok(ContradictionResolution::KeepMoreEvidence);
            } else {
                return Ok(ContradictionResolution::CreateConditionalRule);
            }
        }

        // Moderate similarity might be generalizable
        if comparison.pattern_similarity > 0.6 && comparison.pattern_similarity <= 0.9 {
            return Ok(ContradictionResolution::GeneralizeToResolve);
        }

        // Low confidence rules should be deprecated
        if rule1.metrics.confidence < self.config.contradiction_deprecation_threshold &&
           rule2.metrics.confidence < self.config.contradiction_deprecation_threshold {
            return Ok(ContradictionResolution::DeprecateBoth);
        }

        // Default to manual review for complex cases
        Ok(ContradictionResolution::ManualReview)
    }

    /// Check if outcomes contradict each other
    fn check_outcome_contradiction(&self, outcome1: &RuleOutcome, outcome2: &RuleOutcome) -> bool {
        match (outcome1, outcome2) {
            (RuleOutcome::Single(s1), RuleOutcome::Negation(n1)) => s1 == n1,
            (RuleOutcome::Negation(n1), RuleOutcome::Single(s1)) => n1 == s1,
            (RuleOutcome::Action(a1), RuleOutcome::Negation(n1)) => a1 == n1,
            (RuleOutcome::Negation(n1), RuleOutcome::Action(a1)) => n1 == a1,
            (RuleOutcome::StateChange { to_state: t1, .. }, RuleOutcome::StateChange { to_state: t2, .. }) => {
                // Contradictory if they lead to different end states
                t1 != t2
            },
            _ => false, // Most outcomes don't directly contradict
        }
    }

    /// Resolve contradictions automatically if configured
    pub fn auto_resolve_contradictions(&mut self) -> Result<usize> {
        if !self.config.auto_resolve_contradictions {
            return Ok(0);
        }

        let contradiction_result = self.detect_contradictions()?;
        let mut resolved_count = 0;

        for (rule1_id, rule2_id) in contradiction_result.contradictory_pairs {
            if let Some(recommendation) = contradiction_result.resolution_recommendations.get(&(rule1_id, rule2_id)) {
                match recommendation {
                    ContradictionResolution::KeepHigherConfidence => {
                        let rule1 = self.rule_database.get_rule(rule1_id);
                        let rule2 = self.rule_database.get_rule(rule2_id);
                        
                        if let (Some(r1), Some(r2)) = (rule1, rule2) {
                            if r1.metrics.confidence < r2.metrics.confidence {
                                self.deprecate_rule(rule1_id, "Contradicts higher confidence rule")?;
                            } else {
                                self.deprecate_rule(rule2_id, "Contradicts higher confidence rule")?;
                            }
                            resolved_count += 1;
                        }
                    },
                    ContradictionResolution::KeepMoreEvidence => {
                        let rule1 = self.rule_database.get_rule(rule1_id);
                        let rule2 = self.rule_database.get_rule(rule2_id);
                        
                        if let (Some(r1), Some(r2)) = (rule1, rule2) {
                            if r1.evidence_count() < r2.evidence_count() {
                                self.deprecate_rule(rule1_id, "Contradicts rule with more evidence")?;
                            } else {
                                self.deprecate_rule(rule2_id, "Contradicts rule with more evidence")?;
                            }
                            resolved_count += 1;
                        }
                    },
                    ContradictionResolution::DeprecateBoth => {
                        self.deprecate_rule(rule1_id, "Low confidence contradictory rule")?;
                        self.deprecate_rule(rule2_id, "Low confidence contradictory rule")?;
                        resolved_count += 1;
                    },
                    _ => {
                        // Other resolution types require manual intervention
                    }
                }
            }
        }

        self.maintenance_stats.contradictions_resolved += resolved_count;
        Ok(resolved_count)
    }

    /// Deprecate a rule
    fn deprecate_rule(&mut self, rule_id: Uuid, reason: &str) -> Result<()> {
        if let Some(mut rule) = self.rule_database.get_rule(rule_id).cloned() {
            rule.is_active = false;
            
            // Create update record
            let update = RuleUpdate {
                id: Uuid::new_v4(),
                rule_id,
                update_type: UpdateType::Deprecation,
                evidence: Vec::new(),
                previous_metrics: rule.metrics.clone(),
                updated_metrics: rule.metrics.clone(),
                timestamp: Utc::now(),
                update_confidence: 1.0, // Full confidence in deprecation
                description: reason.to_string(),
            };

            self.update_history.push(update);
            self.rule_database.update_rule(rule)?;
            self.maintenance_stats.rules_deprecated += 1;
        }
        Ok(())
    }

    /// Enhanced query interface for external systems
    pub fn query_rules_enhanced(&mut self, query_interface: RuleQueryInterface) -> Result<Vec<EnhancedRuleResult>> {
        // Check cache first
        let cache_key = self.generate_cache_key(&query_interface);
        
        if query_interface.smart_caching {
            if let Some((cached_result, cached_at)) = self.query_cache.get(&cache_key) {
                let age_minutes = (Utc::now() - *cached_at).num_minutes() as u64;
                if age_minutes <= query_interface.max_cache_age_minutes {
                    // Return cached result (simplified - would need to handle multiple results)
                    self.maintenance_stats.cache_hit_rate = 
                        (self.maintenance_stats.cache_hit_rate * 0.9) + (1.0 * 0.1);
                    return Ok(vec![cached_result.clone()]);
                }
            }
        }

        // Perform query
        let rules = self.rule_database.query_rules(&query_interface.query);
        let mut enhanced_results = Vec::new();

        for rule in rules {
            let match_confidence = self.calculate_query_match_confidence(rule, &query_interface.query)?;
            let related_rules = self.find_related_rules(rule.id)?;
            let selection_reasoning = self.generate_selection_reasoning(rule, &query_interface.query);

            let cache_metadata = if query_interface.smart_caching {
                Some(CacheMetadata {
                    cached_at: Utc::now(),
                    cache_key: cache_key.clone(),
                    hit_count: 0,
                })
            } else {
                None
            };

            enhanced_results.push(EnhancedRuleResult {
                rule: rule.clone(),
                match_confidence,
                related_rules,
                selection_reasoning,
                cache_metadata,
            });
        }

        // Sort by match confidence
        enhanced_results.sort_by(|a, b| b.match_confidence.partial_cmp(&a.match_confidence).unwrap_or(std::cmp::Ordering::Equal));

        // Cache the first result if caching is enabled
        if query_interface.smart_caching && !enhanced_results.is_empty() {
            self.query_cache.insert(cache_key, (enhanced_results[0].clone(), Utc::now()));
        }

        Ok(enhanced_results)
    }

    /// Generate cache key for query
    fn generate_cache_key(&self, query_interface: &RuleQueryInterface) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        format!("{:?}", query_interface.query).hash(&mut hasher);
        format!("query_{}", hasher.finish())
    }

    /// Calculate how well a rule matches a query
    fn calculate_query_match_confidence(&self, rule: &Rule, query: &RuleQuery) -> Result<f64> {
        let mut confidence = 1.0;

        // Check confidence threshold
        if let Some(min_confidence) = query.min_confidence {
            if rule.metrics.confidence < min_confidence {
                confidence *= 0.5; // Reduce confidence for not meeting threshold
            }
        }

        // Check support threshold
        if let Some(min_support) = query.min_support {
            if rule.metrics.support < min_support {
                confidence *= 0.7; // Reduce confidence for not meeting support threshold
            }
        }

        // Check context filters
        for (key, value) in &query.context_filters {
            if let Some(rule_value) = rule.context.get(key) {
                if rule_value != value {
                    confidence *= 0.8; // Reduce confidence for context mismatch
                }
            } else {
                confidence *= 0.6; // Reduce confidence for missing context
            }
        }

        // Check active status
        if query.active_only && !rule.is_active {
            confidence = 0.0; // No confidence for inactive rules when active_only is set
        }

        Ok(confidence)
    }

    /// Find rules related to a given rule
    fn find_related_rules(&self, rule_id: Uuid) -> Result<Vec<Uuid>> {
        let target_rule = self.rule_database.get_rule(rule_id)
            .ok_or_else(|| anyhow::anyhow!("Rule not found: {}", rule_id))?;

        let all_rules = self.rule_database.get_active_rules();
        let mut related = Vec::new();

        for rule in all_rules {
            if rule.id != rule_id {
                let similarity = self.calculate_rule_similarity(target_rule, rule)?;
                if similarity > 0.6 { // Threshold for "related"
                    related.push(rule.id);
                }
            }
        }

        // Limit to top 5 most related
        related.sort_by(|a, b| {
            let sim_a = self.rule_database.get_rule(*a).map(|r| 
                self.calculate_rule_similarity(target_rule, r).unwrap_or(0.0)
            ).unwrap_or(0.0);
            let sim_b = self.rule_database.get_rule(*b).map(|r| 
                self.calculate_rule_similarity(target_rule, r).unwrap_or(0.0)
            ).unwrap_or(0.0);
            sim_b.partial_cmp(&sim_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        related.truncate(5);

        Ok(related)
    }

    /// Generate reasoning for why a rule was selected
    fn generate_selection_reasoning(&self, rule: &Rule, query: &RuleQuery) -> String {
        let mut reasons = Vec::new();

        if let Some(min_confidence) = query.min_confidence {
            if rule.metrics.confidence >= min_confidence {
                reasons.push(format!("Meets confidence threshold ({:.2} >= {:.2})", 
                                   rule.metrics.confidence, min_confidence));
            }
        }

        if let Some(min_support) = query.min_support {
            if rule.metrics.support >= min_support {
                reasons.push(format!("Meets support threshold ({:.2} >= {:.2})", 
                                   rule.metrics.support, min_support));
            }
        }

        if !query.context_filters.is_empty() {
            let matching_contexts = query.context_filters.iter()
                .filter(|(key, value)| rule.context.get(*key) == Some(value))
                .count();
            if matching_contexts > 0 {
                reasons.push(format!("Matches {} context filter(s)", matching_contexts));
            }
        }

        if rule.usage_count > 10 {
            reasons.push(format!("Frequently used rule ({} applications)", rule.usage_count));
        }

        if rule.success_rate() > 0.8 {
            reasons.push(format!("High success rate ({:.1}%)", rule.success_rate() * 100.0));
        }

        if reasons.is_empty() {
            "General rule match".to_string()
        } else {
            reasons.join("; ")
        }
    }

    /// Get maintenance statistics
    pub fn get_maintenance_stats(&self) -> &MaintenanceStats {
        &self.maintenance_stats
    }

    /// Get rule database
    pub fn get_rule_database(&self) -> &RuleDatabase {
        &self.rule_database
    }

    /// Get generalized rules
    pub fn get_generalized_rules(&self) -> &HashMap<Uuid, GeneralizedRule> {
        &self.generalized_rules
    }

    /// Get rule versions
    pub fn get_rule_versions(&self, rule_id: Uuid) -> Option<&Vec<RuleVersion>> {
        self.rule_versions.get(&rule_id)
    }

    /// Get update history
    pub fn get_update_history(&self) -> &[RuleUpdate] {
        &self.update_history
    }

    /// Clear query cache
    pub fn clear_query_cache(&mut self) {
        self.query_cache.clear();
    }

    /// Update configuration
    pub fn set_config(&mut self, config: RuleGeneralizationConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &RuleGeneralizationConfig {
        &self.config
    }
}

impl Default for RuleGeneralizationSystem {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function for calculating Levenshtein distance
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let s1_len = s1_chars.len();
    let s2_len = s2_chars.len();

    if s1_len == 0 {
        return s2_len;
    }
    if s2_len == 0 {
        return s1_len;
    }

    let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];

    // Initialize first row and column
    for i in 0..=s1_len {
        matrix[i][0] = i;
    }
    for j in 0..=s2_len {
        matrix[0][j] = j;
    }

    // Fill the matrix
    for i in 1..=s1_len {
        for j in 1..=s2_len {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,    // deletion
                    matrix[i][j - 1] + 1,    // insertion
                ),
                matrix[i - 1][j - 1] + cost, // substitution
            );
        }
    }

    matrix[s1_len][s2_len]
}

// Helper trait to get evidence count from Rule
trait EvidenceCount {
    fn evidence_count(&self) -> usize;
}

impl EvidenceCount for Rule {
    fn evidence_count(&self) -> usize {
        // This is a placeholder - the actual implementation depends on how evidence is stored
        // For now, use usage_count as a proxy
        self.usage_count as usize
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

    // ============================================================================
    // Task 5.2: Rule Formalization Framework Tests
    // ============================================================================

    #[test]
    fn test_rule_formalization_config_creation() {
        let config = RuleFormalizationConfig::default();
        assert_eq!(config.min_pattern_frequency_for_rule, 3);
        assert_eq!(config.min_rule_confidence, 0.7);
        assert_eq!(config.max_rules_per_batch, 50);
        assert!(config.enable_contradiction_detection);
    }

    #[test]
    fn test_rule_metrics_creation_and_update() {
        let mut metrics = RuleMetrics::new(0.6, 0.8);
        assert_eq!(metrics.support, 0.6);
        assert_eq!(metrics.confidence, 0.8);
        assert_eq!(metrics.generality, 0.5);
        assert_eq!(metrics.reusability, 0.0);

        metrics.update(0.7, 0.9, 5);
        assert_eq!(metrics.support, 0.7);
        assert_eq!(metrics.confidence, 0.9);
        assert!(metrics.reusability > 0.0);
    }

    #[test]
    fn test_rule_creation_from_pattern() {
        let pattern = DetectedPattern::new(
            PatternType::TemporalSequence,
            vec!["event1".to_string(), "event2".to_string()],
            5,
            0.8,
            vec![Uuid::new_v4()],
        );

        let outcome = RuleOutcome::Single("result".to_string());
        let rule = Rule::from_pattern(&pattern, outcome);

        assert_eq!(rule.source_patterns, vec![pattern.id]);
        assert_eq!(rule.metrics.confidence, 0.8);
        assert_eq!(rule.usage_count, 0);
        assert!(rule.is_active);
        assert_eq!(rule.version, 1);
    }

    #[test]
    fn test_rule_pattern_conversion() {
        // Test temporal sequence pattern
        let temporal_pattern = DetectedPattern::new(
            PatternType::TemporalSequence,
            vec!["A".to_string(), "B".to_string()],
            3,
            0.7,
            vec![Uuid::new_v4()],
        );
        let rule = Rule::from_pattern(&temporal_pattern, RuleOutcome::Single("C".to_string()));
        
        match rule.pattern {
            RulePattern::Sequence(elements) => {
                assert_eq!(elements, vec!["A".to_string(), "B".to_string()]);
            }
            _ => panic!("Expected Sequence pattern"),
        }

        // Test co-occurrence pattern
        let cooccurrence_pattern = DetectedPattern::new(
            PatternType::CoOccurrence,
            vec!["X".to_string(), "Y".to_string()],
            4,
            0.6,
            vec![Uuid::new_v4()],
        );
        let rule2 = Rule::from_pattern(&cooccurrence_pattern, RuleOutcome::Single("Z".to_string()));
        
        match rule2.pattern {
            RulePattern::CoOccurrence(elements) => {
                assert_eq!(elements, vec!["X".to_string(), "Y".to_string()]);
            }
            _ => panic!("Expected CoOccurrence pattern"),
        }
    }

    #[test]
    fn test_rule_evidence_update() {
        let pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["frequent_event".to_string()],
            3,
            0.7,
            vec![Uuid::new_v4()],
        );

        let mut rule = Rule::from_pattern(&pattern, RuleOutcome::Single("outcome".to_string()));
        let initial_version = rule.version;

        rule.update_with_evidence(true, 0.8, 0.9);
        assert_eq!(rule.usage_count, 1);
        assert_eq!(rule.success_count, 1);
        assert_eq!(rule.version, initial_version + 1);
        assert_eq!(rule.success_rate(), 1.0);

        rule.update_with_evidence(false, 0.7, 0.8);
        assert_eq!(rule.usage_count, 2);
        assert_eq!(rule.success_count, 1);
        assert_eq!(rule.success_rate(), 0.5);
    }

    #[test]
    fn test_rule_deprecation_check() {
        let config = RuleFormalizationConfig::default();
        let pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["test".to_string()],
            3,
            0.2, // Below deprecation threshold
            vec![Uuid::new_v4()],
        );

        let rule = Rule::from_pattern(&pattern, RuleOutcome::Single("outcome".to_string()));
        assert!(rule.should_deprecate(&config));

        let pattern2 = DetectedPattern::new(
            PatternType::Frequency,
            vec!["test".to_string()],
            3,
            0.8, // Above deprecation threshold
            vec![Uuid::new_v4()],
        );

        let rule2 = Rule::from_pattern(&pattern2, RuleOutcome::Single("outcome".to_string()));
        assert!(!rule2.should_deprecate(&config));
    }

    #[test]
    fn test_rule_database_operations() {
        let mut db = RuleDatabase::new();
        
        let pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["test_event".to_string()],
            5,
            0.8,
            vec![Uuid::new_v4()],
        );

        let rule = Rule::from_pattern(&pattern, RuleOutcome::Single("test_outcome".to_string()));
        let rule_id = rule.id;

        // Test storage
        let stored_id = db.store_rule(rule).unwrap();
        assert_eq!(stored_id, rule_id);

        // Test retrieval
        let retrieved_rule = db.get_rule(rule_id).unwrap();
        assert_eq!(retrieved_rule.id, rule_id);

        // Test statistics
        let stats = db.get_stats();
        assert_eq!(stats.total_rules, 1);
        assert_eq!(stats.active_rules, 1);
        assert_eq!(stats.deprecated_rules, 0);
    }

    #[test]
    fn test_rule_query_filtering() {
        let mut db = RuleDatabase::new();
        
        // Create rules with different confidence levels
        let high_confidence_pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["high_conf".to_string()],
            5,
            0.9,
            vec![Uuid::new_v4()],
        );
        let high_rule = Rule::from_pattern(&high_confidence_pattern, RuleOutcome::Single("high_outcome".to_string()));
        db.store_rule(high_rule).unwrap();

        let low_confidence_pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["low_conf".to_string()],
            3,
            0.4,
            vec![Uuid::new_v4()],
        );
        let low_rule = Rule::from_pattern(&low_confidence_pattern, RuleOutcome::Single("low_outcome".to_string()));
        db.store_rule(low_rule).unwrap();

        // Test confidence filtering
        let high_conf_query = RuleQuery {
            min_confidence: Some(0.8),
            active_only: true,
            ..Default::default()
        };
        let high_conf_results = db.query_rules(&high_conf_query);
        assert_eq!(high_conf_results.len(), 1);

        // Test limit
        let limited_query = RuleQuery {
            limit: Some(1),
            ..Default::default()
        };
        let limited_results = db.query_rules(&limited_query);
        assert_eq!(limited_results.len(), 1);
    }

    #[test]
    fn test_rule_formalization_engine() {
        // Use more lenient configuration for testing
        let config = RuleFormalizationConfig {
            min_pattern_frequency_for_rule: 3,
            min_rule_confidence: 0.6,
            max_rules_per_batch: 50,
            min_support_threshold: 0.3, // Lower threshold for testing
            min_generality_threshold: 0.3, // Lower threshold for testing
            validation_window_hours: 168,
            enable_contradiction_detection: true,
            deprecation_threshold: 0.3,
        };
        let mut engine = RuleFormalizationEngine::with_config(config);
        
        // Create test patterns with higher frequency to meet thresholds
        let patterns = vec![
            DetectedPattern::new(
                PatternType::TemporalSequence,
                vec!["A".to_string(), "B".to_string()],
                50, // Higher frequency to meet support threshold
                0.8,
                vec![Uuid::new_v4()],
            ),
            DetectedPattern::new(
                PatternType::CoOccurrence,
                vec!["X".to_string(), "Y".to_string()],
                40, // Higher frequency to meet support threshold
                0.7,
                vec![Uuid::new_v4()],
            ),
        ];

        // Test pattern formalization
        let rule_ids = engine.formalize_patterns(&patterns).unwrap();
        assert_eq!(rule_ids.len(), 2);

        // Test rule database access
        let db_stats = engine.get_rule_database().get_stats();
        assert_eq!(db_stats.total_rules, 2);
        assert_eq!(db_stats.active_rules, 2);
    }

    #[test]
    fn test_outcome_creation_from_patterns() {
        let engine = RuleFormalizationEngine::new();

        // Test temporal sequence outcome
        let temporal_pattern = DetectedPattern::new(
            PatternType::TemporalSequence,
            vec!["start".to_string(), "end".to_string()],
            3,
            0.7,
            vec![Uuid::new_v4()],
        );
        let outcome = engine.create_outcome_from_pattern(&temporal_pattern);
        match outcome {
            RuleOutcome::Single(text) => assert_eq!(text, "end"),
            _ => panic!("Expected Single outcome"),
        }

        // Test co-occurrence outcome
        let cooccurrence_pattern = DetectedPattern::new(
            PatternType::CoOccurrence,
            vec!["A".to_string(), "B".to_string()],
            4,
            0.6,
            vec![Uuid::new_v4()],
        );
        let outcome2 = engine.create_outcome_from_pattern(&cooccurrence_pattern);
        match outcome2 {
            RuleOutcome::Multiple(outcomes) => {
                assert_eq!(outcomes.len(), 2);
                assert_eq!(outcomes[0].0, "A");
                assert_eq!(outcomes[1].0, "B");
            }
            _ => panic!("Expected Multiple outcome"),
        }
    }

    #[test]
    fn test_rule_comparison() {
        let mut engine = RuleFormalizationEngine::new();
        
        // Create similar rules
        let pattern1 = DetectedPattern::new(
            PatternType::TemporalSequence,
            vec!["A".to_string(), "B".to_string()],
            5,
            0.8,
            vec![Uuid::new_v4()],
        );
        let rule1 = Rule::from_pattern(&pattern1, RuleOutcome::Single("C".to_string()));
        let rule1_id = rule1.id;
        engine.get_rule_database_mut().store_rule(rule1).unwrap();

        let pattern2 = DetectedPattern::new(
            PatternType::TemporalSequence,
            vec!["A".to_string(), "B".to_string()],
            3,
            0.7,
            vec![Uuid::new_v4()],
        );
        let rule2 = Rule::from_pattern(&pattern2, RuleOutcome::Single("C".to_string()));
        let rule2_id = rule2.id;
        engine.get_rule_database_mut().store_rule(rule2).unwrap();

        // Test rule comparison
        let comparison = engine.compare_rules(rule1_id, rule2_id).unwrap();
        assert!(comparison.pattern_similarity > 0.9);
        assert!(!comparison.outcomes_contradict);
        assert_eq!(comparison.recommendation, ComparisonRecommendation::Merge);
    }

    #[test]
    fn test_pattern_similarity_calculation() {
        let engine = RuleFormalizationEngine::new();

        // Test identical patterns
        let pattern1 = RulePattern::Single("test".to_string());
        let pattern2 = RulePattern::Single("test".to_string());
        let similarity = engine.calculate_pattern_similarity(&pattern1, &pattern2);
        assert_eq!(similarity, 1.0);

        // Test different patterns
        let pattern3 = RulePattern::Single("different".to_string());
        let similarity2 = engine.calculate_pattern_similarity(&pattern1, &pattern3);
        assert_eq!(similarity2, 0.0);

        // Test sequence similarity
        let seq1 = RulePattern::Sequence(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
        let seq2 = RulePattern::Sequence(vec!["A".to_string(), "B".to_string()]);
        let seq_similarity = engine.calculate_pattern_similarity(&seq1, &seq2);
        assert!(seq_similarity > 0.0 && seq_similarity < 1.0);
    }

    #[test]
    fn test_outcome_contradiction_detection() {
        let engine = RuleFormalizationEngine::new();

        // Test contradictory outcomes
        let outcome1 = RuleOutcome::Single("success".to_string());
        let outcome2 = RuleOutcome::Negation("success".to_string());
        assert!(engine.check_outcome_contradiction(&outcome1, &outcome2));

        // Test non-contradictory outcomes
        let outcome3 = RuleOutcome::Single("success".to_string());
        let outcome4 = RuleOutcome::Single("failure".to_string());
        assert!(!engine.check_outcome_contradiction(&outcome3, &outcome4));

        // Test state change contradictions
        let state1 = RuleOutcome::StateChange {
            from_state: "idle".to_string(),
            to_state: "active".to_string(),
        };
        let state2 = RuleOutcome::StateChange {
            from_state: "idle".to_string(),
            to_state: "inactive".to_string(),
        };
        assert!(engine.check_outcome_contradiction(&state1, &state2));
    }

    #[test]
    fn test_temporal_constraints() {
        let constraints = TemporalConstraints {
            max_delay_minutes: 60.0,
            min_delay_minutes: 5.0,
            strict_order: true,
        };

        assert_eq!(constraints.max_delay_minutes, 60.0);
        assert_eq!(constraints.min_delay_minutes, 5.0);
        assert!(constraints.strict_order);

        // Test equality
        let constraints2 = TemporalConstraints {
            max_delay_minutes: 60.0,
            min_delay_minutes: 5.0,
            strict_order: true,
        };
        assert_eq!(constraints, constraints2);
    }
} 