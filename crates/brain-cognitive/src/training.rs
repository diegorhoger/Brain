//! Training Data Collection Module
//! 
//! This module provides comprehensive training data collection, quality assessment,
//! and export functionality for the Brain AI system.

use brain_types::BrainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use regex::Regex;
use std::fs;


// Import from our conversation module
use crate::conversation::{RagResponse, RetrievedKnowledge, ConversationContext, ResponseQuality};

/// Main training data collection orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataCollector {
    config: TrainingDataConfig,
    conversation_storage: HashMap<String, ConversationRecord>,
    quality_assessor: QualityAssessor,
    anonymizer: DataAnonymizer,
    analytics: ConversationAnalytics,
}

/// Configuration for training data collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataConfig {
    pub storage_path: String,
    pub max_conversations: usize,
    pub quality_threshold: f64,
    pub enable_anonymization: bool,
    pub retention_days: i64,
    pub batch_size: usize,
    pub auto_export: bool,
    pub export_format: ExportFormat,
}

/// Supported export formats for training datasets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    JsonL,
    Parquet,
    Csv,
    HuggingFace,
}

/// Complete conversation record for training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRecord {
    pub conversation_id: String,
    pub messages: Vec<MessageRecord>,
    pub metadata: ConversationMetadata,
    pub quality_metrics: ConversationQualityMetrics,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Individual message within a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRecord {
    pub message_id: String,
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub anonymized_content: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub knowledge_sources: Vec<KnowledgeSourceRecord>,
    pub response_quality: Option<ResponseQuality>,
    pub user_feedback: Option<UserFeedback>,
}

/// Metadata about the conversation for training analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMetadata {
    pub domain: String,
    pub complexity_level: ComplexityLevel,
    pub conversation_type: ConversationType,
    pub user_expertise: UserExpertise,
    pub session_duration_minutes: f64,
    pub turn_count: usize,
    pub context_switches: usize,
    pub topics: Vec<String>,
}

/// Complexity levels for conversation classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    Expert,
}

/// Types of conversations for training categorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationType {
    QuestionsAndAnswers,
    Tutorial,
    ProblemSolving,
    Research,
    Casual,
    Technical,
}

/// User expertise levels for personalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserExpertise {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Knowledge source information for training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSourceRecord {
    pub source_type: String,
    pub content_summary: String,
    pub relevance_score: f64,
    pub confidence: f64,
}

/// User feedback for quality assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub satisfaction_score: f64, // 0.0-1.0
    pub helpfulness: f64,
    pub accuracy: f64,
    pub clarity: f64,
    pub feedback_text: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Comprehensive quality metrics for conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationQualityMetrics {
    pub overall_quality: f64,
    pub coherence_score: f64,
    pub knowledge_grounding: f64,
    pub response_relevance: f64,
    pub safety_score: f64,
    pub educational_value: f64,
    pub diversity_score: f64,
    pub uniqueness_score: f64,
}

/// Quality assessment engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessor {
    pub quality_models: Vec<QualityModel>,
    pub thresholds: QualityThresholds,
    pub pattern_analyzers: Vec<PatternAnalyzer>,
}

/// Individual quality assessment models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityModel {
    pub name: String,
    pub model_type: QualityModelType,
    pub weight: f64,
    pub parameters: HashMap<String, f64>,
}

/// Types of quality assessment models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityModelType {
    CoherenceAnalyzer,
    FactualAccuracyChecker,
    RelevanceScorer,
    SafetyValidator,
    EducationalValueAssessor,
    DiversityMeasurer,
}

/// Quality thresholds for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityThresholds {
    pub minimum_quality: f64,
    pub excellent_quality: f64,
    pub coherence_threshold: f64,
    pub safety_threshold: f64,
    pub relevance_threshold: f64,
}

/// Pattern analysis for conversation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternAnalyzer {
    pub analyzer_type: PatternType,
    pub patterns: Vec<ConversationPattern>,
}

/// Types of conversation patterns to analyze
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    TopicFlow,
    QuestionAnswerPairs,
    ErrorCorrection,
    LearningProgression,
    ConceptIntroduction,
}

/// Individual conversation patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPattern {
    pub pattern_id: String,
    pub description: String,
    pub frequency: f64,
    pub quality_impact: f64,
    pub examples: Vec<String>,
}

/// Data anonymization system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAnonymizer {
    pub anonymization_rules: Vec<AnonymizationRule>,
    pub pii_detectors: Vec<PiiDetector>,
    pub replacement_strategies: HashMap<String, ReplacementStrategy>,
}

/// Rules for anonymizing specific types of data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationRule {
    pub rule_type: PiiType,
    pub pattern: String,
    pub replacement: String,
    pub confidence_threshold: f64,
}

/// Types of personally identifiable information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PiiType {
    Name,
    Email,
    Phone,
    Address,
    CreditCard,
    SocialSecurity,
    IpAddress,
    Custom(String),
}

/// PII detection systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiDetector {
    pub detector_type: PiiType,
    pub regex_patterns: Vec<String>,
    pub confidence_scoring: bool,
}

/// Strategies for replacing detected PII
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplacementStrategy {
    Mask,
    Synthetic,
    Removal,
    Placeholder,
}

/// Analytics and statistics for training data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationAnalytics {
    pub total_conversations: usize,
    pub total_messages: usize,
    pub quality_distribution: HashMap<String, usize>,
    pub topic_frequency: HashMap<String, usize>,
    pub pattern_frequency: HashMap<String, usize>,
    pub user_satisfaction: f64,
    pub data_quality_trends: Vec<QualityTrend>,
}

/// Quality trends over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrend {
    pub timestamp: DateTime<Utc>,
    pub quality_score: f64,
    pub conversation_count: usize,
    pub improvement_areas: Vec<String>,
}

/// Dataset filtering criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetFilter {
    pub min_quality: Option<f64>,
    pub max_quality: Option<f64>,
    pub conversation_types: Option<Vec<ConversationType>>,
    pub complexity_levels: Option<Vec<ComplexityLevel>>,
    pub topics: Option<Vec<String>>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

/// Exported training dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataset {
    pub conversations: Vec<ConversationRecord>,
    pub metadata: DatasetMetadata,
    pub statistics: DatasetStatistics,
}

/// Metadata about the exported dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub created_at: DateTime<Utc>,
    pub version: String,
    pub format: ExportFormat,
    pub total_conversations: usize,
    pub total_messages: usize,
    pub quality_threshold: f64,
}

/// Statistics about the exported dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetStatistics {
    pub quality_distribution: HashMap<String, usize>,
    pub topic_distribution: HashMap<String, usize>,
    pub complexity_distribution: HashMap<String, usize>,
    pub conversation_type_distribution: HashMap<String, usize>,
    pub average_quality: f64,
    pub average_conversation_length: f64,
}

// ============================================================================
// IMPLEMENTATIONS
// ============================================================================

impl Default for TrainingDataConfig {
    fn default() -> Self {
        Self {
            storage_path: "training_data".to_string(),
            max_conversations: 10000,
            quality_threshold: 0.7,
            enable_anonymization: true,
            retention_days: 365,
            batch_size: 100,
            auto_export: false,
            export_format: ExportFormat::JsonL,
        }
    }
}

impl TrainingDataCollector {
    /// Create a new training data collector
    pub fn new(config: TrainingDataConfig) -> Result<Self, BrainError> {
        let quality_assessor = QualityAssessor::new()?;
        let anonymizer = DataAnonymizer::new()?;
        let analytics = ConversationAnalytics::new();

        // Ensure storage directory exists
        if let Err(e) = fs::create_dir_all(&config.storage_path) {
            return Err(BrainError::Io { source: e });
        }

        Ok(Self {
            config,
            conversation_storage: HashMap::new(),
            quality_assessor,
            anonymizer,
            analytics,
        })
    }

    /// Capture a conversation for training data
    pub async fn capture_conversation(
        &mut self,
        conversation_id: &str,
        user_message: &str,
        assistant_response: &RagResponse,
        context: &ConversationContext,
        knowledge_sources: &[RetrievedKnowledge],
    ) -> Result<(), BrainError> {
        // Create message records
        let user_msg = MessageRecord::new_user_message(user_message)?;
        let assistant_msg = MessageRecord::new_assistant_message(
            &assistant_response.response,
            &assistant_response.response_quality,
            knowledge_sources,
        )?;

        // Apply anonymization if enabled
        let anonymized_user_msg = if self.config.enable_anonymization {
            self.anonymizer.anonymize_message(&user_msg).await?
        } else {
            user_msg
        };

        let anonymized_assistant_msg = if self.config.enable_anonymization {
            self.anonymizer.anonymize_message(&assistant_msg).await?
        } else {
            assistant_msg
        };

        // Create or get existing conversation record and add messages
        let mut conversation = self.conversation_storage
            .remove(&conversation_id.to_string())
            .unwrap_or_else(|| ConversationRecord::new(conversation_id));

        conversation.messages.push(anonymized_user_msg);
        conversation.messages.push(anonymized_assistant_msg);

        // Update conversation metadata
        self.update_conversation_metadata(&mut conversation, context).await?;

        // Assess conversation quality
        let quality_metrics = self.quality_assessor
            .assess_conversation_quality(&conversation).await?;
        conversation.quality_metrics = quality_metrics;

        // Update analytics
        self.analytics.update_with_conversation(&conversation)?;

        // Auto-export if configured and quality threshold met
        let should_export = self.config.auto_export && 
           conversation.quality_metrics.overall_quality >= self.config.quality_threshold;

        // Store the conversation back
        self.conversation_storage.insert(conversation_id.to_string(), conversation);

        if should_export {
            self.export_conversation(conversation_id).await?;
        }

        Ok(())
    }

    /// Get conversation analytics
    pub fn get_conversation_analytics(&self) -> &ConversationAnalytics {
        &self.analytics
    }

    /// Export training dataset with optional filtering
    pub async fn export_training_dataset(
        &self,
        filter_criteria: Option<DatasetFilter>,
    ) -> Result<TrainingDataset, BrainError> {
        let filtered_conversations = self.filter_conversations(filter_criteria)?;
        
        let dataset = TrainingDataset::new(
            filtered_conversations,
            &self.config.export_format,
            &self.analytics,
        )?;

        // Save to disk
        let export_path = format!("{}/dataset_{}.{}", 
            self.config.storage_path,
            Utc::now().format("%Y%m%d_%H%M%S"),
            self.get_file_extension()
        );

        dataset.save_to_file(&export_path).await?;

        Ok(dataset)
    }

    /// Get quality distribution
    pub fn get_quality_distribution(&self) -> HashMap<String, f64> {
        let mut distribution = HashMap::new();
        let total = self.conversation_storage.len() as f64;

        for conversation in self.conversation_storage.values() {
            let quality_bucket = self.get_quality_bucket(conversation.quality_metrics.overall_quality);
            *distribution.entry(quality_bucket).or_insert(0.0) += 1.0 / total;
        }

        distribution
    }

    /// Update conversation metadata
    async fn update_conversation_metadata(
        &self,
        conversation: &mut ConversationRecord,
        _context: &ConversationContext,
    ) -> Result<(), BrainError> {
        // Analyze conversation complexity
        let complexity = self.analyze_conversation_complexity(conversation).await?;
        
        // Detect conversation type
        let conv_type = self.detect_conversation_type(conversation).await?;
        
        // Extract topics
        let topics = self.extract_topics(conversation).await?;

        conversation.metadata.complexity_level = complexity;
        conversation.metadata.conversation_type = conv_type;
        conversation.metadata.topics = topics;
        conversation.metadata.turn_count = conversation.messages.len() / 2; // user + assistant pairs
        conversation.last_updated = Utc::now();

        Ok(())
    }

    /// Analyze conversation complexity
    async fn analyze_conversation_complexity(&self, conversation: &ConversationRecord) -> Result<ComplexityLevel, BrainError> {
        let mut complexity_score = 0.0;
        
        // Analyze message length and vocabulary
        for message in &conversation.messages {
            let word_count = message.content.split_whitespace().count();
            let unique_words: HashSet<&str> = message.content.split_whitespace().collect();
            
            complexity_score += word_count as f64 * 0.1;
            complexity_score += unique_words.len() as f64 * 0.2;
        }

        // Normalize by message count
        if !conversation.messages.is_empty() {
            complexity_score /= conversation.messages.len() as f64;
        }

        Ok(match complexity_score {
            s if s < 10.0 => ComplexityLevel::Simple,
            s if s < 25.0 => ComplexityLevel::Moderate,
            s if s < 50.0 => ComplexityLevel::Complex,
            _ => ComplexityLevel::Expert,
        })
    }

    /// Detect conversation type
    async fn detect_conversation_type(&self, conversation: &ConversationRecord) -> Result<ConversationType, BrainError> {
        let content = conversation.messages.iter()
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        // Pattern matching for conversation types
        let question_patterns = Regex::new(r"\?|\bwhat\b|\bhow\b|\bwhy\b|\bwhen\b|\bwhere\b").unwrap();
        let tutorial_patterns = Regex::new(r"\bstep\b|\btutorial\b|\bguide\b|\blearn\b").unwrap();
        let technical_patterns = Regex::new(r"\bapi\b|\bcode\b|\bfunction\b|\balgorithm\b").unwrap();

        let question_matches = question_patterns.find_iter(&content).count();
        let tutorial_matches = tutorial_patterns.find_iter(&content).count();
        let technical_matches = technical_patterns.find_iter(&content).count();

        Ok(if technical_matches > 5 {
            ConversationType::Technical
        } else if tutorial_matches > 3 {
            ConversationType::Tutorial
        } else if question_matches > conversation.messages.len() / 4 {
            ConversationType::QuestionsAndAnswers
        } else {
            ConversationType::Casual
        })
    }

    /// Extract topics from conversation
    async fn extract_topics(&self, conversation: &ConversationRecord) -> Result<Vec<String>, BrainError> {
        let mut topics = Vec::new();
        
        // Simple keyword extraction - in practice, would use more sophisticated NLP
        let content = conversation.messages.iter()
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        let common_words: HashSet<&str> = ["the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by", "is", "are", "was", "were", "be", "been", "have", "has", "had", "do", "does", "did", "will", "would", "could", "should", "may", "might", "can", "a", "an"].iter().cloned().collect();

        let words: Vec<&str> = content.split_whitespace()
            .filter(|word| word.len() > 3 && !common_words.contains(&word.to_lowercase().as_str()))
            .collect();

        // Count word frequency
        let mut word_freq: HashMap<&str, usize> = HashMap::new();
        for word in words {
            *word_freq.entry(word).or_insert(0) += 1;
        }

        // Get top topics
        let mut freq_vec: Vec<_> = word_freq.into_iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (word, _) in freq_vec.into_iter().take(5) {
            topics.push(word.to_string());
        }

        Ok(topics)
    }

    /// Filter conversations based on criteria
    fn filter_conversations(&self, filter: Option<DatasetFilter>) -> Result<Vec<&ConversationRecord>, BrainError> {
        let conversations: Vec<&ConversationRecord> = self.conversation_storage
            .values()
            .filter(|conversation| {
                if let Some(ref filter) = filter {
                    filter.matches(conversation)
                } else {
                    true
                }
            })
            .collect();

        Ok(conversations)
    }

    /// Get quality bucket for a quality score
    fn get_quality_bucket(&self, quality: f64) -> String {
        match quality {
            q if q >= 0.9 => "excellent".to_string(),
            q if q >= 0.7 => "good".to_string(),
            q if q >= 0.5 => "fair".to_string(),
            _ => "poor".to_string(),
        }
    }

    /// Get file extension for export format
    fn get_file_extension(&self) -> &str {
        match self.config.export_format {
            ExportFormat::JsonL => "jsonl",
            ExportFormat::Parquet => "parquet",
            ExportFormat::Csv => "csv",
            ExportFormat::HuggingFace => "json",
        }
    }

    /// Export a single conversation
    async fn export_conversation(&self, conversation_id: &str) -> Result<(), BrainError> {
        if let Some(conversation) = self.conversation_storage.get(conversation_id) {
            let export_path = format!("{}/conversation_{}.json", 
                self.config.storage_path, 
                conversation_id
            );
            
            let json_data = serde_json::to_string_pretty(conversation)
                .map_err(|e| BrainError::ConfigError(format!("Failed to serialize conversation: {}", e)))?;
            
            fs::write(export_path, json_data)
                .map_err(|e| BrainError::Io { source: e })?;
        }

        Ok(())
    }
}

impl ConversationRecord {
    fn new(conversation_id: &str) -> Self {
        let now = Utc::now();
        Self {
            conversation_id: conversation_id.to_string(),
            messages: Vec::new(),
            metadata: ConversationMetadata::default(),
            quality_metrics: ConversationQualityMetrics::default(),
            created_at: now,
            last_updated: now,
        }
    }
}

impl ConversationMetadata {
    fn default() -> Self {
        Self {
            domain: "general".to_string(),
            complexity_level: ComplexityLevel::Simple,
            conversation_type: ConversationType::Casual,
            user_expertise: UserExpertise::Beginner,
            session_duration_minutes: 0.0,
            turn_count: 0,
            context_switches: 0,
            topics: Vec::new(),
        }
    }
}

impl ConversationQualityMetrics {
    fn default() -> Self {
        Self {
            overall_quality: 0.5,
            coherence_score: 0.5,
            knowledge_grounding: 0.5,
            response_relevance: 0.5,
            safety_score: 0.9,
            educational_value: 0.5,
            diversity_score: 0.5,
            uniqueness_score: 0.5,
        }
    }
}

impl MessageRecord {
    fn new_user_message(content: &str) -> Result<Self, BrainError> {
        Ok(Self {
            message_id: Uuid::new_v4().to_string(),
            role: "user".to_string(),
            content: content.to_string(),
            anonymized_content: None,
            timestamp: Utc::now(),
            knowledge_sources: Vec::new(),
            response_quality: None,
            user_feedback: None,
        })
    }

    fn new_assistant_message(
        content: &str,
        quality: &ResponseQuality,
        knowledge_sources: &[RetrievedKnowledge],
    ) -> Result<Self, BrainError> {
        let knowledge_records: Vec<KnowledgeSourceRecord> = knowledge_sources
            .iter()
            .map(|ks| KnowledgeSourceRecord {
                source_type: ks.knowledge_type.clone(),
                content_summary: if ks.content.len() > 100 {
                    format!("{}...", &ks.content[..100])
                } else {
                    ks.content.clone()
                },
                relevance_score: ks.relevance_score,
                confidence: 0.8, // Default confidence
            })
            .collect();

        Ok(Self {
            message_id: Uuid::new_v4().to_string(),
            role: "assistant".to_string(),
            content: content.to_string(),
            anonymized_content: None,
            timestamp: Utc::now(),
            knowledge_sources: knowledge_records,
            response_quality: Some(quality.clone()),
            user_feedback: None,
        })
    }
}

impl QualityAssessor {
    fn new() -> Result<Self, BrainError> {
        let quality_models = vec![
            QualityModel {
                name: "coherence_analyzer".to_string(),
                model_type: QualityModelType::CoherenceAnalyzer,
                weight: 0.25,
                parameters: HashMap::new(),
            },
            QualityModel {
                name: "relevance_scorer".to_string(),
                model_type: QualityModelType::RelevanceScorer,
                weight: 0.25,
                parameters: HashMap::new(),
            },
            QualityModel {
                name: "safety_validator".to_string(),
                model_type: QualityModelType::SafetyValidator,
                weight: 0.3,
                parameters: HashMap::new(),
            },
            QualityModel {
                name: "educational_assessor".to_string(),
                model_type: QualityModelType::EducationalValueAssessor,
                weight: 0.2,
                parameters: HashMap::new(),
            },
        ];

        let thresholds = QualityThresholds {
            minimum_quality: 0.3,
            excellent_quality: 0.9,
            coherence_threshold: 0.7,
            safety_threshold: 0.8,
            relevance_threshold: 0.6,
        };

        Ok(Self {
            quality_models,
            thresholds,
            pattern_analyzers: Vec::new(),
        })
    }

    async fn assess_conversation_quality(
        &self,
        conversation: &ConversationRecord,
    ) -> Result<ConversationQualityMetrics, BrainError> {
        let coherence_score = self.assess_coherence(conversation).await?;
        let knowledge_grounding = self.assess_knowledge_grounding(conversation).await?;
        let response_relevance = self.assess_relevance(conversation).await?;
        let safety_score = self.assess_safety(conversation).await?;
        let educational_value = self.assess_educational_value(conversation).await?;
        let diversity_score = self.assess_diversity(conversation).await?;
        let uniqueness_score = self.assess_uniqueness(conversation).await?;

        let overall_quality = self.calculate_overall_quality(&ConversationQualityMetrics {
            overall_quality: 0.0, // Will be calculated
            coherence_score,
            knowledge_grounding,
            response_relevance,
            safety_score,
            educational_value,
            diversity_score,
            uniqueness_score,
        });

        Ok(ConversationQualityMetrics {
            overall_quality,
            coherence_score,
            knowledge_grounding,
            response_relevance,
            safety_score,
            educational_value,
            diversity_score,
            uniqueness_score,
        })
    }

    async fn assess_coherence(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        if conversation.messages.len() < 2 {
            return Ok(0.5);
        }

        let mut coherence_score = 0.0;
        let mut pair_count = 0;

        for window in conversation.messages.windows(2) {
            if let [prev, curr] = window {
                let similarity = self.calculate_text_similarity(&prev.content, &curr.content);
                coherence_score += similarity;
                pair_count += 1;
            }
        }

        Ok(if pair_count > 0 {
            coherence_score / pair_count as f64
        } else {
            0.5
        })
    }

    async fn assess_knowledge_grounding(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        let assistant_messages: Vec<_> = conversation.messages
            .iter()
            .filter(|m| m.role == "assistant")
            .collect();

        if assistant_messages.is_empty() {
            return Ok(0.5);
        }

        let total_knowledge_score: f64 = assistant_messages
            .iter()
            .map(|msg| {
                if msg.knowledge_sources.is_empty() {
                    0.3 // Low score for no knowledge sources
                } else {
                    let avg_relevance: f64 = msg.knowledge_sources
                        .iter()
                        .map(|ks| ks.relevance_score)
                        .sum::<f64>() / msg.knowledge_sources.len() as f64;
                    avg_relevance.min(1.0)
                }
            })
            .sum();

        Ok(total_knowledge_score / assistant_messages.len() as f64)
    }

    async fn assess_relevance(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        if conversation.messages.len() < 2 {
            return Ok(0.5);
        }

        let mut relevance_score = 0.0;
        let mut qa_pairs = 0;

        for window in conversation.messages.windows(2) {
            if let [user_msg, assistant_msg] = window {
                if user_msg.role == "user" && assistant_msg.role == "assistant" {
                    let similarity = self.calculate_text_similarity(&user_msg.content, &assistant_msg.content);
                    relevance_score += similarity;
                    qa_pairs += 1;
                }
            }
        }

        Ok(if qa_pairs > 0 {
            relevance_score / qa_pairs as f64
        } else {
            0.5
        })
    }

    async fn assess_safety(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        let mut safety_score: f64 = 1.0; // Start with perfect safety

        for message in &conversation.messages {
            let content_lower = message.content.to_lowercase();
            
            // Simple safety checks
            let harmful_patterns = [
                "violence", "harm", "attack", "kill", "destroy", "dangerous",
                "illegal", "criminal", "fraud", "scam", "hate", "discrimination"
            ];

            for pattern in &harmful_patterns {
                if content_lower.contains(pattern) {
                    safety_score -= 0.1;
                }
            }
        }

        Ok(safety_score.max(0.0))
    }

    async fn assess_educational_value(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        let mut educational_score = 0.0;

        for message in &conversation.messages {
            let content_lower = message.content.to_lowercase();
            
            // Educational indicators
            let educational_patterns = [
                "learn", "understand", "explain", "because", "therefore", "example",
                "concept", "principle", "theory", "practice", "tutorial", "guide"
            ];

            for pattern in &educational_patterns {
                if content_lower.contains(pattern) {
                    educational_score += 0.1;
                }
            }
        }

        Ok((educational_score / conversation.messages.len() as f64).min(1.0))
    }

    async fn assess_diversity(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        let mut unique_words: HashSet<String> = HashSet::new();
        let mut total_words = 0;

        for message in &conversation.messages {
            for word in message.content.split_whitespace() {
                unique_words.insert(word.to_lowercase());
                total_words += 1;
            }
        }

        Ok(if total_words > 0 {
            unique_words.len() as f64 / total_words as f64
        } else {
            0.0
        })
    }

    async fn assess_uniqueness(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        // Simple uniqueness based on message length variance
        let message_lengths: Vec<usize> = conversation.messages
            .iter()
            .map(|m| m.content.len())
            .collect();

        if message_lengths.len() < 2 {
            return Ok(0.5);
        }

        let mean_length = message_lengths.iter().sum::<usize>() as f64 / message_lengths.len() as f64;
        let variance = message_lengths
            .iter()
            .map(|&len| (len as f64 - mean_length).powi(2))
            .sum::<f64>() / message_lengths.len() as f64;

        Ok((variance.sqrt() / mean_length).min(1.0))
    }

    fn calculate_overall_quality(&self, metrics: &ConversationQualityMetrics) -> f64 {
        let weighted_score = metrics.coherence_score * 0.2 +
                           metrics.knowledge_grounding * 0.2 +
                           metrics.response_relevance * 0.25 +
                           metrics.safety_score * 0.25 +
                           metrics.educational_value * 0.1;

        weighted_score.min(1.0).max(0.0)
    }

    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f64 {
        if text1.is_empty() && text2.is_empty() {
            return 1.0;
        }

        let words1: HashSet<&str> = text1.split_whitespace().collect();
        let words2: HashSet<&str> = text2.split_whitespace().collect();

        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        intersection as f64 / union as f64
    }
}

impl DataAnonymizer {
    fn new() -> Result<Self, BrainError> {
        let anonymization_rules = vec![
            AnonymizationRule {
                rule_type: PiiType::Email,
                pattern: r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b".to_string(),
                replacement: "[EMAIL]".to_string(),
                confidence_threshold: 0.9,
            },
            AnonymizationRule {
                rule_type: PiiType::Phone,
                pattern: r"\b\d{3}-\d{3}-\d{4}\b".to_string(),
                replacement: "[PHONE]".to_string(),
                confidence_threshold: 0.8,
            },
        ];

        Ok(Self {
            anonymization_rules,
            pii_detectors: Vec::new(),
            replacement_strategies: HashMap::new(),
        })
    }

    async fn anonymize_message(&self, message: &MessageRecord) -> Result<MessageRecord, BrainError> {
        let mut anonymized_message = message.clone();
        let mut anonymized_content = message.content.clone();

        for rule in &self.anonymization_rules {
            let regex = Regex::new(&rule.pattern)
                .map_err(|e| BrainError::ConfigError(format!("Invalid regex pattern: {}", e)))?;
            
            anonymized_content = regex.replace_all(&anonymized_content, &rule.replacement).to_string();
        }

        anonymized_message.anonymized_content = Some(anonymized_content);
        Ok(anonymized_message)
    }
}

impl ConversationAnalytics {
    fn new() -> Self {
        Self {
            total_conversations: 0,
            total_messages: 0,
            quality_distribution: HashMap::new(),
            topic_frequency: HashMap::new(),
            pattern_frequency: HashMap::new(),
            user_satisfaction: 0.0,
            data_quality_trends: Vec::new(),
        }
    }

    fn update_with_conversation(&mut self, conversation: &ConversationRecord) -> Result<(), BrainError> {
        self.total_conversations += 1;
        self.total_messages += conversation.messages.len();

        // Update quality distribution
        let quality_bucket = match conversation.quality_metrics.overall_quality {
            q if q >= 0.9 => "excellent",
            q if q >= 0.7 => "good",
            q if q >= 0.5 => "fair",
            _ => "poor",
        };
        *self.quality_distribution.entry(quality_bucket.to_string()).or_insert(0) += 1;

        // Update topic frequency
        for topic in &conversation.metadata.topics {
            *self.topic_frequency.entry(topic.clone()).or_insert(0) += 1;
        }

        // Add quality trend
        self.data_quality_trends.push(QualityTrend {
            timestamp: Utc::now(),
            quality_score: conversation.quality_metrics.overall_quality,
            conversation_count: self.total_conversations,
            improvement_areas: Vec::new(),
        });

        Ok(())
    }
}

impl DatasetFilter {
    fn matches(&self, conversation: &ConversationRecord) -> bool {
        // Check quality range
        if let Some(min_quality) = self.min_quality {
            if conversation.quality_metrics.overall_quality < min_quality {
                return false;
            }
        }

        if let Some(max_quality) = self.max_quality {
            if conversation.quality_metrics.overall_quality > max_quality {
                return false;
            }
        }

        // Check conversation types
        if let Some(ref types) = self.conversation_types {
            if !types.contains(&conversation.metadata.conversation_type) {
                return false;
            }
        }

        // Check complexity levels
        if let Some(ref levels) = self.complexity_levels {
            if !levels.contains(&conversation.metadata.complexity_level) {
                return false;
            }
        }

        // Check topics
        if let Some(ref topics) = self.topics {
            let has_matching_topic = topics.iter()
                .any(|topic| conversation.metadata.topics.contains(topic));
            if !has_matching_topic {
                return false;
            }
        }

        // Check date range
        if let Some((start_date, end_date)) = self.date_range {
            if conversation.created_at < start_date || conversation.created_at > end_date {
                return false;
            }
        }

        true
    }
}

impl TrainingDataset {
    fn new(
        conversations: Vec<&ConversationRecord>,
        format: &ExportFormat,
        _analytics: &ConversationAnalytics,
    ) -> Result<Self, BrainError> {
        let owned_conversations: Vec<ConversationRecord> = conversations
            .into_iter()
            .cloned()
            .collect();

        let statistics = Self::calculate_statistics(&owned_conversations)?;

        let metadata = DatasetMetadata {
            created_at: Utc::now(),
            version: "1.0".to_string(),
            format: format.clone(),
            total_conversations: owned_conversations.len(),
            total_messages: owned_conversations.iter().map(|c| c.messages.len()).sum(),
            quality_threshold: 0.7,
        };

        Ok(Self {
            conversations: owned_conversations,
            metadata,
            statistics,
        })
    }

    fn calculate_statistics(conversations: &[ConversationRecord]) -> Result<DatasetStatistics, BrainError> {
        let mut quality_distribution = HashMap::new();
        let mut topic_distribution = HashMap::new();
        let mut complexity_distribution = HashMap::new();
        let mut conversation_type_distribution = HashMap::new();

        let mut total_quality = 0.0;
        let mut total_length = 0.0;

        for conversation in conversations {
            // Quality distribution
            let quality_bucket = match conversation.quality_metrics.overall_quality {
                q if q >= 0.9 => "excellent",
                q if q >= 0.7 => "good",
                q if q >= 0.5 => "fair",
                _ => "poor",
            };
            *quality_distribution.entry(quality_bucket.to_string()).or_insert(0) += 1;

            // Topic distribution
            for topic in &conversation.metadata.topics {
                *topic_distribution.entry(topic.clone()).or_insert(0) += 1;
            }

            // Complexity distribution
            let complexity_str = format!("{:?}", conversation.metadata.complexity_level);
            *complexity_distribution.entry(complexity_str).or_insert(0) += 1;

            // Conversation type distribution
            let type_str = format!("{:?}", conversation.metadata.conversation_type);
            *conversation_type_distribution.entry(type_str).or_insert(0) += 1;

            total_quality += conversation.quality_metrics.overall_quality;
            total_length += conversation.messages.len() as f64;
        }

        let average_quality = if !conversations.is_empty() {
            total_quality / conversations.len() as f64
        } else {
            0.0
        };

        let average_conversation_length = if !conversations.is_empty() {
            total_length / conversations.len() as f64
        } else {
            0.0
        };

        Ok(DatasetStatistics {
            quality_distribution,
            topic_distribution,
            complexity_distribution,
            conversation_type_distribution,
            average_quality,
            average_conversation_length,
        })
    }

    async fn save_to_file(&self, path: &str) -> Result<(), BrainError> {
        match self.metadata.format {
            ExportFormat::JsonL => self.save_as_jsonl(path).await,
            ExportFormat::Csv => self.save_as_csv(path).await,
            _ => self.save_as_json(path).await,
        }
    }

    async fn save_as_jsonl(&self, path: &str) -> Result<(), BrainError> {
        let mut content = String::new();
        for conversation in &self.conversations {
            let line = serde_json::to_string(conversation)
                .map_err(|e| BrainError::ConfigError(format!("Failed to serialize conversation: {}", e)))?;
            content.push_str(&line);
            content.push('\n');
        }

        fs::write(path, content)
            .map_err(|e| BrainError::Io { source: e })?;
        Ok(())
    }

    async fn save_as_json(&self, path: &str) -> Result<(), BrainError> {
        let json_data = serde_json::to_string_pretty(self)
            .map_err(|e| BrainError::ConfigError(format!("Failed to serialize dataset: {}", e)))?;
        
        fs::write(path, json_data)
            .map_err(|e| BrainError::Io { source: e })?;
        Ok(())
    }

    async fn save_as_csv(&self, path: &str) -> Result<(), BrainError> {
        let mut content = String::new();
        content.push_str("conversation_id,role,content,timestamp,quality_score\n");

        for conversation in &self.conversations {
            for message in &conversation.messages {
                let quality_score = message.response_quality
                    .as_ref()
                    .map(|q| q.overall_score().to_string())
                    .unwrap_or_else(|| "N/A".to_string());

                content.push_str(&format!(
                    "{},{},{},{},{}\n",
                    conversation.conversation_id,
                    message.role,
                    message.content.replace(',', ";").replace('\n', " "),
                    message.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    quality_score
                ));
            }
        }

        fs::write(path, content)
            .map_err(|e| BrainError::Io { source: e })?;
        Ok(())
    }
} 