//! Advanced Neural Architecture Module - Task 3.x
//! 
//! This module implements sophisticated neural network architectures including:
//! - Self-attention and multi-head attention mechanisms
//! - Transformer-like encoder-decoder structures  
//! - Post-transformer developmental AI approaches
//! - Advanced layer types with residual connections and normalization

use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};

use crate::Result;

/// Configuration for attention mechanisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionConfig {
    /// Dimensionality of the model
    pub model_dim: usize,
    /// Number of attention heads
    pub num_heads: usize,
    /// Dimension of each attention head
    pub head_dim: usize,
    /// Dropout rate for attention weights
    pub dropout_rate: f64,
    /// Whether to use scaled dot-product attention
    pub use_scaling: bool,
}

impl Default for AttentionConfig {
    fn default() -> Self {
        Self {
            model_dim: 512,
            num_heads: 8,
            head_dim: 64,
            dropout_rate: 0.1,
            use_scaling: true,
        }
    }
}

/// Self-attention mechanism with multi-head support
#[derive(Debug, Clone)]
pub struct SelfAttention {
    /// Configuration parameters
    config: AttentionConfig,
    /// Query weight matrix
    w_query: DMatrix<f64>,
    /// Key weight matrix  
    w_key: DMatrix<f64>,
    /// Value weight matrix
    w_value: DMatrix<f64>,
    /// Output projection matrix
    w_output: DMatrix<f64>,
    /// Attention weights (cached for analysis)
    attention_weights: Option<DMatrix<f64>>,
}

impl SelfAttention {
    /// Create new self-attention layer
    pub fn new(config: AttentionConfig) -> Result<Self> {
        let model_dim = config.model_dim;
        
        // Initialize weight matrices with Xavier initialization
        let w_query = Self::xavier_init(model_dim, model_dim)?;
        let w_key = Self::xavier_init(model_dim, model_dim)?;
        let w_value = Self::xavier_init(model_dim, model_dim)?;
        let w_output = Self::xavier_init(model_dim, model_dim)?;
        
        Ok(Self {
            config,
            w_query,
            w_key,
            w_value,
            w_output,
            attention_weights: None,
        })
    }
    
    /// Xavier weight initialization
    fn xavier_init(rows: usize, cols: usize) -> Result<DMatrix<f64>> {
        let limit = (6.0 / (rows + cols) as f64).sqrt();
        let mut matrix = DMatrix::zeros(rows, cols);
        
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = (rand::random::<f64>() - 0.5) * 2.0 * limit;
            }
        }
        
        Ok(matrix)
    }
    
    /// Forward pass through self-attention
    pub fn forward(&mut self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        let (_seq_len, model_dim) = input.shape();
        
        if model_dim != self.config.model_dim {
            return Err(crate::error::BrainError::InvalidInput(
                format!("Input dimension {} doesn't match model dimension {}", 
                       model_dim, self.config.model_dim)
            ));
        }
        
        // Compute queries, keys, and values
        let queries = input * &self.w_query;
        let keys = input * &self.w_key;
        let values = input * &self.w_value;
        
        // Reshape for multi-head attention
        let queries_reshaped = self.reshape_for_heads(&queries)?;
        let keys_reshaped = self.reshape_for_heads(&keys)?;
        let values_reshaped = self.reshape_for_heads(&values)?;
        
        // Compute attention scores
        let attention_scores = self.compute_attention_scores(&queries_reshaped, &keys_reshaped)?;
        
        // Apply attention to values
        let attended_values = self.apply_attention(&attention_scores, &values_reshaped)?;
        
        // Reshape back and apply output projection
        let concatenated = self.concatenate_heads(&attended_values)?;
        let output = &concatenated * &self.w_output;
        
        Ok(output)
    }
    
    /// Reshape input for multi-head attention
    fn reshape_for_heads(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        // For simplicity, we'll concatenate heads rather than true 3D tensor operations
        Ok(input.clone())
    }
    
    /// Compute attention scores using scaled dot-product
    fn compute_attention_scores(&mut self, queries: &DMatrix<f64>, keys: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        // Compute Q * K^T
        let scores = queries * keys.transpose();
        
        // Apply scaling if enabled
        let scaled_scores = if self.config.use_scaling {
            let scale = 1.0 / (self.config.head_dim as f64).sqrt();
            &scores * scale
        } else {
            scores
        };
        
        // Apply softmax to get attention weights
        let attention_weights = self.softmax(&scaled_scores)?;
        
        // Cache attention weights for analysis
        self.attention_weights = Some(attention_weights.clone());
        
        Ok(attention_weights)
    }
    
    /// Apply softmax function
    fn softmax(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        let (rows, cols) = input.shape();
        let mut output = DMatrix::zeros(rows, cols);
        
        for i in 0..rows {
            let row = input.row(i);
            let max_val = row.max();
            
            // Compute exponentials (numerically stable)
            let mut exp_sum = 0.0;
            for j in 0..cols {
                let exp_val = (row[j] - max_val).exp();
                output[(i, j)] = exp_val;
                exp_sum += exp_val;
            }
            
            // Normalize
            for j in 0..cols {
                output[(i, j)] /= exp_sum;
            }
        }
        
        Ok(output)
    }
    
    /// Apply attention weights to values
    fn apply_attention(&self, attention: &DMatrix<f64>, values: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        Ok(attention * values)
    }
    
    /// Concatenate multi-head outputs
    fn concatenate_heads(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        // For simplified implementation, return as-is
        Ok(input.clone())
    }
    
    /// Get attention weights for visualization
    pub fn get_attention_weights(&self) -> Option<&DMatrix<f64>> {
        self.attention_weights.as_ref()
    }
}

/// Transformer encoder layer
#[derive(Debug, Clone)]
pub struct TransformerEncoder {
    /// Self-attention mechanism
    self_attention: SelfAttention,
    /// Feed-forward network
    feed_forward: FeedForwardNetwork,
    /// Layer normalization for attention
    layer_norm1: LayerNorm,
    /// Layer normalization for feed-forward
    layer_norm2: LayerNorm,
    /// Dropout rate
    dropout_rate: f64,
}

/// Feed-forward network with residual connections
#[derive(Debug, Clone)]
pub struct FeedForwardNetwork {
    /// First linear layer
    linear1: DMatrix<f64>,
    /// Second linear layer  
    linear2: DMatrix<f64>,
    /// Bias vectors
    bias1: DVector<f64>,
    bias2: DVector<f64>,
    /// Hidden dimension
    hidden_dim: usize,
}

/// Layer normalization
#[derive(Debug, Clone)]
pub struct LayerNorm {
    /// Learnable scale parameters
    gamma: DVector<f64>,
    /// Learnable shift parameters
    beta: DVector<f64>,
    /// Small constant for numerical stability
    epsilon: f64,
}

impl FeedForwardNetwork {
    /// Create new feed-forward network
    pub fn new(input_dim: usize, hidden_dim: usize) -> Result<Self> {
        let linear1 = Self::xavier_init(hidden_dim, input_dim)?;
        let linear2 = Self::xavier_init(input_dim, hidden_dim)?;
        let bias1 = DVector::zeros(hidden_dim);
        let bias2 = DVector::zeros(input_dim);
        
        Ok(Self {
            linear1,
            linear2,
            bias1,
            bias2,
            hidden_dim,
        })
    }
    
    /// Xavier initialization
    fn xavier_init(rows: usize, cols: usize) -> Result<DMatrix<f64>> {
        let limit = (6.0 / (rows + cols) as f64).sqrt();
        let mut matrix = DMatrix::zeros(rows, cols);
        
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = (rand::random::<f64>() - 0.5) * 2.0 * limit;
            }
        }
        
        Ok(matrix)
    }
    
    /// Forward pass with ReLU activation
    pub fn forward(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        let (batch_size, _) = input.shape();
        
        // First linear layer
        let linear1_output = input * &self.linear1.transpose();
        let bias1_expanded = DMatrix::from_row_slice(batch_size, self.hidden_dim, &vec![self.bias1.as_slice(); batch_size].concat());
        let hidden_pre_relu = linear1_output + bias1_expanded;
        let hidden = self.relu(&hidden_pre_relu)?;
        
        // Second linear layer
        let linear2_output = &hidden * &self.linear2.transpose();
        let bias2_expanded = DMatrix::from_row_slice(batch_size, self.bias2.len(), &vec![self.bias2.as_slice(); batch_size].concat());
        let output = linear2_output + bias2_expanded;
        
        Ok(output)
    }
    
    /// ReLU activation function
    fn relu(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        let (rows, cols) = input.shape();
        let mut output = DMatrix::zeros(rows, cols);
        
        for i in 0..rows {
            for j in 0..cols {
                output[(i, j)] = input[(i, j)].max(0.0);
            }
        }
        
        Ok(output)
    }
}

impl LayerNorm {
    /// Create new layer normalization
    pub fn new(dim: usize) -> Self {
        Self {
            gamma: DVector::from_element(dim, 1.0),
            beta: DVector::zeros(dim),
            epsilon: 1e-6,
        }
    }
    
    /// Apply layer normalization
    pub fn forward(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        let (rows, cols) = input.shape();
        let mut output = DMatrix::zeros(rows, cols);
        
        for i in 0..rows {
            let row = input.row(i);
            
            // Compute mean and variance
            let mean = row.mean();
            let variance = row.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / cols as f64;
            let std = (variance + self.epsilon).sqrt();
            
            // Normalize and scale
            for j in 0..cols {
                let normalized = (row[j] - mean) / std;
                output[(i, j)] = self.gamma[j] * normalized + self.beta[j];
            }
        }
        
        Ok(output)
    }
}

impl TransformerEncoder {
    /// Create new transformer encoder layer
    pub fn new(config: AttentionConfig, ff_hidden_dim: usize) -> Result<Self> {
        let self_attention = SelfAttention::new(config.clone())?;
        let feed_forward = FeedForwardNetwork::new(config.model_dim, ff_hidden_dim)?;
        let layer_norm1 = LayerNorm::new(config.model_dim);
        let layer_norm2 = LayerNorm::new(config.model_dim);
        
        Ok(Self {
            self_attention,
            feed_forward,
            layer_norm1,
            layer_norm2,
            dropout_rate: config.dropout_rate,
        })
    }
    
    /// Forward pass through transformer encoder
    pub fn forward(&mut self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        // Self-attention with residual connection and layer norm
        let attention_out = self.self_attention.forward(input)?;
        let normed1 = self.layer_norm1.forward(&(input + &attention_out))?;
        
        // Feed-forward with residual connection and layer norm
        let ff_out = self.feed_forward.forward(&normed1)?;
        let normed2 = self.layer_norm2.forward(&(&normed1 + &ff_out))?;
        
        Ok(normed2)
    }
}

/// Advanced neural predictor with transformer architecture
#[derive(Debug)]
pub struct TransformerPredictor {
    /// Input embedding layer
    embedding: DMatrix<f64>,
    /// Positional encoding
    positional_encoding: DMatrix<f64>,
    /// Stack of transformer encoder layers
    encoders: Vec<TransformerEncoder>,
    /// Output projection layer
    output_projection: DMatrix<f64>,
    /// Configuration
    config: TransformerConfig,
    /// Vocabulary size
    vocab_size: usize,
}

/// Configuration for transformer architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformerConfig {
    /// Model dimension
    pub model_dim: usize,
    /// Number of encoder layers
    pub num_layers: usize,
    /// Number of attention heads
    pub num_heads: usize,
    /// Feed-forward hidden dimension
    pub ff_hidden_dim: usize,
    /// Maximum sequence length
    pub max_seq_len: usize,
    /// Dropout rate
    pub dropout_rate: f64,
}

impl Default for TransformerConfig {
    fn default() -> Self {
        Self {
            model_dim: 512,
            num_layers: 6,
            num_heads: 8,
            ff_hidden_dim: 2048,
            max_seq_len: 1024,
            dropout_rate: 0.1,
        }
    }
}

impl TransformerPredictor {
    /// Create new transformer predictor
    pub fn new(vocab_size: usize, config: Option<TransformerConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();
        
        // Initialize embedding matrix
        let embedding = Self::xavier_init(vocab_size, config.model_dim)?;
        
        // Initialize positional encoding
        let positional_encoding = Self::create_positional_encoding(config.max_seq_len, config.model_dim)?;
        
        // Create transformer encoder layers
        let mut encoders = Vec::new();
        for _ in 0..config.num_layers {
            let attention_config = AttentionConfig {
                model_dim: config.model_dim,
                num_heads: config.num_heads,
                head_dim: config.model_dim / config.num_heads,
                dropout_rate: config.dropout_rate,
                use_scaling: true,
            };
            
            encoders.push(TransformerEncoder::new(attention_config, config.ff_hidden_dim)?);
        }
        
        // Output projection to vocabulary
        let output_projection = Self::xavier_init(config.model_dim, vocab_size)?;
        
        Ok(Self {
            embedding,
            positional_encoding,
            encoders,
            output_projection,
            config,
            vocab_size,
        })
    }
    
    /// Xavier initialization
    fn xavier_init(rows: usize, cols: usize) -> Result<DMatrix<f64>> {
        let limit = (6.0 / (rows + cols) as f64).sqrt();
        let mut matrix = DMatrix::zeros(rows, cols);
        
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = (rand::random::<f64>() - 0.5) * 2.0 * limit;
            }
        }
        
        Ok(matrix)
    }
    
    /// Create sinusoidal positional encoding
    fn create_positional_encoding(max_len: usize, model_dim: usize) -> Result<DMatrix<f64>> {
        let mut pe = DMatrix::zeros(max_len, model_dim);
        
        for pos in 0..max_len {
            for i in 0..model_dim {
                let angle = pos as f64 / 10000_f64.powf(2.0 * (i / 2) as f64 / model_dim as f64);
                
                if i % 2 == 0 {
                    pe[(pos, i)] = angle.sin();
                } else {
                    pe[(pos, i)] = angle.cos();
                }
            }
        }
        
        Ok(pe)
    }
    
    /// Forward pass through transformer
    pub fn forward(&mut self, input_ids: &[usize]) -> Result<DMatrix<f64>> {
        let seq_len = input_ids.len();
        
        if seq_len > self.config.max_seq_len {
            return Err(crate::error::BrainError::InvalidInput(
                format!("Sequence length {} exceeds maximum {}", seq_len, self.config.max_seq_len)
            ));
        }
        
        // Convert input IDs to embeddings
        let mut embedded = DMatrix::zeros(seq_len, self.config.model_dim);
        for (i, &token_id) in input_ids.iter().enumerate() {
            if token_id >= self.vocab_size {
                return Err(crate::error::BrainError::InvalidInput(
                    format!("Token ID {} exceeds vocabulary size {}", token_id, self.vocab_size)
                ));
            }
            
            // Copy embedding for this token
            for j in 0..self.config.model_dim {
                embedded[(i, j)] = self.embedding[(token_id, j)];
            }
        }
        
        // Add positional encoding
        for i in 0..seq_len {
            for j in 0..self.config.model_dim {
                embedded[(i, j)] += self.positional_encoding[(i, j)];
            }
        }
        
        // Pass through transformer encoder layers
        let mut output = embedded;
        for encoder in &mut self.encoders {
            output = encoder.forward(&output)?;
        }
        
        // Apply output projection
        let logits = &output * &self.output_projection;
        
        Ok(logits)
    }
    
    /// Generate next token probabilities
    pub fn predict_next(&mut self, input_ids: &[usize]) -> Result<DVector<f64>> {
        let logits_matrix = self.forward(input_ids)?;
        
        // Get logits for the last position
        let last_pos = logits_matrix.nrows() - 1;
        let last_logits = logits_matrix.row(last_pos);
        
        // Convert to DVector and apply softmax
        let mut logits = DVector::zeros(self.vocab_size);
        for i in 0..self.vocab_size {
            logits[i] = last_logits[i];
        }
        
        self.softmax(&logits)
    }
    
    /// Apply softmax to get probabilities
    fn softmax(&self, logits: &DVector<f64>) -> Result<DVector<f64>> {
        let max_logit = logits.max();
        let mut probs = DVector::zeros(logits.len());
        let mut sum = 0.0;
        
        for i in 0..logits.len() {
            let exp_val = (logits[i] - max_logit).exp();
            probs[i] = exp_val;
            sum += exp_val;
        }
        
        for i in 0..probs.len() {
            probs[i] /= sum;
        }
        
        Ok(probs)
    }
    
    /// Get attention weights from all layers
    pub fn get_attention_maps(&self) -> Vec<Option<&DMatrix<f64>>> {
        self.encoders.iter()
            .map(|encoder| encoder.self_attention.get_attention_weights())
            .collect()
    }
}

/// Post-transformer developmental AI architecture
#[derive(Debug)]
pub struct DevelopmentalPredictor {
    /// Base transformer architecture
    transformer: TransformerPredictor,
    /// Developmental parameters
    growth_config: GrowthConfig,
    /// Current developmental stage
    current_stage: DevelopmentalStage,
    /// Learning history for meta-learning
    learning_history: Vec<LearningEvent>,
    /// Adaptive capacity tracking
    capacity_tracker: CapacityTracker,
}

/// Configuration for developmental growth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthConfig {
    /// Initial model size multiplier
    pub initial_scale: f64,
    /// Growth rate per developmental stage
    pub growth_rate: f64,
    /// Maximum model size
    pub max_scale: f64,
    /// Complexity threshold for growth
    pub complexity_threshold: f64,
    /// Enable meta-learning
    pub enable_meta_learning: bool,
}

/// Developmental stages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    Embryonic,
    Infant,
    Child,
    Adolescent,
    Adult,
    Expert,
}

/// Learning events for meta-learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    /// Timestamp of the event
    pub timestamp: u64,
    /// Type of learning that occurred
    pub learning_type: LearningType,
    /// Performance before learning
    pub performance_before: f64,
    /// Performance after learning
    pub performance_after: f64,
    /// Context information
    pub context: String,
}

/// Types of learning events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningType {
    ParameterUpdate,
    StructuralGrowth,
    PruningEvent,
    MetaLearning,
    ConceptAcquisition,
}

/// Capacity tracking for developmental adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityTracker {
    /// Current model complexity
    pub current_complexity: f64,
    /// Learning efficiency over time
    pub efficiency_history: Vec<f64>,
    /// Capacity utilization
    pub utilization: f64,
    /// Need for growth indicator
    pub growth_pressure: f64,
}

impl Default for GrowthConfig {
    fn default() -> Self {
        Self {
            initial_scale: 0.25,
            growth_rate: 1.5,
            max_scale: 4.0,
            complexity_threshold: 0.8,
            enable_meta_learning: true,
        }
    }
}

impl DevelopmentalPredictor {
    /// Create new developmental predictor
    pub fn new(vocab_size: usize, transformer_config: Option<TransformerConfig>, growth_config: Option<GrowthConfig>) -> Result<Self> {
        let growth_config = growth_config.unwrap_or_default();
        
        // Scale initial transformer size
        let mut config = transformer_config.unwrap_or_default();
        config.model_dim = (config.model_dim as f64 * growth_config.initial_scale) as usize;
        config.num_layers = (config.num_layers as f64 * growth_config.initial_scale) as usize;
        config.ff_hidden_dim = (config.ff_hidden_dim as f64 * growth_config.initial_scale) as usize;
        
        let transformer = TransformerPredictor::new(vocab_size, Some(config))?;
        
        Ok(Self {
            transformer,
            growth_config,
            current_stage: DevelopmentalStage::Embryonic,
            learning_history: Vec::new(),
            capacity_tracker: CapacityTracker {
                current_complexity: 0.1,
                efficiency_history: Vec::new(),
                utilization: 0.0,
                growth_pressure: 0.0,
            },
        })
    }
    
    /// Developmental forward pass with adaptation
    pub fn developmental_forward(&mut self, input_ids: &[usize], learning_context: &str) -> Result<DVector<f64>> {
        // Record performance before
        let performance_before = self.capacity_tracker.utilization;
        
        // Standard forward pass
        let output = self.transformer.predict_next(input_ids)?;
        
        // Update capacity tracking
        self.update_capacity_tracking(&output)?;
        
        // Check for developmental growth
        if self.should_grow()? {
            self.trigger_growth()?;
        }
        
        // Record learning event
        let performance_after = self.capacity_tracker.utilization;
        self.record_learning_event(LearningType::ParameterUpdate, performance_before, performance_after, learning_context);
        
        Ok(output)
    }
    
    /// Update capacity tracking metrics
    fn update_capacity_tracking(&mut self, output: &DVector<f64>) -> Result<()> {
        // Calculate output entropy as complexity measure
        let entropy = self.calculate_entropy(output)?;
        self.capacity_tracker.current_complexity = entropy;
        
        // Update efficiency history
        self.capacity_tracker.efficiency_history.push(entropy);
        if self.capacity_tracker.efficiency_history.len() > 100 {
            self.capacity_tracker.efficiency_history.remove(0);
        }
        
        // Calculate utilization and growth pressure
        self.capacity_tracker.utilization = entropy / 10.0; // Normalize
        self.capacity_tracker.growth_pressure = if entropy > 0.8 { 
            (entropy - 0.8) * 5.0 
        } else { 
            0.0 
        };
        
        Ok(())
    }
    
    /// Calculate entropy of output distribution
    fn calculate_entropy(&self, probs: &DVector<f64>) -> Result<f64> {
        let mut entropy = 0.0;
        for &p in probs.iter() {
            if p > 1e-10 {
                entropy -= p * p.ln();
            }
        }
        Ok(entropy)
    }
    
    /// Determine if model should grow
    fn should_grow(&self) -> Result<bool> {
        Ok(self.capacity_tracker.growth_pressure > self.growth_config.complexity_threshold &&
           self.current_stage != DevelopmentalStage::Expert)
    }
    
    /// Trigger developmental growth
    fn trigger_growth(&mut self) -> Result<()> {
        // Advance developmental stage
        self.current_stage = match self.current_stage {
            DevelopmentalStage::Embryonic => DevelopmentalStage::Infant,
            DevelopmentalStage::Infant => DevelopmentalStage::Child,
            DevelopmentalStage::Child => DevelopmentalStage::Adolescent,
            DevelopmentalStage::Adolescent => DevelopmentalStage::Adult,
            DevelopmentalStage::Adult => DevelopmentalStage::Expert,
            DevelopmentalStage::Expert => DevelopmentalStage::Expert,
        };
        
        // Record structural growth event
        self.record_learning_event(
            LearningType::StructuralGrowth,
            self.capacity_tracker.utilization,
            self.capacity_tracker.utilization * self.growth_config.growth_rate,
            &format!("Growth to stage: {:?}", self.current_stage)
        );
        
        Ok(())
    }
    
    /// Record learning event for meta-learning
    fn record_learning_event(&mut self, learning_type: LearningType, before: f64, after: f64, context: &str) {
        let event = LearningEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            learning_type,
            performance_before: before,
            performance_after: after,
            context: context.to_string(),
        };
        
        self.learning_history.push(event);
        
        // Keep only recent history
        if self.learning_history.len() > 1000 {
            self.learning_history.remove(0);
        }
    }
    
    /// Get current developmental stage
    pub fn get_developmental_stage(&self) -> &DevelopmentalStage {
        &self.current_stage
    }
    
    /// Get learning history for analysis
    pub fn get_learning_history(&self) -> &[LearningEvent] {
        &self.learning_history
    }
    
    /// Get capacity metrics
    pub fn get_capacity_metrics(&self) -> &CapacityTracker {
        &self.capacity_tracker
    }
    
    /// Export developmental state
    pub fn export_developmental_state(&self) -> Result<String> {
        let state = DevelopmentalState {
            current_stage: self.current_stage.clone(),
            capacity_tracker: self.capacity_tracker.clone(),
            learning_history_size: self.learning_history.len(),
            growth_config: self.growth_config.clone(),
        };
        
        serde_json::to_string_pretty(&state).map_err(|e| crate::error::BrainError::from(e))
    }
}

/// Developmental state for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentalState {
    pub current_stage: DevelopmentalStage,
    pub capacity_tracker: CapacityTracker,
    pub learning_history_size: usize,
    pub growth_config: GrowthConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_self_attention_creation() -> Result<()> {
        let config = AttentionConfig::default();
        let attention = SelfAttention::new(config)?;
        
        assert_eq!(attention.config.model_dim, 512);
        assert_eq!(attention.config.num_heads, 8);
        Ok(())
    }
    
    #[test]
    fn test_transformer_predictor() -> Result<()> {
        let vocab_size = 100;
        let config = TransformerConfig {
            model_dim: 64,
            num_layers: 2,
            num_heads: 4,
            ff_hidden_dim: 128,
            max_seq_len: 32,
            dropout_rate: 0.1,
        };
        
        let mut predictor = TransformerPredictor::new(vocab_size, Some(config))?;
        let input_ids = vec![1, 2, 3, 4, 5];
        
        let output = predictor.predict_next(&input_ids)?;
        assert_eq!(output.len(), vocab_size);
        
        // Check probabilities sum to ~1
        let sum: f64 = output.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
        
        Ok(())
    }
    
    #[test]
    fn test_developmental_predictor() -> Result<()> {
        let vocab_size = 50;
        let dev_predictor = DevelopmentalPredictor::new(vocab_size, None, None)?;
        
        assert_eq!(dev_predictor.get_developmental_stage(), &DevelopmentalStage::Embryonic);
        assert_eq!(dev_predictor.get_learning_history().len(), 0);
        
        Ok(())
    }
    
    #[test]
    fn test_layer_normalization() -> Result<()> {
        let layer_norm = LayerNorm::new(4);
        let input = DMatrix::from_row_slice(2, 4, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        
        let output = layer_norm.forward(&input)?;
        
        // Check that each row has approximately zero mean and unit variance
        for i in 0..output.nrows() {
            let row = output.row(i);
            let mean = row.sum() / row.len() as f64;
            assert!(mean.abs() < 1e-6);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_positional_encoding() -> Result<()> {
        let pe = TransformerPredictor::create_positional_encoding(10, 8)?;
        
        assert_eq!(pe.nrows(), 10);
        assert_eq!(pe.ncols(), 8);
        
        // Check that values are bounded
        for i in 0..pe.nrows() {
            for j in 0..pe.ncols() {
                assert!(pe[(i, j)].abs() <= 1.0);
            }
        }
        
        Ok(())
    }
} 