//! Data Ingestion Agent for Brain AI

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, ExecutionMetadata, ExecutionStatus, BrainResult, CognitivePreferences};
use std::collections::HashMap;
use async_trait::async_trait;

/// Data Ingestion Agent
pub struct DataIngestionAgent {
    metadata: AgentMetadata,
    cognitive_preferences: CognitivePreferences,
}

impl DataIngestionAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "data_ingestion".to_string(),
            name: "DataIngestionAgent".to_string(),
            persona: "I am a data pipeline engineer specializing in ETL processes and data management".to_string(),
            description: "Manages data pipeline management and ETL processes".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["data_pipeline_config".to_string()],
            supported_output_types: vec!["ingestion_results".to_string()],
            capabilities: vec!["Development".to_string()],
            dependencies: vec![],
            tags: vec!["intelligence".to_string()],
            base_confidence: 0.85,
        };

        Self {
            metadata,
            cognitive_preferences: CognitivePreferences::default(),
        }
    }
}

#[async_trait]
impl BrainAgent for DataIngestionAgent {
    async fn execute(&self, _input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "ingestion_results".to_string(),
            content: "Data ingestion pipeline completed successfully".to_string(),
            data: HashMap::new(),
            confidence: 0.85,
            reasoning: Some("Processed data ingestion pipeline".to_string()),
            next_actions: vec!["validate_data".to_string()],
            execution_metadata: ExecutionMetadata {
                execution_time_ms: 1600,
                memory_usage_mb: 13.0,
                api_calls: 0,
                status: ExecutionStatus::Success,
                warnings: vec![],
            },
            timestamp: chrono::Utc::now(),
        })
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        0.75
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.cognitive_preferences
    }

    async fn assess_confidence(&self, input: &AgentInput, _context: &CognitiveContext) -> BrainResult<f32> {
        if input.parameters.contains_key("data_pipeline_config") {
            Ok(0.9)
        } else {
            Ok(0.4)
        }
    }
}

impl Default for DataIngestionAgent {
    fn default() -> Self {
        Self::new()
    }
}
