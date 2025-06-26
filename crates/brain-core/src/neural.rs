//! Neural Architecture Domain Logic and Abstractions
//! 
//! This module defines core neural architecture abstractions and domain logic
//! without any I/O dependencies. Infrastructure implementations are
//! provided through trait implementations.

use brain_types::*;
use serde::{Deserialize, Serialize};

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

/// Repository trait for neural models
#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
pub trait NeuralRepository: Send + Sync {
    async fn save_model(&mut self, model: &NeuralArchitecture) -> Result<()>;
    async fn load_model(&self) -> Result<Option<NeuralArchitecture>>;
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
}
