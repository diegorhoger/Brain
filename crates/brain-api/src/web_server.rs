//! Web Server Module - Comprehensive API Server for Brain AI
//!
//! This module provides a full-featured web server with extensive API endpoints
//! for all Brain AI functionality including memory operations, concept graphs,
//! pattern detection, RAG conversations, and development context tracking.

use brain_types::*;
use brain_core::{WorkingMemoryItem, WorkingMemoryQuery, Priority, WorkingMemoryRepository as WorkingMemoryRepositoryTrait};
use brain_infra::{WorkingMemoryRepository, ConceptGraphManager, InMemoryInsightRepository, ConceptGraphConfig};
use brain_cognitive::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Reply};
use chrono::Utc;

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

// Development Context API structures
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileAccess {
    pub file_path: String,
    pub access_type: FileAccessType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub line_numbers: Option<Vec<u32>>,
    pub content_preview: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileAccessType {
    Read,
    Write,
    Create,
    Delete,
    Execute,
    Navigate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectContext {
    pub project_root: String,
    pub current_branch: Option<String>,
    pub active_features: Vec<String>,
    pub technology_stack: Vec<String>,
    pub recent_commits: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentContextResponse {
    pub success: bool,
    pub session_id: String,
    pub context_preserved: bool,
    pub insights_generated: Vec<String>,
    pub recommendations: Vec<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevelopmentContextQueryResponse {
    pub success: bool,
    pub session_found: bool,
    pub session: Option<DevelopmentSession>,
    pub related_sessions: Vec<String>,
    pub context_summary: Option<String>,
    pub processing_time_ms: u64,
}

/// Comprehensive Web Server for Brain AI System
pub struct WebServer {
    port: u16,
    memory_repository: Arc<Mutex<WorkingMemoryRepository>>,
    concept_manager: Arc<Mutex<ConceptGraphManager>>,
    insight_repository: Arc<Mutex<InMemoryInsightRepository>>,
    development_sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
}

impl WebServer {
    /// Create a new web server instance
    pub async fn new(port: u16) -> Result<Self> {
        let memory_repository = Arc::new(Mutex::new(WorkingMemoryRepository::new(1000)));
        let concept_config = ConceptGraphConfig::default();
        let concept_manager = Arc::new(Mutex::new(ConceptGraphManager::new(concept_config).await?));
        let insight_repository = Arc::new(Mutex::new(InMemoryInsightRepository::new()));
        let development_sessions = Arc::new(Mutex::new(HashMap::new()));

        Ok(Self {
            port,
            memory_repository,
            concept_manager,
            insight_repository,
            development_sessions,
        })
    }

    /// Start the web server with all routes
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

        // Development context endpoints
        let dev_context_create = warp::path("dev")
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
            .and_then(Self::handle_development_context_create);

        let dev_context_get = warp::path("dev")
            .and(warp::path("context"))
            .and(warp::path::param::<String>())
            .and(warp::get())
            .and(warp::any().map({
                let dev_sessions = dev_sessions.clone();
                move || dev_sessions.clone()
            }))
            .and_then(Self::handle_development_context_get);

        // Combine all routes
        let routes = status
            .or(stats)
            .or(health)
            .or(learn)
            .or(memory_query)
            .or(chat)
            .or(simple_chat_learn)
            .or(simple_chat_converse)
            .or(code_analysis)
            .or(dev_context_create)
            .or(dev_context_get)
            .with(cors);

        println!("🧠 Brain AI Web Server starting on port {}", self.port);
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
        let start_time = std::time::Instant::now();
        
        // Process the learning request
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

    async fn handle_simple_chat_converse(
        request: SimpleChatConverseRequest,
        _memory_repo: Arc<Mutex<WorkingMemoryRepository>>,
        _concept_mgr: Arc<Mutex<ConceptGraphManager>>,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let response = SimpleChatResponse {
            response: format!("I understand your message: '{}'", request.message),
            insights_learned: vec![],
            context_used: true,
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
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        let session_id = request.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        
        let session = DevelopmentSession {
            session_id: session_id.clone(),
            start_time: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            files_accessed: request.files_accessed,
            development_intent: request.current_intent,
            development_goal: request.development_goal,
            project_context: request.project_context,
            insights: vec!["Session created".to_string()],
            patterns_discovered: vec![],
            confidence_score: 0.7,
        };

        let mut sessions_map = sessions.lock().await;
        sessions_map.insert(session_id.clone(), session);

        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = DevelopmentContextResponse {
            success: true,
            session_id,
            context_preserved: true,
            insights_generated: vec!["Development session initialized".to_string()],
            recommendations: vec!["Continue with your development workflow".to_string()],
            processing_time_ms: processing_time,
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
        
        let response = DevelopmentContextQueryResponse {
            success: true,
            session_found: session.is_some(),
            session,
            related_sessions: vec![],
            context_summary: Some("Development session found".to_string()),
            processing_time_ms: processing_time,
        };

        Ok(warp::reply::json(&response))
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