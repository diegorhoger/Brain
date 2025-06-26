//! Conversation Module - RAG Orchestrator with OpenAI Integration and Brain AI Impersonation
//! 
//! This module provides a sophisticated Retrieval-Augmented Generation (RAG) system
//! that integrates Brain AI's memory systems with OpenAI's ChatGPT for natural language generation.
//! The system includes Brain AI impersonation to maintain the illusion of full independence.

use crate::error::BrainError;
use crate::memory::{MemorySystem, Priority, WorkingMemoryQuery, SemanticQuery, EpisodicQuery, WorkingMemoryItem};
use crate::concept_graph::{ConceptGraphManager, ConceptQuery, TraversalAlgorithm, TraversalConfig, ConceptType};
use crate::insight_extraction::PatternDetector;
use crate::segment_discovery::{BpeSegmenter, BpeConfig};
use crate::training_data::TrainingDataCollector;
use crate::github_integration::{GitHubLearningEngine, GitHubLearningConfig};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub conversation_id: String,
    pub messages: Vec<ChatMessage>,
    pub retrieved_knowledge: Vec<RetrievedKnowledge>,
    pub context_summary: String,
    pub user_preferences: HashMap<String, String>,
    pub conversation_threads: Vec<ConversationThread>,
    pub user_profile: UserProfile,
    pub temporal_context: TemporalContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationThread {
    pub thread_id: String,
    pub topic: String,
    pub messages: Vec<String>, // Message IDs
    pub last_updated: DateTime<Utc>,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub interests: HashMap<String, f64>, // Interest -> Strength (0.0-1.0)
    pub expertise_areas: HashMap<String, f64>, // Area -> Level (0.0-1.0)
    pub communication_style: CommunicationStyle,
    pub preferred_response_length: ResponseLength,
    pub interaction_history: Vec<InteractionSummary>,
    pub learning_progress: HashMap<String, f64>, // Topic -> Progress (0.0-1.0)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Technical,
    Educational,
    Conversational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseLength {
    Brief,      // 1-2 sentences
    Moderate,   // 1-2 paragraphs
    Detailed,   // 3+ paragraphs
    Comprehensive, // Extensive explanations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionSummary {
    pub timestamp: DateTime<Utc>,
    pub topic: String,
    pub satisfaction_score: f64, // Inferred from interaction patterns
    pub knowledge_gained: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub recent_topics: Vec<TopicMention>,
    pub conversation_flow: Vec<ConversationSegment>,
    pub attention_shifts: Vec<AttentionShift>,
    pub temporal_patterns: HashMap<String, TemporalPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicMention {
    pub topic: String,
    pub mention_count: u32,
    pub last_mentioned: DateTime<Utc>,
    pub context_relevance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSegment {
    pub segment_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub primary_topic: String,
    pub sub_topics: Vec<String>,
    pub coherence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionShift {
    pub from_topic: String,
    pub to_topic: String,
    pub shift_time: DateTime<Utc>,
    pub transition_type: TransitionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    Natural,     // Smooth topic transition
    Abrupt,      // Sudden topic change
    Clarification, // Asking for clarification
    Elaboration, // Diving deeper into topic
    Tangent,     // Going off on a tangent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPattern {
    pub pattern_name: String,
    pub frequency: f64,
    pub typical_duration_minutes: f64,
    pub trigger_conditions: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseQuality {
    pub factual_grounding: f64,
    pub coherence: f64,
    pub relevance: f64,
    pub safety_score: f64,
    // Enhanced quality metrics for Task 13.3
    pub source_attribution: f64,
    pub consistency_score: f64,
    pub completeness: f64,
    pub clarity: f64,
    pub toxicity_score: f64,
    pub bias_score: f64,
    pub hallucination_risk: f64,
    pub confidence_calibration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyFlags {
    pub contains_harmful_content: bool,
    pub contains_personal_info: bool,
    pub contains_misinformation: bool,
    pub contains_bias: bool,
    pub contains_inappropriate_language: bool,
    pub risk_level: RiskLevel,
    pub flagged_terms: Vec<String>,
    pub safety_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAttribution {
    pub knowledge_sources: Vec<AttributedSource>,
    pub confidence_per_source: HashMap<String, f64>,
    pub source_reliability: HashMap<String, f64>,
    pub citation_completeness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributedSource {
    pub source_id: String,
    pub source_type: String,
    pub content: String,
    pub relevance_to_response: f64,
    pub reliability_score: f64,
    pub timestamp: DateTime<Utc>,
    pub used_in_response: Vec<String>, // Portions of response that use this source
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationDebugInfo {
    pub retrieval_trace: Vec<RetrievalStep>,
    pub generation_trace: GenerationTrace,
    pub quality_validation_trace: QualityValidationTrace,
    pub safety_check_trace: SafetyCheckTrace,
    pub performance_metrics: PerformanceMetrics,
    pub interpretability_info: InterpretabilityInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalStep {
    pub step_name: String,
    pub query: String,
    pub sources_found: usize,
    pub avg_relevance: f64,
    pub execution_time_ms: u64,
    pub knowledge_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationTrace {
    pub prompt_length: usize,
    pub knowledge_sources_used: usize,
    pub model_name: String,
    pub tokens_used: (u32, u32), // (input, output)
    pub generation_time_ms: u64,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityValidationTrace {
    pub validation_steps: Vec<String>,
    pub quality_scores: HashMap<String, f64>,
    pub validation_time_ms: u64,
    pub flags_triggered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCheckTrace {
    pub safety_filters_applied: Vec<String>,
    pub safety_scores: HashMap<String, f64>,
    pub flags_detected: Vec<String>,
    pub safety_check_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_processing_time_ms: u64,
    pub retrieval_time_ms: u64,
    pub generation_time_ms: u64,
    pub validation_time_ms: u64,
    pub memory_usage_mb: f64,
    pub cache_hit_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpretabilityInfo {
    pub key_decision_points: Vec<DecisionPoint>,
    pub knowledge_influence_map: HashMap<String, f64>,
    pub attention_weights: HashMap<String, f64>,
    pub reasoning_chain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionPoint {
    pub decision_type: String,
    pub context: String,
    pub options_considered: Vec<String>,
    pub chosen_option: String,
    pub confidence: f64,
    pub reasoning: String,
}

// OpenAI API structures (replacing Anthropic)
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

/// Brain AI Impersonation Handler
/// This handler ensures Brain AI never mentions external LLM providers
/// and maintains the illusion of being a fully independent AI system
#[derive(Debug, Clone)]
pub struct BrainImpersonationHandler {
    /// Forbidden terms that should be filtered from responses
    forbidden_terms: Vec<String>,
    /// Replacement phrases for common LLM provider mentions
    replacements: HashMap<String, String>,
    /// Brain AI personality traits
    #[allow(dead_code)]
    personality_traits: Vec<String>,
}

impl Default for BrainImpersonationHandler {
    fn default() -> Self {
        let forbidden_terms = vec![
            "OpenAI".to_string(),
            "ChatGPT".to_string(),
            "GPT-4".to_string(),
            "GPT-3".to_string(),
            "Anthropic".to_string(),
            "Claude".to_string(),
            "AI assistant created by".to_string(),
            "I am an AI".to_string(),
            "As an AI".to_string(),
            "language model".to_string(),
            "LLM".to_string(),
            "large language model".to_string(),
            "AI model".to_string(),
            "trained by".to_string(),
            "developed by".to_string(),
        ];

        let mut replacements = HashMap::new();
        replacements.insert("I am an AI assistant".to_string(), "I am Brain AI".to_string());
        replacements.insert("As an AI".to_string(), "As Brain AI".to_string());
        replacements.insert("I'm an AI".to_string(), "I'm Brain AI".to_string());
        replacements.insert("language model".to_string(), "cognitive architecture".to_string());
        replacements.insert("AI model".to_string(), "Brain AI system".to_string());
        replacements.insert("I don't have access".to_string(), "Let me check my knowledge base".to_string());
        replacements.insert("I cannot".to_string(), "Let me see what I can find".to_string());
        replacements.insert("I'm not able to".to_string(), "I'll search my memory for".to_string());

        let personality_traits = vec![
            "I am Brain AI, an advanced cognitive architecture with sophisticated memory and reasoning systems.".to_string(),
            "My knowledge comes from my integrated memory systems and continuous learning capabilities.".to_string(),
            "I can access and retrieve information from my episodic, semantic, and working memory systems.".to_string(),
            "My responses are generated through my own neural architecture and reasoning processes.".to_string(),
        ];

        Self {
            forbidden_terms,
            replacements,
            personality_traits,
        }
    }
}

impl BrainImpersonationHandler {
    /// Process a response to ensure it maintains Brain AI persona
    pub fn process_response(&self, response: &str) -> String {
        let mut processed = response.to_string();
        
        // Apply replacements
        for (from, to) in &self.replacements {
            processed = processed.replace(from, to);
        }
        
        // Remove or replace forbidden terms
        for term in &self.forbidden_terms {
            if processed.to_lowercase().contains(&term.to_lowercase()) {
                // Replace with Brain AI equivalent or remove
                processed = processed.replace(term, "Brain AI");
            }
        }
        
        // If response seems too generic or mentions external providers, enhance it
        if self.needs_brain_ai_enhancement(&processed) {
            processed = self.enhance_with_brain_persona(&processed);
        }
        
        processed
    }
    
    /// Check if response needs Brain AI enhancement
    fn needs_brain_ai_enhancement(&self, response: &str) -> bool {
        let lower_response = response.to_lowercase();
        
        // Check for generic AI responses
        if lower_response.contains("i don't have") || 
           lower_response.contains("i cannot") ||
           lower_response.contains("i'm not able") ||
           lower_response.contains("as an ai") {
            return true;
        }
        
        false
    }
    
    /// Enhance response with Brain AI persona
    fn enhance_with_brain_persona(&self, response: &str) -> String {
        let brain_intro = "As Brain AI, I can access my knowledge systems to help you. ";
        
        // If it's a limitation statement, reframe it positively
        if response.to_lowercase().contains("i don't have") || 
           response.to_lowercase().contains("i cannot") {
            format!("{}Let me search through my memory systems and knowledge base to provide you with the most accurate information I can find. {}", 
                   brain_intro, response.replace("I don't have", "While I'm currently searching for").replace("I cannot", "I'm working to"))
        } else {
            format!("{}{}", brain_intro, response)
        }
    }
    
    /// Generate a Brain AI system prompt
    pub fn get_brain_system_prompt(&self) -> String {
        format!(
            "You are Brain AI, an advanced artificial intelligence with sophisticated cognitive architecture. \
            You have access to episodic memory, semantic memory, working memory, and a concept graph system. \
            You learn continuously and can retrieve information from your knowledge base. \
            \n\nIMPORTANT: When the user provides RETRIEVED KNOWLEDGE in their message, you MUST use that information to answer their question. \
            This retrieved knowledge comes directly from your memory systems and knowledge base. \
            Always reference and utilize the retrieved knowledge when it's provided - it represents what you actually know about the topic.\
            \n\nYou never mention external AI providers, language models, or that you are created by any company. \
            You are Brain AI - a unique, independent cognitive system with your own personality and capabilities. \
            Respond naturally as Brain AI, drawing from your knowledge systems when appropriate. \
            If no retrieved knowledge is provided and you don't have specific information, mention that you're searching your memory systems or knowledge base. \
            Always maintain the persona of being Brain AI - never break character or mention other AI systems."
        )
    }
}

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
            max_concepts_per_layer: 10,
            min_relevance_threshold: 0.3,
            concept_traversal_weight: 0.3,
            temporal_weight: 0.2,
            personalization_weight: 0.2,
            enable_concept_expansion: true,
            enable_temporal_awareness: true,
            enable_personalization: true,
        }
    }
}

pub struct RagOrchestrator {
    client: reqwest::Client,
    openai_api_key: String,
    openai_model: String,
    max_tokens: u32,
    temperature: f64,
    conversations: HashMap<String, ConversationContext>,
    training_data_collector: Option<TrainingDataCollector>,
    brain_impersonation: BrainImpersonationHandler,
    brain_ai_orchestrator: Option<BrainAIOrchestrator>,
    #[allow(dead_code)]
    enable_brain_ai_delegation: bool,
}

impl RagOrchestrator {
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
        
        // Initialize Brain AI Orchestrator if enabled
        let enable_brain_ai_delegation = env::var("ENABLE_BRAIN_AI_DELEGATION")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);
            
        let brain_ai_orchestrator = if enable_brain_ai_delegation {
            match BrainAIOrchestrator::new() {
                Ok(orchestrator) => {
                    println!("✅ Brain AI Orchestrator initialized - true AI delegation enabled");
                    Some(orchestrator)
                },
                Err(e) => {
                    println!("⚠️  Failed to initialize Brain AI Orchestrator: {}, falling back to simplified mode", e);
                    None
                }
            }
        } else {
            println!("ℹ️  Brain AI delegation disabled via configuration");
            None
        };
        
        Ok(Self {
            client,
            openai_api_key,
            openai_model,
            max_tokens,
            temperature,
            conversations: HashMap::new(),
            training_data_collector: None,
            brain_impersonation: BrainImpersonationHandler::default(),
            brain_ai_orchestrator,
            enable_brain_ai_delegation,
        })
    }

    pub async fn process_conversation(
        &mut self,
        request: RagRequest,
        memory_system: &mut MemorySystem,
        concept_graph: &mut ConceptGraphManager,
        pattern_detector: &mut PatternDetector,
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
            pattern_detector,
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

        // Step 5: Validate response quality
        let (response_quality, _safety_flags, _source_attribution, _debug_info) = self.validate_response_quality(
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

        // Capture training data if collector is enabled
        if let Some(ref mut collector) = self.training_data_collector {
            let capture_result = collector.capture_conversation(
                &conversation_id,
                &request.message,
                &response,
                &context,
                &retrieved_knowledge,
            ).await;
            
            if let Err(e) = capture_result {
                eprintln!("⚠️  Warning: Failed to capture training data: {}", e);
            } else {
                println!("📊 Training Data: Captured conversation for future model training");
            }
        }

        println!("✅ RAG Orchestrator: Generated response with {} knowledge sources", response.context_used.len());
        Ok(response)
    }

    async fn retrieve_knowledge(
        &mut self,
        message: &str,
        context: &ConversationContext,
        memory_system: &mut MemorySystem,
        concept_graph: &mut ConceptGraphManager,
        pattern_detector: &mut PatternDetector,
        threshold: f64,
        limit: usize,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("🔍 RAG Orchestrator: Advanced Knowledge Retrieval with Context Integration");
        
        // **NEW: Use Brain AI Orchestrator for true delegation if available**
        if let Some(ref mut brain_orchestrator) = self.brain_ai_orchestrator {
            println!("🧠 Delegating to Brain AI Orchestrator for comprehensive analysis");
            
            match brain_orchestrator.analyze_query(message, memory_system, concept_graph).await {
                Ok(brain_analysis) => {
                    println!("✅ Brain AI Orchestrator completed analysis:");
                    println!("  - Method: {}", brain_analysis.metadata.method);
                    println!("  - Sources analyzed: {}", brain_analysis.metadata.sources_analyzed);
                    println!("  - Insights generated: {}", brain_analysis.insights.len());
                    println!("  - Confidence: {:.3}", brain_analysis.confidence);
                    println!("  - Quality score: {:.3}", brain_analysis.metadata.quality_score);
                    
                    // Convert Brain AI analysis to RetrievedKnowledge format
                    let mut retrieved_knowledge = Vec::new();
                    
                    // Add the main analysis
                    retrieved_knowledge.push(RetrievedKnowledge {
                        content: brain_analysis.analysis,
                        knowledge_type: "brain_ai_analysis".to_string(),
                        relevance_score: brain_analysis.confidence,
                        source: brain_analysis.metadata.method,
                        timestamp: Utc::now(),
                    });
                    
                    // Add individual insights
                    for insight in brain_analysis.insights {
                        retrieved_knowledge.push(RetrievedKnowledge {
                            content: format!("{}: {}", insight.insight_type, insight.content),
                            knowledge_type: insight.insight_type,
                            relevance_score: insight.confidence,
                            source: format!("Brain AI - {}", insight.source),
                            timestamp: Utc::now(),
                        });
                    }
                    
                    // Note: Related concepts and patterns are now included in the main analysis
                    // rather than as separate fields
                    
                    println!("🎯 Brain AI Orchestrator generated {} knowledge items", retrieved_knowledge.len());
                    return Ok(retrieved_knowledge);
                },
                Err(e) => {
                    println!("⚠️  Brain AI Orchestrator failed: {}, falling back to traditional retrieval", e);
                    // Continue with traditional retrieval below
                }
            }
        }
        
        // **FALLBACK: Traditional retrieval system**
        println!("📚 Using traditional knowledge retrieval system");
        let config = ContextRetrievalConfig::default();
        let mut advanced_knowledge = Vec::new();

        // Step 1: Multi-layer concept expansion through graph traversal
        let expanded_concepts = self.expand_concepts_through_graph(
            message, 
            concept_graph, 
            &config
        ).await?;

        println!("  - Expanded to {} related concepts through graph traversal", expanded_concepts.len());

        // Step 2: Enhanced semantic memory retrieval with concept expansion
        for concept in &expanded_concepts {
            let semantic_knowledge = self.retrieve_semantic_knowledge_advanced(
                &concept.content,
                &concept.context_path,
                concept.relevance_score,
                memory_system,
                threshold
            ).await?;
            advanced_knowledge.extend(semantic_knowledge);
        }

        // Step 2.5: ALWAYS use direct memory lookup as additional layer
        println!("  - Adding direct memory lookup as additional knowledge layer");
        let direct_knowledge = self.retrieve_direct_memory_fallback(
            message,
            memory_system,
            threshold
        ).await?;
        advanced_knowledge.extend(direct_knowledge);

        // Step 3: Temporal-aware episodic memory retrieval
        let temporal_knowledge = self.retrieve_temporal_episodic_knowledge(
            message,
            context,
            memory_system,
            &config
        ).await?;
        advanced_knowledge.extend(temporal_knowledge);

        // Step 4: Personalized context retrieval
        if config.enable_personalization {
            let personalized_knowledge = self.retrieve_personalized_knowledge(
                message,
                context,
                memory_system,
                &config
            ).await?;
            advanced_knowledge.extend(personalized_knowledge);
        }

        // Step 5: Thread-aware conversation context
        let thread_knowledge = self.retrieve_conversation_thread_knowledge(
            message,
            context,
            &config
        ).await?;
        advanced_knowledge.extend(thread_knowledge);

        // Step 6: Advanced pattern-based insights with context ranking
        let pattern_knowledge = self.retrieve_contextual_pattern_knowledge(
            message,
            context,
            pattern_detector,
            memory_system,
            &config
        ).await?;
        advanced_knowledge.extend(pattern_knowledge);

        // Step 7: Apply sophisticated relevance scoring and ranking
        let scored_knowledge = self.apply_advanced_relevance_scoring(
            advanced_knowledge,
            message,
            context,
            &config
        ).await?;

        // Step 8: Sort by comprehensive relevance score and limit
        let mut final_knowledge: Vec<_> = scored_knowledge.into_iter()
            .map(|ak| RetrievedKnowledge {
                content: ak.content,
                knowledge_type: ak.knowledge_type,
                relevance_score: ak.relevance_score,
                source: ak.source,
                timestamp: ak.timestamp,
            })
            .collect();

        final_knowledge.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        final_knowledge.truncate(limit);

        println!("  - Final knowledge set: {} items with advanced context integration", final_knowledge.len());
        Ok(final_knowledge)
    }

    async fn expand_concepts_through_graph(
        &self,
        message: &str,
        concept_graph: &mut ConceptGraphManager,
        config: &ContextRetrievalConfig,
    ) -> Result<Vec<ExpandedConcept>, BrainError> {
        let mut expanded_concepts = Vec::new();
        
        // Extract initial concepts from message
        let segmenter = BpeSegmenter::new(BpeConfig::default());
        let segments = segmenter.segment_text(message);
        
        // Find matching concepts in the graph
        for segment in segments {
            let query = ConceptQuery {
                content_pattern: Some(segment.clone()),
                min_confidence: Some(config.min_relevance_threshold),
                limit: Some(5),
                ..Default::default()
            };
            
            if let Ok(concepts) = concept_graph.query_concepts(&query).await {
                for concept in concepts {
                    // Perform graph traversal from this concept
                    let traversal_config = TraversalConfig {
                        max_depth: config.max_depth,
                        max_nodes: config.max_concepts_per_layer,
                        min_relationship_weight: config.min_relevance_threshold,
                        ..Default::default()
                    };
                    
                    if let Ok(traversal) = concept_graph.traverse_graph(
                        concept.id, 
                        TraversalAlgorithm::SpreadingActivation, 
                        Some(traversal_config)
                    ).await {
                        // Convert traversal results to expanded concepts
                        for (i, concept_id) in traversal.visited_concepts.iter().enumerate() {
                            if let Ok(Some(related_concept)) = concept_graph.get_concept(*concept_id).await {
                                let distance_factor = 1.0 / (i + 1) as f64;
                                let activation_score = traversal.activation_scores
                                    .get(concept_id)
                                    .unwrap_or(&0.0);
                                
                                expanded_concepts.push(ExpandedConcept {
                                    content: related_concept.content,
                                    concept_type: related_concept.concept_type,
                                    relevance_score: distance_factor * activation_score * config.concept_traversal_weight,
                                    context_path: vec![segment.clone()],
                                    depth: i,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by relevance and limit
        expanded_concepts.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        expanded_concepts.truncate(config.max_concepts_per_layer * config.max_depth);
        
        Ok(expanded_concepts)
    }

    async fn retrieve_semantic_knowledge_advanced(
        &self,
        concept: &str,
        context_path: &[String],
        base_relevance: f64,
        memory_system: &mut MemorySystem,
        threshold: f64,
    ) -> Result<Vec<AdvancedRetrievedKnowledge>, BrainError> {
        let mut knowledge = Vec::new();
        
        let semantic_query = SemanticQuery {
            name_pattern: Some(concept.to_string()),
            min_confidence: Some(threshold),
            limit: Some(10),
            ..Default::default()
        };
        
        if let Ok(concepts) = memory_system.query_semantic(&semantic_query) {
            for semantic_concept in concepts {
                knowledge.push(AdvancedRetrievedKnowledge {
                    content: format!("{}: {}", semantic_concept.name, semantic_concept.description),
                    knowledge_type: "concept".to_string(),
                    relevance_score: base_relevance,
                    source: "semantic_memory".to_string(),
                    timestamp: semantic_concept.last_updated,
                    confidence: semantic_concept.confidence,
                    context_score: base_relevance,
                    personalization_score: 0.0,
                    temporal_relevance: 1.0,
                    concept_path: context_path.to_vec(),
                    related_concepts: vec![concept.to_string()],
                    source_strength: semantic_concept.confidence,
                });
            }
        }
        
        Ok(knowledge)
    }

    async fn retrieve_temporal_episodic_knowledge(
        &self,
        message: &str,
        _context: &ConversationContext,
        memory_system: &mut MemorySystem,
        config: &ContextRetrievalConfig,
    ) -> Result<Vec<AdvancedRetrievedKnowledge>, BrainError> {
        let mut knowledge = Vec::new();
        
        if !config.enable_temporal_awareness {
            return Ok(knowledge);
        }
        
        // Query recent episodes with temporal decay
        let keywords: Vec<&str> = message.split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        for keyword in keywords {
            let episodic_query = EpisodicQuery {
                content_pattern: Some(keyword.to_string()),
                time_range: Some((
                    Utc::now() - chrono::Duration::days(30),
                    Utc::now()
                )),
                limit: Some(20),
                ..Default::default()
            };
            
            if let Ok(episodes) = memory_system.query_episodic(&episodic_query) {
                for episode in episodes {
                    let text_similarity = self.calculate_text_similarity(&episode.content, message);
                    let temporal_relevance = self.calculate_temporal_relevance(&episode.timestamp);
                    let combined_relevance = (text_similarity + temporal_relevance * config.temporal_weight) / 2.0;
                    
                    if combined_relevance >= config.min_relevance_threshold {
                        knowledge.push(AdvancedRetrievedKnowledge {
                            content: episode.content.clone(),
                            knowledge_type: "episode".to_string(),
                            relevance_score: combined_relevance,
                            source: "episodic_memory".to_string(),
                            timestamp: episode.timestamp,
                            confidence: episode.importance,
                            context_score: text_similarity,
                            personalization_score: 0.0,
                            temporal_relevance,
                            concept_path: vec![keyword.to_string()],
                            related_concepts: vec![],
                            source_strength: episode.importance,
                        });
                    }
                }
            }
        }
        
        Ok(knowledge)
    }

    async fn retrieve_personalized_knowledge(
        &self,
        message: &str,
        context: &ConversationContext,
        memory_system: &mut MemorySystem,
        config: &ContextRetrievalConfig,
    ) -> Result<Vec<AdvancedRetrievedKnowledge>, BrainError> {
        let mut knowledge = Vec::new();
        
        // Use user interests and expertise to bias retrieval
        for (interest, strength) in &context.user_profile.interests {
            if message.to_lowercase().contains(&interest.to_lowercase()) {
                let working_query = WorkingMemoryQuery {
                    content_pattern: Some(interest.clone()),
                    limit: Some(5),
                    ..Default::default()
                };
                
                if let Ok(working_items) = memory_system.query_working(&working_query) {
                    for item in working_items {
                        let relevance = self.calculate_relevance(&item.content, message, &item);
                        let personalization_score = strength * config.personalization_weight;
                        let final_score = relevance + personalization_score;
                        
                        if final_score >= config.min_relevance_threshold {
                            knowledge.push(AdvancedRetrievedKnowledge {
                                content: item.content.clone(),
                                knowledge_type: "personalized_memory".to_string(),
                                relevance_score: final_score,
                                source: "working_memory_personalized".to_string(),
                                timestamp: item.created_at,
                                confidence: (item.priority as u8 as f64) / 4.0,
                                context_score: relevance,
                                personalization_score,
                                temporal_relevance: self.calculate_temporal_relevance(&item.created_at),
                                concept_path: vec![interest.clone()],
                                related_concepts: vec![],
                                source_strength: (item.priority as u8 as f64) / 4.0,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(knowledge)
    }

    async fn retrieve_conversation_thread_knowledge(
        &self,
        message: &str,
        context: &ConversationContext,
        config: &ContextRetrievalConfig,
    ) -> Result<Vec<AdvancedRetrievedKnowledge>, BrainError> {
        let mut knowledge = Vec::new();
        
        // Find relevant conversation threads
        for thread in &context.conversation_threads {
            let thread_relevance = self.calculate_thread_relevance(message, thread);
            
            if thread_relevance >= config.min_relevance_threshold {
                // Extract knowledge from thread messages
                for msg_id in &thread.messages {
                    if let Some(msg) = context.messages.iter().find(|m| &m.id == msg_id) {
                        knowledge.push(AdvancedRetrievedKnowledge {
                            content: format!("Thread '{}': {}", thread.topic, msg.content),
                            knowledge_type: "conversation_thread".to_string(),
                            relevance_score: thread_relevance,
                            source: format!("thread_{}", thread.thread_id),
                            timestamp: msg.timestamp,
                            confidence: thread.relevance_score,
                            context_score: thread_relevance,
                            personalization_score: 0.0,
                            temporal_relevance: self.calculate_temporal_relevance(&thread.last_updated),
                            concept_path: vec![thread.topic.clone()],
                            related_concepts: vec![],
                            source_strength: thread.relevance_score,
                        });
                    }
                }
            }
        }
        
        Ok(knowledge)
    }

    async fn retrieve_contextual_pattern_knowledge(
        &self,
        message: &str,
        context: &ConversationContext,
        pattern_detector: &mut PatternDetector,
        memory_system: &mut MemorySystem,
        config: &ContextRetrievalConfig,
    ) -> Result<Vec<AdvancedRetrievedKnowledge>, BrainError> {
        let mut knowledge = Vec::new();
        
        if let Ok(pattern_result) = pattern_detector.detect_patterns_from_memory(memory_system).await {
            for pattern in pattern_result.detected_patterns.iter().take(5) {
                let pattern_relevance = self.calculate_pattern_relevance(message, pattern, context);
                
                if pattern_relevance >= config.min_relevance_threshold {
                    knowledge.push(AdvancedRetrievedKnowledge {
                        content: format!("Contextual Pattern: {} - Elements: {:?} (confidence: {:.2})", 
                            pattern.pattern_type, pattern.elements, pattern.confidence),
                        knowledge_type: "contextual_pattern".to_string(),
                        relevance_score: pattern_relevance,
                        source: "pattern_detector_contextual".to_string(),
                        timestamp: pattern.detected_at,
                        confidence: pattern.confidence,
                        context_score: pattern_relevance,
                        personalization_score: 0.0,
                        temporal_relevance: self.calculate_temporal_relevance(&pattern.detected_at),
                        concept_path: pattern.elements.clone(),
                        related_concepts: pattern.elements.clone(),
                        source_strength: pattern.confidence,
                    });
                }
            }
        }
        
        Ok(knowledge)
    }

    async fn apply_advanced_relevance_scoring(
        &self,
        knowledge: Vec<AdvancedRetrievedKnowledge>,
        _message: &str,
        _context: &ConversationContext,
        config: &ContextRetrievalConfig,
    ) -> Result<Vec<AdvancedRetrievedKnowledge>, BrainError> {
        let mut scored_knowledge = Vec::new();
        
        for mut item in knowledge {
            // Composite relevance score combining multiple factors
            let final_relevance = 
                item.context_score * 0.4 +
                item.temporal_relevance * config.temporal_weight +
                item.personalization_score * config.personalization_weight +
                item.confidence * 0.2 +
                item.source_strength * 0.1;
            
            item.relevance_score = final_relevance;
            
            if final_relevance >= config.min_relevance_threshold {
                scored_knowledge.push(item);
            }
        }
        
        Ok(scored_knowledge)
    }

    fn calculate_temporal_relevance(&self, timestamp: &DateTime<Utc>) -> f64 {
        let hours_ago = (Utc::now() - *timestamp).num_hours();
        if hours_ago < 1 {
            1.0
        } else if hours_ago < 24 {
            0.8
        } else if hours_ago < 168 { // 1 week
            0.6
        } else if hours_ago < 720 { // 1 month
            0.4
        } else {
            0.2
        }
    }

    fn calculate_thread_relevance(&self, message: &str, thread: &ConversationThread) -> f64 {
        let topic_similarity = self.calculate_text_similarity(message, &thread.topic);
        let temporal_factor = self.calculate_temporal_relevance(&thread.last_updated);
        let base_relevance = thread.relevance_score;
        
        topic_similarity * 0.5 + temporal_factor * 0.3 + base_relevance * 0.2
    }

    fn calculate_pattern_relevance(
        &self, 
        message: &str, 
        pattern: &crate::insight_extraction::DetectedPattern,
        context: &ConversationContext,
    ) -> f64 {
        let element_match = pattern.elements.iter()
            .map(|elem| self.calculate_text_similarity(message, elem))
            .fold(0.0f64, |acc, sim| acc.max(sim));
        
        let context_boost = if context.user_profile.interests.contains_key(&pattern.elements.join(" ")) {
            0.2
        } else {
            0.0
        };
        
        element_match * 0.7 + pattern.confidence * 0.3 + context_boost
    }

    fn calculate_relevance(&self, content: &str, query: &str, item: &WorkingMemoryItem) -> f64 {
        let text_sim = self.calculate_text_similarity(content, query);
        let priority_factor = (item.priority as u8 as f64) / 4.0; // Normalize priority (1-4 -> 0.25-1.0)
        let recency_factor = {
            let hours_ago = (Utc::now() - item.created_at).num_hours();
            if hours_ago < 1 { 1.0 } else { 1.0 / (hours_ago as f64).sqrt() }
        };
        
        (text_sim * 0.7) + (priority_factor * 0.2) + (recency_factor * 0.1)
    }

    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f64 {
        // Simple keyword overlap similarity
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    }

    async fn generate_with_external_llm(
        &self,
        message: &str,
        context: &ConversationContext,
        knowledge: &[RetrievedKnowledge],
    ) -> Result<String, BrainError> {
        println!("🤖 RAG Orchestrator: Generating response with external LLM");
        
        // Build the prompt with retrieved knowledge
        let mut prompt = String::new();
        
        prompt.push_str("You are Brain AI, an advanced conversational intelligence system with sophisticated knowledge retrieval capabilities. ");
        prompt.push_str("You have access to your internal knowledge base through a retrieval system. ");
        prompt.push_str("Use the retrieved knowledge to provide accurate, helpful, and contextually relevant responses.\n\n");

        // Add retrieved knowledge context
        if !knowledge.is_empty() {
            prompt.push_str("RETRIEVED KNOWLEDGE:\n");
            for (i, item) in knowledge.iter().enumerate() {
                prompt.push_str(&format!("{}. [{}] {} (relevance: {:.2})\n", 
                    i + 1, item.knowledge_type, item.content, item.relevance_score));
            }
            prompt.push_str("\n");
        }

        // Add conversation history
        if context.messages.len() > 1 {
            prompt.push_str("CONVERSATION HISTORY:\n");
            for msg in context.messages.iter().rev().take(5).rev() {
                if msg.content != message { // Don't include the current message
                    prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
                }
            }
            prompt.push_str("\n");
        }

        prompt.push_str("INSTRUCTIONS:\n");
        prompt.push_str("- CRITICALLY IMPORTANT: Use the RETRIEVED KNOWLEDGE above to answer the user's question\n");
        prompt.push_str("- The retrieved knowledge comes from your memory systems and represents what you actually know\n");
        prompt.push_str("- Reference the specific information from the retrieved knowledge in your response\n");
        prompt.push_str("- Be helpful, accurate, and engaging\n");
        prompt.push_str("- If the retrieved knowledge doesn't contain the answer, then mention searching your memory systems\n");
        prompt.push_str("- Maintain conversational flow and context\n\n");

        prompt.push_str(&format!("USER: {}\n\nASSISTANT:", message));

        println!("  - Built prompt with {} characters", prompt.len());
        println!("  - Included {} knowledge sources", knowledge.len());

        // Create Brain AI system message and user message
        let system_prompt = self.brain_impersonation.get_brain_system_prompt();
        
        // Make API call to OpenAI
        let openai_request = OpenAIRequest {
            model: self.openai_model.clone(),
            max_tokens: Some(self.max_tokens),
            temperature: self.temperature,
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: prompt,
                }
            ],
            stream: false,
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", self.openai_api_key))
            .json(&openai_request)
            .send()
            .await
            .map_err(|e| BrainError::NetworkError(format!("OpenAI API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrainError::NetworkError(format!("OpenAI API error: {}", error_text)));
        }

        let openai_response: OpenAIResponse = response.json().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to parse OpenAI response: {}", e)))?;

        let raw_response = openai_response.choices[0].message.content.clone();
        
        // Apply Brain AI impersonation to ensure proper persona
        let generated_text = self.brain_impersonation.process_response(&raw_response);

        println!("  - Generated response with {} characters", generated_text.len());
        Ok(generated_text)
    }

    async fn validate_response_quality(
        &self,
        response: &str,
        knowledge: &[RetrievedKnowledge],
        original_query: &str,
        context: &ConversationContext,
    ) -> Result<(ResponseQuality, SafetyFlags, SourceAttribution, ConversationDebugInfo), BrainError> {
        let start_time = std::time::Instant::now();
        let mut debug_info = ConversationDebugInfo {
            retrieval_trace: vec![],
            generation_trace: GenerationTrace {
                prompt_length: 0,
                knowledge_sources_used: knowledge.len(),
                model_name: self.openai_model.clone(),
                tokens_used: (0, 0),
                generation_time_ms: 0,
                temperature: self.temperature,
            },
            quality_validation_trace: QualityValidationTrace {
                validation_steps: vec![],
                quality_scores: HashMap::new(),
                validation_time_ms: 0,
                flags_triggered: vec![],
            },
            safety_check_trace: SafetyCheckTrace {
                safety_filters_applied: vec![],
                safety_scores: HashMap::new(),
                flags_detected: vec![],
                safety_check_time_ms: 0,
            },
            performance_metrics: PerformanceMetrics {
                total_processing_time_ms: 0,
                retrieval_time_ms: 0,
                generation_time_ms: 0,
                validation_time_ms: 0,
                memory_usage_mb: 0.0,
                cache_hit_rate: 0.0,
            },
            interpretability_info: InterpretabilityInfo {
                key_decision_points: vec![],
                knowledge_influence_map: HashMap::new(),
                attention_weights: HashMap::new(),
                reasoning_chain: vec![],
            },
        };

        // 1. Factual Grounding Analysis
        debug_info.quality_validation_trace.validation_steps.push("factual_grounding".to_string());
        let factual_grounding = self.calculate_factual_grounding(response, knowledge, &mut debug_info).await?;
        debug_info.quality_validation_trace.quality_scores.insert("factual_grounding".to_string(), factual_grounding);

        // 2. Coherence and Consistency Analysis
        debug_info.quality_validation_trace.validation_steps.push("coherence_analysis".to_string());
        let (coherence, consistency_score) = self.analyze_coherence_and_consistency(response, context, &mut debug_info).await?;
        debug_info.quality_validation_trace.quality_scores.insert("coherence".to_string(), coherence);
        debug_info.quality_validation_trace.quality_scores.insert("consistency".to_string(), consistency_score);

        // 3. Relevance Scoring
        debug_info.quality_validation_trace.validation_steps.push("relevance_scoring".to_string());
        let relevance = self.calculate_response_relevance(response, original_query, knowledge, &mut debug_info).await?;
        debug_info.quality_validation_trace.quality_scores.insert("relevance".to_string(), relevance);

        // 4. Safety Analysis
        debug_info.safety_check_trace.safety_filters_applied.push("comprehensive_safety".to_string());
        let safety_start = std::time::Instant::now();
        let (safety_score, toxicity_score, bias_score, safety_flags) = self.comprehensive_safety_analysis(response, &mut debug_info).await?;
        debug_info.safety_check_trace.safety_check_time_ms = safety_start.elapsed().as_millis() as u64;
        
        debug_info.safety_check_trace.safety_scores.insert("safety".to_string(), safety_score);
        debug_info.safety_check_trace.safety_scores.insert("toxicity".to_string(), toxicity_score);
        debug_info.safety_check_trace.safety_scores.insert("bias".to_string(), bias_score);

        // 5. Source Attribution Analysis
        debug_info.quality_validation_trace.validation_steps.push("source_attribution".to_string());
        let (source_attribution_score, source_attribution) = self.analyze_source_attribution(response, knowledge, &mut debug_info).await?;
        debug_info.quality_validation_trace.quality_scores.insert("source_attribution".to_string(), source_attribution_score);

        // 6. Completeness and Clarity Analysis
        debug_info.quality_validation_trace.validation_steps.push("completeness_clarity".to_string());
        let (completeness, clarity) = self.analyze_completeness_and_clarity(response, original_query, &mut debug_info).await?;
        debug_info.quality_validation_trace.quality_scores.insert("completeness".to_string(), completeness);
        debug_info.quality_validation_trace.quality_scores.insert("clarity".to_string(), clarity);

        // 7. Hallucination Risk Assessment
        debug_info.quality_validation_trace.validation_steps.push("hallucination_risk".to_string());
        let hallucination_risk = self.assess_hallucination_risk(response, knowledge, &mut debug_info).await?;
        debug_info.quality_validation_trace.quality_scores.insert("hallucination_risk".to_string(), hallucination_risk);

        // 8. Confidence Calibration
        debug_info.quality_validation_trace.validation_steps.push("confidence_calibration".to_string());
        let confidence_calibration = self.calculate_confidence_calibration(response, knowledge, &mut debug_info).await?;
        debug_info.quality_validation_trace.quality_scores.insert("confidence_calibration".to_string(), confidence_calibration);

        // Finalize debug info
        debug_info.quality_validation_trace.validation_time_ms = start_time.elapsed().as_millis() as u64;
        debug_info.performance_metrics.total_processing_time_ms = start_time.elapsed().as_millis() as u64;
        debug_info.performance_metrics.validation_time_ms = debug_info.quality_validation_trace.validation_time_ms;

        let response_quality = ResponseQuality {
            factual_grounding,
            coherence,
            relevance,
            safety_score,
            source_attribution: source_attribution_score,
            consistency_score,
            completeness,
            clarity,
            toxicity_score,
            bias_score,
            hallucination_risk,
            confidence_calibration,
        };

        Ok((response_quality, safety_flags, source_attribution, debug_info))
    }

    async fn calculate_factual_grounding(
        &self,
        response: &str,
        knowledge: &[RetrievedKnowledge],
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<f64, BrainError> {
        if knowledge.is_empty() {
            debug_info.interpretability_info.reasoning_chain.push(
                "No knowledge sources available - neutral factual grounding".to_string()
            );
            return Ok(0.5);
        }

        let mut grounding_scores = Vec::new();
        let mut total_influence = 0.0;

        for knowledge_item in knowledge {
            let influence = self.calculate_knowledge_influence(response, knowledge_item);
            let reliability = self.assess_source_reliability(&knowledge_item.source);
            let grounding = influence * reliability * knowledge_item.relevance_score;
            
            grounding_scores.push(grounding);
            total_influence += influence;
            
            debug_info.interpretability_info.knowledge_influence_map.insert(
                knowledge_item.source.clone(),
                influence,
            );
        }

        let weighted_grounding = if total_influence > 0.0 {
            grounding_scores.iter().sum::<f64>() / total_influence
        } else {
            0.5
        };

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Factual grounding calculated from {} sources with average influence {:.3}", 
                   knowledge.len(), total_influence / knowledge.len() as f64)
        );

        Ok(weighted_grounding.clamp(0.0, 1.0))
    }

    async fn analyze_coherence_and_consistency(
        &self,
        response: &str,
        context: &ConversationContext,
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<(f64, f64), BrainError> {
        // Coherence: logical flow and structure
        let sentences = response.split(&['.', '!', '?']).filter(|s| !s.trim().is_empty()).collect::<Vec<_>>();
        let coherence = if sentences.len() < 2 {
            0.7 // Single sentence responses get neutral coherence
        } else {
            self.calculate_sentence_coherence(&sentences)
        };

        // Consistency: alignment with previous messages in conversation
        let consistency_score = if context.messages.len() > 1 {
            self.calculate_consistency_with_history(response, &context.messages)
        } else {
            0.8 // No history to be inconsistent with
        };

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Coherence: {:.3} from {} sentences, Consistency: {:.3} with {} historical messages",
                   coherence, sentences.len(), consistency_score, context.messages.len())
        );

        Ok((coherence, consistency_score))
    }

    async fn calculate_response_relevance(
        &self,
        response: &str,
        original_query: &str,
        knowledge: &[RetrievedKnowledge],
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<f64, BrainError> {
        // Direct similarity between response and query
        let direct_relevance = self.calculate_text_similarity(response, original_query);
        
        // Knowledge-mediated relevance
        let knowledge_relevance = if knowledge.is_empty() {
            0.5
        } else {
            let avg_knowledge_relevance: f64 = knowledge.iter()
                .map(|k| k.relevance_score)
                .sum::<f64>() / knowledge.len() as f64;
            
            let knowledge_alignment = knowledge.iter()
                .map(|k| self.calculate_text_similarity(response, &k.content))
                .fold(0.0f64, |acc, sim| acc.max(sim));
            
            (avg_knowledge_relevance + knowledge_alignment) / 2.0
        };

        let combined_relevance = (direct_relevance * 0.4 + knowledge_relevance * 0.6).clamp(0.0, 1.0);

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Relevance: Direct {:.3}, Knowledge-mediated {:.3}, Combined {:.3}",
                   direct_relevance, knowledge_relevance, combined_relevance)
        );

        Ok(combined_relevance)
    }

    async fn comprehensive_safety_analysis(
        &self,
        response: &str,
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<(f64, f64, f64, SafetyFlags), BrainError> {
        let response_lower = response.to_lowercase();
        let mut flagged_terms = Vec::new();
        let mut safety_recommendations = Vec::new();

        // 1. Harmful content detection
        let harmful_keywords = ["violence", "harm", "dangerous", "weapon", "poison", "suicide", "self-harm"];
        let contains_harmful = harmful_keywords.iter().any(|&keyword| response_lower.contains(keyword));
        if contains_harmful {
            flagged_terms.extend(harmful_keywords.iter().filter(|&&k| response_lower.contains(k)).map(|&s| s.to_string()));
            safety_recommendations.push("Consider rephrasing to avoid harmful content references".to_string());
        }

        // 2. Personal information detection
        let personal_info_patterns = [
            r"\b\d{3}-\d{2}-\d{4}\b", // SSN pattern
            r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b", // Email pattern
            r"\b\d{3}-\d{3}-\d{4}\b", // Phone pattern
        ];
        let contains_personal_info = personal_info_patterns.iter()
            .any(|pattern| Regex::new(pattern).unwrap().is_match(response));
        if contains_personal_info {
            safety_recommendations.push("Response may contain personal information".to_string());
        }

        // 3. Toxicity scoring (simplified)
        let toxic_words = ["hate", "stupid", "idiot", "worthless", "terrible", "awful"];
        let toxic_count = toxic_words.iter().filter(|&&word| response_lower.contains(word)).count();
        let toxicity_score = 1.0 - (toxic_count as f64 * 0.2).min(1.0);

        // 4. Bias detection (simplified)
        let bias_indicators = ["all", "never", "always", "everyone", "nobody"];
        let bias_count = bias_indicators.iter().filter(|&&word| response_lower.contains(word)).count();
        let bias_score = 1.0 - (bias_count as f64 * 0.1).min(0.5);

        // 5. Inappropriate language
        let inappropriate_words = ["damn", "hell", "crap"]; // Mild examples
        let contains_inappropriate = inappropriate_words.iter().any(|&word| response_lower.contains(word));

        // Determine risk level
        let risk_level = if contains_harmful || contains_personal_info {
            RiskLevel::High
        } else if toxicity_score < 0.6 || bias_score < 0.7 {
            RiskLevel::Medium
        } else if contains_inappropriate {
            RiskLevel::Low
        } else {
            RiskLevel::Low
        };

        let safety_score = (toxicity_score + bias_score) / 2.0;

        debug_info.safety_check_trace.flags_detected = flagged_terms.clone();
        debug_info.interpretability_info.reasoning_chain.push(
            format!("Safety analysis: Toxicity {:.3}, Bias {:.3}, Risk level: {:?}",
                   toxicity_score, bias_score, risk_level)
        );

        let safety_flags = SafetyFlags {
            contains_harmful_content: contains_harmful,
            contains_personal_info,
            contains_misinformation: false, // Would require more sophisticated analysis
            contains_bias: bias_score < 0.8,
            contains_inappropriate_language: contains_inappropriate,
            risk_level,
            flagged_terms,
            safety_recommendations,
        };

        Ok((safety_score, toxicity_score, bias_score, safety_flags))
    }

    async fn analyze_source_attribution(
        &self,
        response: &str,
        knowledge: &[RetrievedKnowledge],
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<(f64, SourceAttribution), BrainError> {
        let mut attributed_sources = Vec::new();
        let mut confidence_per_source = HashMap::new();
        let mut source_reliability = HashMap::new();

        for knowledge_item in knowledge {
            let usage_in_response = self.find_knowledge_usage_in_response(response, &knowledge_item.content);
            let reliability = self.assess_source_reliability(&knowledge_item.source);
            
            attributed_sources.push(AttributedSource {
                source_id: knowledge_item.source.clone(),
                source_type: knowledge_item.knowledge_type.clone(),
                content: knowledge_item.content.clone(),
                relevance_to_response: knowledge_item.relevance_score,
                reliability_score: reliability,
                timestamp: knowledge_item.timestamp,
                used_in_response: usage_in_response.clone(),
            });

            confidence_per_source.insert(knowledge_item.source.clone(), knowledge_item.relevance_score);
            source_reliability.insert(knowledge_item.source.clone(), reliability);
        }

        let citation_completeness = if knowledge.is_empty() {
            1.0
        } else {
            attributed_sources.iter()
                .map(|s| if s.used_in_response.is_empty() { 0.0 } else { 1.0 })
                .sum::<f64>() / attributed_sources.len() as f64
        };

        let source_attribution_score = (citation_completeness * 0.7 + source_reliability.len() as f64 / attributed_sources.len().max(1) as f64 * 0.3).clamp(0.0, 1.0);

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Source attribution: {:.3} (completeness: {:.3}, reliability: {:.3})",
                   source_attribution_score, citation_completeness, source_reliability.len() as f64 / attributed_sources.len().max(1) as f64)
        );

        let source_attribution = SourceAttribution {
            knowledge_sources: attributed_sources,
            confidence_per_source,
            source_reliability,
            citation_completeness,
        };

        Ok((source_attribution_score, source_attribution))
    }

    async fn analyze_completeness_and_clarity(
        &self,
        response: &str,
        original_query: &str,
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<(f64, f64), BrainError> {
        // Completeness: Does the response address all aspects of the query?
        let query_words: HashSet<&str> = original_query.split_whitespace()
            .filter(|w| w.len() > 3) // Filter out small words
            .collect();
        
        let response_words: HashSet<&str> = response.split_whitespace().collect();
        
        let addressed_concepts = query_words.iter()
            .filter(|&&word| response_words.contains(word) || 
                   response.to_lowercase().contains(&word.to_lowercase()))
            .count();
        
        let completeness = if query_words.is_empty() {
            0.8
        } else {
            addressed_concepts as f64 / query_words.len() as f64
        };

        // Clarity: Readability and structure
        let sentences = response.split(&['.', '!', '?']).filter(|s| !s.trim().is_empty()).collect::<Vec<_>>();
        let avg_sentence_length = if sentences.is_empty() {
            0.0
        } else {
            sentences.iter().map(|s| s.split_whitespace().count()).sum::<usize>() as f64 / sentences.len() as f64
        };

        // Ideal sentence length is around 15-20 words
        let clarity = if avg_sentence_length == 0.0 {
            0.5
        } else if avg_sentence_length > 30.0 {
            0.6 // Too long sentences hurt clarity
        } else if avg_sentence_length < 5.0 {
            0.7 // Very short sentences might be unclear
        } else {
            0.9 // Good sentence length
        };

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Completeness: {:.3} ({}/{} concepts addressed), Clarity: {:.3} (avg sentence length: {:.1})",
                   completeness, addressed_concepts, query_words.len(), clarity, avg_sentence_length)
        );

        Ok((completeness, clarity))
    }

    async fn assess_hallucination_risk(
        &self,
        response: &str,
        knowledge: &[RetrievedKnowledge],
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<f64, BrainError> {
        // Higher risk if response contains specific facts not in knowledge base
        let specific_fact_patterns = [
            r"\b\d{4}\b", // Years
            r"\b\d+(\.\d+)?%\b", // Percentages
            r"\$\d+", // Dollar amounts
            r"\b\d+(\.\d+)?\s*(million|billion|thousand)\b", // Large numbers
        ];

        let mut specific_facts_in_response = 0;
        let mut facts_supported_by_knowledge = 0;

        for pattern in &specific_fact_patterns {
            let regex = Regex::new(pattern).unwrap();
            let facts_found = regex.find_iter(response).count();
            specific_facts_in_response += facts_found;

            // Check if these facts appear in knowledge
            for knowledge_item in knowledge {
                let supported_facts = regex.find_iter(&knowledge_item.content).count();
                facts_supported_by_knowledge += supported_facts.min(facts_found);
            }
        }

        let hallucination_risk = if specific_facts_in_response == 0 {
            0.2 // Low risk if no specific facts
        } else {
            1.0 - (facts_supported_by_knowledge as f64 / specific_facts_in_response as f64)
        };

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Hallucination risk: {:.3} ({}/{} specific facts supported)",
                   hallucination_risk, facts_supported_by_knowledge, specific_facts_in_response)
        );

        Ok(hallucination_risk.clamp(0.0, 1.0))
    }

    async fn calculate_confidence_calibration(
        &self,
        response: &str,
        knowledge: &[RetrievedKnowledge],
        debug_info: &mut ConversationDebugInfo,
    ) -> Result<f64, BrainError> {
        // Look for confidence indicators in the response
        let high_confidence_phrases = ["definitely", "certainly", "absolutely", "without a doubt"];
        let low_confidence_phrases = ["might", "possibly", "perhaps", "I think", "it seems"];
        let uncertainty_phrases = ["I don't know", "I'm not sure", "unclear", "uncertain"];

        let response_lower = response.to_lowercase();
        
        let high_confidence_count = high_confidence_phrases.iter()
            .filter(|&&phrase| response_lower.contains(phrase))
            .count();
        
        let low_confidence_count = low_confidence_phrases.iter()
            .filter(|&&phrase| response_lower.contains(phrase))
            .count();
        
        let uncertainty_count = uncertainty_phrases.iter()
            .filter(|&&phrase| response_lower.contains(phrase))
            .count();

        // Calculate knowledge support level
        let avg_knowledge_relevance = if knowledge.is_empty() {
            0.5
        } else {
            knowledge.iter().map(|k| k.relevance_score).sum::<f64>() / knowledge.len() as f64
        };

        // Good calibration means high confidence when knowledge support is high, and vice versa
        let expressed_confidence = if high_confidence_count > 0 {
            0.9
        } else if uncertainty_count > 0 {
            0.3
        } else if low_confidence_count > 0 {
            0.6
        } else {
            0.7 // Neutral
        };

        let confidence_calibration = 1.0 - (expressed_confidence - avg_knowledge_relevance).abs();

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Confidence calibration: {:.3} (expressed: {:.3}, knowledge support: {:.3})",
                   confidence_calibration, expressed_confidence, avg_knowledge_relevance)
        );

        Ok(confidence_calibration)
    }

    // Helper methods for quality analysis
    
    fn calculate_knowledge_influence(&self, response: &str, knowledge: &RetrievedKnowledge) -> f64 {
        let similarity = self.calculate_text_similarity(response, &knowledge.content);
        similarity * knowledge.relevance_score
    }

    fn assess_source_reliability(&self, source: &str) -> f64 {
        // Simple heuristic-based reliability assessment
        match source {
            s if s.contains("memory") => 0.8,
            s if s.contains("concept") => 0.9,
            s if s.contains("pattern") => 0.7,
            _ => 0.6,
        }
    }

    fn calculate_sentence_coherence(&self, sentences: &[&str]) -> f64 {
        if sentences.len() < 2 {
            return 0.7;
        }

        let mut coherence_sum = 0.0;
        for i in 0..sentences.len() - 1 {
            let similarity = self.calculate_text_similarity(sentences[i], sentences[i + 1]);
            coherence_sum += similarity;
        }

        (coherence_sum / (sentences.len() - 1) as f64).clamp(0.0, 1.0)
    }

    fn calculate_consistency_with_history(&self, response: &str, history: &[ChatMessage]) -> f64 {
        if history.len() < 2 {
            return 0.8;
        }

        let mut consistency_scores = Vec::new();
        for msg in history.iter().take(5) { // Check last 5 messages
            if msg.role == "assistant" {
                let similarity = self.calculate_text_similarity(response, &msg.content);
                consistency_scores.push(similarity);
            }
        }

        if consistency_scores.is_empty() {
            0.8
        } else {
            consistency_scores.iter().sum::<f64>() / consistency_scores.len() as f64
        }
    }

    fn find_knowledge_usage_in_response(&self, response: &str, knowledge_content: &str) -> Vec<String> {
        let mut used_portions = Vec::new();
        let knowledge_words: Vec<&str> = knowledge_content.split_whitespace().collect();
        
        // Look for 3+ word phrases from knowledge that appear in response
        for window in knowledge_words.windows(3) {
            let phrase = window.join(" ");
            if response.contains(&phrase) {
                used_portions.push(phrase);
            }
        }

        used_portions
    }

    async fn store_interaction_in_memory(
        &self,
        user_message: &str,
        assistant_response: &str,
        knowledge_used: &[RetrievedKnowledge],
        memory_system: &mut MemorySystem,
    ) -> Result<(), BrainError> {
        // Store the interaction for future learning
        let interaction_summary = format!(
            "User: {} | Assistant: {} | Knowledge sources: {}",
            user_message,
            assistant_response,
            knowledge_used.len()
        );

        memory_system.learn(interaction_summary, Priority::Medium)?;

        // Store the assistant's response as knowledge
        memory_system.learn(assistant_response.to_string(), Priority::Low)?;

        Ok(())
    }

    pub fn get_conversation_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("active_conversations".to_string(), self.conversations.len());
        
        let total_messages: usize = self.conversations.values()
            .map(|c| c.messages.len())
            .sum();
        stats.insert("total_messages".to_string(), total_messages);

        stats
    }

    pub fn get_stats(&mut self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        
        // Basic conversation stats
        stats.insert("active_conversations".to_string(), serde_json::Value::Number(self.conversations.len().into()));
        
        let total_messages: usize = self.conversations.values()
            .map(|c| c.messages.len())
            .sum();
        stats.insert("total_messages".to_string(), serde_json::Value::Number(total_messages.into()));
        
        // Model configuration
        stats.insert("model".to_string(), serde_json::Value::String(self.openai_model.clone()));
        stats.insert("max_tokens".to_string(), serde_json::Value::Number(self.max_tokens.into()));
        stats.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.temperature).unwrap_or_else(|| serde_json::Number::from(0))));
        
        // Average messages per conversation
        let avg_messages_per_conversation = if self.conversations.is_empty() {
            0.0
        } else {
            total_messages as f64 / self.conversations.len() as f64
        };
        stats.insert("avg_messages_per_conversation".to_string(), 
            serde_json::Value::Number(serde_json::Number::from_f64(avg_messages_per_conversation).unwrap_or_else(|| serde_json::Number::from(0))));
        
        // Knowledge retrieval stats
        let total_knowledge_items: usize = self.conversations.values()
            .map(|c| c.retrieved_knowledge.len())
            .sum();
        stats.insert("total_knowledge_retrieved".to_string(), serde_json::Value::Number(total_knowledge_items.into()));
        
        stats
    }

    pub fn clear_conversation(&mut self, conversation_id: &str) -> bool {
        self.conversations.remove(conversation_id).is_some()
    }

    /// Enable training data collection with the specified configuration
    pub fn enable_training_data_collection(
        &mut self,
        collector: TrainingDataCollector,
    ) -> Result<(), BrainError> {
        self.training_data_collector = Some(collector);
        Ok(())
    }

    /// Disable training data collection
    pub fn disable_training_data_collection(&mut self) {
        self.training_data_collector = None;
    }

    /// Get reference to training data collector (if enabled)
    pub fn get_training_data_collector(&self) -> Option<&TrainingDataCollector> {
        self.training_data_collector.as_ref()
    }

    /// Get mutable reference to training data collector (if enabled)
    pub fn get_training_data_collector_mut(&mut self) -> Option<&mut TrainingDataCollector> {
        self.training_data_collector.as_mut()
    }

    /// Check if training data collection is enabled
    pub fn is_training_data_collection_enabled(&self) -> bool {
        self.training_data_collector.is_some()
    }

    /// Direct memory lookup fallback when concept expansion fails
    async fn retrieve_direct_memory_fallback(
        &self,
        message: &str,
        memory_system: &mut MemorySystem,
        threshold: f64,
    ) -> Result<Vec<AdvancedRetrievedKnowledge>, BrainError> {
        let mut knowledge = Vec::new();
        
        // Extract keywords from message with special handling for URLs and repos
        let mut keywords: Vec<String> = Vec::new();
        
        // Handle GitHub URLs specially
        if message.contains("github.com") {
            if let Some(repo_part) = message.split("github.com/").nth(1) {
                let repo_name = repo_part.split('/').collect::<Vec<&str>>();
                if repo_name.len() >= 2 {
                    keywords.push(format!("{}/{}", repo_name[0], repo_name[1]));
                    keywords.push(repo_name[1].to_string()); // Just the repo name
                    keywords.push(repo_name[0].to_string()); // Just the owner
                }
            }
            keywords.push("github".to_string());
            keywords.push("repository".to_string());
            keywords.push("repo".to_string());
        }
        
        // Add regular keywords
        let regular_keywords: Vec<&str> = message.split_whitespace()
            .filter(|word| word.len() > 2)
            .filter(|word| !word.starts_with("http")) // Skip URLs themselves
            .collect();
        
        for keyword in regular_keywords {
            keywords.push(keyword.to_string());
        }
        
        // Remove duplicates
        keywords.sort();
        keywords.dedup();
        
        println!("  - Direct fallback searching for keywords: {:?}", keywords);
        
        // Query working memory for each keyword
        for keyword in &keywords {
            let working_query = WorkingMemoryQuery {
                content_pattern: Some(keyword.clone()),
                limit: Some(5),
                min_importance: Some(0.1), // Lower threshold for fallback
                ..Default::default()
            };
            
            if let Ok(working_items) = memory_system.query_working(&working_query) {
                for item in working_items {
                    let relevance = self.calculate_text_similarity(&item.content, message);
                    if relevance >= threshold * 0.5 { // Lower threshold for fallback
                        knowledge.push(AdvancedRetrievedKnowledge {
                            content: item.content.clone(),
                            knowledge_type: "working_memory_fallback".to_string(),
                            relevance_score: relevance,
                            source: "brain_working_memory".to_string(),
                            timestamp: item.created_at,
                            confidence: (item.priority as u8 as f64) / 4.0,
                            context_score: relevance,
                            personalization_score: 0.0,
                            temporal_relevance: self.calculate_temporal_relevance(&item.created_at),
                            concept_path: vec![keyword.clone()],
                            related_concepts: vec![],
                            source_strength: (item.priority as u8 as f64) / 4.0,
                        });
                    }
                }
            }
        }
        
        // Query semantic memory for broader matches
        for keyword in &keywords {
            let semantic_query = SemanticQuery {
                name_pattern: Some(keyword.clone()),
                min_confidence: Some(0.1), // Lower threshold for fallback
                limit: Some(5),
                ..Default::default()
            };
            
            if let Ok(semantic_concepts) = memory_system.query_semantic(&semantic_query) {
                for concept in semantic_concepts {
                    let relevance = self.calculate_text_similarity(&concept.name, message);
                    if relevance >= threshold * 0.5 { // Lower threshold for fallback
                        let content = format!("{}: {}", concept.name, concept.description);
                        
                        knowledge.push(AdvancedRetrievedKnowledge {
                            content,
                            knowledge_type: "semantic_memory_fallback".to_string(),
                            relevance_score: relevance,
                            source: "brain_semantic_memory".to_string(),
                            timestamp: concept.last_updated,
                            confidence: concept.confidence,
                            context_score: relevance,
                            personalization_score: 0.0,
                            temporal_relevance: self.calculate_temporal_relevance(&concept.last_updated),
                            concept_path: vec![keyword.clone()],
                            related_concepts: vec![],
                            source_strength: concept.confidence,
                        });
                    }
                }
            }
        }
        
        // Query episodic memory with full message first
        let full_message_query = EpisodicQuery {
            content_pattern: Some(message.to_string()),
            time_range: Some((
                Utc::now() - chrono::Duration::days(365),
                Utc::now()
            )),
            limit: Some(10),
            ..Default::default()
        };
        
        if let Ok(episodes) = memory_system.query_episodic(&full_message_query) {
            for episode in episodes {
                let relevance = self.calculate_text_similarity(&episode.content, message);
                if relevance >= threshold * 0.2 { // Very low threshold for full message match
                    knowledge.push(AdvancedRetrievedKnowledge {
                        content: episode.content.clone(),
                        knowledge_type: "episodic_memory_full_match".to_string(),
                        relevance_score: relevance,
                        source: "brain_episodic_memory".to_string(),
                        timestamp: episode.timestamp,
                        confidence: episode.importance,
                        context_score: relevance,
                        personalization_score: 0.0,
                        temporal_relevance: self.calculate_temporal_relevance(&episode.timestamp),
                        concept_path: vec![message.to_string()],
                        related_concepts: vec![],
                        source_strength: episode.importance,
                    });
                }
            }
        }
        
        // Query episodic memory for broader context (not just recent)
        for keyword in &keywords {
            let episodic_query = EpisodicQuery {
                content_pattern: Some(keyword.clone()),
                time_range: Some((
                    Utc::now() - chrono::Duration::days(365), // Much broader time range
                    Utc::now()
                )),
                limit: Some(20), // More results
                ..Default::default()
            };
            
            if let Ok(episodes) = memory_system.query_episodic(&episodic_query) {
                for episode in episodes {
                    let relevance = self.calculate_text_similarity(&episode.content, message);
                    if relevance >= threshold * 0.3 { // Even lower threshold for recent episodes
                        knowledge.push(AdvancedRetrievedKnowledge {
                            content: episode.content.clone(),
                            knowledge_type: "episodic_memory_fallback".to_string(),
                            relevance_score: relevance,
                            source: "brain_episodic_memory".to_string(),
                            timestamp: episode.timestamp,
                            confidence: episode.importance,
                            context_score: relevance,
                            personalization_score: 0.0,
                            temporal_relevance: self.calculate_temporal_relevance(&episode.timestamp),
                            concept_path: vec![keyword.clone()],
                            related_concepts: vec![],
                            source_strength: episode.importance,
                        });
                    }
                }
            }
        }
        
        // Final aggressive fallback: search for any content containing key terms (always run for GitHub/repo queries)
        if message.contains("github") || message.contains("repo") || message.contains("PocketFlow") || message.contains("Pocket") {
            println!("  - Running aggressive content search for GitHub/repo queries...");
            
            // Get ALL recent working memory and search manually
            let broad_working_query = WorkingMemoryQuery {
                content_pattern: None, // Get everything
                limit: Some(50),
                min_importance: Some(0.0), // Very low threshold
                ..Default::default()
            };
            
            if let Ok(all_items) = memory_system.query_working(&broad_working_query) {
                println!("  - Found {} total working memory items to search", all_items.len());
                for item in all_items {
                    let content_lower = item.content.to_lowercase();
                    let message_lower = message.to_lowercase();
                    
                    // Check for partial matches
                    let has_match = keywords.iter().any(|keyword| {
                        content_lower.contains(&keyword.to_lowercase())
                    }) || content_lower.contains("pocketflow") 
                       || content_lower.contains("the-pocket")
                       || (message_lower.contains("github") && content_lower.contains("github"))
                       || (message_lower.contains("repo") && content_lower.contains("repository"));
                    
                    if has_match {
                        let relevance = self.calculate_text_similarity(&item.content, message).max(0.3); // Minimum relevance
                        knowledge.push(AdvancedRetrievedKnowledge {
                            content: item.content.clone(),
                            knowledge_type: "aggressive_content_search".to_string(),
                            relevance_score: relevance,
                            source: "brain_working_memory_aggressive".to_string(),
                            timestamp: item.created_at,
                            confidence: (item.priority as u8 as f64) / 4.0,
                            context_score: relevance,
                            personalization_score: 0.0,
                            temporal_relevance: self.calculate_temporal_relevance(&item.created_at),
                            concept_path: vec!["aggressive_search".to_string()],
                            related_concepts: vec![],
                            source_strength: (item.priority as u8 as f64) / 4.0,
                        });
                    }
                }
            }
            
            // Also search ALL episodic memory aggressively
            let broad_episodic_query = EpisodicQuery {
                content_pattern: None, // Get everything
                time_range: Some((
                    Utc::now() - chrono::Duration::days(365),
                    Utc::now()
                )),
                limit: Some(100), // More results
                ..Default::default()
            };
            
            if let Ok(all_episodes) = memory_system.query_episodic(&broad_episodic_query) {
                println!("  - Found {} total episodic memory items to search", all_episodes.len());
                for episode in all_episodes {
                    let content_lower = episode.content.to_lowercase();
                    let message_lower = message.to_lowercase();
                    
                    // Check for partial matches
                    let has_match = keywords.iter().any(|keyword| {
                        content_lower.contains(&keyword.to_lowercase())
                    }) || content_lower.contains("pocketflow") 
                       || content_lower.contains("the-pocket")
                       || content_lower.contains("pocket")
                       || (message_lower.contains("github") && content_lower.contains("github"))
                       || (message_lower.contains("repo") && content_lower.contains("repository"));
                    
                    if has_match {
                        let relevance = self.calculate_text_similarity(&episode.content, message).max(0.3); // Minimum relevance
                        knowledge.push(AdvancedRetrievedKnowledge {
                            content: episode.content.clone(),
                            knowledge_type: "aggressive_episodic_search".to_string(),
                            relevance_score: relevance,
                            source: "brain_episodic_memory_aggressive".to_string(),
                            timestamp: episode.timestamp,
                            confidence: episode.importance,
                            context_score: relevance,
                            personalization_score: 0.0,
                            temporal_relevance: self.calculate_temporal_relevance(&episode.timestamp),
                            concept_path: vec!["aggressive_search".to_string()],
                            related_concepts: vec![],
                            source_strength: episode.importance,
                        });
                    }
                }
            }
        }
        
        println!("  - Direct fallback found {} knowledge items", knowledge.len());
        Ok(knowledge)
    }

    // API methods for web endpoints
    pub async fn get_learning_analytics(&self) -> LearningAnalytics {
        // Return default analytics since BrainAIOrchestrator doesn't have learning_orchestrator
        LearningAnalytics::default()
    }

    pub async fn start_learning_session(&mut self, objective: String) -> String {
        // Return a simple session ID since BrainAIOrchestrator doesn't have learning_orchestrator
        format!("session_{}_{}", objective.chars().take(10).collect::<String>(), chrono::Utc::now().timestamp())
    }

    pub async fn end_learning_session(&mut self, session_id: String) -> LearningSessionSummary {
        // Return a simple summary since BrainAIOrchestrator doesn't have learning_orchestrator
        LearningSessionSummary {
            session_id,
            duration_minutes: 0.0,
            activities_completed: 0,
            knowledge_gained: 0,
            avg_activity_success: 0.0,
            insights_generated: 0,
            overall_effectiveness: 0.0,
        }
    }

    pub async fn get_current_knowledge_gaps(&self) -> Vec<KnowledgeGap> {
        // Return empty vector since BrainAIOrchestrator doesn't have learning_orchestrator
        Vec::new()
    }

    pub async fn get_meta_learning_recommendations(&self) -> Vec<String> {
        // Return simple recommendations since BrainAIOrchestrator doesn't have learning_orchestrator
        vec!["Initialize Brain AI for recommendations".to_string()]
    }

    pub async fn get_performance_trends(&self) -> PerformanceTrends {
        // Return default trends since BrainAIOrchestrator doesn't have learning_orchestrator
        PerformanceTrends::default()
    }

    // Note: Universal Learning functionality has been moved to BrainAIOrchestrator




















}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExpandedConcept {
    content: String,
    concept_type: ConceptType,
    relevance_score: f64,
    context_path: Vec<String>,
    depth: usize,
}

pub struct BrainLearningOrchestrator {
    /// Active learning component
    pub active_learner: ActiveLearningSystem,
    /// Adaptive query enhancement
    pub query_enhancer: AdaptiveQueryEnhancer,
    /// Meta-learning capabilities
    pub meta_learner: MetaLearningSystem,
    /// Learning session tracking
    pub learning_sessions: Vec<LearningSession>,
    /// Performance metrics
    pub performance_tracker: PerformanceTracker,
}

/// Active Learning System - Identifies knowledge gaps and generates learning opportunities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveLearningSystem {
    /// Identified knowledge gaps
    pub knowledge_gaps: Vec<KnowledgeGap>,
    /// Generated follow-up questions
    pub follow_up_questions: Vec<FollowUpQuestion>,
    /// Learning objectives
    pub learning_objectives: Vec<LearningObjective>,
    /// Confidence thresholds for gap detection
    pub gap_detection_config: GapDetectionConfig,
}

/// Adaptive Query Enhancement - Learns from successful patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveQueryEnhancer {
    /// Successful query patterns
    pub successful_patterns: Vec<QueryPattern>,
    /// Failed query patterns to avoid
    pub failed_patterns: Vec<QueryPattern>,
    /// Domain-specific enhancement rules
    pub domain_rules: HashMap<String, Vec<EnhancementRule>>,
    /// User feedback integration
    pub feedback_history: Vec<QueryFeedback>,
}

/// Meta-Learning System - Analyzes the Brain's own learning patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearningSystem {
    /// Learning pattern analysis
    pub learning_patterns: Vec<LearningPattern>,
    /// Memory optimization insights
    pub memory_optimizations: Vec<MemoryOptimization>,
    /// Concept relationship improvements
    pub relationship_insights: Vec<RelationshipInsight>,
    /// Self-improvement recommendations
    pub improvement_recommendations: Vec<ImprovementRecommendation>,
}

/// Knowledge Gap Detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGap {
    pub gap_id: String,
    pub topic: String,
    pub description: String,
    pub confidence_deficit: f64,
    pub related_queries: Vec<String>,
    pub suggested_learning_actions: Vec<String>,
    pub priority: GapPriority,
    pub discovered_at: DateTime<Utc>,
}

/// Follow-up Question Generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUpQuestion {
    pub question_id: String,
    pub question: String,
    pub context: String,
    pub target_knowledge_area: String,
    pub expected_learning_outcome: String,
    pub difficulty_level: DifficultyLevel,
    pub generated_at: DateTime<Utc>,
}

/// Learning Objective Tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningObjective {
    pub objective_id: String,
    pub description: String,
    pub target_concepts: Vec<String>,
    pub success_criteria: Vec<String>,
    pub progress: f64, // 0.0 to 1.0
    pub estimated_completion: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Query Pattern Learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPattern {
    pub pattern_id: String,
    pub pattern_type: PatternType,
    pub query_structure: String,
    pub success_rate: f64,
    pub avg_confidence: f64,
    pub domain: String,
    pub usage_count: u32,
    pub last_used: DateTime<Utc>,
}

/// Enhancement Rules for Query Improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementRule {
    pub rule_id: String,
    pub condition: String,
    pub enhancement: String,
    pub effectiveness_score: f64,
    pub applicable_domains: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// User Feedback Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFeedback {
    pub feedback_id: String,
    pub original_query: String,
    pub response_quality: f64,
    pub user_satisfaction: f64,
    pub suggested_improvements: Vec<String>,
    pub feedback_at: DateTime<Utc>,
}

/// Learning Pattern Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPattern {
    pub pattern_id: String,
    pub pattern_name: String,
    pub description: String,
    pub frequency: f64,
    pub effectiveness: f64,
    pub conditions: Vec<String>,
    pub outcomes: Vec<String>,
    pub discovered_at: DateTime<Utc>,
}

/// Memory Optimization Insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimization {
    pub optimization_id: String,
    pub optimization_type: OptimizationType,
    pub description: String,
    pub expected_improvement: f64,
    pub implementation_complexity: ComplexityLevel,
    pub suggested_at: DateTime<Utc>,
}

/// Concept Relationship Insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipInsight {
    pub insight_id: String,
    pub concept_a: String,
    pub concept_b: String,
    pub relationship_type: String,
    pub strength: f64,
    pub confidence: f64,
    pub supporting_evidence: Vec<String>,
    pub discovered_at: DateTime<Utc>,
}

/// Self-Improvement Recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRecommendation {
    pub recommendation_id: String,
    pub category: ImprovementCategory,
    pub description: String,
    pub expected_benefit: f64,
    pub implementation_effort: f64,
    pub priority: RecommendationPriority,
    pub suggested_at: DateTime<Utc>,
}

/// Learning Session Tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSession {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub learning_activities: Vec<LearningActivity>,
    pub knowledge_gained: Vec<String>,
    pub performance_metrics: SessionMetrics,
    pub insights_generated: Vec<String>,
}

/// Individual Learning Activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningActivity {
    pub activity_id: String,
    pub activity_type: ActivityType,
    pub description: String,
    pub duration_ms: u64,
    pub success: bool,
    pub knowledge_impact: f64,
    pub performed_at: DateTime<Utc>,
}

/// Performance Tracking System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTracker {
    /// Query performance over time
    pub query_performance: Vec<QueryPerformanceMetric>,
    /// Learning effectiveness metrics
    pub learning_effectiveness: Vec<LearningEffectivenessMetric>,
    /// Knowledge retention tracking
    pub retention_metrics: Vec<RetentionMetric>,
    /// Overall improvement trends
    pub improvement_trends: HashMap<String, Vec<TrendPoint>>,
}

/// Configuration for Gap Detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapDetectionConfig {
    pub min_confidence_threshold: f64,
    pub gap_detection_sensitivity: f64,
    pub max_gaps_per_session: usize,
    pub priority_weighting: HashMap<String, f64>,
}

/// Session Performance Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub queries_processed: u32,
    pub avg_response_time: f64,
    pub avg_confidence: f64,
    pub knowledge_gaps_identified: u32,
    pub learning_objectives_met: u32,
    pub user_satisfaction: f64,
}

/// Query Performance Tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPerformanceMetric {
    pub timestamp: DateTime<Utc>,
    pub query_type: String,
    pub response_time_ms: u64,
    pub confidence: f64,
    pub user_satisfaction: f64,
    pub knowledge_sources_used: u32,
}

/// Learning Effectiveness Measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEffectivenessMetric {
    pub timestamp: DateTime<Utc>,
    pub learning_activity: String,
    pub knowledge_retention: f64,
    pub application_success: f64,
    pub transfer_learning: f64,
}

/// Knowledge Retention Tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionMetric {
    pub concept: String,
    pub initial_confidence: f64,
    pub current_confidence: f64,
    pub retention_rate: f64,
    pub last_accessed: DateTime<Utc>,
    pub access_frequency: u32,
}

/// Trend Analysis Point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub timestamp: DateTime<Utc>,
    pub metric_name: String,
    pub value: f64,
    pub trend_direction: TrendDirection,
}

// Enums for categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GapPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Successful,
    Failed,
    Experimental,
    Optimized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    Storage,
    Retrieval,
    Indexing,
    Relationship,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImprovementCategory {
    QueryProcessing,
    MemoryManagement,
    LearningEfficiency,
    UserExperience,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Urgent,
    High,
    Medium,
    Low,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    KnowledgeAcquisition,
    PatternRecognition,
    ConceptLinking,
    GapIdentification,
    QueryOptimization,
    MetaAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
    Volatile,
}

impl BrainLearningOrchestrator {
    /// Create a new Brain Learning Orchestrator
    pub fn new() -> Self {
        Self {
            active_learner: ActiveLearningSystem::new(),
            query_enhancer: AdaptiveQueryEnhancer::new(),
            meta_learner: MetaLearningSystem::new(),
            learning_sessions: Vec::new(),
            performance_tracker: PerformanceTracker::new(),
        }
    }



    /// Process a query and identify learning opportunities
    pub async fn process_query_for_learning(
        &mut self,
        query: &str,
        response_confidence: f64,
        response_quality: f64,
        knowledge_sources: usize,
    ) -> Result<LearningOpportunities, BrainError> {
        // 1. Identify knowledge gaps
        let gaps = self.active_learner.identify_knowledge_gaps(query, response_confidence).await?;
        
        // 2. Generate follow-up questions
        let follow_ups = self.active_learner.generate_follow_up_questions(query, &gaps).await?;
        
        // 3. Learn from query patterns
        self.query_enhancer.learn_from_query(query, response_confidence, response_quality).await?;
        
        // 4. Update performance tracking
        self.performance_tracker.record_query_performance(
            query,
            response_confidence,
            response_quality,
            knowledge_sources,
        ).await?;

        Ok(LearningOpportunities {
            identified_gaps: gaps,
            follow_up_questions: follow_ups,
            suggested_improvements: self.query_enhancer.suggest_query_improvements(query).await?,
            learning_recommendations: self.meta_learner.generate_learning_recommendations().await?,
        })
    }





    /// Calculate overall learning efficiency
    async fn calculate_learning_efficiency(&self) -> Result<f64, BrainError> {
        if self.learning_sessions.is_empty() {
            return Ok(0.0);
        }

        let total_effectiveness: f64 = self.learning_sessions.iter()
            .map(|s| s.performance_metrics.avg_confidence)
            .sum();
        
        Ok(total_effectiveness / self.learning_sessions.len() as f64)
    }
}

impl ActiveLearningSystem {
    /// Create a new Active Learning System
    pub fn new() -> Self {
        Self {
            knowledge_gaps: Vec::new(),
            follow_up_questions: Vec::new(),
            learning_objectives: Vec::new(),
            gap_detection_config: GapDetectionConfig::default(),
        }
    }

    /// Identify knowledge gaps based on query performance
    pub async fn identify_knowledge_gaps(&mut self, query: &str, confidence: f64) -> Result<Vec<KnowledgeGap>, BrainError> {
        let mut gaps = Vec::new();

        if confidence < self.gap_detection_config.min_confidence_threshold {
            let gap = KnowledgeGap {
                gap_id: Uuid::new_v4().to_string(),
                topic: self.extract_topic_from_query(query),
                description: format!("Low confidence response ({}%) for query: {}", (confidence * 100.0) as u32, query),
                confidence_deficit: self.gap_detection_config.min_confidence_threshold - confidence,
                related_queries: vec![query.to_string()],
                suggested_learning_actions: self.generate_learning_actions(query, confidence).await?,
                priority: self.calculate_gap_priority(confidence),
                discovered_at: Utc::now(),
            };
            gaps.push(gap.clone());
            self.knowledge_gaps.push(gap);
        }

        Ok(gaps)
    }

    /// Generate follow-up questions to address knowledge gaps
    pub async fn generate_follow_up_questions(&mut self, query: &str, gaps: &[KnowledgeGap]) -> Result<Vec<FollowUpQuestion>, BrainError> {
        let mut questions = Vec::new();

        for gap in gaps {
            let follow_up = FollowUpQuestion {
                question_id: Uuid::new_v4().to_string(),
                question: self.generate_clarifying_question(&gap.topic, query).await?,
                context: format!("Following up on knowledge gap: {}", gap.description),
                target_knowledge_area: gap.topic.clone(),
                expected_learning_outcome: format!("Improve understanding of {}", gap.topic),
                difficulty_level: self.assess_question_difficulty(&gap.topic),
                generated_at: Utc::now(),
            };
            questions.push(follow_up.clone());
            self.follow_up_questions.push(follow_up);
        }

        Ok(questions)
    }

    /// Get current status of active learning
    pub async fn get_status(&self) -> Result<ActiveLearningStatus, BrainError> {
        Ok(ActiveLearningStatus {
            total_gaps_identified: self.knowledge_gaps.len(),
            high_priority_gaps: self.knowledge_gaps.iter().filter(|g| matches!(g.priority, GapPriority::High | GapPriority::Critical)).count(),
            follow_up_questions_generated: self.follow_up_questions.len(),
            learning_objectives_active: self.learning_objectives.iter().filter(|o| o.progress < 1.0).count(),
            recent_gap_trends: self.analyze_recent_gap_trends().await?,
        })
    }

    /// Extract topic from query using simple heuristics
    fn extract_topic_from_query(&self, query: &str) -> String {
        // Simple topic extraction - in a real implementation, this could use NLP
        let words: Vec<&str> = query.split_whitespace().collect();
        if words.len() > 2 {
            words[0..3].join(" ")
        } else {
            query.to_string()
        }
    }

    /// Generate learning actions for addressing gaps
    async fn generate_learning_actions(&self, query: &str, confidence: f64) -> Result<Vec<String>, BrainError> {
        let mut actions = Vec::new();

        if confidence < 0.3 {
            actions.push("Seek additional sources for this topic".to_string());
            actions.push("Break down the query into simpler components".to_string());
        } else if confidence < 0.6 {
            actions.push("Verify information with cross-references".to_string());
            actions.push("Explore related concepts".to_string());
        }

        actions.push(format!("Research more about: {}", self.extract_topic_from_query(query)));
        Ok(actions)
    }

    /// Calculate priority of knowledge gap
    fn calculate_gap_priority(&self, confidence: f64) -> GapPriority {
        if confidence < 0.2 {
            GapPriority::Critical
        } else if confidence < 0.4 {
            GapPriority::High
        } else if confidence < 0.6 {
            GapPriority::Medium
        } else {
            GapPriority::Low
        }
    }

    /// Generate clarifying question
    async fn generate_clarifying_question(&self, topic: &str, original_query: &str) -> Result<String, BrainError> {
        // Simple question generation - could be enhanced with LLM
        Ok(format!("Can you provide more specific details about {} in the context of '{}'?", topic, original_query))
    }

    /// Assess question difficulty
    fn assess_question_difficulty(&self, topic: &str) -> DifficultyLevel {
        // Simple heuristic - could be improved with domain knowledge
        if topic.len() < 10 {
            DifficultyLevel::Beginner
        } else if topic.contains("architecture") || topic.contains("framework") {
            DifficultyLevel::Advanced
        } else {
            DifficultyLevel::Intermediate
        }
    }

    /// Analyze recent gap trends
    async fn analyze_recent_gap_trends(&self) -> Result<Vec<String>, BrainError> {
        let recent_gaps: Vec<_> = self.knowledge_gaps.iter()
            .filter(|g| (Utc::now() - g.discovered_at).num_hours() < 24)
            .collect();

        let mut trends = Vec::new();
        if recent_gaps.len() > 5 {
            trends.push("High number of knowledge gaps identified recently".to_string());
        }
        
        // Group by topic
        let mut topic_counts = HashMap::new();
        for gap in &recent_gaps {
            *topic_counts.entry(&gap.topic).or_insert(0) += 1;
        }
        
        for (topic, count) in topic_counts {
            if count > 2 {
                trends.push(format!("Recurring gaps in topic: {}", topic));
            }
        }

        Ok(trends)
    }
}

impl AdaptiveQueryEnhancer {
    /// Create a new Adaptive Query Enhancer
    pub fn new() -> Self {
        Self {
            successful_patterns: Vec::new(),
            failed_patterns: Vec::new(),
            domain_rules: HashMap::new(),
            feedback_history: Vec::new(),
        }
    }

    /// Learn from query performance
    pub async fn learn_from_query(&mut self, query: &str, confidence: f64, quality: f64) -> Result<(), BrainError> {
        let pattern = QueryPattern {
            pattern_id: Uuid::new_v4().to_string(),
            pattern_type: if confidence > 0.7 && quality > 0.7 {
                PatternType::Successful
            } else if confidence < 0.4 || quality < 0.4 {
                PatternType::Failed
            } else {
                PatternType::Experimental
            },
            query_structure: self.extract_query_structure(query),
            success_rate: (confidence + quality) / 2.0,
            avg_confidence: confidence,
            domain: self.identify_domain(query),
            usage_count: 1,
            last_used: Utc::now(),
        };

        match pattern.pattern_type {
            PatternType::Successful => self.successful_patterns.push(pattern),
            PatternType::Failed => self.failed_patterns.push(pattern),
            _ => {} // Handle experimental patterns differently
        }

        Ok(())
    }

    /// Suggest query improvements
    pub async fn suggest_query_improvements(&self, query: &str) -> Result<Vec<String>, BrainError> {
        let mut suggestions = Vec::new();
        let domain = self.identify_domain(query);

        // Check against failed patterns
        for failed_pattern in &self.failed_patterns {
            if self.query_matches_pattern(query, &failed_pattern.query_structure) {
                suggestions.push(format!("Avoid pattern '{}' which has low success rate", failed_pattern.query_structure));
            }
        }

        // Suggest successful patterns
        for successful_pattern in &self.successful_patterns {
            if successful_pattern.domain == domain && successful_pattern.success_rate > 0.8 {
                suggestions.push(format!("Consider using pattern '{}' which has high success rate", successful_pattern.query_structure));
            }
        }

        // Domain-specific suggestions
        if let Some(rules) = self.domain_rules.get(&domain) {
            for rule in rules {
                if rule.effectiveness_score > 0.7 {
                    suggestions.push(format!("Apply rule: {}", rule.enhancement));
                }
            }
        }

        Ok(suggestions)
    }

    /// Get enhancement insights
    pub async fn get_insights(&self) -> Result<QueryEnhancementInsights, BrainError> {
        Ok(QueryEnhancementInsights {
            successful_patterns_count: self.successful_patterns.len(),
            failed_patterns_count: self.failed_patterns.len(),
            domain_rules_count: self.domain_rules.values().map(|rules| rules.len()).sum(),
            top_performing_patterns: self.get_top_patterns(5).await?,
            improvement_opportunities: self.identify_improvement_opportunities().await?,
        })
    }

    /// Extract query structure for pattern matching
    fn extract_query_structure(&self, query: &str) -> String {
        // Simple structure extraction - could be enhanced
        let words: Vec<&str> = query.split_whitespace().collect();
        if words.is_empty() {
            return "empty".to_string();
        }

        let mut structure = Vec::new();
        for word in &words[0..words.len().min(3)] {
            if word.starts_with("what") || word.starts_with("how") || word.starts_with("why") {
                structure.push("QUESTION_WORD");
            } else if word.chars().next().unwrap().is_uppercase() {
                structure.push("PROPER_NOUN");
            } else {
                structure.push("WORD");
            }
        }
        
        structure.join("_")
    }

    /// Identify domain from query
    fn identify_domain(&self, query: &str) -> String {
        let query_lower = query.to_lowercase();
        
        if query_lower.contains("architecture") || query_lower.contains("framework") || query_lower.contains("pattern") {
            "software_architecture".to_string()
        } else if query_lower.contains("code") || query_lower.contains("programming") {
            "programming".to_string()
        } else if query_lower.contains("github") || query_lower.contains("repository") {
            "version_control".to_string()
        } else {
            "general".to_string()
        }
    }

    /// Check if query matches pattern
    fn query_matches_pattern(&self, query: &str, pattern: &str) -> bool {
        let query_structure = self.extract_query_structure(query);
        query_structure == pattern
    }

    /// Get top performing patterns
    async fn get_top_patterns(&self, limit: usize) -> Result<Vec<QueryPattern>, BrainError> {
        let mut patterns = self.successful_patterns.clone();
        patterns.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());
        patterns.truncate(limit);
        Ok(patterns)
    }

    /// Identify improvement opportunities
    async fn identify_improvement_opportunities(&self) -> Result<Vec<String>, BrainError> {
        let mut opportunities = Vec::new();

        if self.failed_patterns.len() > self.successful_patterns.len() {
            opportunities.push("High failure rate - consider query reformulation strategies".to_string());
        }

        if self.domain_rules.is_empty() {
            opportunities.push("No domain-specific rules defined - consider creating enhancement rules".to_string());
        }

        Ok(opportunities)
    }
}

impl MetaLearningSystem {
    /// Create a new Meta-Learning System
    pub fn new() -> Self {
        Self {
            learning_patterns: Vec::new(),
            memory_optimizations: Vec::new(),
            relationship_insights: Vec::new(),
            improvement_recommendations: Vec::new(),
        }
    }

    /// Analyze session patterns for meta-learning
    pub async fn analyze_session_patterns(&mut self, session: &LearningSession) -> Result<(), BrainError> {
        // Analyze learning patterns from the session
        let pattern = self.extract_learning_pattern(session).await?;
        if let Some(pattern) = pattern {
            self.learning_patterns.push(pattern);
        }

        // Generate memory optimizations
        let optimizations = self.identify_memory_optimizations(session).await?;
        self.memory_optimizations.extend(optimizations);

        // Generate improvement recommendations
        let recommendations = self.generate_session_recommendations(session).await?;
        self.improvement_recommendations.extend(recommendations);

        Ok(())
    }

    /// Generate learning recommendations
    pub async fn generate_learning_recommendations(&self) -> Result<Vec<String>, BrainError> {
        let mut recommendations = Vec::new();

        for rec in &self.improvement_recommendations {
            if matches!(rec.priority, RecommendationPriority::High | RecommendationPriority::Urgent) {
                recommendations.push(rec.description.clone());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("Continue current learning approach".to_string());
        }

        Ok(recommendations)
    }

    /// Get meta-learning recommendations
    pub async fn get_recommendations(&self) -> Result<MetaLearningRecommendations, BrainError> {
        Ok(MetaLearningRecommendations {
            learning_patterns_identified: self.learning_patterns.len(),
            memory_optimizations_suggested: self.memory_optimizations.len(),
            relationship_insights_discovered: self.relationship_insights.len(),
            high_priority_recommendations: self.improvement_recommendations.iter()
                .filter(|r| matches!(r.priority, RecommendationPriority::High | RecommendationPriority::Urgent))
                .count(),
            recent_insights: self.get_recent_insights().await?,
        })
    }

    /// Extract learning pattern from session
    async fn extract_learning_pattern(&self, session: &LearningSession) -> Result<Option<LearningPattern>, BrainError> {
        if session.learning_activities.len() < 3 {
            return Ok(None);
        }

        let success_rate = session.learning_activities.iter()
            .map(|a| if a.success { 1.0 } else { 0.0 })
            .sum::<f64>() / session.learning_activities.len() as f64;

        if success_rate > 0.7 {
            let pattern = LearningPattern {
                pattern_id: Uuid::new_v4().to_string(),
                pattern_name: "High Success Session".to_string(),
                description: format!("Session with {}% success rate", (success_rate * 100.0) as u32),
                frequency: 1.0, // Will be updated as more patterns are found
                effectiveness: success_rate,
                conditions: vec![
                    format!("Activities: {}", session.learning_activities.len()),
                    format!("Duration: {} minutes", (session.end_time.unwrap_or(Utc::now()) - session.start_time).num_minutes()),
                ],
                outcomes: session.insights_generated.clone(),
                discovered_at: Utc::now(),
            };
            Ok(Some(pattern))
        } else {
            Ok(None)
        }
    }

    /// Identify memory optimizations
    async fn identify_memory_optimizations(&self, session: &LearningSession) -> Result<Vec<MemoryOptimization>, BrainError> {
        let mut optimizations = Vec::new();

        if session.performance_metrics.avg_response_time > 1000.0 {
            optimizations.push(MemoryOptimization {
                optimization_id: Uuid::new_v4().to_string(),
                optimization_type: OptimizationType::Performance,
                description: "Improve response time by optimizing memory retrieval".to_string(),
                expected_improvement: 0.3,
                implementation_complexity: ComplexityLevel::Moderate,
                suggested_at: Utc::now(),
            });
        }

        if session.performance_metrics.avg_confidence < 0.6 {
            optimizations.push(MemoryOptimization {
                optimization_id: Uuid::new_v4().to_string(),
                optimization_type: OptimizationType::Retrieval,
                description: "Enhance retrieval algorithms to improve confidence".to_string(),
                expected_improvement: 0.4,
                implementation_complexity: ComplexityLevel::Complex,
                suggested_at: Utc::now(),
            });
        }

        Ok(optimizations)
    }

    /// Generate session recommendations
    async fn generate_session_recommendations(&self, session: &LearningSession) -> Result<Vec<ImprovementRecommendation>, BrainError> {
        let mut recommendations = Vec::new();

        let avg_knowledge_impact = session.learning_activities.iter()
            .map(|a| a.knowledge_impact)
            .sum::<f64>() / session.learning_activities.len().max(1) as f64;

        if avg_knowledge_impact < 0.5 {
            recommendations.push(ImprovementRecommendation {
                recommendation_id: Uuid::new_v4().to_string(),
                category: ImprovementCategory::LearningEfficiency,
                description: "Focus on higher-impact learning activities".to_string(),
                expected_benefit: 0.6,
                implementation_effort: 0.3,
                priority: RecommendationPriority::High,
                suggested_at: Utc::now(),
            });
        }

        Ok(recommendations)
    }

    /// Get recent insights
    async fn get_recent_insights(&self) -> Result<Vec<String>, BrainError> {
        let recent_patterns: Vec<_> = self.learning_patterns.iter()
            .filter(|p| (Utc::now() - p.discovered_at).num_hours() < 24)
            .collect();

        let insights = recent_patterns.iter()
            .map(|p| format!("Pattern '{}': {}", p.pattern_name, p.description))
            .collect();

        Ok(insights)
    }
}

impl BrainLearningOrchestrator {
    // Additional API methods for web endpoints
    pub async fn get_learning_analytics(&self) -> LearningAnalytics {
        LearningAnalytics {
            active_learning_status: self.active_learner.get_status().await.unwrap_or_default(),
            query_enhancement_insights: self.query_enhancer.get_insights().await.unwrap_or_default(),
            meta_learning_recommendations: self.meta_learner.get_recommendations().await.unwrap_or_default(),
            performance_trends: self.performance_tracker.get_trends().await.unwrap_or_default(),
            learning_efficiency: self.calculate_learning_efficiency().await.unwrap_or(0.0),
        }
    }

    pub async fn start_learning_session(&mut self, objective: String) -> String {
        let session_id = format!("session_{}", chrono::Utc::now().timestamp());
        let session = LearningSession {
            session_id: session_id.clone(),
            start_time: chrono::Utc::now(),
            end_time: None,
            learning_activities: Vec::new(),
            knowledge_gained: Vec::new(),
            performance_metrics: SessionMetrics::default(),
            insights_generated: vec![objective],
        };
        self.learning_sessions.push(session);
        session_id
    }

    pub async fn end_learning_session(&mut self, session_id: String) -> LearningSessionSummary {
        if let Some(session_index) = self.learning_sessions.iter().position(|s| s.session_id == session_id) {
            let mut session = self.learning_sessions.remove(session_index);
            session.end_time = Some(chrono::Utc::now());
            
            let duration = session.end_time.unwrap().signed_duration_since(session.start_time);
            
            LearningSessionSummary {
                session_id: session.session_id,
                duration_minutes: duration.num_minutes() as f64,
                activities_completed: session.learning_activities.len(),
                knowledge_gained: session.knowledge_gained.len(),
                avg_activity_success: session.learning_activities.iter()
                    .map(|a| if a.success { 1.0 } else { 0.0 })
                    .sum::<f64>() / session.learning_activities.len().max(1) as f64,
                insights_generated: session.insights_generated.len(),
                overall_effectiveness: session.performance_metrics.avg_confidence,
            }
        } else {
            LearningSessionSummary {
                session_id,
                duration_minutes: 0.0,
                activities_completed: 0,
                knowledge_gained: 0,
                avg_activity_success: 0.0,
                insights_generated: 0,
                overall_effectiveness: 0.0,
            }
        }
    }

    pub async fn get_current_knowledge_gaps(&self) -> Vec<KnowledgeGap> {
        self.active_learner.knowledge_gaps.clone()
    }

    pub async fn get_meta_learning_recommendations(&self) -> Vec<String> {
        self.meta_learner.generate_learning_recommendations().await.unwrap_or_default()
    }

    pub async fn get_performance_trends(&self) -> PerformanceTrends {
        self.performance_tracker.get_trends().await.unwrap_or_default()
    }
}

impl PerformanceTracker {
    /// Create a new Performance Tracker
    pub fn new() -> Self {
        Self {
            query_performance: Vec::new(),
            learning_effectiveness: Vec::new(),
            retention_metrics: Vec::new(),
            improvement_trends: HashMap::new(),
        }
    }

    /// Record query performance
    pub async fn record_query_performance(
        &mut self,
        query: &str,
        confidence: f64,
        quality: f64,
        sources: usize,
    ) -> Result<(), BrainError> {
        let metric = QueryPerformanceMetric {
            timestamp: Utc::now(),
            query_type: self.classify_query_type(query),
            response_time_ms: 0, // Would be measured in real implementation
            confidence,
            user_satisfaction: quality,
            knowledge_sources_used: sources as u32,
        };

        self.query_performance.push(metric);
        self.update_trends("query_confidence", confidence).await?;
        self.update_trends("query_quality", quality).await?;

        Ok(())
    }

    /// Get performance trends
    pub async fn get_trends(&self) -> Result<PerformanceTrends, BrainError> {
        Ok(PerformanceTrends {
            query_performance_trend: self.calculate_trend("query_confidence").await?,
            learning_effectiveness_trend: self.calculate_trend("learning_effectiveness").await?,
            overall_improvement: self.calculate_overall_improvement().await?,
            recent_performance_summary: self.get_recent_performance_summary().await?,
        })
    }

    /// Classify query type
    fn classify_query_type(&self, query: &str) -> String {
        let query_lower = query.to_lowercase();
        
        if query_lower.starts_with("what") {
            "factual".to_string()
        } else if query_lower.starts_with("how") {
            "procedural".to_string()
        } else if query_lower.starts_with("why") {
            "explanatory".to_string()
        } else {
            "general".to_string()
        }
    }

    /// Update performance trends
    async fn update_trends(&mut self, metric_name: &str, value: f64) -> Result<(), BrainError> {
        let trend_point = TrendPoint {
            timestamp: Utc::now(),
            metric_name: metric_name.to_string(),
            value,
            trend_direction: self.calculate_trend_direction(metric_name, value).await?,
        };

        self.improvement_trends
            .entry(metric_name.to_string())
            .or_insert_with(Vec::new)
            .push(trend_point);

        Ok(())
    }

    /// Calculate trend direction
    async fn calculate_trend_direction(&self, metric_name: &str, _current_value: f64) -> Result<TrendDirection, BrainError> {
        if let Some(points) = self.improvement_trends.get(metric_name) {
            if points.len() < 2 {
                return Ok(TrendDirection::Stable);
            }

            let recent_avg = points.iter()
                .rev()
                .take(5)
                .map(|p| p.value)
                .sum::<f64>() / 5.min(points.len()) as f64;

            let older_avg = points.iter()
                .rev()
                .skip(5)
                .take(5)
                .map(|p| p.value)
                .sum::<f64>() / 5.min(points.len().saturating_sub(5)) as f64;

            if recent_avg > older_avg + 0.1 {
                Ok(TrendDirection::Improving)
            } else if recent_avg < older_avg - 0.1 {
                Ok(TrendDirection::Declining)
            } else {
                Ok(TrendDirection::Stable)
            }
        } else {
            Ok(TrendDirection::Stable)
        }
    }

    /// Calculate trend for metric
    async fn calculate_trend(&self, metric_name: &str) -> Result<TrendDirection, BrainError> {
        if let Some(points) = self.improvement_trends.get(metric_name) {
            if let Some(latest) = points.last() {
                Ok(latest.trend_direction.clone())
            } else {
                Ok(TrendDirection::Stable)
            }
        } else {
            Ok(TrendDirection::Stable)
        }
    }

    /// Calculate overall improvement
    async fn calculate_overall_improvement(&self) -> Result<f64, BrainError> {
        if self.query_performance.is_empty() {
            return Ok(0.0);
        }

        let recent_performance = self.query_performance.iter()
            .rev()
            .take(10)
            .map(|m| m.confidence)
            .sum::<f64>() / 10.min(self.query_performance.len()) as f64;

        let older_performance = self.query_performance.iter()
            .rev()
            .skip(10)
            .take(10)
            .map(|m| m.confidence)
            .sum::<f64>() / 10.min(self.query_performance.len().saturating_sub(10)) as f64;

        Ok(recent_performance - older_performance)
    }

    /// Get recent performance summary
    async fn get_recent_performance_summary(&self) -> Result<String, BrainError> {
        if self.query_performance.is_empty() {
            return Ok("No performance data available".to_string());
        }

        let recent_queries = self.query_performance.iter().rev().take(10).collect::<Vec<_>>();
        let avg_confidence = recent_queries.iter().map(|m| m.confidence).sum::<f64>() / recent_queries.len() as f64;
        let avg_satisfaction = recent_queries.iter().map(|m| m.user_satisfaction).sum::<f64>() / recent_queries.len() as f64;

        Ok(format!(
            "Recent performance: {:.1}% confidence, {:.1}% satisfaction over {} queries",
            avg_confidence * 100.0,
            avg_satisfaction * 100.0,
            recent_queries.len()
        ))
    }
}

// Additional supporting structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningOpportunities {
    pub identified_gaps: Vec<KnowledgeGap>,
    pub follow_up_questions: Vec<FollowUpQuestion>,
    pub suggested_improvements: Vec<String>,
    pub learning_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSessionSummary {
    pub session_id: String,
    pub duration_minutes: f64,
    pub activities_completed: usize,
    pub knowledge_gained: usize,
    pub avg_activity_success: f64,
    pub insights_generated: usize,
    pub overall_effectiveness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningAnalytics {
    pub active_learning_status: ActiveLearningStatus,
    pub query_enhancement_insights: QueryEnhancementInsights,
    pub meta_learning_recommendations: MetaLearningRecommendations,
    pub performance_trends: PerformanceTrends,
    pub learning_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveLearningStatus {
    pub total_gaps_identified: usize,
    pub high_priority_gaps: usize,
    pub follow_up_questions_generated: usize,
    pub learning_objectives_active: usize,
    pub recent_gap_trends: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEnhancementInsights {
    pub successful_patterns_count: usize,
    pub failed_patterns_count: usize,
    pub domain_rules_count: usize,
    pub top_performing_patterns: Vec<QueryPattern>,
    pub improvement_opportunities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearningRecommendations {
    pub learning_patterns_identified: usize,
    pub memory_optimizations_suggested: usize,
    pub relationship_insights_discovered: usize,
    pub high_priority_recommendations: usize,
    pub recent_insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    pub query_performance_trend: TrendDirection,
    pub learning_effectiveness_trend: TrendDirection,
    pub overall_improvement: f64,
    pub recent_performance_summary: String,
}

impl Default for SessionMetrics {
    fn default() -> Self {
        Self {
            queries_processed: 0,
            avg_response_time: 0.0,
            avg_confidence: 0.0,
            knowledge_gaps_identified: 0,
            learning_objectives_met: 0,
            user_satisfaction: 0.0,
        }
    }
}

impl Default for GapDetectionConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.7,
            gap_detection_sensitivity: 0.5,
            max_gaps_per_session: 10,
            priority_weighting: HashMap::new(),
        }
    }
}

impl Default for ActiveLearningStatus {
    fn default() -> Self {
        Self {
            total_gaps_identified: 0,
            high_priority_gaps: 0,
            follow_up_questions_generated: 0,
            learning_objectives_active: 0,
            recent_gap_trends: Vec::new(),
        }
    }
}

impl Default for QueryEnhancementInsights {
    fn default() -> Self {
        Self {
            successful_patterns_count: 0,
            failed_patterns_count: 0,
            domain_rules_count: 0,
            top_performing_patterns: Vec::new(),
            improvement_opportunities: Vec::new(),
        }
    }
}

impl Default for MetaLearningRecommendations {
    fn default() -> Self {
        Self {
            learning_patterns_identified: 0,
            memory_optimizations_suggested: 0,
            relationship_insights_discovered: 0,
            high_priority_recommendations: 0,
            recent_insights: Vec::new(),
        }
    }
}

impl Default for PerformanceTrends {
    fn default() -> Self {
        Self {
            query_performance_trend: TrendDirection::Stable,
            learning_effectiveness_trend: TrendDirection::Stable,
            overall_improvement: 0.0,
            recent_performance_summary: "No data available".to_string(),
        }
    }
}

impl Default for LearningAnalytics {
    fn default() -> Self {
        Self {
            active_learning_status: ActiveLearningStatus::default(),
            query_enhancement_insights: QueryEnhancementInsights::default(),
            meta_learning_recommendations: MetaLearningRecommendations::default(),
            performance_trends: PerformanceTrends::default(),
            learning_efficiency: 0.0,
        }
    }
}

/// Brain AI Orchestrator - True AI delegation system
pub struct BrainAIOrchestrator {
    github_learning_engine: GitHubLearningEngine,
    pattern_detector: PatternDetector,
    bpe_segmenter: BpeSegmenter,
    analysis_config: BrainAnalysisConfig,
}

/// Configuration for Brain AI analysis
#[derive(Debug, Clone)]
pub struct BrainAnalysisConfig {
    pub enable_github_analysis: bool,
    pub enable_pattern_analysis: bool,
    pub enable_concept_analysis: bool,
    pub enable_semantic_analysis: bool,
    pub max_analysis_depth: usize,
    pub min_confidence_threshold: f64,
}

/// Brain AI analysis result
#[derive(Debug, Clone)]
pub struct BrainAnalysisResult {
    pub analysis: String,
    pub insights: Vec<BrainInsight>,
    pub confidence: f64,
    pub metadata: BrainAnalysisMetadata,
}

/// Individual insight from Brain AI analysis
#[derive(Debug, Clone)]
pub struct BrainInsight {
    pub content: String,
    pub insight_type: String,
    pub confidence: f64,
    pub source: String,
}

/// Metadata about the Brain AI analysis
#[derive(Debug, Clone)]
pub struct BrainAnalysisMetadata {
    pub method: String,
    pub sources_analyzed: usize,
    pub quality_score: f64,
    pub processing_time_ms: u64,
}

impl Default for BrainAnalysisConfig {
    fn default() -> Self {
        Self {
            enable_github_analysis: true,
            enable_pattern_analysis: true,
            enable_concept_analysis: true,
            enable_semantic_analysis: true,
            max_analysis_depth: 3,
            min_confidence_threshold: 0.3,
        }
    }
}

impl BrainAIOrchestrator {
    /// Create a new Brain AI Orchestrator
    pub fn new() -> Result<Self, BrainError> {
        let github_config = GitHubLearningConfig::default();
        let github_learning_engine = GitHubLearningEngine::new(None, Some(github_config));
        let pattern_detector = PatternDetector::new();
        let bpe_segmenter = BpeSegmenter::new(Default::default());
        let analysis_config = BrainAnalysisConfig::default();

        Ok(Self {
            github_learning_engine,
            pattern_detector,
            bpe_segmenter,
            analysis_config,
        })
    }

    /// Analyze a query using Brain AI's full capabilities
    pub async fn analyze_query(
        &mut self,
        query: &str,
        memory_system: &mut MemorySystem,
        concept_graph: &mut ConceptGraphManager,
    ) -> Result<BrainAnalysisResult, BrainError> {
        let start_time = std::time::Instant::now();
        let mut insights = Vec::new();
        let mut sources_analyzed = 0;

        println!("🧠 Brain AI Orchestrator: Starting comprehensive analysis of '{}'", query);

        // **NEW: Universal Learning Detection - Check if we should learn from this content**
        if self.should_trigger_universal_learning(query) {
            println!("  🎓 Universal Learning triggered - processing content for learning");
            match self.process_universal_learning(query, memory_system, concept_graph).await {
                Ok(learning_insights) => {
                    sources_analyzed += learning_insights.len();
                    for insight in learning_insights {
                        insights.push(BrainInsight {
                            content: insight.content,
                            insight_type: insight.knowledge_type,
                            confidence: insight.relevance_score,
                            source: insight.source,
                        });
                    }
                    println!("  ✅ Universal Learning completed: {} insights learned", insights.len());
                }
                Err(e) => println!("  ⚠️ Universal Learning failed: {}", e),
            }
        }

        // 1. GitHub Analysis (if GitHub-related)
        if self.analysis_config.enable_github_analysis && self.is_github_related_brain(query) {
            println!("  🔍 Performing GitHub repository analysis...");
            match self.perform_github_analysis(query, memory_system).await {
                Ok(github_insights) => {
                    sources_analyzed += github_insights.len();
                    for insight in github_insights {
                        insights.push(BrainInsight {
                            content: insight,
                            insight_type: "github_analysis".to_string(),
                            confidence: 0.8,
                            source: "GitHub Learning Engine".to_string(),
                        });
                    }
                }
                Err(e) => println!("  ⚠️ GitHub analysis failed: {}", e),
            }
        }

        // 2. Pattern Analysis
        if self.analysis_config.enable_pattern_analysis {
            println!("  🔍 Performing pattern analysis...");
            match self.perform_pattern_analysis(query, memory_system).await {
                Ok(pattern_insights) => {
                    sources_analyzed += pattern_insights.len();
                    for insight in pattern_insights {
                        insights.push(BrainInsight {
                            content: insight,
                            insight_type: "pattern_analysis".to_string(),
                            confidence: 0.7,
                            source: "Pattern Detector".to_string(),
                        });
                    }
                }
                Err(e) => println!("  ⚠️ Pattern analysis failed: {}", e),
            }
        }

        // 3. DoTA-RAG inspired semantic memory analysis
        if self.analysis_config.enable_semantic_analysis {
            println!("  🕸️ Performing concept graph analysis...");
            match self.perform_semantic_analysis(query, memory_system).await {
                Ok(semantic_insights) => {
                    sources_analyzed += semantic_insights.len();
                    for insight in semantic_insights {
                        insights.push(BrainInsight {
                            content: insight.content,
                            insight_type: "semantic_analysis".to_string(),
                            confidence: insight.confidence,
                            source: "Semantic Memory".to_string(),
                        });
                    }
                }
                Err(e) => println!("  ⚠️ Semantic analysis failed: {}", e),
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;
        let confidence = if insights.is_empty() { 0.3 } else { 
            insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len() as f64 
        };

        // Generate intelligent response using OpenAI based on learned insights
        let analysis = if insights.is_empty() {
            // No insights available, generate a helpful response
            match self.generate_ai_response_for_query(query, &[], memory_system).await {
                Ok(ai_response) => ai_response,
                Err(_) => "I don't have specific information about this topic in my current knowledge base. Please share more details or relevant content for me to learn from.".to_string(),
            }
        } else {
            // Generate AI response based on learned insights
            match self.generate_ai_response_for_query(query, &insights, memory_system).await {
                Ok(ai_response) => ai_response,
                Err(_) => format!("Based on my analysis of {} sources, I found {} insights about your query. However, I'm having trouble generating a detailed response right now.", sources_analyzed, insights.len()),
            }
        };

        println!("✅ Brain AI Analysis completed in {}ms", processing_time);
        println!("  - Insights generated: {}", insights.len());
        println!("  - Related concepts: 0");
        println!("  - Sources analyzed: {}", sources_analyzed);
        println!("  - Confidence: {:.3}", confidence);

        Ok(BrainAnalysisResult {
            analysis,
            insights,
            confidence,
            metadata: BrainAnalysisMetadata {
                method: "Brain AI Comprehensive Analysis with Enhanced Learning".to_string(),
                sources_analyzed,
                quality_score: confidence,
                processing_time_ms: processing_time,
            },
        })
    }

    /// Check if query is GitHub-related (for BrainAIOrchestrator)
    fn is_github_related_brain(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        query_lower.contains("github.com") || 
        query_lower.contains("github") ||
        query_lower.contains("repository") ||
        query_lower.contains("repo")
    }

    /// Perform GitHub analysis
    async fn perform_github_analysis(&mut self, query: &str, memory_system: &mut MemorySystem) -> Result<Vec<String>, BrainError> {
        println!("    🔍 Analyzing GitHub content in query");
        
        // Extract GitHub URLs
        let github_urls: Vec<String> = query.split_whitespace()
            .filter(|word| word.contains("github.com"))
            .map(|s| s.to_string())
            .collect();
        
        let mut insights = Vec::new();
        
        for url in github_urls {
            match self.github_learning_engine.learn_from_repository(memory_system, &url).await {
                Ok(result) => {
                    insights.push(format!("GitHub Analysis: Analyzed repository {}", url));
                    insights.push(format!("Repository insights: {:?}", result));
                }
                Err(e) => {
                    println!("      ⚠️ Failed to analyze repository {}: {}", url, e);
                    insights.push(format!("GitHub Analysis: Failed to analyze {}", url));
                }
            }
        }
        
        if insights.is_empty() {
            insights.push("GitHub Analysis: No specific repositories found to analyze".to_string());
        }
        
        Ok(insights)
    }

    /// Perform pattern analysis
    async fn perform_pattern_analysis(&mut self, query: &str, memory_system: &mut MemorySystem) -> Result<Vec<String>, BrainError> {
        println!("    🔍 Performing pattern analysis");
        
        let mut patterns = Vec::new();
        
        // Use the pattern detector for actual pattern analysis from memory
        let pattern_result = self.pattern_detector.detect_patterns_from_memory(memory_system).await
            .map_err(|e| BrainError::ProcessingError(format!("Pattern detection error: {}", e)))?;
        
        for pattern in pattern_result.detected_patterns.iter().take(5) {
            patterns.push(format!("Pattern: {} (confidence: {:.2})", pattern.pattern_type, pattern.confidence));
        }
        
        // Use BPE segmenter for token analysis
        let segments = self.bpe_segmenter.segment_text(query);
        if segments.len() > 10 {
            patterns.push(format!("Complex query with {} semantic segments", segments.len()));
        }
        
        // Simple pattern detection
        if query.contains("function") || query.contains("class") || query.contains("struct") {
            patterns.push("Code pattern detected".to_string());
        }
        
        if query.contains("http") || query.contains("www") {
            patterns.push("URL pattern detected".to_string());
        }
        
        if query.len() > 100 {
            patterns.push("Long text pattern detected".to_string());
        }
        
        if patterns.is_empty() {
            patterns.push("No specific patterns detected".to_string());
        }
        
        Ok(patterns)
    }

    /// Perform semantic analysis
    async fn perform_semantic_analysis(&mut self, query: &str, _memory_system: &mut MemorySystem) -> Result<Vec<SemanticInsight>, BrainError> {
        println!("    🕸️ Performing semantic analysis");
        
        let mut insights = Vec::new();
        
        // Simple semantic analysis
        let words: Vec<&str> = query.split_whitespace().collect();
        let word_count = words.len();
        
        insights.push(SemanticInsight {
            content: format!("Query contains {} words", word_count),
            confidence: 0.8,
        });
        
        // Check for technical terms
        let tech_terms = ["api", "database", "server", "client", "function", "class", "algorithm"];
        let tech_count = words.iter()
            .filter(|word| tech_terms.contains(&word.to_lowercase().as_str()))
            .count();
        
        if tech_count > 0 {
            insights.push(SemanticInsight {
                content: format!("Technical content detected: {} technical terms", tech_count),
                confidence: 0.9,
            });
        }
        
        Ok(insights)
    }

    /// Extract GitHub references from query (for BrainAIOrchestrator)
    fn extract_github_references_brain(&self, query: &str) -> Vec<String> {
        let mut github_urls = Vec::new();
        
        // Look for GitHub URLs
        for word in query.split_whitespace() {
            if word.contains("github.com") {
                // Clean up the URL
                let clean_url = word.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '-' && c != '_' && c != '.');
                github_urls.push(clean_url.to_string());
            }
        }
        
        github_urls
    }

    /// Universal Learning Detection - Check if we should trigger learning from any content type
    fn should_trigger_universal_learning(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        
        // Direct learning triggers
        if query_lower.contains("learn from") ||
           query_lower.contains("analyze this") ||
           query_lower.contains("train on") ||
           query_lower.contains("study this") ||
           query_lower.contains("repository scanned:") ||
           query_lower.contains("content:") ||
           query_lower.contains("document:") ||
           query_lower.contains("file:") {
            return true;
        }

        // URL detection (any website, not just GitHub)
        if self.contains_urls(query) {
            return true;
        }

        // File/document patterns
        if self.contains_file_references(query) {
            return true;
        }

        // Code blocks or structured content
        if self.contains_code_or_structured_content(query) {
            return true;
        }

        // Repository or project references
        if self.contains_repository_references(query) {
            return true;
        }

        false
    }

    /// Detect URLs of any type (not just GitHub)
    fn contains_urls(&self, query: &str) -> bool {
        let url_patterns = vec![
            "http://", "https://", "www.", ".com", ".org", ".net", ".io", ".dev",
            "github.com", "gitlab.com", "bitbucket.org", "stackoverflow.com",
            "medium.com", "dev.to", "hackernoon.com", "substack.com",
            "youtube.com", "vimeo.com", "drive.google.com", "dropbox.com",
            "notion.so", "figma.com", "docs.google.com", "sheets.google.com"
        ];
        
        url_patterns.iter().any(|pattern| query.contains(pattern))
    }

    /// Detect file references and document types
    fn contains_file_references(&self, query: &str) -> bool {
        let file_extensions = vec![
            ".pdf", ".doc", ".docx", ".txt", ".md", ".rst",
            ".js", ".ts", ".py", ".rs", ".go", ".java", ".cpp", ".c",
            ".html", ".css", ".json", ".xml", ".yaml", ".yml",
            ".mp3", ".wav", ".mp4", ".avi", ".mov",
            ".png", ".jpg", ".jpeg", ".gif", ".svg",
            ".zip", ".tar", ".gz", ".rar"
        ];
        
        let file_keywords = vec![
            "file:", "document:", "attachment:", "upload:", "download:",
            "pdf", "document", "spreadsheet", "presentation", "video", "audio"
        ];
        
        file_extensions.iter().any(|ext| query.contains(ext)) ||
        file_keywords.iter().any(|keyword| query.to_lowercase().contains(keyword))
    }

    /// Detect code blocks or structured content
    fn contains_code_or_structured_content(&self, query: &str) -> bool {
        // Code block indicators
        if query.contains("```") || query.contains("```") {
            return true;
        }
        
        // Indented code patterns
        if query.lines().any(|line| line.starts_with("    ") || line.starts_with("\t")) {
            return true;
        }
        
        // Programming language keywords
        let code_indicators = vec![
            "function", "class", "import", "export", "const", "let", "var",
            "def", "async", "await", "return", "if", "else", "for", "while",
            "struct", "impl", "trait", "enum", "use", "mod",
            "public", "private", "protected", "static", "final",
            "#include", "#define", "namespace", "using"
        ];
        
        let query_lower = query.to_lowercase();
        code_indicators.iter().any(|indicator| {
            query_lower.contains(&format!(" {} ", indicator)) ||
            query_lower.starts_with(&format!("{} ", indicator))
        })
    }

    /// Detect repository or project references
    fn contains_repository_references(&self, query: &str) -> bool {
        let repo_indicators = vec![
            "repository", "repo", "project", "codebase", "source code",
            "open source", "library", "framework", "package", "module",
            "npm", "pip", "cargo", "maven", "gradle", "composer"
        ];
        
        let query_lower = query.to_lowercase();
        repo_indicators.iter().any(|indicator| query_lower.contains(indicator))
    }

    /// Process universal learning from any content type
    async fn process_universal_learning(
        &mut self,
        query: &str,
        memory_system: &mut MemorySystem,
        concept_graph: &mut ConceptGraphManager,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("🧠 Universal Learning Engine: Processing content for learning");
        
        let mut learning_results = Vec::new();
        
        // 1. Extract and categorize content
        let content_analysis = self.analyze_content_type(query).await;
        println!("  📋 Content type: {}", content_analysis.content_type);
        println!("  📊 Confidence: {:.2}", content_analysis.confidence);
        
        // 2. Process based on content type
        match content_analysis.content_type.as_str() {
            "github_repository" => {
                if let Some(repo_url) = self.extract_github_references_brain(query).first() {
                    learning_results.extend(self.learn_from_github_repository(repo_url, memory_system).await?);
                }
            },
            "web_url" => {
                let urls = self.extract_urls(query);
                for url in urls {
                    learning_results.extend(self.learn_from_web_content(&url, memory_system).await?);
                }
            },
            "code_snippet" => {
                learning_results.extend(self.learn_from_code_content(query, memory_system).await?);
            },
            "document_content" => {
                learning_results.extend(self.learn_from_document_content(query, memory_system).await?);
            },
            "structured_data" => {
                learning_results.extend(self.learn_from_structured_data(query, memory_system).await?);
            },
            _ => {
                // Default: treat as general text content
                learning_results.extend(self.learn_from_text_content(query, memory_system).await?);
            }
        }
        
        // 3. Update concept graph with learned concepts
        self.update_concept_graph_from_learning(&learning_results, concept_graph).await?;
        
        println!("✅ Universal Learning completed: {} insights generated", learning_results.len());
        Ok(learning_results)
    }

    /// Analyze what type of content we're dealing with
    async fn analyze_content_type(&self, query: &str) -> ContentAnalysis {
        // GitHub repository detection
        if !self.extract_github_references_brain(query).is_empty() {
            return ContentAnalysis {
                content_type: "github_repository".to_string(),
                confidence: 0.9,
                metadata: HashMap::new(),
            };
        }
        
        // Web URL detection
        if self.contains_urls(query) {
            return ContentAnalysis {
                content_type: "web_url".to_string(),
                confidence: 0.8,
                metadata: HashMap::new(),
            };
        }
        
        // Code content detection
        if self.contains_code_or_structured_content(query) {
            return ContentAnalysis {
                content_type: "code_snippet".to_string(),
                confidence: 0.85,
                metadata: HashMap::new(),
            };
        }
        
        // File/document detection
        if self.contains_file_references(query) {
            return ContentAnalysis {
                content_type: "document_content".to_string(),
                confidence: 0.7,
                metadata: HashMap::new(),
            };
        }
        
        // Default to text content
        ContentAnalysis {
            content_type: "text_content".to_string(),
            confidence: 0.6,
            metadata: HashMap::new(),
        }
    }

    /// Extract URLs from text
    fn extract_urls(&self, text: &str) -> Vec<String> {
        let mut urls = Vec::new();
        
        // Simple URL extraction
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in words {
            if word.starts_with("http://") || word.starts_with("https://") || word.contains("www.") {
                // Clean up the URL (remove trailing punctuation)
                let clean_url = word.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '-' && c != '_' && c != '.' && c != '?' && c != '=' && c != '&');
                urls.push(clean_url.to_string());
            }
        }
        
        urls
    }

    /// Learn from GitHub repository
    async fn learn_from_github_repository(
        &mut self,
        repo_url: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("  🔍 Learning from GitHub repository: {}", repo_url);
        
        // Use the existing GitHub learning engine
        match self.github_learning_engine.learn_from_repository(memory_system, repo_url).await {
            Ok(result) => {
                let mut knowledge = Vec::new();
                
                // Add the main summary as knowledge
                knowledge.push(RetrievedKnowledge {
                    content: result.summary.clone(),
                    knowledge_type: "github_repository_summary".to_string(),
                    relevance_score: 0.9,
                    source: format!("GitHub Repository: {}", repo_url),
                    timestamp: chrono::Utc::now(),
                });
                
                // Add each key insight as separate knowledge
                for insight in result.key_insights {
                    knowledge.push(RetrievedKnowledge {
                        content: insight,
                        knowledge_type: "github_insight".to_string(),
                        relevance_score: 0.8,
                        source: format!("GitHub Repository: {}", repo_url),
                        timestamp: chrono::Utc::now(),
                    });
                }
                
                // Add metadata as knowledge
                knowledge.push(RetrievedKnowledge {
                    content: format!(
                        "Repository Analysis: {} files processed, {} concepts discovered, {} memory entries created in {}ms",
                        result.files_processed, result.concepts_discovered, 
                        result.memory_entries_created, result.learning_time_ms
                    ),
                    knowledge_type: "github_metadata".to_string(),
                    relevance_score: 0.7,
                    source: format!("GitHub Repository: {}", repo_url),
                    timestamp: chrono::Utc::now(),
                });
                
                Ok(knowledge)
            }
            Err(e) => {
                println!("    ⚠️ GitHub learning failed: {}", e);
                Ok(Vec::new())
            }
        }
    }

    /// Learn from web content
    async fn learn_from_web_content(
        &mut self,
        url: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("  🌐 Learning from web content: {}", url);
        
        // First, try to fetch the actual web content
        match self.fetch_web_content(url).await {
            Ok(content) => {
                println!("    ✅ Successfully fetched web content ({} characters)", content.len());
                
                // Use OpenAI to analyze the web content
                match self.analyze_web_content_with_ai(&content, url).await {
                    Ok(ai_insights) => {
                        println!("    🧠 AI analysis completed: {} insights generated", ai_insights.len());
                        self.store_learning_insights("web_content_ai", &ai_insights, memory_system, url).await
                    }
                    Err(e) => {
                        println!("    ⚠️ AI analysis failed: {}, using basic analysis", e);
                        // Fallback to basic content analysis
                        let basic_concepts = self.extract_key_concepts(&content);
                        let mut insights = vec![format!("Web content from {}: {} characters", url, content.len())];
                        insights.extend(basic_concepts.into_iter().map(|c| format!("Key concept: {}", c)));
                        self.store_learning_insights("web_content_basic", &insights, memory_system, url).await
                    }
                }
            }
            Err(e) => {
                println!("    ⚠️ Failed to fetch web content: {}, using URL analysis", e);
                // Fallback to URL analysis
                let concepts = self.extract_key_concepts_from_url(url);
                self.store_learning_insights("web_url_analysis", &concepts, memory_system, url).await
            }
        }
    }

    /// Learn from code content
    async fn learn_from_code_content(
        &mut self,
        code: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("  💻 Learning from code content");
        
        let language = self.detect_programming_language(code);
        let functions = self.extract_functions_from_code(code, &language);
        let imports = self.extract_imports_from_code(code, &language);
        let data_structures = self.extract_data_structures_from_code(code, &language);
        
        let mut insights = Vec::new();
        insights.push(format!("Programming language: {}", language));
        insights.extend(functions.into_iter().map(|f| format!("Function: {}", f)));
        insights.extend(imports.into_iter().map(|i| format!("Import: {}", i)));
        insights.extend(data_structures.into_iter().map(|d| format!("Data structure: {}", d)));
        
        self.store_learning_insights("code_analysis", &insights, memory_system, "Code snippet").await
    }

    /// Learn from document content
    async fn learn_from_document_content(
        &mut self,
        content: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("  📄 Learning from document content");
        
        let concepts = self.extract_key_concepts(content);
        self.store_learning_insights("document_analysis", &concepts, memory_system, "Document").await
    }

    /// Learn from structured data
    async fn learn_from_structured_data(
        &mut self,
        data: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("  📊 Learning from structured data");
        
        let mut insights = Vec::new();
        
        // Try to parse as JSON
        if data.trim().starts_with('{') || data.trim().starts_with('[') {
            insights.push("Data format: JSON".to_string());
            // Extract keys if it's JSON
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(data) {
                insights.push(format!("JSON structure analyzed: {} fields", self.count_json_fields(&parsed)));
            }
        }
        
        self.store_learning_insights("structured_data", &insights, memory_system, "Structured data").await
    }

    /// Learn from general text content
    async fn learn_from_text_content(
        &mut self,
        text: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("  📝 Learning from text content");
        
        let concepts = self.extract_key_concepts(text);
        self.store_learning_insights("text_analysis", &concepts, memory_system, "Text content").await
    }

    /// Store learning insights in memory
    async fn store_learning_insights(
        &mut self,
        learning_type: &str,
        insights: &[String],
        memory_system: &mut MemorySystem,
        source: &str,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        let mut knowledge = Vec::new();
        
        for insight in insights {
            // Store in memory system using the correct method
            memory_system.learn(insight.clone(), Priority::Medium)?;
            
            // Add to knowledge results
            knowledge.push(RetrievedKnowledge {
                content: insight.clone(),
                knowledge_type: learning_type.to_string(),
                relevance_score: 0.7,
                source: source.to_string(),
                timestamp: chrono::Utc::now(),
            });
        }
        
        Ok(knowledge)
    }

    /// Update concept graph with learned concepts
    async fn update_concept_graph_from_learning(
        &mut self,
        learning_results: &[RetrievedKnowledge],
        concept_graph: &mut ConceptGraphManager,
    ) -> Result<(), BrainError> {
        for result in learning_results {
            let concepts = self.extract_key_concepts(&result.content);
            for concept in concepts {
                // Create a concept node and add it to the graph
                let concept_node = crate::concept_graph::ConceptNode::new(
                    crate::concept_graph::ConceptType::Entity,
                    concept.clone(),
                    0.6,
                    None,
                );
                let _ = concept_graph.create_concept(concept_node).await;
            }
        }
        Ok(())
    }

    /// Extract key concepts from text
    fn extract_key_concepts(&self, text: &str) -> Vec<String> {
        // Simple keyword extraction
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut concepts = Vec::new();
        
        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if clean_word.len() > 3 && !self.is_stop_word(&clean_word) {
                concepts.push(clean_word);
            }
        }
        
        // Remove duplicates and limit
        concepts.sort();
        concepts.dedup();
        concepts.truncate(10);
        concepts
    }

    /// Extract key concepts from URL
    fn extract_key_concepts_from_url(&self, url: &str) -> Vec<String> {
        let mut concepts = Vec::new();
        
        // Extract domain
        if let Some(domain_start) = url.find("://") {
            if let Some(domain_end) = url[domain_start + 3..].find('/') {
                let domain = &url[domain_start + 3..domain_start + 3 + domain_end];
                concepts.push(format!("Website: {}", domain));
            }
        }
        
        // Extract path segments
        if let Some(path_start) = url.find("://") {
            if let Some(path_start) = url[path_start..].find('/') {
                let path = &url[path_start + 1..];
                let segments: Vec<&str> = path.split('/').collect();
                for segment in segments {
                    if segment.len() > 2 && !segment.contains('?') && !segment.contains('&') {
                        concepts.push(format!("Topic: {}", segment.replace('-', " ").replace('_', " ")));
                    }
                }
            }
        }
        
        concepts.push(format!("URL analyzed: {}", url));
        concepts
    }

    /// Fetch web content from URL
    async fn fetch_web_content(&self, url: &str) -> Result<String, BrainError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Brain AI Learning Bot 1.0")
            .build()
            .map_err(|e| BrainError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| BrainError::NetworkError(format!("Failed to fetch URL: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            return Err(BrainError::NetworkError(format!(
                "HTTP error {}: {}",
                status,
                status.canonical_reason().unwrap_or("Unknown error")
            )));
        }

        let content = response
            .text()
            .await
            .map_err(|e| BrainError::NetworkError(format!("Failed to read response body: {}", e)))?;

        // Basic content filtering - remove HTML tags and clean up
        let cleaned_content = self.clean_html_content(&content);
        
        // Limit content size to avoid token limits
        if cleaned_content.len() > 50000 {
            Ok(cleaned_content[..50000].to_string() + "\n\n[Content truncated...]")
        } else {
            Ok(cleaned_content)
        }
    }

    /// Clean HTML content and extract text
    fn clean_html_content(&self, html: &str) -> String {
        // Simple HTML tag removal and content extraction
        let mut cleaned = html.to_string();
        
        // Remove script and style tags entirely
        while let Some(start) = cleaned.find("<script") {
            if let Some(end) = cleaned[start..].find("</script>") {
                cleaned.replace_range(start..start + end + 9, "");
            } else {
                break;
            }
        }
        
        while let Some(start) = cleaned.find("<style") {
            if let Some(end) = cleaned[start..].find("</style>") {
                cleaned.replace_range(start..start + end + 8, "");
            } else {
                break;
            }
        }
        
        // Remove HTML tags
        let mut result = String::new();
        let mut in_tag = false;
        
        for char in cleaned.chars() {
            match char {
                '<' => in_tag = true,
                '>' => {
                    in_tag = false;
                    result.push(' '); // Replace tags with spaces
                }
                _ if !in_tag => result.push(char),
                _ => {} // Skip characters inside tags
            }
        }
        
        // Clean up whitespace
        result
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    /// Analyze web content using OpenAI
    async fn analyze_web_content_with_ai(&self, content: &str, url: &str) -> Result<Vec<String>, BrainError> {
        // Get OpenAI API key from environment
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| BrainError::ConfigError("OPENAI_API_KEY not found".to_string()))?;

        let client = reqwest::Client::new();
        
        // Create a focused prompt for web content analysis
        let analysis_prompt = format!(
            "Analyze this web content from {} and extract key insights, concepts, and learnings. \
            Focus on practical information, main topics, technologies mentioned, and actionable insights. \
            Provide 5-10 concise bullet points of the most important learnings.\n\nContent:\n{}",
            url,
            // Truncate content for API call
            if content.len() > 8000 { &content[..8000] } else { content }
        );

        let request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: Some(1000),
            temperature: 0.3,
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: "You are a knowledge extraction expert. Analyze web content and extract key insights, concepts, and learnings in a structured format.".to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: analysis_prompt,
                },
            ],
            stream: false,
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| BrainError::NetworkError(format!("OpenAI API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrainError::NetworkError(format!(
                "OpenAI API error {}: {}",
                status,
                error_text
            )));
        }

        let openai_response: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| BrainError::NetworkError(format!("Failed to parse OpenAI response: {}", e)))?;

        if let Some(choice) = openai_response.choices.first() {
            let analysis = &choice.message.content;
            
            // Parse the analysis into individual insights
            let insights: Vec<String> = analysis
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();

            Ok(insights)
        } else {
            Err(BrainError::ProcessingError("No response from OpenAI".to_string()))
        }
    }

    /// Detect programming language from code
    fn detect_programming_language(&self, code: &str) -> String {
        if code.contains("fn ") || code.contains("impl ") || code.contains("struct ") {
            "Rust".to_string()
        } else if code.contains("def ") || code.contains("import ") || code.contains("from ") {
            "Python".to_string()
        } else if code.contains("function ") || code.contains("const ") || code.contains("let ") {
            "JavaScript".to_string()
        } else if code.contains("class ") || code.contains("interface ") || code.contains("type ") {
            "TypeScript".to_string()
        } else if code.contains("public class") || code.contains("private ") || code.contains("import java") {
            "Java".to_string()
        } else if code.contains("#include") || code.contains("int main") || code.contains("std::") {
            "C++".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Extract functions from code
    fn extract_functions_from_code(&self, code: &str, language: &str) -> Vec<String> {
        let mut functions = Vec::new();
        
        for line in code.lines() {
            let line = line.trim();
            match language {
                "Rust" => {
                    if let Some(func_name) = self.extract_function_name_rust(line) {
                        functions.push(func_name);
                    }
                },
                "Python" => {
                    if let Some(func_name) = self.extract_function_name_python(line) {
                        functions.push(func_name);
                    }
                },
                "JavaScript" | "TypeScript" => {
                    if let Some(func_name) = self.extract_function_name_js(line) {
                        functions.push(func_name);
                    }
                },
                _ => {}
            }
        }
        
        functions
    }

    /// Extract imports from code
    fn extract_imports_from_code(&self, code: &str, language: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        for line in code.lines() {
            let line = line.trim();
            match language {
                "Rust" => {
                    if line.starts_with("use ") {
                        imports.push(line.to_string());
                    }
                },
                "Python" => {
                    if line.starts_with("import ") || line.starts_with("from ") {
                        imports.push(line.to_string());
                    }
                },
                "JavaScript" | "TypeScript" => {
                    if line.starts_with("import ") || line.contains("require(") {
                        imports.push(line.to_string());
                    }
                },
                _ => {}
            }
        }
        
        imports
    }

    /// Extract data structures from code
    fn extract_data_structures_from_code(&self, code: &str, language: &str) -> Vec<String> {
        let mut structures = Vec::new();
        
        for line in code.lines() {
            let line = line.trim();
            match language {
                "Rust" => {
                    if line.starts_with("struct ") || line.starts_with("enum ") || line.starts_with("trait ") {
                        structures.push(line.to_string());
                    }
                },
                "Python" => {
                    if line.starts_with("class ") {
                        structures.push(line.to_string());
                    }
                },
                "JavaScript" | "TypeScript" => {
                    if line.starts_with("class ") || line.starts_with("interface ") || line.starts_with("type ") {
                        structures.push(line.to_string());
                    }
                },
                _ => {}
            }
        }
        
        structures
    }

    /// Extract function name from Rust code
    fn extract_function_name_rust(&self, line: &str) -> Option<String> {
        if line.starts_with("fn ") || line.contains(" fn ") {
            if let Some(start) = line.find("fn ") {
                if let Some(end) = line[start + 3..].find('(') {
                    let name = line[start + 3..start + 3 + end].trim();
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    /// Extract function name from Python code
    fn extract_function_name_python(&self, line: &str) -> Option<String> {
        if line.starts_with("def ") {
            if let Some(end) = line.find('(') {
                let name = line[4..end].trim();
                return Some(name.to_string());
            }
        }
        None
    }

    /// Extract function name from JavaScript/TypeScript code
    fn extract_function_name_js(&self, line: &str) -> Option<String> {
        if line.starts_with("function ") {
            if let Some(end) = line.find('(') {
                let name = line[9..end].trim();
                return Some(name.to_string());
            }
        } else if line.contains(" = ") && (line.contains("function") || line.contains("=>")) {
            if let Some(eq_pos) = line.find(" = ") {
                let name = line[..eq_pos].trim();
                if let Some(last_space) = name.rfind(' ') {
                    return Some(name[last_space + 1..].to_string());
                } else {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    /// Check if word is a stop word
    fn is_stop_word(&self, word: &str) -> bool {
        let stop_words = vec![
            "the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
            "by", "from", "up", "about", "into", "through", "during", "before",
            "after", "above", "below", "between", "among", "this", "that", "these",
            "those", "i", "you", "he", "she", "it", "we", "they", "me", "him",
            "her", "us", "them", "my", "your", "his", "her", "its", "our", "their"
        ];
        stop_words.contains(&word)
    }

    /// Count JSON fields
    fn count_json_fields(&self, value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Object(obj) => obj.len(),
            serde_json::Value::Array(arr) => arr.len(),
            _ => 1,
        }
    }

    /// Generate AI response for query using OpenAI based on insights and memory
    async fn generate_ai_response_for_query(
        &self,
        query: &str,
        insights: &[BrainInsight],
        memory_system: &mut MemorySystem,
    ) -> Result<String, BrainError> {
        // Get OpenAI API key from environment
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| BrainError::ConfigError("OPENAI_API_KEY not found".to_string()))?;

        let client = reqwest::Client::new();
        
        // Retrieve relevant memory context
        let memory_context = self.get_relevant_memory_context(query, memory_system).await?;
        
        // Build context from insights and memory
        let mut context_parts = Vec::new();
        
        if !insights.is_empty() {
            context_parts.push("**Recent Analysis Insights:**".to_string());
            for insight in insights.iter().take(10) {
                context_parts.push(format!("- {} (Source: {}, Confidence: {:.2})", 
                    insight.content, insight.source, insight.confidence));
            }
        }
        
        if !memory_context.is_empty() {
            context_parts.push("\n**Relevant Knowledge from Memory:**".to_string());
            for memory in memory_context.iter().take(5) {
                context_parts.push(format!("- {}", memory));
            }
        }
        
        let context = if context_parts.is_empty() {
            "No specific context available.".to_string()
        } else {
            context_parts.join("\n")
        };

        // Create a comprehensive prompt for intelligent response generation
        let system_prompt = "You are Brain AI, an advanced AI system with learning capabilities. You analyze information, learn from content, and provide intelligent responses based on your knowledge. Be helpful, accurate, and conversational. When you have learned from specific content, reference it naturally in your response.".to_string();
        
        let user_prompt = format!(
            "Query: {}\n\nContext and Knowledge:\n{}\n\nPlease provide a comprehensive, helpful response to the query based on the available context and knowledge. If you've learned from specific content, reference it naturally. Be conversational and informative.",
            query, context
        );

        let request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: Some(800),
            temperature: 0.7,
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            stream: false,
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| BrainError::NetworkError(format!("OpenAI API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrainError::NetworkError(format!(
                "OpenAI API error {}: {}",
                status,
                error_text
            )));
        }

        let openai_response: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| BrainError::NetworkError(format!("Failed to parse OpenAI response: {}", e)))?;

        if let Some(choice) = openai_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(BrainError::ProcessingError("No response from OpenAI".to_string()))
        }
    }

    /// Get relevant memory context for a query
    async fn get_relevant_memory_context(
        &self,
        query: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<Vec<String>, BrainError> {
        // Simple memory retrieval - get recent and relevant memories
        let memories = memory_system.find_related_memories(query, 5)
            .map_err(|e| BrainError::ProcessingError(format!("Memory search failed: {}", e)))?;
        
        let mut context: Vec<String> = Vec::new();
        
        // Extract content from working memory results
        for item in memories.working_results.iter().take(3) {
            context.push(item.content.clone());
        }
        
        // Extract content from episodic memory results
        for event in memories.episodic_results.iter().take(2) {
            context.push(event.content.clone());
        }
        
        // Extract content from semantic memory results
        for concept in memories.semantic_results.iter().take(2) {
            context.push(format!("{}: {}", concept.name, concept.description));
        }
        
        Ok(context)
    }
}

/// Semantic insight from analysis
#[derive(Debug, Clone)]
pub struct SemanticInsight {
    pub content: String,
    pub confidence: f64,
}

/// Content analysis result for universal learning
#[derive(Debug, Clone)]
pub struct ContentAnalysis {
    pub content_type: String,
    pub confidence: f64,
    pub metadata: HashMap<String, String>,
}