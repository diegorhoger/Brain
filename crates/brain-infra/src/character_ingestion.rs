//! Character Ingestion Infrastructure Implementation
//! 
//! This module provides concrete implementations of the character ingestion
//! traits defined in brain-core, including file I/O, persistence, and
//! the actual neural network implementation.

use brain_types::*;
use brain_core::{
    CharacterVocab, ModelConfig, PredictionMode, InputType, PerformanceMetrics,
    PredictionFeedback, PerformanceComparison, CharacterPredictorModel,
    CharacterPredictorService, CharacterIngestionRepository, PerformanceTracker,
    CharacterSegmentProvider, character_utils
};
use nalgebra::{DMatrix, DVector};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;
use async_trait::async_trait;

/// File-based character ingestion repository
pub struct FileCharacterIngestionRepository {
    base_path: String,
}

impl FileCharacterIngestionRepository {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl CharacterIngestionRepository for FileCharacterIngestionRepository {
    async fn save_model(&self, model: &CharacterPredictorModel) -> Result<String> {
        let model_id = uuid::Uuid::new_v4().to_string();
        let path = format!("{}/{}.json", self.base_path, model_id);
        
        tokio::fs::create_dir_all(&self.base_path).await
            .map_err(|e| BrainError::Other(format!("Failed to create directory: {}", e)))?;
        
        let json = serde_json::to_string_pretty(model)
            .map_err(|e| BrainError::Other(format!("Failed to serialize model: {}", e)))?;
        
        tokio::fs::write(&path, json).await
            .map_err(|e| BrainError::Other(format!("Failed to write model file: {}", e)))?;
        
        Ok(model_id)
    }
    
    async fn load_model(&self, model_id: &str) -> Result<CharacterPredictorModel> {
        let path = format!("{}/{}.json", self.base_path, model_id);
        
        let contents = tokio::fs::read_to_string(&path).await
            .map_err(|e| BrainError::Other(format!("Failed to read model file: {}", e)))?;
        
        let model: CharacterPredictorModel = serde_json::from_str(&contents)
            .map_err(|e| BrainError::Other(format!("Failed to deserialize model: {}", e)))?;
        
        Ok(model)
    }
    
    async fn list_models(&self) -> Result<Vec<String>> {
        let mut models = Vec::new();
        
        let mut dir = tokio::fs::read_dir(&self.base_path).await
            .map_err(|e| BrainError::Other(format!("Failed to read directory: {}", e)))?;
        
        while let Some(entry) = dir.next_entry().await
            .map_err(|e| BrainError::Other(format!("Failed to read directory entry: {}", e)))? {
            
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let model_id = file_name.trim_end_matches(".json");
                    models.push(model_id.to_string());
                }
            }
        }
        
        Ok(models)
    }
    
    async fn delete_model(&self, model_id: &str) -> Result<()> {
        let path = format!("{}/{}.json", self.base_path, model_id);
        
        tokio::fs::remove_file(&path).await
            .map_err(|e| BrainError::Other(format!("Failed to delete model file: {}", e)))?;
        
        Ok(())
    }
}

/// Character predictor implementation using feedforward neural network
pub struct CharacterPredictor {
    config: ModelConfig,
    vocab: CharacterVocab,
    // Network weights
    embedding: DMatrix<f64>,
    hidden_weights: DMatrix<f64>,
    hidden_bias: DVector<f64>,
    output_weights: DMatrix<f64>,
    output_bias: DVector<f64>,
    // State
    prediction_mode: PredictionMode,
    performance_metrics: PerformanceMetrics,
}

impl CharacterPredictor {
    /// Create a new predictor
    pub fn new(vocab: CharacterVocab, config: Option<ModelConfig>) -> Result<Self> {
        let mut config = config.unwrap_or_default();
        config.vocab_size = vocab.vocab_size();

        let mut rng = thread_rng();
        
        // Initialize weights with Xavier initialization
        let embedding = DMatrix::from_fn(config.vocab_size, config.embedding_dim, |_, _| {
            rng.gen_range(-1.0..1.0) / (config.vocab_size as f64).sqrt()
        });
        
        let hidden_weights = DMatrix::from_fn(config.embedding_dim, config.hidden_dim, |_, _| {
            rng.gen_range(-1.0..1.0) / (config.embedding_dim as f64).sqrt()
        });
        
        let hidden_bias = DVector::zeros(config.hidden_dim);
        
        let output_weights = DMatrix::from_fn(config.hidden_dim, config.vocab_size, |_, _| {
            rng.gen_range(-1.0..1.0) / (config.hidden_dim as f64).sqrt()
        });
        
        let output_bias = DVector::zeros(config.vocab_size);

        Ok(Self {
            config,
            vocab,
            embedding,
            hidden_weights,
            hidden_bias,
            output_weights,
            output_bias,
            prediction_mode: PredictionMode::CharacterOnly,
            performance_metrics: PerformanceMetrics::new(),
        })
    }

    /// Forward pass
    fn forward(&self, input_idx: usize) -> Result<DVector<f64>> {
        // Get embedding
        let embedded = self.embedding.row(input_idx).transpose();
        
        // Hidden layer
        let hidden_pre = &self.hidden_weights.transpose() * &embedded + &self.hidden_bias;
        let hidden = hidden_pre.map(|x| x.max(0.0)); // ReLU activation
        
        // Output layer
        let output = &self.output_weights.transpose() * &hidden + &self.output_bias;
        
        Ok(output)
    }

    /// Apply softmax to get probabilities
    fn softmax(&self, logits: &DVector<f64>) -> DVector<f64> {
        let max_val = logits.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let exp_vals: DVector<f64> = logits.map(|x| (x - max_val).exp());
        let sum = exp_vals.sum();
        exp_vals / sum
    }

    /// Simple training on a sequence
    pub fn train_sequence_sync(&mut self, sequence: &str, _batch_size: usize, epochs: usize) -> Result<Vec<f64>> {
        let mut losses = Vec::new();
        let encoded = self.vocab.encode(sequence);
        
        for epoch in 0..epochs {
            let mut epoch_loss = 0.0;
            let mut num_batches = 0;

            // Simple batch processing
            for chunk in encoded.windows(2) {
                if chunk.len() < 2 {
                    continue;
                }
                
                let input_idx = chunk[0];
                let target_idx = chunk[1];
                
                // Forward pass
                let logits = self.forward(input_idx)?;
                let probs = self.softmax(&logits);
                
                // Calculate loss (cross-entropy)
                let loss = -probs[target_idx].ln();
                epoch_loss += loss;
                num_batches += 1;
                
                // Simple backward pass (gradient descent)
                self.backward_simple(input_idx, target_idx, &logits, &probs)?;
            }
            
            let avg_loss = if num_batches > 0 { epoch_loss / num_batches as f64 } else { 0.0 };
            losses.push(avg_loss);
        }
        
        Ok(losses)
    }

    /// Simple backward pass implementation
    fn backward_simple(&mut self, input_idx: usize, target_idx: usize, _logits: &DVector<f64>, probs: &DVector<f64>) -> Result<()> {
        let lr = self.config.learning_rate;
        
        // Output layer gradients
        let mut output_grad = probs.clone();
        output_grad[target_idx] -= 1.0; // Cross-entropy gradient
        
        // Update output bias
        self.output_bias -= lr * &output_grad;
        
        // Get current embeddings and hidden activations for this input
        let embedded = self.embedding.row(input_idx).transpose();
        let hidden_pre = &self.hidden_weights.transpose() * &embedded + &self.hidden_bias;
        let hidden = hidden_pre.map(|x| x.max(0.0)); // ReLU activation
        
        // Update output weights
        for i in 0..self.config.hidden_dim {
            for j in 0..self.config.vocab_size {
                self.output_weights[(i, j)] -= lr * output_grad[j] * hidden[i];
            }
        }
        
        // Hidden layer gradients (simplified)
        let hidden_grad = &self.output_weights * &output_grad;
        let hidden_grad_relu = hidden_grad.component_mul(&hidden_pre.map(|x| if x > 0.0 { 1.0 } else { 0.0 }));
        
        // Update hidden bias
        self.hidden_bias -= lr * &hidden_grad_relu;
        
        // Update hidden weights
        for i in 0..self.config.embedding_dim {
            for j in 0..self.config.hidden_dim {
                self.hidden_weights[(i, j)] -= lr * hidden_grad_relu[j] * embedded[i];
            }
        }
        
        // Update embeddings
        let embedding_grad = &self.hidden_weights * &hidden_grad_relu;
        for i in 0..self.config.embedding_dim {
            self.embedding[(input_idx, i)] -= lr * embedding_grad[i];
        }
        
        Ok(())
    }

    /// Generate text from a prefix
    pub fn generate_sync(&self, prefix: &str, max_length: usize, temperature: f64) -> Result<String> {
        let mut result = prefix.to_string();
        let mut current_context = prefix.to_string();
        
        for _ in 0..max_length {
            if current_context.is_empty() {
                break;
            }
            
            let last_char = current_context.chars().last().unwrap_or(' ');
            let input_idx = self.vocab.char_to_index(last_char);
            
            let logits = self.forward(input_idx)?;
            let probs = self.softmax(&logits);
            
            // Sample from probability distribution
            let next_idx = self.sample_from_probs(&probs, temperature)?;
            let next_char = self.vocab.index_to_char(next_idx);
            
            result.push(next_char);
            current_context.push(next_char);
            
            // Keep context window manageable
            if current_context.len() > self.config.sequence_length {
                current_context = current_context.chars().skip(1).collect();
            }
        }
        
        Ok(result)
    }

    /// Sample from probability distribution
    fn sample_from_probs(&self, probs: &DVector<f64>, temperature: f64) -> Result<usize> {
        let probs_slice: Vec<f64> = probs.iter().copied().collect();
        character_utils::sample_from_probs(&probs_slice, temperature)
    }

    /// Predict character with confidence
    pub fn predict_char_with_confidence_sync(&mut self, input: &str) -> Result<(char, f64)> {
        if input.is_empty() {
            return Ok((' ', 0.0));
        }
        
        let last_char = input.chars().last().unwrap_or(' ');
        let input_idx = self.vocab.char_to_index(last_char);
        
        let logits = self.forward(input_idx)?;
        let probs = self.softmax(&logits);
        
        // Get the most likely character
        let (best_idx, &confidence) = probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap_or((0, &0.0));
        
        let predicted_char = self.vocab.index_to_char(best_idx);
        
        Ok((predicted_char, confidence))
    }

    /// Convert to domain model
    pub fn to_model(&self) -> CharacterPredictorModel {
        CharacterPredictorModel {
            config: self.config.clone(),
            vocab: self.vocab.clone(),
            embedding: self.embedding.row_iter().map(|row| row.iter().copied().collect()).collect(),
            hidden_weights: self.hidden_weights.row_iter().map(|row| row.iter().copied().collect()).collect(),
            hidden_bias: self.hidden_bias.iter().copied().collect(),
            output_weights: self.output_weights.row_iter().map(|row| row.iter().copied().collect()).collect(),
            output_bias: self.output_bias.iter().copied().collect(),
            prediction_mode: self.prediction_mode,
            performance_metrics: self.performance_metrics.clone(),
        }
    }

    /// Load from domain model
    pub fn from_model(model: CharacterPredictorModel) -> Result<Self> {
        let embedding = DMatrix::from_row_slice(
            model.config.vocab_size,
            model.config.embedding_dim,
            &model.embedding.into_iter().flatten().collect::<Vec<_>>()
        );
        
        let hidden_weights = DMatrix::from_row_slice(
            model.config.embedding_dim,
            model.config.hidden_dim,
            &model.hidden_weights.into_iter().flatten().collect::<Vec<_>>()
        );
        
        let hidden_bias = DVector::from_vec(model.hidden_bias);
        
        let output_weights = DMatrix::from_row_slice(
            model.config.hidden_dim,
            model.config.vocab_size,
            &model.output_weights.into_iter().flatten().collect::<Vec<_>>()
        );
        
        let output_bias = DVector::from_vec(model.output_bias);

        Ok(Self {
            config: model.config,
            vocab: model.vocab,
            embedding,
            hidden_weights,
            hidden_bias,
            output_weights,
            output_bias,
            prediction_mode: model.prediction_mode,
            performance_metrics: model.performance_metrics,
        })
    }
}

#[async_trait]
impl CharacterPredictorService for CharacterPredictor {
    async fn predict_next_char(&mut self, input: &str) -> Result<(char, f64)> {
        self.predict_char_with_confidence_sync(input)
    }
    
    async fn predict_next_segment(&mut self, segments: &[String]) -> Result<(String, f64)> {
        // For now, use the last segment as context for character prediction
        let context = segments.last().map(|s| s.as_str()).unwrap_or("");
        let (char, confidence) = self.predict_char_with_confidence_sync(context)?;
        Ok((char.to_string(), confidence))
    }
    
    async fn predict_hybrid(&mut self, char_context: &str, _segment_context: &[String]) -> Result<(String, f64)> {
        // For now, just use character context
        let (char, confidence) = self.predict_char_with_confidence_sync(char_context)?;
        Ok((char.to_string(), confidence))
    }
    
    async fn generate(&self, prefix: &str, max_length: usize, temperature: f64) -> Result<String> {
        self.generate_sync(prefix, max_length, temperature)
    }
    
    async fn train_sequence(&mut self, sequence: &str, batch_size: usize, epochs: usize) -> Result<Vec<f64>> {
        self.train_sequence_sync(sequence, batch_size, epochs)
    }
    
    fn get_prediction_mode(&self) -> PredictionMode {
        self.prediction_mode
    }
    
    fn set_prediction_mode(&mut self, mode: PredictionMode) {
        self.prediction_mode = mode;
    }
    
    fn get_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }
}

/// Simple performance tracker implementation
pub struct SimplePerformanceTracker {
    metrics: PerformanceMetrics,
    comparison: PerformanceComparison,
}

impl SimplePerformanceTracker {
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::new(),
            comparison: PerformanceComparison {
                character_only: PerformanceMetrics::new(),
                segment_aware: PerformanceMetrics::new(),
                hybrid: PerformanceMetrics::new(),
            },
        }
    }
}

#[async_trait]
impl PerformanceTracker for SimplePerformanceTracker {
    async fn track_prediction(&mut self, feedback: PredictionFeedback) -> Result<()> {
        self.metrics.total_predictions += 1;
        if feedback.is_correct {
            self.metrics.correct_predictions += 1;
        }
        
        // Update averages
        let total = self.metrics.total_predictions as f64;
        self.metrics.average_confidence = 
            (self.metrics.average_confidence * (total - 1.0) + feedback.confidence) / total;
        self.metrics.average_prediction_time_ms = 
            (self.metrics.average_prediction_time_ms * (total - 1.0) + feedback.prediction_time_ms as f64) / total;
        
        // Update mode-specific metrics
        match feedback.input_type {
            InputType::Character => {
                self.comparison.character_only.total_predictions += 1;
                if feedback.is_correct {
                    self.comparison.character_only.correct_predictions += 1;
                }
            }
            InputType::Segment => {
                self.comparison.segment_aware.total_predictions += 1;
                if feedback.is_correct {
                    self.comparison.segment_aware.correct_predictions += 1;
                }
            }
            InputType::Hybrid => {
                self.comparison.hybrid.total_predictions += 1;
                if feedback.is_correct {
                    self.comparison.hybrid.correct_predictions += 1;
                }
            }
        }
        
        Ok(())
    }
    
    fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }
    
    fn get_performance_comparison(&self) -> PerformanceComparison {
        self.comparison.clone()
    }
    
    async fn export_metrics(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(&self.metrics)
            .map_err(|e| BrainError::Other(format!("Failed to serialize metrics: {}", e)))?;
        Ok(json)
    }
    
    async fn import_metrics(&mut self, json_data: &str) -> Result<()> {
        let metrics: PerformanceMetrics = serde_json::from_str(json_data)
            .map_err(|e| BrainError::Other(format!("Failed to deserialize metrics: {}", e)))?;
        self.metrics = metrics;
        Ok(())
    }
}

/// Simple segment provider implementation
pub struct SimpleSegmentProvider {
    segments: Vec<String>,
}

impl SimpleSegmentProvider {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }
    
    pub fn from_text(text: &str) -> Self {
        // Simple word-based segmentation
        let segments: Vec<String> = text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        Self { segments }
    }
}

#[async_trait]
impl CharacterSegmentProvider for SimpleSegmentProvider {
    async fn get_segments(&self, text: &str) -> Result<Vec<String>> {
        // Simple word-based segmentation
        Ok(text.split_whitespace().map(|s| s.to_string()).collect())
    }
    
    async fn get_segment_quality(&self, segment: &str) -> Result<f64> {
        // Simple quality based on length and character variety
        let length_score = (segment.len() as f64 / 10.0).min(1.0);
        let char_variety = segment.chars().collect::<std::collections::HashSet<_>>().len() as f64;
        let variety_score = (char_variety / segment.len() as f64).min(1.0);
        
        Ok((length_score + variety_score) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_predictor_creation() -> Result<()> {
        let vocab = CharacterVocab::from_text("hello world");
        let predictor = CharacterPredictor::new(vocab, None)?;
        assert_eq!(predictor.get_prediction_mode(), PredictionMode::CharacterOnly);
        Ok(())
    }

    #[test]
    fn test_forward_pass() -> Result<()> {
        let vocab = CharacterVocab::from_text("hello");
        let predictor = CharacterPredictor::new(vocab, None)?;
        let output = predictor.forward(0)?;
        assert_eq!(output.len(), predictor.config.vocab_size);
        Ok(())
    }

    #[test]
    fn test_generation() -> Result<()> {
        let vocab = CharacterVocab::from_text("hello world");
        let predictor = CharacterPredictor::new(vocab, None)?;
        let result = predictor.generate_sync("h", 5, 1.0)?;
        assert!(result.starts_with("h"));
        assert!(result.len() > 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_performance_tracker() -> Result<()> {
        let mut tracker = SimplePerformanceTracker::new();
        
        let feedback = PredictionFeedback {
            input: "test".to_string(),
            input_type: InputType::Character,
            predicted: "t".to_string(),
            actual: "t".to_string(),
            confidence: 0.8,
            prediction_time_ms: 10,
            context_length: 4,
            segment_quality: None,
            is_correct: true,
        };
        
        tracker.track_prediction(feedback).await?;
        
        let metrics = tracker.get_metrics();
        assert_eq!(metrics.total_predictions, 1);
        assert_eq!(metrics.correct_predictions, 1);
        assert_eq!(metrics.accuracy(), 1.0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_segment_provider() -> Result<()> {
        let provider = SimpleSegmentProvider::from_text("hello world test");
        let segments = provider.get_segments("hello world").await?;
        assert_eq!(segments, vec!["hello", "world"]);
        
        let quality = provider.get_segment_quality("hello").await?;
        assert!(quality > 0.0 && quality <= 1.0);
        
        Ok(())
    }
} 