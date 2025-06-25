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
                            source: format!("Brain AI - {}", insight.evidence.join(", ")),
                            timestamp: Utc::now(),
                        });
                    }
                    
                    // Add related concepts as knowledge
                    for concept in brain_analysis.related_concepts {
                        retrieved_knowledge.push(RetrievedKnowledge {
                            content: format!("Related concept: {}", concept),
                            knowledge_type: "related_concept".to_string(),
                            relevance_score: 0.6,
                            source: "Brain AI Concept Analysis".to_string(),
                            timestamp: Utc::now(),
                        });
                    }
                    
                    // Add patterns as knowledge
                    for pattern in brain_analysis.patterns {
                        retrieved_knowledge.push(RetrievedKnowledge {
                            content: format!("Detected pattern: {}", pattern),
                            knowledge_type: "pattern".to_string(),
                            relevance_score: 0.7,
                            source: "Brain AI Pattern Detection".to_string(),
                            timestamp: Utc::now(),
                        });
                    }
                    
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
        if let Some(ref brain_ai) = self.brain_ai_orchestrator {
            brain_ai.learning_orchestrator.get_learning_analytics().await
        } else {
            LearningAnalytics::default()
        }
    }

    pub async fn start_learning_session(&mut self, objective: String) -> String {
        if let Some(ref mut brain_ai) = self.brain_ai_orchestrator {
            brain_ai.learning_orchestrator.start_learning_session(objective).await
        } else {
            format!("session_{}", chrono::Utc::now().timestamp())
        }
    }

    pub async fn end_learning_session(&mut self, session_id: String) -> LearningSessionSummary {
        if let Some(ref mut brain_ai) = self.brain_ai_orchestrator {
            brain_ai.learning_orchestrator.end_learning_session(session_id).await
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
        if let Some(ref brain_ai) = self.brain_ai_orchestrator {
            brain_ai.learning_orchestrator.get_current_knowledge_gaps().await
        } else {
            Vec::new()
        }
    }

    pub async fn get_meta_learning_recommendations(&self) -> Vec<String> {
        if let Some(ref brain_ai) = self.brain_ai_orchestrator {
            brain_ai.learning_orchestrator.get_meta_learning_recommendations().await
        } else {
            vec!["Initialize Brain AI for recommendations".to_string()]
        }
    }

    pub async fn get_performance_trends(&self) -> PerformanceTrends {
        if let Some(ref brain_ai) = self.brain_ai_orchestrator {
            brain_ai.learning_orchestrator.get_performance_trends().await
        } else {
            PerformanceTrends::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExpandedConcept {
    content: String,
    concept_type: ConceptType,
    relevance_score: f64,
    context_path: Vec<String>,
    depth: usize,
}

/// Brain AI Orchestrator - True delegation to Brain AI's actual capabilities
pub struct BrainAIOrchestrator {
    /// GitHub learning engine for repository analysis
    github_learning_engine: GitHubLearningEngine,
    /// Pattern detector for insight extraction
    pattern_detector: PatternDetector,
    /// BPE segmenter for text analysis
    #[allow(dead_code)]
    bpe_segmenter: BpeSegmenter,
    /// Configuration for analysis depth
    analysis_config: BrainAnalysisConfig,
    /// Enhanced LLM training integration system
    learning_orchestrator: BrainLearningOrchestrator,
}

/// Configuration for Brain AI analysis depth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainAnalysisConfig {
    /// Enable deep GitHub repository analysis
    pub enable_github_analysis: bool,
    /// Enable pattern extraction and insight generation
    pub enable_pattern_analysis: bool,
    /// Enable concept graph traversal and relationship discovery
    pub enable_concept_analysis: bool,
    /// Enable semantic memory analysis
    pub enable_semantic_analysis: bool,
    /// Maximum analysis depth
    pub max_analysis_depth: usize,
    /// Minimum confidence threshold for results
    pub min_confidence_threshold: f64,
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

/// Rich analysis result from Brain AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainAnalysisResult {
    /// Detailed analysis content
    pub analysis: String,
    /// Structured insights discovered
    pub insights: Vec<BrainInsight>,
    /// Confidence in the analysis
    pub confidence: f64,
    /// Analysis metadata
    pub metadata: BrainAnalysisMetadata,
    /// Related concepts discovered
    pub related_concepts: Vec<String>,
    /// Patterns identified
    pub patterns: Vec<String>,
}

/// Individual insight from Brain AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInsight {
    /// Type of insight
    pub insight_type: String,
    /// Insight content
    pub content: String,
    /// Confidence in this insight
    pub confidence: f64,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// Metadata about Brain AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainAnalysisMetadata {
    /// Analysis method used
    pub method: String,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Number of sources analyzed
    pub sources_analyzed: usize,
    /// Complexity score of the analysis
    pub complexity_score: f64,
    /// Quality assessment
    pub quality_score: f64,
}

impl BrainAIOrchestrator {
    /// Create new Brain AI orchestrator
    pub fn new() -> Result<Self, BrainError> {
        let github_token = env::var("GITHUB_TOKEN").ok();
        let github_config = GitHubLearningConfig::default();
        let github_learning_engine = GitHubLearningEngine::new(github_token, Some(github_config));
        
        let pattern_detector = PatternDetector::new();
        
        let bpe_config = BpeConfig::default();
        let bpe_segmenter = BpeSegmenter::new(bpe_config);
        
        let analysis_config = BrainAnalysisConfig::default();
        
        Ok(Self {
            github_learning_engine,
            pattern_detector,
            bpe_segmenter,
            analysis_config,
            learning_orchestrator: BrainLearningOrchestrator::new(),
        })
    }
    
    /// Perform comprehensive Brain AI analysis of a query
    pub async fn analyze_query(
        &mut self,
        query: &str,
        memory_system: &mut MemorySystem,
        concept_graph: &mut ConceptGraphManager,
    ) -> Result<BrainAnalysisResult, BrainError> {
        let start_time = std::time::Instant::now();
        println!("🧠 Brain AI Orchestrator: Starting comprehensive analysis of '{}'", query);
        
        let mut insights = Vec::new();
        let mut related_concepts = Vec::new();
        let mut patterns = Vec::new();
        let mut sources_analyzed = 0;
        let mut analysis_parts = Vec::new();
        
        // 1. GitHub Repository Analysis (if query mentions GitHub/repos)
        if self.analysis_config.enable_github_analysis && self.is_github_related(query) {
            if let Ok(github_analysis) = self.perform_github_analysis(query, memory_system).await {
                analysis_parts.push(format!("GitHub Repository Analysis: {}", github_analysis.analysis));
                insights.extend(github_analysis.insights);
                related_concepts.extend(github_analysis.related_concepts);
                sources_analyzed += github_analysis.metadata.sources_analyzed;
            }
        }
        
        // 2. Pattern Analysis and Insight Extraction
        if self.analysis_config.enable_pattern_analysis {
            if let Ok(pattern_analysis) = self.perform_pattern_analysis(query, memory_system).await {
                analysis_parts.push(format!("Pattern Analysis: {}", pattern_analysis.analysis));
                insights.extend(pattern_analysis.insights);
                patterns.extend(pattern_analysis.patterns);
                sources_analyzed += pattern_analysis.metadata.sources_analyzed;
            }
        }
        
        // 3. Concept Graph Analysis
        if self.analysis_config.enable_concept_analysis {
            if let Ok(concept_analysis) = self.perform_concept_analysis(query, concept_graph).await {
                analysis_parts.push(format!("Concept Analysis: {}", concept_analysis.analysis));
                insights.extend(concept_analysis.insights);
                related_concepts.extend(concept_analysis.related_concepts);
                sources_analyzed += concept_analysis.metadata.sources_analyzed;
            }
        }
        
        // 4. Semantic Memory Analysis
        if self.analysis_config.enable_semantic_analysis {
            if let Ok(semantic_analysis) = self.perform_semantic_analysis(query, memory_system).await {
                analysis_parts.push(format!("Semantic Analysis: {}", semantic_analysis.analysis));
                insights.extend(semantic_analysis.insights);
                sources_analyzed += semantic_analysis.metadata.sources_analyzed;
            }
        }
        
        // 5. Combine and synthesize results
        let mut combined_analysis = if analysis_parts.is_empty() {
            format!("Based on my analysis of '{}', I need to gather more information from my knowledge systems.", query)
        } else {
            format!("Comprehensive Analysis of '{}': {}", query, analysis_parts.join(" "))
        };
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        // Calculate overall confidence and quality
        let confidence = if insights.is_empty() { 0.3 } else {
            insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len() as f64
        };
        
        let complexity_score = self.calculate_complexity_score(query, &insights);
        let quality_score = self.calculate_quality_score(&insights, sources_analyzed);
        
        println!("✅ Brain AI Analysis completed in {}ms", processing_time);
        println!("  - Insights generated: {}", insights.len());
        println!("  - Related concepts: {}", related_concepts.len());
        println!("  - Sources analyzed: {}", sources_analyzed);
        println!("  - Confidence: {:.3}", confidence);
        
        // Enhanced LLM Training Integration - Process query for learning
        let learning_opportunities = self.learning_orchestrator
            .process_query_for_learning(query, confidence, quality_score, sources_analyzed)
            .await
            .unwrap_or_else(|_| LearningOpportunities {
                identified_gaps: Vec::new(),
                follow_up_questions: Vec::new(),
                suggested_improvements: Vec::new(),
                learning_recommendations: Vec::new(),
            });

        // If learning opportunities were identified, enhance the analysis
        if !learning_opportunities.identified_gaps.is_empty() || !learning_opportunities.follow_up_questions.is_empty() {
            let mut enhanced_analysis = combined_analysis.clone();
            
            if !learning_opportunities.identified_gaps.is_empty() {
                enhanced_analysis.push_str(&format!("\n\n🧠 **Learning Opportunities Identified:** {} knowledge gaps detected for deeper understanding.", learning_opportunities.identified_gaps.len()));
            }
            
            if !learning_opportunities.follow_up_questions.is_empty() {
                enhanced_analysis.push_str(&format!("\n\n❓ **Follow-up Questions Generated:** {} clarifying questions to enhance knowledge.", learning_opportunities.follow_up_questions.len()));
            }
            
            combined_analysis = enhanced_analysis;
        }

        Ok(BrainAnalysisResult {
            analysis: combined_analysis,
            insights,
            confidence,
            metadata: BrainAnalysisMetadata {
                method: "Brain AI Comprehensive Analysis with Enhanced Learning".to_string(),
                processing_time_ms: processing_time,
                sources_analyzed,
                complexity_score,
                quality_score,
            },
            related_concepts,
            patterns,
        })
    }
    
    /// Perform GitHub repository analysis
    async fn perform_github_analysis(
        &self,
        query: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<BrainAnalysisResult, BrainError> {
        println!("  🔍 Performing GitHub analysis...");
        
        // Extract GitHub URLs or repository names from query
        let github_urls = self.extract_github_references(query);
        
        if github_urls.is_empty() {
            // Search for GitHub-related content in memory
            return self.analyze_github_memory_content(query, memory_system).await;
        }
        
        let mut all_insights = Vec::new();
        let mut all_concepts = Vec::new();
        let mut sources_count = 0;
        let mut analysis_content = Vec::new();
        
        // Analyze each GitHub reference
        for github_url in github_urls {
            if let Ok(learning_result) = self.github_learning_engine
                .learn_from_repository(memory_system, &github_url).await {
                
                sources_count += learning_result.files_processed;
                
                // Create detailed analysis from learning result
                let detailed_analysis = format!(
                    "Repository '{}' Analysis: Processed {} files ({} bytes total) in {}ms. Discovered {} concepts and created {} memory entries. Key insights: {}. Summary: {}",
                    learning_result.repository,
                    learning_result.files_processed,
                    learning_result.total_content_size,
                    learning_result.learning_time_ms,
                    learning_result.concepts_discovered,
                    learning_result.memory_entries_created,
                    learning_result.key_insights.join("; "),
                    learning_result.summary
                );
                
                analysis_content.push(detailed_analysis);
                
                // Convert key insights to Brain insights
                for (i, insight) in learning_result.key_insights.iter().enumerate() {
                    all_insights.push(BrainInsight {
                        insight_type: "repository_insight".to_string(),
                        content: insight.clone(),
                        confidence: 0.8 + (i as f64 * 0.05).min(0.95), // Higher confidence for first insights
                        evidence: vec![format!("Analyzed {} files from {}", learning_result.files_processed, learning_result.repository)],
                    });
                }
                
                // Extract concepts from repository name and description
                all_concepts.push(learning_result.repository.clone());
                if learning_result.concepts_discovered > 0 {
                    all_concepts.push(format!("{}_concepts", learning_result.repository.replace('/', "_")));
                }
            }
        }
        
        let combined_analysis = if analysis_content.is_empty() {
            "No GitHub repositories found to analyze in the query.".to_string()
        } else {
            analysis_content.join(" ")
        };
        
        Ok(BrainAnalysisResult {
            analysis: combined_analysis,
            insights: all_insights.clone(),
            confidence: if sources_count > 0 { 0.85 } else { 0.3 },
            metadata: BrainAnalysisMetadata {
                method: "GitHub Repository Analysis".to_string(),
                processing_time_ms: 0, // Will be calculated by caller
                sources_analyzed: sources_count,
                complexity_score: (sources_count as f64 / 50.0).min(1.0),
                quality_score: if all_insights.is_empty() { 0.3 } else { 0.8 },
            },
            related_concepts: all_concepts,
            patterns: vec!["repository_structure".to_string(), "code_patterns".to_string()],
        })
    }
    
    /// Analyze GitHub-related content in memory
    async fn analyze_github_memory_content(
        &self,
        query: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<BrainAnalysisResult, BrainError> {
        println!("  📚 Analyzing GitHub-related memory content...");
        
        let mut insights = Vec::new();
        let mut concepts = Vec::new();
        let mut sources_count = 0;
        
        // Search for GitHub-related memories
        let github_keywords = vec!["github", "repository", "repo", "code", "project"];
        
        for keyword in github_keywords {
            let working_query = WorkingMemoryQuery {
                content_pattern: Some(keyword.to_string()),
                limit: Some(10),
                min_importance: Some(0.1),
                ..Default::default()
            };
            
            if let Ok(working_items) = memory_system.query_working(&working_query) {
                for item in working_items {
                    if self.is_relevant_to_query(&item.content, query) {
                        sources_count += 1;
                        
                        insights.push(BrainInsight {
                            insight_type: "memory_insight".to_string(),
                            content: item.content.clone(),
                            confidence: (item.priority as u8 as f64) / 4.0,
                            evidence: vec![format!("Retrieved from working memory with keyword '{}'", keyword)],
                        });
                        
                        // Extract concepts from content
                        let item_concepts = self.extract_concepts_from_text(&item.content);
                        concepts.extend(item_concepts);
                    }
                }
            }
        }
        
        let analysis = if insights.is_empty() {
            "No GitHub-related content found in memory systems.".to_string()
        } else {
            format!("Found {} GitHub-related memories in my knowledge systems: {}", 
                   insights.len(),
                   insights.iter().take(3).map(|i| i.content.as_str()).collect::<Vec<_>>().join("; "))
        };
        
        Ok(BrainAnalysisResult {
            analysis,
            insights: insights.clone(),
            confidence: if sources_count > 0 { 0.7 } else { 0.2 },
            metadata: BrainAnalysisMetadata {
                method: "GitHub Memory Analysis".to_string(),
                processing_time_ms: 0,
                sources_analyzed: sources_count,
                complexity_score: (sources_count as f64 / 20.0).min(1.0),
                quality_score: if insights.is_empty() { 0.2 } else { 0.6 },
            },
            related_concepts: concepts,
            patterns: vec!["memory_patterns".to_string()],
        })
    }
    
    /// Perform pattern analysis using Brain AI's pattern detector
    async fn perform_pattern_analysis(
        &mut self,
        _query: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<BrainAnalysisResult, BrainError> {
        println!("  🔍 Performing pattern analysis...");
        
        let mut insights = Vec::new();
        let mut patterns = Vec::new();
        let mut sources_count = 0;
        
        // Get recent memories for pattern analysis
        let working_query = WorkingMemoryQuery {
            content_pattern: None,
            limit: Some(50),
            min_importance: Some(0.1),
            ..Default::default()
        };
        
        if let Ok(working_items) = memory_system.query_working(&working_query) {
            sources_count = working_items.len();
            
            // Convert to text for pattern detection
            let _memory_texts: Vec<String> = working_items.iter()
                .map(|item| item.content.clone())
                .collect();
            
            // Detect patterns in memory content
            if let Ok(pattern_result) = self.pattern_detector.detect_patterns_from_memory(memory_system).await {
                for pattern in pattern_result.detected_patterns {
                    patterns.push(format!("{:?}", pattern.pattern_type));
                    
                    insights.push(BrainInsight {
                        insight_type: "pattern_insight".to_string(),
                        content: format!("Detected pattern '{:?}' with confidence {:.3}: {}", 
                                       pattern.pattern_type, pattern.confidence, pattern.elements.join(", ")),
                        confidence: pattern.confidence,
                        evidence: vec![format!("Found {} evidence items", pattern.evidence.len())],
                    });
                }
            }
        }
        
        let analysis = if patterns.is_empty() {
            "No significant patterns detected in current memory systems.".to_string()
        } else {
            format!("Detected {} patterns in memory systems: {}", 
                   patterns.len(), patterns.join(", "))
        };
        
        Ok(BrainAnalysisResult {
            analysis,
            insights: insights.clone(),
            confidence: if patterns.is_empty() { 0.3 } else { 0.75 },
            metadata: BrainAnalysisMetadata {
                method: "Brain AI Pattern Detection".to_string(),
                processing_time_ms: 0,
                sources_analyzed: sources_count,
                complexity_score: (patterns.len() as f64 / 10.0).min(1.0),
                quality_score: if insights.is_empty() { 0.3 } else { 0.7 },
            },
            related_concepts: patterns.clone(),
            patterns,
        })
    }
    
    /// Perform concept graph analysis
    async fn perform_concept_analysis(
        &self,
        query: &str,
        concept_graph: &mut ConceptGraphManager,
    ) -> Result<BrainAnalysisResult, BrainError> {
        println!("  🕸️ Performing concept graph analysis...");
        
        let mut insights = Vec::new();
        let mut related_concepts = Vec::new();
        let mut sources_count = 0;
        
        // Extract key terms from query
        let query_terms = self.extract_key_terms(query);
        
        for term in query_terms {
            // Search for concepts related to this term
            let concept_query = ConceptQuery {
                content_pattern: Some(term.clone()),
                concept_type: None,
                min_confidence: Some(0.1),
                limit: Some(10),
                ..Default::default()
            };
            
            if let Ok(concepts) = concept_graph.query_concepts(&concept_query).await {
                sources_count += concepts.len();
                
                for concept in concepts {
                    related_concepts.push(concept.content.clone());
                    
                    insights.push(BrainInsight {
                        insight_type: "concept_insight".to_string(),
                        content: format!("Concept '{}' (type: {:?}, confidence: {:.3}): {}", 
                                       concept.content, concept.concept_type, concept.confidence_score,
                                       concept.description.as_ref().unwrap_or(&"No description".to_string())),
                        confidence: concept.confidence_score,
                        evidence: vec![format!("Found in concept graph with {} metadata entries", concept.metadata.len())],
                    });
                    
                    // Get related concepts through graph traversal
                    let traversal_config = TraversalConfig {
                        max_depth: 2,
                        max_nodes: 5,
                        min_relationship_weight: 0.3,
                        activation_spread_factor: 0.8,
                        activation_decay_factor: 0.9,
                        follow_relationship_types: vec![],
                    };
                    
                    if let Ok(traversal_result) = concept_graph.traverse_graph(concept.id, TraversalAlgorithm::BreadthFirst, Some(traversal_config)).await {
                        for concept_id in traversal_result.visited_concepts {
                            if let Ok(Some(related_concept)) = concept_graph.get_concept(concept_id).await {
                                if !related_concepts.contains(&related_concept.content) {
                                    related_concepts.push(related_concept.content);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        let analysis = if related_concepts.is_empty() {
            "No related concepts found in the concept graph.".to_string()
        } else {
            format!("Found {} related concepts in the knowledge graph: {}", 
                   related_concepts.len(), 
                   related_concepts.iter().take(5).cloned().collect::<Vec<_>>().join(", "))
        };
        
        Ok(BrainAnalysisResult {
            analysis,
            insights: insights.clone(),
            confidence: if sources_count > 0 { 0.8 } else { 0.2 },
            metadata: BrainAnalysisMetadata {
                method: "Concept Graph Analysis".to_string(),
                processing_time_ms: 0,
                sources_analyzed: sources_count,
                complexity_score: (related_concepts.len() as f64 / 20.0).min(1.0),
                quality_score: if insights.is_empty() { 0.2 } else { 0.75 },
            },
            related_concepts,
            patterns: vec!["concept_relationships".to_string()],
        })
    }
    
    /// Perform semantic memory analysis using DoTA-RAG inspired multi-stage retrieval
    async fn perform_semantic_analysis(
        &self,
        query: &str,
        memory_system: &mut MemorySystem,
    ) -> Result<BrainAnalysisResult, BrainError> {
        println!("  🧩 Performing DoTA-RAG inspired semantic memory analysis...");
        
        let mut insights = Vec::new();
        let mut concepts = Vec::new();
        let mut sources_count = 0;
        let mut all_retrieved_content = Vec::new();
        
        // Stage 1: Query Rewriting and Expansion (inspired by DoTA-RAG)
        let expanded_queries = self.rewrite_and_expand_query(query);
        println!("  📝 Generated {} expanded queries from original", expanded_queries.len());
        
        // Stage 2: Dynamic Routing to Specialized Search Strategies
        for (strategy_name, search_query) in expanded_queries {
            println!("  🔄 Executing strategy '{}' with query: '{}'", strategy_name, search_query);
            
            let strategy_results = match strategy_name.as_str() {
                "direct_exact" => self.search_exact_matches(memory_system, &search_query).await,
                "semantic_fuzzy" => self.search_semantic_fuzzy(memory_system, &search_query).await,
                "architectural_patterns" => self.search_architectural_patterns(memory_system, &search_query).await,
                "conceptual_synonyms" => self.search_conceptual_synonyms(memory_system, &search_query).await,
                "contextual_embedding" => self.search_contextual_embeddings(memory_system, &search_query).await,
                _ => Vec::new(),
            };
            
            all_retrieved_content.extend(strategy_results);
        }
        
        // Stage 3: Multi-stage Retrieval and Ranking (DoTA-RAG approach)
        let ranked_results = self.rank_and_deduplicate_results(query, all_retrieved_content);
        println!("  📊 Ranked and deduplicated to {} high-quality results", ranked_results.len());
        
        // Stage 4: Generate insights from ranked results
        for (rank, content) in ranked_results.iter().enumerate() {
            sources_count += 1;
            
            let confidence = self.calculate_dynamic_confidence(query, content, rank);
            let insight_type = self.classify_insight_type(content);
            
            insights.push(BrainInsight {
                insight_type,
                content: content.clone(),
                confidence,
                evidence: vec![format!("Multi-stage retrieval rank {}, confidence {:.3}", rank + 1, confidence)],
            });
            
            // Extract concepts from high-quality content
            concepts.extend(self.extract_advanced_concepts(content));
        }
        
        let insights_empty = insights.is_empty();
        let insights_len = insights.len();
        let confidence = self.calculate_overall_confidence(&insights, sources_count);
        
        let final_analysis = if insights_empty {
            "DoTA-RAG inspired analysis found no relevant semantic knowledge in memory systems.".to_string()
        } else {
            format!("DoTA-RAG multi-stage analysis found {} high-confidence insights from {} sources. Top insights: {}",
                insights_len,
                sources_count,
                insights.iter().take(2).map(|i| &i.content[..i.content.len().min(100)]).collect::<Vec<_>>().join("; "))
        };
        
        println!("  ✅ DoTA-RAG analysis complete: {} insights, {:.3} confidence", insights_len, confidence);
        
        Ok(BrainAnalysisResult {
            analysis: final_analysis,
            insights,
            confidence,
            metadata: BrainAnalysisMetadata {
                method: "DoTA-RAG Multi-Stage Semantic Analysis".to_string(),
                processing_time_ms: 0,
                sources_analyzed: sources_count,
                complexity_score: (sources_count as f64 / 10.0).min(1.0),
                quality_score: if insights_empty { 0.3 } else { 0.9 },
            },
            related_concepts: concepts,
            patterns: vec!["multi_stage_retrieval".to_string(), "dynamic_routing".to_string()],
        })
    }

    /// Stage 1: Query rewriting and expansion (DoTA-RAG inspired)
    fn rewrite_and_expand_query(&self, query: &str) -> Vec<(String, String)> {
        let mut expanded_queries = Vec::new();
        
        // Direct exact match
        expanded_queries.push(("direct_exact".to_string(), query.to_string()));
        
        // Architectural pattern specific rewrites
        if query.to_lowercase().contains("architecture") || query.to_lowercase().contains("pattern") {
            expanded_queries.extend(vec![
                ("architectural_patterns".to_string(), "Node-Flow Architecture".to_string()),
                ("architectural_patterns".to_string(), "Async Parallel Processing".to_string()),
                ("architectural_patterns".to_string(), "Batch Optimization Framework".to_string()),
                ("architectural_patterns".to_string(), "BaseNode pattern".to_string()),
                ("architectural_patterns".to_string(), "Flow orchestration".to_string()),
            ]);
        }
        
        // PocketFlow specific rewrites
        if query.to_lowercase().contains("pocketflow") {
            expanded_queries.extend(vec![
                ("semantic_fuzzy".to_string(), "PocketFlow framework".to_string()),
                ("semantic_fuzzy".to_string(), "100-line LLM framework".to_string()),
                ("semantic_fuzzy".to_string(), "Agents build Agents".to_string()),
                ("contextual_embedding".to_string(), "The-Pocket/PocketFlow".to_string()),
            ]);
        }
        
        // Conceptual synonym expansion
        let conceptual_terms = self.generate_conceptual_synonyms(query);
        for term in conceptual_terms {
            expanded_queries.push(("conceptual_synonyms".to_string(), term));
        }
        
        // Component-based queries
        if query.to_lowercase().contains("component") || query.to_lowercase().contains("class") {
            expanded_queries.extend(vec![
                ("semantic_fuzzy".to_string(), "BaseNode class".to_string()),
                ("semantic_fuzzy".to_string(), "Flow class".to_string()),
                ("semantic_fuzzy".to_string(), "BatchNode ParallelBatchNode".to_string()),
            ]);
        }
        
        expanded_queries
    }

    /// Generate conceptual synonyms and related terms
    fn generate_conceptual_synonyms(&self, query: &str) -> Vec<String> {
        let mut synonyms = Vec::new();
        let query_lower = query.to_lowercase();
        
        // Architecture synonyms
        if query_lower.contains("architecture") {
            synonyms.extend(vec![
                "design pattern".to_string(),
                "framework structure".to_string(),
                "system design".to_string(),
                "architectural approach".to_string(),
                "implementation pattern".to_string(),
            ]);
        }
        
        // Pattern synonyms
        if query_lower.contains("pattern") {
            synonyms.extend(vec![
                "design approach".to_string(),
                "implementation strategy".to_string(),
                "architectural style".to_string(),
                "framework paradigm".to_string(),
            ]);
        }
        
        // Component synonyms
        if query_lower.contains("component") || query_lower.contains("class") {
            synonyms.extend(vec![
                "building block".to_string(),
                "module".to_string(),
                "element".to_string(),
                "unit".to_string(),
            ]);
        }
        
        synonyms
    }

    /// Stage 2a: Search for exact matches
    async fn search_exact_matches(&self, memory_system: &MemorySystem, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        // Try working memory exact match
        let working_query = WorkingMemoryQuery {
            content_pattern: Some(query.to_string()),
            priority: None,
            min_importance: None,
            created_after: None,
            limit: Some(5),
        };
        
        if let Ok(items) = memory_system.query_working(&working_query) {
            for item in items {
                results.push(item.content);
            }
        }
        
        results
    }

    /// Stage 2b: Semantic fuzzy search with embedding-like behavior
    async fn search_semantic_fuzzy(&self, memory_system: &MemorySystem, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        // Use find_related_memories for semantic search
        if let Ok(related_results) = memory_system.find_related_memories(query, 5) {
            for item in related_results.working_results {
                results.push(item.content);
            }
            for concept in related_results.semantic_results {
                results.push(format!("Semantic concept '{}': {}", concept.name, concept.description));
            }
        }
        
        // Also try keyword-based fuzzy matching
        let keywords = self.extract_advanced_keywords(query);
        for keyword in keywords {
            let keyword_query = WorkingMemoryQuery {
                content_pattern: Some(keyword),
                priority: None,
                min_importance: None,
                created_after: None,
                limit: Some(3),
            };
            
            if let Ok(items) = memory_system.query_working(&keyword_query) {
                for item in items {
                    if !results.contains(&item.content) {
                        results.push(item.content);
                    }
                }
            }
        }
        
        results
    }

    /// Stage 2c: Architectural pattern specific search
    async fn search_architectural_patterns(&self, memory_system: &MemorySystem, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        // Search for architectural terms in stored content
        let arch_terms = vec![
            "BaseNode", "Flow", "AsyncFlow", "BatchNode", "ParallelBatchNode",
            "architecture", "pattern", "framework", "design", "orchestration",
            "workflow", "agent", "pipeline", "processing"
        ];
        
        for term in arch_terms {
            let term_query = WorkingMemoryQuery {
                content_pattern: Some(term.to_string()),
                priority: None,
                min_importance: None,
                created_after: None,
                limit: Some(2),
            };
            
            if let Ok(items) = memory_system.query_working(&term_query) {
                for item in items {
                    if !results.contains(&item.content) && self.is_architecturally_relevant(&item.content, query) {
                        results.push(item.content);
                    }
                }
            }
        }
        
        results
    }

    /// Stage 2d: Conceptual synonym search
    async fn search_conceptual_synonyms(&self, memory_system: &MemorySystem, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        // Use semantic memory for conceptual search
        let semantic_query = SemanticQuery {
            name_pattern: Some(query.to_string()),
            min_confidence: Some(0.2),
            limit: Some(5),
            ..Default::default()
        };
        
        if let Ok(concepts) = memory_system.query_semantic(&semantic_query) {
            for concept in concepts {
                results.push(format!("Conceptual match '{}' (confidence: {:.3}): {}", 
                    concept.name, concept.confidence, concept.description));
            }
        }
        
        results
    }

    /// Stage 2e: Contextual embedding search (simulated)
    async fn search_contextual_embeddings(&self, memory_system: &MemorySystem, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        // Simulate embedding-based search by looking for contextual matches
        let context_terms = self.extract_contextual_terms(query);
        
        for term in context_terms {
            let context_query = WorkingMemoryQuery {
                content_pattern: Some(term),
                priority: None,
                min_importance: None,
                created_after: None,
                limit: Some(2),
            };
            
            if let Ok(items) = memory_system.query_working(&context_query) {
                for item in items {
                    if !results.contains(&item.content) && self.calculate_contextual_relevance(&item.content, query) > 0.5 {
                        results.push(item.content);
                    }
                }
            }
        }
        
        results
    }

    /// Stage 3: Rank and deduplicate results (DoTA-RAG inspired)
    fn rank_and_deduplicate_results(&self, query: &str, mut results: Vec<String>) -> Vec<String> {
        // Remove duplicates
        results.sort();
        results.dedup();
        
        // Score and rank results
        let mut scored_results: Vec<(f64, String)> = results
            .into_iter()
            .map(|content| {
                let score = self.calculate_comprehensive_relevance_score(query, &content);
                (score, content)
            })
            .collect();
        
        // Sort by score (highest first)
        scored_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return top-ranked results
        scored_results.into_iter()
            .take(10) // Limit to top 10 results
            .map(|(_, content)| content)
            .collect()
    }

    /// Calculate comprehensive relevance score (DoTA-RAG inspired ranking)
    fn calculate_comprehensive_relevance_score(&self, query: &str, content: &str) -> f64 {
        let mut score = 0.0;
        
        // Exact match bonus
        if content.to_lowercase().contains(&query.to_lowercase()) {
            score += 1.0;
        }
        
        // Keyword overlap scoring
        let query_keywords = self.extract_advanced_keywords(query);
        let content_lower = content.to_lowercase();
        let keyword_matches = query_keywords.iter()
            .filter(|keyword| content_lower.contains(&keyword.to_lowercase()))
            .count();
        score += (keyword_matches as f64 / query_keywords.len().max(1) as f64) * 0.8;
        
        // Architectural relevance bonus
        if self.is_architecturally_relevant(content, query) {
            score += 0.6;
        }
        
        // Content quality scoring (longer, structured content gets higher scores)
        if content.len() > 100 {
            score += 0.3;
        }
        if content.contains("class ") || content.contains("def ") || content.contains("import ") {
            score += 0.4; // Code content bonus
        }
        if content.contains("README") || content.contains("documentation") {
            score += 0.5; // Documentation bonus
        }
        
        // PocketFlow specific bonuses
        if content.to_lowercase().contains("pocketflow") {
            score += 0.7;
        }
        if content.to_lowercase().contains("basenode") || content.to_lowercase().contains("flow") {
            score += 0.5;
        }
        
        score
    }

    /// Extract advanced keywords with stemming and expansion
    fn extract_advanced_keywords(&self, text: &str) -> Vec<String> {
        let mut keywords = Vec::new();
        
        // Basic word extraction
        for word in text.split_whitespace() {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if clean_word.len() > 2 && !self.is_stop_word(&clean_word) {
                keywords.push(clean_word.clone());
                keywords.push(word.to_string()); // Also keep original case
            }
        }
        
        // Add technical term variations
        if text.to_lowercase().contains("architecture") {
            keywords.extend(vec!["arch".to_string(), "design".to_string(), "pattern".to_string()]);
        }
        if text.to_lowercase().contains("pattern") {
            keywords.extend(vec!["approach".to_string(), "style".to_string(), "method".to_string()]);
        }
        
        keywords.sort();
        keywords.dedup();
        keywords
    }

    /// Check if a word is a stop word
    fn is_stop_word(&self, word: &str) -> bool {
        let stop_words = vec![
            "the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by",
            "is", "are", "was", "were", "be", "been", "have", "has", "had", "do", "does", "did",
            "will", "would", "could", "should", "may", "might", "can", "a", "an", "what", "how",
            "where", "when", "why", "who", "which", "that", "this", "these", "those", "about",
            "tell", "me", "you", "your", "my", "his", "her", "its", "our", "their"
        ];
        stop_words.contains(&word)
    }

    /// Extract contextual terms for embedding-like search
    fn extract_contextual_terms(&self, query: &str) -> Vec<String> {
        let mut terms = Vec::new();
        
        // Add the full query
        terms.push(query.to_string());
        
        // Add domain-specific context terms
        terms.extend(vec![
            "framework".to_string(),
            "python".to_string(),
            "llm".to_string(),
            "agent".to_string(),
            "workflow".to_string(),
            "orchestration".to_string(),
            "processing".to_string(),
            "async".to_string(),
            "parallel".to_string(),
            "batch".to_string(),
        ]);
        
        terms
    }

    /// Check if content is architecturally relevant
    fn is_architecturally_relevant(&self, content: &str, query: &str) -> bool {
        let content_lower = content.to_lowercase();
        let query_lower = query.to_lowercase();
        
        // Check for architectural keywords
        let arch_keywords = vec![
            "architecture", "pattern", "design", "framework", "structure",
            "basenode", "flow", "batch", "parallel", "async", "orchestration",
            "workflow", "agent", "pipeline", "processing", "class", "component"
        ];
        
        let has_arch_keywords = arch_keywords.iter()
            .any(|keyword| content_lower.contains(keyword) || query_lower.contains(keyword));
        
        // Check for code patterns
        let has_code_patterns = content_lower.contains("class ") || 
                               content_lower.contains("def ") || 
                               content_lower.contains("import ");
        
        has_arch_keywords || has_code_patterns
    }

    /// Calculate contextual relevance
    fn calculate_contextual_relevance(&self, content: &str, query: &str) -> f64 {
        let content_lower = content.to_lowercase();
        let query_lower = query.to_lowercase();
        
        let mut relevance = 0.0;
        
        // Direct mention bonus
        if content_lower.contains(&query_lower) {
            relevance += 0.8;
        }
        
        // Keyword overlap
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let matching_words = query_words.iter()
            .filter(|word| content_lower.contains(*word))
            .count();
        
        relevance += (matching_words as f64 / query_words.len().max(1) as f64) * 0.6;
        
        // Context bonus for PocketFlow
        if content_lower.contains("pocketflow") && query_lower.contains("pocketflow") {
            relevance += 0.5;
        }
        
        relevance.min(1.0)
    }

    /// Calculate dynamic confidence based on rank and content quality
    fn calculate_dynamic_confidence(&self, query: &str, content: &str, rank: usize) -> f64 {
        let base_confidence = 0.9 - (rank as f64 * 0.1); // Decrease confidence with rank
        let relevance_bonus = self.calculate_contextual_relevance(content, query) * 0.3;
        let quality_bonus = if content.len() > 200 { 0.1 } else { 0.0 };
        
        (base_confidence + relevance_bonus + quality_bonus).min(1.0).max(0.1)
    }

    /// Classify insight type based on content
    fn classify_insight_type(&self, content: &str) -> String {
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("class ") || content_lower.contains("def ") {
            "code_analysis".to_string()
        } else if content_lower.contains("readme") || content_lower.contains("documentation") {
            "documentation_insight".to_string()
        } else if content_lower.contains("architecture") || content_lower.contains("pattern") {
            "architectural_insight".to_string()
        } else if content_lower.contains("semantic concept") {
            "semantic_concept".to_string()
        } else {
            "general_knowledge".to_string()
        }
    }

    /// Extract advanced concepts from content
    fn extract_advanced_concepts(&self, content: &str) -> Vec<String> {
        let mut concepts = Vec::new();
        
        // Extract capitalized terms (likely proper nouns/concepts)
        for word in content.split_whitespace() {
            if word.len() > 2 && word.chars().next().unwrap().is_uppercase() {
                concepts.push(word.to_string());
            }
        }
        
        // Extract technical terms
        let content_lower = content.to_lowercase();
        if content_lower.contains("basenode") {
            concepts.push("BaseNode".to_string());
        }
        if content_lower.contains("flow") {
            concepts.push("Flow".to_string());
        }
        if content_lower.contains("batch") {
            concepts.push("Batch Processing".to_string());
        }
        if content_lower.contains("async") {
            concepts.push("Asynchronous Processing".to_string());
        }
        
        concepts.sort();
        concepts.dedup();
        concepts
    }

    /// Calculate overall confidence based on insights and sources
    fn calculate_overall_confidence(&self, insights: &[BrainInsight], sources_count: usize) -> f64 {
        if insights.is_empty() {
            return if sources_count > 0 { 0.3 } else { 0.1 };
        }

        let insight_confidence = insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len() as f64;
        let source_bonus = (sources_count as f64 / 20.0).min(0.2);
        
        (insight_confidence + source_bonus).min(1.0)
    }

    /// Check if query is GitHub related
    fn is_github_related(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        query_lower.contains("github") ||
        query_lower.contains("repository") ||
        query_lower.contains("repo") ||
        query_lower.contains("pocketflow") ||
        query_lower.contains("the-pocket")
    }

    /// Calculate complexity score based on query and insights
    fn calculate_complexity_score(&self, _query: &str, insights: &[BrainInsight]) -> f64 {
        if insights.is_empty() {
            0.1
        } else {
            // Base complexity on number of insights and their types
            let base_score = (insights.len() as f64 / 10.0).min(1.0);
            let type_diversity = insights.iter()
                .map(|i| &i.insight_type)
                .collect::<std::collections::HashSet<_>>()
                .len() as f64 / 5.0; // Normalize by expected max types
            
            (base_score + type_diversity * 0.3).min(1.0)
        }
    }

    /// Calculate quality score based on insights and sources
    fn calculate_quality_score(&self, insights: &[BrainInsight], sources_analyzed: usize) -> f64 {
        if insights.is_empty() {
            0.2
        } else {
            let avg_confidence = insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len() as f64;
            let source_factor = (sources_analyzed as f64 / 20.0).min(0.3); // Bonus for more sources
            let completeness_factor = if insights.len() >= 3 { 0.2 } else { 0.0 };
            
            (avg_confidence + source_factor + completeness_factor).min(1.0)
        }
    }

    /// Extract GitHub references from query
    fn extract_github_references(&self, query: &str) -> Vec<String> {
        let mut github_refs = Vec::new();
        
        // Look for GitHub URLs
        if query.contains("github.com/") {
            // Extract GitHub URLs using simple pattern matching
            let words: Vec<&str> = query.split_whitespace().collect();
            for word in words {
                if word.contains("github.com/") {
                    github_refs.push(word.to_string());
                }
            }
        }
        
        // Look for repository patterns
        if query.to_lowercase().contains("pocketflow") {
            github_refs.push("The-Pocket/PocketFlow".to_string());
        }
        
        github_refs
    }

    /// Extract key terms from query for search
    fn extract_key_terms(&self, query: &str) -> Vec<String> {
        let mut terms = Vec::new();
        
        // Add the full query
        terms.push(query.to_string());
        
        // Extract important words (longer than 3 characters, not stop words)
        for word in query.split_whitespace() {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if clean_word.len() > 3 && !self.is_stop_word(&clean_word.to_lowercase()) {
                terms.push(clean_word.to_string());
                terms.push(clean_word.to_lowercase());
            }
        }
        
        // Add domain-specific terms for PocketFlow
        if query.to_lowercase().contains("pocketflow") {
            terms.extend(vec![
                "PocketFlow".to_string(),
                "BaseNode".to_string(),
                "Flow".to_string(),
                "BatchNode".to_string(),
                "ParallelBatchNode".to_string(),
                "README".to_string(),
                "framework".to_string(),
                "architecture".to_string(),
            ]);
        }
        
        // Add architectural terms
        if query.to_lowercase().contains("architecture") || query.to_lowercase().contains("pattern") {
            terms.extend(vec![
                "architecture".to_string(),
                "pattern".to_string(),
                "design".to_string(),
                "framework".to_string(),
                "Node-Flow".to_string(),
                "Async".to_string(),
                "Parallel".to_string(),
                "Batch".to_string(),
                "Optimization".to_string(),
            ]);
        }
        
        terms.sort();
        terms.dedup();
        terms
    }

    /// Check if content is relevant to the query
    fn is_content_relevant(&self, content: &str, query: &str) -> bool {
        let content_lower = content.to_lowercase();
        let query_lower = query.to_lowercase();
        
        // Direct mention
        if content_lower.contains(&query_lower) {
            return true;
        }
        
        // Check for key terms
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let matching_words = query_words.iter()
            .filter(|word| word.len() > 3 && content_lower.contains(*word))
            .count();
        
        // Require at least 2 matching words for relevance
        matching_words >= 2 || content_lower.len() > 500 // Long content is likely to be comprehensive
    }

    /// Check if content is relevant to query
    fn is_relevant_to_query(&self, content: &str, query: &str) -> bool {
        self.is_content_relevant(content, query)
    }

    /// Extract concepts from text content
    fn extract_concepts_from_text(&self, content: &str) -> Vec<String> {
        let mut concepts = Vec::new();
        
        // Extract capitalized words (likely concepts)
        for word in content.split_whitespace() {
            if word.len() > 2 && word.chars().next().unwrap().is_uppercase() {
                let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
                if clean_word.len() > 2 {
                    concepts.push(clean_word.to_string());
                }
            }
        }
        
        // Extract technical terms
        let content_lower = content.to_lowercase();
        let tech_terms = vec![
            ("basenode", "BaseNode"),
            ("flow", "Flow"),
            ("async", "Async"),
            ("parallel", "Parallel"),
            ("batch", "Batch"),
            ("framework", "Framework"),
            ("architecture", "Architecture"),
            ("pattern", "Pattern"),
            ("orchestration", "Orchestration"),
            ("workflow", "Workflow"),
            ("agent", "Agent"),
            ("pipeline", "Pipeline"),
        ];
        
        for (search_term, concept_name) in tech_terms {
            if content_lower.contains(search_term) {
                concepts.push(concept_name.to_string());
            }
        }
        
        concepts.sort();
        concepts.dedup();
        concepts
    }
}

/// Enhanced LLM Training Integration System
#[derive(Debug, Clone, Serialize, Deserialize)]
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