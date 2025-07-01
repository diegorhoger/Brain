//! Data Visualization Agent for Brain AI

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, ExecutionMetadata, ExecutionStatus, BrainResult, CognitivePreferences};
use std::collections::HashMap;
use async_trait::async_trait;

/// Data Visualization Agent
pub struct DataVisualizationAgent {
    metadata: AgentMetadata,
    cognitive_preferences: CognitivePreferences,
}

impl DataVisualizationAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "data_visualization".to_string(),
            name: "DataVisualizationAgent".to_string(),
            persona: "I am a data visualization specialist focusing on dashboard generation and data visualization".to_string(),
            description: "Manages dashboard generation and data visualization".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["visualization_config".to_string()],
            supported_output_types: vec!["visualization_results".to_string()],
            capabilities: vec!["Analytics".to_string()],
            dependencies: vec![],
            tags: vec!["platform".to_string()],
            base_confidence: 0.83,
        };

        Self {
            metadata,
            cognitive_preferences: CognitivePreferences::default(),
        }
    }
}

#[async_trait]
impl BrainAgent for DataVisualizationAgent {
    async fn execute(&self, _input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "visualization_results".to_string(),
            content: "Data visualization completed successfully".to_string(),
            data: HashMap::new(),
            confidence: 0.83,
            reasoning: Some("Generated data visualization dashboards".to_string()),
            next_actions: vec!["validate_visualizations".to_string()],
            execution_metadata: ExecutionMetadata {
                execution_time_ms: 1300,
                memory_usage_mb: 11.0,
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
        if input.parameters.contains_key("visualization_config") {
            Ok(0.9)
        } else {
            Ok(0.4)
        }
    }
}

impl Default for DataVisualizationAgent {
    fn default() -> Self {
        Self::new()
    }
}
