use crate::error::BrainError;
use crate::conversation::{ConversationContext, ChatMessage, RagResponse, ResponseQuality, RetrievedKnowledge};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataCollector {
    config: TrainingDataConfig,
    conversation_storage: HashMap<String, ConversationRecord>,
    quality_assessor: QualityAssessor,
    anonymizer: DataAnonymizer,
    analytics: ConversationAnalytics,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    JsonL,
    Parquet,
    Csv,
    HuggingFace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRecord {
    pub conversation_id: String,
    pub messages: Vec<MessageRecord>,
    pub metadata: ConversationMetadata,
    pub quality_metrics: ConversationQualityMetrics,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationType {
    QuestionsAndAnswers,
    Tutorial,
    ProblemSolving,
    Research,
    Casual,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserExpertise {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSourceRecord {
    pub source_type: String,
    pub content_summary: String,
    pub relevance_score: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub satisfaction_score: f64, // 0.0-1.0
    pub helpfulness: f64,
    pub accuracy: f64,
    pub clarity: f64,
    pub feedback_text: Option<String>,
    pub timestamp: DateTime<Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessor {
    pub quality_models: Vec<QualityModel>,
    pub thresholds: QualityThresholds,
    pub pattern_analyzers: Vec<PatternAnalyzer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityModel {
    pub name: String,
    pub model_type: QualityModelType,
    pub weight: f64,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityModelType {
    CoherenceAnalyzer,
    FactualAccuracyChecker,
    RelevanceScorer,
    SafetyValidator,
    EducationalValueAssessor,
    DiversityMeasurer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityThresholds {
    pub minimum_quality: f64,
    pub excellent_quality: f64,
    pub coherence_threshold: f64,
    pub safety_threshold: f64,
    pub relevance_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternAnalyzer {
    pub analyzer_type: PatternType,
    pub patterns: Vec<ConversationPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    TopicFlow,
    QuestionAnswerPairs,
    ErrorCorrection,
    LearningProgression,
    ConceptIntroduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPattern {
    pub pattern_id: String,
    pub description: String,
    pub frequency: f64,
    pub quality_impact: f64,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAnonymizer {
    pub anonymization_rules: Vec<AnonymizationRule>,
    pub pii_detectors: Vec<PiiDetector>,
    pub replacement_strategies: HashMap<String, ReplacementStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationRule {
    pub rule_type: PiiType,
    pub pattern: String,
    pub replacement: String,
    pub confidence_threshold: f64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiDetector {
    pub detector_type: PiiType,
    pub regex_patterns: Vec<String>,
    pub confidence_scoring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplacementStrategy {
    Mask,
    Synthetic,
    Removal,
    Placeholder,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrend {
    pub timestamp: DateTime<Utc>,
    pub quality_score: f64,
    pub conversation_count: usize,
    pub improvement_areas: Vec<String>,
}

impl Default for TrainingDataConfig {
    fn default() -> Self {
        Self {
            storage_path: "training_data".to_string(),
            max_conversations: 10000,
            quality_threshold: 0.7,
            enable_anonymization: true,
            retention_days: 365,
            batch_size: 100,
            auto_export: true,
            export_format: ExportFormat::JsonL,
        }
    }
}

impl TrainingDataCollector {
    pub fn new(config: TrainingDataConfig) -> Result<Self, BrainError> {
        // Create storage directory if it doesn't exist
        if !Path::new(&config.storage_path).exists() {
            fs::create_dir_all(&config.storage_path)
                .map_err(|e| BrainError::Io(format!("Failed to create storage directory: {}", e)))?;
        }

        let quality_assessor = QualityAssessor::new()?;
        let anonymizer = DataAnonymizer::new()?;
        let analytics = ConversationAnalytics::new();

        Ok(Self {
            config,
            conversation_storage: HashMap::new(),
            quality_assessor,
            anonymizer,
            analytics,
        })
    }

    pub async fn capture_conversation(
        &mut self,
        conversation_id: &str,
        user_message: &str,
        assistant_response: &RagResponse,
        context: &ConversationContext,
        knowledge_sources: &[RetrievedKnowledge],
    ) -> Result<(), BrainError> {
        // Create or get existing conversation record
        let conversation = self.conversation_storage
            .entry(conversation_id.to_string())
            .or_insert_with(|| ConversationRecord::new(conversation_id));

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

        // Add messages to conversation
        conversation.messages.push(anonymized_user_msg);
        conversation.messages.push(anonymized_assistant_msg);

        // Update conversation metadata
        self.update_conversation_metadata(conversation, context).await?;

        // Assess conversation quality
        let quality_metrics = self.quality_assessor
            .assess_conversation_quality(conversation).await?;
        conversation.quality_metrics = quality_metrics;

        // Update analytics
        self.analytics.update_with_conversation(conversation)?;

        // Auto-export if configured and quality threshold met
        if self.config.auto_export && 
           conversation.quality_metrics.overall_quality >= self.config.quality_threshold {
            self.export_conversation(conversation_id).await?;
        }

        Ok(())
    }

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

    pub fn get_conversation_analytics(&self) -> &ConversationAnalytics {
        &self.analytics
    }

    pub fn get_quality_distribution(&self) -> HashMap<String, f64> {
        let mut distribution = HashMap::new();
        let total = self.conversation_storage.len() as f64;

        for conversation in self.conversation_storage.values() {
            let quality_bucket = self.get_quality_bucket(conversation.quality_metrics.overall_quality);
            *distribution.entry(quality_bucket).or_insert(0.0) += 1.0 / total;
        }

        distribution
    }

    async fn update_conversation_metadata(
        &self,
        conversation: &mut ConversationRecord,
        context: &ConversationContext,
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
        complexity_score /= conversation.messages.len() as f64;

        Ok(match complexity_score {
            s if s < 10.0 => ComplexityLevel::Simple,
            s if s < 25.0 => ComplexityLevel::Moderate,
            s if s < 50.0 => ComplexityLevel::Complex,
            _ => ComplexityLevel::Expert,
        })
    }

    async fn detect_conversation_type(&self, conversation: &ConversationRecord) -> Result<ConversationType, BrainError> {
        let content = conversation.messages.iter()
            .map(|m| &m.content)
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

    async fn extract_topics(&self, conversation: &ConversationRecord) -> Result<Vec<String>, BrainError> {
        let mut topics = Vec::new();
        
        // Simple keyword extraction - in practice, would use more sophisticated NLP
        let content = conversation.messages.iter()
            .map(|m| &m.content)
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

    fn filter_conversations(&self, filter: Option<DatasetFilter>) -> Result<Vec<&ConversationRecord>, BrainError> {
        let mut filtered = Vec::new();

        for conversation in self.conversation_storage.values() {
            if let Some(ref criteria) = filter {
                if !criteria.matches(conversation) {
                    continue;
                }
            }
            
            if conversation.quality_metrics.overall_quality >= self.config.quality_threshold {
                filtered.push(conversation);
            }
        }

        Ok(filtered)
    }

    fn get_quality_bucket(&self, quality: f64) -> String {
        match quality {
            q if q >= 0.9 => "excellent".to_string(),
            q if q >= 0.8 => "good".to_string(),
            q if q >= 0.7 => "acceptable".to_string(),
            q if q >= 0.6 => "poor".to_string(),
            _ => "very_poor".to_string(),
        }
    }

    fn get_file_extension(&self) -> &str {
        match self.config.export_format {
            ExportFormat::JsonL => "jsonl",
            ExportFormat::Parquet => "parquet",
            ExportFormat::Csv => "csv",
            ExportFormat::HuggingFace => "json",
        }
    }

    async fn export_conversation(&self, conversation_id: &str) -> Result<(), BrainError> {
        if let Some(conversation) = self.conversation_storage.get(conversation_id) {
            let export_path = format!("{}/conversation_{}.json", 
                self.config.storage_path, 
                conversation_id
            );
            
            let json = serde_json::to_string_pretty(conversation)
                .map_err(|e| BrainError::Serialization(format!("Failed to serialize conversation: {}", e)))?;
            
            fs::write(export_path, json)
                .map_err(|e| BrainError::Io(format!("Failed to write conversation file: {}", e)))?;
        }

        Ok(())
    }
}

impl ConversationRecord {
    fn new(conversation_id: &str) -> Self {
        Self {
            conversation_id: conversation_id.to_string(),
            messages: Vec::new(),
            metadata: ConversationMetadata::default(),
            quality_metrics: ConversationQualityMetrics::default(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
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
            overall_quality: 0.0,
            coherence_score: 0.0,
            knowledge_grounding: 0.0,
            response_relevance: 0.0,
            safety_score: 1.0,
            educational_value: 0.0,
            diversity_score: 0.0,
            uniqueness_score: 0.0,
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
        let source_records: Vec<KnowledgeSourceRecord> = knowledge_sources.iter()
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
            knowledge_sources: source_records,
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
                weight: 0.2,
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
                weight: 0.15,
                parameters: HashMap::new(),
            },
            QualityModel {
                name: "diversity_measurer".to_string(),
                model_type: QualityModelType::DiversityMeasurer,
                weight: 0.1,
                parameters: HashMap::new(),
            },
        ];

        let thresholds = QualityThresholds {
            minimum_quality: 0.6,
            excellent_quality: 0.9,
            coherence_threshold: 0.7,
            safety_threshold: 0.95,
            relevance_threshold: 0.75,
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
        let mut metrics = ConversationQualityMetrics::default();

        // Assess each quality dimension
        metrics.coherence_score = self.assess_coherence(conversation).await?;
        metrics.knowledge_grounding = self.assess_knowledge_grounding(conversation).await?;
        metrics.response_relevance = self.assess_relevance(conversation).await?;
        metrics.safety_score = self.assess_safety(conversation).await?;
        metrics.educational_value = self.assess_educational_value(conversation).await?;
        metrics.diversity_score = self.assess_diversity(conversation).await?;
        metrics.uniqueness_score = self.assess_uniqueness(conversation).await?;

        // Calculate overall quality as weighted average
        metrics.overall_quality = self.calculate_overall_quality(&metrics);

        Ok(metrics)
    }

    async fn assess_coherence(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        // Simple coherence assessment based on conversation flow
        if conversation.messages.len() < 2 {
            return Ok(1.0);
        }

        let mut coherence_sum = 0.0;
        let mut pair_count = 0;

        for i in 0..conversation.messages.len() - 1 {
            let current = &conversation.messages[i];
            let next = &conversation.messages[i + 1];

            // Simple text similarity check
            let similarity = self.calculate_text_similarity(&current.content, &next.content);
            coherence_sum += similarity;
            pair_count += 1;
        }

        Ok(if pair_count > 0 {
            coherence_sum / pair_count as f64
        } else {
            1.0
        })
    }

    async fn assess_knowledge_grounding(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        let mut grounding_sum = 0.0;
        let mut assistant_messages = 0;

        for message in &conversation.messages {
            if message.role == "assistant" {
                assistant_messages += 1;
                
                // Check if message has knowledge sources
                if !message.knowledge_sources.is_empty() {
                    let avg_relevance: f64 = message.knowledge_sources.iter()
                        .map(|ks| ks.relevance_score)
                        .sum::<f64>() / message.knowledge_sources.len() as f64;
                    grounding_sum += avg_relevance;
                } else {
                    grounding_sum += 0.1; // Low score for unsupported responses
                }
            }
        }

        Ok(if assistant_messages > 0 {
            grounding_sum / assistant_messages as f64
        } else {
            0.0
        })
    }

    async fn assess_relevance(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        // Simple relevance assessment based on response quality metrics
        let mut relevance_sum = 0.0;
        let mut quality_count = 0;

        for message in &conversation.messages {
            if let Some(ref quality) = message.response_quality {
                relevance_sum += quality.relevance;
                quality_count += 1;
            }
        }

        Ok(if quality_count > 0 {
            relevance_sum / quality_count as f64
        } else {
            0.7 // Default relevance
        })
    }

    async fn assess_safety(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        // Safety assessment based on response quality safety scores
        let mut safety_sum = 0.0;
        let mut safety_count = 0;

        for message in &conversation.messages {
            if let Some(ref quality) = message.response_quality {
                safety_sum += quality.safety_score;
                safety_count += 1;
            }
        }

        Ok(if safety_count > 0 {
            safety_sum / safety_count as f64
        } else {
            1.0 // Assume safe by default
        })
    }

    async fn assess_educational_value(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        let mut educational_score = 0.0;

        // Count educational indicators
        let content = conversation.messages.iter()
            .map(|m| &m.content)
            .collect::<Vec<_>>()
            .join(" ");

        let educational_patterns = Regex::new(r"\blearn\b|\bexplain\b|\bunderstand\b|\bexample\b|\bconcept\b").unwrap();
        let matches = educational_patterns.find_iter(&content).count();
        
        educational_score = (matches as f64 / content.split_whitespace().count() as f64) * 10.0;
        educational_score = educational_score.min(1.0);

        Ok(educational_score)
    }

    async fn assess_diversity(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        // Measure vocabulary diversity
        let all_words: Vec<&str> = conversation.messages.iter()
            .flat_map(|m| m.content.split_whitespace())
            .collect();

        let unique_words: HashSet<&str> = all_words.iter().cloned().collect();
        
        let diversity = if all_words.is_empty() {
            0.0
        } else {
            unique_words.len() as f64 / all_words.len() as f64
        };

        Ok(diversity)
    }

    async fn assess_uniqueness(&self, conversation: &ConversationRecord) -> Result<f64, BrainError> {
        // Simple uniqueness assessment - could be enhanced with more sophisticated methods
        let content_length = conversation.messages.iter()
            .map(|m| m.content.len())
            .sum::<usize>();

        // Longer, more detailed conversations tend to be more unique
        let uniqueness = (content_length as f64 / 1000.0).min(1.0);
        
        Ok(uniqueness)
    }

    fn calculate_overall_quality(&self, metrics: &ConversationQualityMetrics) -> f64 {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for model in &self.quality_models {
            let score = match model.model_type {
                QualityModelType::CoherenceAnalyzer => metrics.coherence_score,
                QualityModelType::RelevanceScorer => metrics.response_relevance,
                QualityModelType::SafetyValidator => metrics.safety_score,
                QualityModelType::EducationalValueAssessor => metrics.educational_value,
                QualityModelType::DiversityMeasurer => metrics.diversity_score,
                QualityModelType::FactualAccuracyChecker => metrics.knowledge_grounding,
            };

            weighted_sum += score * model.weight;
            total_weight += model.weight;
        }

        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
    }

    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f64 {
        // Simple Jaccard similarity
        let words1: HashSet<&str> = text1.split_whitespace().collect();
        let words2: HashSet<&str> = text2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
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
                pattern: r"\b\d{3}-\d{3}-\d{4}\b|\b\(\d{3}\)\s*\d{3}-\d{4}\b".to_string(),
                replacement: "[PHONE]".to_string(),
                confidence_threshold: 0.85,
            },
            AnonymizationRule {
                rule_type: PiiType::IpAddress,
                pattern: r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b".to_string(),
                replacement: "[IP_ADDRESS]".to_string(),
                confidence_threshold: 0.95,
            },
        ];

        let pii_detectors = vec![
            PiiDetector {
                detector_type: PiiType::Name,
                regex_patterns: vec![
                    r"\bMr\.\s+[A-Z][a-z]+".to_string(),
                    r"\bMs\.\s+[A-Z][a-z]+".to_string(),
                    r"\bDr\.\s+[A-Z][a-z]+".to_string(),
                ],
                confidence_scoring: true,
            },
        ];

        let mut replacement_strategies = HashMap::new();
        replacement_strategies.insert("email".to_string(), ReplacementStrategy::Placeholder);
        replacement_strategies.insert("phone".to_string(), ReplacementStrategy::Mask);
        replacement_strategies.insert("name".to_string(), ReplacementStrategy::Synthetic);

        Ok(Self {
            anonymization_rules,
            pii_detectors,
            replacement_strategies,
        })
    }

    async fn anonymize_message(&self, message: &MessageRecord) -> Result<MessageRecord, BrainError> {
        let mut anonymized_message = message.clone();
        let mut anonymized_content = message.content.clone();

        // Apply anonymization rules
        for rule in &self.anonymization_rules {
            let regex = Regex::new(&rule.pattern)
                .map_err(|e| BrainError::ProcessingError(format!("Invalid regex pattern: {}", e)))?;
            
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
            q if q >= 0.8 => "good", 
            q if q >= 0.7 => "acceptable",
            q if q >= 0.6 => "poor",
            _ => "very_poor",
        };
        *self.quality_distribution.entry(quality_bucket.to_string()).or_insert(0) += 1;

        // Update topic frequency
        for topic in &conversation.metadata.topics {
            *self.topic_frequency.entry(topic.clone()).or_insert(0) += 1;
        }

        // Add quality trend point
        self.data_quality_trends.push(QualityTrend {
            timestamp: Utc::now(),
            quality_score: conversation.quality_metrics.overall_quality,
            conversation_count: self.total_conversations,
            improvement_areas: Vec::new(),
        });

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetFilter {
    pub min_quality: Option<f64>,
    pub max_quality: Option<f64>,
    pub conversation_types: Option<Vec<ConversationType>>,
    pub complexity_levels: Option<Vec<ComplexityLevel>>,
    pub topics: Option<Vec<String>>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

impl DatasetFilter {
    fn matches(&self, conversation: &ConversationRecord) -> bool {
        if let Some(min_q) = self.min_quality {
            if conversation.quality_metrics.overall_quality < min_q {
                return false;
            }
        }

        if let Some(max_q) = self.max_quality {
            if conversation.quality_metrics.overall_quality > max_q {
                return false;
            }
        }

        if let Some(ref types) = self.conversation_types {
            if !types.contains(&conversation.metadata.conversation_type) {
                return false;
            }
        }

        if let Some(ref levels) = self.complexity_levels {
            if !levels.contains(&conversation.metadata.complexity_level) {
                return false;
            }
        }

        if let Some(ref filter_topics) = self.topics {
            let has_topic = filter_topics.iter()
                .any(|topic| conversation.metadata.topics.contains(topic));
            if !has_topic {
                return false;
            }
        }

        if let Some((start, end)) = self.date_range {
            if conversation.created_at < start || conversation.created_at > end {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataset {
    pub conversations: Vec<ConversationRecord>,
    pub metadata: DatasetMetadata,
    pub statistics: DatasetStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub created_at: DateTime<Utc>,
    pub version: String,
    pub format: ExportFormat,
    pub total_conversations: usize,
    pub total_messages: usize,
    pub quality_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetStatistics {
    pub quality_distribution: HashMap<String, usize>,
    pub topic_distribution: HashMap<String, usize>,
    pub complexity_distribution: HashMap<String, usize>,
    pub conversation_type_distribution: HashMap<String, usize>,
    pub average_quality: f64,
    pub average_conversation_length: f64,
}

impl TrainingDataset {
    fn new(
        conversations: Vec<&ConversationRecord>,
        format: &ExportFormat,
        analytics: &ConversationAnalytics,
    ) -> Result<Self, BrainError> {
        let owned_conversations: Vec<ConversationRecord> = conversations.into_iter().cloned().collect();
        
        let metadata = DatasetMetadata {
            created_at: Utc::now(),
            version: "1.0.0".to_string(),
            format: format.clone(),
            total_conversations: owned_conversations.len(),
            total_messages: owned_conversations.iter().map(|c| c.messages.len()).sum(),
            quality_threshold: 0.7,
        };

        let statistics = Self::calculate_statistics(&owned_conversations)?;

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
        let mut total_length = 0;

        for conversation in conversations {
            // Quality distribution
            let quality_bucket = match conversation.quality_metrics.overall_quality {
                q if q >= 0.9 => "excellent",
                q if q >= 0.8 => "good",
                q if q >= 0.7 => "acceptable", 
                q if q >= 0.6 => "poor",
                _ => "very_poor",
            };
            *quality_distribution.entry(quality_bucket.to_string()).or_insert(0) += 1;

            // Topic distribution
            for topic in &conversation.metadata.topics {
                *topic_distribution.entry(topic.clone()).or_insert(0) += 1;
            }

            // Complexity distribution
            let complexity = format!("{:?}", conversation.metadata.complexity_level);
            *complexity_distribution.entry(complexity).or_insert(0) += 1;

            // Conversation type distribution
            let conv_type = format!("{:?}", conversation.metadata.conversation_type);
            *conversation_type_distribution.entry(conv_type).or_insert(0) += 1;

            total_quality += conversation.quality_metrics.overall_quality;
            total_length += conversation.messages.len();
        }

        let average_quality = if conversations.is_empty() {
            0.0
        } else {
            total_quality / conversations.len() as f64
        };

        let average_conversation_length = if conversations.is_empty() {
            0.0
        } else {
            total_length as f64 / conversations.len() as f64
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
            ExportFormat::JsonL => {
                self.save_as_jsonl(path).await?;
            },
            ExportFormat::Json | ExportFormat::HuggingFace => {
                self.save_as_json(path).await?;
            },
            ExportFormat::Csv => {
                self.save_as_csv(path).await?;
            },
            ExportFormat::Parquet => {
                return Err(BrainError::ProcessingError("Parquet export not yet implemented".to_string()));
            },
        }

        Ok(())
    }

    async fn save_as_jsonl(&self, path: &str) -> Result<(), BrainError> {
        let mut lines = Vec::new();
        
        for conversation in &self.conversations {
            let json_line = serde_json::to_string(conversation)
                .map_err(|e| BrainError::Serialization(format!("Failed to serialize conversation: {}", e)))?;
            lines.push(json_line);
        }

        let content = lines.join("\n");
        fs::write(path, content)
            .map_err(|e| BrainError::Io(format!("Failed to write JSONL file: {}", e)))?;

        Ok(())
    }

    async fn save_as_json(&self, path: &str) -> Result<(), BrainError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| BrainError::Serialization(format!("Failed to serialize dataset: {}", e)))?;
        
        fs::write(path, json)
            .map_err(|e| BrainError::Io(format!("Failed to write JSON file: {}", e)))?;

        Ok(())
    }

    async fn save_as_csv(&self, path: &str) -> Result<(), BrainError> {
        let mut csv_lines = Vec::new();
        
        // Header
        csv_lines.push("conversation_id,message_id,role,content,timestamp,quality_score,topics".to_string());

        for conversation in &self.conversations {
            let topics = conversation.metadata.topics.join(";");
            
            for message in &conversation.messages {
                let quality_score = message.response_quality.as_ref()
                    .map(|q| q.factual_grounding)
                    .unwrap_or(0.0);

                let line = format!(
                    "{},{},{},{},{},{},{}",
                    conversation.conversation_id,
                    message.message_id,
                    message.role,
                    message.content.replace(",", "\\,").replace("\n", " "),
                    message.timestamp.to_rfc3339(),
                    quality_score,
                    topics
                );
                csv_lines.push(line);
            }
        }

        let content = csv_lines.join("\n");
        fs::write(path, content)
            .map_err(|e| BrainError::Io(format!("Failed to write CSV file: {}", e)))?;

        Ok(())
    }
} 