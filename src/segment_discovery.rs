//! Segment Discovery Module
//! 
//! This module implements Byte-Pair Encoding (BPE) and related algorithms for 
//! identifying recurring character patterns and building segment vocabularies.
//! It builds upon the Character Ingestion Engine to discover higher-level patterns.
//! 
//! Advanced heuristics include entropy-based boundary detection, contextual 
//! co-occurrence tracking, and confidence scoring for improved segmentation quality.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use crate::integration::{SegmentProvider, PredictionFeedbackTrait, PerformanceMetrics, PredictionFeedback};

/// Configuration for BPE algorithm with advanced heuristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpeConfig {
    /// Minimum frequency for a pair to be considered for merging
    pub min_frequency: usize,
    /// Maximum vocabulary size to limit memory usage
    pub max_vocab_size: usize,
    /// Number of merge operations to perform
    pub num_merges: usize,
    /// Whether to include single characters in initial vocabulary
    pub include_chars: bool,
    /// Minimum entropy threshold for boundary detection
    pub min_entropy_threshold: f64,
    /// Context window size for co-occurrence tracking
    pub context_window_size: usize,
    /// Minimum confidence score for segments
    pub min_confidence: f64,
    /// Enable advanced heuristics
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

/// Segment statistics for tracking usage and patterns with advanced metrics and lifecycle data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentStats {
    pub segment: String,
    pub frequency: usize,
    pub length: usize,
    pub formed_from: Option<SegmentPair>,
    pub merge_step: Option<usize>,
    /// Confidence score based on frequency, stability, and context (0.0-1.0)
    pub confidence: f64,
    /// Entropy score for boundary detection
    pub entropy: f64,
    /// Context stability score
    pub context_stability: f64,
    /// Timestamp when this segment was first created
    pub created_at: u64,
    /// Timestamp when this segment was last accessed/used
    pub last_accessed: u64,
    /// Timestamp when this segment was last modified
    pub last_modified: u64,
    /// Number of times this segment has been accessed
    pub access_count: usize,
    /// Whether this segment is archived (not actively used)
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
            confidence: 0.5, // Default confidence for characters
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
            confidence: 0.0, // Will be calculated
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
        let length_bonus = (self.length as f64).ln() / 10.0; // Longer segments get bonus
        self.confidence = (frequency_score + stability_factor + length_bonus).min(1.0);
        self.last_modified = current_timestamp();
    }

    /// Mark this segment as accessed, updating access count and timestamp
    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
        self.last_accessed = current_timestamp();
    }

    /// Check if this segment should be considered for pruning based on age and usage
    pub fn is_candidate_for_pruning(&self, config: &PruningConfig) -> bool {
        let now = current_timestamp();
        let age_days = (now - self.created_at) / (24 * 60 * 60);
        let days_since_access = (now - self.last_accessed) / (24 * 60 * 60);
        
        // Don't prune if archived (user decision)
        if self.is_archived {
            return false;
        }
        
        // Prune if below minimum confidence and old enough
        if self.confidence < config.min_confidence_threshold && age_days > config.min_age_days {
            return true;
        }
        
        // Prune if not accessed recently and has low usage
        if days_since_access > config.max_inactive_days && self.access_count < config.min_access_count {
            return true;
        }
        
        false
    }

    /// Archive this segment (mark as archived but don't delete)
    pub fn archive(&mut self) {
        self.is_archived = true;
        self.last_modified = current_timestamp();
    }
}

/// Configuration for segment pruning policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruningConfig {
    /// Minimum confidence threshold for keeping segments
    pub min_confidence_threshold: f64,
    /// Minimum age in days before a segment can be pruned
    pub min_age_days: u64,
    /// Maximum days of inactivity before pruning consideration
    pub max_inactive_days: u64,
    /// Minimum access count to avoid pruning
    pub min_access_count: usize,
    /// Maximum number of segments to keep (0 = unlimited)
    pub max_segments: usize,
    /// Whether to enable automatic pruning
    pub enable_auto_pruning: bool,
}

impl Default for PruningConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.1, // Very low threshold
            min_age_days: 7,               // At least a week old
            max_inactive_days: 30,         // Not accessed for a month
            min_access_count: 5,           // Used at least 5 times
            max_segments: 50000,           // Reasonable limit
            enable_auto_pruning: true,
        }
    }
}

/// Storage configuration for persistent segment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Path to the main segments database file
    pub segments_path: String,
    /// Path to the archived segments file
    pub archive_path: String,
    /// Path to the context matrix file
    pub context_path: String,
    /// How often to save to disk (0 = save immediately)
    pub save_interval_seconds: u64,
    /// Whether to compress stored data
    pub compress_data: bool,
    /// Maximum backup files to keep
    pub max_backups: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            segments_path: "segments.json".to_string(),
            archive_path: "segments_archive.json".to_string(),
            context_path: "context_matrix.json".to_string(),
            save_interval_seconds: 300, // Save every 5 minutes
            compress_data: false,       // Keep readable for now
            max_backups: 5,
        }
    }
}

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Context tracking for co-occurrence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMatrix {
    /// Maps segment pairs to their co-occurrence counts within context windows
    /// Key format: "segment1|segment2" (lexicographically ordered)
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

    /// Create a serializable key for segment pairs
    fn make_key(&self, seg1: &str, seg2: &str) -> String {
        if seg1 < seg2 {
            format!("{}|{}", seg1, seg2)
        } else {
            format!("{}|{}", seg2, seg1)
        }
    }

    /// Record co-occurrence of two segments within context window
    pub fn record_co_occurrence(&mut self, seg1: &str, seg2: &str) {
        let key = self.make_key(seg1, seg2);
        *self.co_occurrence.entry(key).or_insert(0) += 1;
        self.total_observations += 1;
    }

    /// Get co-occurrence strength between two segments (0.0-1.0)
    pub fn get_co_occurrence_strength(&self, seg1: &str, seg2: &str) -> f64 {
        let key = self.make_key(seg1, seg2);
        
        if let Some(&count) = self.co_occurrence.get(&key) {
            count as f64 / self.total_observations.max(1) as f64
        } else {
            0.0
        }
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

    /// Calculate entropy at character positions to identify likely boundaries
    pub fn calculate_position_entropies(&self, text: &str) -> Vec<f64> {
        let chars: Vec<char> = text.chars().collect();
        let mut entropies = Vec::new();

        for i in 0..chars.len() {
            let entropy = self.calculate_entropy_at_position(&chars, i);
            entropies.push(entropy);
        }

        entropies
    }

    /// Calculate entropy at a specific position using sliding window
    fn calculate_entropy_at_position(&self, chars: &[char], position: usize) -> f64 {
        let start = position.saturating_sub(self.window_size / 2);
        let end = (position + self.window_size / 2).min(chars.len());
        
        if end <= start {
            return 0.0;
        }

        let window = &chars[start..end];
        self.calculate_shannon_entropy(window)
    }

    /// Calculate Shannon entropy for a sequence of characters
    fn calculate_shannon_entropy(&self, chars: &[char]) -> f64 {
        if chars.is_empty() {
            return 0.0;
        }

        let mut counts = HashMap::new();
        for &ch in chars {
            *counts.entry(ch).or_insert(0) += 1;
        }

        let total = chars.len() as f64;
        let mut entropy = 0.0;

        for &count in counts.values() {
            let probability = count as f64 / total;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }

        entropy
    }
}

/// Byte-Pair Encoding segmenter with advanced heuristics and persistent storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpeSegmenter {
    config: BpeConfig,
    /// Storage configuration for persistence
    storage_config: StorageConfig,
    /// Pruning configuration for lifecycle management
    pruning_config: PruningConfig,
    /// Vocabulary of discovered segments with their statistics
    segments: HashMap<String, SegmentStats>,
    /// Frequency counts for pairs (key format: "left|right")
    pair_frequencies: HashMap<String, usize>,
    /// Number of merge operations performed
    merge_count: usize,
    /// Context tracking matrix
    context_matrix: ContextMatrix,
    /// Training text for entropy analysis
    training_text: String,
    /// Timestamp of last save operation
    last_save: u64,
    /// Archived segments for historical analysis
    archived_segments: HashMap<String, SegmentStats>,
}

impl BpeSegmenter {
    /// Convert SegmentPair to serializable string key
    fn pair_to_key(pair: &SegmentPair) -> String {
        format!("{}|{}", pair.left, pair.right)
    }

    /// Convert string key back to SegmentPair
    fn key_to_pair(key: &str) -> Option<SegmentPair> {
        let parts: Vec<&str> = key.split('|').collect();
        if parts.len() == 2 {
            Some(SegmentPair::new(parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }

    /// Create a new BPE segmenter with optional storage configuration
    pub fn new(config: BpeConfig) -> Self {
        Self::with_storage(config, StorageConfig::default(), PruningConfig::default())
    }

    /// Create a new BPE segmenter with custom storage and pruning configuration
    pub fn with_storage(config: BpeConfig, storage_config: StorageConfig, pruning_config: PruningConfig) -> Self {
        Self {
            config,
            storage_config,
            pruning_config,
            segments: HashMap::new(),
            pair_frequencies: HashMap::new(),
            merge_count: 0,
            context_matrix: ContextMatrix::new(),
            training_text: String::new(),
            last_save: current_timestamp(),
            archived_segments: HashMap::new(),
        }
    }

    /// Create a default BPE segmenter
    pub fn default() -> Self {
        Self::new(BpeConfig::default())
    }

    /// Load segmenter from persistent storage
    pub fn load_from_storage<P: AsRef<Path>>(storage_path: P) -> Result<Self> {
        let file = File::open(storage_path)?;
        let reader = BufReader::new(file);
        let mut segmenter: Self = serde_json::from_reader(reader)?;
        
        // Load context matrix if available
        if Path::new(&segmenter.storage_config.context_path).exists() {
            segmenter.load_context_matrix()?;
        }
        
        // Load archived segments if available
        if Path::new(&segmenter.storage_config.archive_path).exists() {
            segmenter.load_archived_segments()?;
        }
        
        Ok(segmenter)
    }

    /// Save segmenter to persistent storage
    pub fn save_to_storage<P: AsRef<Path>>(&mut self, storage_path: P) -> Result<()> {
        // Create backup if file exists
        if storage_path.as_ref().exists() {
            self.create_backup(&storage_path)?;
        }
        
        let file = File::create(storage_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        
        // Save context matrix separately
        self.save_context_matrix()?;
        
        // Save archived segments separately
        self.save_archived_segments()?;
        
        self.last_save = current_timestamp();
        Ok(())
    }

    /// Check if automatic save is needed based on time interval
    pub fn should_auto_save(&self) -> bool {
        if self.storage_config.save_interval_seconds == 0 {
            return false; // Immediate save mode
        }
        
        let now = current_timestamp();
        (now - self.last_save) >= self.storage_config.save_interval_seconds
    }

    /// Perform automatic save if needed
    pub fn auto_save_if_needed(&mut self) -> Result<()> {
        if self.should_auto_save() {
            self.save_to_storage(&self.storage_config.segments_path.clone())?;
        }
        Ok(())
    }

    /// Create a backup of the current storage file
    fn create_backup<P: AsRef<Path>>(&self, storage_path: P) -> Result<()> {
        let storage_path = storage_path.as_ref();
        let timestamp = current_timestamp();
        let backup_path = format!("{}.backup.{}", 
            storage_path.to_string_lossy(), 
            timestamp
        );
        
        std::fs::copy(storage_path, &backup_path)?;
        
        // Clean up old backups
        self.cleanup_old_backups(storage_path)?;
        
        Ok(())
    }

    /// Clean up old backup files
    fn cleanup_old_backups<P: AsRef<Path>>(&self, storage_path: P) -> Result<()> {
        let storage_path = storage_path.as_ref();
        let storage_name = storage_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("segments");
        
        let parent_dir = storage_path.parent().unwrap_or(Path::new("."));
        
        // Find all backup files
        let mut backups = Vec::new();
        for entry in std::fs::read_dir(parent_dir)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            
            if file_name_str.starts_with(&format!("{}.backup.", storage_name)) {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(created) = metadata.created() {
                        backups.push((entry.path(), created));
                    }
                }
            }
        }
        
        // Sort by creation time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Remove excess backups
        for (path, _) in backups.iter().skip(self.storage_config.max_backups) {
            let _ = std::fs::remove_file(path); // Ignore errors
        }
        
        Ok(())
    }

    /// Save context matrix to separate file
    fn save_context_matrix(&self) -> Result<()> {
        let file = File::create(&self.storage_config.context_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.context_matrix)?;
        Ok(())
    }

    /// Load context matrix from separate file
    fn load_context_matrix(&mut self) -> Result<()> {
        let file = File::open(&self.storage_config.context_path)?;
        let reader = BufReader::new(file);
        self.context_matrix = serde_json::from_reader(reader)?;
        Ok(())
    }

    /// Save archived segments to separate file
    fn save_archived_segments(&self) -> Result<()> {
        let file = File::create(&self.storage_config.archive_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.archived_segments)?;
        Ok(())
    }

    /// Load archived segments from separate file
    fn load_archived_segments(&mut self) -> Result<()> {
        let file = File::open(&self.storage_config.archive_path)?;
        let reader = BufReader::new(file);
        self.archived_segments = serde_json::from_reader(reader)?;
        Ok(())
    }

    /// Perform segment pruning based on configuration
    pub fn prune_segments(&mut self) -> Result<Vec<String>> {
        if !self.pruning_config.enable_auto_pruning {
            return Ok(Vec::new());
        }

        let mut pruned_segments = Vec::new();
        let mut segments_to_archive = Vec::new();

        // Identify segments for pruning
        for (segment, stats) in &self.segments {
            if stats.is_candidate_for_pruning(&self.pruning_config) {
                segments_to_archive.push(segment.clone());
            }
        }

        // Move segments to archive
        for segment in segments_to_archive {
            if let Some(mut stats) = self.segments.remove(&segment) {
                stats.archive();
                self.archived_segments.insert(segment.clone(), stats);
                pruned_segments.push(segment);
            }
        }

        // Enforce max segments limit if configured
        if self.pruning_config.max_segments > 0 && self.segments.len() > self.pruning_config.max_segments {
            let excess = self.segments.len() - self.pruning_config.max_segments;
            
            // Sort by confidence (ascending) to remove least confident first
            let mut segment_confidence: Vec<_> = self.segments.iter()
                .map(|(seg, stats)| (seg.clone(), stats.confidence))
                .collect();
            segment_confidence.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            
            for (segment, _) in segment_confidence.iter().take(excess) {
                if let Some(mut stats) = self.segments.remove(segment) {
                    stats.archive();
                    self.archived_segments.insert(segment.clone(), stats);
                    pruned_segments.push(segment.clone());
                }
            }
        }

        Ok(pruned_segments)
    }

    /// Get segments that are candidates for pruning (without actually pruning them)
    pub fn get_pruning_candidates(&self) -> Vec<&SegmentStats> {
        self.segments.values()
            .filter(|stats| stats.is_candidate_for_pruning(&self.pruning_config))
            .collect()
    }

    /// Mark a segment as accessed (for usage tracking)
    pub fn mark_segment_accessed(&mut self, segment: &str) {
        if let Some(stats) = self.segments.get_mut(segment) {
            stats.mark_accessed();
        }
    }

    /// Get archived segments
    pub fn get_archived_segments(&self) -> &HashMap<String, SegmentStats> {
        &self.archived_segments
    }

    /// Restore a segment from archive
    pub fn restore_from_archive(&mut self, segment: &str) -> Result<bool> {
        if let Some(mut stats) = self.archived_segments.remove(segment) {
            stats.is_archived = false;
            stats.last_modified = current_timestamp();
            self.segments.insert(segment.to_string(), stats);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Initialize vocabulary from text corpus
    pub fn initialize_from_text(&mut self, text: &str) -> Result<()> {
        self.training_text = text.to_string();
        
        // Initialize character vocabulary if enabled
        if self.config.include_chars {
            let chars: std::collections::HashSet<char> = text.chars().collect();
            for ch in chars {
                let stats = SegmentStats::new_char(ch);
                self.segments.insert(ch.to_string(), stats);
            }
        }
        
        // Tokenize text into initial characters and count frequencies
        self.count_initial_frequencies(text)?;
        
        // Initialize context tracking if advanced heuristics are enabled
        if self.config.enable_advanced_heuristics {
            self.initialize_context_tracking(text)?;
        }
        
        Ok(())
    }
    
    /// Initialize context tracking from text
    fn initialize_context_tracking(&mut self, text: &str) -> Result<()> {
        let chars: Vec<char> = text.chars().collect();
        let window_size = self.config.context_window_size;
        
        for window in chars.windows(window_size) {
            for i in 0..window.len() {
                for j in (i + 1)..window.len() {
                    let seg1 = window[i].to_string();
                    let seg2 = window[j].to_string();
                    self.context_matrix.record_co_occurrence(&seg1, &seg2);
                }
            }
        }
        
        Ok(())
    }
    
    /// Count initial character pair frequencies
    fn count_initial_frequencies(&mut self, text: &str) -> Result<()> {
        let chars: Vec<char> = text.chars().collect();
        
        // Count character pairs
        for window in chars.windows(2) {
            if window.len() == 2 {
                let pair = SegmentPair::new(
                    window[0].to_string(),
                    window[1].to_string()
                );
                let key = Self::pair_to_key(&pair);
                *self.pair_frequencies.entry(key).or_insert(0) += 1;
                
                // Update individual character frequencies
                if self.config.include_chars {
                    if let Some(left_stats) = self.segments.get_mut(&window[0].to_string()) {
                        left_stats.frequency += 1;
                    }
                    if let Some(right_stats) = self.segments.get_mut(&window[1].to_string()) {
                        right_stats.frequency += 1;
                    }
                }
            }
        }
        
        // Handle last character frequency
        if let Some(&last_char) = chars.last() {
            if self.config.include_chars {
                if let Some(stats) = self.segments.get_mut(&last_char.to_string()) {
                    stats.frequency += 1;
                }
            }
        }
        
        Ok(())
    }
    
    /// Perform BPE training to discover segments with advanced heuristics
    pub fn train(&mut self) -> Result<()> {
        for step in 0..self.config.num_merges {
            if !self.perform_merge_step(step)? {
                break; // No more valid merges
            }
            
            if self.segments.len() >= self.config.max_vocab_size {
                break; // Reached vocabulary size limit
            }
        }
        
        // Apply advanced heuristics if enabled
        if self.config.enable_advanced_heuristics {
            self.apply_advanced_heuristics()?;
        }
        
        Ok(())
    }
    
    /// Apply advanced heuristics for improved segmentation
    fn apply_advanced_heuristics(&mut self) -> Result<()> {
        // Calculate entropy scores
        self.calculate_entropy_scores()?;
        
        // Update confidence scores
        self.update_confidence_scores()?;
        
        // Perform segment splitting based on entropy analysis
        self.perform_entropy_based_splitting()?;
        
        // Update context stability scores
        self.update_context_stability()?;
        
        Ok(())
    }
    
    /// Calculate entropy scores for segments
    fn calculate_entropy_scores(&mut self) -> Result<()> {
        if self.training_text.is_empty() {
            return Ok(());
        }
        
        let analyzer = EntropyAnalyzer::new(5); // Use window size of 5
        let entropies = analyzer.calculate_position_entropies(&self.training_text);
        
        // Update entropy scores for segments based on their positions
        for (segment_str, stats) in &mut self.segments {
            if let Some(pos) = self.training_text.find(segment_str) {
                if pos < entropies.len() {
                    stats.entropy = entropies[pos];
                }
            }
        }
        
        Ok(())
    }
    
    /// Update confidence scores for all segments
    fn update_confidence_scores(&mut self) -> Result<()> {
        let total_frequency: usize = self.segments.values().map(|s| s.frequency).sum();
        
        for stats in self.segments.values_mut() {
            let stability_factor = stats.context_stability;
            stats.update_confidence(total_frequency, stability_factor);
        }
        
        Ok(())
    }
    
    /// Perform entropy-based segment splitting
    fn perform_entropy_based_splitting(&mut self) -> Result<()> {
        let segments_to_split: Vec<String> = self.segments
            .iter()
            .filter(|(_, stats)| {
                stats.length > 2 && 
                stats.entropy > self.config.min_entropy_threshold &&
                stats.confidence < self.config.min_confidence
            })
            .map(|(segment, _)| segment.clone())
            .collect();
        
        for segment in segments_to_split {
            self.split_segment(&segment)?;
        }
        
        Ok(())
    }
    
    /// Split a segment into smaller parts if beneficial
    fn split_segment(&mut self, segment: &str) -> Result<()> {
        if segment.len() < 3 {
            return Ok(()); // Too short to split meaningfully
        }
        
        // Find the best split point (simplified heuristic)
        let mid_point = segment.len() / 2;
        let (left_part, right_part) = segment.split_at(mid_point);
        
        // Only split if both parts would be meaningful
        if left_part.len() > 0 && right_part.len() > 0 {
            // Remove original segment
            self.segments.remove(segment);
            
            // Add split segments if they don't already exist
            if !self.segments.contains_key(left_part) {
                self.segments.insert(left_part.to_string(), SegmentStats::new_char(
                    left_part.chars().next().unwrap_or('?')
                ));
            }
            if !self.segments.contains_key(right_part) {
                self.segments.insert(right_part.to_string(), SegmentStats::new_char(
                    right_part.chars().next().unwrap_or('?')
                ));
            }
        }
        
        Ok(())
    }
    
    /// Update context stability scores
    fn update_context_stability(&mut self) -> Result<()> {
        // Collect segment keys to avoid borrowing conflicts
        let segment_keys: Vec<String> = self.segments.keys().cloned().collect();
        
        for segment_str in &segment_keys {
            let mut stability_sum = 0.0;
            let mut count = 0;
            
            // Calculate average co-occurrence strength with other segments
            for other_segment in &segment_keys {
                if other_segment != segment_str {
                    let strength = self.context_matrix.get_co_occurrence_strength(segment_str, other_segment);
                    stability_sum += strength;
                    count += 1;
                }
            }
            
            let stability = if count > 0 {
                stability_sum / count as f64
            } else {
                0.0
            };
            
            // Update the specific segment's context stability
            if let Some(stats) = self.segments.get_mut(segment_str) {
                stats.context_stability = stability;
            }
        }
        
        Ok(())
    }
    
    /// Perform a single merge step
    fn perform_merge_step(&mut self, step: usize) -> Result<bool> {
        // Find the most frequent pair
        let best_pair = self.find_most_frequent_pair()?;
        
        match best_pair {
            Some((pair, frequency)) => {
                if frequency < self.config.min_frequency {
                    return Ok(false); // No pairs meet minimum frequency
                }
                
                // Create new segment
                let mut new_segment = SegmentStats::new_merged(pair.clone(), frequency, step);
                
                // Calculate initial confidence if advanced heuristics are enabled
                if self.config.enable_advanced_heuristics {
                    let total_freq: usize = self.segments.values().map(|s| s.frequency).sum();
                    new_segment.update_confidence(total_freq, 0.0);
                }
                
                let segment_key = new_segment.segment.clone();
                
                // Add to vocabulary
                self.segments.insert(segment_key.clone(), new_segment);
                
                // Remove the merged pair from frequency map
                let pair_key = Self::pair_to_key(&pair);
                self.pair_frequencies.remove(&pair_key);
                
                // Update pair frequencies (this is simplified - in full BPE we'd 
                // re-tokenize the corpus with the new segment)
                self.update_frequencies_after_merge(&pair, &segment_key)?;
                
                self.merge_count += 1;
                Ok(true)
            }
            None => Ok(false), // No pairs to merge
        }
    }
    
    /// Find the most frequent pair above minimum threshold
    fn find_most_frequent_pair(&self) -> Result<Option<(SegmentPair, usize)>> {
        let mut best_pair: Option<(SegmentPair, usize)> = None;
        
        for (key, &frequency) in &self.pair_frequencies {
            if frequency >= self.config.min_frequency {
                if let Some(pair) = Self::key_to_pair(key) {
                    match &best_pair {
                        None => best_pair = Some((pair, frequency)),
                        Some((_, best_freq)) => {
                            if frequency > *best_freq {
                                best_pair = Some((pair, frequency));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(best_pair)
    }
    
    /// Update frequency counts after merging a pair
    fn update_frequencies_after_merge(&mut self, merged_pair: &SegmentPair, _new_segment: &str) -> Result<()> {
        // This is a simplified version. In full BPE, we would:
        // 1. Re-tokenize the entire corpus with the new segment
        // 2. Recalculate all pair frequencies
        // For now, we just remove pairs that contain the merged components
        
        let keys_to_remove: Vec<String> = self.pair_frequencies
            .keys()
            .filter(|key| {
                if let Some(pair) = Self::key_to_pair(key) {
                    pair.left == merged_pair.left || 
                    pair.right == merged_pair.right ||
                    pair.left == merged_pair.right ||
                    pair.right == merged_pair.left
                } else {
                    false
                }
            })
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            self.pair_frequencies.remove(&key);
        }
        
        Ok(())
    }
    
    /// Get discovered segments sorted by frequency
    pub fn get_segments_by_frequency(&self) -> Vec<&SegmentStats> {
        let mut segments: Vec<&SegmentStats> = self.segments.values().collect();
        segments.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        segments
    }
    
    /// Get segments sorted by confidence score
    pub fn get_segments_by_confidence(&self) -> Vec<&SegmentStats> {
        let mut segments: Vec<&SegmentStats> = self.segments.values().collect();
        segments.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        segments
    }
    
    /// Get high-confidence segments above threshold
    pub fn get_high_confidence_segments(&self) -> Vec<&SegmentStats> {
        self.segments
            .values()
            .filter(|stats| stats.confidence >= self.config.min_confidence)
            .collect()
    }
    
    /// Get segments that were formed by merging (not single characters)
    pub fn get_merged_segments(&self) -> Vec<&SegmentStats> {
        self.segments
            .values()
            .filter(|stats| stats.formed_from.is_some())
            .collect()
    }
    
    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.segments.len()
    }
    
    /// Get number of merges performed
    pub fn merge_count(&self) -> usize {
        self.merge_count
    }
    
    /// Get context co-occurrence strength between two segments
    pub fn get_co_occurrence_strength(&self, segment1: &str, segment2: &str) -> f64 {
        self.context_matrix.get_co_occurrence_strength(segment1, segment2)
    }
    
    /// Basic segmentation of text using discovered vocabulary
    /// (Simplified version - would need more sophisticated tokenization)
    pub fn segment_text(&self, text: &str) -> Vec<String> {
        // For now, return character-level segmentation
        // In full implementation, this would use the learned BPE merges
        text.chars().map(|c| c.to_string()).collect()
    }
    
    /// Get statistics about the BPE process with advanced metrics
    pub fn get_stats(&self) -> BpeStats {
        let merged_segments = self.get_merged_segments().len();
        let char_segments = self.segments.len() - merged_segments;
        let high_confidence_segments = self.get_high_confidence_segments().len();
        
        let average_confidence = if !self.segments.is_empty() {
            self.segments.values().map(|s| s.confidence).sum::<f64>() / self.segments.len() as f64
        } else {
            0.0
        };
        
        let average_entropy = if !self.segments.is_empty() {
            self.segments.values().map(|s| s.entropy).sum::<f64>() / self.segments.len() as f64
        } else {
            0.0
        };
        
        BpeStats {
            total_segments: self.segments.len(),
            character_segments: char_segments,
            merged_segments: merged_segments,
            merges_performed: self.merge_count,
            max_segment_length: self.segments.values().map(|s| s.length).max().unwrap_or(0),
            high_confidence_segments: high_confidence_segments,
            average_confidence,
            average_entropy,
            context_observations: self.context_matrix.total_observations,
        }
    }

    /// Get all segments as a vector of strings (for SegmentProvider trait)
    pub fn get_all_segments(&self) -> Vec<String> {
        self.segments.keys().cloned().collect()
    }
    
    /// Get segment statistics by segment string (for SegmentProvider trait)
    pub fn get_segment_stats_by_string(&self, segment: &str) -> Option<&SegmentStats> {
        self.segments.get(segment)
    }
}

/// Implementation of SegmentProvider trait for BpeSegmenter
impl SegmentProvider for BpeSegmenter {
    fn get_segments(&self) -> Vec<String> {
        self.get_all_segments()
    }
    
    fn segment_text(&self, text: &str) -> Vec<String> {
        self.segment_text(text)
    }
    
    fn get_segment_stats(&self, segment: &str) -> Option<crate::segment_discovery::SegmentStats> {
        self.get_segment_stats_by_string(segment).cloned()
    }
    
    fn get_high_confidence_segments(&self) -> Vec<String> {
        self.get_high_confidence_segments()
            .into_iter()
            .map(|stats| stats.segment.clone())
            .collect()
    }
}

/// Extended BpeSegmenter with feedback capabilities
pub struct FeedbackBpeSegmenter {
    /// The underlying BPE segmenter
    pub segmenter: BpeSegmenter,
    /// Performance metrics for tracking prediction feedback
    performance_metrics: PerformanceMetrics,
    /// Segment-specific performance tracking
    segment_performance: std::collections::HashMap<String, PerformanceMetrics>,
}

impl FeedbackBpeSegmenter {
    /// Create a new feedback-enabled BPE segmenter
    pub fn new(config: BpeConfig) -> Self {
        Self {
            segmenter: BpeSegmenter::new(config),
            performance_metrics: PerformanceMetrics::new(),
            segment_performance: std::collections::HashMap::new(),
        }
    }
    
    /// Create with storage configuration
    pub fn with_storage(config: BpeConfig, storage_config: StorageConfig, pruning_config: PruningConfig) -> Self {
        Self {
            segmenter: BpeSegmenter::with_storage(config, storage_config, pruning_config),
            performance_metrics: PerformanceMetrics::new(),
            segment_performance: std::collections::HashMap::new(),
        }
    }
    
    /// Initialize from text and return the feedback-enabled segmenter
    pub fn from_text(text: &str, config: Option<BpeConfig>) -> Result<Self> {
        let mut segmenter = Self::new(config.unwrap_or_default());
        segmenter.segmenter.initialize_from_text(text)?;
        segmenter.segmenter.train()?;
        Ok(segmenter)
    }
    
    /// Get the underlying segmenter
    pub fn get_segmenter(&self) -> &BpeSegmenter {
        &self.segmenter
    }
    
    /// Get mutable reference to the underlying segmenter
    pub fn get_segmenter_mut(&mut self) -> &mut BpeSegmenter {
        &mut self.segmenter
    }
}

impl SegmentProvider for FeedbackBpeSegmenter {
    fn get_segments(&self) -> Vec<String> {
        self.segmenter.get_segments()
    }
    
    fn segment_text(&self, text: &str) -> Vec<String> {
        self.segmenter.segment_text(text)
    }
    
    fn get_segment_stats(&self, segment: &str) -> Option<crate::segment_discovery::SegmentStats> {
        self.segmenter.get_segment_stats(segment)
    }
    
    fn get_high_confidence_segments(&self) -> Vec<String> {
        self.segmenter.get_high_confidence_segments()
            .into_iter()
            .map(|stats| stats.segment.clone())
            .collect()
    }
}

impl PredictionFeedbackTrait for FeedbackBpeSegmenter {
    fn report_prediction(&mut self, feedback: PredictionFeedback) -> Result<()> {
        // Update overall performance metrics
        self.performance_metrics.add_feedback(&feedback);
        
        // Update segment-specific performance if it's a segment prediction
        if feedback.input_type == crate::integration::InputType::Segment {
            let segment_metrics = self.segment_performance
                .entry(feedback.input.clone())
                .or_insert_with(PerformanceMetrics::new);
            segment_metrics.add_feedback(&feedback);
            
            // Mark the segment as accessed in the underlying segmenter
            self.segmenter.mark_segment_accessed(&feedback.input);
        }
        
        Ok(())
    }
    
    fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }
    
    fn reset_metrics(&mut self) {
        self.performance_metrics = PerformanceMetrics::new();
        self.segment_performance.clear();
    }
    
    fn get_high_performing_segments(&self, min_accuracy: f64) -> Vec<String> {
        self.segment_performance
            .iter()
            .filter(|(_, metrics)| metrics.accuracy >= min_accuracy)
            .map(|(segment, _)| segment.clone())
            .collect()
    }
    
    fn get_low_performing_segments(&self, max_accuracy: f64) -> Vec<String> {
        self.segment_performance
            .iter()
            .filter(|(_, metrics)| metrics.accuracy <= max_accuracy)
            .map(|(segment, _)| segment.clone())
            .collect()
    }
}

/// Statistics about the BPE training process with advanced metrics
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpe_config_default() {
        let config = BpeConfig::default();
        assert_eq!(config.min_frequency, 2);
        assert_eq!(config.max_vocab_size, 10000);
        assert!(config.include_chars);
        assert!(config.enable_advanced_heuristics);
        assert_eq!(config.context_window_size, 3);
        assert!(config.min_confidence > 0.0);
    }

    #[test]
    fn test_segment_pair() {
        let pair = SegmentPair::new("a".to_string(), "b".to_string());
        assert_eq!(pair.merged(), "ab");
    }

    #[test]
    fn test_bpe_initialization() -> Result<()> {
        let mut bpe = BpeSegmenter::default();
        let text = "hello world hello";
        
        bpe.initialize_from_text(text)?;
        
        // Should have character segments
        assert!(bpe.vocab_size() > 0);
        
        // Should have pair frequencies
        assert!(!bpe.pair_frequencies.is_empty());
        
        // Should have context observations if advanced heuristics enabled
        if bpe.config.enable_advanced_heuristics {
            assert!(bpe.context_matrix.total_observations > 0);
        }
        
        Ok(())
    }

    #[test]
    fn test_bpe_training() -> Result<()> {
        let config = BpeConfig {
            min_frequency: 2,
            max_vocab_size: 100,
            num_merges: 10,
            include_chars: true,
            min_entropy_threshold: 0.5,
            context_window_size: 3,
            min_confidence: 0.3,
            enable_advanced_heuristics: true,
        };
        
        let mut bpe = BpeSegmenter::new(config);
        let text = "hello hello world world test test";
        
        bpe.initialize_from_text(text)?;
        let initial_vocab_size = bpe.vocab_size();
        
        bpe.train()?;
        
        // Should have performed some merges
        assert!(bpe.merge_count() > 0);
        
        // Vocabulary should have grown
        assert!(bpe.vocab_size() >= initial_vocab_size);
        
        // Should have some merged segments
        assert!(!bpe.get_merged_segments().is_empty());
        
        // Test advanced metrics
        let stats = bpe.get_stats();
        assert!(stats.context_observations > 0);
        assert!(stats.average_confidence >= 0.0);
        assert!(stats.average_entropy >= 0.0);
        
        Ok(())
    }

    #[test]
    fn test_segment_stats() {
        let char_stats = SegmentStats::new_char('a');
        assert_eq!(char_stats.segment, "a");
        assert_eq!(char_stats.length, 1);
        assert!(char_stats.formed_from.is_none());
        assert_eq!(char_stats.confidence, 0.5); // Default confidence for characters
        
        let pair = SegmentPair::new("a".to_string(), "b".to_string());
        let merged_stats = SegmentStats::new_merged(pair, 5, 0);
        assert_eq!(merged_stats.segment, "ab");
        assert_eq!(merged_stats.frequency, 5);
        assert_eq!(merged_stats.length, 2);
        assert!(merged_stats.formed_from.is_some());
        assert_eq!(merged_stats.confidence, 0.0); // Will be calculated later
    }

    #[test]
    fn test_context_matrix() {
        let mut matrix = ContextMatrix::new();
        
        // Record some co-occurrences
        matrix.record_co_occurrence("a", "b");
        matrix.record_co_occurrence("b", "c");
        matrix.record_co_occurrence("a", "b"); // Duplicate
        
        assert_eq!(matrix.total_observations, 3);
        
        // Test co-occurrence strength
        let strength_ab = matrix.get_co_occurrence_strength("a", "b");
        let strength_bc = matrix.get_co_occurrence_strength("b", "c");
        let strength_ac = matrix.get_co_occurrence_strength("a", "c");
        
        assert!(strength_ab > strength_bc); // "a-b" occurs twice
        assert_eq!(strength_ac, 0.0); // "a-c" never co-occur
        
        // Test symmetry
        assert_eq!(strength_ab, matrix.get_co_occurrence_strength("b", "a"));
    }

    #[test]
    fn test_entropy_analyzer() {
        let analyzer = EntropyAnalyzer::new(3);
        let text = "aaabbbccc";
        
        let entropies = analyzer.calculate_position_entropies(text);
        assert_eq!(entropies.len(), text.len());
        
        // All entropies should be non-negative
        for entropy in &entropies {
            assert!(*entropy >= 0.0);
        }
        
        // Test specific entropy calculation
        let chars = vec!['a', 'a', 'b', 'b'];
        let entropy = analyzer.calculate_shannon_entropy(&chars);
        assert!(entropy > 0.0); // Should have some entropy with mixed characters
        
        // Test uniform distribution (maximum entropy)
        let uniform_chars = vec!['a', 'b', 'c', 'd'];
        let uniform_entropy = analyzer.calculate_shannon_entropy(&uniform_chars);
        assert!(uniform_entropy > entropy); // More diverse = higher entropy
    }

    #[test]
    fn test_confidence_scoring() {
        let mut stats = SegmentStats::new_merged(
            SegmentPair::new("t".to_string(), "h".to_string()),
            10,
            0
        );
        
        // Test confidence update
        stats.update_confidence(100, 0.5); // frequency=10, total=100, stability=0.5
        
        assert!(stats.confidence > 0.0);
        assert!(stats.confidence <= 1.0);
        
        // Higher frequency should give higher confidence
        let mut high_freq_stats = SegmentStats::new_merged(
            SegmentPair::new("e".to_string(), " ".to_string()),
            50,
            0
        );
        high_freq_stats.update_confidence(100, 0.5);
        
        assert!(high_freq_stats.confidence > stats.confidence);
    }

    #[test]
    fn test_advanced_segment_filtering() -> Result<()> {
        let config = BpeConfig {
            min_frequency: 1,
            max_vocab_size: 100,
            num_merges: 5,
            include_chars: true,
            min_entropy_threshold: 0.1,
            context_window_size: 3,
            min_confidence: 0.2,
            enable_advanced_heuristics: true,
        };
        
        let mut bpe = BpeSegmenter::new(config);
        let text = "the quick brown fox jumps over the lazy dog";
        
        bpe.initialize_from_text(text)?;
        bpe.train()?;
        
        // Test different segment filtering methods
        let by_frequency = bpe.get_segments_by_frequency();
        let by_confidence = bpe.get_segments_by_confidence();
        let high_confidence = bpe.get_high_confidence_segments();
        
        assert!(!by_frequency.is_empty());
        assert!(!by_confidence.is_empty());
        
        // High confidence segments should be a subset of all segments
        assert!(high_confidence.len() <= by_confidence.len());
        
        // Test co-occurrence strength
        let strength = bpe.get_co_occurrence_strength("t", "h");
        assert!(strength >= 0.0 && strength <= 1.0);
        
        Ok(())
    }

    #[test]
    fn test_disable_advanced_heuristics() -> Result<()> {
        let config = BpeConfig {
            enable_advanced_heuristics: false,
            min_frequency: 2,
            max_vocab_size: 100,
            num_merges: 10,
            include_chars: true,
            min_entropy_threshold: 0.5,
            context_window_size: 3,
            min_confidence: 0.3,
        };
        
        let mut segmenter = BpeSegmenter::new(config);
        segmenter.initialize_from_text("hello world hello world")?;
        segmenter.train()?;
        
        let stats = segmenter.get_stats();
        
        // Should still train successfully but without advanced heuristics
        assert!(stats.total_segments > 0);
        // Should have lower confidence scores without advanced analysis
        assert!(stats.average_confidence < 0.8);
        
        Ok(())
    }

    #[test]
    fn test_persistent_storage() -> Result<()> {
        use tempfile::NamedTempFile;
        
        // Create temporary files for testing
        let temp_file = NamedTempFile::new()?;
        let storage_path = temp_file.path().to_string_lossy().to_string();
        
        let storage_config = StorageConfig {
            segments_path: storage_path.clone(),
            archive_path: format!("{}.archive", storage_path),
            context_path: format!("{}.context", storage_path),
            save_interval_seconds: 0, // Immediate save
            compress_data: false,
            max_backups: 3,
        };
        
        // Create and train a segmenter
        let mut segmenter = BpeSegmenter::with_storage(
            BpeConfig::default(),
            storage_config.clone(),
            PruningConfig::default()
        );
        
        segmenter.initialize_from_text("hello world hello world")?;
        segmenter.train()?;
        
        let original_segments = segmenter.segments.len();
        assert!(original_segments > 0);
        
        // Save to storage
        segmenter.save_to_storage(&storage_path)?;
        
        // Load from storage
        let loaded_segmenter = BpeSegmenter::load_from_storage(&storage_path)?;
        
        // Verify data integrity
        assert_eq!(loaded_segmenter.segments.len(), original_segments);
        assert_eq!(loaded_segmenter.config.min_frequency, segmenter.config.min_frequency);
        
        Ok(())
    }

    #[test]
    fn test_segment_lifecycle_tracking() -> Result<()> {
        let mut segmenter = BpeSegmenter::default();
        segmenter.initialize_from_text("test segment lifecycle")?;
        segmenter.train()?;
        
        // Check that segments have lifecycle data
        let segment_keys: Vec<String> = segmenter.segments.keys().cloned().collect();
        
        for segment in &segment_keys {
            let stats = &segmenter.segments[segment];
            assert!(stats.created_at > 0);
            assert!(stats.last_accessed > 0);
            assert!(stats.last_modified > 0);
            assert_eq!(stats.access_count, 0); // Not accessed yet
            assert!(!stats.is_archived);
        }
        
        // Test marking as accessed
        for segment in &segment_keys {
            segmenter.mark_segment_accessed(segment);
        }
        
        // Verify access counts updated
        for (_, stats) in &segmenter.segments {
            assert_eq!(stats.access_count, 1);
        }
        
        Ok(())
    }

    #[test]
    fn test_segment_pruning() -> Result<()> {
        let pruning_config = PruningConfig {
            min_confidence_threshold: 0.8, // High threshold for aggressive pruning
            min_age_days: 0,               // Allow immediate pruning
            max_inactive_days: 0,          // No activity required
            min_access_count: 100,         // High access count required
            max_segments: 5,               // Low limit
            enable_auto_pruning: true,
        };
        
        let mut segmenter = BpeSegmenter::with_storage(
            BpeConfig::default(),
            StorageConfig::default(),
            pruning_config
        );
        
        segmenter.initialize_from_text("the quick brown fox jumps over the lazy dog")?;
        segmenter.train()?;
        
        let initial_count = segmenter.segments.len();
        assert!(initial_count > 5); // Should have more than max_segments
        
        // Perform pruning
        let pruned = segmenter.prune_segments()?;
        
        // Should have pruned some segments
        assert!(!pruned.is_empty());
        assert!(segmenter.segments.len() <= 5); // Should respect max_segments limit
        assert!(!segmenter.archived_segments.is_empty()); // Should have archived segments
        
        // Test restoration
        let first_pruned = &pruned[0];
        let restored = segmenter.restore_from_archive(first_pruned)?;
        assert!(restored);
        assert!(segmenter.segments.contains_key(first_pruned));
        
        Ok(())
    }

    #[test]
    fn test_pruning_candidates() -> Result<()> {
        let mut segmenter = BpeSegmenter::default();
        segmenter.initialize_from_text("test pruning candidates")?;
        segmenter.train()?;
        
        // Get pruning candidates
        let candidates = segmenter.get_pruning_candidates();
        
        // All candidates should meet pruning criteria
        for candidate in candidates {
            assert!(candidate.is_candidate_for_pruning(&segmenter.pruning_config));
        }
        
        Ok(())
    }

    #[test]
    fn test_auto_save_functionality() -> Result<()> {
        use tempfile::NamedTempFile;
        
        let temp_file = NamedTempFile::new()?;
        let storage_path = temp_file.path().to_string_lossy().to_string();
        
        let storage_config = StorageConfig {
            segments_path: storage_path.clone(),
            save_interval_seconds: 1, // 1 second interval
            ..StorageConfig::default()
        };
        
        let mut segmenter = BpeSegmenter::with_storage(
            BpeConfig::default(),
            storage_config,
            PruningConfig::default()
        );
        
        // Initially should need save
        assert!(!segmenter.should_auto_save()); // Just created
        
        // Simulate time passing
        segmenter.last_save = current_timestamp() - 2; // 2 seconds ago
        assert!(segmenter.should_auto_save());
        
        // Auto save should work
        segmenter.initialize_from_text("test auto save")?;
        segmenter.auto_save_if_needed()?;
        
        Ok(())
    }

    #[test]
    fn test_backup_creation() -> Result<()> {
        use tempfile::TempDir;
        use std::fs;
        
        let temp_dir = TempDir::new()?;
        let storage_path = temp_dir.path().join("segments.json");
        
        // Create initial file
        fs::write(&storage_path, "initial content")?;
        
        let storage_config = StorageConfig {
            segments_path: storage_path.to_string_lossy().to_string(),
            max_backups: 2,
            ..StorageConfig::default()
        };
        
        let mut segmenter = BpeSegmenter::with_storage(
            BpeConfig::default(),
            storage_config,
            PruningConfig::default()
        );
        
        segmenter.initialize_from_text("test backup")?;
        
        // Save should create backup
        segmenter.save_to_storage(&storage_path)?;
        
        // Check that backup was created
        let backup_files: Vec<_> = fs::read_dir(temp_dir.path())?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_name().to_string_lossy().contains(".backup.")
            })
            .collect();
        
        assert!(!backup_files.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_segment_archival() -> Result<()> {
        let mut segmenter = BpeSegmenter::default();
        segmenter.initialize_from_text("test archival system")?;
        segmenter.train()?;
        
        let segment_to_archive = segmenter.segments.keys().next().unwrap().clone();
        
        // Archive a segment manually
        if let Some(stats) = segmenter.segments.get_mut(&segment_to_archive) {
            stats.archive();
            assert!(stats.is_archived);
        }
        
        // Should not be candidate for pruning once archived
        let archived_stats = segmenter.segments.get(&segment_to_archive).unwrap();
        assert!(!archived_stats.is_candidate_for_pruning(&segmenter.pruning_config));
        
        Ok(())
    }

    #[test]
    fn test_segment_access_tracking() -> Result<()> {
        let mut segmenter = BpeSegmenter::default();
        segmenter.initialize_from_text("access tracking test")?;
        segmenter.train()?;
        
        let segment = segmenter.segments.keys().next().unwrap().clone();
        let initial_access_count = segmenter.segments[&segment].access_count;
        let initial_last_accessed = segmenter.segments[&segment].last_accessed;
        
        // Mark as accessed
        segmenter.mark_segment_accessed(&segment);
        
        let stats = &segmenter.segments[&segment];
        assert_eq!(stats.access_count, initial_access_count + 1);
        assert!(stats.last_accessed >= initial_last_accessed);
        
        Ok(())
    }
} 