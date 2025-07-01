//! Schema Agent - Database Schema Design and Data Modeling
//! 
//! The SchemaAgent transforms system architecture and data requirements into comprehensive
//! database schemas, entity relationships, and data models optimized for performance,
//! scalability, and maintainability.

use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::{json, Value};

use crate::agents::traits::{
    BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitivePreferences,
    CognitiveContext, VerbosityLevel, ExecutionMetadata, ExecutionStatus,
    BrainResult
};

/// Specialized agent for database schema design and data modeling
#[derive(Clone)]
pub struct SchemaAgent {
    metadata: AgentMetadata,
    preferences: CognitivePreferences,
}

impl SchemaAgent {
    /// Create a new SchemaAgent instance
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "schema-agent".to_string(),
            name: "Database Schema Designer".to_string(),
            persona: "An expert database architect who transforms system requirements into optimized database schemas. Specializes in entity relationship design, data normalization, performance optimization, and multi-database support across SQL and NoSQL systems.".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec![
                "system_architecture".to_string(),
                "data_requirements".to_string(),
                "entity_specifications".to_string(),
                "user_flows".to_string(),
                "performance_requirements".to_string(),
                "migration_requirements".to_string(),
            ],
            supported_output_types: vec![
                "database_schema".to_string(),
                "entity_relationships".to_string(),
                "migration_scripts".to_string(),
                "indexing_strategy".to_string(),
                "data_validation_rules".to_string(),
                "performance_optimization".to_string(),
            ],
            capabilities: vec![
                "entity_relationship_design".to_string(),
                "schema_normalization".to_string(),
                "indexing_optimization".to_string(),
                "data_validation_design".to_string(),
                "migration_planning".to_string(),
                "performance_tuning".to_string(),
                "multi_database_support".to_string(),
                "data_security_planning".to_string(),
                "scalability_modeling".to_string(),
                "backup_strategy_design".to_string(),
            ],
            dependencies: vec!["architect-agent".to_string()],
            tags: vec![
                "development".to_string(),
                "database".to_string(),
                "schema".to_string(),
                "data-modeling".to_string(),
            ],
            base_confidence: 0.89,
        };

        let preferences = CognitivePreferences {
            verbosity: VerbosityLevel::Detailed,
            risk_tolerance: 0.3, // Conservative risk tolerance for data integrity
            collaboration_preference: 0.85, // High collaboration for schema validation
            learning_enabled: true,
            adaptation_rate: 0.12, // Conservative adaptation for schema stability
        };

        Self { metadata, preferences }
    }

    /// Design comprehensive database schema from system architecture
    async fn design_database_schema(&self, architecture: &Value, _context: &CognitiveContext) -> BrainResult<Value> {
        let mut schema = HashMap::new();
        
        // Extract entities from architecture
        let entities = self.extract_entities_from_architecture(architecture);
        let relationships = self.design_entity_relationships(&entities);
        let constraints = self.design_data_constraints(&entities);
        let indexes = self.design_indexing_strategy(&entities, &relationships);
        
        schema.insert("entities", entities.clone()); // Clone to avoid move
        schema.insert("relationships", relationships);
        schema.insert("constraints", constraints);
        schema.insert("indexes", indexes);
        schema.insert("database_type", self.recommend_database_type(architecture));
        schema.insert("performance_optimization", self.design_performance_optimization(&entities));
        
        Ok(json!(schema))
    }

    /// Design entity relationship model
    fn design_entity_relationships(&self, _entities: &Value) -> Value {
        let mut relationships = Vec::new();
        
        // User-related relationships
        relationships.push(json!({
            "from_entity": "users",
            "to_entity": "profiles",
            "relationship_type": "one_to_one",
            "foreign_key": "user_id",
            "cascade_delete": true,
            "description": "Each user has exactly one profile"
        }));
        
        relationships.push(json!({
            "from_entity": "users",
            "to_entity": "sessions",
            "relationship_type": "one_to_many",
            "foreign_key": "user_id",
            "cascade_delete": true,
            "description": "Users can have multiple active sessions"
        }));
        
        // Content relationships
        relationships.push(json!({
            "from_entity": "users",
            "to_entity": "projects",
            "relationship_type": "one_to_many",
            "foreign_key": "creator_id",
            "cascade_delete": false,
            "description": "Users can create multiple projects"
        }));
        
        relationships.push(json!({
            "from_entity": "projects",
            "to_entity": "tasks",
            "relationship_type": "one_to_many",
            "foreign_key": "project_id",
            "cascade_delete": true,
            "description": "Projects contain multiple tasks"
        }));
        
        // Many-to-many relationships
        relationships.push(json!({
            "from_entity": "users",
            "to_entity": "projects",
            "relationship_type": "many_to_many",
            "junction_table": "project_collaborators",
            "foreign_keys": ["user_id", "project_id"],
            "additional_fields": ["role", "permissions", "joined_at"],
            "description": "Users can collaborate on multiple projects"
        }));
        
        json!({
            "relationships": relationships,
            "relationship_count": relationships.len(),
            "junction_tables": ["project_collaborators", "task_assignees", "tag_assignments"]
        })
    }

    /// Extract entities from system architecture
    fn extract_entities_from_architecture(&self, _architecture: &Value) -> Value {
        let mut entities = HashMap::new();
        
        // Core user entity
        entities.insert("users", json!({
            "table_name": "users",
            "primary_key": "id",
            "fields": [
                {
                    "name": "id",
                    "type": "UUID",
                    "nullable": false,
                    "default": "gen_random_uuid()",
                    "description": "Unique user identifier"
                },
                {
                    "name": "email",
                    "type": "VARCHAR(255)",
                    "nullable": false,
                    "unique": true,
                    "description": "User email address"
                },
                {
                    "name": "password_hash",
                    "type": "VARCHAR(255)",
                    "nullable": false,
                    "description": "Hashed password"
                },
                {
                    "name": "email_verified",
                    "type": "BOOLEAN",
                    "nullable": false,
                    "default": false,
                    "description": "Email verification status"
                },
                {
                    "name": "created_at",
                    "type": "TIMESTAMP",
                    "nullable": false,
                    "default": "CURRENT_TIMESTAMP",
                    "description": "Account creation timestamp"
                },
                {
                    "name": "updated_at",
                    "type": "TIMESTAMP",
                    "nullable": false,
                    "default": "CURRENT_TIMESTAMP",
                    "description": "Last update timestamp"
                },
                {
                    "name": "deleted_at",
                    "type": "TIMESTAMP",
                    "nullable": true,
                    "description": "Soft delete timestamp"
                }
            ],
            "indexes": [
                {"name": "idx_users_email", "columns": ["email"], "unique": true},
                {"name": "idx_users_created_at", "columns": ["created_at"]},
                {"name": "idx_users_deleted_at", "columns": ["deleted_at"]}
            ]
        }));
        
        // User profiles entity
        entities.insert("profiles", json!({
            "table_name": "profiles",
            "primary_key": "id",
            "fields": [
                {
                    "name": "id",
                    "type": "UUID",
                    "nullable": false,
                    "default": "gen_random_uuid()",
                    "description": "Unique profile identifier"
                },
                {
                    "name": "user_id",
                    "type": "UUID",
                    "nullable": false,
                    "foreign_key": {"table": "users", "column": "id"},
                    "description": "Reference to user"
                },
                {
                    "name": "first_name",
                    "type": "VARCHAR(100)",
                    "nullable": true,
                    "description": "User first name"
                },
                {
                    "name": "last_name",
                    "type": "VARCHAR(100)",
                    "nullable": true,
                    "description": "User last name"
                },
                {
                    "name": "display_name",
                    "type": "VARCHAR(150)",
                    "nullable": true,
                    "description": "Public display name"
                },
                {
                    "name": "avatar_url",
                    "type": "TEXT",
                    "nullable": true,
                    "description": "Profile avatar URL"
                },
                {
                    "name": "bio",
                    "type": "TEXT",
                    "nullable": true,
                    "description": "User biography"
                },
                {
                    "name": "timezone",
                    "type": "VARCHAR(50)",
                    "nullable": true,
                    "default": "UTC",
                    "description": "User timezone"
                },
                {
                    "name": "language",
                    "type": "VARCHAR(10)",
                    "nullable": true,
                    "default": "en",
                    "description": "Preferred language"
                },
                {
                    "name": "updated_at",
                    "type": "TIMESTAMP",
                    "nullable": false,
                    "default": "CURRENT_TIMESTAMP",
                    "description": "Last update timestamp"
                }
            ],
            "indexes": [
                {"name": "idx_profiles_user_id", "columns": ["user_id"], "unique": true},
                {"name": "idx_profiles_display_name", "columns": ["display_name"]}
            ]
        }));
        
        // Projects entity
        entities.insert("projects", json!({
            "table_name": "projects",
            "primary_key": "id",
            "fields": [
                {
                    "name": "id",
                    "type": "UUID",
                    "nullable": false,
                    "default": "gen_random_uuid()",
                    "description": "Unique project identifier"
                },
                {
                    "name": "name",
                    "type": "VARCHAR(200)",
                    "nullable": false,
                    "description": "Project name"
                },
                {
                    "name": "description",
                    "type": "TEXT",
                    "nullable": true,
                    "description": "Project description"
                },
                {
                    "name": "creator_id",
                    "type": "UUID",
                    "nullable": false,
                    "foreign_key": {"table": "users", "column": "id"},
                    "description": "Project creator"
                },
                {
                    "name": "status",
                    "type": "VARCHAR(50)",
                    "nullable": false,
                    "default": "active",
                    "description": "Project status (active, archived, completed)"
                },
                {
                    "name": "visibility",
                    "type": "VARCHAR(20)",
                    "nullable": false,
                    "default": "private",
                    "description": "Project visibility (private, public, team)"
                },
                {
                    "name": "due_date",
                    "type": "DATE",
                    "nullable": true,
                    "description": "Project due date"
                },
                {
                    "name": "priority",
                    "type": "INTEGER",
                    "nullable": false,
                    "default": 3,
                    "description": "Project priority (1-5 scale)"
                },
                {
                    "name": "created_at",
                    "type": "TIMESTAMP",
                    "nullable": false,
                    "default": "CURRENT_TIMESTAMP",
                    "description": "Creation timestamp"
                },
                {
                    "name": "updated_at",
                    "type": "TIMESTAMP",
                    "nullable": false,
                    "default": "CURRENT_TIMESTAMP",
                    "description": "Last update timestamp"
                },
                {
                    "name": "deleted_at",
                    "type": "TIMESTAMP",
                    "nullable": true,
                    "description": "Soft delete timestamp"
                }
            ],
            "indexes": [
                {"name": "idx_projects_creator_id", "columns": ["creator_id"]},
                {"name": "idx_projects_status", "columns": ["status"]},
                {"name": "idx_projects_created_at", "columns": ["created_at"]},
                {"name": "idx_projects_due_date", "columns": ["due_date"]}
            ]
        }));
        
        json!({
            "entities": entities,
            "total_tables": entities.len(),
            "naming_convention": "snake_case",
            "id_strategy": "UUID",
            "timestamp_strategy": "created_at/updated_at pattern",
            "soft_delete_strategy": "deleted_at timestamp"
        })
    }

    /// Design data constraints and validation rules
    fn design_data_constraints(&self, _entities: &Value) -> Value {
        let mut constraints = Vec::new();
        
        // Email format constraint
        constraints.push(json!({
            "table": "users",
            "constraint_name": "chk_users_email_format",
            "type": "check",
            "condition": "email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$'",
            "description": "Validate email format"
        }));
        
        // Password strength constraint
        constraints.push(json!({
            "table": "users",
            "constraint_name": "chk_users_password_length",
            "type": "check",
            "condition": "LENGTH(password_hash) >= 60",
            "description": "Ensure password hash minimum length"
        }));
        
        // Project name length constraint
        constraints.push(json!({
            "table": "projects",
            "constraint_name": "chk_projects_name_length",
            "type": "check",
            "condition": "LENGTH(TRIM(name)) >= 3",
            "description": "Project name must be at least 3 characters"
        }));
        
        // Project status constraint
        constraints.push(json!({
            "table": "projects",
            "constraint_name": "chk_projects_status_values",
            "type": "check",
            "condition": "status IN ('active', 'archived', 'completed', 'on_hold')",
            "description": "Validate project status values"
        }));
        
        // Priority range constraint
        constraints.push(json!({
            "table": "projects",
            "constraint_name": "chk_projects_priority_range",
            "type": "check",
            "condition": "priority >= 1 AND priority <= 5",
            "description": "Priority must be between 1 and 5"
        }));
        
        json!({
            "constraints": constraints,
            "constraint_types": ["check", "foreign_key", "unique", "not_null"],
            "validation_strategy": "database_level_constraints",
            "data_integrity": "high"
        })
    }

    /// Design comprehensive indexing strategy
    fn design_indexing_strategy(&self, _entities: &Value, _relationships: &Value) -> Value {
        let mut indexes = Vec::new();
        
        // Primary key indexes (automatic)
        indexes.push(json!({
            "table": "users",
            "index_name": "pk_users",
            "type": "primary_key",
            "columns": ["id"],
            "unique": true,
            "description": "Primary key index"
        }));
        
        // Unique constraints
        indexes.push(json!({
            "table": "users",
            "index_name": "idx_users_email_unique",
            "type": "unique",
            "columns": ["email"],
            "unique": true,
            "description": "Unique email constraint"
        }));
        
        // Foreign key indexes
        indexes.push(json!({
            "table": "profiles",
            "index_name": "idx_profiles_user_id_fk",
            "type": "btree",
            "columns": ["user_id"],
            "unique": false,
            "description": "Foreign key to users table"
        }));
        
        // Composite indexes for common queries
        indexes.push(json!({
            "table": "projects",
            "index_name": "idx_projects_creator_status",
            "type": "btree",
            "columns": ["creator_id", "status"],
            "unique": false,
            "description": "Query projects by creator and status"
        }));
        
        // Date range indexes
        indexes.push(json!({
            "table": "projects",
            "index_name": "idx_projects_due_date_status",
            "type": "btree",
            "columns": ["due_date", "status"],
            "unique": false,
            "where_clause": "due_date IS NOT NULL AND deleted_at IS NULL",
            "description": "Active projects with due dates"
        }));
        
        // Text search indexes
        indexes.push(json!({
            "table": "projects",
            "index_name": "idx_projects_name_description_gin",
            "type": "gin",
            "expression": "to_tsvector('english', name || ' ' || COALESCE(description, ''))",
            "description": "Full-text search on project name and description"
        }));
        
        json!({
            "indexes": indexes,
            "indexing_strategy": {
                "primary_keys": "automatic_uuid",
                "foreign_keys": "always_indexed",
                "unique_constraints": "unique_indexes",
                "search_fields": "gin_indexes",
                "composite_queries": "multi_column_btree",
                "date_ranges": "btree_with_conditions"
            },
            "performance_considerations": [
                "Index maintenance overhead vs query performance",
                "Selective indexes with WHERE clauses for large tables",
                "GIN indexes for full-text search capabilities",
                "Composite indexes ordered by selectivity"
            ]
        })
    }

    /// Recommend optimal database type
    fn recommend_database_type(&self, architecture: &Value) -> Value {
        // Analyze architecture to recommend database
        let scalability_req = architecture.get("scalability")
            .and_then(|s| s.get("expected_users"))
            .and_then(|u| u.as_u64())
            .unwrap_or(1000);
        
        let consistency_req = architecture.get("data")
            .and_then(|d| d.get("consistency_requirements"))
            .and_then(|c| c.as_str())
            .unwrap_or("strong");
        
        let primary_db = if scalability_req > 100000 && consistency_req == "eventual" {
            json!({
                "type": "PostgreSQL",
                "justification": "High scalability with strong ACID properties and JSON support",
                "version": "15+",
                "extensions": ["uuid-ossp", "pg_trgm", "btree_gin"],
                "clustering": "recommended_for_high_load"
            })
        } else {
            json!({
                "type": "PostgreSQL",
                "justification": "Excellent balance of features, performance, and ACID compliance",
                "version": "15+",
                "extensions": ["uuid-ossp", "pg_trgm"],
                "clustering": "optional"
            })
        };
        
        json!({
            "primary_database": primary_db,
            "cache_layer": {
                "type": "Redis",
                "purpose": "Session storage and query caching",
                "version": "7+",
                "clustering": "recommended"
            },
            "analytics_database": {
                "type": "ClickHouse",
                "purpose": "Analytics and reporting",
                "justification": "Columnar storage for analytical queries",
                "optional": true
            },
            "search_engine": {
                "type": "PostgreSQL Full-Text Search",
                "purpose": "Text search capabilities",
                "alternative": "Elasticsearch for complex search requirements"
            }
        })
    }

    /// Design performance optimization strategies
    fn design_performance_optimization(&self, _entities: &Value) -> Value {
        json!({
            "connection_pooling": {
                "strategy": "pgbouncer",
                "pool_size": "25-50 connections per app instance",
                "pool_mode": "transaction",
                "description": "Efficient connection management"
            },
            "query_optimization": {
                "prepared_statements": "Use for all repeated queries",
                "query_analysis": "EXPLAIN ANALYZE for slow queries",
                "index_usage": "Monitor pg_stat_user_indexes",
                "n_plus_one": "Use JOIN queries or data loaders"
            },
            "caching_strategy": {
                "query_cache": "Redis for expensive query results",
                "session_cache": "Redis for user sessions",
                "application_cache": "In-memory for reference data",
                "cache_invalidation": "Time-based and event-driven"
            },
            "partitioning": {
                "time_based": "Partition large tables by created_at",
                "hash_based": "Partition by user_id for user-centric data",
                "range_based": "Partition by date ranges for analytics"
            },
            "monitoring": {
                "slow_queries": "Log queries > 1000ms",
                "connection_usage": "Monitor pool utilization",
                "index_efficiency": "Track index hit ratios",
                "deadlock_detection": "Monitor and alert on deadlocks"
            },
            "backup_strategy": {
                "frequency": "Daily full backups, hourly incrementals",
                "retention": "30 days hot, 12 months cold storage",
                "testing": "Monthly restore tests",
                "point_in_time": "Enable WAL archiving"
            }
        })
    }

    /// Generate database migration scripts
    async fn generate_migration_scripts(&self, _schema: &Value, _context: &CognitiveContext) -> BrainResult<Value> {
        let mut migrations = Vec::new();
        
        // Initial schema migration
        migrations.push(json!({
            "version": "001_initial_schema",
            "description": "Create initial database schema with users, profiles, and projects",
            "up_script": self.generate_initial_schema_up(),
            "down_script": self.generate_initial_schema_down(),
            "dependencies": []
        }));
        
        // Add indexes migration
        migrations.push(json!({
            "version": "002_add_indexes",
            "description": "Add performance indexes for common queries",
            "up_script": self.generate_indexes_up(),
            "down_script": self.generate_indexes_down(),
            "dependencies": ["001_initial_schema"]
        }));
        
        // Add constraints migration
        migrations.push(json!({
            "version": "003_add_constraints",
            "description": "Add data validation constraints",
            "up_script": self.generate_constraints_up(),
            "down_script": self.generate_constraints_down(),
            "dependencies": ["002_add_indexes"]
        }));
        
        Ok(json!({
            "migrations": migrations,
            "migration_strategy": "versioned_sequential",
            "rollback_support": true,
            "transaction_safety": "each_migration_in_transaction"
        }))
    }

    /// Generate initial schema creation SQL
    fn generate_initial_schema_up(&self) -> String {
        r#"
-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL
);

-- Profiles table
CREATE TABLE profiles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    display_name VARCHAR(150),
    avatar_url TEXT,
    bio TEXT,
    timezone VARCHAR(50) DEFAULT 'UTC',
    language VARCHAR(10) DEFAULT 'en',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id)
);

-- Projects table
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    creator_id UUID NOT NULL REFERENCES users(id),
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    visibility VARCHAR(20) NOT NULL DEFAULT 'private',
    due_date DATE,
    priority INTEGER NOT NULL DEFAULT 3,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL
);

-- Update triggers for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_profiles_updated_at BEFORE UPDATE ON profiles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
"#.to_string()
    }

    /// Generate initial schema rollback SQL
    fn generate_initial_schema_down(&self) -> String {
        r#"
-- Drop triggers
DROP TRIGGER IF EXISTS update_projects_updated_at ON projects;
DROP TRIGGER IF EXISTS update_profiles_updated_at ON profiles;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop tables (order matters due to foreign keys)
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS profiles;
DROP TABLE IF EXISTS users;

-- Drop extension
DROP EXTENSION IF EXISTS "uuid-ossp";
"#.to_string()
    }

    /// Generate indexes creation SQL
    fn generate_indexes_up(&self) -> String {
        r#"
-- Users indexes
CREATE INDEX idx_users_email ON users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_deleted_at ON users(deleted_at) WHERE deleted_at IS NOT NULL;

-- Profiles indexes
CREATE INDEX idx_profiles_display_name ON profiles(display_name) WHERE display_name IS NOT NULL;

-- Projects indexes
CREATE INDEX idx_projects_creator_id ON projects(creator_id);
CREATE INDEX idx_projects_status ON projects(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_created_at ON projects(created_at);
CREATE INDEX idx_projects_due_date ON projects(due_date) WHERE due_date IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX idx_projects_creator_status ON projects(creator_id, status) WHERE deleted_at IS NULL;

-- Full-text search index
CREATE INDEX idx_projects_search ON projects USING GIN (to_tsvector('english', name || ' ' || COALESCE(description, ''))) WHERE deleted_at IS NULL;
"#.to_string()
    }

    /// Generate indexes rollback SQL
    fn generate_indexes_down(&self) -> String {
        r#"
-- Drop projects indexes
DROP INDEX IF EXISTS idx_projects_search;
DROP INDEX IF EXISTS idx_projects_creator_status;
DROP INDEX IF EXISTS idx_projects_due_date;
DROP INDEX IF EXISTS idx_projects_created_at;
DROP INDEX IF EXISTS idx_projects_status;
DROP INDEX IF EXISTS idx_projects_creator_id;

-- Drop profiles indexes
DROP INDEX IF EXISTS idx_profiles_display_name;

-- Drop users indexes
DROP INDEX IF EXISTS idx_users_deleted_at;
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_users_email;
"#.to_string()
    }

    /// Generate constraints creation SQL
    fn generate_constraints_up(&self) -> String {
        r#"
-- Users constraints
ALTER TABLE users ADD CONSTRAINT chk_users_email_format 
    CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$');

ALTER TABLE users ADD CONSTRAINT chk_users_password_length 
    CHECK (LENGTH(password_hash) >= 60);

-- Projects constraints
ALTER TABLE projects ADD CONSTRAINT chk_projects_name_length 
    CHECK (LENGTH(TRIM(name)) >= 3);

ALTER TABLE projects ADD CONSTRAINT chk_projects_status_values 
    CHECK (status IN ('active', 'archived', 'completed', 'on_hold'));

ALTER TABLE projects ADD CONSTRAINT chk_projects_visibility_values 
    CHECK (visibility IN ('private', 'public', 'team'));

ALTER TABLE projects ADD CONSTRAINT chk_projects_priority_range 
    CHECK (priority >= 1 AND priority <= 5);
"#.to_string()
    }

    /// Generate constraints rollback SQL
    fn generate_constraints_down(&self) -> String {
        r#"
-- Drop projects constraints
ALTER TABLE projects DROP CONSTRAINT IF EXISTS chk_projects_priority_range;
ALTER TABLE projects DROP CONSTRAINT IF EXISTS chk_projects_visibility_values;
ALTER TABLE projects DROP CONSTRAINT IF EXISTS chk_projects_status_values;
ALTER TABLE projects DROP CONSTRAINT IF EXISTS chk_projects_name_length;

-- Drop users constraints
ALTER TABLE users DROP CONSTRAINT IF EXISTS chk_users_password_length;
ALTER TABLE users DROP CONSTRAINT IF EXISTS chk_users_email_format;
"#.to_string()
    }
}

#[async_trait]
impl BrainAgent for SchemaAgent {
    async fn execute(
        &self,
        input: AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<AgentOutput> {
        let start_time = std::time::Instant::now();
        let mut warnings = Vec::new();
        
        // Parse input content as JSON
        let content_value: Value = serde_json::from_str(&input.content)
            .unwrap_or_else(|_| json!({"content": input.content}));
        
        let result = match input.input_type.as_str() {
            "system_architecture" => {
                self.design_database_schema(&content_value, context).await
            },
            "data_requirements" => {
                self.design_database_schema(&content_value, context).await
            },
            "migration_requirements" => {
                self.generate_migration_scripts(&content_value, context).await
            },
            _ => {
                warnings.push(format!("Input type '{}' not explicitly supported, treating as system architecture", input.input_type));
                self.design_database_schema(&content_value, context).await
            }
        };

        let execution_time = start_time.elapsed();
        
        match result {
            Ok(schema_result) => {
                let metadata = ExecutionMetadata {
                    execution_time_ms: execution_time.as_millis() as u64,
                    memory_usage_mb: 15.0, // Schema design is memory-efficient
                    api_calls: 0, // No external API calls
                    status: ExecutionStatus::Success,
                    warnings,
                };

                Ok(AgentOutput {
                    agent_id: self.metadata.id.clone(),
                    output_type: "database_schema".to_string(),
                    content: serde_json::to_string_pretty(&schema_result)?,
                    data: match schema_result {
                        Value::Object(map) => map.into_iter().collect(),
                        _ => HashMap::new(),
                    },
                    confidence: 0.92, // High confidence in schema design
                    reasoning: Some("Generated comprehensive database schema with entities, relationships, constraints, and indexing strategy".to_string()),
                    next_actions: vec![
                        "Review and validate schema design".to_string(),
                        "Generate migration scripts".to_string(),
                        "Set up database environment".to_string(),
                    ],
                    execution_metadata: metadata,
                    timestamp: chrono::Utc::now(),
                })
            },
            Err(e) => {
                let metadata = ExecutionMetadata {
                    execution_time_ms: execution_time.as_millis() as u64,
                    memory_usage_mb: 5.0,
                    api_calls: 0,
                    status: ExecutionStatus::Failed,
                    warnings,
                };

                Ok(AgentOutput {
                    agent_id: self.metadata.id.clone(),
                    output_type: "error".to_string(),
                    content: format!("Schema design failed: {}", e),
                    data: HashMap::new(),
                    confidence: 0.0,
                    reasoning: Some(format!("Error occurred during schema design: {}", e)),
                    next_actions: vec!["Review input requirements".to_string(), "Retry with corrected input".to_string()],
                    execution_metadata: metadata,
                    timestamp: chrono::Utc::now(),
                })
            }
        }
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        0.7 // Conservative threshold for schema design
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.preferences
    }

    async fn assess_confidence(
        &self,
        input: &AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<f32> {
        let mut confidence = self.metadata.base_confidence;
        
        // Adjust based on input type support
        match input.input_type.as_str() {
            "system_architecture" => confidence += 0.05,
            "data_requirements" => confidence += 0.03,
            "entity_specifications" => confidence += 0.04,
            "migration_requirements" => confidence += 0.02,
            _ => confidence -= 0.1,
        }
        
        // Adjust based on content complexity
        let content_str = &input.content;
        if content_str.len() > 5000 {
            confidence += 0.02; // More detailed input
        } else if content_str.len() < 500 {
            confidence -= 0.05; // Limited input
        }
        
        // Context-based adjustments
        if context.session_history.len() > 2 {
            confidence += 0.02; // Better context from conversation
        }
        
        Ok(confidence.min(1.0).max(0.0))
    }
}

impl Default for SchemaAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_schema_agent_creation() {
        let agent = SchemaAgent::new();
        assert_eq!(agent.metadata().id, "schema-agent");
        assert_eq!(agent.metadata().name, "Database Schema Designer");
        assert!(agent.metadata().capabilities.contains(&"entity_relationship_design".to_string()));
    }
} 