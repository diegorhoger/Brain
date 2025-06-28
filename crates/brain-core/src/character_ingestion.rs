//! Character Ingestion Domain Logic
//! 
//! This module defines the core character ingestion abstractions and domain logic
//! without any I/O dependencies. Infrastructure implementations are provided
//! through trait implementations.

use brain_types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// Character vocabulary for mapping characters to indices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterVocab {
    char_to_idx: HashMap<char, usize>,
    idx_to_char: Vec<char>,
    vocab_size: usize,
}

impl CharacterVocab {
    /// Create a new vocabulary from text
    pub fn from_text(text: &str) -> Self {
        let mut chars: Vec<char> = text.chars().collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        chars.sort_unstable();
        
        // Add special tokens
        let mut vocab = vec!['\0', '?']; // PAD and UNK tokens
        vocab.extend(chars);
        
        let char_to_idx: HashMap<char, usize> = vocab
            .iter()
            .enumerate()
            .map(|(idx, &ch)| (ch, idx))
            .collect();
        
        Self {
            char_to_idx,
            idx_to_char: vocab.clone(),
            vocab_size: vocab.len(),
        }
    }
    
    /// Convert character to index
    pub fn char_to_index(&self, ch: char) -> usize {
        self.char_to_idx.get(&ch).copied().unwrap_or(1) // 1 is '?'
    }
    
    /// Convert index to character
    pub fn index_to_char(&self, idx: usize) -> char {
        self.idx_to_char.get(idx).copied().unwrap_or('?')
    }
    
    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab_size
    }
    
    /// Get vocabulary size (alias for compatibility)
    pub fn size(&self) -> usize {
        self.vocab_size
    }
    
    /// Encode text to indices
    pub fn encode(&self, text: &str) -> Vec<usize> {
        text.chars().map(|ch| self.char_to_index(ch)).collect()
    }
    
    /// Decode indices to text
    pub fn decode(&self, indices: &[usize]) -> String {
        indices
            .iter()
            .map(|&idx| self.index_to_char(idx))
            .collect()
    }
}

/// Model configuration for character prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub vocab_size: usize,
    pub embedding_dim: usize,
    pub hidden_dim: usize,
    pub learning_rate: f64,
    pub sequence_length: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            vocab_size: 0,
            embedding_dim: 128,
            hidden_dim: 256,
            learning_rate: 0.001,
            sequence_length: 32,
        }
    }
}

/// Prediction modes for character ingestion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionMode {
    CharacterOnly,
    SegmentAware,
    Hybrid,
}

/// Input types for prediction feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    Character,
    Segment,
    Hybrid,
}

/// Performance metrics for character prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_predictions: u64,
    pub correct_predictions: u64,
    pub average_confidence: f64,
    pub average_prediction_time_ms: f64,
    pub character_accuracy: f64,
    pub segment_accuracy: f64,
    pub hybrid_accuracy: f64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            total_predictions: 0,
            correct_predictions: 0,
            average_confidence: 0.0,
            average_prediction_time_ms: 0.0,
            character_accuracy: 0.0,
            segment_accuracy: 0.0,
            hybrid_accuracy: 0.0,
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_predictions == 0 {
            0.0
        } else {
            self.correct_predictions as f64 / self.total_predictions as f64
        }
    }
}

/// Feedback for prediction performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionFeedback {
    pub input: String,
    pub input_type: InputType,
    pub predicted: String,
    pub actual: String,
    pub confidence: f64,
    pub prediction_time_ms: u64,
    pub context_length: usize,
    pub segment_quality: Option<f64>,
    pub is_correct: bool,
}

/// Performance comparison between different prediction modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub character_only: PerformanceMetrics,
    pub segment_aware: PerformanceMetrics,
    pub hybrid: PerformanceMetrics,
}

/// Character predictor domain model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPredictorModel {
    pub config: ModelConfig,
    pub vocab: CharacterVocab,
    pub embedding: Vec<Vec<f64>>,
    pub hidden_weights: Vec<Vec<f64>>,
    pub hidden_bias: Vec<f64>,
    pub output_weights: Vec<Vec<f64>>,
    pub output_bias: Vec<f64>,
    pub prediction_mode: PredictionMode,
    pub performance_metrics: PerformanceMetrics,
}

/// Trait for character prediction services
#[async_trait]
pub trait CharacterPredictorService: Send + Sync {
    /// Predict the next character with confidence
    async fn predict_next_char(&mut self, input: &str) -> Result<(char, f64)>;
    
    /// Predict the next segment with confidence
    async fn predict_next_segment(&mut self, segments: &[String]) -> Result<(String, f64)>;
    
    /// Predict using hybrid approach
    async fn predict_hybrid(&mut self, char_context: &str, segment_context: &[String]) -> Result<(String, f64)>;
    
    /// Generate text from a prefix
    async fn generate(&self, prefix: &str, max_length: usize, temperature: f64) -> Result<String>;
    
    /// Train on a sequence
    async fn train_sequence(&mut self, sequence: &str, batch_size: usize, epochs: usize) -> Result<Vec<f64>>;
    
    /// Get current prediction mode
    fn get_prediction_mode(&self) -> PredictionMode;
    
    /// Set prediction mode
    fn set_prediction_mode(&mut self, mode: PredictionMode);
    
    /// Get performance metrics
    fn get_metrics(&self) -> &PerformanceMetrics;
}

/// Trait for segment providers
#[async_trait]
pub trait SegmentProvider: Send + Sync {
    /// Get segments from text
    async fn get_segments(&self, text: &str) -> Result<Vec<String>>;
    
    /// Get segment quality score
    async fn get_segment_quality(&self, segment: &str) -> Result<f64>;
}

/// Trait for performance tracking
#[async_trait]
pub trait PerformanceTracker: Send + Sync {
    /// Track a prediction result
    async fn track_prediction(&mut self, feedback: PredictionFeedback) -> Result<()>;
    
    /// Get current performance metrics
    fn get_metrics(&self) -> &PerformanceMetrics;
    
    /// Get performance comparison across modes
    fn get_performance_comparison(&self) -> PerformanceComparison;
    
    /// Export metrics as JSON
    async fn export_metrics(&self) -> Result<String>;
    
    /// Import metrics from JSON
    async fn import_metrics(&mut self, json_data: &str) -> Result<()>;
}

/// Trait for character ingestion repository
#[async_trait]
pub trait CharacterIngestionRepository: Send + Sync {
    /// Save a character predictor model
    async fn save_model(&self, model: &CharacterPredictorModel) -> Result<String>;
    
    /// Load a character predictor model
    async fn load_model(&self, model_id: &str) -> Result<CharacterPredictorModel>;
    
    /// List available models
    async fn list_models(&self) -> Result<Vec<String>>;
    
    /// Delete a model
    async fn delete_model(&self, model_id: &str) -> Result<()>;
}

/// Utility functions for character ingestion
pub mod utils {
    use super::*;

    /// Apply softmax to get probabilities
    pub fn softmax(logits: &[f64]) -> Vec<f64> {
        let max_val = logits.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let exp_vals: Vec<f64> = logits.iter().map(|x| (x - max_val).exp()).collect();
        let sum: f64 = exp_vals.iter().sum();
        exp_vals.iter().map(|x| x / sum).collect()
    }

    /// Sample from probability distribution
    pub fn sample_from_probs(probs: &[f64], temperature: f64) -> Result<usize> {
        use rand::prelude::*;
        
        if temperature <= 0.0 {
            return Ok(probs.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i)
                .unwrap_or(0));
        }
        
        let adjusted_probs: Vec<f64> = probs.iter()
            .map(|p| (p / temperature).exp())
            .collect();
        let sum: f64 = adjusted_probs.iter().sum();
        let normalized: Vec<f64> = adjusted_probs.iter().map(|p| p / sum).collect();
        
        let mut rng = thread_rng();
        let rand_val: f64 = rng.gen();
        let mut cumulative = 0.0;
        
        for (i, &prob) in normalized.iter().enumerate() {
            cumulative += prob;
            if rand_val <= cumulative {
                return Ok(i);
            }
        }
        
        Ok(normalized.len() - 1)
    }

    /// Calculate cosine similarity between two vectors
    pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() {
            return 0.0;
        }
        
        let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_vocab() {
        let vocab = CharacterVocab::from_text("hello world");
        assert!(vocab.vocab_size() > 0);
        assert_eq!(vocab.char_to_index('h'), vocab.char_to_index('h'));
        assert_eq!(vocab.index_to_char(vocab.char_to_index('o')), 'o');
    }

    #[test]
    fn test_model_config() {
        let config = ModelConfig::default();
        assert_eq!(config.embedding_dim, 128);
        assert_eq!(config.hidden_dim, 256);
        assert_eq!(config.learning_rate, 0.001);
    }

    #[test]
    fn test_performance_metrics() {
        let metrics = PerformanceMetrics::new();
        assert_eq!(metrics.total_predictions, 0);
        assert_eq!(metrics.accuracy(), 0.0);
    }

    #[test]
    fn test_softmax() {
        let logits = vec![1.0, 2.0, 3.0];
        let probs = utils::softmax(&logits);
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((utils::cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);
        
        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert!((utils::cosine_similarity(&c, &d) - 0.0).abs() < 1e-6);
    }
} 