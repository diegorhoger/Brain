//! Database Infrastructure
//! 
//! Database connection management and utilities for the Brain AI system.

use brain_types::*;
use sqlx::{SqlitePool, Row};
use std::path::Path;

/// Database connection manager
pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    /// Create a new database manager with SQLite
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| BrainError::DatabaseError(format!("Failed to connect to database: {}", e)))?;
        
        Ok(Self { pool })
    }

    /// Create a new in-memory database for testing
    pub async fn new_in_memory() -> Result<Self> {
        Self::new("sqlite::memory:").await
    }

    /// Create a new file-based database
    pub async fn new_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let database_url = format!("sqlite:{}", path.as_ref().display());
        Self::new(&database_url).await
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Initialize database schema
    pub async fn initialize_schema(&self) -> Result<()> {
        // Memory tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS working_memory (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                priority INTEGER NOT NULL,
                decay_factor REAL NOT NULL,
                created_at TEXT NOT NULL,
                last_accessed TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create working_memory table: {}", e)))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS episodic_memory (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                importance REAL NOT NULL,
                timestamp TEXT NOT NULL,
                tags TEXT NOT NULL,
                context TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create episodic_memory table: {}", e)))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS semantic_memory (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                embedding BLOB NOT NULL,
                confidence REAL NOT NULL,
                frequency INTEGER NOT NULL,
                last_updated TEXT NOT NULL,
                source_events TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create semantic_memory table: {}", e)))?;

        // Concept graph tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS concepts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                confidence REAL NOT NULL,
                current_activation REAL NOT NULL,
                total_activations INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                last_activated TEXT NOT NULL,
                tags TEXT NOT NULL,
                metadata TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create concepts table: {}", e)))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS relationships (
                id TEXT PRIMARY KEY,
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relationship_type TEXT NOT NULL,
                strength REAL NOT NULL,
                activation_count INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                last_activated TEXT NOT NULL,
                FOREIGN KEY (source_id) REFERENCES concepts (id),
                FOREIGN KEY (target_id) REFERENCES concepts (id)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create relationships table: {}", e)))?;

        // Segmentation tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS segments (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                segment_type TEXT NOT NULL,
                frequency INTEGER NOT NULL,
                confidence REAL NOT NULL,
                created_at TEXT NOT NULL,
                last_seen TEXT NOT NULL,
                archived INTEGER NOT NULL DEFAULT 0,
                archived_at TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create segments table: {}", e)))?;

        // Insights table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS insights (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                confidence REAL NOT NULL,
                source TEXT NOT NULL,
                insight_type TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create insights table: {}", e)))?;

        // Models table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS models (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                model_type TEXT NOT NULL,
                version TEXT NOT NULL,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL,
                metadata TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::DatabaseError(format!("Failed to create models table: {}", e)))?;

        Ok(())
    }

    /// Health check for the database connection
    pub async fn health_check(&self) -> Result<bool> {
        let result = sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| BrainError::DatabaseError(format!("Health check failed: {}", e)))?;

        let value: i32 = result.get(0);
        Ok(value == 1)
    }

    /// Close the database connection
    pub async fn close(self) {
        self.pool.close().await;
    }
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:brain.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            acquire_timeout_seconds: 30,
            idle_timeout_seconds: 600,
        }
    }
} 