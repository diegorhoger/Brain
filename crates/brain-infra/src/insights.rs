//! Insights Infrastructure Implementations
//! 
//! Concrete implementations of insight repository traits using
//! in-memory storage and database backends.

use brain_core::*;
use brain_types::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// In-memory implementation of InsightRepository
pub struct InMemoryInsightRepository {
    insights: Arc<RwLock<HashMap<Uuid, Insight>>>,
}

impl InMemoryInsightRepository {
    pub fn new() -> Self {
        Self {
            insights: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryInsightRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
#[allow(async_fn_in_trait)]
impl InsightRepository for InMemoryInsightRepository {
    async fn store_insight(&mut self, insight: Insight) -> Result<Uuid> {
        let id = insight.id;
        let mut insights = self.insights.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        insights.insert(id, insight);
        Ok(id)
    }

    async fn get_insight(&self, id: Uuid) -> Result<Option<Insight>> {
        let insights = self.insights.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(insights.get(&id).cloned())
    }

    async fn get_insights_by_type(&self, insight_type: InsightType) -> Result<Vec<Insight>> {
        let insights = self.insights.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        let results: Vec<Insight> = insights
            .values()
            .filter(|insight| insight.insight_type == insight_type)
            .cloned()
            .collect();

        Ok(results)
    }
}

impl InMemoryInsightRepository {
    /// Additional helper methods not in the trait

    pub async fn update_insight(&mut self, insight: &Insight) -> Result<()> {
        let mut insights = self.insights.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        insights.insert(insight.id, insight.clone());
        Ok(())
    }

    pub async fn remove_insight(&mut self, id: Uuid) -> Result<()> {
        let mut insights = self.insights.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        insights.remove(&id);
        Ok(())
    }

    pub async fn update_confidence(&mut self, id: Uuid, new_confidence: f64) -> Result<()> {
        let mut insights = self.insights.write().map_err(|_| BrainError::LockError("Failed to acquire write lock".to_string()))?;
        
        if let Some(insight) = insights.get_mut(&id) {
            insight.confidence = new_confidence.clamp(0.0, 1.0);
        }
        
        Ok(())
    }

    pub async fn get_all_insights(&self) -> Result<Vec<Insight>> {
        let insights = self.insights.read().map_err(|_| BrainError::LockError("Failed to acquire read lock".to_string()))?;
        Ok(insights.values().cloned().collect())
    }
}

 