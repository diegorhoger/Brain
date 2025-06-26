//! Configuration types and structures for Brain AI

use serde::{Deserialize, Serialize};

/// Main configuration structure for Brain AI
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrainConfig {
    pub server: ServerConfig,
    pub memory: MemoryConfig,
    pub cognitive: CognitiveConfig,
    pub analysis: AnalysisConfig,
}

/// Server configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

/// Memory system configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryConfig {
    pub max_episodic_memories: usize,
    pub semantic_threshold: f64,
    pub consolidation_interval_hours: u64,
    pub database_url: Option<String>,
}

/// Cognitive architecture configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CognitiveConfig {
    pub learning_rate: f64,
    pub confidence_threshold: f64,
    pub max_conversation_history: usize,
    pub enable_meta_learning: bool,
}

/// Code analysis configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisConfig {
    pub max_file_size_mb: u64,
    pub supported_languages: Vec<String>,
    pub pattern_confidence_threshold: f64,
    pub enable_tree_sitter: bool,
}

impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            memory: MemoryConfig::default(),
            cognitive: CognitiveConfig::default(),
            analysis: AnalysisConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
            max_connections: 1000,
            timeout_seconds: 30,
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_episodic_memories: 10000,
            semantic_threshold: 0.75,
            consolidation_interval_hours: 24,
            database_url: None,
        }
    }
}

impl Default for CognitiveConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            confidence_threshold: 0.7,
            max_conversation_history: 50,
            enable_meta_learning: true,
        }
    }
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            max_file_size_mb: 10,
            supported_languages: vec![
                "rust".to_string(),
                "javascript".to_string(),
                "typescript".to_string(),
                "python".to_string(),
                "java".to_string(),
            ],
            pattern_confidence_threshold: 0.8,
            enable_tree_sitter: true,
        }
    }
}
