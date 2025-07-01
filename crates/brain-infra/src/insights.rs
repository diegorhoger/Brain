//! Advanced Insight Extraction Infrastructure
//! 
//! This module implements the sophisticated insight extraction system that generalizes rules and causal patterns
//! from experiences stored in memory and the concept graph. It provides three major components:
//! 
//! 1. Pattern Detection System - Monitors memory stores and identifies recurring patterns
//! 2. Rule Formalization Engine - Converts patterns into formal rules with validation
//! 3. Rule Generalization System - Advanced rule management and generalization

use brain_types::*;
use brain_core::{Insight, InsightType, InsightRepository};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

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

impl PatternDetector {
    /// Create a new pattern detector with default configuration
    pub fn new() -> Self {
        Self {
            config: PatternDetectionConfig::default(),
            pattern_cache: HashMap::new(),
            detection_stats: DetectionStats::default(),
            last_processed_at: None,
        }
    }

    /// Create a pattern detector with custom configuration
    pub fn with_config(config: PatternDetectionConfig) -> Self {
        Self {
            config,
            pattern_cache: HashMap::new(),
            detection_stats: DetectionStats::default(),
            last_processed_at: None,
        }
    }

    /// Detect patterns from memory system
    /// Note: This is a placeholder implementation that would integrate with actual memory system
    pub async fn detect_patterns_from_memory(
        &mut self,
        _memory_items: &[String], // Placeholder for actual memory items
    ) -> Result<PatternDetectionResult> {
        let start_time = std::time::Instant::now();
        
        // Placeholder implementation - in real system would analyze memory items
        let mut detected_patterns = Vec::new();
        
        // Simple frequency pattern detection as example
        let mut element_counts = HashMap::new();
        for item in _memory_items {
            *element_counts.entry(item.clone()).or_insert(0) += 1;
        }
        
        for (element, count) in element_counts {
            if count >= self.config.min_pattern_frequency {
                let pattern = DetectedPattern::new(
                    PatternType::Frequency,
                    vec![element],
                    count,
                    count as f64 / _memory_items.len() as f64,
                    vec![Uuid::new_v4()], // Placeholder evidence
                );
                detected_patterns.push(pattern);
            }
        }
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        // Update cache and stats
        self.update_pattern_cache(&detected_patterns);
        self.update_detection_stats(&detected_patterns, _memory_items.len(), processing_time);
        
        let mut pattern_type_counts = HashMap::new();
        for pattern in &detected_patterns {
            *pattern_type_counts.entry(pattern.pattern_type.clone()).or_insert(0) += 1;
        }
        
        Ok(PatternDetectionResult {
            detected_patterns,
            items_processed: _memory_items.len(),
            processing_time_ms: processing_time,
            filtered_patterns: 0,
            pattern_type_counts,
        })
    }

    /// Extract content pattern from text
    #[allow(dead_code)]
    fn extract_content_pattern(&self, content: &str) -> String {
        // Simple pattern extraction - in real implementation would use NLP
        content.split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Update pattern cache with new patterns
    fn update_pattern_cache(&mut self, patterns: &[DetectedPattern]) {
        for pattern in patterns {
            let key = format!("{:?}_{}", pattern.pattern_type, pattern.elements.join("_"));
            self.pattern_cache.insert(key, pattern.clone());
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
            *self.detection_stats.patterns_by_type.entry(pattern.pattern_type.clone()).or_insert(0) += 1;
        }
    }

    /// Get cached patterns
    pub fn get_cached_patterns(&self) -> Vec<&DetectedPattern> {
        self.pattern_cache.values().collect()
    }

    /// Get detection statistics
    pub fn get_detection_stats(&self) -> &DetectionStats {
        &self.detection_stats
    }

    /// Get current configuration
    pub fn get_config(&self) -> &PatternDetectionConfig {
        &self.config
    }

    /// Set new configuration
    pub fn set_config(&mut self, config: PatternDetectionConfig) {
        self.config = config;
    }

    /// Clear pattern cache
    pub fn clear_cache(&mut self) {
        self.pattern_cache.clear();
    }

    /// Reset statistics
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

/// Advanced Insight Extraction Manager
/// 
/// This combines pattern detection with the core insight system to provide
/// a comprehensive insight extraction infrastructure.
pub struct InsightExtractionManager {
    /// Core insight repository
    insight_repository: Box<dyn InsightRepository>,
    /// Pattern detection system
    pattern_detector: PatternDetector,
    /// Insights generated from patterns
    pattern_insights: HashMap<Uuid, Vec<Uuid>>, // Pattern ID -> Insight IDs
}

impl InsightExtractionManager {
    /// Create new insight extraction manager
    pub fn new(insight_repository: Box<dyn InsightRepository>) -> Self {
        Self {
            insight_repository,
            pattern_detector: PatternDetector::new(),
            pattern_insights: HashMap::new(),
        }
    }

    /// Create manager with custom pattern detection config
    pub fn with_pattern_config(
        insight_repository: Box<dyn InsightRepository>,
        pattern_config: PatternDetectionConfig,
    ) -> Self {
        Self {
            insight_repository,
            pattern_detector: PatternDetector::with_config(pattern_config),
            pattern_insights: HashMap::new(),
        }
    }

    /// Extract insights from content using pattern detection
    pub async fn extract_insights_from_content(&mut self, content: &[String]) -> Result<Vec<Insight>> {
        // Detect patterns first
        let pattern_result = self.pattern_detector.detect_patterns_from_memory(content).await?;
        
        let mut insights = Vec::new();
        let mut pattern_insight_ids = Vec::new();
        
        // Convert significant patterns to insights
        for pattern in &pattern_result.detected_patterns {
            if pattern.is_significant(self.pattern_detector.get_config()) {
                let insight_content = format!(
                    "Pattern detected: {} with {} occurrences (confidence: {:.2})",
                    pattern.elements.join(" -> "),
                    pattern.frequency,
                    pattern.confidence
                );
                
                let insight_type = match pattern.pattern_type {
                    PatternType::TemporalSequence => InsightType::Pattern,
                    PatternType::CoOccurrence => InsightType::Relationship,
                    PatternType::Causal => InsightType::Relationship,
                    PatternType::Correlation => InsightType::Relationship,
                    PatternType::Frequency => InsightType::Trend,
                    PatternType::Hierarchical => InsightType::Relationship,
                    PatternType::Similarity => InsightType::Pattern,
                    PatternType::Negation => InsightType::Anomaly,
                };
                
                let insight = Insight {
                    id: Uuid::new_v4(),
                    content: insight_content,
                    confidence: pattern.confidence,
                    source: "PatternDetection".to_string(),
                    insight_type,
                };
                
                // Store the insight
                let insight_id = self.insight_repository.store_insight(insight.clone()).await?;
                pattern_insight_ids.push(insight_id);
                insights.push(insight);
            }
        }
        
        // Track pattern-to-insight mappings for future reference
        for pattern in &pattern_result.detected_patterns {
            self.pattern_insights.insert(pattern.id, pattern_insight_ids.clone());
        }
        
        Ok(insights)
    }

    /// Get pattern detection statistics
    pub fn get_pattern_stats(&self) -> &DetectionStats {
        self.pattern_detector.get_detection_stats()
    }

    /// Get insights generated from a specific pattern
    pub async fn get_insights_for_pattern(&self, pattern_id: Uuid) -> Result<Vec<Insight>> {
        if let Some(insight_ids) = self.pattern_insights.get(&pattern_id) {
            let mut insights = Vec::new();
            for &insight_id in insight_ids {
                if let Some(insight) = self.insight_repository.get_insight(insight_id).await? {
                    insights.push(insight);
                }
            }
            Ok(insights)
        } else {
            Ok(Vec::new())
        }
    }

    /// Update pattern detection configuration
    pub fn update_pattern_config(&mut self, config: PatternDetectionConfig) {
        self.pattern_detector.set_config(config);
    }

    /// Clear pattern caches and reset statistics
    pub fn reset_pattern_detection(&mut self) {
        self.pattern_detector.clear_cache();
        self.pattern_detector.reset_stats();
        self.pattern_insights.clear();
    }
}

/// Enhanced in-memory implementation of InsightRepository with advanced features
#[derive(Debug)]
pub struct InMemoryInsightRepository {
    insights: Arc<RwLock<HashMap<Uuid, Insight>>>,
    /// Index by insight type for efficient retrieval
    type_index: Arc<RwLock<HashMap<InsightType, Vec<Uuid>>>>,
    /// Index by confidence ranges
    confidence_index: Arc<RwLock<HashMap<String, Vec<Uuid>>>>, // "high", "medium", "low"
    /// Index by source
    source_index: Arc<RwLock<HashMap<String, Vec<Uuid>>>>,
}

impl InMemoryInsightRepository {
    pub fn new() -> Self {
        Self {
            insights: Arc::new(RwLock::new(HashMap::new())),
            type_index: Arc::new(RwLock::new(HashMap::new())),
            confidence_index: Arc::new(RwLock::new(HashMap::new())),
            source_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get confidence category for indexing
    fn get_confidence_category(confidence: f64) -> String {
        if confidence >= 0.8 {
            "high".to_string()
        } else if confidence >= 0.5 {
            "medium".to_string()
        } else {
            "low".to_string()
        }
    }

    /// Update all indexes for an insight
    fn update_indexes(&self, insight: &Insight) -> Result<()> {
        // Update type index
        let mut type_index = self.type_index.write()
            .map_err(|_| BrainError::LockError("Failed to acquire type index write lock".to_string()))?;
        type_index.entry(insight.insight_type.clone()).or_insert_with(Vec::new).push(insight.id);

        // Update confidence index
        let mut confidence_index = self.confidence_index.write()
            .map_err(|_| BrainError::LockError("Failed to acquire confidence index write lock".to_string()))?;
        let category = Self::get_confidence_category(insight.confidence);
        confidence_index.entry(category).or_insert_with(Vec::new).push(insight.id);

        // Update source index
        let mut source_index = self.source_index.write()
            .map_err(|_| BrainError::LockError("Failed to acquire source index write lock".to_string()))?;
        source_index.entry(insight.source.clone()).or_insert_with(Vec::new).push(insight.id);

        Ok(())
    }

    /// Get insights by confidence range
    pub async fn get_insights_by_confidence_range(&self, min_confidence: f64, max_confidence: f64) -> Result<Vec<Insight>> {
        let insights = self.insights.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let results: Vec<Insight> = insights
            .values()
            .filter(|insight| insight.confidence >= min_confidence && insight.confidence <= max_confidence)
            .cloned()
            .collect();

        Ok(results)
    }

    /// Get insights by source
    pub async fn get_insights_by_source(&self, source: &str) -> Result<Vec<Insight>> {
        let source_index = self.source_index.read()
            .map_err(|_| BrainError::LockError("Failed to acquire source index read lock".to_string()))?;
        
        if let Some(insight_ids) = source_index.get(source) {
            let insights = self.insights.read()
                .map_err(|_| BrainError::LockError("Failed to acquire insights read lock".to_string()))?;
            
            let results: Vec<Insight> = insight_ids
                .iter()
                .filter_map(|id| insights.get(id).cloned())
                .collect();
            
            Ok(results)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get top insights by confidence
    pub async fn get_top_insights(&self, limit: usize) -> Result<Vec<Insight>> {
        let insights = self.insights.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        
        let mut results: Vec<Insight> = insights.values().cloned().collect();
        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        
        Ok(results)
    }
}

impl Default for InMemoryInsightRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
impl InsightRepository for InMemoryInsightRepository {
    async fn store_insight(&mut self, insight: Insight) -> Result<Uuid> {
        let id = insight.id;
        
        // Update indexes first
        self.update_indexes(&insight)?;
        
        // Store the insight
        let mut insights = self.insights.write()
            .map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        insights.insert(id, insight);
        
        Ok(id)
    }

    async fn get_insight(&self, id: Uuid) -> Result<Option<Insight>> {
        let insights = self.insights.read()
            .map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(insights.get(&id).cloned())
    }

    async fn get_insights_by_type(&self, insight_type: InsightType) -> Result<Vec<Insight>> {
        let type_index = self.type_index.read()
            .map_err(|_| BrainError::LockError("Failed to acquire type index read lock".to_string()))?;
        
        if let Some(insight_ids) = type_index.get(&insight_type) {
            let insights = self.insights.read()
                .map_err(|_| BrainError::LockError("Failed to acquire insights read lock".to_string()))?;
            
            let results: Vec<Insight> = insight_ids
                .iter()
                .filter_map(|id| insights.get(id).cloned())
                .collect();
            
            Ok(results)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detection_config_creation() {
        let config = PatternDetectionConfig::default();
        assert_eq!(config.min_pattern_frequency, 3);
        assert_eq!(config.min_confidence_threshold, 0.6);
        assert!(config.incremental_detection);
    }

    #[test]
    fn test_detected_pattern_creation() {
        let pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["test".to_string()],
            5,
            0.8,
            vec![Uuid::new_v4()],
        );
        
        assert_eq!(pattern.pattern_type, PatternType::Frequency);
        assert_eq!(pattern.elements, vec!["test".to_string()]);
        assert_eq!(pattern.frequency, 5);
        assert_eq!(pattern.confidence, 0.8);
        assert_eq!(pattern.strength, 0.8);
    }

    #[test]
    fn test_pattern_significance_check() {
        let config = PatternDetectionConfig {
            min_pattern_frequency: 3,
            min_confidence_threshold: 0.6,
            significance_threshold: 0.05,
            ..Default::default()
        };
        
        let mut pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["test".to_string()],
            5,
            0.8,
            vec![Uuid::new_v4()],
        );
        pattern.significance = 0.01; // Below threshold (good)
        
        assert!(pattern.is_significant(&config));
        
        // Test with low frequency
        pattern.frequency = 1;
        assert!(!pattern.is_significant(&config));
    }

    #[test]
    fn test_pattern_detector_creation() {
        let detector = PatternDetector::new();
        assert_eq!(detector.get_config().min_pattern_frequency, 3);
        assert_eq!(detector.get_detection_stats().total_patterns_detected, 0);
    }

    #[test]
    fn test_pattern_detector_with_custom_config() {
        let config = PatternDetectionConfig {
            min_pattern_frequency: 5,
            min_confidence_threshold: 0.8,
            ..Default::default()
        };
        
        let detector = PatternDetector::with_config(config.clone());
        assert_eq!(detector.get_config().min_pattern_frequency, 5);
        assert_eq!(detector.get_config().min_confidence_threshold, 0.8);
    }

    #[tokio::test]
    async fn test_pattern_detection_from_memory() {
        let mut detector = PatternDetector::new();
        
        // Test data with repeated elements
        let memory_items = vec![
            "apple".to_string(),
            "banana".to_string(),
            "apple".to_string(),
            "cherry".to_string(),
            "apple".to_string(),
            "banana".to_string(),
        ];
        
        let result = detector.detect_patterns_from_memory(&memory_items).await.unwrap();
        
        assert_eq!(result.items_processed, 6);
        assert!(!result.detected_patterns.is_empty());
        
        // Should detect "apple" as a frequent pattern (appears 3 times)
        let apple_pattern = result.detected_patterns.iter()
            .find(|p| p.elements.contains(&"apple".to_string()));
        assert!(apple_pattern.is_some());
        
        let pattern = apple_pattern.unwrap();
        assert_eq!(pattern.pattern_type, PatternType::Frequency);
        assert_eq!(pattern.frequency, 3);
    }

    #[tokio::test]
    async fn test_insight_repository_operations() {
        let mut repository = InMemoryInsightRepository::new();
        
        let insight = Insight {
            id: Uuid::new_v4(),
            content: "Test insight".to_string(),
            confidence: 0.8,
            source: "TestSource".to_string(),
            insight_type: InsightType::Pattern,
        };
        
        // Store insight
        let stored_id = repository.store_insight(insight.clone()).await.unwrap();
        assert_eq!(stored_id, insight.id);
        
        // Retrieve insight
        let retrieved = repository.get_insight(insight.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, "Test insight");
        
        // Get insights by type
        let pattern_insights = repository.get_insights_by_type(InsightType::Pattern).await.unwrap();
        assert_eq!(pattern_insights.len(), 1);
        assert_eq!(pattern_insights[0].content, "Test insight");
    }

    #[tokio::test]
    async fn test_insight_repository_advanced_queries() {
        let mut repository = InMemoryInsightRepository::new();
        
        // Create multiple insights with different properties
        let insights = vec![
            Insight {
                id: Uuid::new_v4(),
                content: "High confidence insight".to_string(),
                confidence: 0.9,
                source: "SourceA".to_string(),
                insight_type: InsightType::Pattern,
            },
            Insight {
                id: Uuid::new_v4(),
                content: "Medium confidence insight".to_string(),
                confidence: 0.6,
                source: "SourceB".to_string(),
                insight_type: InsightType::Trend,
            },
            Insight {
                id: Uuid::new_v4(),
                content: "Low confidence insight".to_string(),
                confidence: 0.3,
                source: "SourceA".to_string(),
                insight_type: InsightType::Anomaly,
            },
        ];
        
        // Store all insights
        for insight in &insights {
            repository.store_insight(insight.clone()).await.unwrap();
        }
        
        // Test confidence range query
        let high_confidence = repository.get_insights_by_confidence_range(0.8, 1.0).await.unwrap();
        assert_eq!(high_confidence.len(), 1);
        assert_eq!(high_confidence[0].confidence, 0.9);
        
        // Test source query
        let source_a_insights = repository.get_insights_by_source("SourceA").await.unwrap();
        assert_eq!(source_a_insights.len(), 2);
        
        // Test top insights
        let top_insights = repository.get_top_insights(2).await.unwrap();
        assert_eq!(top_insights.len(), 2);
        assert!(top_insights[0].confidence >= top_insights[1].confidence);
    }

    #[tokio::test]
    async fn test_insight_extraction_manager() {
        let repository = Box::new(InMemoryInsightRepository::new());
        
        // Create custom config with lower thresholds for testing
        let config = PatternDetectionConfig {
            min_pattern_frequency: 3,
            min_confidence_threshold: 0.4, // Lower threshold for testing
            ..Default::default()
        };
        
        let mut manager = InsightExtractionManager::with_pattern_config(repository, config);
        
        // Test data with patterns - "error" appears 3 times
        let content = vec![
            "error".to_string(),
            "warning".to_string(),
            "error".to_string(),
            "info".to_string(),
            "error".to_string(),
            "debug".to_string(),
        ];
        
        let insights = manager.extract_insights_from_content(&content).await.unwrap();
        
        assert!(!insights.is_empty(), "Expected insights to be generated from patterns");
        
        // Should generate insight for "error" pattern (appears 3 times)
        let error_insight = insights.iter()
            .find(|i| i.content.contains("error"));
        assert!(error_insight.is_some(), "Expected to find error pattern insight");
        
        let insight = error_insight.unwrap();
        assert_eq!(insight.insight_type, InsightType::Trend); // Frequency patterns map to Trend
        assert_eq!(insight.source, "PatternDetection");
        assert!(insight.confidence > 0.0);
    }

    #[tokio::test]
    async fn test_pattern_detection_stats_tracking() {
        let mut detector = PatternDetector::new();
        
        let memory_items1 = vec!["a".to_string(), "b".to_string(), "a".to_string(), "a".to_string()];
        let memory_items2 = vec!["x".to_string(), "y".to_string(), "x".to_string(), "x".to_string()];
        
        // First detection
        detector.detect_patterns_from_memory(&memory_items1).await.unwrap();
        let stats = detector.get_detection_stats();
        assert_eq!(stats.detection_operations, 1);
        assert_eq!(stats.total_items_processed, 4);
        
        // Second detection
        detector.detect_patterns_from_memory(&memory_items2).await.unwrap();
        let stats = detector.get_detection_stats();
        assert_eq!(stats.detection_operations, 2);
        assert_eq!(stats.total_items_processed, 8);
        assert!(stats.average_patterns_per_operation > 0.0);
    }

    #[test]
    fn test_pattern_type_display() {
        assert_eq!(PatternType::Frequency.to_string(), "Frequency");
        assert_eq!(PatternType::TemporalSequence.to_string(), "TemporalSequence");
        assert_eq!(PatternType::Causal.to_string(), "Causal");
    }

    #[test]
    fn test_detection_stats_default() {
        let stats = DetectionStats::default();
        assert_eq!(stats.total_patterns_detected, 0);
        assert_eq!(stats.detection_operations, 0);
        assert_eq!(stats.average_patterns_per_operation, 0.0);
    }

    #[test]
    fn test_temporal_info_creation() {
        let temporal_info = TemporalInfo {
            average_delay_minutes: 15.5,
            delay_std_dev: 3.2,
            min_delay_minutes: 10.0,
            max_delay_minutes: 25.0,
        };
        
        assert_eq!(temporal_info.average_delay_minutes, 15.5);
        assert_eq!(temporal_info.min_delay_minutes, 10.0);
        assert_eq!(temporal_info.max_delay_minutes, 25.0);
    }

    #[test]
    fn test_pattern_cache_operations() {
        let mut detector = PatternDetector::new();
        
        let pattern = DetectedPattern::new(
            PatternType::Frequency,
            vec!["test".to_string()],
            5,
            0.8,
            vec![Uuid::new_v4()],
        );
        
        detector.update_pattern_cache(&[pattern.clone()]);
        
        let cached_patterns = detector.get_cached_patterns();
        assert_eq!(cached_patterns.len(), 1);
        assert_eq!(cached_patterns[0].elements, vec!["test".to_string()]);
        
        // Test cache clearing
        detector.clear_cache();
        assert_eq!(detector.get_cached_patterns().len(), 0);
    }

    #[test]
    fn test_stats_reset() {
        let mut detector = PatternDetector::new();
        
        // Manually set some stats to verify reset
        detector.detection_stats.total_patterns_detected = 10;
        detector.detection_stats.detection_operations = 5;
        
        assert_eq!(detector.get_detection_stats().total_patterns_detected, 10);
        
        detector.reset_stats();
        
        let stats = detector.get_detection_stats();
        assert_eq!(stats.total_patterns_detected, 0);
        assert_eq!(stats.detection_operations, 0);
        assert_eq!(stats.average_patterns_per_operation, 0.0);
    }
}

 