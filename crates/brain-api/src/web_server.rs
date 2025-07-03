//! Web Server Module - Comprehensive API Server for Brain AI
//!
//! This module provides a full-featured web server with extensive API endpoints
//! for all Brain AI functionality including memory operations, concept graphs,
//! pattern detection, RAG conversations, and development context tracking.

use brain_types::*;
use brain_core::{WorkingMemoryItem, WorkingMemoryQuery, Priority, WorkingMemoryRepository as WorkingMemoryRepositoryTrait};
use brain_infra::{WorkingMemoryRepository, ConceptGraphManager, InMemoryInsightRepository, ConceptGraphConfig};
// Removed unused brain_cognitive import
use crate::agents::{
    AgentApiManager, AgentExecutionRequest, WorkflowExecutionRequest,
};
use crate::websocket::WebSocketManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Reply};
use std::path::{Path, PathBuf};
use std::fs;

// Request/Response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessRequest {
    pub text: String,
    #[serde(default)]
    pub is_github_url: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationRequest {
    pub scenario: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub context_used: bool,
    pub suggestions: Vec<String>,
}

// Simplified chat structures
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleChatLearnRequest {
    pub content: String,
    #[serde(default = "default_true")]
    pub extract_insights: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleChatConverseRequest {
    pub message: String,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleChatResponse {
    pub response: String,
    #[serde(default)]
    pub insights_learned: Vec<String>,
    #[serde(default)]
    pub context_used: bool,
}

// Code Pattern Recognition API structures
#[derive(Debug, Serialize, Deserialize)]
pub struct CodePatternAnalysisRequest {
    pub code_content: String,
    pub file_path: Option<String>,
    pub language: Option<String>,
    #[serde(default = "default_true")]
    pub store_patterns: bool,
    #[serde(default)]
    pub analysis_depth: PatternAnalysisDepth,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

fn default_true() -> bool {
    true
}

/// Extract GitHub URL from a text message
fn extract_github_url(text: &str) -> Option<String> {
    // Look for GitHub URLs in the text
    let patterns = [
        "https://github.com/",
        "http://github.com/",
        "github.com/",
    ];
    
    for pattern in patterns {
        if let Some(start) = text.find(pattern) {
            let url_start = if pattern.starts_with("http") {
                start
            } else {
                // Add https:// prefix if not present
                return Some(format!("https://{}", &text[start..]
                    .split_whitespace()
                    .next()
                    .unwrap_or("")));
            };
            
            // Extract the URL (until whitespace or end)
            let url_part = &text[url_start..]
                .split_whitespace()
                .next()
                .unwrap_or("");
                
            // Clean up trailing punctuation
            let url = url_part.trim_end_matches(['.', ',', '!', '?', ')', ']', '}']);
            
            if !url.is_empty() {
                return Some(url.to_string());
            }
        }
    }
    
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub processing_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub uptime: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub memory_usage: String,
    pub confidence: f64,
    pub active_processes: u32,
    pub response_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub system_status: String,
    pub memory_efficiency: String,
    pub processing_speed: String,
    pub active_connections: u32,
    pub uptime: String,
    pub last_backup: String,
}

// Enhanced Development Context API structures
#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentContextRequest {
    pub session_id: Option<String>,
    pub files_accessed: Vec<FileAccess>,
    pub current_intent: Option<String>,
    pub development_goal: Option<String>,
    pub project_context: Option<ProjectContext>,
    #[serde(default = "default_true")]
    pub auto_save: bool,
    #[serde(default)]
    pub merge_with_existing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccess {
    pub file_path: String,
    pub access_type: FileAccessType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub line_numbers: Option<Vec<u32>>,
    pub content_preview: Option<String>,
    pub file_size: Option<u64>,
    pub language: Option<String>,
    pub change_type: Option<ChangeType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileAccessType {
    Read,
    Write,
    Create,
    Delete,
    Execute,
    Navigate,
    Debug,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Addition,
    Modification,
    Deletion,
    Refactor,
    BugFix,
    Feature,
    Documentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub project_root: String,
    pub current_branch: Option<String>,
    pub active_features: Vec<String>,
    pub technology_stack: Vec<String>,
    pub recent_commits: Vec<String>,
    pub dependencies: Option<Vec<String>>,
    pub build_system: Option<String>,
    pub test_framework: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentContextResponse {
    pub success: bool,
    pub session_id: String,
    pub context_preserved: bool,
    pub insights_generated: Vec<String>,
    pub recommendations: Vec<String>,
    pub processing_time_ms: u64,
    pub intent_recognized: Option<DevelopmentIntent>,
    pub patterns_detected: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentSession {
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub files_accessed: Vec<FileAccess>,
    pub development_intent: Option<String>,
    pub development_goal: Option<String>,
    pub project_context: Option<ProjectContext>,
    pub insights: Vec<String>,
    pub patterns_discovered: Vec<String>,
    pub confidence_score: f64,
    pub session_tags: Vec<String>,
    pub focus_areas: Vec<String>,
    pub productivity_metrics: ProductivityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductivityMetrics {
    pub files_modified: u32,
    pub lines_added: u32,
    pub lines_removed: u32,
    pub commits_made: u32,
    pub tests_written: u32,
    pub bugs_fixed: u32,
    pub session_duration_minutes: u32,
}

impl Default for ProductivityMetrics {
    fn default() -> Self {
        Self {
            files_modified: 0,
            lines_added: 0,
            lines_removed: 0,
            commits_made: 0,
            tests_written: 0,
            bugs_fixed: 0,
            session_duration_minutes: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DevelopmentIntent {
    FeatureDevelopment,
    BugFixing,
    Refactoring,
    Testing,
    Documentation,
    CodeReview,
    Architecture,
    Performance,
    Security,
    Debugging,
    Learning,
    Experimentation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentContextQueryResponse {
    pub success: bool,
    pub session_found: bool,
    pub session: Option<DevelopmentSession>,
    pub related_sessions: Vec<String>,
    pub context_summary: Option<String>,
    pub processing_time_ms: u64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub success: bool,
    pub sessions: Vec<SessionSummary>,
    pub total_count: usize,
    pub active_sessions: usize,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub files_count: usize,
    pub intent: Option<String>,
    pub tags: Vec<String>,
    pub duration_minutes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextAnalysisRequest {
    pub project_root: Option<String>,
    pub time_window_hours: Option<u32>,
    pub include_patterns: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextAnalysisResponse {
    pub success: bool,
    pub analysis_summary: String,
    pub development_patterns: Vec<DevelopmentPattern>,
    pub productivity_insights: Vec<String>,
    pub recommendations: Vec<String>,
    pub focus_areas: Vec<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentPattern {
    pub pattern_type: String,
    pub description: String,
    pub frequency: u32,
    pub confidence: f64,
    pub impact: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUpdateRequest {
    pub development_intent: Option<String>,
    pub development_goal: Option<String>,
    pub tags: Option<Vec<String>>,
    pub additional_files: Option<Vec<FileAccess>>,
    pub project_context: Option<ProjectContext>,
}

pub struct WebServer {
    port: u16,
    memory_repository: Arc<Mutex<WorkingMemoryRepository>>,
    concept_manager: Arc<Mutex<ConceptGraphManager>>,
    insight_repository: Arc<Mutex<InMemoryInsightRepository>>,
    development_sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
    sessions_file_path: PathBuf,
    agent_api_manager: Arc<AgentApiManager>,
    websocket_manager: Arc<WebSocketManager>,
}

impl WebServer {
    /// Create a new web server instance
    pub async fn new(port: u16) -> Result<Self> {
        let memory_repository = Arc::new(Mutex::new(WorkingMemoryRepository::new(1000)));
        let concept_config = ConceptGraphConfig {
            uri: "bolt://localhost:7687".to_string(),
            username: "neo4j".to_string(),
            password: "password".to_string(),
            database: None,
            pool_size: 10,
            timeout_seconds: 30,
        };
        let concept_manager = Arc::new(Mutex::new(ConceptGraphManager::new(concept_config).await?));
        let insight_repository = Arc::new(Mutex::new(InMemoryInsightRepository::new()));
        let development_sessions = Arc::new(Mutex::new(HashMap::new()));
        
        // Initialize Agent API Manager
        let agent_api_manager = Arc::new(AgentApiManager::new().await?);
        
        // Initialize WebSocket Manager
        let websocket_manager = Arc::new(WebSocketManager::new());
        
        // Create sessions directory if it doesn't exist
        let sessions_dir = Path::new("data/sessions");
        if !sessions_dir.exists() {
            fs::create_dir_all(sessions_dir).map_err(|e| BrainError::Io { source: e })?;
        }
        
        let sessions_file_path = sessions_dir.join("development_sessions.json");
        
        // Load existing sessions if file exists
        let mut server = Self {
            port,
            memory_repository,
            concept_manager,
            insight_repository,
            development_sessions,
            sessions_file_path,
            agent_api_manager,
            websocket_manager,
        };
        
        server.load_sessions().await?;
        
        Ok(server)
    }

    /// Load sessions from persistent storage
    async fn load_sessions(&mut self) -> Result<()> {
        if self.sessions_file_path.exists() {
            match fs::read_to_string(&self.sessions_file_path) {
                Ok(content) => {
                    match serde_json::from_str::<HashMap<String, DevelopmentSession>>(&content) {
                        Ok(sessions) => {
                            let mut sessions_map = self.development_sessions.lock().await;
                            *sessions_map = sessions;
                            println!("Loaded {} development sessions", sessions_map.len());
                        }
                        Err(e) => {
                            eprintln!("Failed to parse sessions file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read sessions file: {}", e);
                }
            }
        }
        Ok(())
    }

    /// Analyze file access patterns to recognize development intent
    fn recognize_intent(files_accessed: &[FileAccess], _project_context: &Option<ProjectContext>) -> Option<DevelopmentIntent> {
        let file_paths: Vec<&str> = files_accessed.iter().map(|f| f.file_path.as_str()).collect();
        let access_types: Vec<&FileAccessType> = files_accessed.iter().map(|f| &f.access_type).collect();
        
        // Test-related patterns
        if file_paths.iter().any(|p| p.contains("test") || p.contains("spec")) ||
           access_types.iter().any(|t| matches!(t, FileAccessType::Test)) {
            return Some(DevelopmentIntent::Testing);
        }
        
        // Documentation patterns
        if file_paths.iter().any(|p| p.ends_with(".md") || p.ends_with(".txt") || p.contains("doc")) {
            return Some(DevelopmentIntent::Documentation);
        }
        
        // Configuration and build patterns
        if file_paths.iter().any(|p| p.contains("config") || p.contains("Cargo.toml") || p.contains("package.json")) {
            return Some(DevelopmentIntent::Architecture);
        }
        
        // Debug patterns
        if access_types.iter().any(|t| matches!(t, FileAccessType::Debug)) {
            return Some(DevelopmentIntent::Debugging);
        }
        
        // Bug fixing patterns (looking for specific change types)
        if files_accessed.iter().any(|f| matches!(f.change_type, Some(ChangeType::BugFix))) {
            return Some(DevelopmentIntent::BugFixing);
        }
        
        // Refactoring patterns
        if files_accessed.iter().any(|f| matches!(f.change_type, Some(ChangeType::Refactor))) {
            return Some(DevelopmentIntent::Refactoring);
        }
        
        // Feature development (default for new files and modifications)
        if access_types.iter().any(|t| matches!(t, FileAccessType::Create | FileAccessType::Write)) {
            return Some(DevelopmentIntent::FeatureDevelopment);
        }
        
        None
    }

    /// Generate insights based on session data
    fn generate_insights(session: &DevelopmentSession) -> Vec<String> {
        let mut insights = Vec::new();
        
        // File access patterns
        if session.files_accessed.len() > 10 {
            insights.push("High file activity detected - consider focusing on fewer files for better productivity".to_string());
        }
        
        // Language diversity
        let languages: std::collections::HashSet<_> = session.files_accessed
            .iter()
            .filter_map(|f| f.language.as_ref())
            .collect();
        if languages.len() > 3 {
            insights.push(format!("Working with {} different languages - context switching may impact productivity", languages.len()));
        }
        
        // Time-based insights
        let duration = chrono::Utc::now().signed_duration_since(session.start_time);
        if duration.num_hours() > 4 {
            insights.push("Long development session detected - consider taking breaks for better focus".to_string());
        }
        
        // File type patterns
        let test_files = session.files_accessed.iter().filter(|f| 
            f.file_path.contains("test") || f.file_path.contains("spec")
        ).count();
        let total_files = session.files_accessed.len();
        
        if total_files > 0 && test_files as f64 / (total_files as f64) < 0.2 {
            insights.push("Low test file activity - consider adding more tests for better code quality".to_string());
        }
        
        insights
    }

    /// Generate recommendations based on session analysis
    fn generate_recommendations(session: &DevelopmentSession) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Based on intent
        match session.development_intent.as_deref() {
            Some("FeatureDevelopment") => {
                recommendations.push("Consider writing tests for new features".to_string());
                recommendations.push("Document new functionality for future reference".to_string());
            }
            Some("BugFixing") => {
                recommendations.push("Add regression tests to prevent similar bugs".to_string());
                recommendations.push("Update documentation if behavior changed".to_string());
            }
            Some("Refactoring") => {
                recommendations.push("Ensure all tests pass after refactoring".to_string());
                recommendations.push("Update documentation to reflect structural changes".to_string());
            }
            _ => {}
        }
        
        // File-based recommendations
        let has_rust_files = session.files_accessed.iter().any(|f| f.file_path.ends_with(".rs"));
        let has_cargo_toml = session.files_accessed.iter().any(|f| f.file_path.contains("Cargo.toml"));
        
        if has_rust_files && !has_cargo_toml {
            recommendations.push("Consider checking Cargo.toml for dependency updates".to_string());
        }
        
        recommendations
    }

    /// Start the web server
    pub async fn start(&self) -> Result<()> {
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

        // Clone Arc references for use in routes
        let memory_repo = self.memory_repository.clone();
        let concept_mgr = self.concept_manager.clone();
        let insight_repo = self.insight_repository.clone();
        let dev_sessions = self.development_sessions.clone();
        let sessions_file_path = self.sessions_file_path.clone();
        let agent_api_mgr = self.agent_api_manager.clone();
        let websocket_mgr = self.websocket_manager.clone();

        // Health and status endpoints
        let status = warp::path("status")
            .and(warp::get())
            .and_then(Self::handle_status);

        let stats = warp::path("stats")
            .and(warp::get())
            .and_then(Self::handle_stats);

        let health = warp::path("health")
            .and(warp::get())
            .and_then(Self::handle_health);

        // Memory operations
        let learn = warp::path("learn")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and_then(Self::handle_learn);

        let memory_query = warp::path("memory")
            .and(warp::path("query"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and_then(Self::handle_memory_query);

        // Chat endpoints
        let chat = warp::path("chat")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and(warp::any().map({
                let insight_repo = insight_repo.clone();
                move || insight_repo.clone()
            }))
            .and_then(Self::handle_chat);

        // Simple chat endpoints
        let simple_chat_learn = warp::path("simple")
            .and(warp::path("learn"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and_then(Self::handle_simple_chat_learn);

        let simple_chat_converse = warp::path("simple")
            .and(warp::path("converse"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and_then(Self::handle_simple_chat_converse);

        // API endpoints (for frontend compatibility)
        let api_chat_learn = warp::path("api")
            .and(warp::path("chat"))
            .and(warp::path("learn"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and_then(Self::handle_simple_chat_learn);

        let api_chat_converse = warp::path("api")
            .and(warp::path("chat"))
            .and(warp::path("converse"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and_then(Self::handle_simple_chat_converse);

        // Static file serving
        let _static_files = warp::fs::dir("web");

        let index = warp::path::end()
            .and(warp::get())
            .map(|| warp::redirect::found(warp::http::Uri::from_static("/chat.html")));

        // Code pattern analysis
        let code_analysis = warp::path("code")
            .and(warp::path("analyze"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and_then(Self::handle_code_pattern_analysis);

        // Enhanced Development context endpoints
        let dev_context_create = warp::path("api")
            .and(warp::path("dev"))
            .and(warp::path("context"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and(warp::any().map({
                let sessions_file_path = sessions_file_path.clone();
                move || sessions_file_path.clone()
            }))
            .and_then(Self::handle_development_context_create);

        let dev_context_get = warp::path("api")
            .and(warp::path("dev"))
            .and(warp::path("context"))
            .and(warp::path::param::<String>())
            .and(warp::get())
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and_then(Self::handle_development_context_get);

        let dev_context_update = warp::path("api")
            .and(warp::path("dev"))
            .and(warp::path("context"))
            .and(warp::path::param::<String>())
            .and(warp::put())
            .and(warp::body::json())
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and(warp::any().map({
                let sessions_file_path = sessions_file_path.clone();
                move || sessions_file_path.clone()
            }))
            .and_then(Self::handle_development_context_update);

        let dev_context_delete = warp::path("api")
            .and(warp::path("dev"))
            .and(warp::path("context"))
            .and(warp::path::param::<String>())
            .and(warp::delete())
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and(warp::any().map({
                let sessions_file_path = sessions_file_path.clone();
                move || sessions_file_path.clone()
            }))
            .and_then(Self::handle_development_context_delete);

        let dev_sessions_list = warp::path("api")
            .and(warp::path("dev"))
            .and(warp::path("sessions"))
            .and(warp::get())
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and_then(Self::handle_development_sessions_list);

        let dev_context_analyze = warp::path("api")
            .and(warp::path("dev"))
            .and(warp::path("context"))
            .and(warp::path("analyze"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and_then(Self::handle_development_context_analyze);

        // Legacy endpoints for backward compatibility
        let legacy_dev_context_create = warp::path("dev")
            .and(warp::path("context"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let memory_repo = memory_repo.clone();
                move || memory_repo.clone()
            }))
            .and(warp::any().map({
                let concept_mgr = concept_mgr.clone();
                move || concept_mgr.clone()
            }))
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and(warp::any().map({
                let sessions_file_path = sessions_file_path.clone();
                move || sessions_file_path.clone()
            }))
            .and_then(Self::handle_development_context_create);

        let legacy_dev_context_get = warp::path("dev")
            .and(warp::path("context"))
            .and(warp::path::param::<String>())
            .and(warp::get())
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and_then(Self::handle_development_context_get);

        // Agent API endpoints
        let agent_list = warp::path("api")
            .and(warp::path("agents"))
            .and(warp::get())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_agent_list);

        let agent_execute = warp::path("api")
            .and(warp::path("agents"))
            .and(warp::path::param::<String>())
            .and(warp::path("execute"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_agent_execute);

        let agent_status = warp::path("api")
            .and(warp::path("agents"))
            .and(warp::path::param::<String>())
            .and(warp::path("status"))
            .and(warp::get())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_agent_status);

        let workflow_execute = warp::path("api")
            .and(warp::path("workflows"))
            .and(warp::path("execute"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_workflow_execute);

        // CPP (Cognitive Preference Profiles) endpoints
        let profile_list = warp::path("api")
            .and(warp::path("profiles"))
            .and(warp::get())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_profile_list);

        let profile_create = warp::path("api")
            .and(warp::path("profiles"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_profile_create);

        let profile_get = warp::path("api")
            .and(warp::path("profiles"))
            .and(warp::path::param::<String>())
            .and(warp::get())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_profile_get);

        let profile_update = warp::path("api")
            .and(warp::path("profiles"))
            .and(warp::path::param::<String>())
            .and(warp::put())
            .and(warp::body::json())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_profile_update);

        let profile_presets = warp::path("api")
            .and(warp::path("profiles"))
            .and(warp::path("presets"))
            .and(warp::get())
            .and(warp::any().map({
                let agent_api_mgr = agent_api_mgr.clone();
                move || agent_api_mgr.clone()
            }))
            .and_then(Self::handle_profile_presets);

        // WebSocket endpoint for real-time agent updates
        let websocket = warp::path("ws")
            .and(warp::ws())
            .and(warp::any().map({
                let websocket_mgr = websocket_mgr.clone();
                move || websocket_mgr.clone()
            }))
            .and_then(Self::handle_websocket);

        // Combine all routes
        let routes = index
            .or(status)
            .or(stats)
            .or(health)
            .or(learn)
            .or(memory_query)
            .or(chat)
            .or(simple_chat_learn)
            .or(simple_chat_converse)
            .or(api_chat_learn)
            .or(api_chat_converse)
            .or(code_analysis)
            .or(dev_context_create)
            .or(dev_context_get)
            .or(dev_context_update)
            .or(dev_context_delete)
            .or(dev_sessions_list)
            .or(dev_context_analyze)
            .or(legacy_dev_context_create)
            .or(legacy_dev_context_get)
            .or(agent_list)
            .or(agent_execute)
            .or(agent_status)
            .or(workflow_execute)
            .or(profile_list)
            .or(profile_create)
            .or(profile_get)
            .or(profile_update)
            .or(profile_presets)
            .or(websocket)
            .with(cors);

        println!("🧠 Brain AI Web Server starting on port {}", self.port);
        println!("📊 Development Context API endpoints:");
        println!("  POST /api/dev/context - Create/update development session");
        println!("  GET  /api/dev/context/{{id}} - Get development session");
        println!("  PUT  /api/dev/context/{{id}} - Update development session");
        println!("  DELETE /api/dev/context/{{id}} - Delete development session");
        println!("  GET  /api/dev/sessions - List all sessions");
        println!("  POST /api/dev/context/analyze - Analyze development patterns");
        println!("🤖 Agent API endpoints:");
        println!("  GET  /api/agents - List all available agents");
        println!("  POST /api/agents/{{agent_name}}/execute - Execute specific agent");
        println!("  GET  /api/agents/{{agent_name}}/status - Get agent status");
        println!("  POST /api/workflows/execute - Execute multi-agent workflow");
        println!("🔄 WebSocket endpoints:");
        println!("  WS   /ws - Real-time agent updates and monitoring");
        
        warp::serve(routes)
            .run(([127, 0, 0, 1], self.port))
            .await;

        Ok(())
    }

    // Handler implementations
    async fn handle_status() -> std::result::Result<impl Reply, warp::Rejection> {
        let response = StatusResponse {
            status: "healthy".to_string(),
            uptime: "running".to_string(),
            version: "0.8.0".to_string(),
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_stats() -> std::result::Result<impl Reply, warp::Rejection> {
        let response = StatsResponse {
            memory_usage: "128MB".to_string(),
            confidence: 0.85,
            active_processes: 1,
            response_time: 50,
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_health() -> std::result::Result<impl Reply, warp::Rejection> {
        let response = HealthResponse {
            system_status: "operational".to_string(),
            memory_efficiency: "high".to_string(),
            processing_speed: "optimal".to_string(),
            active_connections: 0,
            uptime: "running".to_string(),
            last_backup: "recent".to_string(),
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_learn(
        request: ProcessRequest,
        memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // Process learning request
        let start_time = std::time::Instant::now();
        
        // Check if this is a GitHub URL
        let text_trimmed = request.text.trim();
        let is_github_url = request.is_github_url || 
            (text_trimmed.contains("github.com/") && 
             (text_trimmed.starts_with("https://github.com/") || 
              text_trimmed.starts_with("http://github.com/") ||
              text_trimmed.starts_with("github.com/")));
        
        if is_github_url {
            // Handle GitHub repository learning
            use brain_infra::{GitHubLearningEngine, GitHubLearningConfig};
            use std::env;
            
            let github_token = env::var("GITHUB_TOKEN").ok();
            let config = GitHubLearningConfig::default();
            let github_engine = GitHubLearningEngine::new(github_token, Some(config));
            
            let mut repo = memory_repo.lock().await;
            
            // Ensure URL has proper https:// prefix
            let github_url = if text_trimmed.starts_with("http") {
                text_trimmed.to_string()
            } else {
                format!("https://{}", text_trimmed)
            };
            
            match github_engine.learn_from_repository(&mut *repo, &github_url).await {
                Ok(learning_result) => {
                    let processing_time = start_time.elapsed().as_millis() as u64;
                    
                    let response = ProcessResponse {
                        success: true,
                        message: format!("Successfully learned from GitHub repository: {}", learning_result.repository),
                        data: Some(serde_json::json!({
                            "repository": learning_result.repository,
                            "files_processed": learning_result.files_processed,
                            "total_content_size": learning_result.total_content_size,
                            "memory_entries_created": learning_result.memory_entries_created,
                            "key_insights": learning_result.key_insights,
                            "summary": learning_result.summary
                        })),
                        processing_time,
                    };
                    
                    Ok(warp::reply::json(&response))
                }
                Err(e) => {
                    let processing_time = start_time.elapsed().as_millis() as u64;
                    
                    let response = ProcessResponse {
                        success: false,
                        message: format!("Failed to learn from GitHub repository: {}", e),
                        data: Some(serde_json::json!({"error": e.to_string()})),
                        processing_time,
                    };
                    
                    Ok(warp::reply::json(&response))
                }
            }
        } else {
            // Handle regular text learning
            let mut repo = memory_repo.lock().await;
            
            // Create a working memory item from the text
            let item = WorkingMemoryItem {
                id: uuid::Uuid::new_v4(),
                content: request.text.clone(),
                priority: Priority::Medium,
                created_at: chrono::Utc::now(),
                last_accessed: chrono::Utc::now(),
                access_count: 0,
                decay_factor: 1.0,
            };

            repo.store_item(item).await.map_err(|_| warp::reject())?;

            let processing_time = start_time.elapsed().as_millis() as u64;
            
            let response = ProcessResponse {
                success: true,
                message: "Content learned successfully".to_string(),
                data: Some(serde_json::json!({"text_length": request.text.len()})),
                processing_time,
            };

            Ok(warp::reply::json(&response))
        }
    }

    async fn handle_memory_query(
        request: QueryRequest,
        memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let repo = memory_repo.lock().await;
        
        // Create a working memory query
        let query = WorkingMemoryQuery {
            content_pattern: Some(request.query.clone()),
            priority: None,
            min_importance: Some(0.5),
            created_after: None,
            limit: Some(10),
        };

        let results = repo.query_items(&query).await.map_err(|_| warp::reject())?;
        
        Ok(warp::reply::json(&results))
    }

    async fn handle_chat(
        request: ChatRequest,
        _memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
        _concept_mgr: Arc<Mutex<ConceptGraphManager>>,
        _insight_repo: Arc<Mutex<InMemoryInsightRepository>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // Generate a simple response (placeholder for sophisticated chat logic)
        let response = ChatResponse {
            response: format!("I understand you said: '{}'", request.message),
            context_used: true,
            suggestions: vec![
                "Tell me more about that".to_string(),
                "Can you explain further?".to_string(),
            ],
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_simple_chat_learn(
        request: SimpleChatLearnRequest,
        memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
        _concept_mgr: Arc<Mutex<ConceptGraphManager>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // GitHub URL learning handler
        // Check if this is a GitHub URL
        let content_trimmed = request.content.trim();
        let is_github_url = content_trimmed.contains("github.com/") && 
            (content_trimmed.starts_with("https://github.com/") || 
             content_trimmed.starts_with("http://github.com/") ||
             content_trimmed.starts_with("github.com/"));
        
        // GitHub URL detection and learning logic
        
        if is_github_url {
            // Handle GitHub repository learning
            use brain_infra::{GitHubLearningEngine, GitHubLearningConfig};
            use std::env;
            
            let github_token = env::var("GITHUB_TOKEN").ok();
            let config = GitHubLearningConfig::default();
            let github_engine = GitHubLearningEngine::new(github_token, Some(config));
            
            let mut repo = memory_repo.lock().await;
            
            // Ensure URL has proper https:// prefix
            let github_url = if content_trimmed.starts_with("http") {
                content_trimmed.to_string()
            } else {
                format!("https://{}", content_trimmed)
            };
            
            match github_engine.learn_from_repository(&mut *repo, &github_url).await {
                Ok(learning_result) => {
                    let response = SimpleChatResponse {
                        response: format!("🎉 Successfully learned from GitHub repository: {}!\n\n📊 Processed {} files with {} total characters of content.\n\n🧠 Created {} memory entries and discovered key insights about the repository.", 
                            learning_result.repository,
                            learning_result.files_processed,
                            learning_result.total_content_size,
                            learning_result.memory_entries_created
                        ),
                        insights_learned: learning_result.key_insights,
                        context_used: true,
                    };
                    
                    Ok(warp::reply::json(&response))
                }
                Err(e) => {
                    let response = SimpleChatResponse {
                        response: format!("❌ Failed to learn from GitHub repository: {}\n\nTip: Make sure the repository URL is valid and accessible.", e),
                        insights_learned: vec!["GitHub learning failed".to_string()],
                        context_used: false,
                    };
                    
                    Ok(warp::reply::json(&response))
                }
            }
        } else {
            // Handle regular text learning
            let mut repo = memory_repo.lock().await;
            
            // Store the learning content
            let item = WorkingMemoryItem {
                id: uuid::Uuid::new_v4(),
                content: request.content.clone(),
                priority: Priority::Medium,
                created_at: chrono::Utc::now(),
                last_accessed: chrono::Utc::now(),
                access_count: 0,
                decay_factor: 1.0,
            };

            repo.store_item(item).await.map_err(|_| warp::reject())?;

            let response = SimpleChatResponse {
                response: "Content learned successfully".to_string(),
                insights_learned: vec!["New information stored".to_string()],
                context_used: false,
            };

            Ok(warp::reply::json(&response))
        }
    }

    async fn handle_simple_chat_converse(
        request: SimpleChatConverseRequest,
        memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
        _concept_mgr: Arc<Mutex<ConceptGraphManager>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // Handle conversation with automatic GitHub learning
        let message = request.message.to_lowercase();
        
        // Check if the message contains a GitHub URL and learn from it automatically
        
        if request.message.contains("github.com/") && 
           (request.message.contains("https://") || request.message.contains("http://")) {
            // Extract GitHub URL from the message
            if let Some(github_url) = extract_github_url(&request.message) {
                // Automatically learn from the GitHub repository
                use brain_infra::{GitHubLearningEngine, GitHubLearningConfig};
                use std::env;
                
                let github_token = env::var("GITHUB_TOKEN").ok();
                let config = GitHubLearningConfig::default();
                let github_engine = GitHubLearningEngine::new(github_token, Some(config));
                
                let mut repo = memory_repo.lock().await;
                
                // Try to learn from GitHub repository
                match github_engine.learn_from_repository(&mut *repo, &github_url).await {
                                         Ok(learning_result) => {
                         // Successfully learned from GitHub
                         let response = SimpleChatResponse {
                            response: format!("🎉 I've automatically learned from the GitHub repository: {}!\n\n📊 I processed {} files and created {} memory entries with key insights:\n\n{}\n\nNow I can answer questions about this repository. What would you like to know?", 
                                learning_result.repository,
                                learning_result.files_processed,
                                learning_result.memory_entries_created,
                                learning_result.key_insights.join("\n• ")
                            ),
                            insights_learned: learning_result.key_insights,
                            context_used: true,
                        };
                        
                        return Ok(warp::reply::json(&response));
                    }
                                         Err(_e) => {
                         // GitHub learning failed, but continue with regular conversation
                    }
                }
                
                drop(repo);
            }
        }
        
        // First, try to find relevant content in memory
        let memory_repo_lock = memory_repo.lock().await;
        
        // Check if user is asking about something specific we might have learned
        let mut found_content = Vec::new();
        
        // Try to extract key terms from the message for memory search
        let search_terms = if message.contains("what") && (message.contains("learn") || message.contains("know")) {
            // Extract what they're asking about - more flexible search
            if let Some(start) = message.find("know about") {
                let topic = message[start + 10..].trim().replace("?", "").to_lowercase();
                if !topic.is_empty() {
                    vec![topic]
                } else { vec![] }
            } else if let Some(start) = message.find("learn about") {
                let topic = message[start + 11..].trim().replace("?", "").to_lowercase();
                if !topic.is_empty() {
                    vec![topic]
                } else { vec![] }
            } else {
                // Try to extract any significant words from the question
                let words: Vec<String> = message.split_whitespace()
                    .filter(|w| w.len() > 3 && !["what", "learn", "know", "about", "tell", "from", "that", "this", "with", "have", "been"].contains(&w.to_lowercase().as_str()))
                    .map(|w| w.replace("?", "").to_lowercase())
                    .collect();
                words
            }
        } else if message.contains("tell me about") {
            // Extract the topic after "tell me about"
            if let Some(start) = message.find("tell me about") {
                let topic = message[start + 13..].trim().replace("?", "").to_lowercase();
                if !topic.is_empty() {
                    vec![topic]
                } else { vec![] }
            } else { vec![] }
        } else {
            // For any other message, try to extract meaningful words for search
            let words: Vec<String> = message.split_whitespace()
                .filter(|w| w.len() > 4 && !["what", "learn", "know", "about", "tell", "from", "that", "this", "with", "have", "been", "would", "could", "should"].contains(&w.to_lowercase().as_str()))
                .map(|w| w.replace("?", "").replace("!", "").to_lowercase())
                .take(3) // Limit to 3 terms to avoid too broad search
                .collect();
            words
        };
        
        // Search memory for relevant content
        let mut unique_content = std::collections::HashSet::new();
        for term in &search_terms {
            let query = WorkingMemoryQuery {
                content_pattern: Some(term.clone()),
                priority: None,
                min_importance: Some(0.1),
                created_after: None,
                limit: Some(5),
            };
            
            if let Ok(results) = memory_repo_lock.query_items(&query).await {
                for item in results {
                    // Only add unique content to avoid duplication
                    if unique_content.insert(item.content.clone()) {
                        found_content.push(item.content);
                    }
                }
            }
        }
        
        drop(memory_repo_lock);
        
        // Generate intelligent responses based on message content and memory
        let memory_repo_lock = memory_repo.lock().await;
        let stats = memory_repo_lock.stats().await.unwrap_or_else(|_| {
            use brain_core::memory::MemoryStats;
            use chrono::Utc;
            MemoryStats {
                total_items: 0,
                size_bytes: 0,
                last_access: Utc::now(),
                access_count: 0,
                consolidation_count: 0,
            }
        });
        drop(memory_repo_lock);
        
        let response_text = if !found_content.is_empty() {
            // We found relevant content in memory, provide intelligent response
            let content_summary = found_content.join("\n\n");
            format!("Based on what I've learned:\n\n{}\n\nI can tell you more about any specific aspect that interests you!", content_summary)
        } else if message.contains("what") && (message.contains("learn") || message.contains("remember")) && stats.total_items > 0 {
            // User is asking what we learned and we have content
            format!("I have {} items in my memory system. Could you be more specific about what you'd like me to recall? Try asking about a specific topic or concept.", stats.total_items)
        } else if stats.total_items == 0 && (message.contains("learn") || message.contains("teach") || message.contains("remember")) {
            "I don't have any learned content yet. Share something with me and I'll analyze and remember it!".to_string()
        } else {
            // Generate more dynamic, varied responses
            let responses = vec![
                format!("That's an interesting question! I currently have {} items in my memory. What specific topic would you like to explore?", stats.total_items),
                format!("I'm ready to help! With {} items in my knowledge base, I can discuss various topics. What are you curious about?", stats.total_items),
                format!("Great question! I've learned {} different things so far. What would you like to know more about?", stats.total_items),
                "I'm here to chat and learn! What topic interests you most right now?".to_string(),
                "I'm ready for a conversation! What would you like to discuss or teach me about?".to_string(),
                "Interesting! What specific aspect of that topic would you like to explore?".to_string(),
            ];
            
            // Use the length of the message to pick a response (pseudo-random but consistent)
            let index = message.len() % responses.len();
            responses[index].clone()
        };

        let response = SimpleChatResponse {
            response: response_text,
            insights_learned: if !found_content.is_empty() { 
                vec!["Found relevant content in memory".to_string()] 
            } else { 
                vec!["Memory available for learning".to_string()] 
            },
            context_used: !found_content.is_empty(),
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_code_pattern_analysis(
        request: CodePatternAnalysisRequest,
        _memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
        _concept_mgr: Arc<Mutex<ConceptGraphManager>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        // Analyze code patterns (simplified implementation)
        let patterns = vec![
            CodePattern {
                pattern_type: CodePatternType::Function,
                name: "main_function".to_string(),
                description: "Main entry point function".to_string(),
                code_snippet: Some("fn main() { ... }".to_string()),
                file_location: request.file_path.clone(),
                confidence: 0.9,
                related_patterns: vec![],
                concept_id: Some(uuid::Uuid::new_v4().to_string()),
            }
        ];

        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = CodePatternAnalysisResponse {
            success: true,
            patterns_found: patterns,
            concepts_created: 1,
            relationships_formed: 0,
            analysis_time_ms: processing_time,
            confidence_score: 0.85,
            language_detected: request.language.or_else(|| Some("rust".to_string())),
            architectural_insights: vec!["Standard Rust application structure".to_string()],
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_development_context_create(
        request: DevelopmentContextRequest,
        _memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
        _concept_mgr: Arc<Mutex<ConceptGraphManager>>,
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
        sessions_file_path: PathBuf,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        let session_id = request.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        
        // Recognize intent from file access patterns
        let intent_recognized = Self::recognize_intent(&request.files_accessed, &request.project_context);
        
        let session = DevelopmentSession {
            session_id: session_id.clone(),
            start_time: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            files_accessed: request.files_accessed.clone(),
            development_intent: request.current_intent.or_else(|| {
                intent_recognized.as_ref().map(|i| format!("{:?}", i))
            }),
            development_goal: request.development_goal,
            project_context: request.project_context,
            insights: vec!["Session created".to_string()],
            patterns_discovered: vec![],
            confidence_score: 0.7,
            session_tags: vec![],
            focus_areas: vec![],
            productivity_metrics: ProductivityMetrics::default(),
        };

        // Generate insights and recommendations
        let insights = Self::generate_insights(&session);
        let recommendations = Self::generate_recommendations(&session);
        let patterns_detected = vec!["Initial session pattern".to_string()];

        let mut sessions_map = sessions.lock().await;
        sessions_map.insert(session_id.clone(), session);
        drop(sessions_map);

        // Save sessions to file if auto_save is enabled
        if request.auto_save {
            if let Err(e) = Self::save_sessions_to_file(&sessions, &sessions_file_path).await {
                eprintln!("Failed to save sessions: {}", e);
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = DevelopmentContextResponse {
            success: true,
            session_id,
            context_preserved: true,
            insights_generated: insights,
            recommendations,
            processing_time_ms: processing_time,
            intent_recognized,
            patterns_detected,
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_development_context_get(
        session_id: String,
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        let sessions_map = sessions.lock().await;
        
        let session = sessions_map.get(&session_id).cloned();
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let recommendations = if let Some(ref session) = session {
            Self::generate_recommendations(session)
        } else {
            vec![]
        };
        
        let response = DevelopmentContextQueryResponse {
            success: true,
            session_found: session.is_some(),
            session,
            related_sessions: vec![],
            context_summary: Some("Development session found".to_string()),
            processing_time_ms: processing_time,
            recommendations,
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_development_context_update(
        session_id: String,
        request: SessionUpdateRequest,
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
        sessions_file_path: PathBuf,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        let mut sessions_map = sessions.lock().await;
        
        let mut success = false;
        let mut intent_recognized = None;
        let mut insights_generated = vec![];
        let mut recommendations = vec![];

        if let Some(session) = sessions_map.get_mut(&session_id) {
            // Update session fields
            if let Some(intent) = request.development_intent {
                session.development_intent = Some(intent);
            }
            if let Some(goal) = request.development_goal {
                session.development_goal = Some(goal);
            }
            if let Some(tags) = request.tags {
                session.session_tags = tags;
            }
            if let Some(additional_files) = request.additional_files {
                session.files_accessed.extend(additional_files);
                intent_recognized = Self::recognize_intent(&session.files_accessed, &session.project_context);
            }
            if let Some(project_context) = request.project_context {
                session.project_context = Some(project_context);
            }
            
            session.last_updated = chrono::Utc::now();
            
            // Generate new insights and recommendations
            insights_generated = Self::generate_insights(session);
            recommendations = Self::generate_recommendations(session);
            success = true;
        }
        
        drop(sessions_map);

        // Save sessions to file
        if let Err(e) = Self::save_sessions_to_file(&sessions, &sessions_file_path).await {
            eprintln!("Failed to save sessions: {}", e);
        }

        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = DevelopmentContextResponse {
            success,
            session_id,
            context_preserved: success,
            insights_generated,
            recommendations,
            processing_time_ms: processing_time,
            intent_recognized,
            patterns_detected: vec!["Session updated".to_string()],
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_development_context_delete(
        session_id: String,
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
        sessions_file_path: PathBuf,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        let mut sessions_map = sessions.lock().await;
        
        let removed = sessions_map.remove(&session_id).is_some();
        drop(sessions_map);

        // Save sessions to file
        if let Err(e) = Self::save_sessions_to_file(&sessions, &sessions_file_path).await {
            eprintln!("Failed to save sessions: {}", e);
        }

        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = DevelopmentContextResponse {
            success: removed,
            session_id,
            context_preserved: false,
            insights_generated: vec!["Session deleted".to_string()],
            recommendations: vec![],
            processing_time_ms: processing_time,
            intent_recognized: None,
            patterns_detected: vec![],
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_development_sessions_list(
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        let sessions_map = sessions.lock().await;
        
        let session_summaries: Vec<SessionSummary> = sessions_map.iter()
            .map(|(id, session)| SessionSummary {
                session_id: id.clone(),
                start_time: session.start_time,
                last_updated: session.last_updated,
                files_count: session.files_accessed.len(),
                intent: session.development_intent.clone(),
                tags: session.session_tags.clone(),
                duration_minutes: (session.last_updated - session.start_time).num_minutes() as u32,
            })
            .collect();
        
        let response = SessionListResponse {
            success: true,
            sessions: session_summaries,
            total_count: sessions_map.len(),
            active_sessions: sessions_map.len(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        };

        Ok(warp::reply::json(&response))
    }

    async fn handle_development_context_analyze(
        request: ContextAnalysisRequest,
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        let sessions_map = sessions.lock().await;
        
        // Analyze all sessions or filter by project root if specified
        let relevant_sessions: Vec<&DevelopmentSession> = sessions_map.values()
            .filter(|session| {
                if let Some(ref project_root) = request.project_root {
                    session.project_context.as_ref()
                        .map_or(false, |ctx| ctx.project_root == *project_root)
                } else {
                    true
                }
            })
            .collect();

        // Generate development patterns
        let mut development_patterns = vec![];
        let mut productivity_insights = vec![];
        let mut recommendations = vec![];
        let mut focus_areas = vec![];

        if !relevant_sessions.is_empty() {
            // Analyze file type patterns
            let mut file_types = std::collections::HashMap::new();
            for session in &relevant_sessions {
                for file_access in &session.files_accessed {
                    if let Some(ext) = std::path::Path::new(&file_access.file_path).extension() {
                        if let Some(ext_str) = ext.to_str() {
                            *file_types.entry(ext_str.to_string()).or_insert(0) += 1;
                        }
                    }
                }
            }

            for (file_type, count) in file_types {
                development_patterns.push(DevelopmentPattern {
                    pattern_type: "FileType".to_string(),
                    description: format!("Frequent {} file access", file_type),
                    frequency: count,
                    confidence: 0.8,
                    impact: "Medium".to_string(),
                });
            }

            // Generate productivity insights
            let total_files: usize = relevant_sessions.iter()
                .map(|s| s.files_accessed.len())
                .sum();
            
            productivity_insights.push(format!("Analyzed {} sessions with {} total file accesses", 
                relevant_sessions.len(), total_files));

            if total_files > 50 {
                productivity_insights.push("High file activity detected across sessions".to_string());
                recommendations.push("Consider organizing files better for easier navigation".to_string());
            }

            // Analyze session durations
            let avg_duration: i64 = relevant_sessions.iter()
                .map(|s| (s.last_updated - s.start_time).num_minutes())
                .sum::<i64>() / relevant_sessions.len() as i64;
            
            if avg_duration > 240 { // More than 4 hours
                productivity_insights.push("Long development sessions detected".to_string());
                recommendations.push("Consider taking more breaks for better productivity".to_string());
            }

            // Identify focus areas
            let mut intents = std::collections::HashMap::new();
            for session in &relevant_sessions {
                if let Some(ref intent) = session.development_intent {
                    *intents.entry(intent.clone()).or_insert(0) += 1;
                }
            }

            for (intent, count) in intents {
                if count > 1 {
                    focus_areas.push(format!("{} ({}x)", intent, count));
                }
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = ContextAnalysisResponse {
            success: true,
            analysis_summary: format!("Analyzed {} development sessions", relevant_sessions.len()),
            development_patterns,
            productivity_insights,
            recommendations,
            focus_areas,
            processing_time_ms: processing_time,
        };

        Ok(warp::reply::json(&response))
    }

    // Agent API handlers
    async fn handle_agent_list(
        agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        match agent_api_mgr.list_agents().await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(e) => {
                eprintln!("Error listing agents: {}", e);
                let error_response = serde_json::json!({
                    "success": false,
                    "error": e.to_string(),
                    "agents": [],
                    "total_count": 0,
                    "categories": {}
                });
                Ok(warp::reply::json(&error_response))
            }
        }
    }

    async fn handle_agent_execute(
        agent_name: String,
        request: AgentExecutionRequest,
        agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        match agent_api_mgr.execute_agent(&agent_name, request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(e) => {
                eprintln!("Error executing agent {}: {}", agent_name, e);
                let error_response = serde_json::json!({
                    "success": false,
                    "error": e.to_string(),
                    "execution_id": uuid::Uuid::new_v4().to_string(),
                    "agent_name": agent_name,
                    "execution_time_ms": 0
                });
                Ok(warp::reply::json(&error_response))
            }
        }
    }

    async fn handle_agent_status(
        agent_name: String,
        agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        match agent_api_mgr.get_agent_status(&agent_name).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(e) => {
                eprintln!("Error getting agent status for {}: {}", agent_name, e);
                let error_response = serde_json::json!({
                    "success": false,
                    "error": e.to_string(),
                    "agent_name": agent_name,
                    "status": "error"
                });
                Ok(warp::reply::json(&error_response))
            }
        }
    }

    async fn handle_workflow_execute(
        request: WorkflowExecutionRequest,
        agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        match agent_api_mgr.execute_workflow(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(e) => {
                eprintln!("Error executing workflow: {}", e);
                let error_response = serde_json::json!({
                    "success": false,
                    "error": e.to_string(),
                    "workflow_id": uuid::Uuid::new_v4().to_string(),
                    "total_execution_time_ms": 0
                });
                Ok(warp::reply::json(&error_response))
            }
        }
    }

    // CPP (Cognitive Preference Profile) handlers
    async fn handle_profile_list(
        _agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // For now, return a basic response indicating CPP functionality is available
        // This would be extended with proper user ID handling and profile management
        let response = serde_json::json!({
            "success": true,
            "profiles": [],
            "total_count": 0,
            "message": "CPP profile management available. Provide user_id parameter for specific profiles."
        });
        Ok(warp::reply::json(&response))
    }

    async fn handle_profile_create(
        request: crate::agents::CreateProfileRequest,
        _agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // For now, return a basic success response
        // This would be extended with proper CPP integration once the AgentApiManager methods are working
        let response = serde_json::json!({
            "success": true,
            "profile_id": request.user_id,
            "profile_name": request.name,
            "user_id": request.user_id,
            "preferences": request.preferences,
            "created_at": chrono::Utc::now(),
            "updated_at": chrono::Utc::now(),
            "active": true,
            "message": "Profile creation acknowledged. Full CPP integration pending."
        });
        Ok(warp::reply::json(&response))
    }

    async fn handle_profile_get(
        user_id: String,
        _agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // For now, return a default profile response
        let response = serde_json::json!({
            "success": true,
            "profile_id": user_id,
            "profile_name": format!("{}_profile", user_id),
            "user_id": user_id,
            "preferences": {
                "interaction_mode": "focused",
                "verbosity_level": "detailed",
                "communication_tone": "adaptive",
                "autonomy_boundaries": {
                    "decision_autonomy_level": "semi_auto",
                    "confirmation_required": [],
                    "auto_execute_threshold": 0.8
                },
                "cognitive_load_management": {
                    "chunking_enabled": true,
                    "progressive_disclosure": true,
                    "complexity_threshold": 0.6
                },
                "emotional_sensitivity": "medium"
            },
            "created_at": chrono::Utc::now(),
            "updated_at": chrono::Utc::now(),
            "active": true,
            "message": "Default profile returned. Full CPP integration pending."
        });
        Ok(warp::reply::json(&response))
    }

    async fn handle_profile_update(
        user_id: String,
        request: crate::agents::CreateProfileRequest,
        _agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // For now, return a basic update response
        let response = serde_json::json!({
            "success": true,
            "profile_id": user_id,
            "profile_name": request.name,
            "user_id": user_id,
            "preferences": request.preferences,
            "created_at": chrono::Utc::now(),
            "updated_at": chrono::Utc::now(),
            "active": true,
            "message": "Profile update acknowledged. Full CPP integration pending."
        });
        Ok(warp::reply::json(&response))
    }

    async fn handle_profile_presets(
        _agent_api_mgr: Arc<AgentApiManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // Return some default presets for demonstration
        let response = serde_json::json!({
            "success": true,
            "presets": [
                {
                    "id": "developer_focused",
                    "name": "Developer - Focused",
                    "description": "Optimized for focused development work with minimal distractions",
                    "target_persona": "Experienced developers working on complex projects",
                    "tags": ["developer", "focused", "technical"],
                    "popularity_score": 0.88,
                    "profile": {
                        "interaction_mode": "focused",
                        "verbosity_level": "detailed",
                        "communication_tone": "technical",
                        "autonomy_boundaries": {
                            "decision_autonomy_level": "semi_auto",
                            "confirmation_required": [],
                            "auto_execute_threshold": 0.8
                        },
                        "cognitive_load_management": {
                            "chunking_enabled": false,
                            "progressive_disclosure": false,
                            "complexity_threshold": 0.8
                        },
                        "emotional_sensitivity": "low"
                    }
                },
                {
                    "id": "beginner_guided",
                    "name": "Beginner - Guided",
                    "description": "Perfect for newcomers who need step-by-step guidance",
                    "target_persona": "New users learning development concepts",
                    "tags": ["beginner", "guided", "learning"],
                    "popularity_score": 0.85,
                    "profile": {
                        "interaction_mode": "collaborative",
                        "verbosity_level": "comprehensive",
                        "communication_tone": "casual",
                        "autonomy_boundaries": {
                            "decision_autonomy_level": "manual",
                            "confirmation_required": ["all"],
                            "auto_execute_threshold": 0.3
                        },
                        "cognitive_load_management": {
                            "chunking_enabled": true,
                            "progressive_disclosure": true,
                            "complexity_threshold": 0.3
                        },
                        "emotional_sensitivity": "high"
                    }
                }
            ],
            "total_count": 2,
            "message": "Default presets returned. Full CPP integration pending."
        });
        Ok(warp::reply::json(&response))
    }

    /// Handle WebSocket connections for real-time agent updates
    async fn handle_websocket(
        ws: warp::ws::Ws,
        websocket_mgr: Arc<WebSocketManager>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        Ok(ws.on_upgrade(move |socket| {
            let websocket_mgr = websocket_mgr.clone();
            async move {
                let _client_id = websocket_mgr.add_client(socket).await;
                // Client management is handled within add_client
            }
        }))
    }

    /// Helper method to save sessions to file
    async fn save_sessions_to_file(
        sessions: &Arc<Mutex<HashMap<String, DevelopmentSession>>>,
        sessions_file_path: &PathBuf,
    ) -> Result<()> {
        let sessions_map = sessions.lock().await;
        let content = serde_json::to_string_pretty(&*sessions_map)
            .map_err(|e| BrainError::Serialization { source: Box::new(e) })?;
        
        fs::write(sessions_file_path, content)
            .map_err(|e| BrainError::Io { source: e })?;
        
        Ok(())
    }
}

/// Start the web server on the specified port
pub async fn start_web_server(port: u16) -> Result<()> {
    let server = WebServer::new(port).await?;
    server.start().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_web_server_creation() {
        let server = WebServer::new(3030).await;
        assert!(server.is_ok());
    }

    #[test]
    fn test_process_request_serialization() {
        let request = ProcessRequest {
            text: "test content".to_string(),
            is_github_url: false,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test content"));
    }

    #[test]
    fn test_chat_message_creation() {
        let message = ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };
        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Hello");
    }

    #[test]
    fn test_development_session_creation() {
        let session = DevelopmentSession {
            session_id: "test-123".to_string(),
            start_time: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            files_accessed: vec![],
            development_intent: None,
            development_goal: None,
            project_context: None,
            insights: vec![],
            patterns_discovered: vec![],
            confidence_score: 0.8,
            session_tags: vec![],
            focus_areas: vec![],
            productivity_metrics: ProductivityMetrics::default(),
        };
        assert_eq!(session.session_id, "test-123");
        assert_eq!(session.confidence_score, 0.8);
    }

    #[test]
    fn test_code_pattern_types() {
        let pattern = CodePattern {
            pattern_type: CodePatternType::Function,
            name: "test_fn".to_string(),
            description: "Test function".to_string(),
            code_snippet: None,
            file_location: None,
            confidence: 0.9,
            related_patterns: vec![],
            concept_id: None,
        };
        assert!(matches!(pattern.pattern_type, CodePatternType::Function));
    }
} 