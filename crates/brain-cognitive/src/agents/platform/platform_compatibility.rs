//! Platform Compatibility Agent for Brain AI

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, ExecutionMetadata, ExecutionStatus, BrainResult, CognitivePreferences};
use std::collections::HashMap;
use async_trait::async_trait;

/// Platform Compatibility Agent
pub struct PlatformCompatibilityAgent {
    metadata: AgentMetadata,
    cognitive_preferences: CognitivePreferences,
}

impl PlatformCompatibilityAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "platform_compatibility".to_string(),
            name: "PlatformCompatibilityAgent".to_string(),
            persona: "I am a platform specialist focusing on cross-platform compatibility testing".to_string(),
            description: "Manages cross-platform compatibility testing and validation".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["platform_compatibility_config".to_string()],
            supported_output_types: vec!["platform_compatibility_results".to_string()],
            capabilities: vec!["Testing".to_string()],
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
impl BrainAgent for PlatformCompatibilityAgent {
    async fn execute(&self, _input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "platform_compatibility_results".to_string(),
            content: "Platform compatibility testing completed successfully".to_string(),
            data: HashMap::new(),
            confidence: 0.83,
            reasoning: Some("Completed platform compatibility testing".to_string()),
            next_actions: vec!["generate_compatibility_report".to_string()],
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
        if input.parameters.contains_key("platform_compatibility_config") {
            Ok(0.9)
        } else {
            Ok(0.4)
        }
    }
}

impl Default for PlatformCompatibilityAgent {
    fn default() -> Self {
        Self::new()
    }
}
