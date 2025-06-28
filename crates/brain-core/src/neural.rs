//! Neural Architecture Domain Logic and Abstractions
//! 
//! This module defines sophisticated neural architecture abstractions including:
//! - Self-attention and multi-head attention mechanisms
//! - Transformer-like encoder-decoder structures  
//! - Post-transformer developmental AI approaches
//! - Advanced layer types with residual connections and normalization

use brain_types::*;
use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};

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

/// Transformer configuration
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

/// Developmental growth configuration
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

impl Default for GrowthConfig {
    fn default() -> Self {
        Self {
            initial_scale: 0.5,
            growth_rate: 1.2,
            max_scale: 4.0,
            complexity_threshold: 0.8,
            enable_meta_learning: true,
        }
    }
}

/// Developmental stages for AI growth
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DevelopmentalStage {
    Embryonic,
    Infant,
    Child,
    Adolescent,
    Adult,
    Expert,
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

/// Learning event record
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

/// Capacity tracking for developmental AI
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

impl Default for CapacityTracker {
    fn default() -> Self {
        Self {
            current_complexity: 0.0,
            efficiency_history: Vec::new(),
            utilization: 0.0,
            growth_pressure: 0.0,
        }
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

/// Neural network layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    pub input_size: usize,
    pub output_size: usize,
    pub activation: ActivationType,
}

/// Activation function types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationType {
    ReLU,
    Sigmoid,
    Tanh,
    Linear,
}

/// Neural network architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralArchitecture {
    pub layers: Vec<LayerConfig>,
    pub learning_rate: f64,
}

/// Self-attention mechanism trait
#[async_trait::async_trait]
pub trait SelfAttentionService: Send + Sync {
    /// Forward pass through self-attention
    async fn forward(&mut self, input: &DMatrix<f64>) -> Result<DMatrix<f64>>;
    
    /// Get attention weights for visualization
    async fn get_attention_weights(&self) -> Option<DMatrix<f64>>;
}

/// Transformer encoder trait
#[async_trait::async_trait]
pub trait TransformerEncoderService: Send + Sync {
    /// Forward pass through transformer encoder
    async fn forward(&mut self, input: &DMatrix<f64>) -> Result<DMatrix<f64>>;
}

/// Transformer predictor trait
#[async_trait::async_trait]
pub trait TransformerPredictorService: Send + Sync {
    /// Forward pass with input token IDs
    async fn forward(&mut self, input_ids: &[usize]) -> Result<DMatrix<f64>>;
    
    /// Predict next token probabilities
    async fn predict_next(&mut self, input_ids: &[usize]) -> Result<DVector<f64>>;
    
    /// Get attention maps from all layers
    async fn get_attention_maps(&self) -> Vec<Option<DMatrix<f64>>>;
}

/// Developmental predictor trait
#[async_trait::async_trait]
pub trait DevelopmentalPredictorService: Send + Sync {
    /// Forward pass with developmental learning
    async fn developmental_forward(&mut self, input_ids: &[usize], learning_context: &str) -> Result<DVector<f64>>;
    
    /// Get current developmental stage
    async fn get_developmental_stage(&self) -> DevelopmentalStage;
    
    /// Get learning history
    async fn get_learning_history(&self) -> Vec<LearningEvent>;
    
    /// Get capacity metrics
    async fn get_capacity_metrics(&self) -> CapacityTracker;
    
    /// Export developmental state
    async fn export_developmental_state(&self) -> Result<String>;
}

/// Feed-forward network trait
#[async_trait::async_trait]
pub trait FeedForwardService: Send + Sync {
    /// Forward pass through feed-forward network
    async fn forward(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>>;
}

/// Layer normalization trait
#[async_trait::async_trait]
pub trait LayerNormService: Send + Sync {
    /// Forward pass through layer normalization
    async fn forward(&self, input: &DMatrix<f64>) -> Result<DMatrix<f64>>;
}

/// Repository trait for neural models
#[async_trait::async_trait]
pub trait NeuralRepository: Send + Sync {
    async fn save_model(&mut self, model: &NeuralArchitecture) -> Result<()>;
    async fn load_model(&self) -> Result<Option<NeuralArchitecture>>;
    
    /// Save transformer configuration
    async fn save_transformer_config(&mut self, config: &TransformerConfig) -> Result<()>;
    
    /// Load transformer configuration
    async fn load_transformer_config(&self) -> Result<Option<TransformerConfig>>;
    
    /// Save developmental state
    async fn save_developmental_state(&mut self, state: &DevelopmentalState) -> Result<()>;
    
    /// Load developmental state
    async fn load_developmental_state(&self) -> Result<Option<DevelopmentalState>>;
}

/// Neural service for model management
pub struct NeuralService {
    repository: Box<dyn NeuralRepository>,
}

impl NeuralService {
    pub fn new(repository: Box<dyn NeuralRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_model(&mut self, layers: Vec<LayerConfig>) -> Result<NeuralArchitecture> {
        let model = NeuralArchitecture {
            layers,
            learning_rate: 0.001,
        };
        self.repository.save_model(&model).await?;
        Ok(model)
    }

    pub async fn create_transformer_config(&mut self, config: TransformerConfig) -> Result<()> {
        self.repository.save_transformer_config(&config).await
    }

    pub async fn get_transformer_config(&self) -> Result<Option<TransformerConfig>> {
        self.repository.load_transformer_config().await
    }

    pub async fn save_developmental_state(&mut self, state: DevelopmentalState) -> Result<()> {
        self.repository.save_developmental_state(&state).await
    }

    pub async fn get_developmental_state(&self) -> Result<Option<DevelopmentalState>> {
        self.repository.load_developmental_state().await
    }
}
