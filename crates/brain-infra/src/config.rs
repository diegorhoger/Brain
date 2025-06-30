//! Configuration Infrastructure
//! 
//! Configuration management and environment variable handling for the Brain AI system.

use brain_types::*;
use serde::{Deserialize, Serialize};
use std::env;

/// Main configuration structure for Brain AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainConfig {
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    pub memory: MemoryConfig,
    pub learning: LearningConfig,
    pub external_apis: ExternalApiConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub rate_limit_requests_per_minute: u32,
    pub request_timeout_seconds: u64,
}

/// Memory system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub working_memory_capacity: usize,
    pub episodic_memory_retention_days: u32,
    pub semantic_memory_similarity_threshold: f64,
    pub consolidation_interval_hours: u32,
    pub decay_rate: f64,
}

/// Learning system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    pub curiosity_threshold: f64,
    pub novelty_detection_sensitivity: f64,
    pub insight_confidence_threshold: f64,
    pub pattern_recognition_depth: u32,
    pub learning_rate: f64,
}

/// External API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalApiConfig {
    pub github_token: Option<String>,
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub timeout_seconds: u64,
}

impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            api: ApiConfig::default(),
            memory: MemoryConfig::default(),
            learning: LearningConfig::default(),
            external_apis: ExternalApiConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:data/brain.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            acquire_timeout_seconds: 30,
            idle_timeout_seconds: 600,
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3030,
            cors_origins: vec!["*".to_string()],
            rate_limit_requests_per_minute: 100,
            request_timeout_seconds: 30,
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            working_memory_capacity: 1000,
            episodic_memory_retention_days: 30,
            semantic_memory_similarity_threshold: 0.8,
            consolidation_interval_hours: 24,
            decay_rate: 0.1,
        }
    }
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            curiosity_threshold: 0.7,
            novelty_detection_sensitivity: 0.8,
            insight_confidence_threshold: 0.6,
            pattern_recognition_depth: 5,
            learning_rate: 0.01,
        }
    }
}

impl Default for ExternalApiConfig {
    fn default() -> Self {
        Self {
            github_token: None,
            openai_api_key: None,
            anthropic_api_key: None,
            timeout_seconds: 30,
        }
    }
}

impl BrainConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Database configuration
        if let Ok(url) = env::var("DATABASE_URL") {
            config.database.url = url;
        }
        if let Ok(max_conn) = env::var("DATABASE_MAX_CONNECTIONS") {
            config.database.max_connections = max_conn.parse()
                .map_err(|_| BrainError::ConfigError("Invalid DATABASE_MAX_CONNECTIONS".to_string()))?;
        }

        // API configuration
        if let Ok(host) = env::var("API_HOST") {
            config.api.host = host;
        }
        if let Ok(port) = env::var("API_PORT") {
            config.api.port = port.parse()
                .map_err(|_| BrainError::ConfigError("Invalid API_PORT".to_string()))?;
        }
        if let Ok(cors) = env::var("API_CORS_ORIGINS") {
            config.api.cors_origins = cors.split(',').map(|s| s.trim().to_string()).collect();
        }

        // Memory configuration
        if let Ok(capacity) = env::var("MEMORY_WORKING_CAPACITY") {
            config.memory.working_memory_capacity = capacity.parse()
                .map_err(|_| BrainError::ConfigError("Invalid MEMORY_WORKING_CAPACITY".to_string()))?;
        }
        if let Ok(retention) = env::var("MEMORY_EPISODIC_RETENTION_DAYS") {
            config.memory.episodic_memory_retention_days = retention.parse()
                .map_err(|_| BrainError::ConfigError("Invalid MEMORY_EPISODIC_RETENTION_DAYS".to_string()))?;
        }
        if let Ok(threshold) = env::var("MEMORY_SEMANTIC_SIMILARITY_THRESHOLD") {
            config.memory.semantic_memory_similarity_threshold = threshold.parse()
                .map_err(|_| BrainError::ConfigError("Invalid MEMORY_SEMANTIC_SIMILARITY_THRESHOLD".to_string()))?;
        }

        // Learning configuration
        if let Ok(curiosity) = env::var("LEARNING_CURIOSITY_THRESHOLD") {
            config.learning.curiosity_threshold = curiosity.parse()
                .map_err(|_| BrainError::ConfigError("Invalid LEARNING_CURIOSITY_THRESHOLD".to_string()))?;
        }
        if let Ok(novelty) = env::var("LEARNING_NOVELTY_DETECTION_SENSITIVITY") {
            config.learning.novelty_detection_sensitivity = novelty.parse()
                .map_err(|_| BrainError::ConfigError("Invalid LEARNING_NOVELTY_DETECTION_SENSITIVITY".to_string()))?;
        }

        // External API configuration
        config.external_apis.github_token = env::var("GITHUB_TOKEN").ok();
        config.external_apis.openai_api_key = env::var("OPENAI_API_KEY").ok();
        config.external_apis.anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok();

        Ok(config)
    }

    /// Load configuration from a TOML file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BrainError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| BrainError::ConfigError(format!("Failed to parse config file: {}", e)))
    }

    /// Save configuration to a TOML file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| BrainError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| BrainError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate database configuration
        if self.database.url.is_empty() {
            return Err(BrainError::ConfigError("Database URL cannot be empty".to_string()));
        }
        if self.database.max_connections == 0 {
            return Err(BrainError::ConfigError("Database max_connections must be > 0".to_string()));
        }

        // Validate API configuration
        if self.api.port == 0 {
            return Err(BrainError::ConfigError("API port must be > 0".to_string()));
        }

        // Validate memory configuration
        if self.memory.working_memory_capacity == 0 {
            return Err(BrainError::ConfigError("Working memory capacity must be > 0".to_string()));
        }
        if !(0.0..=1.0).contains(&self.memory.semantic_memory_similarity_threshold) {
            return Err(BrainError::ConfigError("Semantic memory similarity threshold must be between 0.0 and 1.0".to_string()));
        }

        // Validate learning configuration
        if !(0.0..=1.0).contains(&self.learning.curiosity_threshold) {
            return Err(BrainError::ConfigError("Curiosity threshold must be between 0.0 and 1.0".to_string()));
        }
        if !(0.0..=1.0).contains(&self.learning.novelty_detection_sensitivity) {
            return Err(BrainError::ConfigError("Novelty detection sensitivity must be between 0.0 and 1.0".to_string()));
        }

        Ok(())
    }
} 