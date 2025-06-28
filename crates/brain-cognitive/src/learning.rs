//! Curiosity-Driven Learning Module
//! 
//! This module implements curiosity-driven learning for Brain AI, creating intelligent 
//! learning priorities based on novelty detection, knowledge gaps, and meta-memory insights.
//! 
//! ## Architecture
//! 
//! The curiosity-driven learning system follows hexagonal architecture with:
//! - **Core Domain**: Learning priorities, curiosity drives, knowledge gaps
//! - **Ports**: Trait-based interfaces for novelty detection and meta-memory
//! - **Adapters**: Concrete implementations for different learning strategies
//! 
//! ## Key Features:
//! - Learning priority scoring based on novelty and knowledge gaps
//! - Curiosity modeling with multiple drives (novelty, uncertainty, progress)
//! - Adaptive attention allocation to maximize learning efficiency
//! - Interest persistence and pattern tracking
//! - Integration with meta-memory and novelty detection systems
//! - Learning outcome tracking and strategy optimization

use async_trait::async_trait;
use brain_types::error::BrainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::meta::{KnowledgeType, MetaMemoryService};

/// Configuration for curiosity-driven learning system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityConfig {
    /// Weight for novelty-driven curiosity (0.0-1.0)
    pub novelty_weight: f64,
    /// Weight for uncertainty-driven curiosity (0.0-1.0)
    pub uncertainty_weight: f64,
    /// Weight for progress-driven curiosity (0.0-1.0)
    pub progress_weight: f64,
    /// Minimum curiosity score to trigger learning (0.0-1.0)
    pub learning_threshold: f64,
    /// Maximum number of learning priorities to maintain
    pub max_learning_priorities: usize,
    /// Exploration vs exploitation balance (0.0=exploit, 1.0=explore)
    pub exploration_rate: f64,
    /// Learning rate for curiosity model updates
    pub learning_rate: f64,
    /// Decay rate for interest over time
    pub interest_decay_rate: f64,
    /// Minimum confidence threshold for considering knowledge reliable
    pub confidence_threshold: f64,
    /// Window size for tracking learning progress
    pub progress_window_size: usize,
}

impl Default for CuriosityConfig {
    fn default() -> Self {
        Self {
            novelty_weight: 0.4,
            uncertainty_weight: 0.3,
            progress_weight: 0.3,
            learning_threshold: 0.3,
            max_learning_priorities: 100,
            exploration_rate: 0.6,
            learning_rate: 0.1,
            interest_decay_rate: 0.01,
            confidence_threshold: 0.7,
            progress_window_size: 20,
        }
    }
}

/// Types of curiosity drives
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CuriosityDrive {
    /// Driven by novelty - seeking new and unexpected information
    NoveltySeeker,
    /// Driven by uncertainty - wanting to resolve ambiguous situations
    UncertaintyResolver,
    /// Driven by progress - seeking to improve understanding
    ProgressOptimizer,
    /// Driven by pattern completion - filling in missing pieces
    PatternCompleter,
    /// Driven by contradiction - resolving conflicting information
    ConflictResolver,
}

impl std::fmt::Display for CuriosityDrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CuriosityDrive::NoveltySeeker => write!(f, "Novelty Seeker"),
            CuriosityDrive::UncertaintyResolver => write!(f, "Uncertainty Resolver"),
            CuriosityDrive::ProgressOptimizer => write!(f, "Progress Optimizer"),
            CuriosityDrive::PatternCompleter => write!(f, "Pattern Completer"),
            CuriosityDrive::ConflictResolver => write!(f, "Conflict Resolver"),
        }
    }
}

/// Learning priority item representing something to focus on
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPriority {
    /// Unique identifier
    pub id: Uuid,
    /// Input or topic that triggered curiosity
    pub content: String,
    /// Overall curiosity score (0.0-1.0)
    pub curiosity_score: f64,
    /// Dominant curiosity drive
    pub primary_drive: CuriosityDrive,
    /// Breakdown of curiosity by drive type
    pub drive_scores: HashMap<CuriosityDrive, f64>,
    /// Knowledge gaps identified
    pub knowledge_gaps: Vec<KnowledgeGap>,
    /// Expected learning value
    pub expected_value: f64,
    /// Current learning progress (0.0-1.0)
    pub progress: f64,
    /// Number of learning attempts
    pub attempt_count: u32,
    /// Success rate of learning attempts
    pub success_rate: f64,
    /// Time when priority was created
    pub created_at: DateTime<Utc>,
    /// Time when priority was last accessed
    pub last_accessed_at: DateTime<Utc>,
    /// Time when priority expires
    pub expires_at: Option<DateTime<Utc>>,
    /// Associated metadata
    pub metadata: HashMap<String, String>,
    /// Whether this priority is currently active
    pub is_active: bool,
}

impl LearningPriority {
    /// Create a new learning priority
    pub fn new(content: String, curiosity_score: f64, primary_drive: CuriosityDrive) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content,
            curiosity_score: curiosity_score.clamp(0.0, 1.0),
            primary_drive,
            drive_scores: HashMap::new(),
            knowledge_gaps: Vec::new(),
            expected_value: 0.0,
            progress: 0.0,
            attempt_count: 0,
            success_rate: 0.0,
            created_at: now,
            last_accessed_at: now,
            expires_at: None,
            metadata: HashMap::new(),
            is_active: true,
        }
    }

    /// Update progress and success metrics
    pub fn update_progress(&mut self, new_progress: f64, success: bool) {
        self.progress = new_progress.clamp(0.0, 1.0);
        self.attempt_count += 1;
        
        if success {
            let success_count = (self.success_rate * (self.attempt_count - 1) as f64) + 1.0;
            self.success_rate = success_count / self.attempt_count as f64;
        } else {
            let success_count = self.success_rate * (self.attempt_count - 1) as f64;
            self.success_rate = success_count / self.attempt_count as f64;
        }
        
        self.last_accessed_at = Utc::now();
    }

    /// Calculate current priority score based on various factors
    pub fn calculate_priority_score(&self, config: &CuriosityConfig) -> f64 {
        let base_score = self.curiosity_score;
        let progress_bonus = (1.0 - self.progress) * 0.2; // More bonus for less progress
        let success_penalty = if self.success_rate < 0.3 { 0.1 } else { 0.0 };
        let age_factor = {
            let age_hours = Utc::now().signed_duration_since(self.created_at).num_hours() as f64;
            // Slight decay over time to prevent stale priorities
            (1.0 - config.interest_decay_rate * age_hours).max(0.1)
        };
        
        (base_score + progress_bonus - success_penalty) * age_factor
    }
}

/// Knowledge gap identified by the curiosity system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGap {
    /// Unique identifier for the gap
    pub id: Uuid,
    /// Type of knowledge with the gap
    pub knowledge_type: KnowledgeType,
    /// Specific area or concept with the gap
    pub topic: String,
    /// Confidence level in current knowledge (0.0-1.0)
    pub current_confidence: f64,
    /// Desired confidence level
    pub target_confidence: f64,
    /// Importance of filling this gap (0.0-1.0)
    pub importance: f64,
    /// Estimated effort to fill the gap
    pub estimated_effort: f64,
    /// Related knowledge components
    pub related_components: Vec<Uuid>,
}

impl KnowledgeGap {
    /// Create a new knowledge gap
    pub fn new(
        knowledge_type: KnowledgeType,
        topic: String,
        current_confidence: f64,
        target_confidence: f64,
        importance: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            knowledge_type,
            topic,
            current_confidence: current_confidence.clamp(0.0, 1.0),
            target_confidence: target_confidence.clamp(0.0, 1.0),
            importance: importance.clamp(0.0, 1.0),
            estimated_effort: 1.0,
            related_components: Vec::new(),
        }
    }

    /// Calculate the urgency of addressing this gap
    pub fn calculate_urgency(&self) -> f64 {
        let confidence_deficit = (self.target_confidence - self.current_confidence).max(0.0);
        confidence_deficit * self.importance
    }
}

/// Interest model for learning preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestModel {
    /// Preferred curiosity drives
    pub drive_preferences: HashMap<CuriosityDrive, f64>,
    /// Learning success rates by knowledge type
    pub type_success_rates: HashMap<KnowledgeType, f64>,
    /// Topics of high interest
    pub high_interest_topics: HashMap<String, f64>,
    /// Recent learning history
    pub recent_learning: VecDeque<LearningEvent>,
    /// Adaptation parameters
    pub adaptation_rate: f64,
    /// Last model update time
    pub last_updated: DateTime<Utc>,
}

impl Default for InterestModel {
    fn default() -> Self {
        Self {
            drive_preferences: HashMap::new(),
            type_success_rates: HashMap::new(),
            high_interest_topics: HashMap::new(),
            recent_learning: VecDeque::new(),
            adaptation_rate: 0.1,
            last_updated: Utc::now(),
        }
    }
}

/// Learning event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    /// Event identifier
    pub id: Uuid,
    /// Learning priority that triggered this event
    pub priority_id: Uuid,
    /// Content that was learned about
    pub content: String,
    /// Curiosity drive that initiated learning
    pub drive: CuriosityDrive,
    /// Knowledge type involved
    pub knowledge_type: KnowledgeType,
    /// Whether the learning was successful
    pub success: bool,
    /// Learning progress achieved
    pub progress_gained: f64,
    /// Time spent learning
    pub duration_minutes: f64,
    /// Satisfaction level with the learning (0.0-1.0)
    pub satisfaction: f64,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
}

impl LearningEvent {
    /// Create a new learning event
    pub fn new(
        priority_id: Uuid,
        content: String,
        drive: CuriosityDrive,
        knowledge_type: KnowledgeType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            priority_id,
            content,
            drive,
            knowledge_type,
            success: false,
            progress_gained: 0.0,
            duration_minutes: 0.0,
            satisfaction: 0.0,
            timestamp: Utc::now(),
        }
    }
}

/// Statistics for curiosity learning system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityStats {
    /// Total learning priorities created
    pub total_priorities: usize,
    /// Active learning priorities
    pub active_priorities: usize,
    /// Completed learning priorities
    pub completed_priorities: usize,
    /// Average curiosity score
    pub average_curiosity_score: f64,
    /// Learning success rate
    pub overall_success_rate: f64,
    /// Most common curiosity drives
    pub drive_distribution: HashMap<CuriosityDrive, usize>,
    /// Knowledge gaps by type
    pub gaps_by_type: HashMap<KnowledgeType, usize>,
    /// Average learning progress
    pub average_progress: f64,
    /// Learning events in recent period
    pub recent_learning_events: usize,
    /// Top interest areas
    pub top_interests: Vec<(String, f64)>,
}

impl Default for CuriosityStats {
    fn default() -> Self {
        Self {
            total_priorities: 0,
            active_priorities: 0,
            completed_priorities: 0,
            average_curiosity_score: 0.0,
            overall_success_rate: 0.0,
            drive_distribution: HashMap::new(),
            gaps_by_type: HashMap::new(),
            average_progress: 0.0,
            recent_learning_events: 0,
            top_interests: Vec::new(),
        }
    }
}

/// Novelty assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyAssessment {
    /// Overall novelty score (0.0-1.0)
    pub novelty_score: f64,
    /// Novelty level classification
    pub novelty_level: NoveltyLevel,
    /// Specific aspects that contribute to novelty
    pub novelty_factors: Vec<String>,
    /// Confidence in the assessment
    pub assessment_confidence: f64,
}

/// Novelty level classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NoveltyLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Trait for novelty detection services
#[async_trait]
pub trait NoveltyDetector: Send + Sync {
    /// Assess the novelty of input content
    async fn assess_novelty(&self, input: &str) -> Result<NoveltyAssessment, BrainError>;
    
    /// Update novelty models based on new input
    async fn update_models(&mut self, input: &str) -> Result<(), BrainError>;
}

/// Trait for curiosity learning services
#[async_trait]
pub trait CuriosityLearningService: Send + Sync {
    /// Assess curiosity level for given input
    async fn assess_curiosity(&mut self, input: &str) -> Result<f64, BrainError>;
    
    /// Create learning priority based on curiosity assessment
    async fn create_learning_priority(
        &mut self,
        input: &str,
        curiosity_score: f64,
    ) -> Result<LearningPriority, BrainError>;
    
    /// Get top learning priorities
    async fn get_top_priorities(&self, limit: usize) -> Result<Vec<LearningPriority>, BrainError>;
    
    /// Record a learning event
    async fn record_learning_event(&mut self, event: LearningEvent) -> Result<(), BrainError>;
    
    /// Update learning progress for a priority
    async fn update_progress(
        &mut self,
        priority_id: Uuid,
        progress: f64,
        success: bool,
    ) -> Result<(), BrainError>;
    
    /// Get curiosity statistics
    async fn get_stats(&self) -> Result<CuriosityStats, BrainError>;
}

/// Main curiosity-driven learning engine
pub struct CuriosityLearningEngine {
    /// Configuration
    config: CuriosityConfig,
    /// Meta-memory service for knowledge gap detection
    meta_memory: Arc<MetaMemoryService>,
    /// Novelty detector for assessing input novelty
    novelty_detector: Arc<dyn NoveltyDetector>,
    /// Current learning priorities (sorted by priority score)
    learning_priorities: Arc<RwLock<BTreeMap<String, LearningPriority>>>,
    /// Interest model for learning preferences
    interest_model: Arc<RwLock<InterestModel>>,
    /// System statistics
    stats: Arc<RwLock<CuriosityStats>>,
}

impl CuriosityLearningEngine {
    /// Create a new curiosity learning engine
    pub fn new(
        config: CuriosityConfig,
        meta_memory: Arc<MetaMemoryService>,
        novelty_detector: Arc<dyn NoveltyDetector>,
    ) -> Self {
        Self {
            config,
            meta_memory,
            novelty_detector,
            learning_priorities: Arc::new(RwLock::new(BTreeMap::new())),
            interest_model: Arc::new(RwLock::new(InterestModel::default())),
            stats: Arc::new(RwLock::new(CuriosityStats::default())),
        }
    }

    /// Calculate novelty-based curiosity
    async fn calculate_novelty_curiosity(&self, novelty_assessment: &NoveltyAssessment) -> f64 {
        let base_novelty = novelty_assessment.novelty_score;
        let confidence_factor = novelty_assessment.assessment_confidence;
        
        // Weight novelty by assessment confidence
        base_novelty * confidence_factor * self.config.novelty_weight
    }

    /// Calculate uncertainty-based curiosity from knowledge gaps
    async fn calculate_uncertainty_curiosity(&self, knowledge_gaps: &[KnowledgeGap]) -> f64 {
        if knowledge_gaps.is_empty() {
            return 0.0;
        }

        let total_uncertainty: f64 = knowledge_gaps
            .iter()
            .map(|gap| gap.calculate_urgency())
            .sum();
        
        let avg_uncertainty = total_uncertainty / knowledge_gaps.len() as f64;
        avg_uncertainty * self.config.uncertainty_weight
    }

    /// Calculate progress-based curiosity
    async fn calculate_progress_curiosity(&self, _input: &str) -> f64 {
        // For now, return a moderate progress curiosity
        // In a full implementation, this would analyze recent learning progress
        0.5 * self.config.progress_weight
    }

    /// Identify knowledge gaps for given input
    async fn identify_knowledge_gaps(&self, input: &str) -> Result<Vec<KnowledgeGap>, BrainError> {
        // Query meta-memory for low-confidence items related to input
        let low_confidence_items = self.meta_memory.get_low_confidence_components().await
            .map_err(|e| BrainError::Other(format!("Meta-memory query failed: {}", e)))?;

        let mut gaps = Vec::new();
        for item in low_confidence_items {
            // Simple topic extraction (in practice, this would be more sophisticated)
            let topic = if input.len() > 50 {
                input[..50].to_string()
            } else {
                input.to_string()
            };

            let gap = KnowledgeGap::new(
                item.knowledge_type,
                topic,
                item.confidence_score,
                0.8, // Target confidence
                0.7, // Importance
            );
            gaps.push(gap);
        }

        Ok(gaps)
    }

    /// Determine primary curiosity drive
    fn determine_primary_drive(
        &self,
        novelty_assessment: &NoveltyAssessment,
        knowledge_gaps: &[KnowledgeGap],
    ) -> CuriosityDrive {
        if novelty_assessment.novelty_score > 0.7 {
            CuriosityDrive::NoveltySeeker
        } else if !knowledge_gaps.is_empty() {
            let avg_uncertainty: f64 = knowledge_gaps
                .iter()
                .map(|gap| gap.calculate_urgency())
                .sum::<f64>() / knowledge_gaps.len() as f64;
            
            if avg_uncertainty > 0.6 {
                CuriosityDrive::UncertaintyResolver
            } else {
                CuriosityDrive::ProgressOptimizer
            }
        } else {
            CuriosityDrive::PatternCompleter
        }
    }

    /// Update interest model based on learning event
    async fn update_interest_model(&self, event: &LearningEvent) {
        let mut model = self.interest_model.write().await;
        
        // Update drive preferences based on success
        let current_pref = model.drive_preferences.get(&event.drive).copied().unwrap_or(0.5);
        let adjustment = if event.success { 0.1 } else { -0.05 };
        let new_pref = (current_pref + adjustment).clamp(0.0, 1.0);
        model.drive_preferences.insert(event.drive.clone(), new_pref);
        
        // Update knowledge type success rates
        let current_rate = model.type_success_rates.get(&event.knowledge_type).copied().unwrap_or(0.5);
        let new_rate = if event.success {
            (current_rate * 0.9 + 0.1).min(1.0)
        } else {
            (current_rate * 0.9).max(0.0)
        };
        model.type_success_rates.insert(event.knowledge_type.clone(), new_rate);
        
        // Add to recent learning history
        model.recent_learning.push_back(event.clone());
        if model.recent_learning.len() > self.config.progress_window_size {
            model.recent_learning.pop_front();
        }
        
        model.last_updated = Utc::now();
    }

    /// Update system statistics
    async fn update_stats(&self) {
        let priorities = self.learning_priorities.read().await;
        let mut stats = self.stats.write().await;
        
        stats.total_priorities = priorities.len();
        stats.active_priorities = priorities.values().filter(|p| p.is_active).count();
        stats.completed_priorities = priorities.values().filter(|p| p.progress >= 1.0).count();
        
        if !priorities.is_empty() {
            stats.average_curiosity_score = priorities.values()
                .map(|p| p.curiosity_score)
                .sum::<f64>() / priorities.len() as f64;
            
            stats.average_progress = priorities.values()
                .map(|p| p.progress)
                .sum::<f64>() / priorities.len() as f64;
            
            stats.overall_success_rate = priorities.values()
                .map(|p| p.success_rate)
                .sum::<f64>() / priorities.len() as f64;
        }
        
        // Update drive distribution
        stats.drive_distribution.clear();
        for priority in priorities.values() {
            *stats.drive_distribution.entry(priority.primary_drive.clone()).or_insert(0) += 1;
        }
    }
}

#[async_trait]
impl CuriosityLearningService for CuriosityLearningEngine {
    /// Assess curiosity level for given input
    async fn assess_curiosity(&mut self, input: &str) -> Result<f64, BrainError> {
        // Get novelty assessment
        let novelty_assessment = self.novelty_detector.assess_novelty(input).await?;
        
        // Identify knowledge gaps
        let knowledge_gaps = self.identify_knowledge_gaps(input).await?;
        
        // Calculate different types of curiosity
        let novelty_curiosity = self.calculate_novelty_curiosity(&novelty_assessment).await;
        let uncertainty_curiosity = self.calculate_uncertainty_curiosity(&knowledge_gaps).await;
        let progress_curiosity = self.calculate_progress_curiosity(input).await;
        
        // Combine curiosity scores
        let total_curiosity = novelty_curiosity + uncertainty_curiosity + progress_curiosity;
        
        Ok(total_curiosity.clamp(0.0, 1.0))
    }

    /// Create learning priority based on curiosity assessment
    async fn create_learning_priority(
        &mut self,
        input: &str,
        curiosity_score: f64,
    ) -> Result<LearningPriority, BrainError> {
        if curiosity_score < self.config.learning_threshold {
            return Err(BrainError::InvalidInput(
                "Curiosity score below learning threshold".to_string()
            ));
        }

        // Get novelty assessment and knowledge gaps
        let novelty_assessment = self.novelty_detector.assess_novelty(input).await?;
        let knowledge_gaps = self.identify_knowledge_gaps(input).await?;
        
        // Determine primary drive
        let primary_drive = self.determine_primary_drive(&novelty_assessment, &knowledge_gaps);
        
        // Create learning priority
        let mut priority = LearningPriority::new(
            input.to_string(),
            curiosity_score,
            primary_drive,
        );
        
        priority.knowledge_gaps = knowledge_gaps;
        priority.expected_value = curiosity_score * 0.8; // Simple heuristic
        
        // Store priority
        let priority_key = format!("{:010.6}_{}", 
            1.0 - priority.calculate_priority_score(&self.config), // Inverted for descending order
            priority.id
        );
        
        self.learning_priorities.write().await.insert(priority_key, priority.clone());
        
        // Update statistics
        self.update_stats().await;
        
        Ok(priority)
    }

    /// Get top learning priorities
    async fn get_top_priorities(&self, limit: usize) -> Result<Vec<LearningPriority>, BrainError> {
        let priorities = self.learning_priorities.read().await;
        let top_priorities: Vec<LearningPriority> = priorities
            .values()
            .filter(|p| p.is_active)
            .take(limit)
            .cloned()
            .collect();
        
        Ok(top_priorities)
    }

    /// Record a learning event
    async fn record_learning_event(&mut self, event: LearningEvent) -> Result<(), BrainError> {
        // Update interest model
        self.update_interest_model(&event).await;
        
        // Update corresponding learning priority if it exists
        let mut priorities = self.learning_priorities.write().await;
        if let Some((_, priority)) = priorities.iter_mut()
            .find(|(_, p)| p.id == event.priority_id) {
            priority.update_progress(event.progress_gained, event.success);
        }
        
        // Update statistics
        drop(priorities); // Release the write lock
        self.update_stats().await;
        
        Ok(())
    }

    /// Update learning progress for a priority
    async fn update_progress(
        &mut self,
        priority_id: Uuid,
        progress: f64,
        success: bool,
    ) -> Result<(), BrainError> {
        let mut priorities = self.learning_priorities.write().await;
        
        if let Some((_, priority)) = priorities.iter_mut()
            .find(|(_, p)| p.id == priority_id) {
            priority.update_progress(progress, success);
            Ok(())
        } else {
            Err(BrainError::NotFound(format!("Learning priority not found: {}", priority_id)))
        }
    }

    /// Get curiosity statistics
    async fn get_stats(&self) -> Result<CuriosityStats, BrainError> {
        let stats = self.stats.read().await;
        Ok(stats.clone())
    }
}

/// Builder for curiosity learning engine
pub struct CuriosityLearningEngineBuilder {
    config: Option<CuriosityConfig>,
    meta_memory: Option<Arc<MetaMemoryService>>,
    novelty_detector: Option<Arc<dyn NoveltyDetector>>,
}

impl CuriosityLearningEngineBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: None,
            meta_memory: None,
            novelty_detector: None,
        }
    }

    /// Set configuration
    pub fn with_config(mut self, config: CuriosityConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set meta-memory service
    pub fn with_meta_memory(mut self, meta_memory: Arc<MetaMemoryService>) -> Self {
        self.meta_memory = Some(meta_memory);
        self
    }

    /// Set novelty detector
    pub fn with_novelty_detector(mut self, novelty_detector: Arc<dyn NoveltyDetector>) -> Self {
        self.novelty_detector = Some(novelty_detector);
        self
    }

    /// Build the curiosity learning engine
    pub fn build(self) -> Result<CuriosityLearningEngine, BrainError> {
        let config = self.config.unwrap_or_default();
        let meta_memory = self.meta_memory.ok_or_else(|| 
            BrainError::ConfigError("Meta-memory service is required".to_string())
        )?;
        let novelty_detector = self.novelty_detector.ok_or_else(|| 
            BrainError::ConfigError("Novelty detector is required".to_string())
        )?;

        Ok(CuriosityLearningEngine::new(config, meta_memory, novelty_detector))
    }
}

impl Default for CuriosityLearningEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
} 