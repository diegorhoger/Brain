//! Conversation Context Management
//! 
//! This module manages conversation context, user profiles, and temporal patterns
//! in conversations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{
    RetrievedKnowledge, ChatMessage, CommunicationStyle, ResponseLength,
    InteractionSummary, TopicMention, ConversationSegment, AttentionShift,
    TransitionType, TemporalPattern
};

/// Main conversation context structure
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

impl ConversationContext {
    /// Create a new conversation context
    pub fn new(conversation_id: String) -> Self {
        Self {
            conversation_id,
            messages: Vec::new(),
            retrieved_knowledge: Vec::new(),
            context_summary: String::new(),
            user_preferences: HashMap::new(),
            conversation_threads: Vec::new(),
            user_profile: UserProfile::default(),
            temporal_context: TemporalContext::default(),
        }
    }

    /// Add a message to the conversation
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
        self.update_temporal_context();
    }

    /// Get the last N messages
    pub fn get_recent_messages(&self, limit: usize) -> &[ChatMessage] {
        let start = if self.messages.len() > limit {
            self.messages.len() - limit
        } else {
            0
        };
        &self.messages[start..]
    }

    /// Update temporal context with latest patterns
    fn update_temporal_context(&mut self) {
        // Implementation for updating temporal patterns
        // This would analyze message flow and update temporal_context
    }

    /// Extract conversation topics
    pub fn extract_topics(&self) -> Vec<String> {
        self.temporal_context.recent_topics
            .iter()
            .map(|t| t.topic.clone())
            .collect()
    }

    /// Calculate conversation coherence
    pub fn calculate_coherence(&self) -> f64 {
        if self.temporal_context.conversation_flow.is_empty() {
            return 1.0;
        }

        let total_coherence: f64 = self.temporal_context.conversation_flow
            .iter()
            .map(|segment| segment.coherence_score)
            .sum();

        total_coherence / self.temporal_context.conversation_flow.len() as f64
    }
}

/// Conversation thread for topic tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationThread {
    pub thread_id: String,
    pub topic: String,
    pub messages: Vec<String>, // Message IDs
    pub last_updated: DateTime<Utc>,
    pub relevance_score: f64,
}

impl ConversationThread {
    /// Create a new conversation thread
    pub fn new(topic: String) -> Self {
        Self {
            thread_id: uuid::Uuid::new_v4().to_string(),
            topic,
            messages: Vec::new(),
            last_updated: Utc::now(),
            relevance_score: 1.0,
        }
    }

    /// Add a message to this thread
    pub fn add_message(&mut self, message_id: String) {
        self.messages.push(message_id);
        self.last_updated = Utc::now();
    }

    /// Update relevance score based on recent activity
    pub fn update_relevance(&mut self, score: f64) {
        self.relevance_score = score.clamp(0.0, 1.0);
        self.last_updated = Utc::now();
    }
}

/// User profile for personalization
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

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            interests: HashMap::new(),
            expertise_areas: HashMap::new(),
            communication_style: CommunicationStyle::Conversational,
            preferred_response_length: ResponseLength::Moderate,
            interaction_history: Vec::new(),
            learning_progress: HashMap::new(),
        }
    }
}

impl UserProfile {
    /// Update user interest based on interaction
    pub fn update_interest(&mut self, topic: &str, strength_delta: f64) {
        let current = self.interests.get(topic).unwrap_or(&0.0);
        let new_strength = (current + strength_delta).clamp(0.0, 1.0);
        self.interests.insert(topic.to_string(), new_strength);
    }

    /// Update expertise level for an area
    pub fn update_expertise(&mut self, area: &str, level_delta: f64) {
        let current = self.expertise_areas.get(area).unwrap_or(&0.0);
        let new_level = (current + level_delta).clamp(0.0, 1.0);
        self.expertise_areas.insert(area.to_string(), new_level);
    }

    /// Add interaction summary
    pub fn add_interaction(&mut self, summary: InteractionSummary) {
        self.interaction_history.push(summary);
        
        // Keep only recent interactions (last 100)
        if self.interaction_history.len() > 100 {
            self.interaction_history.remove(0);
        }
    }

    /// Get top interests
    pub fn get_top_interests(&self, limit: usize) -> Vec<(String, f64)> {
        let mut interests: Vec<_> = self.interests.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        interests.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        interests.into_iter().take(limit).collect()
    }
}

/// Temporal context for conversation flow tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub recent_topics: Vec<TopicMention>,
    pub conversation_flow: Vec<ConversationSegment>,
    pub attention_shifts: Vec<AttentionShift>,
    pub temporal_patterns: HashMap<String, TemporalPattern>,
}

impl Default for TemporalContext {
    fn default() -> Self {
        Self {
            recent_topics: Vec::new(),
            conversation_flow: Vec::new(),
            attention_shifts: Vec::new(),
            temporal_patterns: HashMap::new(),
        }
    }
}

impl TemporalContext {
    /// Add or update a topic mention
    pub fn mention_topic(&mut self, topic: String, relevance: f64) {
        if let Some(existing) = self.recent_topics.iter_mut().find(|t| t.topic == topic) {
            existing.mention_count += 1;
            existing.last_mentioned = Utc::now();
            existing.context_relevance = relevance;
        } else {
            self.recent_topics.push(TopicMention {
                topic,
                mention_count: 1,
                last_mentioned: Utc::now(),
                context_relevance: relevance,
            });
        }

        // Keep only recent topics (last 20)
        self.recent_topics.sort_by(|a, b| b.last_mentioned.cmp(&a.last_mentioned));
        if self.recent_topics.len() > 20 {
            self.recent_topics.truncate(20);
        }
    }

    /// Start a new conversation segment
    pub fn start_segment(&mut self, topic: String) -> String {
        let segment = ConversationSegment {
            segment_id: uuid::Uuid::new_v4().to_string(),
            start_time: Utc::now(),
            end_time: None,
            primary_topic: topic,
            sub_topics: Vec::new(),
            coherence_score: 1.0,
        };

        let segment_id = segment.segment_id.clone();
        self.conversation_flow.push(segment);
        segment_id
    }

    /// End the current conversation segment
    pub fn end_current_segment(&mut self) {
        if let Some(last_segment) = self.conversation_flow.last_mut() {
            if last_segment.end_time.is_none() {
                last_segment.end_time = Some(Utc::now());
            }
        }
    }

    /// Record an attention shift
    pub fn record_attention_shift(&mut self, from_topic: String, to_topic: String, transition_type: TransitionType) {
        self.attention_shifts.push(AttentionShift {
            from_topic,
            to_topic,
            shift_time: Utc::now(),
            transition_type,
        });

        // Keep only recent shifts (last 50)
        if self.attention_shifts.len() > 50 {
            self.attention_shifts.remove(0);
        }
    }

    /// Get conversation flow summary
    pub fn get_flow_summary(&self) -> String {
        if self.conversation_flow.is_empty() {
            return "No conversation flow recorded".to_string();
        }

        let topics: Vec<String> = self.conversation_flow
            .iter()
            .map(|segment| segment.primary_topic.clone())
            .collect();

        format!("Conversation flow: {}", topics.join(" → "))
    }
} 