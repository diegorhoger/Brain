//! Segmentation Infrastructure Implementation
//! 
//! This module provides concrete implementations of the segmentation traits
//! defined in brain-core, including BPE algorithm implementation.

use brain_types::*;
use brain_core::{
    BpeConfig, SegmentStats, PruningConfig, BpeStats,
    SegmentRepository, SegmentationProvider,
    segmentation::SegmentPair
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;

/// Get current timestamp
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Context matrix for tracking segment co-occurrences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMatrix {
    /// Maps segment pairs to their co-occurrence counts within context windows
    co_occurrence: HashMap<String, usize>,
    /// Total context observations
    total_observations: usize,
}

impl ContextMatrix {
    pub fn new() -> Self {
        Self {
            co_occurrence: HashMap::new(),
            total_observations: 0,
        }
    }

    fn make_key(&self, seg1: &str, seg2: &str) -> String {
        if seg1 <= seg2 {
            format!("{}|{}", seg1, seg2)
        } else {
            format!("{}|{}", seg2, seg1)
        }
    }

    pub fn record_co_occurrence(&mut self, seg1: &str, seg2: &str) {
        let key = self.make_key(seg1, seg2);
        *self.co_occurrence.entry(key).or_insert(0) += 1;
        self.total_observations += 1;
    }

    pub fn get_co_occurrence_strength(&self, seg1: &str, seg2: &str) -> f64 {
        if self.total_observations == 0 {
            return 0.0;
        }
        
        let key = self.make_key(seg1, seg2);
        let count = self.co_occurrence.get(&key).copied().unwrap_or(0);
        count as f64 / self.total_observations as f64
    }
}

/// Entropy analyzer for boundary detection
#[derive(Debug, Clone)]
pub struct EntropyAnalyzer {
    /// Window size for entropy calculation
    window_size: usize,
}

impl EntropyAnalyzer {
    pub fn new(window_size: usize) -> Self {
        Self { window_size }
    }

    pub fn calculate_position_entropies(&self, text: &str) -> Vec<f64> {
        let chars: Vec<char> = text.chars().collect();
        let mut entropies = Vec::new();
        
        for i in 0..chars.len() {
            let entropy = self.calculate_entropy_at_position(&chars, i);
            entropies.push(entropy);
        }
        
        entropies
    }

    fn calculate_entropy_at_position(&self, chars: &[char], position: usize) -> f64 {
        let start = position.saturating_sub(self.window_size / 2);
        let end = (position + self.window_size / 2 + 1).min(chars.len());
        
        if end <= start {
            return 0.0;
        }
        
        let window = &chars[start..end];
        self.calculate_shannon_entropy(window)
    }

    fn calculate_shannon_entropy(&self, chars: &[char]) -> f64 {
        if chars.is_empty() {
            return 0.0;
        }
        
        let mut frequency = HashMap::new();
        for &ch in chars {
            *frequency.entry(ch).or_insert(0) += 1;
        }
        
        let total = chars.len() as f64;
        let mut entropy = 0.0;
        
        for count in frequency.values() {
            let p = *count as f64 / total;
            if p > 0.0 {
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
}

/// In-memory segment repository implementation
pub struct InMemorySegmentRepository {
    segments: HashMap<String, SegmentStats>,
    archived_segments: HashMap<String, SegmentStats>,
}

impl InMemorySegmentRepository {
    pub fn new() -> Self {
        Self {
            segments: HashMap::new(),
            archived_segments: HashMap::new(),
        }
    }
}

#[async_trait]
impl SegmentRepository for InMemorySegmentRepository {
    async fn store_segment(&mut self, stats: SegmentStats) -> Result<()> {
        self.segments.insert(stats.segment.clone(), stats);
        Ok(())
    }

    async fn get_segment(&self, segment: &str) -> Result<Option<SegmentStats>> {
        Ok(self.segments.get(segment).cloned())
    }

    async fn update_segment(&mut self, stats: &SegmentStats) -> Result<()> {
        self.segments.insert(stats.segment.clone(), stats.clone());
        Ok(())
    }

    async fn remove_segment(&mut self, segment: &str) -> Result<()> {
        self.segments.remove(segment);
        Ok(())
    }

    async fn get_all_segments(&self) -> Result<Vec<SegmentStats>> {
        Ok(self.segments.values().cloned().collect())
    }

    async fn get_segments_by_frequency(&self) -> Result<Vec<SegmentStats>> {
        let mut segments: Vec<SegmentStats> = self.segments.values().cloned().collect();
        segments.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        Ok(segments)
    }

    async fn get_segments_by_confidence(&self) -> Result<Vec<SegmentStats>> {
        let mut segments: Vec<SegmentStats> = self.segments.values().cloned().collect();
        segments.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        Ok(segments)
    }

    async fn get_high_confidence_segments(&self) -> Result<Vec<SegmentStats>> {
        Ok(self.segments.values()
            .filter(|s| s.confidence >= 0.7)
            .cloned()
            .collect())
    }

    async fn get_pruning_candidates(&self, config: &PruningConfig) -> Result<Vec<SegmentStats>> {
        Ok(self.segments.values()
            .filter(|s| s.is_candidate_for_pruning(config))
            .cloned()
            .collect())
    }

    async fn archive_segment(&mut self, segment: &str) -> Result<bool> {
        if let Some(mut stats) = self.segments.remove(segment) {
            stats.archive();
            self.archived_segments.insert(segment.to_string(), stats);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn restore_from_archive(&mut self, segment: &str) -> Result<bool> {
        if let Some(mut stats) = self.archived_segments.remove(segment) {
            stats.is_archived = false;
            stats.last_modified = current_timestamp();
            self.segments.insert(segment.to_string(), stats);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// BPE segmenter implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpeSegmenter {
    config: BpeConfig,
    segments: HashMap<String, SegmentStats>,
    pair_frequencies: HashMap<String, usize>,
    merge_count: usize,
    context_matrix: ContextMatrix,
    training_text: String,
}

impl BpeSegmenter {
    pub fn new(config: BpeConfig) -> Self {
        Self {
            config,
            segments: HashMap::new(),
            pair_frequencies: HashMap::new(),
            merge_count: 0,
            context_matrix: ContextMatrix::new(),
            training_text: String::new(),
        }
    }

    pub fn default() -> Self {
        Self::new(BpeConfig::default())
    }

    pub fn initialize_from_text(&mut self, text: &str) -> Result<()> {
        self.training_text = text.to_string();
        
        // Initialize character segments
        if self.config.include_chars {
            let mut char_frequencies = HashMap::new();
            for ch in text.chars() {
                *char_frequencies.entry(ch).or_insert(0) += 1;
            }

            for (ch, frequency) in char_frequencies {
                let mut stats = SegmentStats::new_char(ch);
                stats.frequency = frequency;
                self.segments.insert(stats.segment.clone(), stats);
            }
        }

        // Count initial pair frequencies
        self.count_initial_frequencies(text)?;
        
        Ok(())
    }

    fn count_initial_frequencies(&mut self, text: &str) -> Result<()> {
        let chars: Vec<char> = text.chars().collect();
        
        // Count adjacent character pairs
        for window in chars.windows(2) {
            if window.len() == 2 {
                let left = window[0].to_string();
                let right = window[1].to_string();
                let pair = SegmentPair::new(left, right);
                let key = Self::pair_to_key(&pair);
                *self.pair_frequencies.entry(key).or_insert(0) += 1;
            }
        }
        
        Ok(())
    }

    pub fn train(&mut self) -> Result<()> {
        // Perform BPE merge operations
        for step in 0..self.config.num_merges {
            if !self.perform_merge_step(step)? {
                break; // No more merges possible
            }
        }
        
        Ok(())
    }

    fn perform_merge_step(&mut self, step: usize) -> Result<bool> {
        // Find the most frequent pair
        if let Some((pair, frequency)) = self.find_most_frequent_pair()? {
            let new_segment = pair.merged();
            
            // Create new segment statistics
            let new_stats = SegmentStats::new_merged(pair.clone(), frequency, step);
            
            // Update frequencies and remove old pair
            self.update_frequencies_after_merge(&pair)?;
            
            // Add new segment
            self.segments.insert(new_segment.clone(), new_stats);
            self.merge_count += 1;
            
            Ok(true)
        } else {
            Ok(false) // No more pairs to merge
        }
    }

    fn find_most_frequent_pair(&self) -> Result<Option<(SegmentPair, usize)>> {
        let mut best_pair = None;
        let mut best_frequency = 0;
        
        for (key, &frequency) in &self.pair_frequencies {
            if frequency >= self.config.min_frequency && frequency > best_frequency {
                if let Some(pair) = Self::key_to_pair(key) {
                    best_pair = Some(pair);
                    best_frequency = frequency;
                }
            }
        }
        
        Ok(best_pair.map(|pair| (pair, best_frequency)))
    }

    fn update_frequencies_after_merge(&mut self, merged_pair: &SegmentPair) -> Result<()> {
        // Remove the merged pair from frequencies
        let key = Self::pair_to_key(merged_pair);
        self.pair_frequencies.remove(&key);
        
        Ok(())
    }

    fn pair_to_key(pair: &SegmentPair) -> String {
        format!("{}|{}", pair.left, pair.right)
    }

    fn key_to_pair(key: &str) -> Option<SegmentPair> {
        let parts: Vec<&str> = key.split('|').collect();
        if parts.len() == 2 {
            Some(SegmentPair::new(parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }

    // Utility methods for accessing segment data
    pub fn get_segments_by_frequency(&self) -> Vec<&SegmentStats> {
        let mut segments: Vec<&SegmentStats> = self.segments.values().collect();
        segments.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        segments
    }

    pub fn get_segments_by_confidence(&self) -> Vec<&SegmentStats> {
        let mut segments: Vec<&SegmentStats> = self.segments.values().collect();
        segments.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        segments
    }

    pub fn get_high_confidence_segments(&self) -> Vec<&SegmentStats> {
        self.segments.values()
            .filter(|s| s.confidence >= 0.7)
            .collect()
    }

    pub fn vocab_size(&self) -> usize {
        self.segments.len()
    }

    pub fn merge_count(&self) -> usize {
        self.merge_count
    }

    pub fn segment_text(&self, text: &str) -> Vec<String> {
        // Simple segmentation - in practice this would use the learned vocabulary
        // For now, return character-level segmentation
        text.chars().map(|c| c.to_string()).collect()
    }

    pub fn get_stats(&self) -> BpeStats {
        let character_segments = self.segments.values().filter(|s| s.length == 1).count();
        let merged_segments = self.segments.values().filter(|s| s.formed_from.is_some()).count();
        let high_confidence_segments = self.segments.values().filter(|s| s.confidence >= 0.7).count();
        
        let average_confidence = if self.segments.is_empty() {
            0.0
        } else {
            self.segments.values().map(|s| s.confidence).sum::<f64>() / self.segments.len() as f64
        };

        let average_entropy = if self.segments.is_empty() {
            0.0
        } else {
            self.segments.values().map(|s| s.entropy).sum::<f64>() / self.segments.len() as f64
        };

        let max_segment_length = self.segments.values().map(|s| s.length).max().unwrap_or(0);

        BpeStats {
            total_segments: self.segments.len(),
            character_segments,
            merged_segments,
            merges_performed: self.merge_count,
            max_segment_length,
            high_confidence_segments,
            average_confidence,
            average_entropy,
            context_observations: self.context_matrix.total_observations,
        }
    }

    pub fn get_all_segments(&self) -> Vec<String> {
        self.segments.keys().cloned().collect()
    }

    pub fn get_segment_stats_by_string(&self, segment: &str) -> Option<&SegmentStats> {
        self.segments.get(segment)
    }
}

impl SegmentationProvider for BpeSegmenter {
    fn get_segments(&self) -> Vec<String> {
        self.get_all_segments()
    }

    fn segment_text(&self, text: &str) -> Vec<String> {
        self.segment_text(text)
    }

    fn get_segment_stats(&self, segment: &str) -> Option<SegmentStats> {
        self.get_segment_stats_by_string(segment).cloned()
    }

    fn get_high_confidence_segments(&self) -> Vec<String> {
        self.get_high_confidence_segments()
            .into_iter()
            .map(|s| s.segment.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpe_config_default() {
        let config = BpeConfig::default();
        assert_eq!(config.min_frequency, 2);
        assert_eq!(config.max_vocab_size, 10000);
        assert_eq!(config.num_merges, 1000);
        assert!(config.include_chars);
        assert!(config.enable_advanced_heuristics);
    }

    #[test]
    fn test_segment_pair() {
        let pair = SegmentPair::new("a".to_string(), "b".to_string());
        assert_eq!(pair.merged(), "ab");
    }

    #[tokio::test]
    async fn test_in_memory_repository() -> Result<()> {
        let mut repo = InMemorySegmentRepository::new();
        let stats = SegmentStats::new_char('a');
        
        repo.store_segment(stats.clone()).await?;
        let retrieved = repo.get_segment("a").await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().segment, "a");
        
        Ok(())
    }

    #[test]
    fn test_bpe_initialization() -> Result<()> {
        let mut segmenter = BpeSegmenter::new(BpeConfig::default());
        segmenter.initialize_from_text("hello world")?;
        
        assert!(segmenter.vocab_size() > 0);
        assert!(segmenter.get_all_segments().contains(&"h".to_string()));
        assert!(segmenter.get_all_segments().contains(&"e".to_string()));
        
        Ok(())
    }

    #[test]
    fn test_context_matrix() {
        let mut matrix = ContextMatrix::new();
        matrix.record_co_occurrence("a", "b");
        matrix.record_co_occurrence("b", "a"); // Should be same as above
        matrix.record_co_occurrence("c", "d");
        
        assert!(matrix.get_co_occurrence_strength("a", "b") > 0.0);
        assert!(matrix.get_co_occurrence_strength("b", "a") > 0.0);
        assert_eq!(matrix.get_co_occurrence_strength("a", "b"), matrix.get_co_occurrence_strength("b", "a"));
    }

    #[test]
    fn test_entropy_analyzer() {
        let analyzer = EntropyAnalyzer::new(3);
        let entropies = analyzer.calculate_position_entropies("hello");
        assert_eq!(entropies.len(), 5);
        assert!(entropies.iter().all(|&e| e >= 0.0));
    }
} 