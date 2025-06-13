//! Utility functions for the Brain architecture

use crate::Result;
// Removed candle dependency
use std::fs;
use std::path::Path;

// No longer needed since we're using standard Rust types

/// Read text file and return content as string
pub fn read_text_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

/// Create directory if it doesn't exist
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

/// Calculate accuracy given predictions and targets
pub fn calculate_accuracy(predictions: &[usize], targets: &[usize]) -> f32 {
    if predictions.is_empty() {
        return 0.0;
    }
    
    let correct = predictions
        .iter()
        .zip(targets.iter())
        .filter(|(pred, target)| pred == target)
        .count();
    
    correct as f32 / predictions.len() as f32
}

/// Calculate perplexity from cross-entropy loss
pub fn calculate_perplexity(cross_entropy_loss: f32) -> f32 {
    cross_entropy_loss.exp()
} 