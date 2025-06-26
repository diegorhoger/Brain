//! Segmentation Infrastructure Implementations
//! 
//! This module provides in-memory implementations of segmentation-related repositories
//! for development and testing purposes.

use brain_core::*;
use brain_types::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// In-memory implementation of SegmentRepository
pub struct InMemorySegmentRepository {
    segments: Arc<RwLock<HashMap<String, SegmentStats>>>,
}

impl InMemorySegmentRepository {
    pub fn new() -> Self {
        Self {
            segments: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemorySegmentRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SegmentRepository for InMemorySegmentRepository {
    async fn store_segment(&mut self, stats: SegmentStats) -> Result<()> {
        let mut segments = self.segments.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        segments.insert(stats.segment.clone(), stats);
        Ok(())
    }

    async fn get_segment(&self, segment: &str) -> Result<Option<SegmentStats>> {
        let segments = self.segments.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(segments.get(segment).cloned())
    }

    async fn update_segment(&mut self, stats: &SegmentStats) -> Result<()> {
        let mut segments = self.segments.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        segments.insert(stats.segment.clone(), stats.clone());
        Ok(())
    }

    async fn remove_segment(&mut self, segment: &str) -> Result<()> {
        let mut segments = self.segments.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        segments.remove(segment);
        Ok(())
    }

    async fn get_all_segments(&self) -> Result<Vec<SegmentStats>> {
        let segments = self.segments.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(segments.values().cloned().collect())
    }

    async fn get_segments_by_frequency(&self) -> Result<Vec<SegmentStats>> {
        let segments = self.segments.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let mut results: Vec<SegmentStats> = segments.values().cloned().collect();
        results.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        Ok(results)
    }

    async fn get_segments_by_confidence(&self) -> Result<Vec<SegmentStats>> {
        let segments = self.segments.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let mut results: Vec<SegmentStats> = segments.values().cloned().collect();
        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        Ok(results)
    }

    async fn get_high_confidence_segments(&self) -> Result<Vec<SegmentStats>> {
        let segments = self.segments.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let results: Vec<SegmentStats> = segments
            .values()
            .filter(|stats| stats.confidence > 0.7)
            .cloned()
            .collect();
        Ok(results)
    }

    async fn get_pruning_candidates(&self, config: &PruningConfig) -> Result<Vec<SegmentStats>> {
        let segments = self.segments.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let results: Vec<SegmentStats> = segments
            .values()
            .filter(|stats| stats.is_candidate_for_pruning(config))
            .cloned()
            .collect();
        Ok(results)
    }

    async fn archive_segment(&mut self, segment: &str) -> Result<bool> {
        let mut segments = self.segments.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(stats) = segments.get_mut(segment) {
            stats.archive();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn restore_from_archive(&mut self, segment: &str) -> Result<bool> {
        let mut segments = self.segments.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(stats) = segments.get_mut(segment) {
            stats.is_archived = false;
            stats.last_modified = current_timestamp();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl InMemorySegmentRepository {
    /// Helper method to update segment frequency
    pub async fn update_frequency(&mut self, segment: &str, new_frequency: usize) -> Result<()> {
        let mut segments = self.segments.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(stats) = segments.get_mut(segment) {
            stats.frequency = new_frequency;
            stats.last_modified = current_timestamp();
        }
        
        Ok(())
    }

    /// Helper method to mark segment as accessed
    pub async fn mark_accessed(&mut self, segment: &str) -> Result<()> {
        let mut segments = self.segments.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(stats) = segments.get_mut(segment) {
            stats.mark_accessed();
        }
        
        Ok(())
    }
}

// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// Helper function for calculating cosine similarity between embeddings
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        (dot_product / (norm_a * norm_b)) as f64
    }
} 