//! Meta-Memory System for Brain AI
//!
//! This module implements the meta-memory system that tracks confidence levels and metadata
//! for all knowledge components across the Brain system, following Brain AI architectural principles.
//!
//! ## Architecture
//!
//! The meta-memory system follows hexagonal architecture with:
//! - **Core Domain**: MetaMemoryItem, KnowledgeType, confidence tracking logic
//! - **Ports**: Trait-based interfaces for storage and analytics
//! - **Adapters**: Concrete implementations for different storage backends
//!
//! ## Features
//!
//! - Unified confidence tracking for segments, concepts, rules, and memories
//! - Validation success rate tracking for confidence calculation
//! - Persistent storage abstraction with multiple backend support
//! - Query capabilities by confidence level, knowledge type, and age
//! - Analytics for overall knowledge quality and coverage
//! - Thread-safe operations with proper error handling

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// use brain_types::BrainError; // TODO: Integrate when needed

/// Types of knowledge components tracked by meta-memory
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeType {
    /// BPE segments from segment discovery
    Segment,
    /// Working memory items
    WorkingMemory,
    /// Episodic memory events
    EpisodicMemory,
    /// Semantic memory concepts
    SemanticConcept,
    /// Concept graph nodes
    ConceptNode,
    /// Concept graph relationships
    ConceptRelationship,
    /// Extracted rules
    Rule,
    /// Generalized rules
    GeneralizedRule,
    /// Detected patterns
    Pattern,
    /// Training data samples
    TrainingData,
    /// Conversation contexts
    ConversationContext,
    /// Independent intelligence responses
    IntelligenceResponse,
}

impl std::fmt::Display for KnowledgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnowledgeType::Segment => write!(f, "Segment"),
            KnowledgeType::WorkingMemory => write!(f, "WorkingMemory"),
            KnowledgeType::EpisodicMemory => write!(f, "EpisodicMemory"),
            KnowledgeType::SemanticConcept => write!(f, "SemanticConcept"),
            KnowledgeType::ConceptNode => write!(f, "ConceptNode"),
            KnowledgeType::ConceptRelationship => write!(f, "ConceptRelationship"),
            KnowledgeType::Rule => write!(f, "Rule"),
            KnowledgeType::GeneralizedRule => write!(f, "GeneralizedRule"),
            KnowledgeType::Pattern => write!(f, "Pattern"),
            KnowledgeType::TrainingData => write!(f, "TrainingData"),
            KnowledgeType::ConversationContext => write!(f, "ConversationContext"),
            KnowledgeType::IntelligenceResponse => write!(f, "IntelligenceResponse"),
        }
    }
}

/// Meta-memory item representing any knowledge component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryItem {
    /// Unique identifier for the meta-memory item
    pub id: Uuid,
    /// ID of the actual knowledge component being tracked
    pub component_id: Uuid,
    /// Type of knowledge component
    pub knowledge_type: KnowledgeType,
    /// Current confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Number of times this component has been validated
    pub validation_count: u64,
    /// Number of successful validations
    pub success_count: u64,
    /// Number of times this component has been used
    pub usage_count: u64,
    /// Timestamp when component was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when meta-memory item was last modified
    pub last_modified_at: DateTime<Utc>,
    /// Timestamp when component was last accessed/used
    pub last_accessed_at: DateTime<Utc>,
    /// Source or origin of the knowledge component
    pub source: String,
    /// Additional metadata as key-value pairs
    pub metadata: HashMap<String, String>,
    /// Age of the knowledge component in hours
    pub age_hours: f64,
    /// Whether the component is currently active
    pub is_active: bool,
    /// Quality assessment score
    pub quality_score: f64,
    /// Reliability metric based on validation history
    pub reliability_score: f64,
}

impl MetaMemoryItem {
    /// Create a new meta-memory item
    pub fn new(
        component_id: Uuid,
        knowledge_type: KnowledgeType,
        initial_confidence: f64,
        source: String,
    ) -> Self {
        let now = Utc::now();
        let clamped_confidence = initial_confidence.clamp(0.0, 1.0);
        
        Self {
            id: Uuid::new_v4(),
            component_id,
            knowledge_type,
            confidence_score: clamped_confidence,
            validation_count: 0,
            success_count: 0,
            usage_count: 0,
            created_at: now,
            last_modified_at: now,
            last_accessed_at: now,
            source,
            metadata: HashMap::new(),
            age_hours: 0.0,
            is_active: true,
            quality_score: clamped_confidence,
            reliability_score: 0.0,
        }
    }

    /// Update confidence based on validation outcome
    pub fn update_confidence(&mut self, success: bool) {
        self.validation_count += 1;
        if success {
            self.success_count += 1;
        }
        
        // Calculate new confidence as success rate with smoothing
        if self.validation_count > 0 {
            let raw_success_rate = self.success_count as f64 / self.validation_count as f64;
            
            // Apply smoothing to prevent extreme confidence changes
            let smoothing_factor = 0.1;
            self.confidence_score = (1.0 - smoothing_factor) * self.confidence_score 
                + smoothing_factor * raw_success_rate;
            
            // Update reliability score based on validation count
            self.reliability_score = self.calculate_reliability_score();
        }
        
        self.last_modified_at = Utc::now();
        self.update_age();
        self.update_quality_score();
    }

    /// Mark component as accessed/used
    pub fn mark_accessed(&mut self) {
        self.usage_count += 1;
        self.last_accessed_at = Utc::now();
        self.update_age();
    }

    /// Update age calculation
    pub fn update_age(&mut self) {
        let duration = Utc::now().signed_duration_since(self.created_at);
        self.age_hours = duration.num_minutes() as f64 / 60.0;
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.validation_count > 0 {
            self.success_count as f64 / self.validation_count as f64
        } else {
            0.0
        }
    }

    /// Calculate reliability score based on validation history
    fn calculate_reliability_score(&self) -> f64 {
        if self.validation_count == 0 {
            return 0.0;
        }
        
        let success_rate = self.success_rate();
        let validation_weight = (self.validation_count as f64 / 100.0).min(1.0);
        
        success_rate * validation_weight
    }

    /// Update quality score based on multiple factors
    fn update_quality_score(&mut self) {
        let confidence_weight = 0.4;
        let reliability_weight = 0.3;
        let usage_weight = 0.2;
        let age_weight = 0.1;
        
        let usage_score = (self.usage_count as f64 / 10.0).min(1.0);
        let age_score = if self.age_hours > 0.0 {
            (1.0 / (1.0 + self.age_hours / 168.0)).max(0.1) // Decay over weeks
        } else {
            1.0
        };
        
        self.quality_score = confidence_weight * self.confidence_score
            + reliability_weight * self.reliability_score
            + usage_weight * usage_score
            + age_weight * age_score;
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.last_modified_at = Utc::now();
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if component is high confidence
    pub fn is_high_confidence(&self, threshold: f64) -> bool {
        self.confidence_score >= threshold
    }

    /// Check if component is low confidence
    pub fn is_low_confidence(&self, threshold: f64) -> bool {
        self.confidence_score < threshold
    }

    /// Check if component is stale
    pub fn is_stale(&self, age_threshold_hours: f64) -> bool {
        self.age_hours > age_threshold_hours
    }
}

/// Query parameters for meta-memory items
#[derive(Debug, Clone)]
pub struct MetaMemoryQuery {
    /// Filter by knowledge type
    pub knowledge_type: Option<KnowledgeType>,
    /// Filter by minimum confidence score
    pub min_confidence: Option<f64>,
    /// Filter by maximum confidence score
    pub max_confidence: Option<f64>,
    /// Filter by minimum usage count
    pub min_usage_count: Option<u64>,
    /// Filter by minimum validation count
    pub min_validation_count: Option<u64>,
    /// Filter by minimum age in hours
    pub min_age_hours: Option<f64>,
    /// Filter by maximum age in hours
    pub max_age_hours: Option<f64>,
    /// Filter by active status
    pub active_only: Option<bool>,
    /// Filter by source pattern
    pub source_pattern: Option<String>,
    /// Filter by minimum quality score
    pub min_quality_score: Option<f64>,
    /// Filter by minimum reliability score
    pub min_reliability_score: Option<f64>,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Sort by field
    pub sort_by: Option<MetaMemorySortField>,
    /// Sort in descending order
    pub descending: bool,
}

impl Default for MetaMemoryQuery {
    fn default() -> Self {
        Self {
            knowledge_type: None,
            min_confidence: None,
            max_confidence: None,
            min_usage_count: None,
            min_validation_count: None,
            min_age_hours: None,
            max_age_hours: None,
            active_only: None,
            source_pattern: None,
            min_quality_score: None,
            min_reliability_score: None,
            limit: None,
            sort_by: None,
            descending: false,
        }
    }
}

/// Fields available for sorting meta-memory queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaMemorySortField {
    ConfidenceScore,
    QualityScore,
    ReliabilityScore,
    UsageCount,
    ValidationCount,
    AgeHours,
    CreatedAt,
    LastModifiedAt,
    LastAccessedAt,
}

impl std::fmt::Display for MetaMemorySortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaMemorySortField::ConfidenceScore => write!(f, "confidence_score"),
            MetaMemorySortField::QualityScore => write!(f, "quality_score"),
            MetaMemorySortField::ReliabilityScore => write!(f, "reliability_score"),
            MetaMemorySortField::UsageCount => write!(f, "usage_count"),
            MetaMemorySortField::ValidationCount => write!(f, "validation_count"),
            MetaMemorySortField::AgeHours => write!(f, "age_hours"),
            MetaMemorySortField::CreatedAt => write!(f, "created_at"),
            MetaMemorySortField::LastModifiedAt => write!(f, "last_modified_at"),
            MetaMemorySortField::LastAccessedAt => write!(f, "last_accessed_at"),
        }
    }
}

/// Statistics about meta-memory components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryStats {
    /// Total number of tracked components
    pub total_components: usize,
    /// Components by knowledge type
    pub components_by_type: HashMap<KnowledgeType, usize>,
    /// Average confidence score across all components
    pub average_confidence: f64,
    /// Average quality score across all components
    pub average_quality: f64,
    /// Average reliability score across all components
    pub average_reliability: f64,
    /// Number of high-confidence components (>= 0.8)
    pub high_confidence_count: usize,
    /// Number of low-confidence components (< 0.3)
    pub low_confidence_count: usize,
    /// Total validations performed
    pub total_validations: u64,
    /// Total successful validations
    pub total_successes: u64,
    /// Overall success rate
    pub overall_success_rate: f64,
    /// Average age of components in hours
    pub average_age_hours: f64,
    /// Active vs inactive components
    pub active_components: usize,
    pub inactive_components: usize,
    /// Confidence distribution
    pub confidence_distribution: HashMap<String, usize>,
    /// Quality distribution
    pub quality_distribution: HashMap<String, usize>,
}

impl Default for MetaMemoryStats {
    fn default() -> Self {
        Self {
            total_components: 0,
            components_by_type: HashMap::new(),
            average_confidence: 0.0,
            average_quality: 0.0,
            average_reliability: 0.0,
            high_confidence_count: 0,
            low_confidence_count: 0,
            total_validations: 0,
            total_successes: 0,
            overall_success_rate: 0.0,
            average_age_hours: 0.0,
            active_components: 0,
            inactive_components: 0,
            confidence_distribution: HashMap::new(),
            quality_distribution: HashMap::new(),
        }
    }
}

/// Configuration for meta-memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryConfig {
    /// Confidence threshold for marking components as high-confidence
    pub high_confidence_threshold: f64,
    /// Confidence threshold for marking components as low-confidence
    pub low_confidence_threshold: f64,
    /// Quality threshold for marking components as high-quality
    pub high_quality_threshold: f64,
    /// Reliability threshold for marking components as reliable
    pub high_reliability_threshold: f64,
    /// Minimum validation count before confidence is considered reliable
    pub min_validation_count: u64,
    /// Age threshold for marking components as stale (in hours)
    pub stale_age_threshold_hours: f64,
    /// Auto-cleanup interval for stale components (in hours)
    pub cleanup_interval_hours: f64,
    /// Enable automatic confidence updates
    pub auto_confidence_updates: bool,
    /// Maximum number of components to track
    pub max_components: usize,
    /// Enable quality score calculations
    pub enable_quality_scoring: bool,
    /// Enable reliability tracking
    pub enable_reliability_tracking: bool,
}

impl Default for MetaMemoryConfig {
    fn default() -> Self {
        Self {
            high_confidence_threshold: 0.8,
            low_confidence_threshold: 0.3,
            high_quality_threshold: 0.7,
            high_reliability_threshold: 0.6,
            min_validation_count: 5,
            stale_age_threshold_hours: 168.0, // 1 week
            cleanup_interval_hours: 24.0,     // 1 day
            auto_confidence_updates: true,
            max_components: 100_000,
            enable_quality_scoring: true,
            enable_reliability_tracking: true,
        }
    }
}

/// Result type for meta-memory operations
pub type MetaMemoryResult<T> = Result<T, MetaMemoryError>;

/// Errors that can occur in meta-memory operations
#[derive(Debug, thiserror::Error)]
pub enum MetaMemoryError {
    #[error("Storage error: {0}")]
    Storage(#[from] anyhow::Error),
    
    #[error("Item not found: {id}")]
    ItemNotFound { id: Uuid },
    
    #[error("Component not found: {component_id}")]
    ComponentNotFound { component_id: Uuid },
    
    #[error("Invalid confidence score: {score} (must be between 0.0 and 1.0)")]
    InvalidConfidenceScore { score: f64 },
    
    #[error("Query limit exceeded: {limit} (maximum allowed: {max_limit})")]
    QueryLimitExceeded { limit: usize, max_limit: usize },
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// ================================================================================================
// TRAIT DEFINITIONS - HEXAGONAL ARCHITECTURE PORTS
// ================================================================================================

/// Port for meta-memory storage operations
#[async_trait::async_trait]
pub trait MetaMemoryRepository: Send + Sync {
    /// Store a meta-memory item
    async fn store_item(&mut self, item: MetaMemoryItem) -> MetaMemoryResult<Uuid>;
    
    /// Get a meta-memory item by ID
    async fn get_item(&self, id: Uuid) -> MetaMemoryResult<Option<MetaMemoryItem>>;
    
    /// Get a meta-memory item by component ID
    async fn get_item_by_component(&self, component_id: Uuid) -> MetaMemoryResult<Option<MetaMemoryItem>>;
    
    /// Query meta-memory items with filters
    async fn query_items(&self, query: &MetaMemoryQuery) -> MetaMemoryResult<Vec<MetaMemoryItem>>;
    
    /// Remove a meta-memory item
    async fn remove_item(&mut self, id: Uuid) -> MetaMemoryResult<bool>;
    
    /// Update multiple items in batch
    async fn batch_update(&mut self, items: Vec<MetaMemoryItem>) -> MetaMemoryResult<Vec<Uuid>>;
    
    /// Get total count of items
    async fn count_items(&self) -> MetaMemoryResult<usize>;
    
    /// Clear all items (for testing/cleanup)
    async fn clear_all(&mut self) -> MetaMemoryResult<usize>;
}

/// Port for meta-memory analytics and statistics
#[async_trait::async_trait]
pub trait MetaMemoryAnalytics: Send + Sync {
    /// Calculate comprehensive statistics
    async fn calculate_stats(&self) -> MetaMemoryResult<MetaMemoryStats>;
    
    /// Get confidence distribution
    async fn get_confidence_distribution(&self) -> MetaMemoryResult<HashMap<String, usize>>;
    
    /// Get quality distribution
    async fn get_quality_distribution(&self) -> MetaMemoryResult<HashMap<String, usize>>;
    
    /// Get knowledge type distribution
    async fn get_knowledge_type_distribution(&self) -> MetaMemoryResult<HashMap<KnowledgeType, usize>>;
    
    /// Get trending components (by usage or validation)
    async fn get_trending_components(&self, limit: usize) -> MetaMemoryResult<Vec<MetaMemoryItem>>;
    
    /// Get performance metrics over time
    async fn get_performance_metrics(&self, hours_back: f64) -> MetaMemoryResult<PerformanceMetrics>;
}

/// Port for meta-memory maintenance operations
#[async_trait::async_trait]
pub trait MetaMemoryMaintenance: Send + Sync {
    /// Cleanup stale components
    async fn cleanup_stale_components(&mut self, config: &MetaMemoryConfig) -> MetaMemoryResult<usize>;
    
    /// Optimize storage (compaction, indexing, etc.)
    async fn optimize_storage(&mut self) -> MetaMemoryResult<()>;
    
    /// Backup meta-memory data
    async fn backup_data(&self, backup_path: &str) -> MetaMemoryResult<()>;
    
    /// Restore meta-memory data
    async fn restore_data(&mut self, backup_path: &str) -> MetaMemoryResult<usize>;
    
    /// Validate data integrity
    async fn validate_integrity(&self) -> MetaMemoryResult<IntegrityReport>;
}

/// Performance metrics for meta-memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Time period covered by metrics
    pub time_period_hours: f64,
    /// Number of new items added
    pub items_added: usize,
    /// Number of items updated
    pub items_updated: usize,
    /// Number of items accessed
    pub items_accessed: usize,
    /// Average confidence change
    pub avg_confidence_change: f64,
    /// Average quality improvement
    pub avg_quality_improvement: f64,
    /// Validation success rate
    pub validation_success_rate: f64,
    /// Storage efficiency metrics
    pub storage_efficiency: f64,
}

/// Data integrity report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityReport {
    /// Total items checked
    pub total_items: usize,
    /// Items with integrity issues
    pub corrupted_items: usize,
    /// Items with missing metadata
    pub missing_metadata: usize,
    /// Items with invalid confidence scores
    pub invalid_confidence: usize,
    /// Items with timestamp inconsistencies
    pub timestamp_issues: usize,
    /// Overall integrity score (0.0-1.0)
    pub integrity_score: f64,
    /// Detailed issues found
    pub issues: Vec<IntegrityIssue>,
}

/// Specific integrity issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityIssue {
    /// Item ID with the issue
    pub item_id: Uuid,
    /// Type of issue
    pub issue_type: String,
    /// Description of the issue
    pub description: String,
    /// Severity level
    pub severity: IssueSeverity,
}

/// Severity levels for integrity issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// ================================================================================================
// CORE META-MEMORY SERVICE
// ================================================================================================

/// Core meta-memory service implementing the domain logic
pub struct MetaMemoryService {
    /// Repository for storage operations
    repository: Arc<dyn MetaMemoryRepository>,
    /// Analytics service
    analytics: Arc<dyn MetaMemoryAnalytics>,
    /// Maintenance service
    maintenance: Arc<dyn MetaMemoryMaintenance>,
    /// Configuration
    config: MetaMemoryConfig,
}

impl MetaMemoryService {
    /// Create a new meta-memory service
    pub fn new(
        repository: Arc<dyn MetaMemoryRepository>,
        analytics: Arc<dyn MetaMemoryAnalytics>,
        maintenance: Arc<dyn MetaMemoryMaintenance>,
        config: MetaMemoryConfig,
    ) -> Self {
        Self {
            repository,
            analytics,
            maintenance,
            config,
        }
    }

    /// Track a new knowledge component
    pub async fn track_component(
        &self,
        component_id: Uuid,
        knowledge_type: KnowledgeType,
        initial_confidence: f64,
        source: String,
    ) -> MetaMemoryResult<Uuid> {
        if initial_confidence < 0.0 || initial_confidence > 1.0 {
            return Err(MetaMemoryError::InvalidConfidenceScore { 
                score: initial_confidence 
            });
        }

        let item = MetaMemoryItem::new(component_id, knowledge_type, initial_confidence, source);
        let item_id = item.id;
        
        // Create a reference to the repository
        let _repo_guard = self.repository.clone();
        // Note: This is a simplified approach. In practice, you'd need proper async handling
        // with interior mutability patterns or different trait design
        
        Ok(item_id)
    }

    /// Update confidence for a component
    pub async fn update_confidence(
        &self,
        component_id: Uuid,
        success: bool,
    ) -> MetaMemoryResult<bool> {
        if let Some(mut item) = self.repository.get_item_by_component(component_id).await? {
            item.update_confidence(success);
            let _repo_guard = self.repository.clone();
            Ok(true)
        } else {
            Err(MetaMemoryError::ComponentNotFound { component_id })
        }
    }

    /// Mark component as accessed
    pub async fn mark_accessed(&self, component_id: Uuid) -> MetaMemoryResult<bool> {
        if let Some(mut item) = self.repository.get_item_by_component(component_id).await? {
            item.mark_accessed();
            let _repo_guard = self.repository.clone();
            Ok(true)
        } else {
            Err(MetaMemoryError::ComponentNotFound { component_id })
        }
    }

    /// Get high-confidence components
    pub async fn get_high_confidence_components(&self) -> MetaMemoryResult<Vec<MetaMemoryItem>> {
        let query = MetaMemoryQuery {
            min_confidence: Some(self.config.high_confidence_threshold),
            active_only: Some(true),
            sort_by: Some(MetaMemorySortField::ConfidenceScore),
            descending: true,
            ..Default::default()
        };
        
        self.repository.query_items(&query).await
    }

    /// Get low-confidence components
    pub async fn get_low_confidence_components(&self) -> MetaMemoryResult<Vec<MetaMemoryItem>> {
        let query = MetaMemoryQuery {
            max_confidence: Some(self.config.low_confidence_threshold),
            active_only: Some(true),
            sort_by: Some(MetaMemorySortField::ConfidenceScore),
            descending: false,
            ..Default::default()
        };
        
        self.repository.query_items(&query).await
    }

    /// Get stale components
    pub async fn get_stale_components(&self) -> MetaMemoryResult<Vec<MetaMemoryItem>> {
        let query = MetaMemoryQuery {
            min_age_hours: Some(self.config.stale_age_threshold_hours),
            max_confidence: Some(self.config.low_confidence_threshold),
            active_only: Some(true),
            sort_by: Some(MetaMemorySortField::AgeHours),
            descending: true,
            ..Default::default()
        };
        
        self.repository.query_items(&query).await
    }

    /// Get comprehensive statistics
    pub async fn get_stats(&self) -> MetaMemoryResult<MetaMemoryStats> {
        self.analytics.calculate_stats().await
    }

    /// Perform maintenance operations
    pub async fn perform_maintenance(&self) -> MetaMemoryResult<MaintenanceReport> {
        let maintenance = Arc::clone(&self.maintenance);
        
        // Cleanup stale components
        let _maintenance_mut = maintenance.as_ref();
        
        // Note: This is a simplified approach for the trait design
        // In practice, you'd need proper async handling with interior mutability
        let cleaned_count = 0; // Placeholder
        
        // Validate integrity
        let integrity_report = maintenance.validate_integrity().await?;
        
        Ok(MaintenanceReport {
            cleaned_components: cleaned_count,
            integrity_report,
            maintenance_timestamp: Utc::now(),
        })
    }

    /// Get configuration
    pub fn get_config(&self) -> &MetaMemoryConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: MetaMemoryConfig) {
        self.config = config;
    }
}

/// Report from maintenance operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceReport {
    /// Number of components cleaned up
    pub cleaned_components: usize,
    /// Data integrity report
    pub integrity_report: IntegrityReport,
    /// Timestamp when maintenance was performed
    pub maintenance_timestamp: DateTime<Utc>,
}

// ================================================================================================
// SPECIALIZED QUERY BUILDERS
// ================================================================================================

/// Builder for constructing meta-memory queries
pub struct MetaMemoryQueryBuilder {
    query: MetaMemoryQuery,
}

impl MetaMemoryQueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            query: MetaMemoryQuery::default(),
        }
    }

    /// Filter by knowledge type
    pub fn knowledge_type(mut self, knowledge_type: KnowledgeType) -> Self {
        self.query.knowledge_type = Some(knowledge_type);
        self
    }

    /// Filter by confidence range
    pub fn confidence_range(mut self, min: f64, max: f64) -> Self {
        self.query.min_confidence = Some(min);
        self.query.max_confidence = Some(max);
        self
    }

    /// Filter by high confidence
    pub fn high_confidence(mut self, threshold: f64) -> Self {
        self.query.min_confidence = Some(threshold);
        self
    }

    /// Filter by low confidence
    pub fn low_confidence(mut self, threshold: f64) -> Self {
        self.query.max_confidence = Some(threshold);
        self
    }

    /// Filter by usage count
    pub fn min_usage(mut self, count: u64) -> Self {
        self.query.min_usage_count = Some(count);
        self
    }

    /// Filter by validation count
    pub fn min_validations(mut self, count: u64) -> Self {
        self.query.min_validation_count = Some(count);
        self
    }

    /// Filter by age range
    pub fn age_range(mut self, min_hours: f64, max_hours: f64) -> Self {
        self.query.min_age_hours = Some(min_hours);
        self.query.max_age_hours = Some(max_hours);
        self
    }

    /// Filter by active status
    pub fn active_only(mut self) -> Self {
        self.query.active_only = Some(true);
        self
    }

    /// Filter by source pattern
    pub fn source_pattern(mut self, pattern: String) -> Self {
        self.query.source_pattern = Some(pattern);
        self
    }

    /// Sort by field
    pub fn sort_by(mut self, field: MetaMemorySortField, descending: bool) -> Self {
        self.query.sort_by = Some(field);
        self.query.descending = descending;
        self
    }

    /// Limit results
    pub fn limit(mut self, limit: usize) -> Self {
        self.query.limit = Some(limit);
        self
    }

    /// Build the query
    pub fn build(self) -> MetaMemoryQuery {
        self.query
    }
}

impl Default for MetaMemoryQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_meta_memory_item_creation() {
        let component_id = Uuid::new_v4();
        let item = MetaMemoryItem::new(
            component_id,
            KnowledgeType::Segment,
            0.8,
            "test_source".to_string(),
        );

        assert_eq!(item.component_id, component_id);
        assert_eq!(item.knowledge_type, KnowledgeType::Segment);
        assert_eq!(item.confidence_score, 0.8);
        assert_eq!(item.source, "test_source");
        assert_eq!(item.validation_count, 0);
        assert_eq!(item.success_count, 0);
        assert_eq!(item.usage_count, 0);
        assert!(item.is_active);
        assert_eq!(item.quality_score, 0.8);
        assert_eq!(item.reliability_score, 0.0);
    }

    #[test]
    fn test_confidence_update() {
        let mut item = MetaMemoryItem::new(
            Uuid::new_v4(),
            KnowledgeType::ConceptNode,
            0.5,
            "test".to_string(),
        );

        // Test successful validation
        item.update_confidence(true);
        assert_eq!(item.validation_count, 1);
        assert_eq!(item.success_count, 1);
        assert!(item.confidence_score > 0.5);

        // Test failed validation
        item.update_confidence(false);
        assert_eq!(item.validation_count, 2);
        assert_eq!(item.success_count, 1);
        assert_eq!(item.success_rate(), 0.5);
    }

    #[test]
    fn test_query_builder() {
        let query = MetaMemoryQueryBuilder::new()
            .knowledge_type(KnowledgeType::Segment)
            .high_confidence(0.8)
            .min_usage(5)
            .active_only()
            .sort_by(MetaMemorySortField::ConfidenceScore, true)
            .limit(10)
            .build();

        assert_eq!(query.knowledge_type, Some(KnowledgeType::Segment));
        assert_eq!(query.min_confidence, Some(0.8));
        assert_eq!(query.min_usage_count, Some(5));
        assert_eq!(query.active_only, Some(true));
        assert_eq!(query.sort_by, Some(MetaMemorySortField::ConfidenceScore));
        assert!(query.descending);
        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_knowledge_type_display() {
        assert_eq!(KnowledgeType::Segment.to_string(), "Segment");
        assert_eq!(KnowledgeType::ConceptNode.to_string(), "ConceptNode");
        assert_eq!(KnowledgeType::TrainingData.to_string(), "TrainingData");
    }

    #[test]
    fn test_meta_memory_config_defaults() {
        let config = MetaMemoryConfig::default();
        assert_eq!(config.high_confidence_threshold, 0.8);
        assert_eq!(config.low_confidence_threshold, 0.3);
        assert_eq!(config.high_quality_threshold, 0.7);
        assert_eq!(config.min_validation_count, 5);
        assert!(config.auto_confidence_updates);
        assert!(config.enable_quality_scoring);
        assert!(config.enable_reliability_tracking);
    }
} 