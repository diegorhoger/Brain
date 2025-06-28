//! Conversation Service Traits
//! 
//! This module defines trait abstractions for conversation services to enable
//! clean dependency injection and testability.

use brain_types::BrainError;
use brain_core::{
    memory::WorkingMemoryRepository,
    concepts::ConceptRepository,
    insights::InsightRepository,
};
use super::{
    RagRequest, RagResponse, RetrievedKnowledge, ConversationContext,
    ResponseQuality, SafetyFlags, SourceAttribution
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Main conversation service trait for processing conversations
#[async_trait]
pub trait ConversationService: Send + Sync {
    /// Process a conversation request and generate a response
    async fn process_conversation(
        &mut self,
        request: RagRequest,
        memory_repo: &mut dyn WorkingMemoryRepository,
        concept_repo: &mut dyn ConceptRepository,
        insight_repo: &mut dyn InsightRepository,
    ) -> Result<RagResponse, BrainError>;

    /// Get conversation statistics
    fn get_conversation_stats(&self) -> HashMap<String, usize>;

    /// Clear a specific conversation
    fn clear_conversation(&mut self, conversation_id: &str) -> bool;
}

/// Knowledge retrieval service trait
#[async_trait]
pub trait KnowledgeRetriever {
    /// Retrieve relevant knowledge based on a query
    async fn retrieve_knowledge(
        &mut self,
        message: &str,
        context: &ConversationContext,
        memory_repo: &mut dyn WorkingMemoryRepository,
        concept_repo: &mut dyn ConceptRepository,
        insight_repo: &mut dyn InsightRepository,
        threshold: f64,
        limit: usize,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError>;

    /// Calculate relevance score between content and query
    fn calculate_relevance(&self, content: &str, query: &str) -> f64;

    /// Calculate text similarity
    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f64;
}

/// Response generation service trait
#[async_trait]
pub trait ResponseGenerator {
    /// Generate a response using external LLM
    async fn generate_with_external_llm(
        &self,
        message: &str,
        context: &ConversationContext,
        knowledge: &[RetrievedKnowledge],
    ) -> Result<String, BrainError>;

    /// Validate response quality and safety
    async fn validate_response_quality(
        &self,
        response: &str,
        knowledge: &[RetrievedKnowledge],
        original_query: &str,
        context: &ConversationContext,
    ) -> Result<(ResponseQuality, SafetyFlags, SourceAttribution), BrainError>;
}

/// Context management service trait
#[async_trait]
pub trait ContextManager {
    /// Update conversation context with new message
    async fn update_context(
        &mut self,
        context: &mut ConversationContext,
        message: &str,
        response: &str,
        knowledge_used: &[RetrievedKnowledge],
    ) -> Result<(), BrainError>;

    /// Extract user profile from conversation context
    fn extract_user_profile(&self, context: &ConversationContext) -> HashMap<String, String>;

    /// Track temporal patterns in conversation
    async fn track_temporal_patterns(
        &mut self,
        context: &mut ConversationContext,
    ) -> Result<(), BrainError>;
}

/// Memory integration service trait
#[async_trait]
pub trait MemoryIntegrator {
    /// Store interaction in memory for future learning
    async fn store_interaction_in_memory(
        &self,
        user_message: &str,
        assistant_response: &str,
        knowledge_used: &[RetrievedKnowledge],
        memory_repo: &mut dyn WorkingMemoryRepository,
    ) -> Result<(), BrainError>;

    /// Retrieve conversation history from memory
    async fn retrieve_conversation_history(
        &self,
        conversation_id: &str,
        memory_repo: &dyn WorkingMemoryRepository,
        limit: usize,
    ) -> Result<Vec<String>, BrainError>;
}

/// Training data integration trait
#[async_trait]
pub trait TrainingDataIntegrator {
    /// Enable training data collection
    async fn enable_training_data_collection(&mut self) -> Result<(), BrainError>;

    /// Disable training data collection
    async fn disable_training_data_collection(&mut self);

    /// Check if training data collection is enabled
    fn is_training_data_collection_enabled(&self) -> bool;
}

/// Configuration trait for conversation services
pub trait ConversationConfig {
    /// Get maximum tokens for response generation
    fn get_max_tokens(&self) -> u32;

    /// Get temperature for response generation
    fn get_temperature(&self) -> f64;

    /// Get retrieval threshold
    fn get_retrieval_threshold(&self) -> f64;

    /// Get context limit
    fn get_context_limit(&self) -> usize;

    /// Get API configuration
    fn get_api_config(&self) -> HashMap<String, String>;
} 