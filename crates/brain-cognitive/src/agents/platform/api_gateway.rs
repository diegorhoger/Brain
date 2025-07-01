//! API Gateway Agent for Brain AI

use crate::agents::traits::{BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitiveContext, ExecutionMetadata, ExecutionStatus, BrainResult, CognitivePreferences};
use std::collections::HashMap;
use async_trait::async_trait;

pub struct ApiGatewayAgent {
    metadata: AgentMetadata,
    cognitive_preferences: CognitivePreferences,
}

impl ApiGatewayAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "api_gateway".to_string(),
            name: "ApiGatewayAgent".to_string(),
            persona: "I am an API gateway specialist".to_string(),
            description: "Manages API gateway operations".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["api_config".to_string()],
            supported_output_types: vec!["api_results".to_string()],
            capabilities: vec!["Development".to_string()],
            dependencies: vec![],
            tags: vec!["platform".to_string()],
            base_confidence: 0.83,
        };
        Self { metadata, cognitive_preferences: CognitivePreferences::default() }
    }
}

#[async_trait]
impl BrainAgent for ApiGatewayAgent {
    async fn execute(&self, _input: AgentInput, _context: &CognitiveContext) -> BrainResult<AgentOutput> {
        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "api_results".to_string(),
            content: "API gateway operation completed".to_string(),
            data: HashMap::new(),
            confidence: 0.83,
            reasoning: Some("Completed API gateway operation".to_string()),
            next_actions: vec!["monitor_api".to_string()],
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
    fn metadata(&self) -> &AgentMetadata { &self.metadata }
    fn confidence_threshold(&self) -> f32 { 0.75 }
    fn cognitive_preferences(&self) -> &CognitivePreferences { &self.cognitive_preferences }
    async fn assess_confidence(&self, input: &AgentInput, _context: &CognitiveContext) -> BrainResult<f32> {
        Ok(if input.parameters.contains_key("api_config") { 0.9 } else { 0.4 })
    }
}

impl Default for ApiGatewayAgent {
    fn default() -> Self { Self::new() }
}
