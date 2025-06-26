//! Neural Infrastructure Implementations
//! 
//! This module provides in-memory implementations of neural-related repositories
//! for development and testing purposes.

use brain_core::*;
use brain_types::*;
use std::sync::{Arc, RwLock};

/// In-memory implementation of NeuralRepository
pub struct InMemoryNeuralRepository {
    model: Arc<RwLock<Option<NeuralArchitecture>>>,
}

impl InMemoryNeuralRepository {
    pub fn new() -> Self {
        Self {
            model: Arc::new(RwLock::new(None)),
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
} 