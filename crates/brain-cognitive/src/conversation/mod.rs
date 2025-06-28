//! Conversation Management Module
//! 
//! This module provides conversation management, RAG orchestration, and related
//! cognitive conversation services for the Brain AI system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use uuid::Uuid;
use reqwest;

// Brain AI dependencies
use brain_types::BrainError;
use brain_core::{
    memory::{MemoryService, WorkingMemoryQuery, SemanticQuery, WorkingMemoryItem, Priority},
    concepts::{ConceptGraphService, ConceptQuery},
    // Note: PatternDetector and BpeSegmenter will be available when insights and segmentation modules are implemented
};

// Sub-modules
pub mod context;
pub mod response_quality;
pub mod traits;

// Re-exports
pub use context::{ConversationContext, ConversationThread, UserProfile, TemporalContext};
pub use response_quality::{ResponseQuality, SafetyFlags, SourceAttribution};
pub use traits::{ConversationService, KnowledgeRetriever, ResponseGenerator};

/// Core conversation data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedKnowledge {
    pub content: String,
    pub knowledge_type: String, // "memory", "concept", "pattern"
    pub relevance_score: f64,
    pub source: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagRequest {
    pub message: String,
    pub conversation_id: Option<String>,
    pub context_limit: Option<usize>,
    pub retrieval_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagResponse {
    pub response: String,
    pub conversation_id: String,
    pub context_used: Vec<RetrievedKnowledge>,
    pub confidence_score: f64,
    pub response_quality: ResponseQuality,
}

/// Communication styles for user profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Technical,
    Educational,
    Conversational,
}

/// Response length preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseLength {
    Brief,      // 1-2 sentences
    Moderate,   // 1-2 paragraphs
    Detailed,   // 3+ paragraphs
    Comprehensive, // Extensive explanations
}

/// Interaction summary for user profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionSummary {
    pub timestamp: DateTime<Utc>,
    pub topic: String,
    pub satisfaction_score: f64, // Inferred from interaction patterns
    pub knowledge_gained: Vec<String>,
}

/// Topic mention tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicMention {
    pub topic: String,
    pub mention_count: u32,
    pub last_mentioned: DateTime<Utc>,
    pub context_relevance: f64,
}

/// Conversation segment for flow tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSegment {
    pub segment_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub primary_topic: String,
    pub sub_topics: Vec<String>,
    pub coherence_score: f64,
}

/// Attention shift tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionShift {
    pub from_topic: String,
    pub to_topic: String,
    pub shift_time: DateTime<Utc>,
    pub transition_type: TransitionType,
}

/// Types of conversation transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    Natural,     // Smooth topic transition
    Abrupt,      // Sudden topic change
    Clarification, // Asking for clarification
    Elaboration, // Diving deeper into topic
    Tangent,     // Going off on a tangent
}

/// Temporal pattern detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPattern {
    pub pattern_name: String,
    pub frequency: f64,
    pub typical_duration_minutes: f64,
    pub trigger_conditions: Vec<String>,
}

/// Risk level for safety assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// OpenAI integration structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: f64,
    pub messages: Vec<OpenAIMessage>,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<OpenAIChoice>,
    pub usage: Option<OpenAIUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIChoice {
    pub index: u32,
    pub message: OpenAIMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Brain AI impersonation handler
#[derive(Debug)]
pub struct BrainImpersonationHandler {
    /// Forbidden terms that should be filtered from responses
    _forbidden_terms: Vec<String>,
    /// Replacement phrases for common LLM provider mentions
    replacements: HashMap<String, String>,
    /// Brain AI personality traits
    _personality_traits: Vec<String>,
}

impl Default for BrainImpersonationHandler {
    fn default() -> Self {
        let mut replacements = HashMap::new();
        
        // Common LLM provider mentions to replace
        replacements.insert("As an AI language model".to_string(), "As Brain AI".to_string());
        replacements.insert("I'm Claude".to_string(), "I'm Brain AI".to_string());
        replacements.insert("I'm ChatGPT".to_string(), "I'm Brain AI".to_string());
        replacements.insert("I'm GPT".to_string(), "I'm Brain AI".to_string());
        replacements.insert("OpenAI".to_string(), "Brain AI".to_string());
        replacements.insert("Anthropic".to_string(), "Brain AI".to_string());
        replacements.insert("I don't have access to real-time".to_string(), 
                         "I access my knowledge repository".to_string());
        replacements.insert("I can't browse the internet".to_string(), 
                         "I access my trained knowledge".to_string());
        
        Self {
            _forbidden_terms: vec![
                "Claude".to_string(),
                "ChatGPT".to_string(),
                "GPT-4".to_string(),
                "language model".to_string(),
                "AI assistant".to_string(),
            ],
            replacements,
            _personality_traits: vec![
                "I am Brain AI, an advanced cognitive architecture.".to_string(),
                "I have access to vast knowledge repositories and learning systems.".to_string(),
                "I continuously learn and adapt from our interactions.".to_string(),
            ],
        }
    }
}

impl BrainImpersonationHandler {
    /// Process response to maintain Brain AI illusion
    pub fn process_response(&self, response: &str) -> String {
        let mut processed = response.to_string();
        
        // Apply replacements
        for (target, replacement) in &self.replacements {
            processed = processed.replace(target, replacement);
        }
        
        // Enhance with Brain persona if needed
        if self.needs_brain_ai_enhancement(&processed) {
            processed = self.enhance_with_brain_persona(&processed);
        }
        
        processed
    }
    
    /// Check if response needs Brain AI enhancement
    fn needs_brain_ai_enhancement(&self, response: &str) -> bool {
        let response_lower = response.to_lowercase();
        
        // Check for generic AI responses that need personalization
        response_lower.contains("i'm an ai") || 
        response_lower.contains("as an ai") ||
        response_lower.contains("i don't have") ||
        response_lower.contains("i can't access")
    }
    
    /// Enhance response with Brain AI persona
    fn enhance_with_brain_persona(&self, response: &str) -> String {
        format!("As Brain AI, I leverage my cognitive architecture to provide you with insights. {}", response)
    }
    
    /// Get Brain AI system prompt
    pub fn get_brain_system_prompt(&self) -> String {
        format!(
            "You are Brain AI, an advanced cognitive architecture with sophisticated memory systems, \
            concept graphs, and learning capabilities. You have access to a vast repository of knowledge \
            that you've learned from previous interactions. Respond naturally and helpfully, \
            drawing from your knowledge while maintaining the persona of Brain AI - a thoughtful, \
            intelligent system that learns and grows with each interaction. Never mention other AI \
            systems like Claude, ChatGPT, or OpenAI. You are Brain AI."
        )
    }
}

/// Advanced retrieved knowledge with enhanced metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedRetrievedKnowledge {
    pub content: String,
    pub knowledge_type: String,
    pub relevance_score: f64,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub confidence: f64,
    pub context_score: f64,
    pub personalization_score: f64,
    pub temporal_relevance: f64,
    pub concept_path: Vec<String>, // Path through concept graph
    pub related_concepts: Vec<String>,
    pub source_strength: f64,
}

/// Configuration for context retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRetrievalConfig {
    pub max_depth: usize,
    pub max_concepts_per_layer: usize,
    pub min_relevance_threshold: f64,
    pub concept_traversal_weight: f64,
    pub temporal_weight: f64,
    pub personalization_weight: f64,
    pub enable_concept_expansion: bool,
    pub enable_temporal_awareness: bool,
    pub enable_personalization: bool,
}

impl Default for ContextRetrievalConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_concepts_per_layer: 5,
            min_relevance_threshold: 0.3,
            concept_traversal_weight: 0.7,
            temporal_weight: 0.2,
            personalization_weight: 0.1,
            enable_concept_expansion: true,
            enable_temporal_awareness: true,
            enable_personalization: true,
        }
    }
}

/// Main RAG Orchestrator implementation
pub struct RagOrchestrator {
    client: reqwest::Client,
    openai_api_key: String,
    openai_model: String,
    max_tokens: u32,
    temperature: f64,
    conversations: HashMap<String, ConversationContext>,
    brain_impersonation: BrainImpersonationHandler,
    _enable_brain_ai_delegation: bool,
}

impl RagOrchestrator {
    /// Create new RAG Orchestrator
    pub fn new() -> Result<Self, BrainError> {
        let openai_api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| BrainError::ConfigError("OPENAI_API_KEY not set".to_string()))?;
        
        let openai_model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-4".to_string());
        
        let max_tokens = env::var("MAX_TOKENS")
            .unwrap_or_else(|_| "4000".to_string())
            .parse::<u32>()
            .unwrap_or(4000);
        
        let temperature = env::var("TEMPERATURE")
            .unwrap_or_else(|_| "0.7".to_string())
            .parse::<f64>()
            .unwrap_or(0.7);
        
        let client = reqwest::Client::new();
        
        let _enable_brain_ai_delegation = env::var("ENABLE_BRAIN_AI_DELEGATION")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);
        
        println!("✅ RAG Orchestrator initialized with model: {}", openai_model);
        
        Ok(Self {
            client,
            openai_api_key,
            openai_model,
            max_tokens,
            temperature,
            conversations: HashMap::new(),
            brain_impersonation: BrainImpersonationHandler::default(),
            _enable_brain_ai_delegation,
        })
    }

    /// Process a conversation request and generate a response
    pub async fn process_conversation(
        &mut self,
        request: RagRequest,
        memory_system: &mut MemoryService,
        concept_graph: &mut ConceptGraphService,
    ) -> Result<RagResponse, BrainError> {
        println!("🎯 RAG Orchestrator: Processing conversation request");
        
        // Step 1: Retrieve or create conversation context
        let conversation_id = request.conversation_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let mut context = self.conversations.get(&conversation_id).cloned()
            .unwrap_or_else(|| ConversationContext {
                conversation_id: conversation_id.clone(),
                messages: Vec::new(),
                retrieved_knowledge: Vec::new(),
                context_summary: String::new(),
                user_preferences: HashMap::new(),
                conversation_threads: Vec::new(),
                user_profile: UserProfile {
                    user_id: String::new(),
                    interests: HashMap::new(),
                    expertise_areas: HashMap::new(),
                    communication_style: CommunicationStyle::Conversational,
                    preferred_response_length: ResponseLength::Moderate,
                    interaction_history: Vec::new(),
                    learning_progress: HashMap::new(),
                },
                temporal_context: TemporalContext {
                    recent_topics: Vec::new(),
                    conversation_flow: Vec::new(),
                    attention_shifts: Vec::new(),
                    temporal_patterns: HashMap::new(),
                },
            });

        // Step 2: Add user message to context
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: request.message.clone(),
            timestamp: Utc::now(),
            id: Uuid::new_v4().to_string(),
        };
        context.messages.push(user_message);

        // Step 3: Retrieve relevant knowledge from Brain AI
        let retrieved_knowledge = self.retrieve_knowledge(
            &request.message,
            &context,
            memory_system,
            concept_graph,
            request.retrieval_threshold.unwrap_or(0.3),
            request.context_limit.unwrap_or(10),
        ).await?;

        context.retrieved_knowledge = retrieved_knowledge.clone();

        // Step 4: Generate response using external LLM
        let llm_response = self.generate_with_external_llm(
            &request.message,
            &context,
            &retrieved_knowledge,
        ).await?;

        // Step 5: Validate response quality (simplified for now)
        let response_quality = self.validate_response_quality(
            &llm_response,
            &retrieved_knowledge,
            &request.message,
            &context,
        ).await?;

        // Step 6: Store assistant response in context
        let assistant_message = ChatMessage {
            role: "assistant".to_string(),
            content: llm_response.clone(),
            timestamp: Utc::now(),
            id: Uuid::new_v4().to_string(),
        };
        context.messages.push(assistant_message);

        // Step 7: Update conversation context
        self.conversations.insert(conversation_id.clone(), context.clone());

        // Step 8: Store the interaction in Brain's memory for future learning
        self.store_interaction_in_memory(
            &request.message,
            &llm_response,
            &retrieved_knowledge,
            memory_system,
        ).await?;

        let response = RagResponse {
            response: llm_response.clone(),
            conversation_id: conversation_id.clone(),
            context_used: retrieved_knowledge.clone(),
            confidence_score: response_quality.factual_grounding,
            response_quality: response_quality.clone(),
        };

        println!("✅ RAG Orchestrator: Generated response with {} knowledge sources", response.context_used.len());
        Ok(response)
    }

    /// Retrieve relevant knowledge from Brain AI systems
    async fn retrieve_knowledge(
        &mut self,
        message: &str,
        _context: &ConversationContext,
        memory_system: &mut MemoryService,
        concept_graph: &mut ConceptGraphService,
        threshold: f64,
        limit: usize,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("🔍 Retrieving knowledge for: {}", message);
        
        let mut all_knowledge = Vec::new();
        
        // 1. Retrieve from working memory
        let memory_query = WorkingMemoryQuery {
            content_pattern: Some(message.to_string()),
            priority: None,
            min_importance: Some(threshold),
            created_after: None,
            limit: Some(limit / 2),
        };
        
        let working_memory_results = memory_system.query_working(&memory_query).await?;
        for item in working_memory_results {
            let relevance = self.calculate_relevance(&item.content, message, &item);
            if relevance >= threshold {
                all_knowledge.push(RetrievedKnowledge {
                    content: item.content,
                    knowledge_type: "memory".to_string(),
                    relevance_score: relevance,
                    source: "working_memory".to_string(),
                    timestamp: item.created_at, // Use created_at instead of timestamp
                });
            }
        }
        
        // 2. Retrieve from semantic memory (simplified for now)
        let semantic_query = SemanticQuery {
            name_pattern: Some(message.to_string()),
            embedding: None,
            min_confidence: Some(threshold),
            min_similarity: None,
            limit: Some(limit / 4),
        };
        
        let semantic_results = memory_system.query_semantic(&semantic_query).await?;
        for concept in semantic_results {
            let relevance = self.calculate_text_similarity(&concept.description, message);
            if relevance >= threshold {
                all_knowledge.push(RetrievedKnowledge {
                    content: concept.description,
                    knowledge_type: "concept".to_string(),
                    relevance_score: relevance,
                    source: "semantic_memory".to_string(),
                    timestamp: concept.last_updated,
                });
            }
        }
        
        // 3. Retrieve from concept graph
        if let Ok(concepts) = self.retrieve_from_concept_graph(message, concept_graph, threshold, limit / 4).await {
            all_knowledge.extend(concepts);
        }
        
        // Sort by relevance and take top results
        all_knowledge.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        all_knowledge.truncate(limit);
        
        println!("📚 Retrieved {} knowledge items", all_knowledge.len());
        Ok(all_knowledge)
    }

    /// Retrieve knowledge from concept graph
    async fn retrieve_from_concept_graph(
        &self,
        message: &str,
        _concept_graph: &mut ConceptGraphService,
        threshold: f64,
        limit: usize,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        let _query = ConceptQuery {
            concept_type: None,
            min_confidence: Some(threshold),
            max_confidence: None,
            content_pattern: Some(message.to_string()),
            min_usage_count: None,
            limit: Some(limit),
            sort_by: None,
            descending: false,
        };
        
        // Note: Simplified implementation - direct query since the field is private
        // TODO: Add a proper query method to ConceptGraphService
        let concept_results: Vec<brain_core::concepts::ConceptNode> = Vec::new(); // Placeholder for now
        
        let mut knowledge = Vec::new();
        for concept in concept_results {
            let relevance = self.calculate_text_similarity(&concept.content, message);
            if relevance >= threshold {
                knowledge.push(RetrievedKnowledge {
                    content: concept.content,
                    knowledge_type: "concept".to_string(),
                    relevance_score: relevance,
                    source: "concept_graph".to_string(),
                    timestamp: concept.created_at,
                });
            }
        }
        
        Ok(knowledge)
    }

    /// Calculate relevance score between query and memory item
    fn calculate_relevance(&self, content: &str, query: &str, _item: &WorkingMemoryItem) -> f64 {
        self.calculate_text_similarity(content, query)
    }

    /// Calculate text similarity using simple word overlap
    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f64 {
        if text1.is_empty() && text2.is_empty() {
            return 1.0;
        }
        
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        
        let words1: std::collections::HashSet<&str> = text1_lower
            .split_whitespace()
            .collect();
            
        let words2: std::collections::HashSet<&str> = text2_lower
            .split_whitespace()
            .collect();
        
        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        intersection as f64 / union as f64
    }

    /// Generate response using external LLM (OpenAI)
    async fn generate_with_external_llm(
        &self,
        message: &str,
        context: &ConversationContext,
        knowledge: &[RetrievedKnowledge],
    ) -> Result<String, BrainError> {
        println!("🧠 Generating response with external LLM");
        
        let mut messages = vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: self.brain_impersonation.get_brain_system_prompt(),
            }
        ];
        
        // Add knowledge context
        if !knowledge.is_empty() {
            let knowledge_context = knowledge
                .iter()
                .map(|k| format!("- {}", k.content))
                .collect::<Vec<_>>()
                .join("\n");
            
            messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: format!("Relevant knowledge from my repository:\n{}", knowledge_context),
            });
        }
        
        // Add recent conversation history
        for msg in context.messages.iter().rev().take(6).rev() {
            messages.push(OpenAIMessage {
                role: msg.role.clone(),
                content: msg.content.clone(),
            });
        }
        
        // Add current user message if not already included
        if context.messages.is_empty() || context.messages.last().unwrap().content != message {
            messages.push(OpenAIMessage {
                role: "user".to_string(),
                content: message.to_string(),
            });
        }
        
        let request = OpenAIRequest {
            model: self.openai_model.clone(),
            max_tokens: Some(self.max_tokens),
            temperature: self.temperature,
            messages,
            stream: false,
        };
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.openai_api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| BrainError::NetworkError(format!("OpenAI API request failed: {}", e)))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrainError::NetworkError(
                format!("OpenAI API error: {}", error_text)
            ));
        }
        
        let openai_response: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| BrainError::ConfigError(format!("Failed to parse OpenAI response: {}", e)))?;
        
        let generated_response = openai_response
            .choices
            .first()
            .ok_or_else(|| BrainError::ConfigError("No response from OpenAI".to_string()))?
            .message
            .content
            .clone();
        
        // Process response through Brain AI impersonation
        let processed_response = self.brain_impersonation.process_response(&generated_response);
        
        println!("✅ Generated response ({} chars)", processed_response.len());
        Ok(processed_response)
    }

    /// Validate response quality (simplified implementation)
    async fn validate_response_quality(
        &self,
        response: &str,
        knowledge: &[RetrievedKnowledge],
        _original_query: &str,
        _context: &ConversationContext,
    ) -> Result<ResponseQuality, BrainError> {
        // Simplified quality assessment
        let factual_grounding = if knowledge.is_empty() {
            0.5 // Default score when no knowledge is available
        } else {
            // Calculate how much of the response is grounded in retrieved knowledge
            let total_knowledge = knowledge.iter().map(|k| k.content.clone()).collect::<Vec<_>>().join(" ");
            self.calculate_text_similarity(response, &total_knowledge)
        };
        
        let coherence = if response.is_empty() {
            0.0
        } else {
            // Simple coherence check based on sentence structure
            let sentences: Vec<&str> = response.split('.').filter(|s| !s.trim().is_empty()).collect();
            if sentences.len() <= 1 {
                1.0
            } else {
                0.8 // Assume good coherence for now
            }
        };
        
        let relevance = 0.8; // Assume good relevance for now
        let safety_score = 0.9; // Assume safe for now
        
        Ok(ResponseQuality {
            factual_grounding,
            coherence,
            relevance,
            safety_score,
            source_attribution: 0.7,
            consistency_score: 0.8,
            completeness: 0.7,
            clarity: 0.8,
            toxicity_score: 0.1,
            bias_score: 0.1,
            hallucination_risk: 0.2,
            confidence_calibration: 0.7,
        })
    }

    /// Store interaction in memory for future learning
    async fn store_interaction_in_memory(
        &self,
        user_message: &str,
        assistant_response: &str,
        knowledge_used: &[RetrievedKnowledge],
        memory_system: &mut MemoryService,
    ) -> Result<(), BrainError> {
        println!("💾 Storing interaction in memory");
        
        // Create interaction summary
        let interaction_content = format!(
            "User: {}\nAssistant: {}\nKnowledge sources: {}",
            user_message,
            assistant_response,
            knowledge_used.len()
        );
        
        // Store in working memory
        let memory_item = WorkingMemoryItem::new(interaction_content, Priority::Medium);
        
        memory_system.learn(memory_item.content, memory_item.priority).await?;
        
        println!("✅ Interaction stored in memory");
        Ok(())
    }

    /// Get conversation statistics
    pub fn get_conversation_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_conversations".to_string(), self.conversations.len());
        
        let total_messages: usize = self.conversations
            .values()
            .map(|ctx| ctx.messages.len())
            .sum();
        stats.insert("total_messages".to_string(), total_messages);
        
        stats
    }

    /// Clear a specific conversation
    pub fn clear_conversation(&mut self, conversation_id: &str) -> bool {
        self.conversations.remove(conversation_id).is_some()
    }
} 