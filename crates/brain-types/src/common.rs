//! Common types and data structures shared across Brain AI crates

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Generic identifier type
pub type Id = Uuid;

/// Memory identifier
pub type MemoryId = Id;

/// Concept identifier 
pub type ConceptId = Id;

/// Session identifier
pub type SessionId = String;

/// Basic request for processing text content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessRequest {
    pub text: String,
    #[serde(default)]
    pub is_github_url: bool,
}

/// Query request structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryRequest {
    pub query: String,
}

/// Simulation request structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationRequest {
    pub scenario: String,
}

/// Chat message structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat request structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
}

/// Chat response structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResponse {
    pub response: String,
    pub context_used: bool,
    pub suggestions: Vec<String>,
}

/// Simplified chat learning request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleChatLearnRequest {
    pub content: String,
    #[serde(default = "default_true")]
    pub extract_insights: bool,
}

/// Simplified chat conversation request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleChatConverseRequest {
    pub message: String,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
}

/// Simplified chat response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleChatResponse {
    pub response: String,
    #[serde(default)]
    pub insights_learned: Vec<String>,
    #[serde(default)]
    pub context_used: bool,
}

/// Code pattern analysis request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodePatternAnalysisRequest {
    pub code_content: String,
    pub file_path: Option<String>,
    pub language: Option<String>,
    #[serde(default = "default_true")]
    pub store_patterns: bool,
    #[serde(default)]
    pub analysis_depth: PatternAnalysisDepth,
}

/// Pattern analysis depth enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PatternAnalysisDepth {
    Basic,      // Function signatures, class names
    Detailed,   // Include method bodies, relationships
    Deep,       // Full analysis with architectural patterns
}

impl Default for PatternAnalysisDepth {
    fn default() -> Self {
        PatternAnalysisDepth::Detailed
    }
}

/// Code pattern analysis response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodePatternAnalysisResponse {
    pub success: bool,
    pub patterns_found: Vec<CodePattern>,
    pub concepts_created: usize,
    pub relationships_formed: usize,
    pub analysis_time_ms: u64,
    pub confidence_score: f64,
    pub language_detected: Option<String>,
    pub architectural_insights: Vec<String>,
}

/// Code pattern structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodePattern {
    pub pattern_type: CodePatternType,
    pub name: String,
    pub description: String,
    pub code_snippet: Option<String>,
    pub file_location: Option<String>,
    pub confidence: f64,
    pub related_patterns: Vec<String>,
    pub concept_id: Option<String>,
}

/// Code pattern type enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CodePatternType {
    DataStructure,      // Classes, structs, interfaces
    Function,           // Functions, methods
    APIEndpoint,        // REST endpoints, routes
    DesignPattern,      // Singleton, Factory, etc.
    ArchitecturalPattern, // MVC, microservices, etc.
    ImportPattern,      // Dependencies, modules
    NamingConvention,   // Camel case, snake case, etc.
    ErrorHandling,      // Try-catch, Result types
    ConfigurationPattern, // Environment variables, config files
    TestPattern,        // Unit tests, integration tests
}

/// File access type enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileAccessType {
    Read,
    Write,
    Create,
    Delete,
    Execute,
    Navigate,
}

/// File access information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileAccess {
    pub file_path: String,
    pub access_type: FileAccessType,
    pub timestamp: DateTime<Utc>,
    pub line_numbers: Option<Vec<u32>>,
    pub content_preview: Option<String>,
}

/// Project context information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectContext {
    pub project_root: String,
    pub current_branch: Option<String>,
    pub active_features: Vec<String>,
    pub technology_stack: Vec<String>,
    pub recent_commits: Vec<String>,
}

/// Development context request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevelopmentContextRequest {
    pub session_id: Option<String>,
    pub files_accessed: Vec<FileAccess>,
    pub current_intent: Option<String>,
    pub development_goal: Option<String>,
    pub project_context: Option<ProjectContext>,
    #[serde(default = "default_true")]
    pub auto_save: bool,
}

/// Development session structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevelopmentSession {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub files_accessed: Vec<FileAccess>,
    pub development_intent: Option<String>,
    pub development_goal: Option<String>,
    pub project_context: Option<ProjectContext>,
    pub insights: Vec<String>,
    pub patterns_discovered: Vec<String>,
    pub confidence_score: f64,
}

/// Development context response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevelopmentContextResponse {
    pub success: bool,
    pub session_id: String,
    pub context_preserved: bool,
    pub insights_generated: Vec<String>,
    pub recommendations: Vec<String>,
    pub processing_time_ms: u64,
}

/// Development context query response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevelopmentContextQueryResponse {
    pub success: bool,
    pub session_found: bool,
    pub session: Option<DevelopmentSession>,
    pub related_sessions: Vec<String>,
    pub context_summary: Option<String>,
    pub processing_time_ms: u64,
}

/// Standard API response structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub processing_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusResponse {
    pub status: String,
    pub uptime: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatsResponse {
    pub memory_usage: String,
    pub confidence: f64,
    pub active_processes: u32,
    pub response_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthResponse {
    pub system_status: String,
    pub memory_efficiency: String,
    pub processing_speed: String,
    pub active_connections: u32,
    pub uptime: String,
    pub last_backup: String,
}

/// Helper functions
pub fn new_id() -> Id {
    Uuid::new_v4()
}

pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn default_true() -> bool {
    true
}
