//! Character Ingestion Engine
//! 
//! This module implements the foundational character-level predictor using a
//! simple neural network built from scratch. It forms the base layer of the Brain architecture.

use nalgebra::{DMatrix, DVector};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;

use crate::Result;
use crate::integration::{SegmentAwarePredictor, SegmentProvider, PredictionMode, PerformanceTracker, PerformanceMetrics, PredictionFeedback, InputType};

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

/// Model configuration
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

/// Simple character predictor model using feedforward network
#[derive(Debug)]
pub struct CharacterPredictor {
    config: ModelConfig,
    vocab: CharacterVocab,
    // Network weights
    embedding: DMatrix<f64>,
    hidden_weights: DMatrix<f64>,
    hidden_bias: DVector<f64>,
    output_weights: DMatrix<f64>,
    output_bias: DVector<f64>,
    // Integration features
    segment_provider: Option<Box<dyn SegmentProvider>>,
    prediction_mode: PredictionMode,
    performance_tracker: PerformanceMetrics,
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
            segment_provider: None,
            prediction_mode: PredictionMode::CharacterOnly,
            performance_tracker: PerformanceMetrics::new(),
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
    pub fn train_sequence(&mut self, sequence: &str, _batch_size: usize, epochs: usize) -> Result<Vec<f64>> {
        let mut losses = Vec::new();
        let encoded = self.vocab.encode(sequence);
        
        for epoch in 0..epochs {
            let mut epoch_loss = 0.0;
            let mut num_batches = 0;
            let start = Instant::now();

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
                
                // Cross-entropy loss
                let target_prob = probs[target_idx];
                let loss = -target_prob.ln();
                epoch_loss += loss;
                
                // Simple gradient descent (very basic implementation)
                self.backward_simple(input_idx, target_idx, &logits, &probs)?;
                
                num_batches += 1;
            }

            let avg_loss = epoch_loss / num_batches as f64;
            losses.push(avg_loss);

            log::info!(
                "Epoch {}/{}: avg_loss = {:.4}, time = {:.2}s",
                epoch + 1,
                epochs,
                avg_loss,
                start.elapsed().as_secs_f32(),
            );
        }

        Ok(losses)
    }

    /// Simple backward pass (gradient descent)
    fn backward_simple(&mut self, input_idx: usize, target_idx: usize, _logits: &DVector<f64>, probs: &DVector<f64>) -> Result<()> {
        let lr = self.config.learning_rate;
        
        // Output layer gradients
        let mut output_grad = probs.clone();
        output_grad[target_idx] -= 1.0; // Cross-entropy gradient
        
        // Get hidden layer activations
        let embedded = self.embedding.row(input_idx).transpose();
        let hidden_pre = &self.hidden_weights.transpose() * &embedded + &self.hidden_bias;
        let hidden = hidden_pre.map(|x| x.max(0.0)); // ReLU activation
        
        // Update output weights and bias
        for i in 0..self.config.hidden_dim {
            for j in 0..self.config.vocab_size {
                self.output_weights[(i, j)] -= lr * output_grad[j] * hidden[i];
            }
        }
        
        for j in 0..self.config.vocab_size {
            self.output_bias[j] -= lr * output_grad[j];
        }
        
        // Hidden layer gradients
        let hidden_grad = &self.output_weights * &output_grad;
        let hidden_grad_relu = DVector::from_iterator(
            hidden_grad.len(),
            hidden_grad.iter().zip(hidden_pre.iter()).map(|(grad, pre)| {
                if *pre > 0.0 { *grad } else { 0.0 } // ReLU derivative
            })
        );
        
        // Update hidden weights and bias
        for i in 0..self.config.embedding_dim {
            for j in 0..self.config.hidden_dim {
                self.hidden_weights[(i, j)] -= lr * hidden_grad_relu[j] * embedded[i];
            }
        }
        
        for j in 0..self.config.hidden_dim {
            self.hidden_bias[j] -= lr * hidden_grad_relu[j];
        }
        
        // Update embedding
        let embedding_grad = &self.hidden_weights * &hidden_grad_relu;
        for j in 0..self.config.embedding_dim {
            self.embedding[(input_idx, j)] -= lr * embedding_grad[j];
        }
        
        Ok(())
    }

    /// Generate text
    pub fn generate(
        &self,
        prefix: &str,
        max_length: usize,
        temperature: f64,
    ) -> Result<String> {
        let mut result = String::from(prefix);
        let mut rng = thread_rng();
        
        for _ in 0..max_length {
            let last_char = result.chars().last().unwrap_or(' ');
            let input_idx = self.vocab.char_to_index(last_char);
            
            let logits = self.forward(input_idx)?;
            
            // Apply temperature
            let scaled_logits = if temperature > 0.0 {
                logits / temperature
            } else {
                logits
            };
            
            let probs = self.softmax(&scaled_logits);
            let next_idx = self.sample_from_probs(&probs, &mut rng)?;
            let next_char = self.vocab.index_to_char(next_idx);
            
            result.push(next_char);
        }

        Ok(result)
    }

    /// Sample from probability distribution
    fn sample_from_probs(&self, probs: &DVector<f64>, rng: &mut ThreadRng) -> Result<usize> {
        let mut cumsum = 0.0;
        let mut cumprobs = Vec::with_capacity(probs.len());
        
        for &p in probs.iter() {
            cumsum += p;
            cumprobs.push(cumsum);
        }
        
        let r: f64 = rng.gen();
        let total = *cumprobs.last().unwrap();
        let normalized_r = r * total;
        
        for (i, &cumprob) in cumprobs.iter().enumerate() {
            if normalized_r <= cumprob {
                return Ok(i);
            }
        }
        
        Ok(cumprobs.len() - 1)
    }

    /// Save model
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let metadata = json!({
            "config": self.config,
            "vocab": self.vocab,
            "embedding": self.embedding.data.as_vec(),
            "hidden_weights": self.hidden_weights.data.as_vec(),
            "hidden_bias": self.hidden_bias.data.as_vec(),
            "output_weights": self.output_weights.data.as_vec(),
            "output_bias": self.output_bias.data.as_vec(),
        });
        
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &metadata)?;
        
        Ok(())
    }

    /// Load model
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata: serde_json::Value = serde_json::from_reader(reader)?;
        
        let config: ModelConfig = serde_json::from_value(metadata["config"].clone())?;
        let vocab: CharacterVocab = serde_json::from_value(metadata["vocab"].clone())?;
        
        let embedding_data: Vec<f64> = serde_json::from_value(metadata["embedding"].clone())?;
        let embedding = DMatrix::from_vec(config.vocab_size, config.embedding_dim, embedding_data);
        
        let hidden_weights_data: Vec<f64> = serde_json::from_value(metadata["hidden_weights"].clone())?;
        let hidden_weights = DMatrix::from_vec(config.embedding_dim, config.hidden_dim, hidden_weights_data);
        
        let hidden_bias_data: Vec<f64> = serde_json::from_value(metadata["hidden_bias"].clone())?;
        let hidden_bias = DVector::from_vec(hidden_bias_data);
        
        let output_weights_data: Vec<f64> = serde_json::from_value(metadata["output_weights"].clone())?;
        let output_weights = DMatrix::from_vec(config.hidden_dim, config.vocab_size, output_weights_data);
        
        let output_bias_data: Vec<f64> = serde_json::from_value(metadata["output_bias"].clone())?;
        let output_bias = DVector::from_vec(output_bias_data);
        
        Ok(Self {
            config,
            vocab,
            embedding,
            hidden_weights,
            hidden_bias,
            output_weights,
            output_bias,
            segment_provider: None,
            prediction_mode: PredictionMode::CharacterOnly,
            performance_tracker: PerformanceMetrics::new(),
        })
    }

    /// Predict character with confidence and context awareness
    pub fn predict_char_with_confidence(&mut self, input: &str) -> Result<(char, f64)> {
        if input.is_empty() {
            return Err(crate::error::BrainError::InvalidInput("Input cannot be empty".to_string()));
        }
        
        let start_time = std::time::Instant::now();
        
        // Use last character for prediction
        let last_char = input.chars().last().unwrap();
        let input_idx = self.vocab.char_to_index(last_char);
        
        // Forward pass
        let logits = self.forward(input_idx)?;
        let probs = self.softmax(&logits);
        
        // Find the most likely next character
        let (best_idx, best_prob) = probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        
        let predicted_char = self.vocab.index_to_char(best_idx);
        let confidence = *best_prob;
        
        let _prediction_time = start_time.elapsed().as_millis() as u64;
        
        // Create feedback for potential learning (would be used by integration system)
        // Note: Feedback tracking is handled by caller to avoid circular dependencies
        
        Ok((predicted_char, confidence))
    }
    
    /// Enhanced segment-aware prediction with quality assessment
    pub fn predict_segment_with_confidence(&mut self, segments: &[String]) -> Result<(String, f64)> {
        if segments.is_empty() {
            return Err(crate::error::BrainError::InvalidInput("Segments cannot be empty".to_string()));
        }
        
        let start_time = std::time::Instant::now();
        
        // For now, use the last segment's last character for prediction
        // In a real implementation, this would use segment embeddings
        let last_segment = segments.last().unwrap();
        if let Some(last_char) = last_segment.chars().last() {
            let input_idx = self.vocab.char_to_index(last_char);
            
            // Forward pass
            let logits = self.forward(input_idx)?;
            let probs = self.softmax(&logits);
            
            // For segment prediction, we need to map back to segments
            // This is a simplified approach - would need segment vocabulary in practice
            let (best_idx, best_prob) = probs
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap();
            
            let predicted_char = self.vocab.index_to_char(best_idx);
            
            // Try to find a segment that starts with this character
            let predicted_segment = if let Some(provider) = &self.segment_provider {
                let available_segments = provider.get_segments();
                available_segments
                    .into_iter()
                    .find(|seg| seg.starts_with(predicted_char))
                    .unwrap_or_else(|| predicted_char.to_string())
            } else {
                predicted_char.to_string()
            };
            
            let confidence = *best_prob;
            let _prediction_time = start_time.elapsed().as_millis() as u64;
            
            // Calculate segment quality score
            let _segment_quality = self.calculate_segment_quality(&predicted_segment, confidence);
            
            // Note: Feedback tracking is handled by caller to avoid circular dependencies
            
            Ok((predicted_segment, confidence))
        } else {
            Err(crate::error::BrainError::InvalidInput("Empty segment provided".to_string()))
        }
    }
    
    /// Advanced hybrid prediction combining character and segment context
    pub fn predict_hybrid_with_confidence(&mut self, char_context: &str, segment_context: &[String]) -> Result<(String, f64)> {
        let start_time = std::time::Instant::now();
        
        // Get character-level prediction
        let (char_pred, char_conf) = self.predict_char_with_confidence(char_context)?;
        
        // Get segment-level prediction
        let (seg_pred, seg_conf) = self.predict_segment_with_confidence(segment_context)?;
        
        // Combine predictions with weighted confidence
        // Favor higher confidence prediction
        let (final_prediction, final_confidence) = if seg_conf > char_conf + 0.1 {
            (seg_pred, seg_conf * 0.9) // Slight penalty for complexity
        } else if char_conf > seg_conf + 0.1 {
            (char_pred.to_string(), char_conf)
        } else {
            // If close, prefer segment prediction for its richer context
            (seg_pred, (seg_conf + char_conf) / 2.0)
        };
        
        let _prediction_time = start_time.elapsed().as_millis() as u64;
        
        // Calculate hybrid quality score
        let _hybrid_quality = self.calculate_hybrid_quality(char_conf, seg_conf, final_confidence);
        
        // Note: Feedback tracking is handled by caller to avoid circular dependencies
        
        Ok((final_prediction, final_confidence))
    }
    
    /// Calculate segment quality score based on multiple factors
    fn calculate_segment_quality(&self, segment: &str, confidence: f64) -> f64 {
        let mut quality_score = confidence * 0.4; // Base score from confidence
        
        // Length bonus for meaningful segments
        if segment.len() > 1 {
            quality_score += 0.2;
        }
        
        // Frequency bonus if we have segment provider
        if let Some(provider) = &self.segment_provider {
            if let Some(stats) = provider.get_segment_stats(segment) {
                // Higher frequency segments get bonus
                let freq_bonus = (stats.frequency as f64).ln().max(0.0) / 10.0;
                quality_score += freq_bonus.min(0.2);
                
                // Confidence bonus from segment stats
                quality_score += stats.confidence * 0.2;
            }
        }
        
        quality_score.min(1.0)
    }
    
    /// Calculate hybrid prediction quality
    fn calculate_hybrid_quality(&self, char_conf: f64, seg_conf: f64, final_conf: f64) -> f64 {
        // Base score from final confidence
        let mut quality = final_conf * 0.5;
        
        // Bonus for consistency between character and segment predictions
        let consistency = 1.0 - (char_conf - seg_conf).abs();
        quality += consistency * 0.3;
        
        // Bonus for high confidence in both approaches
        let avg_confidence = (char_conf + seg_conf) / 2.0;
        quality += avg_confidence * 0.2;
        
        quality.min(1.0)
    }
    
    /// Create enhanced feedback with context information
    pub fn create_feedback(
        &self, 
        input: &str, 
        input_type: InputType, 
        predicted: &str, 
        actual: &str, 
        confidence: f64, 
        prediction_time_ms: u64,
        context_length: usize,
        segment_quality: Option<f64>,
    ) -> PredictionFeedback {
        PredictionFeedback {
            input: input.to_string(),
            input_type,
            predicted: predicted.to_string(),
            actual: actual.to_string(),
            is_correct: if actual.is_empty() { false } else { predicted == actual },
            confidence,
            prediction_time_ms,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            context_length,
            segment_quality,
        }
    }
}

impl SegmentAwarePredictor for CharacterPredictor {
    fn predict_next_char(&mut self, input: &str) -> Result<(char, f64)> {
        self.predict_char_with_confidence(input)
    }
    
    fn predict_next_segment(&mut self, segments: &[String]) -> Result<(String, f64)> {
        self.predict_segment_with_confidence(segments)
    }
    
    fn predict_hybrid(&mut self, char_context: &str, segment_context: &[String]) -> Result<(String, f64)> {
        self.predict_hybrid_with_confidence(char_context, segment_context)
    }
    
    fn set_segmenter(&mut self, segmenter: Box<dyn SegmentProvider>) {
        self.segment_provider = Some(segmenter);
    }
    
    fn get_prediction_mode(&self) -> PredictionMode {
        self.prediction_mode.clone()
    }
    
    fn set_prediction_mode(&mut self, mode: PredictionMode) {
        self.prediction_mode = mode;
    }
}

impl PerformanceTracker for CharacterPredictor {
    fn track_prediction(&mut self, feedback: PredictionFeedback) -> Result<()> {
        self.performance_tracker.add_feedback(&feedback);
        Ok(())
    }
    
    fn get_metrics(&self) -> &PerformanceMetrics {
        &self.performance_tracker
    }
    
    fn get_performance_comparison(&self) -> crate::integration::PerformanceComparison {
        self.performance_tracker.compare_performance()
    }
    
    fn export_metrics(&self) -> Result<String> {
        serde_json::to_string(&self.performance_tracker)
            .map_err(|e| crate::BrainError::Serialization { source: Box::new(e) })
    }
    
    fn import_metrics(&mut self, json_data: &str) -> Result<()> {
        self.performance_tracker = serde_json::from_str(json_data)
            .map_err(|e| crate::BrainError::Serialization { source: Box::new(e) })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_vocab() {
        let vocab = CharacterVocab::from_text("Hello, World!");
        assert!(vocab.vocab_size() > 0);
        
        let encoded = vocab.encode("Hello");
        let decoded = vocab.decode(&encoded);
        assert_eq!(decoded, "Hello");
    }

    #[test]
    fn test_model_creation() -> Result<()> {
        let vocab = CharacterVocab::from_text("Hello, World!");
        let _model = CharacterPredictor::new(vocab, None)?;
        Ok(())
    }

    #[test]
    fn test_forward_pass() -> Result<()> {
        let vocab = CharacterVocab::from_text("Hello, World!");
        let model = CharacterPredictor::new(vocab, None)?;
        let output = model.forward(0)?;
        assert_eq!(output.len(), model.config.vocab_size);
        Ok(())
    }

    #[test]
    fn test_generation() -> Result<()> {
        let vocab = CharacterVocab::from_text("Hello, World!");
        let model = CharacterPredictor::new(vocab, None)?;
        let generated = model.generate("H", 5, 1.0)?;
        assert!(generated.len() > 1);
        Ok(())
    }
} 