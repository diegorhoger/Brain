//! Segmentation Domain Logic and Abstractions
//! 
//! This module defines core segmentation abstractions and domain logic
//! without any I/O dependencies. Infrastructure implementations are
//! provided through trait implementations.

use brain_types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for BPE algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpeConfig {
    pub min_frequency: usize,
    pub max_vocab_size: usize,
    pub num_merges: usize,
    pub include_chars: bool,
    pub min_entropy_threshold: f64,
    pub context_window_size: usize,
    pub min_confidence: f64,
    pub enable_advanced_heuristics: bool,
}

impl Default for BpeConfig {
    fn default() -> Self {
        Self {
            min_frequency: 2,
            max_vocab_size: 10000,
            num_merges: 1000,
            include_chars: true,
            min_entropy_threshold: 0.5,
            context_window_size: 3,
            min_confidence: 0.3,
            enable_advanced_heuristics: true,
        }
    }
}

/// A pair of characters or segments that can be merged
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SegmentPair {
    pub left: String,
    pub right: String,
}

impl SegmentPair {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
    
    /// Get the merged segment string
    pub fn merged(&self) -> String {
        format!("{}{}", self.left, self.right)
    }
}

/// Segment statistics with advanced metrics and lifecycle data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentStats {
    pub segment: String,
    pub frequency: usize,
    pub length: usize,
    pub formed_from: Option<SegmentPair>,
    pub merge_step: Option<usize>,
    pub confidence: f64,
    pub entropy: f64,
    pub context_stability: f64,
    pub created_at: u64,
    pub last_accessed: u64,
    pub last_modified: u64,
    pub access_count: usize,
    pub is_archived: bool,
}

impl SegmentStats {
    pub fn new_char(ch: char) -> Self {
        let now = current_timestamp();
        Self {
            segment: ch.to_string(),
            frequency: 0,
            length: 1,
            formed_from: None,
            merge_step: None,
            confidence: 0.5,
            entropy: 0.0,
            context_stability: 0.0,
            created_at: now,
            last_accessed: now,
            last_modified: now,
            access_count: 0,
            is_archived: false,
        }
    }
    
    pub fn new_merged(pair: SegmentPair, frequency: usize, merge_step: usize) -> Self {
        let segment = pair.merged();
        let now = current_timestamp();
        Self {
            length: segment.chars().count(),
            segment,
            frequency,
            formed_from: Some(pair),
            merge_step: Some(merge_step),
            confidence: 0.0,
            entropy: 0.0,
            context_stability: 0.0,
            created_at: now,
            last_accessed: now,
            last_modified: now,
            access_count: 0,
            is_archived: false,
        }
    }

    /// Update confidence score based on frequency and stability
    pub fn update_confidence(&mut self, total_frequency: usize, stability_factor: f64) {
        let frequency_score = self.frequency as f64 / total_frequency.max(1) as f64;
        let length_bonus = (self.length as f64).ln() / 10.0;
        self.confidence = (frequency_score + stability_factor + length_bonus).min(1.0);
        self.last_modified = current_timestamp();
    }

    /// Mark this segment as accessed
    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
        self.last_accessed = current_timestamp();
    }

    /// Check if this segment should be pruned
    pub fn is_candidate_for_pruning(&self, config: &PruningConfig) -> bool {
        let now = current_timestamp();
        let age_days = (now - self.created_at) / (24 * 60 * 60);
        let days_since_access = (now - self.last_accessed) / (24 * 60 * 60);
        
        if self.is_archived {
            return false;
        }
        
        if self.confidence < config.min_confidence_threshold && age_days > config.min_age_days {
            return true;
        }
        
        if days_since_access > config.max_inactive_days && self.access_count < config.min_access_count {
            return true;
        }
        
        false
    }

    /// Archive this segment
    pub fn archive(&mut self) {
        self.is_archived = true;
        self.last_modified = current_timestamp();
    }
}

/// Configuration for segment pruning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruningConfig {
    pub min_confidence_threshold: f64,
    pub min_age_days: u64,
    pub max_inactive_days: u64,
    pub min_access_count: usize,
    pub max_segments: usize,
    pub enable_auto_pruning: bool,
}

impl Default for PruningConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.3,
            min_age_days: 7,
            max_inactive_days: 30,
            min_access_count: 5,
            max_segments: 10000,
            enable_auto_pruning: true,
        }
    }
}

/// BPE statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpeStats {
    pub total_segments: usize,
    pub character_segments: usize,
    pub merged_segments: usize,
    pub merges_performed: usize,
    pub max_segment_length: usize,
    pub high_confidence_segments: usize,
    pub average_confidence: f64,
    pub average_entropy: f64,
    pub context_observations: usize,
}

/// Repository trait for segment storage
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait SegmentRepository: Send + Sync {
    async fn store_segment(&mut self, stats: SegmentStats) -> Result<()>;
    async fn get_segment(&self, segment: &str) -> Result<Option<SegmentStats>>;
    async fn update_segment(&mut self, stats: &SegmentStats) -> Result<()>;
    async fn remove_segment(&mut self, segment: &str) -> Result<()>;
    async fn get_all_segments(&self) -> Result<Vec<SegmentStats>>;
    async fn get_segments_by_frequency(&self) -> Result<Vec<SegmentStats>>;
    async fn get_segments_by_confidence(&self) -> Result<Vec<SegmentStats>>;
    async fn get_high_confidence_segments(&self) -> Result<Vec<SegmentStats>>;
    async fn get_pruning_candidates(&self, config: &PruningConfig) -> Result<Vec<SegmentStats>>;
    async fn archive_segment(&mut self, segment: &str) -> Result<bool>;
    async fn restore_from_archive(&mut self, segment: &str) -> Result<bool>;
}

/// Segment provider trait for text segmentation
pub trait SegmentProvider: Send + Sync {
    fn get_segments(&self) -> Vec<String>;
    fn segment_text(&self, text: &str) -> Vec<String>;
    fn get_segment_stats(&self, segment: &str) -> Option<SegmentStats>;
    fn get_high_confidence_segments(&self) -> Vec<String>;
}

/// Segmentation service coordinating segment discovery and management
pub struct SegmentationService {
    repository: Box<dyn SegmentRepository>,
    config: BpeConfig,
    pruning_config: PruningConfig,
}

impl SegmentationService {
    pub fn new(
        repository: Box<dyn SegmentRepository>,
        config: BpeConfig,
        pruning_config: PruningConfig,
    ) -> Self {
        Self {
            repository,
            config,
            pruning_config,
        }
    }

    pub async fn initialize_from_text(&mut self, text: &str) -> Result<()> {
        // Initialize character segments
        let mut char_frequencies = HashMap::new();
        for ch in text.chars() {
            *char_frequencies.entry(ch).or_insert(0) += 1;
        }

        // Store character segments
        for (ch, frequency) in char_frequencies {
            let mut stats = SegmentStats::new_char(ch);
            stats.frequency = frequency;
            self.repository.store_segment(stats).await?;
        }

        Ok(())
    }

    pub async fn train(&mut self) -> Result<()> {
        // This would implement the full BPE training algorithm
        // For now, just a placeholder
        Ok(())
    }

    pub async fn segment_text(&self, text: &str) -> Result<Vec<String>> {
        // This would implement text segmentation using learned segments
        // For now, return character-level segmentation
        Ok(text.chars().map(|c| c.to_string()).collect())
    }

    pub async fn get_stats(&self) -> Result<BpeStats> {
        let all_segments = self.repository.get_all_segments().await?;
        
        let character_segments = all_segments.iter().filter(|s| s.length == 1).count();
        let merged_segments = all_segments.iter().filter(|s| s.formed_from.is_some()).count();
        let high_confidence_segments = all_segments.iter().filter(|s| s.confidence >= 0.7).count();
        
        let average_confidence = if all_segments.is_empty() {
            0.0
        } else {
            all_segments.iter().map(|s| s.confidence).sum::<f64>() / all_segments.len() as f64
        };

        let average_entropy = if all_segments.is_empty() {
            0.0
        } else {
            all_segments.iter().map(|s| s.entropy).sum::<f64>() / all_segments.len() as f64
        };

        let max_segment_length = all_segments.iter().map(|s| s.length).max().unwrap_or(0);

        Ok(BpeStats {
            total_segments: all_segments.len(),
            character_segments,
            merged_segments,
            merges_performed: merged_segments, // Approximation
            max_segment_length,
            high_confidence_segments,
            average_confidence,
            average_entropy,
            context_observations: 0, // Would be tracked separately
        })
    }

    pub async fn prune_segments(&mut self) -> Result<Vec<String>> {
        let candidates = self.repository.get_pruning_candidates(&self.pruning_config).await?;
        let mut pruned = Vec::new();

        for candidate in candidates {
            if candidate.is_candidate_for_pruning(&self.pruning_config) {
                self.repository.remove_segment(&candidate.segment).await?;
                pruned.push(candidate.segment);
            }
        }

        Ok(pruned)
    }

    pub async fn mark_segment_accessed(&mut self, segment: &str) -> Result<()> {
        if let Some(mut stats) = self.repository.get_segment(segment).await? {
            stats.mark_accessed();
            self.repository.update_segment(&stats).await?;
        }
        Ok(())
    }

    pub fn config(&self) -> &BpeConfig {
        &self.config
    }

    pub fn pruning_config(&self) -> &PruningConfig {
        &self.pruning_config
    }

    pub fn set_config(&mut self, config: BpeConfig) {
        self.config = config;
    }

    pub fn set_pruning_config(&mut self, config: PruningConfig) {
        self.pruning_config = config;
    }
}

/// Get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
