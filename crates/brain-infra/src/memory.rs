//! Memory Infrastructure Implementations
//! 
//! Sophisticated implementations of memory repository traits with
//! priority queues, SQLite persistence, vector similarity, and
//! comprehensive consolidation processes.

use brain_core::*;
use brain_types::*;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, Row};
use std::collections::{HashMap, BinaryHeap};
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Advanced working memory implementation with priority queues and decay
pub struct WorkingMemoryRepository {
    items: HashMap<Uuid, WorkingMemoryItem>,
    priority_queue: BinaryHeap<(u64, Uuid)>, // (score * 1000 as int, id)
    max_capacity: usize,
    stats: MemoryStats,
}

impl WorkingMemoryRepository {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            items: HashMap::new(),
            priority_queue: BinaryHeap::new(),
            max_capacity,
            stats: MemoryStats {
                total_items: 0,
                size_bytes: 0,
                last_access: Utc::now(),
                access_count: 0,
                consolidation_count: 0,
            },
        }
    }

    /// Add item to working memory, potentially evicting low-priority items
    pub async fn add_item(&mut self, content: String, priority: Priority) -> Result<Uuid> {
        let item = WorkingMemoryItem::new(content, priority);
        let id = item.id;
        
        // Check capacity and evict if necessary
        if self.items.len() >= self.max_capacity {
            self.evict_lowest_priority().await?;
        }
        
        let score = (item.importance_score() * 1000.0) as u64;
        self.priority_queue.push((score, id));
        self.items.insert(id, item);
        
        self.update_stats();
        Ok(id)
    }

    /// Access an item, updating its access statistics
    pub async fn access_item(&mut self, id: Uuid) -> Option<&WorkingMemoryItem> {
        if let Some(item) = self.items.get_mut(&id) {
            item.last_accessed = Utc::now();
            item.access_count += 1;
            item.update_decay();
            
            self.stats.access_count += 1;
            self.stats.last_access = Utc::now();
            
            Some(&*item)
        } else {
            None
        }
    }

    /// Get items by priority level
    pub fn get_by_priority(&self, priority: Priority) -> Vec<&WorkingMemoryItem> {
        self.items.values()
            .filter(|item| item.priority == priority)
            .collect()
    }

    /// Get consolidation candidates based on age threshold
    pub fn get_consolidation_candidates(&self, age_threshold_hours: i64) -> Vec<&WorkingMemoryItem> {
        let threshold_time = Utc::now() - chrono::Duration::hours(age_threshold_hours);
        self.items.values()
            .filter(|item| item.created_at <= threshold_time)
            .collect()
    }

    /// Remove consolidated items from working memory
    pub async fn remove_consolidated(&mut self, ids: &[Uuid]) -> Result<()> {
        for &id in ids {
            self.items.remove(&id);
        }
        self.rebuild_priority_queue();
        self.update_stats();
        Ok(())
    }

    /// Apply decay to all items and update priority queue
    pub async fn apply_decay(&mut self) -> Result<()> {
        for item in self.items.values_mut() {
            item.update_decay();
        }
        self.rebuild_priority_queue();
        Ok(())
    }

    /// Prune items with low importance scores
    pub async fn prune_low_importance(&mut self, threshold: f64) -> Result<Vec<Uuid>> {
        let mut pruned_ids = Vec::new();
        
        self.items.retain(|&id, item| {
            if item.importance_score() < threshold {
                pruned_ids.push(id);
                false
            } else {
                true
            }
        });
        
        self.rebuild_priority_queue();
        self.update_stats();
        Ok(pruned_ids)
    }

    async fn evict_lowest_priority(&mut self) -> Result<()> {
        if let Some((_, id)) = self.priority_queue.pop() {
            self.items.remove(&id);
            self.update_stats();
        }
        Ok(())
    }

    fn rebuild_priority_queue(&mut self) {
        self.priority_queue.clear();
        for (id, item) in &self.items {
            let score = (item.importance_score() * 1000.0) as u64;
            self.priority_queue.push((score, *id));
        }
    }

    fn update_stats(&mut self) {
        self.stats.total_items = self.items.len();
        self.stats.size_bytes = self.items.len() * std::mem::size_of::<WorkingMemoryItem>();
        self.stats.last_access = Utc::now();
    }
}

#[async_trait::async_trait]
impl brain_core::WorkingMemoryRepository for WorkingMemoryRepository {
    async fn store_item(&mut self, item: WorkingMemoryItem) -> Result<Uuid> {
        let id = item.id;
        let priority = item.priority;
        let content = item.content.clone();
        
        // Use our sophisticated add_item method
        self.add_item(content, priority).await
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<WorkingMemoryItem>> {
        Ok(self.items.get(&id).cloned())
    }

    async fn update_item(&mut self, item: &WorkingMemoryItem) -> Result<()> {
        if self.items.contains_key(&item.id) {
            self.items.insert(item.id, item.clone());
            self.rebuild_priority_queue();
            self.update_stats();
        }
        Ok(())
    }

    async fn remove_item(&mut self, id: Uuid) -> Result<()> {
        self.items.remove(&id);
        self.rebuild_priority_queue();
        self.update_stats();
        Ok(())
    }

    async fn query_items(&self, query: &WorkingMemoryQuery) -> Result<Vec<WorkingMemoryItem>> {
        let mut results: Vec<WorkingMemoryItem> = self.items.values().cloned().collect();

        // Apply filters
        if let Some(ref pattern) = query.content_pattern {
            results.retain(|item| item.content.contains(pattern));
        }

        if let Some(priority) = query.priority {
            results.retain(|item| item.priority == priority);
        }

        if let Some(min_importance) = query.min_importance {
            results.retain(|item| item.importance_score() >= min_importance);
        }

        if let Some(created_after) = query.created_after {
            results.retain(|item| item.created_at >= created_after);
        }

        // Sort by importance score (descending)
        results.sort_by(|a, b| b.importance_score().partial_cmp(&a.importance_score()).unwrap_or(std::cmp::Ordering::Equal));

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn get_consolidation_candidates(&self, age_threshold_hours: i64) -> Result<Vec<WorkingMemoryItem>> {
        let threshold_time = Utc::now() - chrono::Duration::hours(age_threshold_hours);
        let candidates: Vec<WorkingMemoryItem> = self.items
            .values()
            .filter(|item| item.created_at <= threshold_time)
            .cloned()
            .collect();
        Ok(candidates)
    }

    async fn prune_low_importance(&mut self, threshold: f64) -> Result<Vec<Uuid>> {
        self.prune_low_importance(threshold).await
    }

    async fn stats(&self) -> Result<MemoryStats> {
        Ok(self.stats.clone())
    }
}

/// SQLite-based episodic memory implementation with persistence
pub struct EpisodicMemoryRepository {
    connection: Arc<Mutex<Connection>>,
    stats: MemoryStats,
}

impl EpisodicMemoryRepository {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path.as_ref())
            .map_err(|e| BrainError::DatabaseError(format!("Failed to open database: {}", e)))?;
        
        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS episodic_events (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                importance REAL NOT NULL,
                tags TEXT NOT NULL,
                source TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to create events table: {}", e)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS event_context (
                event_id TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                PRIMARY KEY (event_id, key),
                FOREIGN KEY (event_id) REFERENCES episodic_events (id) ON DELETE CASCADE
            )",
            [],
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to create context table: {}", e)))?;

        // Create indexes
        conn.execute("CREATE INDEX IF NOT EXISTS idx_events_timestamp ON episodic_events(timestamp)", [])
            .map_err(|e| BrainError::DatabaseError(format!("Failed to create timestamp index: {}", e)))?;
        
        conn.execute("CREATE INDEX IF NOT EXISTS idx_events_importance ON episodic_events(importance)", [])
            .map_err(|e| BrainError::DatabaseError(format!("Failed to create importance index: {}", e)))?;

        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
            stats: MemoryStats {
                total_items: 0,
                size_bytes: 0,
                last_access: Utc::now(),
                access_count: 0,
                consolidation_count: 0,
            },
        })
    }

    /// Store event with context information
    pub async fn store_event(&mut self, event: EpisodicEvent) -> Result<Uuid> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;
        
        let id_str = event.id.to_string();
        let timestamp_str = event.timestamp.to_rfc3339();
        let tags_json = serde_json::to_string(&event.tags)?;

        conn.execute(
            "INSERT INTO episodic_events (id, content, timestamp, importance, tags, source) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id_str, event.content, timestamp_str, event.importance, tags_json, event.source],
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to insert event: {}", e)))?;

        // Store context
        for (key, value) in &event.context {
            conn.execute(
                "INSERT INTO event_context (event_id, key, value) VALUES (?1, ?2, ?3)",
                params![id_str, key, value],
            ).map_err(|e| BrainError::DatabaseError(format!("Failed to insert context: {}", e)))?;
        }

        Ok(event.id)
    }

    /// Get events by time range
    pub async fn get_events_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<EpisodicEvent>> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;
        
        let start_str = start.to_rfc3339();
        let end_str = end.to_rfc3339();

        let mut stmt = conn.prepare(
            "SELECT id, content, timestamp, importance, tags, source 
             FROM episodic_events 
             WHERE timestamp BETWEEN ?1 AND ?2 
             ORDER BY timestamp DESC"
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        let event_iter = stmt.query_map(params![start_str, end_str], |row| {
            self.row_to_event(row)
        }).map_err(|e| BrainError::DatabaseError(format!("Failed to query events: {}", e)))?;

        let mut events = Vec::new();
        for event_result in event_iter {
            let mut event = event_result
                .map_err(|e| BrainError::DatabaseError(format!("Failed to parse event: {}", e)))?;
            self.load_event_context(&conn, &mut event)?;
            events.push(event);
        }

        Ok(events)
    }

    /// Apply forgetting mechanism with decay
    pub async fn apply_forgetting(&mut self, decay_rate: f64, min_importance: f64) -> Result<usize> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;

        // First, decay importance scores
        conn.execute(
            "UPDATE episodic_events SET importance = importance * ?1",
            params![1.0 - decay_rate],
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to decay importance: {}", e)))?;

        // Then remove events below threshold
        let deleted_count = conn.execute(
            "DELETE FROM episodic_events WHERE importance < ?1",
            params![min_importance],
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to delete low importance events: {}", e)))?;

        Ok(deleted_count)
    }

    fn row_to_event(&self, row: &Row) -> rusqlite::Result<EpisodicEvent> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| rusqlite::Error::InvalidColumnType(0, "id".to_string(), rusqlite::types::Type::Text))?;
        
        let timestamp_str: String = row.get(2)?;
        let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|_| rusqlite::Error::InvalidColumnType(2, "timestamp".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);

        let tags_json: String = row.get(4)?;
        let tags: Vec<String> = serde_json::from_str(&tags_json)
            .map_err(|_| rusqlite::Error::InvalidColumnType(4, "tags".to_string(), rusqlite::types::Type::Text))?;

        Ok(EpisodicEvent {
            id,
            content: row.get(1)?,
            timestamp,
            context: HashMap::new(), // Will be loaded separately
            importance: row.get(3)?,
            tags,
            source: row.get(5)?,
        })
    }

    fn load_event_context(&self, conn: &Connection, event: &mut EpisodicEvent) -> Result<()> {
        let id_str = event.id.to_string();
        let mut stmt = conn.prepare("SELECT key, value FROM event_context WHERE event_id = ?1")
            .map_err(|e| BrainError::DatabaseError(format!("Failed to prepare context query: {}", e)))?;

        let context_iter = stmt.query_map(params![id_str], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).map_err(|e| BrainError::DatabaseError(format!("Failed to query context: {}", e)))?;

        for context_result in context_iter {
            let (key, value) = context_result
                .map_err(|e| BrainError::DatabaseError(format!("Failed to parse context: {}", e)))?;
            event.context.insert(key, value);
        }

        Ok(())
    }

    fn update_stats(&mut self) {
        // Would need to query database for accurate stats
        self.stats.last_access = Utc::now();
        self.stats.access_count += 1;
    }
}

#[async_trait::async_trait]
impl brain_core::EpisodicMemoryRepository for EpisodicMemoryRepository {
    async fn store_event(&mut self, event: EpisodicEvent) -> Result<Uuid> {
        self.store_event(event).await
    }

    async fn get_event(&self, id: Uuid) -> Result<Option<EpisodicEvent>> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;
        
        let id_str = id.to_string();
        let mut stmt = conn.prepare(
            "SELECT id, content, timestamp, importance, tags, source 
             FROM episodic_events WHERE id = ?1"
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        let event_result = stmt.query_row(params![id_str], |row| {
            self.row_to_event(row)
        });

        match event_result {
            Ok(mut event) => {
                self.load_event_context(&conn, &mut event)?;
                Ok(Some(event))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(BrainError::DatabaseError(format!("Failed to get event: {}", e))),
        }
    }

    async fn update_event(&mut self, event: &EpisodicEvent) -> Result<()> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;
        
        let id_str = event.id.to_string();
        let timestamp_str = event.timestamp.to_rfc3339();
        let tags_json = serde_json::to_string(&event.tags)?;

        conn.execute(
            "UPDATE episodic_events 
             SET content = ?1, timestamp = ?2, importance = ?3, tags = ?4, source = ?5 
             WHERE id = ?6",
            params![event.content, timestamp_str, event.importance, tags_json, event.source, id_str],
        ).map_err(|e| BrainError::DatabaseError(format!("Failed to update event: {}", e)))?;

        // Update context - delete and re-insert
        conn.execute("DELETE FROM event_context WHERE event_id = ?1", params![id_str])
            .map_err(|e| BrainError::DatabaseError(format!("Failed to delete old context: {}", e)))?;

        for (key, value) in &event.context {
            conn.execute(
                "INSERT INTO event_context (event_id, key, value) VALUES (?1, ?2, ?3)",
                params![id_str, key, value],
            ).map_err(|e| BrainError::DatabaseError(format!("Failed to insert context: {}", e)))?;
        }

        Ok(())
    }

    async fn remove_event(&mut self, id: Uuid) -> Result<()> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;
        
        let id_str = id.to_string();
        conn.execute("DELETE FROM episodic_events WHERE id = ?1", params![id_str])
            .map_err(|e| BrainError::DatabaseError(format!("Failed to delete event: {}", e)))?;

        Ok(())
    }

    async fn query_events(&self, query: &EpisodicQuery) -> Result<Vec<EpisodicEvent>> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;

        let mut sql = "SELECT id, content, timestamp, importance, tags, source FROM episodic_events WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref pattern) = query.content_pattern {
            sql.push_str(" AND content LIKE ?");
            params.push(Box::new(format!("%{}%", pattern)));
        }

        if let Some(min_importance) = query.min_importance {
            sql.push_str(" AND importance >= ?");
            params.push(Box::new(min_importance));
        }

        if let Some((start, end)) = query.time_range {
            sql.push_str(" AND timestamp BETWEEN ? AND ?");
            params.push(Box::new(start.to_rfc3339()));
            params.push(Box::new(end.to_rfc3339()));
        }

        sql.push_str(" ORDER BY timestamp DESC");

        if let Some(limit) = query.limit {
            sql.push_str(" LIMIT ?");
            params.push(Box::new(limit as i64));
        }

        let mut stmt = conn.prepare(&sql)
            .map_err(|e| BrainError::DatabaseError(format!("Failed to prepare query: {}", e)))?;

        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let event_iter = stmt.query_map(&param_refs[..], |row| {
            self.row_to_event(row)
        }).map_err(|e| BrainError::DatabaseError(format!("Failed to execute query: {}", e)))?;

        let mut events = Vec::new();
        for event_result in event_iter {
            let mut event = event_result
                .map_err(|e| BrainError::DatabaseError(format!("Failed to parse event: {}", e)))?;
            self.load_event_context(&conn, &mut event)?;
            events.push(event);
        }

        Ok(events)
    }

    async fn get_events_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<EpisodicEvent>> {
        self.get_events_by_time_range(start, end).await
    }

    async fn apply_forgetting(&mut self, decay_rate: f64, min_importance: f64) -> Result<usize> {
        self.apply_forgetting(decay_rate, min_importance).await
    }

    async fn stats(&self) -> Result<MemoryStats> {
        let conn = self.connection.lock()
            .map_err(|_| BrainError::LockError("Failed to acquire database lock".to_string()))?;

        let total_items: usize = conn.query_row("SELECT COUNT(*) FROM episodic_events", [], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).map_err(|e| BrainError::DatabaseError(format!("Failed to count events: {}", e)))?;

        Ok(MemoryStats {
            total_items,
            size_bytes: total_items * std::mem::size_of::<EpisodicEvent>(),
            last_access: self.stats.last_access,
            access_count: self.stats.access_count,
            consolidation_count: self.stats.consolidation_count,
        })
    }
}

/// Advanced semantic memory with vector similarity and concept merging
pub struct SemanticMemoryRepository {
    concepts: HashMap<Uuid, SemanticConcept>,
    name_index: HashMap<String, Uuid>,
    stats: MemoryStats,
}

impl SemanticMemoryRepository {
    pub fn new() -> Self {
        Self {
            concepts: HashMap::new(),
            name_index: HashMap::new(),
            stats: MemoryStats {
                total_items: 0,
                size_bytes: 0,
                last_access: Utc::now(),
                access_count: 0,
                consolidation_count: 0,
            },
        }
    }

    /// Find similar concepts using cosine similarity
    pub async fn find_similar(&self, embedding: &[f32], threshold: f64, limit: usize) -> Result<Vec<(Uuid, f64)>> {
        let mut similarities: Vec<(Uuid, f64)> = self.concepts
            .iter()
            .map(|(id, concept)| (*id, cosine_similarity(embedding, &concept.embedding)))
            .filter(|(_, similarity)| *similarity >= threshold)
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.truncate(limit);

        Ok(similarities)
    }

    /// Merge two concepts into one
    pub async fn merge_concepts(&mut self, id1: Uuid, id2: Uuid) -> Result<Uuid> {
        let concept1 = self.concepts.remove(&id1)
            .ok_or_else(|| BrainError::NotFound(format!("Concept {} not found", id1)))?;
        let concept2 = self.concepts.remove(&id2)
            .ok_or_else(|| BrainError::NotFound(format!("Concept {} not found", id2)))?;

        // Remove from name index
        self.name_index.remove(&concept1.name);
        self.name_index.remove(&concept2.name);

        // Create merged concept
        let merged_name = format!("{}/{}", concept1.name, concept2.name);
        let merged_description = format!("{}; {}", concept1.description, concept2.description);
        let merged_embedding = average_embeddings(&[&concept1.embedding, &concept2.embedding]);
        let merged_frequency = concept1.frequency + concept2.frequency;
        let merged_confidence = (concept1.confidence + concept2.confidence) / 2.0;
        
        let mut merged_source_events = concept1.source_events;
        merged_source_events.extend(concept2.source_events);

        let merged_concept = SemanticConcept {
            id: Uuid::new_v4(),
            name: merged_name.clone(),
            description: merged_description,
            embedding: merged_embedding,
            frequency: merged_frequency,
            confidence: merged_confidence,
            last_updated: Utc::now(),
            source_events: merged_source_events,
        };

        let merged_id = merged_concept.id;
        self.name_index.insert(merged_name, merged_id);
        self.concepts.insert(merged_id, merged_concept);
        
        self.update_stats();
        Ok(merged_id)
    }

    fn update_stats(&mut self) {
        self.stats.total_items = self.concepts.len();
        self.stats.size_bytes = self.concepts.len() * std::mem::size_of::<SemanticConcept>();
        self.stats.last_access = Utc::now();
    }
}

impl Default for SemanticMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl brain_core::SemanticMemoryRepository for SemanticMemoryRepository {
    async fn store_concept(&mut self, concept: SemanticConcept) -> Result<Uuid> {
        let id = concept.id;
        self.name_index.insert(concept.name.clone(), id);
        self.concepts.insert(id, concept);
        self.update_stats();
        Ok(id)
    }

    async fn get_concept(&self, id: Uuid) -> Result<Option<SemanticConcept>> {
        Ok(self.concepts.get(&id).cloned())
    }

    async fn update_concept(&mut self, concept: &SemanticConcept) -> Result<()> {
        if let Some(old_concept) = self.concepts.get(&concept.id) {
            // Remove old name from index
            self.name_index.remove(&old_concept.name);
        }
        
        // Insert updated concept
        self.name_index.insert(concept.name.clone(), concept.id);
        self.concepts.insert(concept.id, concept.clone());
        self.update_stats();
        Ok(())
    }

    async fn remove_concept(&mut self, id: Uuid) -> Result<()> {
        if let Some(concept) = self.concepts.remove(&id) {
            self.name_index.remove(&concept.name);
            self.update_stats();
        }
        Ok(())
    }

    async fn query_concepts(&self, query: &SemanticQuery) -> Result<Vec<SemanticConcept>> {
        let mut results: Vec<SemanticConcept> = self.concepts.values().cloned().collect();

        // Apply filters
        if let Some(ref pattern) = query.name_pattern {
            results.retain(|concept| concept.name.contains(pattern));
        }

        if let Some(min_confidence) = query.min_confidence {
            results.retain(|concept| concept.confidence >= min_confidence);
        }

        if let Some(ref embedding) = query.embedding {
            if let Some(min_similarity) = query.min_similarity {
                results.retain(|concept| {
                    cosine_similarity(embedding, &concept.embedding) >= min_similarity
                });
            }
            
            // Sort by similarity if embedding provided
            results.sort_by(|a, b| {
                let sim_b = cosine_similarity(embedding, &b.embedding);
                let sim_a = cosine_similarity(embedding, &a.embedding);
                sim_b.partial_cmp(&sim_a).unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            // Sort by confidence
            results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        }

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn find_similar(&self, embedding: &[f32], threshold: f64, limit: usize) -> Result<Vec<(Uuid, f64)>> {
        self.find_similar(embedding, threshold, limit).await
    }

    async fn merge_concepts(&mut self, id1: Uuid, id2: Uuid) -> Result<Uuid> {
        self.merge_concepts(id1, id2).await
    }

    async fn stats(&self) -> Result<MemoryStats> {
        Ok(self.stats.clone())
    }
}

/// Utility function for cosine similarity calculation
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        (dot_product / (norm_a * norm_b)) as f64
    }
}

/// Average multiple embeddings into one
pub fn average_embeddings(embeddings: &[&Vec<f32>]) -> Vec<f32> {
    if embeddings.is_empty() {
        return Vec::new();
    }

    let len = embeddings[0].len();
    let mut result = vec![0.0; len];

    for embedding in embeddings {
        for (i, &value) in embedding.iter().enumerate() {
            if i < len {
                result[i] += value;
            }
        }
    }

    let count = embeddings.len() as f32;
    for value in &mut result {
        *value /= count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_core::{WorkingMemoryRepository as _, EpisodicMemoryRepository as _, SemanticMemoryRepository as _};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_working_memory_operations() {
        let mut repo = WorkingMemoryRepository::new(10);
        
        let id = repo.add_item("test content".to_string(), Priority::High).await.unwrap();
        let item = repo.get_item(id).await.unwrap().unwrap();
        
        assert_eq!(item.content, "test content");
        assert_eq!(item.priority, Priority::High);
    }

    #[tokio::test]
    async fn test_episodic_memory_persistence() -> Result<()> {
        let temp_file = NamedTempFile::new().unwrap();
        let mut repo = EpisodicMemoryRepository::new(temp_file.path()).await?;
        
        let event = EpisodicEvent::new(
            "test event".to_string(),
            HashMap::new(),
            0.8,
            "test".to_string(),
        );
        
        let id = repo.store_event(event.clone()).await?;
        let retrieved = repo.get_event(id).await?.unwrap();
        
        assert_eq!(retrieved.content, "test event");
        assert_eq!(retrieved.importance, 0.8);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_semantic_memory_similarity() -> Result<()> {
        let mut repo = SemanticMemoryRepository::new();
        
        let concept1 = SemanticConcept::new(
            "test1".to_string(),
            "description1".to_string(),
            vec![1.0, 0.0, 0.0],
        );
        
        let concept2 = SemanticConcept::new(
            "test2".to_string(),
            "description2".to_string(),
            vec![0.8, 0.6, 0.0],
        );
        
        repo.store_concept(concept1).await?;
        repo.store_concept(concept2).await?;
        
        let similar = repo.find_similar(&[1.0, 0.0, 0.0], 0.5, 10).await?;
        assert!(!similar.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);
        
        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &c) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_average_embeddings() {
        let emb1 = vec![1.0, 0.0];
        let emb2 = vec![0.0, 1.0];
        let avg = average_embeddings(&[&emb1, &emb2]);
        
        assert_eq!(avg, vec![0.5, 0.5]);
    }
} 