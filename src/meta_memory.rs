//! Meta-Memory System
//! 
//! This module implements the meta-memory system that tracks confidence levels and metadata
//! for all knowledge components across the Brain system.
//! 
//! ## Task 9.1: Meta-Memory Structure with Confidence Tracking
//! 
//! The meta-memory system provides:
//! - Unified confidence tracking for segments, concepts, rules, and memories
//! - Validation success rate tracking for confidence calculation
//! - Persistent storage of meta-memory data
//! - Query capabilities by confidence level, knowledge type, and age
//! - Analytics for overall knowledge quality and coverage

use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, Row};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

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
        Self {
            id: Uuid::new_v4(),
            component_id,
            knowledge_type,
            confidence_score: initial_confidence.clamp(0.0, 1.0),
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
        }
        
        self.last_modified_at = Utc::now();
        self.update_age();
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

    /// Set metadata value
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.last_modified_at = Utc::now();
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
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
    /// Limit number of results
    pub limit: Option<usize>,
    /// Sort by field
    pub sort_by: Option<String>,
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
            limit: None,
            sort_by: None,
            descending: false,
        }
    }
}

/// Statistics for meta-memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryStats {
    /// Total number of tracked components
    pub total_components: usize,
    /// Components by knowledge type
    pub components_by_type: HashMap<KnowledgeType, usize>,
    /// Average confidence score across all components
    pub average_confidence: f64,
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
}

impl Default for MetaMemoryStats {
    fn default() -> Self {
        Self {
            total_components: 0,
            components_by_type: HashMap::new(),
            average_confidence: 0.0,
            high_confidence_count: 0,
            low_confidence_count: 0,
            total_validations: 0,
            total_successes: 0,
            overall_success_rate: 0.0,
            average_age_hours: 0.0,
            active_components: 0,
            inactive_components: 0,
        }
    }
}

/// Configuration for meta-memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaMemoryConfig {
    /// Path to SQLite database for persistence
    pub database_path: String,
    /// Confidence threshold for marking components as high-confidence
    pub high_confidence_threshold: f64,
    /// Confidence threshold for marking components as low-confidence
    pub low_confidence_threshold: f64,
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
}

impl Default for MetaMemoryConfig {
    fn default() -> Self {
        Self {
            database_path: "meta_memory.db".to_string(),
            high_confidence_threshold: 0.8,
            low_confidence_threshold: 0.3,
            min_validation_count: 5,
            stale_age_threshold_hours: 24.0 * 7.0, // 1 week
            cleanup_interval_hours: 24.0,
            auto_confidence_updates: true,
            max_components: 100_000,
        }
    }
}

/// Main meta-memory system
pub struct MetaMemorySystem {
    /// Configuration
    config: MetaMemoryConfig,
    /// Database connection
    connection: Arc<Mutex<Connection>>,
    /// Statistics tracking
    stats: MetaMemoryStats,
}

impl MetaMemorySystem {
    /// Create a new meta-memory system
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let config = MetaMemoryConfig {
            database_path: db_path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };
        Self::with_config(config)
    }

    /// Create meta-memory system with custom configuration
    pub fn with_config(config: MetaMemoryConfig) -> Result<Self> {
        let conn = Connection::open(&config.database_path)
            .with_context(|| format!("Failed to open meta-memory database: {}", config.database_path))?;
        
        let mut system = Self {
            config,
            connection: Arc::new(Mutex::new(conn)),
            stats: MetaMemoryStats::default(),
        };
        
        system.initialize_database()?;
        system.update_stats()?;
        
        Ok(system)
    }

    /// Initialize database schema
    fn initialize_database(&self) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS meta_memory_items (
                id TEXT PRIMARY KEY,
                component_id TEXT NOT NULL,
                knowledge_type TEXT NOT NULL,
                confidence_score REAL NOT NULL,
                validation_count INTEGER NOT NULL,
                success_count INTEGER NOT NULL,
                usage_count INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                last_modified_at TEXT NOT NULL,
                last_accessed_at TEXT NOT NULL,
                source TEXT NOT NULL,
                age_hours REAL NOT NULL,
                is_active INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS meta_memory_metadata (
                item_id TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                PRIMARY KEY (item_id, key),
                FOREIGN KEY (item_id) REFERENCES meta_memory_items (id)
            )",
            [],
        )?;

        // Create indexes for efficient querying
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_knowledge_type ON meta_memory_items (knowledge_type)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_confidence ON meta_memory_items (confidence_score)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_component_id ON meta_memory_items (component_id)",
            [],
        )?;

        Ok(())
    }

    /// Store a meta-memory item
    pub fn store_item(&mut self, mut item: MetaMemoryItem) -> Result<Uuid> {
        item.update_age();
        
        let conn = self.connection.lock().unwrap();
        
        conn.execute(
            "INSERT OR REPLACE INTO meta_memory_items 
             (id, component_id, knowledge_type, confidence_score, validation_count, 
              success_count, usage_count, created_at, last_modified_at, last_accessed_at, 
              source, age_hours, is_active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                item.id.to_string(),
                item.component_id.to_string(),
                item.knowledge_type.to_string(),
                item.confidence_score,
                item.validation_count,
                item.success_count,
                item.usage_count,
                item.created_at.to_rfc3339(),
                item.last_modified_at.to_rfc3339(),
                item.last_accessed_at.to_rfc3339(),
                item.source,
                item.age_hours,
                if item.is_active { 1 } else { 0 }
            ],
        )?;

        // Store metadata
        for (key, value) in &item.metadata {
            conn.execute(
                "INSERT OR REPLACE INTO meta_memory_metadata (item_id, key, value) VALUES (?1, ?2, ?3)",
                params![item.id.to_string(), key, value],
            )?;
        }

        drop(conn);
        self.update_stats()?;
        
        Ok(item.id)
    }

    /// Get a meta-memory item by ID
    pub fn get_item(&self, id: Uuid) -> Result<Option<MetaMemoryItem>> {
        let conn = self.connection.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT id, component_id, knowledge_type, confidence_score, validation_count,
                    success_count, usage_count, created_at, last_modified_at, last_accessed_at,
                    source, age_hours, is_active
             FROM meta_memory_items WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map(params![id.to_string()], |row| {
            self.row_to_item(row)
        })?;

        if let Some(item_result) = rows.next() {
            let mut item = item_result?;
            
            // Load metadata
            let mut metadata_stmt = conn.prepare(
                "SELECT key, value FROM meta_memory_metadata WHERE item_id = ?1"
            )?;
            
            let metadata_rows = metadata_stmt.query_map(params![id.to_string()], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            for metadata_result in metadata_rows {
                let (key, value) = metadata_result?;
                item.metadata.insert(key, value);
            }

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Get meta-memory item by component ID
    pub fn get_item_by_component(&self, component_id: Uuid) -> Result<Option<MetaMemoryItem>> {
        let conn = self.connection.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT id, component_id, knowledge_type, confidence_score, validation_count,
                    success_count, usage_count, created_at, last_modified_at, last_accessed_at,
                    source, age_hours, is_active
             FROM meta_memory_items WHERE component_id = ?1"
        )?;

        let mut rows = stmt.query_map(params![component_id.to_string()], |row| {
            self.row_to_item(row)
        })?;

        if let Some(item_result) = rows.next() {
            let mut item = item_result?;
            
            // Load metadata
            let mut metadata_stmt = conn.prepare(
                "SELECT key, value FROM meta_memory_metadata WHERE item_id = ?1"
            )?;
            
            let metadata_rows = metadata_stmt.query_map(params![item.id.to_string()], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            for metadata_result in metadata_rows {
                let (key, value) = metadata_result?;
                item.metadata.insert(key, value);
            }

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Query meta-memory items
    pub fn query_items(&self, query: &MetaMemoryQuery) -> Result<Vec<MetaMemoryItem>> {
        let conn = self.connection.lock().unwrap();
        
        let mut sql = "SELECT id, component_id, knowledge_type, confidence_score, validation_count,
                              success_count, usage_count, created_at, last_modified_at, last_accessed_at,
                              source, age_hours, is_active
                       FROM meta_memory_items WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        // Apply filters
        if let Some(knowledge_type) = &query.knowledge_type {
            sql.push_str(" AND knowledge_type = ?");
            params.push(Box::new(knowledge_type.to_string()));
        }

        if let Some(min_confidence) = query.min_confidence {
            sql.push_str(" AND confidence_score >= ?");
            params.push(Box::new(min_confidence));
        }

        if let Some(max_confidence) = query.max_confidence {
            sql.push_str(" AND confidence_score <= ?");
            params.push(Box::new(max_confidence));
        }

        if let Some(min_usage) = query.min_usage_count {
            sql.push_str(" AND usage_count >= ?");
            params.push(Box::new(min_usage as i64));
        }

        if let Some(min_validation) = query.min_validation_count {
            sql.push_str(" AND validation_count >= ?");
            params.push(Box::new(min_validation as i64));
        }

        if let Some(min_age) = query.min_age_hours {
            sql.push_str(" AND age_hours >= ?");
            params.push(Box::new(min_age));
        }

        if let Some(max_age) = query.max_age_hours {
            sql.push_str(" AND age_hours <= ?");
            params.push(Box::new(max_age));
        }

        if let Some(active_only) = query.active_only {
            sql.push_str(" AND is_active = ?");
            params.push(Box::new(if active_only { 1 } else { 0 }));
        }

        if let Some(source_pattern) = &query.source_pattern {
            sql.push_str(" AND source LIKE ?");
            params.push(Box::new(format!("%{}%", source_pattern)));
        }

        // Apply sorting
        if let Some(sort_field) = &query.sort_by {
            sql.push_str(&format!(" ORDER BY {} {}", 
                sort_field, 
                if query.descending { "DESC" } else { "ASC" }
            ));
        }

        // Apply limit
        if let Some(limit) = query.limit {
            sql.push_str(" LIMIT ?");
            params.push(Box::new(limit as i64));
        }

        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        
        let rows = stmt.query_map(&param_refs[..], |row| {
            self.row_to_item(row)
        })?;

        let mut items = Vec::new();
        for row_result in rows {
            let mut item = row_result?;
            
            // Load metadata for each item
            let mut metadata_stmt = conn.prepare(
                "SELECT key, value FROM meta_memory_metadata WHERE item_id = ?1"
            )?;
            
            let metadata_rows = metadata_stmt.query_map(params![item.id.to_string()], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            for metadata_result in metadata_rows {
                let (key, value) = metadata_result?;
                item.metadata.insert(key, value);
            }
            
            items.push(item);
        }

        Ok(items)
    }

    /// Update confidence for a component
    pub fn update_confidence(&mut self, component_id: Uuid, success: bool) -> Result<bool> {
        if let Some(mut item) = self.get_item_by_component(component_id)? {
            item.update_confidence(success);
            self.store_item(item)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Mark component as accessed
    pub fn mark_accessed(&mut self, component_id: Uuid) -> Result<bool> {
        if let Some(mut item) = self.get_item_by_component(component_id)? {
            item.mark_accessed();
            self.store_item(item)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Remove a meta-memory item
    pub fn remove_item(&mut self, id: Uuid) -> Result<bool> {
        let conn = self.connection.lock().unwrap();
        
        // Remove metadata first
        conn.execute(
            "DELETE FROM meta_memory_metadata WHERE item_id = ?1",
            params![id.to_string()],
        )?;
        
        // Remove main item
        let changes = conn.execute(
            "DELETE FROM meta_memory_items WHERE id = ?1",
            params![id.to_string()],
        )?;

        drop(conn);
        
        if changes > 0 {
            self.update_stats()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get high-confidence components
    pub fn get_high_confidence_components(&self) -> Result<Vec<MetaMemoryItem>> {
        let query = MetaMemoryQuery {
            min_confidence: Some(self.config.high_confidence_threshold),
            active_only: Some(true),
            sort_by: Some("confidence_score".to_string()),
            descending: true,
            ..Default::default()
        };
        self.query_items(&query)
    }

    /// Get low-confidence components
    pub fn get_low_confidence_components(&self) -> Result<Vec<MetaMemoryItem>> {
        let query = MetaMemoryQuery {
            max_confidence: Some(self.config.low_confidence_threshold),
            active_only: Some(true),
            sort_by: Some("confidence_score".to_string()),
            descending: false,
            ..Default::default()
        };
        self.query_items(&query)
    }

    /// Get stale components (old and unused)
    pub fn get_stale_components(&self) -> Result<Vec<MetaMemoryItem>> {
        let query = MetaMemoryQuery {
            min_age_hours: Some(self.config.stale_age_threshold_hours),
            max_confidence: Some(self.config.low_confidence_threshold),
            active_only: Some(true),
            sort_by: Some("age_hours".to_string()),
            descending: true,
            ..Default::default()
        };
        self.query_items(&query)
    }

    /// Get statistics
    pub fn get_stats(&self) -> &MetaMemoryStats {
        &self.stats
    }

    /// Update statistics
    pub fn update_stats(&mut self) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        
        // Get total components
        let total_components: usize = conn.query_row(
            "SELECT COUNT(*) FROM meta_memory_items",
            [],
            |row| row.get(0)
        )?;

        // Get components by type
        let mut components_by_type = HashMap::new();
        let mut stmt = conn.prepare(
            "SELECT knowledge_type, COUNT(*) FROM meta_memory_items GROUP BY knowledge_type"
        )?;
        
        let rows = stmt.query_map([], |row| {
            let type_str: String = row.get(0)?;
            let count: usize = row.get(1)?;
            
            // Parse knowledge type
            let knowledge_type = match type_str.as_str() {
                "Segment" => KnowledgeType::Segment,
                "WorkingMemory" => KnowledgeType::WorkingMemory,
                "EpisodicMemory" => KnowledgeType::EpisodicMemory,
                "SemanticConcept" => KnowledgeType::SemanticConcept,
                "ConceptNode" => KnowledgeType::ConceptNode,
                "ConceptRelationship" => KnowledgeType::ConceptRelationship,
                "Rule" => KnowledgeType::Rule,
                "GeneralizedRule" => KnowledgeType::GeneralizedRule,
                "Pattern" => KnowledgeType::Pattern,
                _ => KnowledgeType::Segment, // Default fallback
            };
            
            Ok((knowledge_type, count))
        })?;

        for row_result in rows {
            let (knowledge_type, count) = row_result?;
            components_by_type.insert(knowledge_type, count);
        }

        // Get confidence statistics
        let (avg_confidence, high_confidence_count, low_confidence_count): (f64, usize, usize) = conn.query_row(
            "SELECT 
                AVG(confidence_score),
                SUM(CASE WHEN confidence_score >= ?1 THEN 1 ELSE 0 END),
                SUM(CASE WHEN confidence_score < ?2 THEN 1 ELSE 0 END)
             FROM meta_memory_items",
            params![self.config.high_confidence_threshold, self.config.low_confidence_threshold],
            |row| Ok((
                row.get(0).unwrap_or(0.0),
                row.get(1).unwrap_or(0),
                row.get(2).unwrap_or(0)
            ))
        )?;

        // Get validation statistics
        let (total_validations, total_successes): (u64, u64) = conn.query_row(
            "SELECT SUM(validation_count), SUM(success_count) FROM meta_memory_items",
            [],
            |row| Ok((
                row.get(0).unwrap_or(0),
                row.get(1).unwrap_or(0)
            ))
        )?;

        let overall_success_rate = if total_validations > 0 {
            total_successes as f64 / total_validations as f64
        } else {
            0.0
        };

        // Get age statistics
        let average_age_hours: f64 = conn.query_row(
            "SELECT AVG(age_hours) FROM meta_memory_items",
            [],
            |row| row.get(0)
        ).unwrap_or(0.0);

        // Get active/inactive counts
        let (active_components, inactive_components): (usize, usize) = conn.query_row(
            "SELECT 
                SUM(CASE WHEN is_active = 1 THEN 1 ELSE 0 END),
                SUM(CASE WHEN is_active = 0 THEN 1 ELSE 0 END)
             FROM meta_memory_items",
            [],
            |row| Ok((
                row.get(0).unwrap_or(0),
                row.get(1).unwrap_or(0)
            ))
        )?;

        self.stats = MetaMemoryStats {
            total_components,
            components_by_type,
            average_confidence: avg_confidence,
            high_confidence_count,
            low_confidence_count,
            total_validations,
            total_successes,
            overall_success_rate,
            average_age_hours,
            active_components,
            inactive_components,
        };

        Ok(())
    }

    /// Convert database row to MetaMemoryItem
    fn row_to_item(&self, row: &Row) -> rusqlite::Result<MetaMemoryItem> {
        let knowledge_type_str: String = row.get(2)?;
        let knowledge_type = match knowledge_type_str.as_str() {
            "Segment" => KnowledgeType::Segment,
            "WorkingMemory" => KnowledgeType::WorkingMemory,
            "EpisodicMemory" => KnowledgeType::EpisodicMemory,
            "SemanticConcept" => KnowledgeType::SemanticConcept,
            "ConceptNode" => KnowledgeType::ConceptNode,
            "ConceptRelationship" => KnowledgeType::ConceptRelationship,
            "Rule" => KnowledgeType::Rule,
            "GeneralizedRule" => KnowledgeType::GeneralizedRule,
            "Pattern" => KnowledgeType::Pattern,
            _ => KnowledgeType::Segment,
        };

        Ok(MetaMemoryItem {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            component_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
            knowledge_type,
            confidence_score: row.get(3)?,
            validation_count: row.get(4)?,
            success_count: row.get(5)?,
            usage_count: row.get(6)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .unwrap().with_timezone(&Utc),
            last_modified_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap().with_timezone(&Utc),
            last_accessed_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .unwrap().with_timezone(&Utc),
            source: row.get(10)?,
            age_hours: row.get(11)?,
            is_active: row.get::<_, i32>(12)? != 0,
            metadata: HashMap::new(), // Will be loaded separately
        })
    }

    /// Get configuration
    pub fn get_config(&self) -> &MetaMemoryConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: MetaMemoryConfig) {
        self.config = config;
    }

    /// Cleanup stale components
    pub fn cleanup_stale_components(&mut self) -> Result<usize> {
        let stale_items = self.get_stale_components()?;
        let stale_count = stale_items.len();
        
        for item in stale_items {
            self.remove_item(item.id)?;
        }
        
        Ok(stale_count)
    }

    /// Get confidence distribution
    pub fn get_confidence_distribution(&self) -> Result<HashMap<String, usize>> {
        let conn = self.connection.lock().unwrap();
        
        let mut distribution = HashMap::new();
        
        // Define confidence ranges
        let ranges = [
            ("very_low", 0.0, 0.2),
            ("low", 0.2, 0.4),
            ("medium", 0.4, 0.6),
            ("high", 0.6, 0.8),
            ("very_high", 0.8, 1.0),
        ];
        
        for (label, min_conf, max_conf) in ranges.iter() {
            let count: usize = conn.query_row(
                "SELECT COUNT(*) FROM meta_memory_items WHERE confidence_score >= ?1 AND confidence_score < ?2",
                params![min_conf, max_conf],
                |row| row.get(0)
            )?;
            
            distribution.insert(label.to_string(), count);
        }
        
        Ok(distribution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_meta_memory_item_creation() {
        let component_id = Uuid::new_v4();
        let item = MetaMemoryItem::new(
            component_id,
            KnowledgeType::ConceptNode,
            0.8,
            "test_source".to_string(),
        );

        assert_eq!(item.component_id, component_id);
        assert_eq!(item.knowledge_type, KnowledgeType::ConceptNode);
        assert_eq!(item.confidence_score, 0.8);
        assert_eq!(item.source, "test_source");
        assert_eq!(item.validation_count, 0);
        assert_eq!(item.success_count, 0);
        assert!(item.is_active);
    }

    #[test]
    fn test_confidence_update() {
        let mut item = MetaMemoryItem::new(
            Uuid::new_v4(),
            KnowledgeType::Rule,
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
        
        // Success rate should be 0.5
        assert!((item.success_rate() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_meta_memory_system_creation() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path();
        
        let system = MetaMemorySystem::new(db_path)?;
        assert_eq!(system.get_stats().total_components, 0);
        
        Ok(())
    }

    #[test]
    fn test_store_and_retrieve_item() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path();
        
        let mut system = MetaMemorySystem::new(db_path)?;
        
        let component_id = Uuid::new_v4();
        let mut item = MetaMemoryItem::new(
            component_id,
            KnowledgeType::SemanticConcept,
            0.75,
            "memory_test".to_string(),
        );
        
        item.set_metadata("test_key".to_string(), "test_value".to_string());
        
        let item_id = system.store_item(item.clone())?;
        
        let retrieved = system.get_item(item_id)?.unwrap();
        assert_eq!(retrieved.component_id, component_id);
        assert_eq!(retrieved.knowledge_type, KnowledgeType::SemanticConcept);
        assert_eq!(retrieved.confidence_score, 0.75);
        assert_eq!(retrieved.get_metadata("test_key"), Some(&"test_value".to_string()));
        
        // Test retrieval by component ID
        let retrieved_by_component = system.get_item_by_component(component_id)?.unwrap();
        assert_eq!(retrieved_by_component.id, item_id);
        
        Ok(())
    }

    #[test]
    fn test_query_items() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path();
        
        let mut system = MetaMemorySystem::new(db_path)?;
        
        // Store test items
        let item1 = MetaMemoryItem::new(
            Uuid::new_v4(),
            KnowledgeType::Rule,
            0.9,
            "high_confidence".to_string(),
        );
        
        let item2 = MetaMemoryItem::new(
            Uuid::new_v4(),
            KnowledgeType::ConceptNode,
            0.2,
            "low_confidence".to_string(),
        );
        
        system.store_item(item1)?;
        system.store_item(item2)?;
        
        // Query high-confidence items
        let query = MetaMemoryQuery {
            min_confidence: Some(0.8),
            ..Default::default()
        };
        
        let results = system.query_items(&query)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].knowledge_type, KnowledgeType::Rule);
        
        // Query by knowledge type
        let query = MetaMemoryQuery {
            knowledge_type: Some(KnowledgeType::ConceptNode),
            ..Default::default()
        };
        
        let results = system.query_items(&query)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].confidence_score, 0.2);
        
        Ok(())
    }

    #[test]
    fn test_confidence_update_integration() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path();
        
        let mut system = MetaMemorySystem::new(db_path)?;
        
        let component_id = Uuid::new_v4();
        let item = MetaMemoryItem::new(
            component_id,
            KnowledgeType::Segment,
            0.5,
            "test".to_string(),
        );
        
        system.store_item(item)?;
        
        // Update confidence
        let updated = system.update_confidence(component_id, true)?;
        assert!(updated);
        
        let retrieved = system.get_item_by_component(component_id)?.unwrap();
        assert_eq!(retrieved.validation_count, 1);
        assert_eq!(retrieved.success_count, 1);
        
        Ok(())
    }

    #[test]
    fn test_statistics_update() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path();
        
        let mut system = MetaMemorySystem::new(db_path)?;
        
        // Store test items with different confidence levels
        for i in 0..10 {
            let confidence = (i as f64) / 10.0;
            let item = MetaMemoryItem::new(
                Uuid::new_v4(),
                KnowledgeType::Rule,
                confidence,
                format!("test_{}", i),
            );
            system.store_item(item)?;
        }
        
        let stats = system.get_stats();
        assert_eq!(stats.total_components, 10);
        assert!(stats.average_confidence > 0.0);
        assert!(stats.low_confidence_count > 0);
        assert!(stats.high_confidence_count > 0);
        
        Ok(())
    }

    #[test]
    fn test_knowledge_type_display() {
        assert_eq!(KnowledgeType::Segment.to_string(), "Segment");
        assert_eq!(KnowledgeType::ConceptNode.to_string(), "ConceptNode");
        assert_eq!(KnowledgeType::Rule.to_string(), "Rule");
    }
} 