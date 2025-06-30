//! Meta-Memory System
//!
//! This module provides meta-memory capabilities that track knowledge components,
//! their confidence levels, validation outcomes, and usage patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::Result;
use uuid::Uuid;
use brain_types::BrainError;

/// Types of knowledge that can be tracked in meta-memory
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum KnowledgeType {
    Segment,
    ConceptNode,
    Rule,
    SemanticConcept,
    WorkingMemory,
    EpisodicMemory,
    Pattern,
    ConceptRelationship,
    Memory,
    Insight,
    BPESegment,
    GitHubKnowledge,
    TrainingData,
}

impl std::fmt::Display for KnowledgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnowledgeType::Segment => write!(f, "Segment"),
            KnowledgeType::ConceptNode => write!(f, "ConceptNode"),
            KnowledgeType::Rule => write!(f, "Rule"),
            KnowledgeType::SemanticConcept => write!(f, "SemanticConcept"),
            KnowledgeType::WorkingMemory => write!(f, "WorkingMemory"),
            KnowledgeType::EpisodicMemory => write!(f, "EpisodicMemory"),
            KnowledgeType::Pattern => write!(f, "Pattern"),
            KnowledgeType::ConceptRelationship => write!(f, "ConceptRelationship"),
            KnowledgeType::Memory => write!(f, "Memory"),
            KnowledgeType::Insight => write!(f, "Insight"),
            KnowledgeType::BPESegment => write!(f, "BPESegment"),
            KnowledgeType::GitHubKnowledge => write!(f, "GitHubKnowledge"),
            KnowledgeType::TrainingData => write!(f, "TrainingData"),
        }
    }
}

/// Configuration for the meta-memory system
#[derive(Debug, Clone)]
pub struct MetaMemoryConfig {
    pub database_path: String,
    pub high_confidence_threshold: f64,
    pub low_confidence_threshold: f64,
    pub min_validation_count: u32,
    pub confidence_decay_rate: f64,
    pub max_items: Option<usize>,
    pub enable_persistence: bool,
}

impl Default for MetaMemoryConfig {
    fn default() -> Self {
        Self {
            database_path: "meta_memory.db".to_string(),
            high_confidence_threshold: 0.8,
            low_confidence_threshold: 0.3,
            min_validation_count: 3,
            confidence_decay_rate: 0.01,
            max_items: Some(10000),
            enable_persistence: false, // Simplified for demo
        }
    }
}

/// Individual meta-memory item tracking knowledge components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryItem {
    pub id: Uuid,
    pub component_id: Uuid,
    pub knowledge_type: KnowledgeType,
    pub confidence_score: f64,
    pub validation_count: u32,
    pub success_count: u32,
    pub failure_count: u32,
    pub usage_count: u32,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub last_validated: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

impl MetaMemoryItem {
    pub fn new(
        component_id: Uuid,
        knowledge_type: KnowledgeType,
        initial_confidence: f64,
        source: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            component_id,
            knowledge_type,
            confidence_score: initial_confidence.max(0.0).min(1.0),
            validation_count: 0,
            success_count: 0,
            failure_count: 0,
            usage_count: 0,
            source,
            created_at: Utc::now(),
            last_accessed: None,
            last_validated: None,
            metadata: HashMap::new(),
        }
    }

    /// Calculate success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        if self.validation_count == 0 {
            0.5 // Neutral when no validations
        } else {
            self.success_count as f64 / self.validation_count as f64
        }
    }

    /// Update confidence based on validation outcome
    pub fn update_confidence(&mut self, success: bool) {
        self.validation_count += 1;
        self.last_validated = Some(Utc::now());

        if success {
            self.success_count += 1;
        } else {
            self.failure_count += 1;
        }

        // Update confidence score using Bayesian-like update
        let success_rate = self.success_rate();
        let weight = 1.0 / (1.0 + (-(self.validation_count as f64) * 0.1).exp());
        
        self.confidence_score = (1.0 - weight) * self.confidence_score + weight * success_rate;
        self.confidence_score = self.confidence_score.max(0.0).min(1.0);
    }

    /// Mark as accessed
    pub fn mark_accessed(&mut self) {
        self.usage_count += 1;
        self.last_accessed = Some(Utc::now());
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Query structure for meta-memory items
#[derive(Debug, Clone, Default)]
pub struct MetaMemoryQuery {
    pub knowledge_type: Option<KnowledgeType>,
    pub min_confidence: Option<f64>,
    pub max_confidence: Option<f64>,
    pub min_usage_count: Option<u32>,
    pub max_usage_count: Option<u32>,
    pub min_validation_count: Option<u32>,
    pub max_validation_count: Option<u32>,
    pub source_pattern: Option<String>,
    pub metadata_filters: HashMap<String, String>,
    pub sort_by: Option<String>,
    pub descending: bool,
    pub limit: Option<usize>,
}

/// Statistics about the meta-memory system
#[derive(Debug, Clone)]
pub struct MetaMemoryStats {
    pub total_components: usize,
    pub average_confidence: f64,
    pub high_confidence_count: usize,
    pub low_confidence_count: usize,
    pub total_validations: u32,
    pub total_successes: u32,
    pub total_failures: u32,
    pub total_usage: u32,
    pub knowledge_type_distribution: HashMap<KnowledgeType, usize>,
    pub confidence_distribution: Vec<(f64, usize)>, // (threshold, count)
}

/// Main meta-memory system
pub struct MetaMemorySystem {
    config: MetaMemoryConfig,
    items: HashMap<Uuid, MetaMemoryItem>,
    component_to_meta: HashMap<Uuid, Uuid>, // component_id -> meta_id
}

impl MetaMemorySystem {
    /// Create a new meta-memory system with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(MetaMemoryConfig::default())
    }

    /// Create a new meta-memory system with specified configuration
    pub fn with_config(config: MetaMemoryConfig) -> Result<Self> {
        Ok(Self {
            config,
            items: HashMap::new(),
            component_to_meta: HashMap::new(),
        })
    }

    /// Store a meta-memory item
    pub fn store_item(&mut self, item: MetaMemoryItem) -> Result<Uuid> {
        let meta_id = item.id;
        let component_id = item.component_id;
        
        // Check max items limit
        if let Some(max_items) = self.config.max_items {
            if self.items.len() >= max_items {
                return Err(BrainError::Other("Meta-memory capacity exceeded".to_string()).into());
            }
        }

        self.items.insert(meta_id, item);
        self.component_to_meta.insert(component_id, meta_id);

        Ok(meta_id)
    }

    /// Update confidence for a component
    pub fn update_confidence(&mut self, component_id: Uuid, success: bool) -> Result<()> {
        if let Some(&meta_id) = self.component_to_meta.get(&component_id) {
            if let Some(item) = self.items.get_mut(&meta_id) {
                item.update_confidence(success);
            }
        }
        Ok(())
    }

    /// Mark a component as accessed
    pub fn mark_accessed(&mut self, component_id: Uuid) -> Result<()> {
        if let Some(&meta_id) = self.component_to_meta.get(&component_id) {
            if let Some(item) = self.items.get_mut(&meta_id) {
                item.mark_accessed();
            }
        }
        Ok(())
    }

    /// Get high-confidence components
    pub fn get_high_confidence_components(&self) -> Result<Vec<MetaMemoryItem>> {
        Ok(self.items
            .values()
            .filter(|item| item.confidence_score >= self.config.high_confidence_threshold)
            .cloned()
            .collect())
    }

    /// Get low-confidence components
    pub fn get_low_confidence_components(&self) -> Result<Vec<MetaMemoryItem>> {
        Ok(self.items
            .values()
            .filter(|item| item.confidence_score <= self.config.low_confidence_threshold)
            .cloned()
            .collect())
    }

    /// Query items based on criteria
    pub fn query_items(&self, query: &MetaMemoryQuery) -> Result<Vec<MetaMemoryItem>> {
        let mut results: Vec<MetaMemoryItem> = self.items
            .values()
            .filter(|item| self.matches_query(item, query))
            .cloned()
            .collect();

        // Sort results
        if let Some(sort_field) = &query.sort_by {
            match sort_field.as_str() {
                "confidence_score" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.confidence_score.partial_cmp(&a.confidence_score).unwrap()
                        } else {
                            a.confidence_score.partial_cmp(&b.confidence_score).unwrap()
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
                "validation_count" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.validation_count.cmp(&a.validation_count)
                        } else {
                            a.validation_count.cmp(&b.validation_count)
                        }
                    });
                }
                "created_at" => {
                    results.sort_by(|a, b| {
                        if query.descending {
                            b.created_at.cmp(&a.created_at)
                        } else {
                            a.created_at.cmp(&b.created_at)
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

        Ok(results)
    }

    /// Check if an item matches query criteria
    fn matches_query(&self, item: &MetaMemoryItem, query: &MetaMemoryQuery) -> bool {
        // Knowledge type filter
        if let Some(ref knowledge_type) = query.knowledge_type {
            if &item.knowledge_type != knowledge_type {
                return false;
            }
        }

        // Confidence range filters
        if let Some(min_confidence) = query.min_confidence {
            if item.confidence_score < min_confidence {
                return false;
            }
        }
        if let Some(max_confidence) = query.max_confidence {
            if item.confidence_score > max_confidence {
                return false;
            }
        }

        // Usage count filters
        if let Some(min_usage) = query.min_usage_count {
            if item.usage_count < min_usage {
                return false;
            }
        }
        if let Some(max_usage) = query.max_usage_count {
            if item.usage_count > max_usage {
                return false;
            }
        }

        // Validation count filters
        if let Some(min_validation) = query.min_validation_count {
            if item.validation_count < min_validation {
                return false;
            }
        }
        if let Some(max_validation) = query.max_validation_count {
            if item.validation_count > max_validation {
                return false;
            }
        }

        // Source pattern filter
        if let Some(ref pattern) = query.source_pattern {
            if !item.source.contains(pattern) {
                return false;
            }
        }

        // Metadata filters
        for (key, expected_value) in &query.metadata_filters {
            if let Some(actual_value) = item.metadata.get(key) {
                if actual_value != expected_value {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Get system statistics
    pub fn get_stats(&self) -> MetaMemoryStats {
        let total_components = self.items.len();
        let total_confidence: f64 = self.items.values().map(|item| item.confidence_score).sum();
        let average_confidence = if total_components > 0 {
            total_confidence / total_components as f64
        } else {
            0.0
        };

        let high_confidence_count = self.items
            .values()
            .filter(|item| item.confidence_score >= self.config.high_confidence_threshold)
            .count();

        let low_confidence_count = self.items
            .values()
            .filter(|item| item.confidence_score <= self.config.low_confidence_threshold)
            .count();

        let total_validations: u32 = self.items.values().map(|item| item.validation_count).sum();
        let total_successes: u32 = self.items.values().map(|item| item.success_count).sum();
        let total_failures: u32 = self.items.values().map(|item| item.failure_count).sum();
        let total_usage: u32 = self.items.values().map(|item| item.usage_count).sum();

        // Knowledge type distribution
        let mut knowledge_type_distribution = HashMap::new();
        for item in self.items.values() {
            *knowledge_type_distribution.entry(item.knowledge_type.clone()).or_insert(0) += 1;
        }

        // Confidence distribution
        let confidence_ranges = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let mut confidence_distribution = Vec::new();
        for i in 0..confidence_ranges.len()-1 {
            let min_threshold = confidence_ranges[i];
            let max_threshold = confidence_ranges[i+1];
            let count = self.items
                .values()
                .filter(|item| item.confidence_score >= min_threshold && item.confidence_score < max_threshold)
                .count();
            confidence_distribution.push((max_threshold, count));
        }

        MetaMemoryStats {
            total_components,
            average_confidence,
            high_confidence_count,
            low_confidence_count,
            total_validations,
            total_successes,
            total_failures,
            total_usage,
            knowledge_type_distribution,
            confidence_distribution,
        }
    }

    /// Get item by component ID
    pub fn get_item_by_component(&self, component_id: Uuid) -> Option<&MetaMemoryItem> {
        self.component_to_meta
            .get(&component_id)
            .and_then(|meta_id| self.items.get(meta_id))
    }

    /// Get item by meta ID
    pub fn get_item(&self, meta_id: Uuid) -> Option<&MetaMemoryItem> {
        self.items.get(&meta_id)
    }

    /// Remove item by component ID
    pub fn remove_item(&mut self, component_id: Uuid) -> Result<bool> {
        if let Some(meta_id) = self.component_to_meta.remove(&component_id) {
            self.items.remove(&meta_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.component_to_meta.clear();
    }

    /// Get total item count
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Meta-memory system placeholder 