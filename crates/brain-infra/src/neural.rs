//! Neural Infrastructure Implementations
//! 
//! This module provides sophisticated neural architecture implementations including:
//! - Self-attention mechanisms with multi-head support
//! - Transformer encoders with feed-forward networks
//! - Developmental AI with growth and learning capabilities
//! - Advanced mathematical operations and optimizations

use brain_core::*;
use brain_types::*;
use nalgebra::{DMatrix, DVector};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Self-attention mechanism implementation
pub struct SelfAttentionImpl {
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

impl SelfAttentionImpl {
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
}

#[async_trait::async_trait]
impl SelfAttentionService for SelfAttentionImpl {
    /// Forward pass through self-attention
    async fn forward(&mut self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        let (_seq_len, model_dim) = input.shape();
        
        if model_dim != self.config.model_dim {
            return Err(BrainError::InvalidInput(
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
    
    /// Get attention weights for visualization
    async fn get_attention_weights(&self) -> Option<DMatrix<f64>> {
        self.attention_weights.clone()
    }
}

/// Feed-forward network implementation
pub struct FeedForwardNetworkImpl {
    /// First linear layer
    linear1: DMatrix<f64>,
    /// Second linear layer  
    linear2: DMatrix<f64>,
    /// Bias vectors
    bias1: DVector<f64>,
    bias2: DVector<f64>,
    /// Hidden dimension
    #[allow(dead_code)]
    hidden_dim: usize,
}

impl FeedForwardNetworkImpl {
    /// Create new feed-forward network
    pub fn new(input_dim: usize, hidden_dim: usize) -> Result<Self> {
        let linear1 = Self::xavier_init(input_dim, hidden_dim)?;
        let linear2 = Self::xavier_init(hidden_dim, input_dim)?;
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

#[async_trait::async_trait]
impl FeedForwardService for FeedForwardNetworkImpl {
    /// Forward pass through feed-forward network
    async fn forward(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        // First linear transformation: input * W1 + b1
        let hidden = input * &self.linear1;
        
        // Add bias (broadcasting)
        let mut hidden_with_bias = hidden.clone();
        for i in 0..hidden_with_bias.nrows() {
            for j in 0..hidden_with_bias.ncols() {
                hidden_with_bias[(i, j)] += self.bias1[j];
            }
        }
        
        // Apply ReLU activation
        let activated = self.relu(&hidden_with_bias)?;
        
        // Second linear transformation: hidden * W2 + b2
        let output = &activated * &self.linear2;
        
        // Add bias
        let mut output_with_bias = output;
        for i in 0..output_with_bias.nrows() {
            for j in 0..output_with_bias.ncols() {
                output_with_bias[(i, j)] += self.bias2[j];
            }
        }
        
        Ok(output_with_bias)
    }
}

/// Layer normalization implementation
pub struct LayerNormImpl {
    /// Learnable scale parameters
    gamma: DVector<f64>,
    /// Learnable shift parameters
    beta: DVector<f64>,
    /// Small constant for numerical stability
    epsilon: f64,
}

impl LayerNormImpl {
    /// Create new layer normalization
    pub fn new(dim: usize) -> Self {
        Self {
            gamma: DVector::from_element(dim, 1.0),
            beta: DVector::zeros(dim),
            epsilon: 1e-5,
        }
    }
}

#[async_trait::async_trait]
impl LayerNormService for LayerNormImpl {
    /// Forward pass through layer normalization
    async fn forward(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        let (rows, cols) = input.shape();
        let mut output = DMatrix::zeros(rows, cols);
        
        for i in 0..rows {
            let row = input.row(i);
            
            // Compute mean and variance
            let mean = row.mean();
            let variance = row.map(|x| (x - mean).powi(2)).mean();
            let std_dev = (variance + self.epsilon).sqrt();
            
            // Normalize and apply learnable parameters
            for j in 0..cols {
                let normalized = (row[j] - mean) / std_dev;
                output[(i, j)] = self.gamma[j] * normalized + self.beta[j];
            }
        }
        
        Ok(output)
    }
}

/// Transformer encoder implementation
pub struct TransformerEncoderImpl {
    /// Self-attention layer
    self_attention: SelfAttentionImpl,
    /// Feed-forward network
    feed_forward: FeedForwardNetworkImpl,
    /// Layer normalization for attention
    layer_norm1: LayerNormImpl,
    /// Layer normalization for feed-forward
    layer_norm2: LayerNormImpl,
    /// Dropout rate (reserved for future implementation)
    _dropout_rate: f64,
}

impl TransformerEncoderImpl {
    /// Create new transformer encoder
    pub fn new(config: AttentionConfig, ff_hidden_dim: usize) -> Result<Self> {
        let self_attention = SelfAttentionImpl::new(config.clone())?;
        let feed_forward = FeedForwardNetworkImpl::new(config.model_dim, ff_hidden_dim)?;
        let layer_norm1 = LayerNormImpl::new(config.model_dim);
        let layer_norm2 = LayerNormImpl::new(config.model_dim);
        
        Ok(Self {
            self_attention,
            feed_forward,
            layer_norm1,
            layer_norm2,
            _dropout_rate: config.dropout_rate,
        })
    }
}

#[async_trait::async_trait]
impl TransformerEncoderService for TransformerEncoderImpl {
    /// Forward pass through transformer encoder
    async fn forward(&mut self, input: &DMatrix<f64>) -> Result<DMatrix<f64>> {
        // Self-attention with residual connection and layer norm
        let attention_output = self.self_attention.forward(input).await?;
        let residual1 = input + &attention_output;
        let norm1_output = self.layer_norm1.forward(&residual1).await?;
        
        // Feed-forward with residual connection and layer norm
        let ff_output = self.feed_forward.forward(&norm1_output).await?;
        let residual2 = &norm1_output + &ff_output;
        let norm2_output = self.layer_norm2.forward(&residual2).await?;
        
        Ok(norm2_output)
    }
}

/// Transformer predictor implementation
pub struct TransformerPredictorImpl {
    /// Input embedding layer
    embedding: DMatrix<f64>,
    /// Positional encoding
    positional_encoding: DMatrix<f64>,
    /// Stack of transformer encoder layers
    encoders: Vec<TransformerEncoderImpl>,
    /// Output projection layer
    output_projection: DMatrix<f64>,
    /// Configuration
    config: TransformerConfig,
    /// Vocabulary size
    vocab_size: usize,
}

impl TransformerPredictorImpl {
    /// Create new transformer predictor
    pub fn new(vocab_size: usize, config: Option<TransformerConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();
        
        // Initialize embedding matrix
        let embedding = Self::xavier_init(vocab_size, config.model_dim)?;
        
        // Create positional encoding
        let positional_encoding = Self::create_positional_encoding(config.max_seq_len, config.model_dim)?;
        
        // Create encoder layers
        let mut encoders = Vec::new();
        for _ in 0..config.num_layers {
            let attention_config = AttentionConfig {
                model_dim: config.model_dim,
                num_heads: config.num_heads,
                head_dim: config.model_dim / config.num_heads,
                dropout_rate: config.dropout_rate,
                use_scaling: true,
            };
            encoders.push(TransformerEncoderImpl::new(attention_config, config.ff_hidden_dim)?);
        }
        
        // Output projection
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
    
    /// Create positional encoding
    fn create_positional_encoding(max_len: usize, model_dim: usize) -> Result<DMatrix<f64>> {
        let mut pos_encoding = DMatrix::zeros(max_len, model_dim);
        
        for pos in 0..max_len {
            for i in 0..model_dim {
                let angle = pos as f64 / 10000.0_f64.powf(2.0 * (i / 2) as f64 / model_dim as f64);
                if i % 2 == 0 {
                    pos_encoding[(pos, i)] = angle.sin();
                } else {
                    pos_encoding[(pos, i)] = angle.cos();
                }
            }
        }
        
        Ok(pos_encoding)
    }
    
    /// Apply softmax to logits
    fn softmax(&self, logits: &DVector<f64>) -> Result<DVector<f64>> {
        let max_logit = logits.max();
        let mut exp_logits = DVector::zeros(logits.len());
        let mut sum_exp = 0.0;
        
        for i in 0..logits.len() {
            let exp_val = (logits[i] - max_logit).exp();
            exp_logits[i] = exp_val;
            sum_exp += exp_val;
        }
        
        for i in 0..exp_logits.len() {
            exp_logits[i] /= sum_exp;
        }
        
        Ok(exp_logits)
    }
}

#[async_trait::async_trait]
impl TransformerPredictorService for TransformerPredictorImpl {
    /// Forward pass with input token IDs
    async fn forward(&mut self, input_ids: &[usize]) -> Result<DMatrix<f64>> {
        if input_ids.is_empty() {
            return Err(BrainError::InvalidInput("Input sequence cannot be empty".to_string()));
        }
        
        let seq_len = input_ids.len();
        if seq_len > self.config.max_seq_len {
            return Err(BrainError::InvalidInput(
                format!("Sequence length {} exceeds maximum {}", seq_len, self.config.max_seq_len)
            ));
        }
        
        // Create input embeddings
        let mut input_embeddings = DMatrix::zeros(seq_len, self.config.model_dim);
        for (i, &token_id) in input_ids.iter().enumerate() {
            if token_id >= self.vocab_size {
                return Err(BrainError::InvalidInput(
                    format!("Token ID {} exceeds vocabulary size {}", token_id, self.vocab_size)
                ));
            }
            
            for j in 0..self.config.model_dim {
                input_embeddings[(i, j)] = self.embedding[(token_id, j)];
            }
        }
        
        // Add positional encoding
        for i in 0..seq_len {
            for j in 0..self.config.model_dim {
                input_embeddings[(i, j)] += self.positional_encoding[(i, j)];
            }
        }
        
        // Pass through encoder layers
        let mut hidden_states = input_embeddings;
        for encoder in &mut self.encoders {
            hidden_states = encoder.forward(&hidden_states).await?;
        }
        
        Ok(hidden_states)
    }
    
    /// Predict next token probabilities
    async fn predict_next(&mut self, input_ids: &[usize]) -> Result<DVector<f64>> {
        let hidden_states = self.forward(input_ids).await?;
        
        // Use the last position for prediction
        let last_hidden = hidden_states.row(hidden_states.nrows() - 1);
        
        // Apply output projection
        let mut logits = DVector::zeros(self.vocab_size);
        for i in 0..self.vocab_size {
            for j in 0..self.config.model_dim {
                logits[i] += last_hidden[j] * self.output_projection[(j, i)];
            }
        }
        
        // Apply softmax to get probabilities
        self.softmax(&logits)
    }
    
    /// Get attention maps from all layers
    async fn get_attention_maps(&self) -> Vec<Option<DMatrix<f64>>> {
        self.encoders.iter()
            .map(|encoder| encoder.self_attention.attention_weights.clone())
            .collect()
    }
}

/// Developmental predictor implementation
pub struct DevelopmentalPredictorImpl {
    /// Base transformer architecture
    transformer: TransformerPredictorImpl,
    /// Developmental parameters
    growth_config: GrowthConfig,
    /// Current developmental stage
    current_stage: DevelopmentalStage,
    /// Learning history for meta-learning
    learning_history: Vec<LearningEvent>,
    /// Adaptive capacity tracking
    capacity_tracker: CapacityTracker,
}

impl DevelopmentalPredictorImpl {
    /// Create new developmental predictor
    pub fn new(vocab_size: usize, transformer_config: Option<TransformerConfig>, growth_config: Option<GrowthConfig>) -> Result<Self> {
        let transformer = TransformerPredictorImpl::new(vocab_size, transformer_config)?;
        let growth_config = growth_config.unwrap_or_default();
        
        Ok(Self {
            transformer,
            growth_config,
            current_stage: DevelopmentalStage::Embryonic,
            learning_history: Vec::new(),
            capacity_tracker: CapacityTracker::default(),
        })
    }
    
    /// Update capacity tracking
    fn update_capacity_tracking(&mut self, output: &DVector<f64>) -> Result<()> {
        // Calculate entropy as a measure of output uncertainty
        let entropy = self.calculate_entropy(output)?;
        
        // Update complexity based on entropy
        self.capacity_tracker.current_complexity = entropy;
        
        // Calculate utilization (simplified)
        self.capacity_tracker.utilization = entropy / 10.0; // Normalize roughly
        
        // Update efficiency history
        self.capacity_tracker.efficiency_history.push(1.0 - entropy / 10.0);
        
        // Keep only recent history
        if self.capacity_tracker.efficiency_history.len() > 100 {
            self.capacity_tracker.efficiency_history.remove(0);
        }
        
        // Calculate growth pressure
        let avg_efficiency = if self.capacity_tracker.efficiency_history.is_empty() {
            0.5
        } else {
            self.capacity_tracker.efficiency_history.iter().sum::<f64>() / self.capacity_tracker.efficiency_history.len() as f64
        };
        
        self.capacity_tracker.growth_pressure = if avg_efficiency > self.growth_config.complexity_threshold {
            (avg_efficiency - self.growth_config.complexity_threshold) * 2.0
        } else {
            0.0
        };
        
        Ok(())
    }
    
    /// Calculate entropy of probability distribution
    fn calculate_entropy(&self, probs: &DVector<f64>) -> Result<f64> {
        let mut entropy = 0.0;
        for &p in probs.iter() {
            if p > 0.0 {
                entropy -= p * p.ln();
            }
        }
        Ok(entropy)
    }
    
    /// Check if growth should be triggered
    fn should_grow(&self) -> Result<bool> {
        Ok(self.capacity_tracker.growth_pressure > 0.5)
    }
    
    /// Trigger growth event
    fn trigger_growth(&mut self) -> Result<()> {
        // Advance developmental stage
        self.current_stage = match self.current_stage {
            DevelopmentalStage::Embryonic => DevelopmentalStage::Infant,
            DevelopmentalStage::Infant => DevelopmentalStage::Child,
            DevelopmentalStage::Child => DevelopmentalStage::Adolescent,
            DevelopmentalStage::Adolescent => DevelopmentalStage::Adult,
            DevelopmentalStage::Adult => DevelopmentalStage::Expert,
            DevelopmentalStage::Expert => DevelopmentalStage::Expert, // Stay at expert
        };
        
        // Record learning event
        self.record_learning_event(
            LearningType::StructuralGrowth,
            self.capacity_tracker.current_complexity,
            self.capacity_tracker.current_complexity * self.growth_config.growth_rate,
            "Developmental stage advancement",
        );
        
        Ok(())
    }
    
    /// Record a learning event
    fn record_learning_event(&mut self, learning_type: LearningType, before: f64, after: f64, context: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let event = LearningEvent {
            timestamp,
            learning_type,
            performance_before: before,
            performance_after: after,
            context: context.to_string(),
        };
        
        self.learning_history.push(event);
        
        // Keep only recent events
        if self.learning_history.len() > 1000 {
            self.learning_history.remove(0);
        }
    }
}

#[async_trait::async_trait]
impl DevelopmentalPredictorService for DevelopmentalPredictorImpl {
    /// Forward pass with developmental learning
    async fn developmental_forward(&mut self, input_ids: &[usize], learning_context: &str) -> Result<DVector<f64>> {
        let before_complexity = self.capacity_tracker.current_complexity;
        
        // Get prediction from base transformer
        let output = self.transformer.predict_next(input_ids).await?;
        
        // Update capacity tracking
        self.update_capacity_tracking(&output)?;
        
        // Check if growth should be triggered
        if self.should_grow()? {
            self.trigger_growth()?;
        }
        
        // Record learning event
        self.record_learning_event(
            LearningType::ParameterUpdate,
            before_complexity,
            self.capacity_tracker.current_complexity,
            learning_context,
        );
        
        Ok(output)
    }
    
    /// Get current developmental stage
    async fn get_developmental_stage(&self) -> DevelopmentalStage {
        self.current_stage.clone()
    }
    
    /// Get learning history
    async fn get_learning_history(&self) -> Vec<LearningEvent> {
        self.learning_history.clone()
    }
    
    /// Get capacity metrics
    async fn get_capacity_metrics(&self) -> CapacityTracker {
        self.capacity_tracker.clone()
    }
    
    /// Export developmental state
    async fn export_developmental_state(&self) -> Result<String> {
        let state = DevelopmentalState {
            current_stage: self.current_stage.clone(),
            capacity_tracker: self.capacity_tracker.clone(),
            learning_history_size: self.learning_history.len(),
            growth_config: self.growth_config.clone(),
        };
        
        serde_json::to_string_pretty(&state)
            .map_err(|e| BrainError::Serialization { source: Box::new(e) })
    }
}

/// In-memory implementation of NeuralRepository
pub struct InMemoryNeuralRepository {
    model: Arc<RwLock<Option<NeuralArchitecture>>>,
    transformer_config: Arc<RwLock<Option<TransformerConfig>>>,
    developmental_state: Arc<RwLock<Option<DevelopmentalState>>>,
}

impl InMemoryNeuralRepository {
    pub fn new() -> Self {
        Self {
            model: Arc::new(RwLock::new(None)),
            transformer_config: Arc::new(RwLock::new(None)),
            developmental_state: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for InMemoryNeuralRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl NeuralRepository for InMemoryNeuralRepository {
    async fn save_model(&mut self, model: &NeuralArchitecture) -> Result<()> {
        let mut stored_model = self.model.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        *stored_model = Some(model.clone());
        Ok(())
    }

    async fn load_model(&self) -> Result<Option<NeuralArchitecture>> {
        let stored_model = self.model.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(stored_model.clone())
    }
    
    async fn save_transformer_config(&mut self, config: &TransformerConfig) -> Result<()> {
        let mut stored_config = self.transformer_config.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        *stored_config = Some(config.clone());
        Ok(())
    }
    
    async fn load_transformer_config(&self) -> Result<Option<TransformerConfig>> {
        let stored_config = self.transformer_config.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(stored_config.clone())
    }
    
    async fn save_developmental_state(&mut self, state: &DevelopmentalState) -> Result<()> {
        let mut stored_state = self.developmental_state.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        *stored_state = Some(state.clone());
        Ok(())
    }
    
    async fn load_developmental_state(&self) -> Result<Option<DevelopmentalState>> {
        let stored_state = self.developmental_state.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(stored_state.clone())
    }
}

impl InMemoryNeuralRepository {
    /// Helper method to check if a model is stored
    pub async fn has_model(&self) -> Result<bool> {
        let stored_model = self.model.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(stored_model.is_some())
    }

    /// Helper method to clear the stored model
    pub async fn clear_model(&mut self) -> Result<()> {
        let mut stored_model = self.model.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        *stored_model = None;
        Ok(())
    }
    
    /// Helper method to check if transformer config is stored
    pub async fn has_transformer_config(&self) -> Result<bool> {
        let stored_config = self.transformer_config.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(stored_config.is_some())
    }
    
    /// Helper method to clear transformer config
    pub async fn clear_transformer_config(&mut self) -> Result<()> {
        let mut stored_config = self.transformer_config.write().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        *stored_config = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_self_attention_creation() -> Result<()> {
        let config = AttentionConfig::default();
        let attention = SelfAttentionImpl::new(config)?;
        
        // Test forward pass
        let input = DMatrix::from_element(10, 512, 0.5);
        let mut attention_mut = attention;
        let output = attention_mut.forward(&input).await?;
        
        assert_eq!(output.shape(), (10, 512));
        Ok(())
    }

    #[tokio::test]
    async fn test_transformer_predictor() -> Result<()> {
        let vocab_size = 1000;
        let config = TransformerConfig {
            model_dim: 128,
            num_layers: 2,
            num_heads: 4,
            ff_hidden_dim: 256,
            max_seq_len: 50,
            dropout_rate: 0.1,
        };
        
        let mut predictor = TransformerPredictorImpl::new(vocab_size, Some(config))?;
        
        let input_ids = vec![1, 2, 3, 4, 5];
        let output = predictor.predict_next(&input_ids).await?;
        
        assert_eq!(output.len(), vocab_size);
        
        // Check that probabilities sum to approximately 1
        let sum: f64 = output.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_developmental_predictor() -> Result<()> {
        let vocab_size = 100;
        let transformer_config = TransformerConfig {
            model_dim: 64,
            num_layers: 1,
            num_heads: 2,
            ff_hidden_dim: 128,
            max_seq_len: 20,
            dropout_rate: 0.1,
        };
        
        let mut predictor = DevelopmentalPredictorImpl::new(vocab_size, Some(transformer_config), None)?;
        
        let input_ids = vec![1, 2, 3];
        let output = predictor.developmental_forward(&input_ids, "test context").await?;
        
        assert_eq!(output.len(), vocab_size);
        assert_eq!(predictor.get_developmental_stage().await, DevelopmentalStage::Embryonic);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_layer_normalization() -> Result<()> {
        let layer_norm = LayerNormImpl::new(10);
        
        // Create input with different scales
        let mut input = DMatrix::zeros(5, 10);
        for i in 0..5 {
            for j in 0..10 {
                input[(i, j)] = (i + 1) as f64 * (j + 1) as f64;
            }
        }
        
        let output = layer_norm.forward(&input).await?;
        
        // Check that each row has approximately zero mean and unit variance
        for i in 0..5 {
            let row = output.row(i);
            let mean = row.mean();
            let variance = row.map(|x| (x - mean).powi(2)).mean();
            
            assert!((mean).abs() < 1e-5);
            assert!((variance - 1.0).abs() < 1e-3); // More lenient tolerance for variance
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_neural_repository() -> Result<()> {
        let mut repo = InMemoryNeuralRepository::new();
        
        // Test basic model operations
        assert!(!repo.has_model().await?);
        
        let model = NeuralArchitecture {
            layers: vec![LayerConfig {
                input_size: 10,
                output_size: 5,
                activation: ActivationType::ReLU,
            }],
            learning_rate: 0.001,
        };
        
        repo.save_model(&model).await?;
        assert!(repo.has_model().await?);
        
        let loaded_model = repo.load_model().await?;
        assert!(loaded_model.is_some());
        
        // Test transformer config operations
        let transformer_config = TransformerConfig::default();
        repo.save_transformer_config(&transformer_config).await?;
        assert!(repo.has_transformer_config().await?);
        
        let loaded_config = repo.load_transformer_config().await?;
        assert!(loaded_config.is_some());
        
        Ok(())
    }
} 