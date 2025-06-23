use crate::error::BrainError;
use crate::memory::{MemorySystem, Priority, WorkingMemoryQuery, SemanticQuery, EpisodicQuery, WorkingMemoryItem};
use crate::concept_graph::{ConceptGraphManager, ConceptQuery, TraversalAlgorithm, TraversalConfig, ConceptType};
use crate::insight_extraction::PatternDetector;
use crate::segment_discovery::{BpeSegmenter, BpeConfig};
use crate::training_data::TrainingDataCollector;
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
    pub source_diversity: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicRequest {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f64,
    pub messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicResponse {
    pub content: Vec<AnthropicContent>,
    pub id: String,
    pub model: String,
    pub role: String,
    pub stop_reason: Option<String>,
    pub usage: Option<AnthropicUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicContent {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
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
    anthropic_api_key: String,
    anthropic_model: String,
    max_tokens: u32,
    temperature: f64,
    conversations: HashMap<String, ConversationContext>,
    training_data_collector: Option<TrainingDataCollector>,
}

impl RagOrchestrator {
    pub fn new() -> Result<Self, BrainError> {
        let anthropic_api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| BrainError::ConfigError("ANTHROPIC_API_KEY not set".to_string()))?;
        
        let anthropic_model = env::var("MODEL")
            .unwrap_or_else(|_| "claude-3-opus-20240229".to_string());
        
        let max_tokens = env::var("MAX_TOKENS")
            .unwrap_or_else(|_| "4000".to_string())
            .parse::<u32>()
            .unwrap_or(4000);
        
        let temperature = env::var("TEMPERATURE")
            .unwrap_or_else(|_| "0.7".to_string())
            .parse::<f64>()
            .unwrap_or(0.7);
        
        let client = reqwest::Client::new();
        
        Ok(Self {
            client,
            anthropic_api_key,
            anthropic_model,
            max_tokens,
            temperature,
            conversations: HashMap::new(),
            training_data_collector: None,
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
        self.conversations.insert(conversation_id.clone(), context);

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
        &self,
        message: &str,
        context: &ConversationContext,
        memory_system: &mut MemorySystem,
        concept_graph: &mut ConceptGraphManager,
        pattern_detector: &mut PatternDetector,
        threshold: f64,
        limit: usize,
    ) -> Result<Vec<RetrievedKnowledge>, BrainError> {
        println!("🔍 RAG Orchestrator: Advanced Knowledge Retrieval with Context Integration");
        
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
        prompt.push_str("- Use the retrieved knowledge to inform your response\n");
        prompt.push_str("- Be helpful, accurate, and engaging\n");
        prompt.push_str("- If you don't have specific knowledge about something, say so clearly\n");
        prompt.push_str("- Maintain conversational flow and context\n");
        prompt.push_str("- Reference specific knowledge sources when relevant\n\n");

        prompt.push_str(&format!("USER: {}\n\nASSISTANT:", message));

        println!("  - Built prompt with {} characters", prompt.len());
        println!("  - Included {} knowledge sources", knowledge.len());

        // Make API call to Anthropic
        let anthropic_request = AnthropicRequest {
            model: self.anthropic_model.clone(),
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            messages: vec![
                AnthropicMessage {
                    role: "user".to_string(),
                    content: prompt,
                }
            ],
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.anthropic_api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&anthropic_request)
            .send()
            .await
            .map_err(|e| BrainError::NetworkError(format!("Anthropic API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrainError::NetworkError(format!("Anthropic API error: {}", error_text)));
        }

        let anthropic_response: AnthropicResponse = response.json().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to parse Anthropic response: {}", e)))?;

        let generated_text = anthropic_response.content
            .into_iter()
            .map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");

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
                model_name: self.anthropic_model.clone(),
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

        let source_types: HashSet<String> = attributed_sources.iter()
            .map(|s| s.source_type.clone())
            .collect();
        let source_diversity = source_types.len() as f64 / attributed_sources.len().max(1) as f64;

        let source_attribution_score = (citation_completeness * 0.7 + source_diversity * 0.3).clamp(0.0, 1.0);

        debug_info.interpretability_info.reasoning_chain.push(
            format!("Source attribution: {:.3} (completeness: {:.3}, diversity: {:.3})",
                   source_attribution_score, citation_completeness, source_diversity)
        );

        let source_attribution = SourceAttribution {
            knowledge_sources: attributed_sources,
            confidence_per_source,
            source_reliability,
            citation_completeness,
            source_diversity,
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
        stats.insert("model".to_string(), serde_json::Value::String(self.anthropic_model.clone()));
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExpandedConcept {
    content: String,
    concept_type: ConceptType,
    relevance_score: f64,
    context_path: Vec<String>,
    depth: usize,
}