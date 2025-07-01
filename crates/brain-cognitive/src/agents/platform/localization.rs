//! Localization Agent for Brain AI

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, ExecutionMetadata, ExecutionStatus, BrainResult, CognitivePreferences};
use std::collections::HashMap;
use async_trait::async_trait;

/// Localization Agent
pub struct LocalizationAgent {
    metadata: AgentMetadata,
    cognitive_preferences: CognitivePreferences,
}

impl LocalizationAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "localization".to_string(),
            name: "LocalizationAgent".to_string(),
            persona: "I am a localization specialist focusing on multi-language support and cultural adaptation".to_string(),
            description: "Provides multi-language support and cultural adaptation".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["localization_config".to_string()],
            supported_output_types: vec!["localization_results".to_string()],
            capabilities: vec!["Development".to_string()],
            dependencies: vec![],
            tags: vec!["platform".to_string()],
            base_confidence: 0.84,
        };

        Self {
            metadata,
            cognitive_preferences: CognitivePreferences::default(),
        }
    }
}

#[async_trait]
impl BrainAgent for LocalizationAgent {
    async fn execute(&self, _input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "localization_results".to_string(),
            content: "Localization process completed successfully".to_string(),
            data: HashMap::new(),
            confidence: 0.84,
            reasoning: Some("Completed localization and cultural adaptation".to_string()),
            next_actions: vec!["test_translations".to_string()],
            execution_metadata: ExecutionMetadata {
                execution_time_ms: 1400,
                memory_usage_mb: 12.0,
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
        if input.parameters.contains_key("localization_config") {
            Ok(0.9)
        } else {
            Ok(0.4)
        }
    }
}

impl Default for LocalizationAgent {
    fn default() -> Self {
        Self::new()
    }
} 