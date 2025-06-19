//! Curiosity-Driven Learning Module
//! 
//! This module implements Task 9.3: Curiosity-Driven Learning
//! 
//! The curiosity-driven learning system creates intelligent learning priorities based on:
//! - Novelty detection for identifying interesting inputs
//! - Meta-memory confidence tracking for knowledge gap detection
//! - Adaptive exploration strategies balancing exploration vs exploitation
//! - Interest modeling to track learning preferences and patterns
//! - Feedback loops connecting curiosity with learning outcomes
//! 
//! ## Key Features:
//! - Learning priority scoring based on novelty and knowledge gaps
//! - Curiosity modeling with multiple drives (novelty, uncertainty, progress)
//! - Adaptive attention allocation to maximize learning efficiency
//! - Interest persistence and pattern tracking
//! - Integration with existing Brain cognitive systems
//! - Learning outcome tracking and strategy optimization

use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use rusqlite::{Connection, params};

use crate::meta_memory::{MetaMemorySystem, MetaMemoryQuery, KnowledgeType};
use crate::novelty_detection::{NoveltyDetectionEngine, NoveltyContext, NoveltyLevel, NoveltyAssessment};

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
    /// Enable persistence of curiosity data
    pub enable_persistence: bool,
    /// Database path for curiosity data storage
    pub database_path: String,
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
            enable_persistence: true,
            database_path: "curiosity_learning.db".to_string(),
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
    /// Novelty assessment that contributed to this priority
    pub novelty_assessment: Option<NoveltyAssessment>,
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
            novelty_assessment: None,
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
    /// Type of knowledge that's missing or uncertain
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

/// Interest model tracking learning preferences and patterns
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
            adaptation_rate: 0.05,
            last_updated: Utc::now(),
        }
    }
}

/// Learning event for tracking curiosity outcomes
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

/// Statistics for curiosity-driven learning system
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

/// Main curiosity-driven learning engine
pub struct CuriosityLearningEngine {
    /// Configuration
    config: CuriosityConfig,
    /// Reference to meta-memory system
    meta_memory: Arc<Mutex<MetaMemorySystem>>,
    /// Reference to novelty detection engine
    novelty_engine: Arc<Mutex<NoveltyDetectionEngine>>,
    /// Current learning priorities (sorted by priority score)
    learning_priorities: BTreeMap<String, LearningPriority>, // Key is priority_score_string for sorting
    /// Interest model for learning preferences
    interest_model: InterestModel,
    /// System statistics
    stats: CuriosityStats,
    /// Database connection for persistence
    connection: Option<Arc<Mutex<Connection>>>,
}

impl CuriosityLearningEngine {
    /// Create a new curiosity-driven learning engine
    pub fn new(
        config: CuriosityConfig,
        meta_memory: Arc<Mutex<MetaMemorySystem>>,
        novelty_engine: Arc<Mutex<NoveltyDetectionEngine>>,
    ) -> Result<Self> {
        let connection = if config.enable_persistence {
            let conn = Connection::open(&config.database_path)
                .with_context(|| format!("Failed to open curiosity database: {}", config.database_path))?;
            Some(Arc::new(Mutex::new(conn)))
        } else {
            None
        };

        let engine = Self {
            config,
            meta_memory,
            novelty_engine,
            learning_priorities: BTreeMap::new(),
            interest_model: InterestModel::default(),
            stats: CuriosityStats::default(),
            connection,
        };

        if engine.config.enable_persistence {
            engine.initialize_database()?;
        }

        Ok(engine)
    }

    /// Initialize database tables for persistence
    pub fn initialize_database(&self) -> Result<()> {
        if let Some(conn) = &self.connection {
            let conn = conn.lock().unwrap();
            
            // Learning priorities table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS learning_priorities (
                    id TEXT PRIMARY KEY,
                    content TEXT NOT NULL,
                    curiosity_score REAL NOT NULL,
                    primary_drive TEXT NOT NULL,
                    drive_scores TEXT NOT NULL,
                    knowledge_gaps TEXT NOT NULL,
                    expected_value REAL NOT NULL,
                    progress REAL NOT NULL,
                    attempt_count INTEGER NOT NULL,
                    success_rate REAL NOT NULL,
                    created_at TEXT NOT NULL,
                    last_accessed_at TEXT NOT NULL,
                    expires_at TEXT,
                    metadata TEXT NOT NULL,
                    is_active BOOLEAN NOT NULL
                )",
                [],
            ).context("Failed to create learning_priorities table")?;

            // Learning events table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS learning_events (
                    id TEXT PRIMARY KEY,
                    priority_id TEXT NOT NULL,
                    content TEXT NOT NULL,
                    drive TEXT NOT NULL,
                    knowledge_type TEXT NOT NULL,
                    success BOOLEAN NOT NULL,
                    progress_gained REAL NOT NULL,
                    duration_minutes REAL NOT NULL,
                    satisfaction REAL NOT NULL,
                    timestamp TEXT NOT NULL
                )",
                [],
            ).context("Failed to create learning_events table")?;

            // Interest model table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS interest_model (
                    id INTEGER PRIMARY KEY,
                    drive_preferences TEXT NOT NULL,
                    type_success_rates TEXT NOT NULL,
                    high_interest_topics TEXT NOT NULL,
                    adaptation_rate REAL NOT NULL,
                    last_updated TEXT NOT NULL
                )",
                [],
            ).context("Failed to create interest_model table")?;
        }
        
        Ok(())
    }

    /// Assess curiosity for a given input
    pub fn assess_curiosity(&mut self, input: &str, context: Option<NoveltyContext>) -> Result<f64> {
        // Get novelty assessment
        let novelty_assessment = {
            let mut novelty_engine = self.novelty_engine.lock().unwrap();
            novelty_engine.assess_novelty(input, context)?
        };

        // Get knowledge gaps from meta-memory
        let knowledge_gaps = self.identify_knowledge_gaps(input)?;

        // Calculate curiosity components
        let novelty_curiosity = self.calculate_novelty_curiosity(&novelty_assessment);
        let uncertainty_curiosity = self.calculate_uncertainty_curiosity(&knowledge_gaps)?;
        let progress_curiosity = self.calculate_progress_curiosity(input)?;

        // Combine with configuration weights
        let total_curiosity = (novelty_curiosity * self.config.novelty_weight) +
            (uncertainty_curiosity * self.config.uncertainty_weight) +
            (progress_curiosity * self.config.progress_weight);

        // Create learning priority if curiosity is high enough
        if total_curiosity >= self.config.learning_threshold {
            self.create_learning_priority(input, total_curiosity, novelty_assessment, knowledge_gaps)?;
        }

        Ok(total_curiosity.clamp(0.0, 1.0))
    }

    /// Calculate novelty-based curiosity
    fn calculate_novelty_curiosity(&self, novelty_assessment: &NoveltyAssessment) -> f64 {
        let base_score = novelty_assessment.novelty_score;
        
        // Boost for high novelty
        let novelty_boost = match novelty_assessment.get_novelty_level(&self.novelty_engine.lock().unwrap().get_config()) {
            NoveltyLevel::High => 0.2,
            NoveltyLevel::Medium => 0.1,
            NoveltyLevel::Low => 0.0,
        };
        
        (base_score + novelty_boost).clamp(0.0, 1.0)
    }

    /// Calculate uncertainty-based curiosity
    fn calculate_uncertainty_curiosity(&self, knowledge_gaps: &[KnowledgeGap]) -> Result<f64> {
        if knowledge_gaps.is_empty() {
            return Ok(0.0);
        }

        let meta_memory = self.meta_memory.lock().unwrap();
        let stats = meta_memory.get_stats();
        
        // Calculate average confidence of low-confidence components
        let low_confidence_score = if stats.low_confidence_count > 0 {
            1.0 - (stats.average_confidence.min(self.config.confidence_threshold) / self.config.confidence_threshold)
        } else {
            0.0
        };

        // Factor in the number and importance of knowledge gaps
        let gap_factor = knowledge_gaps.iter()
            .map(|gap| gap.importance * (1.0 - gap.current_confidence))
            .sum::<f64>() / knowledge_gaps.len() as f64;

        Ok((low_confidence_score * 0.6 + gap_factor * 0.4).clamp(0.0, 1.0))
    }

    /// Calculate progress-based curiosity
    fn calculate_progress_curiosity(&self, input: &str) -> Result<f64> {
        // Check if we have existing learning priorities for similar content
        let similar_priorities: Vec<_> = self.learning_priorities.values()
            .filter(|p| self.content_similarity(&p.content, input) > 0.5)
            .collect();

        if similar_priorities.is_empty() {
            return Ok(0.5); // Moderate curiosity for new topics
        }

        // Calculate average progress and success rate for similar content
        let avg_progress: f64 = similar_priorities.iter()
            .map(|p| p.progress)
            .sum::<f64>() / similar_priorities.len() as f64;

        let avg_success_rate: f64 = similar_priorities.iter()
            .map(|p| p.success_rate)
            .sum::<f64>() / similar_priorities.len() as f64;

        // Higher curiosity for partial progress with good success rate
        let progress_curiosity = if avg_progress > 0.1 && avg_progress < 0.8 && avg_success_rate > 0.3 {
            0.8 - avg_progress // More curious about things we're making progress on
        } else if avg_progress >= 0.8 {
            0.2 // Lower curiosity for well-understood topics
        } else {
            0.4 // Moderate curiosity for topics with little progress
        };

        Ok(progress_curiosity.clamp(0.0, 1.0))
    }

    /// Identify knowledge gaps related to input
    fn identify_knowledge_gaps(&self, _input: &str) -> Result<Vec<KnowledgeGap>> {
        let meta_memory = self.meta_memory.lock().unwrap();
        
        // Query for low-confidence components
        let low_confidence_query = MetaMemoryQuery {
            max_confidence: Some(self.config.confidence_threshold),
            active_only: Some(true),
            limit: Some(20),
            ..Default::default()
        };
        
        let low_confidence_items = meta_memory.query_items(&low_confidence_query)?;
        
        // Convert to knowledge gaps
        let mut gaps = Vec::new();
        for item in low_confidence_items {
            let gap = KnowledgeGap {
                knowledge_type: item.knowledge_type,
                topic: format!("Component {}", item.component_id),
                current_confidence: item.confidence_score,
                target_confidence: self.config.confidence_threshold,
                importance: 1.0 - item.confidence_score, // Lower confidence = higher importance
                estimated_effort: (self.config.confidence_threshold - item.confidence_score) * 2.0,
                related_components: vec![item.component_id],
            };
            gaps.push(gap);
        }

        // Sort by importance
        gaps.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal));
        gaps.truncate(10); // Keep top 10 gaps

        Ok(gaps)
    }

    /// Create a learning priority from curiosity assessment
    fn create_learning_priority(
        &mut self,
        input: &str,
        curiosity_score: f64,
        novelty_assessment: NoveltyAssessment,
        knowledge_gaps: Vec<KnowledgeGap>,
    ) -> Result<()> {
        // Determine primary drive
        let primary_drive = self.determine_primary_drive(curiosity_score, &novelty_assessment, &knowledge_gaps);
        
        // Create learning priority
        let mut priority = LearningPriority::new(input.to_string(), curiosity_score, primary_drive.clone());
        priority.novelty_assessment = Some(novelty_assessment);
        priority.knowledge_gaps = knowledge_gaps;
        priority.expected_value = self.calculate_expected_learning_value(&priority);
        
        // Calculate drive scores
        priority.drive_scores.insert(CuriosityDrive::NoveltySeeker, 
            priority.novelty_assessment.as_ref().map(|n| n.novelty_score).unwrap_or(0.0));
        priority.drive_scores.insert(CuriosityDrive::UncertaintyResolver, 
            priority.knowledge_gaps.iter().map(|g| g.importance).sum::<f64>().min(1.0));
        priority.drive_scores.insert(CuriosityDrive::ProgressOptimizer, 
            self.calculate_progress_curiosity(input).unwrap_or(0.0));

        // Store in priority queue (using score as key for sorting)
        let priority_key = format!("{:010.6}_{}", 
            (1.0 - priority.calculate_priority_score(&self.config)) * 1000000.0, 
            priority.id);
        
        self.learning_priorities.insert(priority_key, priority);
        
        // Maintain size limit
        while self.learning_priorities.len() > self.config.max_learning_priorities {
            if let Some((key, _)) = self.learning_priorities.iter().last().map(|(k, v)| (k.clone(), v.clone())) {
                self.learning_priorities.remove(&key);
            }
        }

        self.update_stats();
        Ok(())
    }

    /// Determine primary curiosity drive
    fn determine_primary_drive(
        &self,
        _curiosity_score: f64,
        novelty_assessment: &NoveltyAssessment,
        knowledge_gaps: &[KnowledgeGap],
    ) -> CuriosityDrive {
        let novelty_score = novelty_assessment.novelty_score;
        let uncertainty_score = knowledge_gaps.iter().map(|g| g.importance).sum::<f64>().min(1.0);
        
        if novelty_score > uncertainty_score && novelty_score > 0.6 {
            CuriosityDrive::NoveltySeeker
        } else if uncertainty_score > 0.5 {
            CuriosityDrive::UncertaintyResolver
        } else {
            CuriosityDrive::ProgressOptimizer
        }
    }

    /// Calculate expected learning value for a priority
    fn calculate_expected_learning_value(&self, priority: &LearningPriority) -> f64 {
        let novelty_value = priority.novelty_assessment.as_ref()
            .map(|n| n.novelty_score * 0.4)
            .unwrap_or(0.0);
        
        let gap_value = priority.knowledge_gaps.iter()
            .map(|g| g.importance * (1.0 - g.current_confidence))
            .sum::<f64>() * 0.6;
        
        (novelty_value + gap_value).clamp(0.0, 1.0)
    }

    /// Get top learning priorities
    pub fn get_top_learning_priorities(&self, limit: usize) -> Vec<&LearningPriority> {
        self.learning_priorities.values().take(limit).collect()
    }

    /// Record a learning event
    pub fn record_learning_event(&mut self, event: LearningEvent) -> Result<()> {
        // Update the corresponding learning priority
        if let Some(priority) = self.learning_priorities.values_mut()
            .find(|p| p.id == event.priority_id) {
            priority.update_progress(event.progress_gained, event.success);
        }

        // Update interest model
        self.update_interest_model(&event);

        // Add to recent learning history
        self.interest_model.recent_learning.push_back(event.clone());
        if self.interest_model.recent_learning.len() > self.config.progress_window_size {
            self.interest_model.recent_learning.pop_front();
        }

        // Persist if enabled
        if let Some(conn) = &self.connection {
            let conn = conn.lock().unwrap();
            conn.execute(
                "INSERT INTO learning_events (
                    id, priority_id, content, drive, knowledge_type, success,
                    progress_gained, duration_minutes, satisfaction, timestamp
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    event.id.to_string(),
                    event.priority_id.to_string(),
                    event.content,
                    event.drive.to_string(),
                    event.knowledge_type.to_string(),
                    event.success,
                    event.progress_gained,
                    event.duration_minutes,
                    event.satisfaction,
                    event.timestamp.to_rfc3339(),
                ],
            ).context("Failed to store learning event")?;
        }

        self.update_stats();
        Ok(())
    }

    /// Update interest model based on learning event
    fn update_interest_model(&mut self, event: &LearningEvent) {
        let rate = self.interest_model.adaptation_rate;
        
        // Update drive preferences
        let current_pref = self.interest_model.drive_preferences
            .get(&event.drive).cloned().unwrap_or(0.5);
        let satisfaction_factor = if event.success { event.satisfaction } else { 0.0 };
        let new_pref = current_pref + rate * (satisfaction_factor - current_pref);
        self.interest_model.drive_preferences.insert(event.drive.clone(), new_pref);
        
        // Update type success rates
        let current_rate = self.interest_model.type_success_rates
            .get(&event.knowledge_type).cloned().unwrap_or(0.5);
        let success_value = if event.success { 1.0 } else { 0.0 };
        let new_rate = current_rate + rate * (success_value - current_rate);
        self.interest_model.type_success_rates.insert(event.knowledge_type.clone(), new_rate);
        
        self.interest_model.last_updated = Utc::now();
    }

    /// Update system statistics
    fn update_stats(&mut self) {
        self.stats.total_priorities = self.learning_priorities.len();
        self.stats.active_priorities = self.learning_priorities.values()
            .filter(|p| p.is_active).count();
        self.stats.completed_priorities = self.learning_priorities.values()
            .filter(|p| p.progress >= 0.9).count();
        
        if !self.learning_priorities.is_empty() {
            self.stats.average_curiosity_score = self.learning_priorities.values()
                .map(|p| p.curiosity_score).sum::<f64>() / self.learning_priorities.len() as f64;
            
            self.stats.average_progress = self.learning_priorities.values()
                .map(|p| p.progress).sum::<f64>() / self.learning_priorities.len() as f64;
            
            self.stats.overall_success_rate = self.learning_priorities.values()
                .map(|p| p.success_rate).sum::<f64>() / self.learning_priorities.len() as f64;
        }
        
        // Update drive distribution
        self.stats.drive_distribution.clear();
        for priority in self.learning_priorities.values() {
            *self.stats.drive_distribution.entry(priority.primary_drive.clone()).or_insert(0) += 1;
        }
    }

    /// Calculate content similarity between two strings
    fn content_similarity(&self, s1: &str, s2: &str) -> f64 {
        let words1: Vec<&str> = s1.split_whitespace().collect();
        let words2: Vec<&str> = s2.split_whitespace().collect();
        
        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }
        
        let common_words = words1.iter()
            .filter(|w| words2.contains(w))
            .count();
        
        2.0 * common_words as f64 / (words1.len() + words2.len()) as f64
    }

    /// Get configuration
    pub fn get_config(&self) -> &CuriosityConfig {
        &self.config
    }

    /// Get statistics
    pub fn get_stats(&self) -> &CuriosityStats {
        &self.stats
    }

    /// Get interest model
    pub fn get_interest_model(&self) -> &InterestModel {
        &self.interest_model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::novelty_detection::NoveltyDetectionConfig;
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;

    fn create_test_systems() -> Result<(Arc<Mutex<MetaMemorySystem>>, Arc<Mutex<NoveltyDetectionEngine>>)> {
        let temp_dir = tempdir()?;
        let meta_memory_path = temp_dir.path().join("test_meta.db");
        let meta_memory = Arc::new(Mutex::new(MetaMemorySystem::new(&meta_memory_path)?));
        
        let novelty_config = NoveltyDetectionConfig::default();
        let novelty_engine = Arc::new(Mutex::new(
            NoveltyDetectionEngine::new(novelty_config, meta_memory.clone())?
        ));
        
        Ok((meta_memory, novelty_engine))
    }

    #[test]
    fn test_curiosity_engine_creation() -> Result<()> {
        let (meta_memory, novelty_engine) = create_test_systems()?;
        let config = CuriosityConfig {
            enable_persistence: false,
            ..Default::default()
        };
        
        let engine = CuriosityLearningEngine::new(config, meta_memory, novelty_engine)?;
        assert_eq!(engine.learning_priorities.len(), 0);
        assert_eq!(engine.stats.total_priorities, 0);
        
        Ok(())
    }

    #[test]
    fn test_curiosity_assessment() -> Result<()> {
        let (meta_memory, novelty_engine) = create_test_systems()?;
        let config = CuriosityConfig {
            enable_persistence: false,
            learning_threshold: 0.2,
            ..Default::default()
        };
        
        let mut engine = CuriosityLearningEngine::new(config, meta_memory, novelty_engine)?;
        let curiosity_score = engine.assess_curiosity("This is a fascinating new concept", None)?;
        
        assert!(curiosity_score >= 0.0 && curiosity_score <= 1.0);
        
        Ok(())
    }

    #[test]
    fn test_learning_priority_creation() {
        let priority = LearningPriority::new(
            "Test content".to_string(),
            0.8,
            CuriosityDrive::NoveltySeeker,
        );
        
        assert_eq!(priority.content, "Test content");
        assert_eq!(priority.curiosity_score, 0.8);
        assert_eq!(priority.primary_drive, CuriosityDrive::NoveltySeeker);
        assert_eq!(priority.progress, 0.0);
        assert!(priority.is_active);
    }

    #[test]
    fn test_progress_update() {
        let mut priority = LearningPriority::new(
            "Test content".to_string(),
            0.8,
            CuriosityDrive::NoveltySeeker,
        );
        
        priority.update_progress(0.5, true);
        assert_eq!(priority.progress, 0.5);
        assert_eq!(priority.attempt_count, 1);
        assert_eq!(priority.success_rate, 1.0);
        
        priority.update_progress(0.7, false);
        assert_eq!(priority.progress, 0.7);
        assert_eq!(priority.attempt_count, 2);
        assert_eq!(priority.success_rate, 0.5);
    }

    #[test]
    fn test_content_similarity() -> Result<()> {
        let (meta_memory, novelty_engine) = create_test_systems()?;
        let config = CuriosityConfig {
            enable_persistence: false,
            ..Default::default()
        };
        
        let engine = CuriosityLearningEngine::new(config, meta_memory, novelty_engine)?;
        
        let similarity = engine.content_similarity("hello world", "hello universe");
        assert!(similarity > 0.0 && similarity < 1.0);
        
        let identical = engine.content_similarity("same text", "same text");
        assert_eq!(identical, 1.0);
        
        let different = engine.content_similarity("completely different", "unrelated content");
        assert_eq!(different, 0.0);
        
        Ok(())
    }

    #[test]
    fn test_learning_event_creation() {
        let event = LearningEvent::new(
            Uuid::new_v4(),
            "Test learning".to_string(),
            CuriosityDrive::UncertaintyResolver,
            KnowledgeType::ConceptNode,
        );
        
        assert_eq!(event.content, "Test learning");
        assert_eq!(event.drive, CuriosityDrive::UncertaintyResolver);
        assert_eq!(event.knowledge_type, KnowledgeType::ConceptNode);
        assert!(!event.success);
        assert_eq!(event.progress_gained, 0.0);
    }
} 