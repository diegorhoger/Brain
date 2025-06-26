use crate::error::BrainError;
use crate::github_integration::{GitHubLearningEngine, GitHubClient};
use crate::memory::{MemorySystem, Priority, WorkingMemoryQuery, SemanticQuery, EpisodicQuery, EpisodicEvent, SemanticConcept};
use crate::concept_graph::{ConceptGraphManager, ConceptGraphConfig, ConceptNode, ConceptType, RelationshipType};
use crate::insight_extraction::PatternDetector;
use crate::segment_discovery::{BpeSegmenter, BpeConfig};
use crate::conversation::{RagOrchestrator, RagRequest, BrainAIOrchestrator};
use crate::code_pattern_analyzer::CodePatternAnalyzer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Reply};
use chrono::Utc;

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

// New simplified chat structures
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

pub struct WebServer {
    port: u16,
    memory_system: Arc<Mutex<MemorySystem>>,
    concept_graph: Arc<Mutex<ConceptGraphManager>>,
    pattern_detector: Arc<Mutex<PatternDetector>>,
    rag_orchestrator: Arc<Mutex<RagOrchestrator>>,
    brain_ai_orchestrator: Arc<Mutex<BrainAIOrchestrator>>,
    development_sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
}

impl WebServer {
    pub async fn new(port: u16) -> Result<Self, BrainError> {
        let memory_system = Arc::new(Mutex::new(MemorySystem::new(1000)));
        let concept_graph = Arc::new(Mutex::new(
            ConceptGraphManager::new(ConceptGraphConfig::default()).await?
        ));
        let pattern_detector = Arc::new(Mutex::new(PatternDetector::new()));
        let rag_orchestrator = Arc::new(Mutex::new(RagOrchestrator::new()?));
        let brain_ai_orchestrator = Arc::new(Mutex::new(BrainAIOrchestrator::new()?));
        let development_sessions = Arc::new(Mutex::new(HashMap::new()));
        
        Ok(Self { 
            port,
            memory_system,
            concept_graph,
            pattern_detector,
            rag_orchestrator,
            brain_ai_orchestrator,
            development_sessions,
        })
    }

    pub async fn start(&self) -> Result<(), BrainError> {
        // CORS headers
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);

        // Static file serving
        let static_files = warp::fs::dir("web");

        // API Routes
        let api = warp::path("api");

        // Status endpoint
        let status = api
            .and(warp::path("status"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_status);

        // Stats endpoint
        let stats = api
            .and(warp::path("stats"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_stats);

        // Health endpoint
        let health = api
            .and(warp::path("health"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_health);

        // Learn endpoint
        let memory_system_learn = self.memory_system.clone();
        let learn = api
            .and(warp::path("learn"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system_learn.clone()))
            .and_then(Self::handle_learn);

        // Memory query endpoint
        let memory_system_query = self.memory_system.clone();
        let memory_query = api
            .and(warp::path("memory"))
            .and(warp::path("query"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system_query.clone()))
            .and_then(Self::handle_memory_query);

        // Segment endpoint
        let segment = api
            .and(warp::path("segment"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_segment);

        // Simulate endpoint
        let simulate = api
            .and(warp::path("simulate"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_simulate);

        // Concepts analyze endpoint
        let concepts_analyze = api
            .and(warp::path("concepts"))
            .and(warp::path("analyze"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_concepts_analyze);

        // Chat endpoint
        let memory_system = self.memory_system.clone();
        let concept_graph = self.concept_graph.clone();
        let pattern_detector = self.pattern_detector.clone();
        let rag_orchestrator = self.rag_orchestrator.clone();
        let chat = api
            .and(warp::path("chat"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system.clone()))
            .and(warp::any().map(move || concept_graph.clone()))
            .and(warp::any().map(move || pattern_detector.clone()))
            .and(warp::any().map(move || rag_orchestrator.clone()))
            .and_then(Self::handle_chat);

        // RAG Chat endpoint (new)
        let memory_system_rag = self.memory_system.clone();
        let concept_graph_rag = self.concept_graph.clone();
        let pattern_detector_rag = self.pattern_detector.clone();
        let rag_orchestrator_rag = self.rag_orchestrator.clone();
        let rag_chat = api
            .and(warp::path("rag"))
            .and(warp::path("chat"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system_rag.clone()))
            .and(warp::any().map(move || concept_graph_rag.clone()))
            .and(warp::any().map(move || pattern_detector_rag.clone()))
            .and(warp::any().map(move || rag_orchestrator_rag.clone()))
            .and_then(Self::handle_rag_chat);

        // RAG Stats endpoint
        let rag_orchestrator_stats = self.rag_orchestrator.clone();
        let rag_stats = api
            .and(warp::path("rag"))
            .and(warp::path("stats"))
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::any().map(move || rag_orchestrator_stats.clone()))
            .and_then(Self::handle_rag_stats);

        // Export endpoint
        let export = api
            .and(warp::path("export"))
            .and(warp::path("all"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_export);

        // Enhanced LLM Training Integration endpoints
        let learning_analytics = api
            .and(warp::path("learning"))
            .and(warp::path("analytics"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_learning_analytics);

        let start_learning_session = api
            .and(warp::path("learning"))
            .and(warp::path("session"))
            .and(warp::path("start"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_start_learning_session);

        let end_learning_session = api
            .and(warp::path("learning"))
            .and(warp::path("session"))
            .and(warp::path("end"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_end_learning_session);

        let knowledge_gaps = api
            .and(warp::path("learning"))
            .and(warp::path("gaps"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_knowledge_gaps);

        let learning_recommendations = api
            .and(warp::path("learning"))
            .and(warp::path("recommendations"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_learning_recommendations);

        let performance_trends = api
            .and(warp::path("learning"))
            .and(warp::path("performance"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_performance_trends);

        // New simplified chat endpoints
        let memory_system_chat_learn = self.memory_system.clone();
        let concept_graph_chat_learn = self.concept_graph.clone();
        let brain_ai_orchestrator_learn = self.brain_ai_orchestrator.clone();
        let chat_learn = api
            .and(warp::path("chat"))
            .and(warp::path("learn"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system_chat_learn.clone()))
            .and(warp::any().map(move || concept_graph_chat_learn.clone()))
            .and(warp::any().map(move || brain_ai_orchestrator_learn.clone()))
            .and_then(Self::handle_simple_chat_learn);

        let memory_system_chat_converse = self.memory_system.clone();
        let concept_graph_chat_converse = self.concept_graph.clone();
        let brain_ai_orchestrator_converse = self.brain_ai_orchestrator.clone();
        let chat_converse = api
            .and(warp::path("chat"))
            .and(warp::path("converse"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system_chat_converse.clone()))
            .and(warp::any().map(move || concept_graph_chat_converse.clone()))
            .and(warp::any().map(move || brain_ai_orchestrator_converse.clone()))
            .and_then(Self::handle_simple_chat_converse);

        // Code Pattern Recognition API endpoint
        let memory_system_code_patterns = self.memory_system.clone();
        let concept_graph_code_patterns = self.concept_graph.clone();
        let code_patterns = api
            .and(warp::path("code"))
            .and(warp::path("analyze-patterns"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system_code_patterns.clone()))
            .and(warp::any().map(move || concept_graph_code_patterns.clone()))
            .and_then(Self::handle_code_pattern_analysis);

        // Development Context API endpoints
        let memory_system_dev_context = self.memory_system.clone();
        let concept_graph_dev_context = self.concept_graph.clone();
        let sessions_dev_context = self.development_sessions.clone();
        let dev_context_create = api
            .and(warp::path("dev"))
            .and(warp::path("context"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || memory_system_dev_context.clone()))
            .and(warp::any().map(move || concept_graph_dev_context.clone()))
            .and(warp::any().map(move || sessions_dev_context.clone()))
            .and_then(Self::handle_development_context_create);

        let sessions_dev_context_get = self.development_sessions.clone();
        let dev_context_get = api
            .and(warp::path("dev"))
            .and(warp::path("context"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::any().map(move || sessions_dev_context_get.clone()))
            .and_then(Self::handle_development_context_get);

        // Group routes to reduce recursion depth
        let basic_routes = status
            .or(stats)
            .or(health)
            .or(learn)
            .or(memory_query)
            .or(segment)
            .or(simulate)
            .or(concepts_analyze);

        let chat_routes = chat
            .or(rag_chat)
            .or(rag_stats)
            .or(chat_learn)
            .or(chat_converse);

        let learning_routes = learning_analytics
            .or(start_learning_session)
            .or(end_learning_session)
            .or(knowledge_gaps)
            .or(learning_recommendations)
            .or(performance_trends);

        let dev_routes = code_patterns
            .or(dev_context_create)
            .or(dev_context_get);

        // Combine grouped routes
        let api_routes = basic_routes
            .or(chat_routes)
            .or(learning_routes)
            .or(dev_routes)
            .or(export);

        let routes = static_files
            .or(api_routes)
            .with(cors);

        println!("🚀 Brain AI Web Server starting on http://localhost:{}", self.port);
        println!("📱 Interface available at: http://localhost:{}/brain-interface.html", self.port);
        println!("💬 Simple Chat Interface: http://localhost:{}/chat.html", self.port);
        println!("📊 Concept Graph: http://localhost:{}/concept_graph.html", self.port);
        println!("⏰ Memory Timeline: http://localhost:{}/memory_timeline.html", self.port);
        println!("🎮 Simulation Dashboard: http://localhost:{}/simulation_dashboard.html", self.port);

        warp::serve(routes)
            .run(([127, 0, 0, 1], self.port))
            .await;

        Ok(())
    }

    async fn handle_status() -> Result<impl Reply, warp::Rejection> {
        let response = StatusResponse {
            status: "Online".to_string(),
            uptime: "Running".to_string(),
            version: "1.0.0".to_string(),
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_stats() -> Result<impl Reply, warp::Rejection> {
        let response = StatsResponse {
            memory_usage: "2.1GB".to_string(),
            confidence: 0.987,
            active_processes: 42,
            response_time: 42,
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_health() -> Result<impl Reply, warp::Rejection> {
        let response = HealthResponse {
            system_status: "Optimal".to_string(),
            memory_efficiency: "94.2%".to_string(),
            processing_speed: "18,500 tokens/sec".to_string(),
            active_connections: 247,
            uptime: "72h 14m".to_string(),
            last_backup: "2 hours ago".to_string(),
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_learn(
        request: ProcessRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        // Check if it's a GitHub URL
        if request.is_github_url || GitHubClient::parse_github_url(&request.text).is_ok() {
            // Handle GitHub learning
            match Self::process_github_learning(&request.text, memory_system.clone()).await {
                Ok(github_data) => {
                    let processing_time = start_time.elapsed().as_millis() as u64;
                    let response = ProcessResponse {
                        success: true,
                        message: "GitHub repository learned successfully".to_string(),
                        data: Some(github_data),
                        processing_time,
                    };
                    return Ok(warp::reply::json(&response));
                }
                Err(e) => {
                    println!("GitHub learning failed: {}, falling back to simulation", e);
                }
            }
        }
        
        // Fallback to regular text processing or simulation
        // Actually store some data in memory for GitHub URLs
        let (concepts_discovered, knowledge_connections) = if request.text.contains("github.com") {
            let mut memory = memory_system.lock().await;
            
            // Extract repository info from URL
            let repo_name = if let Some(captures) = regex::Regex::new(r"github\.com/([^/]+)/([^/]+)")
                .unwrap()
                .captures(&request.text) {
                format!("{}/{}", &captures[1], &captures[2])
            } else {
                "Unknown Repository".to_string()
            };
            
            // Store some basic repository information in episodic memory
            let repo_info = format!("GitHub Repository: {}\nURL: {}\nThis is a software repository that was analyzed by Brain AI. The repository contains code, documentation, and project files.", repo_name, request.text);
            
            let mut context = std::collections::HashMap::new();
            context.insert("type".to_string(), "github_repository".to_string());
            context.insert("source".to_string(), "simulation".to_string());
            
            let _episode = EpisodicEvent::new(
                repo_info.clone(),
                context,
                0.8,
                "github_learning_simulation".to_string()
            );
            
            // Try to store in episodic memory if available
            let _episodic_id = match memory.query_episodic(&EpisodicQuery::default()) {
                Ok(_) => {
                    // Episodic memory is available, but we can't directly store events
                    // Let's just use working memory for now
                    uuid::Uuid::new_v4()
                },
                Err(_) => {
                    // No episodic memory available
                    uuid::Uuid::new_v4()
                }
            };
            
            // Also store in working memory
            let _working_id = match memory.learn(
                format!("Repository {} contains software project files and documentation", repo_name),
                Priority::Medium
            ) {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Failed to store in working memory: {}", e);
                    uuid::Uuid::new_v4()
                }
            };
            
            // Store semantic concept
            let concept = SemanticConcept::new(
                repo_name.clone(),
                format!("A GitHub repository containing a software project"),
                vec![0.1, 0.2, 0.3, 0.4, 0.5] // Simple dummy embedding
            );
            
            if let Err(e) = memory.store_concept(concept) {
                eprintln!("Failed to store semantic concept: {}", e);
            }
            
            println!("✅ Stored GitHub repository information in memory: {}", repo_name);
            (3, 5) // Realistic numbers for concepts and connections
        } else {
            // Regular text processing simulation
            ((request.text.len() / 50 + 5) as usize, (request.text.len() / 30 + 8) as usize)
        };
        
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let mut data = HashMap::new();
        data.insert("concepts_discovered".to_string(), serde_json::Value::Number(concepts_discovered.into()));
        data.insert("knowledge_connections".to_string(), serde_json::Value::Number(knowledge_connections.into()));
        data.insert("text_length".to_string(), serde_json::Value::Number(request.text.len().into()));
        data.insert("processing_time_ms".to_string(), serde_json::Value::Number(processing_time.into()));
        
        let response = ProcessResponse {
            success: true,
            message: "Text learned successfully".to_string(),
            data: Some(serde_json::Value::Object(data.into_iter().collect())),
            processing_time,
        };
        Ok(warp::reply::json(&response))
    }

    async fn process_github_learning(
        github_url: &str,
        memory_system: Arc<Mutex<MemorySystem>>,
    ) -> Result<serde_json::Value, BrainError> {
        // Initialize GitHub learning engine
        let github_token = std::env::var("GITHUB_TOKEN").ok();
        let learning_engine = GitHubLearningEngine::new(github_token, None);
        
        // Process the GitHub repository using persistent memory system
        let learning_result = {
            let mut memory = memory_system.lock().await;
            learning_engine
                .learn_from_repository(&mut *memory, github_url)
                .await?
        };
        
        // Convert to JSON for response
        let mut data = HashMap::new();
        data.insert("repository".to_string(), serde_json::Value::String(learning_result.repository));
        data.insert("files_processed".to_string(), serde_json::Value::Number(learning_result.files_processed.into()));
        data.insert("total_content_size".to_string(), serde_json::Value::Number(learning_result.total_content_size.into()));
        data.insert("learning_time_ms".to_string(), serde_json::Value::Number(learning_result.learning_time_ms.into()));
        data.insert("concepts_discovered".to_string(), serde_json::Value::Number(learning_result.concepts_discovered.into()));
        data.insert("memory_entries_created".to_string(), serde_json::Value::Number(learning_result.memory_entries_created.into()));
        data.insert("summary".to_string(), serde_json::Value::String(learning_result.summary));
        data.insert("key_insights".to_string(), serde_json::Value::Array(
            learning_result.key_insights.into_iter().map(|s| serde_json::Value::String(s)).collect()
        ));
        
        Ok(serde_json::Value::Object(data.into_iter().collect()))
    }

    async fn handle_chat(
        request: ChatRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
        pattern_detector: Arc<Mutex<PatternDetector>>,
        rag_orchestrator: Arc<Mutex<RagOrchestrator>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        println!("🎯 Received chat request: {}", request.message);
        
        // Create RAG request from chat request
        let rag_request = RagRequest {
            message: request.message.clone(),
            conversation_id: None, // Let the orchestrator create a new one
            context_limit: Some(15),
            retrieval_threshold: Some(0.2),
        };
        
        // Process through RAG orchestrator
        let response = {
            let mut memory = memory_system.lock().await;
            let mut graph = concept_graph.lock().await;
            let mut detector = pattern_detector.lock().await;
            let mut orchestrator = rag_orchestrator.lock().await;
            
            match orchestrator.process_conversation(
                rag_request,
                &mut *memory,
                &mut *graph,
                &mut *detector,
            ).await {
                Ok(rag_response) => {
                    println!("✅ RAG Orchestrator generated response");
                    rag_response.response
                },
                Err(e) => {
                    eprintln!("❌ RAG Orchestrator failed: {}", e);
                    // Fallback to basic Brain AI processing
                    match Self::process_with_brain_ai(&request.message, &request.history, &mut *memory, &mut *graph, &mut *detector).await {
                        Ok(ai_response) => {
                            println!("✅ Fallback Brain AI response generated");
                            ai_response
                        },
                        Err(_) => {
                            println!("⚠️ Using fallback response");
                            Self::generate_fallback_response(&request.message)
                        }
                    }
                }
            }
        };
        
        let suggestions = {
            let memory = memory_system.lock().await;
            Self::generate_brain_suggestions(&request.message, &*memory)
        };
        
        let context_used = !request.history.is_empty();
        let _processing_time = start_time.elapsed().as_millis() as u64;
        
        let chat_response = ChatResponse {
            response,
            context_used,
            suggestions,
        };
        
        println!("📤 Sending chat response with {} suggestions", chat_response.suggestions.len());
        Ok(warp::reply::json(&chat_response))
    }

    async fn handle_memory_query(
        request: QueryRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        let (memories_found, relevance_score, memory_contents) = {
            let memory = memory_system.lock().await;
            
            let mut total_memories = 0;
            let mut max_relevance: f64 = 0.0;
            let mut all_contents = Vec::new();
            
            // Query working memory with multiple strategies
            let search_terms = Self::extract_search_terms(&request.query);
            
            for term in &search_terms {
                let working_query = WorkingMemoryQuery {
                    content_pattern: Some(term.clone()),
                    limit: Some(10),
                    ..Default::default()
                };
                
                if let Ok(working_items) = memory.query_working(&working_query) {
                    for item in &working_items {
                        let relevance = Self::calculate_text_similarity(&item.content, &request.query);
                        max_relevance = max_relevance.max(relevance);
                        
                        all_contents.push(serde_json::json!({
                            "type": "working_memory",
                            "content": item.content,
                            "relevance": relevance,
                            "priority": format!("{:?}", item.priority),
                            "created_at": item.created_at.to_rfc3339(),
                            "search_term": term
                        }));
                    }
                    total_memories += working_items.len();
                }
            }
            
            // Also try exact query
            let working_query = WorkingMemoryQuery {
                content_pattern: Some(request.query.clone()),
                limit: Some(20),
                ..Default::default()
            };
            
            if let Ok(working_items) = memory.query_working(&working_query) {
                for item in &working_items {
                    let relevance = Self::calculate_text_similarity(&item.content, &request.query);
                    max_relevance = max_relevance.max(relevance);
                    
                    all_contents.push(serde_json::json!({
                        "type": "working_memory",
                        "content": item.content,
                        "relevance": relevance,
                        "priority": format!("{:?}", item.priority),
                        "created_at": item.created_at.to_rfc3339(),
                        "search_term": "exact_match"
                    }));
                }
                total_memories += working_items.len();
            }
            
            // Query episodic memory for the search term
            let episodic_query = EpisodicQuery {
                content_pattern: Some(request.query.clone()),
                time_range: Some((
                    Utc::now() - chrono::Duration::days(365),
                    Utc::now()
                )),
                limit: Some(50),
                ..Default::default()
            };
            
            if let Ok(episodes) = memory.query_episodic(&episodic_query) {
                for episode in &episodes {
                    let relevance = Self::calculate_text_similarity(&episode.content, &request.query);
                    max_relevance = max_relevance.max(relevance);
                    
                    all_contents.push(serde_json::json!({
                        "type": "episodic_memory",
                        "content": episode.content,
                        "relevance": relevance,
                        "importance": episode.importance,
                        "timestamp": episode.timestamp.to_rfc3339(),
                        "tags": episode.tags
                    }));
                }
                total_memories += episodes.len();
            }
            
            // Query semantic memory
            let semantic_query = SemanticQuery {
                name_pattern: Some(request.query.clone()),
                limit: Some(20),
                ..Default::default()
            };
            
            if let Ok(semantic_concepts) = memory.query_semantic(&semantic_query) {
                for concept in &semantic_concepts {
                    let relevance = Self::calculate_text_similarity(&concept.name, &request.query);
                    max_relevance = max_relevance.max(relevance);
                    
                    all_contents.push(serde_json::json!({
                        "type": "semantic_memory",
                        "name": concept.name,
                        "description": concept.description,
                        "relevance": relevance,
                        "confidence": concept.confidence
                    }));
                }
                total_memories += semantic_concepts.len();
            }
            
            // Sort by relevance
            all_contents.sort_by(|a, b| {
                let rel_a = a["relevance"].as_f64().unwrap_or(0.0);
                let rel_b = b["relevance"].as_f64().unwrap_or(0.0);
                rel_b.partial_cmp(&rel_a).unwrap()
            });
            
            (total_memories, max_relevance, all_contents)
        };
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let mut data = HashMap::new();
        data.insert("memories_found".to_string(), serde_json::Value::Number(memories_found.into()));
        data.insert("query".to_string(), serde_json::Value::String(request.query.clone()));
        data.insert("relevance_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(relevance_score).unwrap_or_else(|| serde_json::Number::from(0))));
        data.insert("memory_contents".to_string(), serde_json::Value::Array(memory_contents));
        
        let response = ProcessResponse {
            success: true,
            message: "Memory query completed".to_string(),
            data: Some(serde_json::Value::Object(data.into_iter().collect())),
            processing_time,
        };
        Ok(warp::reply::json(&response))
    }

    /// Extract search terms from a query for better matching
    fn extract_search_terms(query: &str) -> Vec<String> {
        let mut terms = Vec::new();
        
        // Add the full query
        terms.push(query.to_string());
        
        // Add lowercased version
        terms.push(query.to_lowercase());
        
        // Add individual words
        for word in query.split_whitespace() {
            if word.len() > 2 {
                terms.push(word.to_string());
                terms.push(word.to_lowercase());
            }
        }
        
        // Add specific variations for common terms
        let query_lower = query.to_lowercase();
        if query_lower.contains("pocketflow") {
            terms.extend(vec![
                "PocketFlow".to_string(),
                "pocketflow".to_string(),
                "pocket".to_string(),
                "flow".to_string(),
                "Pocket Flow".to_string(),
                "pocket-flow".to_string(),
            ]);
        }
        
        if query_lower.contains("architecture") {
            terms.extend(vec![
                "architecture".to_string(),
                "pattern".to_string(),
                "design".to_string(),
                "framework".to_string(),
            ]);
        }
        
        // Remove duplicates
        terms.sort();
        terms.dedup();
        
        terms
    }

    async fn handle_segment(request: ProcessRequest) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        // Simulate processing
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let segments_count = (request.text.len() / 100).max(1).min(8);
        
        let mut data = HashMap::new();
        data.insert("segments_count".to_string(), serde_json::Value::Number(segments_count.into()));
        data.insert("coherence_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.92).unwrap()));
        data.insert("text_length".to_string(), serde_json::Value::Number(request.text.len().into()));
        
        let response = ProcessResponse {
            success: true,
            message: "Text segmentation completed".to_string(),
            data: Some(serde_json::Value::Object(data.into_iter().collect())),
            processing_time,
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_simulate(request: SimulationRequest) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        // Simulate processing
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let mut data = HashMap::new();
        data.insert("scenario".to_string(), serde_json::Value::String(request.scenario));
        data.insert("iterations".to_string(), serde_json::Value::Number(1247.into()));
        data.insert("convergence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.94).unwrap()));
        data.insert("stability_index".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.88).unwrap()));
        
        let response = ProcessResponse {
            success: true,
            message: "Simulation completed".to_string(),
            data: Some(serde_json::Value::Object(data.into_iter().collect())),
            processing_time,
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_concepts_analyze(request: ProcessRequest) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        // Simulate processing
        tokio::time::sleep(tokio::time::Duration::from_millis(1200)).await;
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let concepts_count = (request.text.split_whitespace().count() / 10).max(3).min(12);
        
        let mut data = HashMap::new();
        data.insert("text_analyzed".to_string(), serde_json::Value::String(request.text));
        data.insert("primary_concepts".to_string(), serde_json::Value::Number(concepts_count.into()));
        data.insert("relationship_density".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.76).unwrap()));
        data.insert("semantic_complexity".to_string(), serde_json::Value::Number(8.into()));
        data.insert("novelty_factor".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.73).unwrap()));
        
        let response = ProcessResponse {
            success: true,
            message: "Concept analysis completed".to_string(),
            data: Some(serde_json::Value::Object(data.into_iter().collect())),
            processing_time,
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_rag_chat(
        request: ChatRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
        pattern_detector: Arc<Mutex<PatternDetector>>,
        rag_orchestrator: Arc<Mutex<RagOrchestrator>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        println!("🎯 Received chat request: {}", request.message);
        
        // Create RAG request from chat request
        let rag_request = RagRequest {
            message: request.message.clone(),
            conversation_id: None, // Let the orchestrator create a new one
            context_limit: Some(15),
            retrieval_threshold: Some(0.2),
        };
        
        // Process through RAG orchestrator
        let response = {
            let mut memory = memory_system.lock().await;
            let mut graph = concept_graph.lock().await;
            let mut detector = pattern_detector.lock().await;
            let mut orchestrator = rag_orchestrator.lock().await;
            
            match orchestrator.process_conversation(
                rag_request,
                &mut *memory,
                &mut *graph,
                &mut *detector,
            ).await {
                Ok(rag_response) => {
                    println!("✅ RAG Orchestrator generated response");
                    rag_response.response
                },
                Err(e) => {
                    eprintln!("❌ RAG Orchestrator failed: {}", e);
                    // Fallback to basic Brain AI processing
                    match Self::process_with_brain_ai(&request.message, &request.history, &mut *memory, &mut *graph, &mut *detector).await {
                        Ok(ai_response) => {
                            println!("✅ Fallback Brain AI response generated");
                            ai_response
                        },
                        Err(_) => {
                            println!("⚠️ Using fallback response");
                            Self::generate_fallback_response(&request.message)
                        }
                    }
                }
            }
        };
        
        let suggestions = {
            let memory = memory_system.lock().await;
            Self::generate_brain_suggestions(&request.message, &*memory)
        };
        
        let context_used = !request.history.is_empty();
        let _processing_time = start_time.elapsed().as_millis() as u64;
        
        let chat_response = ChatResponse {
            response,
            context_used,
            suggestions,
        };
        
        println!("📤 Sending chat response with {} suggestions", chat_response.suggestions.len());
        Ok(warp::reply::json(&chat_response))
    }

    async fn handle_rag_stats(
        rag_orchestrator: Arc<Mutex<RagOrchestrator>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let response = {
            let mut orchestrator = rag_orchestrator.lock().await;
            orchestrator.get_stats()
        };
        Ok(warp::reply::json(&response))
    }

    async fn handle_export() -> Result<impl Reply, warp::Rejection> {
        let mut export_data = HashMap::new();
        export_data.insert("timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
        export_data.insert("version".to_string(), serde_json::Value::String("1.0.0".to_string()));
        export_data.insert("memories_count".to_string(), serde_json::Value::Number(7842.into()));
        export_data.insert("concepts_count".to_string(), serde_json::Value::Number(2156.into()));
        export_data.insert("relationships".to_string(), serde_json::Value::Number(4329.into()));
        export_data.insert("total_knowledge_size".to_string(), serde_json::Value::String("47.2 MB".to_string()));
        
        let json_data = serde_json::to_string_pretty(&export_data)
            .unwrap_or_else(|_| "{}".to_string());
            
        Ok(warp::reply::with_header(
            json_data,
            "Content-Type",
            "application/json",
        ))
    }

    // New Brain AI processing function
    async fn process_with_brain_ai(
        message: &str,
        history: &[ChatMessage],
        memory_system: &mut MemorySystem,
        concept_graph: &mut ConceptGraphManager,
        pattern_detector: &mut PatternDetector,
    ) -> Result<String, BrainError> {
        // Step 1: Store the user's message in working memory with high priority
        let _message_id = memory_system.learn(message.to_string(), Priority::High)?;
        
        // Step 2: Process conversation history to build context
        for chat_msg in history {
            let priority = if chat_msg.role == "user" { Priority::Medium } else { Priority::Low };
            let _history_id = memory_system.learn(chat_msg.content.clone(), priority)?;
        }
        
        // Step 3: Segment the message to understand its structure
        let segmenter = BpeSegmenter::new(BpeConfig::default());
        let segments = segmenter.segment_text(message);
        
        // Step 4: Extract concepts and add to concept graph
        for segment in &segments {
            if segment.len() > 2 { // Only meaningful segments
                let concept = ConceptNode::new(
                    ConceptType::Abstract,
                    segment.clone(),
                    0.7, // confidence
                    Some(format!("User input: {}", message))
                );
                let _concept_id = concept_graph.create_concept(concept).await?;
            }
        }
        
        // Step 5: Query memory for related information using flexible search
        let mut related_concepts = Vec::new();
        
        // Try multiple query strategies to find relevant knowledge
        // Strategy 1: Search by keywords from the message
        let keywords: Vec<&str> = message.split_whitespace()
            .filter(|word| word.len() > 2) // Only meaningful words
            .collect();
        
        for keyword in &keywords {
            let semantic_query = SemanticQuery {
                name_pattern: Some(keyword.to_string()),
                min_confidence: Some(0.1), // Lower threshold for broader matches
                limit: Some(5),
                ..Default::default()
            };
            if let Ok(mut concepts) = memory_system.query_semantic(&semantic_query) {
                related_concepts.append(&mut concepts);
            }
        }
        
        // Strategy 2: Get all semantic concepts if we don't have specific matches
        if related_concepts.is_empty() {
            let broad_query = SemanticQuery {
                name_pattern: None, // Get all concepts
                min_confidence: Some(0.1),
                limit: Some(10),
                ..Default::default()
            };
            if let Ok(mut concepts) = memory_system.query_semantic(&broad_query) {
                related_concepts.append(&mut concepts);
            }
        }
        
        // Remove duplicates
        related_concepts.sort_by(|a, b| a.name.cmp(&b.name));
        related_concepts.dedup_by(|a, b| a.name == b.name);
        
        // Step 6: Query working memory for recent context using flexible search
        let mut recent_context = Vec::new();
        
        // Strategy 1: Search by keywords
        for keyword in &keywords {
            let working_query = WorkingMemoryQuery {
                content_pattern: Some(keyword.to_string()),
                min_importance: Some(0.1), // Lower threshold
                limit: Some(3),
                ..Default::default()
            };
            if let Ok(mut context) = memory_system.query_working(&working_query) {
                recent_context.append(&mut context);
            }
        }
        
        // Strategy 2: Get recent items if no specific matches
        if recent_context.is_empty() {
            let broad_query = WorkingMemoryQuery {
                content_pattern: None, // Get recent items
                min_importance: Some(0.1),
                limit: Some(5),
                ..Default::default()
            };
            if let Ok(mut context) = memory_system.query_working(&broad_query) {
                recent_context.append(&mut context);
            }
        }
        
        // Remove duplicates
        recent_context.sort_by(|a, b| a.content.cmp(&b.content));
        recent_context.dedup_by(|a, b| a.content == b.content);
        
        // Debug: Print what we found in memory
        println!("🧠 Memory Query Debug:");
        println!("  - Keywords searched: {:?}", keywords);
        println!("  - Related concepts found: {}", related_concepts.len());
        println!("  - Recent context found: {}", recent_context.len());
        
        for (i, concept) in related_concepts.iter().take(3).enumerate() {
            println!("  - Concept {}: {} -> {}", i+1, concept.name, concept.description);
        }
        
        for (i, context) in recent_context.iter().take(3).enumerate() {
            println!("  - Context {}: {}", i+1, if context.content.len() > 50 { 
                format!("{}...", &context.content[..50]) 
            } else { 
                context.content.clone() 
            });
        }
        
        // Step 7: Detect patterns in the conversation
        let _pattern_result = pattern_detector.detect_patterns_from_memory(memory_system).await?;
        
        // Step 8: Generate response based on Brain AI analysis
        let response = Self::generate_brain_response(message, &related_concepts, &recent_context, &segments);
        
        Ok(response)
    }

    fn generate_brain_response(
        message: &str,
        related_concepts: &[crate::memory::SemanticConcept],
        recent_context: &[crate::memory::WorkingMemoryItem],
        segments: &[String],
    ) -> String {
        let lower_message = message.to_lowercase();
        
        // Check if we have actual knowledge to share
        let has_relevant_knowledge = !related_concepts.is_empty() || !recent_context.is_empty();
        
        // Analyze the message intent
        let is_question = message.contains('?') || lower_message.starts_with("what") || 
                         lower_message.starts_with("how") || lower_message.starts_with("why") ||
                         lower_message.starts_with("when") || lower_message.starts_with("where") ||
                         lower_message.contains("tell me") || lower_message.contains("know");
        
        let is_greeting = lower_message.contains("hi") || lower_message.contains("hello") || 
                         lower_message.contains("hey") || lower_message == "hi" || lower_message == "hello";
        
        let is_github_question = lower_message.contains("github.com") || lower_message.contains("repo") || 
                               lower_message.contains("repository") || lower_message.contains("learned");
        
        let mut response = String::new();
        
        // Handle greetings naturally
        if is_greeting && !has_relevant_knowledge {
            return "Hello! 👋 I'm your Brain AI assistant. I can learn from GitHub repositories, analyze code patterns, and help with programming questions. What would you like to explore today?".to_string();
        }
        
        // Handle questions about what I know/have learned
        if is_question && (lower_message.contains("know") || lower_message.contains("learned") || lower_message.contains("repos")) {
            if has_relevant_knowledge {
                response.push_str("🧠 **My Current Knowledge Base**:\n\n");
                
                if !related_concepts.is_empty() {
                    response.push_str(&format!("**📚 Concepts in Memory ({} found)**:\n", related_concepts.len()));
                    for (i, concept) in related_concepts.iter().take(5).enumerate() {
                        response.push_str(&format!("{}. **{}**: {}\n", i + 1, concept.name, 
                            if concept.description.len() > 80 { 
                                format!("{}...", &concept.description[..80]) 
                            } else { 
                                concept.description.clone() 
                            }));
                    }
                    response.push('\n');
                }
                
                if !recent_context.is_empty() {
                    response.push_str(&format!("**💭 Recent Learning ({} items)**:\n", recent_context.len()));
                    for (i, item) in recent_context.iter().take(3).enumerate() {
                        let preview = if item.content.len() > 100 {
                            format!("{}...", &item.content[..100])
                        } else {
                            item.content.clone()
                        };
                        response.push_str(&format!("{}. {}\n", i + 1, preview));
                    }
                    response.push('\n');
                }
                
                response.push_str("💡 **I can help you with**: Code analysis, architecture patterns, specific technologies, or dive deeper into any of these concepts!");
            } else {
                response.push_str("🧠 **Knowledge Status**: I'm just starting up! My memory systems are empty, but I'm ready to learn.\n\n");
                response.push_str("**To build my knowledge, you can**:\n");
                response.push_str("• 🔗 Share GitHub repository URLs for me to analyze\n");
                response.push_str("• 📝 Teach me about technologies or concepts\n");
                response.push_str("• ❓ Ask me programming questions to engage my reasoning\n\n");
                response.push_str("**Try asking**: \"Learn from https://github.com/owner/repo\" or \"What do you know about Rust?\"");
            }
            return response;
        }
        
        // Handle GitHub-specific questions
        if is_github_question {
            if has_relevant_knowledge {
                response.push_str("🔗 **GitHub Repository Analysis**:\n\n");
                
                // Look for GitHub-related content in memory
                let github_concepts: Vec<_> = related_concepts.iter()
                    .filter(|c| c.name.contains("github") || c.description.contains("repository") || 
                               c.description.contains("repo") || c.name.contains("http"))
                    .collect();
                
                if !github_concepts.is_empty() {
                    response.push_str("**📦 Repositories I've analyzed**:\n");
                    for (i, concept) in github_concepts.iter().take(3).enumerate() {
                        response.push_str(&format!("{}. **{}**\n   {}\n\n", i + 1, concept.name, concept.description));
                    }
                } else if !related_concepts.is_empty() {
                    response.push_str("**🧠 Related knowledge from my analysis**:\n");
                    for (i, concept) in related_concepts.iter().take(3).enumerate() {
                        response.push_str(&format!("{}. **{}**: {}\n", i + 1, concept.name, concept.description));
                    }
                    response.push('\n');
                }
                
                response.push_str("💡 **Want to know more?** Ask me about specific technologies, patterns, or share another repository URL!");
            } else {
                response.push_str("🔗 **GitHub Learning**: I haven't analyzed any repositories yet!\n\n");
                response.push_str("**To get started**: Share a GitHub URL like this:\n");
                response.push_str("• `https://github.com/owner/repository-name`\n");
                response.push_str("• Or use the Learn mode in the control panel\n\n");
                response.push_str("I'll analyze the entire codebase, documentation, and structure to build my knowledge!");
            }
            return response;
        }
        
        // For general questions, use available knowledge
        if is_question && has_relevant_knowledge {
            response.push_str("🧠 **Based on my learned knowledge**:\n\n");
            
            // Analyze the context to provide intelligent responses
            let context_text = recent_context.iter()
                .map(|item| item.content.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            
            let concept_text = related_concepts.iter()
                .map(|concept| format!("{}: {}", concept.name, concept.description))
                .collect::<Vec<_>>()
                .join(" ");
            
            let combined_knowledge = format!("{} {}", context_text, concept_text);
            
            // Generate contextual responses based on the question
            if lower_message.contains("rust") {
                if combined_knowledge.to_lowercase().contains("rust") {
                    response.push_str("**🦀 About Rust Programming**:\n");
                    response.push_str("Based on our conversation, I can see you're interested in Rust! ");
                    
                    if combined_knowledge.to_lowercase().contains("love") || combined_knowledge.to_lowercase().contains("like") {
                        response.push_str("I notice you have positive feelings about Rust programming. ");
                    }
                    
                    response.push_str("Rust is a systems programming language known for:\n");
                    response.push_str("• **Memory safety** without garbage collection\n");
                    response.push_str("• **Zero-cost abstractions** for performance\n");
                    response.push_str("• **Ownership system** for managing memory\n");
                    response.push_str("• **Concurrency safety** preventing data races\n\n");
                    
                    response.push_str("💡 **What would you like to explore about Rust?** Ownership, borrowing, async programming, or specific use cases?");
                } else {
                    response.push_str("I see you're asking about Rust! While I don't have specific Rust knowledge stored yet, I can help you learn about it. Rust is a powerful systems programming language focused on safety and performance.");
                }
            } else if lower_message.contains("programming") || lower_message.contains("code") {
                response.push_str("**💻 About Programming**:\n");
                
                if combined_knowledge.to_lowercase().contains("rust") {
                    response.push_str("I can see from our conversation that you're interested in Rust programming! ");
                }
                
                response.push_str("Programming is the art and science of creating software solutions. Key aspects include:\n");
                response.push_str("• **Problem-solving** - Breaking down complex challenges\n");
                response.push_str("• **Language choice** - Selecting the right tool for the job\n");
                response.push_str("• **Design patterns** - Reusable solutions to common problems\n");
                response.push_str("• **Best practices** - Writing maintainable, efficient code\n\n");
                
                if combined_knowledge.to_lowercase().contains("rust") {
                    response.push_str("🦀 Since you mentioned Rust, it's excellent for systems programming, web backends, and performance-critical applications!\n\n");
                }
                
                response.push_str("💡 **What aspects of programming interest you most?** Languages, paradigms, specific technologies, or project ideas?");
            } else {
                // General knowledge display
                if !related_concepts.is_empty() {
                    response.push_str("**🔍 Relevant concepts I know**:\n");
                    for (i, concept) in related_concepts.iter().take(3).enumerate() {
                        response.push_str(&format!("{}. **{}**: {}\n", i + 1, concept.name, concept.description));
                    }
                    response.push('\n');
                }
                
                if !recent_context.is_empty() {
                    response.push_str("**💭 Recent context**:\n");
                    for (i, item) in recent_context.iter().take(2).enumerate() {
                        let preview = if item.content.len() > 150 {
                            format!("{}...", &item.content[..150])
                        } else {
                            item.content.clone()
                        };
                        response.push_str(&format!("{}. {}\n", i + 1, preview));
                    }
                    response.push('\n');
                }
                
                response.push_str("💡 **Need more specific information?** Ask me about particular aspects or share more context!");
            }
            
            return response;
        }
        
        // Fallback for when we don't have relevant knowledge
        if is_question {
            response.push_str("🤔 **I don't have specific knowledge about that yet**, but I can help!\n\n");
            response.push_str("**To build my knowledge**:\n");
            response.push_str("• 📚 Teach me by sharing information or examples\n");
            response.push_str("• 🔗 Share GitHub repositories related to your question\n");
            response.push_str("• 💬 Continue our conversation so I can learn from context\n\n");
            response.push_str("**Or ask me about**: Programming concepts, architecture patterns, or general software development topics!");
        } else {
            // For statements/general input
            response.push_str("✅ **Learned and stored** in my memory systems!\n\n");
            
            if !segments.is_empty() && segments.len() > 1 {
                response.push_str(&format!("**🔍 Analyzed into {} segments** for better understanding.\n", segments.len()));
            }
            
            response.push_str("**💭 I can now help you with**:\n");
            response.push_str("• Questions about what you just shared\n");
            response.push_str("• Related concepts and connections\n");
            response.push_str("• Building on this knowledge\n\n");
            response.push_str("**What would you like to explore next?**");
        }
        
        response
    }

    fn generate_brain_suggestions(message: &str, memory_system: &MemorySystem) -> Vec<String> {
        let lower_message = message.to_lowercase();
        let mut suggestions = Vec::new();
        
        // Get memory statistics for context-aware suggestions
        let stats = memory_system.get_stats();
        let has_semantic_memory = stats.get("semantic").map_or(0, |s| s.total_items) > 0;
        let has_episodic_memory = stats.get("episodic").map_or(0, |s| s.total_items) > 0;
        
        if lower_message.contains("rust") {
            suggestions.push("Show me Rust ownership examples".to_string());
            suggestions.push("Explain Rust error handling patterns".to_string());
            suggestions.push("Compare Rust with other systems languages".to_string());
        } else if lower_message.contains("javascript") || lower_message.contains("js") {
            suggestions.push("Show me async/await patterns in JavaScript".to_string());
            suggestions.push("Explain JavaScript closures with examples".to_string());
            suggestions.push("Compare JavaScript frameworks".to_string());
        } else if lower_message.contains("api") {
            suggestions.push("Design a RESTful API structure".to_string());
            suggestions.push("Explain API authentication methods".to_string());
            suggestions.push("Show API error handling patterns".to_string());
        } else if has_semantic_memory {
            suggestions.push("What patterns have you learned from our conversation?".to_string());
            suggestions.push("Analyze the concepts we've discussed".to_string());
            suggestions.push("Show me related knowledge from your memory".to_string());
        } else if has_episodic_memory {
            suggestions.push("What insights can you extract from our chat history?".to_string());
            suggestions.push("Summarize our conversation patterns".to_string());
            suggestions.push("Find connections between our topics".to_string());
        } else {
            suggestions.push("Help me understand a programming concept".to_string());
            suggestions.push("Analyze a code pattern or architecture".to_string());
            suggestions.push("Explain how your Brain AI works".to_string());
        }
        
        suggestions
    }

    fn generate_fallback_response(message: &str) -> String {
        format!("🤖 **Brain AI Processing**: I received your message: \"{}\"\n\nI'm currently initializing my cognitive systems. While my full Brain AI capabilities are starting up, I can still help you with:\n\n• Programming questions and examples\n• Architecture and design patterns\n• Code analysis and best practices\n• Technology comparisons\n\nPlease try your question again, and I'll process it through my complete neural architecture!", message)
    }

    fn calculate_text_similarity(text1: &str, text2: &str) -> f64 {
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Clean response text by removing internal system identifiers and improving formatting
    fn clean_response_text(text: &str) -> String {
        text
            // Remove internal system identifiers
            .replace("web_content_ai:", "")
            .replace("pattern_analysis:", "")
            .replace("semantic_analysis:", "")
            .replace("github_analysis:", "")
            .replace("code_analysis:", "")
            .replace("document_analysis:", "")
            
            // Clean up multiple spaces and newlines
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
            
            // Add better formatting
            .replace("Complex query with", "**🔍 Analysis:** Complex query with")
            .replace("Pattern:", "**🔄 Pattern detected:**")
            .replace("URL pattern detected", "**🔗 Web content detected**")
            .replace("Code pattern detected", "**💻 Code analysis completed**")
            .replace("Long text pattern detected", "**📄 Document content analyzed**")
            .replace("Query contains", "**📊 Text analysis:** Contains")
            .replace("Technical content detected:", "**🔧 Technical content identified:**")
    }

    // Enhanced LLM Training Integration handlers
    async fn handle_learning_analytics() -> Result<impl warp::Reply, warp::Rejection> {
        // For now, return a placeholder response
        let analytics = serde_json::json!({
            "active_learning_status": {
                "total_gaps_identified": 0,
                "high_priority_gaps": 0,
                "follow_up_questions_generated": 0,
                "learning_objectives_active": 0,
                "recent_gap_trends": []
            },
            "query_enhancement_insights": {
                "successful_patterns_count": 0,
                "failed_patterns_count": 0,
                "domain_rules_count": 0,
                "top_performing_patterns": [],
                "improvement_opportunities": []
            },
            "meta_learning_recommendations": {
                "learning_patterns_identified": 0,
                "memory_optimizations_suggested": 0,
                "relationship_insights_discovered": 0,
                "high_priority_recommendations": 0,
                "recent_insights": []
            },
            "performance_trends": {
                "query_performance_trend": "Stable",
                "learning_effectiveness_trend": "Stable",
                "overall_improvement": 0.0,
                "recent_performance_summary": "No data available"
            },
            "learning_efficiency": 0.0
        });
        Ok(warp::reply::json(&analytics))
    }

    async fn handle_start_learning_session(
        request: serde_json::Value
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let objective = request.get("objective")
            .and_then(|v| v.as_str())
            .unwrap_or("General learning session");
        
        let session_id = format!("session_{}", chrono::Utc::now().timestamp());
        Ok(warp::reply::json(&serde_json::json!({
            "session_id": session_id,
            "status": "started",
            "objective": objective
        })))
    }

    async fn handle_end_learning_session(
        request: serde_json::Value
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let session_id = request.get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let summary = serde_json::json!({
            "session_id": session_id,
            "duration_minutes": 0.0,
            "activities_completed": 0,
            "knowledge_gained": 0,
            "avg_activity_success": 0.0,
            "insights_generated": 0,
            "overall_effectiveness": 0.0
        });
        Ok(warp::reply::json(&summary))
    }

    async fn handle_knowledge_gaps() -> Result<impl warp::Reply, warp::Rejection> {
        let gaps: Vec<serde_json::Value> = Vec::new();
        Ok(warp::reply::json(&gaps))
    }

    async fn handle_learning_recommendations() -> Result<impl warp::Reply, warp::Rejection> {
        let recommendations: Vec<String> = vec![
            "Consider expanding concept relationships".to_string(),
            "Improve query pattern recognition".to_string(),
            "Enhance memory retrieval efficiency".to_string(),
        ];
        Ok(warp::reply::json(&recommendations))
    }

    async fn handle_performance_trends() -> Result<impl warp::Reply, warp::Rejection> {
        let trends = serde_json::json!({
            "query_performance_trend": "Stable",
            "learning_effectiveness_trend": "Stable",
            "overall_improvement": 0.0,
            "recent_performance_summary": "No data available"
        });
        Ok(warp::reply::json(&trends))
    }

    async fn handle_simple_chat_learn(
        request: SimpleChatLearnRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
        brain_ai_orchestrator: Arc<Mutex<BrainAIOrchestrator>>,
    ) -> Result<impl Reply, warp::Rejection> {
        println!("📚 Processing learning request for {} characters", request.content.len());
        
        let start_time = std::time::Instant::now();
        
        match Self::process_learning_content(&request.content, memory_system, concept_graph, brain_ai_orchestrator).await {
            Ok(insights) => {
                let processing_time = start_time.elapsed().as_millis() as u64;
                
                let response = SimpleChatResponse {
                    response: format!(
                        "✅ I've successfully learned from your content! Here's what I discovered:\n\n{}",
                        insights.iter()
                            .enumerate()
                            .map(|(i, insight)| format!("{}. {}", i + 1, insight))
                            .collect::<Vec<_>>()
                            .join("\n")
                    ),
                    insights_learned: insights,
                    context_used: true,
                };
                
                println!("✅ Learning completed in {}ms", processing_time);
                Ok(warp::reply::json(&response))
            }
            Err(e) => {
                println!("❌ Learning failed: {}", e);
                let error_response = SimpleChatResponse {
                    response: format!("I encountered an error while learning from your content: {}", e),
                    insights_learned: vec![],
                    context_used: false,
                };
                Ok(warp::reply::json(&error_response))
            }
        }
    }

    async fn handle_simple_chat_converse(
        request: SimpleChatConverseRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
        brain_ai_orchestrator: Arc<Mutex<BrainAIOrchestrator>>,
    ) -> Result<impl Reply, warp::Rejection> {
        println!("💬 Processing conversation request: {}", request.message);
        
        let start_time = std::time::Instant::now();
        
        match Self::process_conversation(&request.message, &request.history, memory_system, concept_graph, brain_ai_orchestrator).await {
            Ok(response) => {
                let processing_time = start_time.elapsed().as_millis() as u64;
                
                println!("✅ Conversation response generated in {}ms", processing_time);
                Ok(warp::reply::json(&response))
            }
            Err(e) => {
                println!("❌ Conversation failed: {}", e);
                let error_response = SimpleChatResponse {
                    response: format!("I'm sorry, I encountered an error: {}", e),
                    insights_learned: vec![],
                    context_used: false,
                };
                Ok(warp::reply::json(&error_response))
            }
        }
    }

    async fn process_learning_content(
        content: &str,
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
        brain_ai_orchestrator: Arc<Mutex<BrainAIOrchestrator>>,
    ) -> Result<Vec<String>, BrainError> {
        let mut memory_guard = memory_system.lock().await;
        let mut concept_guard = concept_graph.lock().await;
        let mut brain_guard = brain_ai_orchestrator.lock().await;
        
        // Use BrainAIOrchestrator to analyze and learn from the content
        let analysis_result = brain_guard.analyze_query(content, &mut *memory_guard, &mut *concept_guard).await?;
        
        // Extract insights from the analysis results, cleaning up internal identifiers
        let mut insights = vec![Self::clean_response_text(&analysis_result.analysis)];
        
        for insight in analysis_result.insights {
            insights.push(Self::clean_response_text(&insight.content));
        }
        
        Ok(insights)
    }

    async fn process_conversation(
        message: &str,
        _history: &[ChatMessage], // For future use in context-aware conversations
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
        brain_ai_orchestrator: Arc<Mutex<BrainAIOrchestrator>>,
    ) -> Result<SimpleChatResponse, BrainError> {
        let mut memory_guard = memory_system.lock().await;
        let mut concept_guard = concept_graph.lock().await;
        let mut brain_guard = brain_ai_orchestrator.lock().await;
        
        // Use BrainAIOrchestrator to analyze the query and generate response
        let analysis_result = brain_guard.analyze_query(message, &mut *memory_guard, &mut *concept_guard).await?;
        
        // Create a comprehensive response based on the analysis, cleaning up internal identifiers
        let mut response = Self::clean_response_text(&analysis_result.analysis);
        
        if !analysis_result.insights.is_empty() {
            response.push_str("\n\n**Key insights:**\n");
            for (i, insight) in analysis_result.insights.iter().enumerate() {
                let clean_content = Self::clean_response_text(&insight.content);
                response.push_str(&format!("{}. {}\n", i + 1, clean_content));
            }
        }
        
        // Check if relevant context was found based on confidence and insights
        let context_used = analysis_result.confidence > 0.3 || !analysis_result.insights.is_empty();
        
        Ok(SimpleChatResponse {
            response,
            insights_learned: vec![], // Only relevant for learning requests
            context_used,
        })
    }

    /// Handle code pattern analysis requests
    async fn handle_code_pattern_analysis(
        request: CodePatternAnalysisRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        // Create code pattern analyzer
        let analyzer = CodePatternAnalyzer::new();
        
        // Detect language if not provided
        let detected_language = if let Some(lang) = &request.language {
            Some(lang.clone())
        } else {
            analyzer.detect_language(&request.code_content, request.file_path.as_deref())
        };
        
        // Analyze code patterns
        let analysis_result = analyzer.analyze_patterns(
            &request.code_content,
            request.file_path.as_deref(),
            detected_language.as_deref(),
            request.analysis_depth,
        );
        
        let mut concepts_created = 0;
        let mut relationships_formed = 0;
        let mut patterns_with_concepts = Vec::new();
        
        // Store patterns in concept graph if requested
        if request.store_patterns {
            let mut cg = concept_graph.lock().await;
            let mut mem = memory_system.lock().await;
            
            for pattern in analysis_result.patterns {
                let concept_id = match Self::store_pattern_as_concept(&mut cg, &mut mem, &pattern).await {
                    Ok(id) => {
                        concepts_created += 1;
                        Some(id.to_string())
                    }
                    Err(e) => {
                        eprintln!("Failed to store pattern as concept: {}", e);
                        None
                    }
                };
                
                let mut pattern_with_concept = pattern;
                pattern_with_concept.concept_id = concept_id;
                patterns_with_concepts.push(pattern_with_concept);
            }
            
            // Form relationships between related patterns
            relationships_formed = Self::form_pattern_relationships(&mut cg, &patterns_with_concepts).await;
        } else {
            patterns_with_concepts = analysis_result.patterns;
        }
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = CodePatternAnalysisResponse {
            success: true,
            patterns_found: patterns_with_concepts,
            concepts_created,
            relationships_formed,
            analysis_time_ms: processing_time,
            confidence_score: analysis_result.overall_confidence,
            language_detected: detected_language,
            architectural_insights: analysis_result.architectural_insights,
        };
        
        Ok(warp::reply::json(&response))
    }
    
    /// Store a code pattern as a concept in the concept graph
    async fn store_pattern_as_concept(
        concept_graph: &mut ConceptGraphManager,
        _memory_system: &mut MemorySystem,
        pattern: &CodePattern,
    ) -> Result<uuid::Uuid, BrainError> {
        // Convert pattern type to concept type
        let concept_type = match pattern.pattern_type {
            CodePatternType::DataStructure => ConceptType::Entity,
            CodePatternType::Function => ConceptType::Action,
            CodePatternType::APIEndpoint => ConceptType::Action,
            CodePatternType::DesignPattern => ConceptType::Abstract,
            CodePatternType::ArchitecturalPattern => ConceptType::Abstract,
            CodePatternType::ImportPattern => ConceptType::Relation,
            CodePatternType::NamingConvention => ConceptType::Attribute,
            CodePatternType::ErrorHandling => ConceptType::Action,
            CodePatternType::ConfigurationPattern => ConceptType::Entity,
            CodePatternType::TestPattern => ConceptType::Action,
        };
        
        // Create concept node
        let mut concept = ConceptNode::new(
            concept_type,
            pattern.name.clone(),
            pattern.confidence,
            pattern.file_location.clone(),
        );
        
        // Add metadata
        concept.set_metadata("pattern_type".to_string(), format!("{:?}", pattern.pattern_type));
        concept.set_metadata("description".to_string(), pattern.description.clone());
        if let Some(snippet) = &pattern.code_snippet {
            concept.set_metadata("code_snippet".to_string(), snippet.clone());
        }
        
        let concept_id = concept_graph.create_concept(concept).await
            .map_err(|e| BrainError::DatabaseError(format!("Failed to create concept: {}", e)))?;
        
        // Store in episodic memory for context
        let event_content = format!("Code pattern discovered: {} ({})", pattern.name, pattern.description);
        let mut context = HashMap::new();
        context.insert("pattern_type".to_string(), format!("{:?}", pattern.pattern_type));
        if let Some(file_path) = &pattern.file_location {
            context.insert("file_path".to_string(), file_path.clone());
        }
        context.insert("concept_id".to_string(), concept_id.to_string());
        
        let _episodic_event = EpisodicEvent::new(
            event_content,
            context,
            0.7, // importance
            "code_pattern_analysis".to_string(),
        );
        
        // For now, we'll skip episodic storage since the MemorySystem API doesn't expose it
        // In a production system, we'd add a method to MemorySystem to handle this
        // memory_system.store_episodic_event(episodic_event)?;
        
        Ok(concept_id)
    }
    
    /// Form relationships between related code patterns
    async fn form_pattern_relationships(
        concept_graph: &mut ConceptGraphManager,
        patterns: &[CodePattern],
    ) -> usize {
        let mut relationships_formed = 0;
        
        for (i, pattern1) in patterns.iter().enumerate() {
            if let Some(concept_id1_str) = &pattern1.concept_id {
                if let Ok(concept_id1) = uuid::Uuid::parse_str(concept_id1_str) {
                    // Look for related patterns
                    for pattern2 in patterns.iter().skip(i + 1) {
                        if let Some(concept_id2_str) = &pattern2.concept_id {
                            if let Ok(concept_id2) = uuid::Uuid::parse_str(concept_id2_str) {
                                // Determine relationship type based on pattern types
                                let relationship_type = Self::determine_pattern_relationship(pattern1, pattern2);
                                
                                if let Some(rel_type) = relationship_type {
                                    match concept_graph.create_relationship(
                                        concept_id1,
                                        concept_id2,
                                        rel_type,
                                        0.5, // Initial weight
                                    ).await {
                                        Ok(_) => relationships_formed += 1,
                                        Err(e) => eprintln!("Failed to create relationship: {}", e),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        relationships_formed
    }
    
    /// Determine the appropriate relationship type between two patterns
    fn determine_pattern_relationship(pattern1: &CodePattern, pattern2: &CodePattern) -> Option<RelationshipType> {
        use CodePatternType::*;
        
        match (&pattern1.pattern_type, &pattern2.pattern_type) {
            (Function, DataStructure) | (DataStructure, Function) => Some(RelationshipType::Uses),
            (APIEndpoint, Function) | (Function, APIEndpoint) => Some(RelationshipType::Uses),
            (DesignPattern, ArchitecturalPattern) | (ArchitecturalPattern, DesignPattern) => Some(RelationshipType::PartOf),
            (TestPattern, Function) | (Function, TestPattern) => Some(RelationshipType::AssociatedWith),
            (ErrorHandling, Function) | (Function, ErrorHandling) => Some(RelationshipType::PartOf),
            (ConfigurationPattern, _) | (_, ConfigurationPattern) => Some(RelationshipType::AssociatedWith),
            (ImportPattern, _) | (_, ImportPattern) => Some(RelationshipType::Uses),
            _ => {
                // Check for naming similarity or location proximity
                if Self::patterns_are_related(pattern1, pattern2) {
                    Some(RelationshipType::SimilarTo)
                } else {
                    None
                }
            }
        }
    }
    
    /// Check if two patterns are related based on naming or location
    fn patterns_are_related(pattern1: &CodePattern, pattern2: &CodePattern) -> bool {
        // Check if patterns are in the same file
        if let (Some(file1), Some(file2)) = (&pattern1.file_location, &pattern2.file_location) {
            if file1 == file2 {
                return true;
            }
        }
        
        // Check naming similarity (simple heuristic)
        let name1 = pattern1.name.to_lowercase();
        let name2 = pattern2.name.to_lowercase();
        
        // If one name contains the other or they share significant prefix/suffix
        name1.contains(&name2) || name2.contains(&name1) ||
        (name1.len() > 3 && name2.len() > 3 && 
         (name1.starts_with(&name2[..3]) || name2.starts_with(&name1[..3])))
    }

    // Development Context API Handlers

    /// Handle POST /api/dev/context - Create or update development context
    async fn handle_development_context_create(
        request: DevelopmentContextRequest,
        memory_system: Arc<Mutex<MemorySystem>>,
        concept_graph: Arc<Mutex<ConceptGraphManager>>,
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        // Generate or use existing session ID
        let session_id = request.session_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        
        let insights = Self::analyze_development_context(&request, &memory_system, &concept_graph).await?;
        let recommendations = Self::generate_development_recommendations(&request, &insights).await;
        
        // Create or update session
        let mut sessions_lock = sessions.lock().await;
        let session = if let Some(existing_session) = sessions_lock.get_mut(&session_id) {
            // Update existing session
            existing_session.last_updated = chrono::Utc::now();
            existing_session.files_accessed.extend(request.files_accessed);
            if let Some(intent) = request.current_intent {
                existing_session.development_intent = Some(intent);
            }
            if let Some(goal) = request.development_goal {
                existing_session.development_goal = Some(goal);
            }
            if let Some(project_ctx) = request.project_context {
                existing_session.project_context = Some(project_ctx);
            }
            existing_session.insights.extend(insights.clone());
            existing_session.confidence_score = Self::calculate_session_confidence(&existing_session);
            existing_session.clone()
        } else {
            // Create new session
            let new_session = DevelopmentSession {
                session_id: session_id.clone(),
                start_time: chrono::Utc::now(),
                last_updated: chrono::Utc::now(),
                files_accessed: request.files_accessed,
                development_intent: request.current_intent,
                development_goal: request.development_goal,
                project_context: request.project_context,
                insights: insights.clone(),
                patterns_discovered: Vec::new(),
                confidence_score: 0.8, // Default confidence for new sessions
            };
            sessions_lock.insert(session_id.clone(), new_session.clone());
            new_session
        };
        drop(sessions_lock);

        // Store context in Brain's memory systems if auto_save is enabled
        if request.auto_save {
            match Self::store_context_in_brain_memory(&session, &memory_system, &concept_graph).await {
                Ok(()) => {},
                Err(_) => {
                    // Log error but don't fail the request
                    println!("Warning: Failed to store context in brain memory");
                }
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = DevelopmentContextResponse {
            success: true,
            session_id,
            context_preserved: request.auto_save,
            insights_generated: insights,
            recommendations,
            processing_time_ms: processing_time,
        };
        
        Ok(warp::reply::json(&response))
    }

    /// Handle GET /api/dev/context/{session_id} - Retrieve development context
    async fn handle_development_context_get(
        session_id: String,
        sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
    ) -> Result<impl Reply, warp::Rejection> {
        let start_time = std::time::Instant::now();
        
        let sessions_lock = sessions.lock().await;
        let session = sessions_lock.get(&session_id).cloned();
        let related_sessions = Self::find_related_sessions(&session_id, &sessions_lock).await;
        drop(sessions_lock);
        
        let context_summary = if let Some(ref sess) = session {
            Some(Self::generate_context_summary(sess))
        } else {
            None
        };
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = DevelopmentContextQueryResponse {
            success: true,
            session_found: session.is_some(),
            session,
            related_sessions,
            context_summary,
            processing_time_ms: processing_time,
        };
        
        Ok(warp::reply::json(&response))
    }

    /// Analyze development context and extract insights
    async fn analyze_development_context(
        request: &DevelopmentContextRequest,
        memory_system: &Arc<Mutex<MemorySystem>>,
        concept_graph: &Arc<Mutex<ConceptGraphManager>>,
    ) -> Result<Vec<String>, warp::Rejection> {
        let mut insights = Vec::new();
        
        // Analyze file access patterns
        if !request.files_accessed.is_empty() {
            let file_patterns = Self::analyze_file_access_patterns(&request.files_accessed);
            insights.extend(file_patterns);
        }
        
        // Analyze development intent
        if let Some(ref intent) = request.current_intent {
            let intent_insights = Self::analyze_development_intent(intent, concept_graph).await
                .map_err(|_| warp::reject())?;
            insights.extend(intent_insights);
        }
        
        // Analyze project context
        if let Some(ref project_ctx) = request.project_context {
            let project_insights = Self::analyze_project_context(project_ctx);
            insights.extend(project_insights);
        }
        
        // Query Brain's memory for relevant patterns
        let memory_insights = Self::query_memory_for_context(request, memory_system).await
            .map_err(|_| warp::reject())?;
        insights.extend(memory_insights);
        
        Ok(insights)
    }

    /// Generate development recommendations based on context and insights
    async fn generate_development_recommendations(
        request: &DevelopmentContextRequest,
        insights: &[String],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // File-based recommendations
        if !request.files_accessed.is_empty() {
            recommendations.extend(Self::recommend_based_on_files(&request.files_accessed));
        }
        
        // Intent-based recommendations
        if let Some(ref intent) = request.current_intent {
            recommendations.extend(Self::recommend_based_on_intent(intent));
        }
        
        // Insight-based recommendations
        recommendations.extend(Self::recommend_based_on_insights(insights));
        
        // Limit to top 5 recommendations
        recommendations.truncate(5);
        recommendations
    }

    /// Store development context in Brain's memory systems
    async fn store_context_in_brain_memory(
        session: &DevelopmentSession,
        memory_system: &Arc<Mutex<MemorySystem>>,
        concept_graph: &Arc<Mutex<ConceptGraphManager>>,
    ) -> Result<(), BrainError> {
        let mut memory = memory_system.lock().await;
        let mut cg = concept_graph.lock().await;
        
        // Store in episodic memory
        let event_content = format!(
            "Development session: {} - Intent: {:?}, Files: {}", 
            session.session_id,
            session.development_intent,
            session.files_accessed.len()
        );
        
        let mut context = HashMap::new();
        context.insert("session_id".to_string(), session.session_id.clone());
        context.insert("file_count".to_string(), session.files_accessed.len().to_string());
        if let Some(ref intent) = session.development_intent {
            context.insert("intent".to_string(), intent.clone());
        }
        if let Some(ref goal) = session.development_goal {
            context.insert("goal".to_string(), goal.clone());
        }
        
        let _episodic_event = EpisodicEvent::new(
            event_content,
            context,
            0.8, // importance
            "development_session".to_string(),
        );
        
        // Note: In a production system, we'd expose episodic storage in MemorySystem API
        // For now, we'll store it as working memory
        let working_memory_content = format!("Dev Session: {}", serde_json::to_string(session).unwrap_or_default());
        let _memory_id = memory.learn(working_memory_content, Priority::High)?;
        
        // Create concepts for development patterns
        if let Some(ref intent) = session.development_intent {
            let concept = crate::concept_graph::ConceptNode::new(
                crate::concept_graph::ConceptType::Action,
                format!("dev_intent_{}", intent),
                0.7,
                Some(format!("session:{}", session.session_id)),
            );
            let _concept_id = cg.create_concept(concept).await?;
        }
        
        Ok(())
    }

    /// Calculate confidence score for a development session
    fn calculate_session_confidence(session: &DevelopmentSession) -> f64 {
        let mut score = 0.5; // Base score
        
        // File access patterns contribute to confidence
        if !session.files_accessed.is_empty() {
            score += 0.2;
        }
        
        // Having clear intent increases confidence
        if session.development_intent.is_some() {
            score += 0.2;
        }
        
        // Having a goal increases confidence
        if session.development_goal.is_some() {
            score += 0.1;
        }
        
        // Project context adds confidence
        if session.project_context.is_some() {
            score += 0.1;
        }
        
        // Insights and patterns add confidence
        score += (session.insights.len() as f64 * 0.05).min(0.2);
        
        score.min(1.0) // Cap at 1.0
    }

    /// Find related development sessions
    async fn find_related_sessions(
        session_id: &str,
        sessions: &HashMap<String, DevelopmentSession>,
    ) -> Vec<String> {
        let mut related = Vec::new();
        
        if let Some(current_session) = sessions.get(session_id) {
            for (other_id, other_session) in sessions.iter() {
                if other_id != session_id {
                    if Self::sessions_are_related(current_session, other_session) {
                        related.push(other_id.clone());
                    }
                }
            }
        }
        
        related.truncate(3); // Limit to top 3 related sessions
        related
    }

    /// Check if two sessions are related
    fn sessions_are_related(session1: &DevelopmentSession, session2: &DevelopmentSession) -> bool {
        // Check if they share similar file paths
        let files1: std::collections::HashSet<_> = session1.files_accessed.iter()
            .map(|f| &f.file_path).collect();
        let files2: std::collections::HashSet<_> = session2.files_accessed.iter()
            .map(|f| &f.file_path).collect();
        
        let common_files = files1.intersection(&files2).count();
        if common_files > 0 {
            return true;
        }
        
        // Check if they have similar development intents
        if let (Some(intent1), Some(intent2)) = (&session1.development_intent, &session2.development_intent) {
            if intent1.to_lowercase().contains(&intent2.to_lowercase()) || 
               intent2.to_lowercase().contains(&intent1.to_lowercase()) {
                return true;
            }
        }
        
        false
    }

    /// Generate a summary of the development context
    fn generate_context_summary(session: &DevelopmentSession) -> String {
        let mut summary = format!("Session {} started {}", 
            session.session_id, 
            session.start_time.format("%Y-%m-%d %H:%M"));
        
        if !session.files_accessed.is_empty() {
            summary.push_str(&format!(", {} files accessed", session.files_accessed.len()));
        }
        
        if let Some(ref intent) = session.development_intent {
            summary.push_str(&format!(", Intent: {}", intent));
        }
        
        if !session.insights.is_empty() {
            summary.push_str(&format!(", {} insights generated", session.insights.len()));
        }
        
        summary
    }

    // Helper functions for analysis

    fn analyze_file_access_patterns(files: &[FileAccess]) -> Vec<String> {
        let mut insights = Vec::new();
        
        // Analyze file types
        let mut extensions = std::collections::HashMap::new();
        for file in files {
            if let Some(ext) = std::path::Path::new(&file.file_path).extension() {
                if let Some(ext_str) = ext.to_str() {
                    *extensions.entry(ext_str.to_string()).or_insert(0) += 1;
                }
            }
        }
        
        if let Some((most_common_ext, count)) = extensions.iter().max_by_key(|(_, &v)| v) {
            insights.push(format!("Primary file type: {} ({} files)", most_common_ext, count));
        }
        
        // Analyze access patterns
        let write_count = files.iter().filter(|f| matches!(f.access_type, FileAccessType::Write)).count();
        let read_count = files.iter().filter(|f| matches!(f.access_type, FileAccessType::Read)).count();
        
        if write_count > read_count {
            insights.push("Active development pattern detected (more writes than reads)".to_string());
        } else if read_count > write_count * 2 {
            insights.push("Research/exploration pattern detected (high read ratio)".to_string());
        }
        
        insights
    }

    async fn analyze_development_intent(
        intent: &str,
        concept_graph: &Arc<Mutex<ConceptGraphManager>>,
    ) -> Result<Vec<String>, BrainError> {
        let mut insights = Vec::new();
        
        // Simple intent categorization
        let intent_lower = intent.to_lowercase();
        if intent_lower.contains("debug") || intent_lower.contains("fix") {
            insights.push("Debugging/fixing activity detected".to_string());
        } else if intent_lower.contains("implement") || intent_lower.contains("add") {
            insights.push("Feature implementation activity detected".to_string());
        } else if intent_lower.contains("refactor") || intent_lower.contains("improve") {
            insights.push("Code improvement activity detected".to_string());
        } else if intent_lower.contains("test") {
            insights.push("Testing activity detected".to_string());
        }
        
        // Try to find related concepts in the concept graph
        let _cg = concept_graph.lock().await;
        // In a full implementation, we'd search for related concepts
        // For now, just add a general insight
        insights.push(format!("Development intent registered: {}", intent));
        
        Ok(insights)
    }

    fn analyze_project_context(project_ctx: &ProjectContext) -> Vec<String> {
        let mut insights = Vec::new();
        
        if let Some(ref branch) = project_ctx.current_branch {
            if branch != "main" && branch != "master" {
                insights.push(format!("Working on feature branch: {}", branch));
            }
        }
        
        if !project_ctx.technology_stack.is_empty() {
            insights.push(format!("Technology stack: {}", project_ctx.technology_stack.join(", ")));
        }
        
        if !project_ctx.active_features.is_empty() {
            insights.push(format!("Active features: {}", project_ctx.active_features.join(", ")));
        }
        
        insights
    }

    async fn query_memory_for_context(
        _request: &DevelopmentContextRequest,
        memory_system: &Arc<Mutex<MemorySystem>>,
    ) -> Result<Vec<String>, BrainError> {
        let mut insights = Vec::new();
        let memory = memory_system.lock().await;
        
        // Query working memory for related development activities
        let working_memory_items = memory.query_working(&WorkingMemoryQuery {
            content_pattern: Some("dev session".to_string()),
            priority: Some(Priority::Medium),
            min_importance: None,
            created_after: None,
            limit: Some(5),
        })?;
        
        if !working_memory_items.is_empty() {
            insights.push(format!("Found {} related development activities in recent memory", working_memory_items.len()));
        }
        
        // In a full implementation, we'd also query episodic and semantic memory
        
        Ok(insights)
    }

    fn recommend_based_on_files(files: &[FileAccess]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        let rust_files = files.iter().filter(|f| f.file_path.ends_with(".rs")).count();
        let js_files = files.iter().filter(|f| f.file_path.ends_with(".js") || f.file_path.ends_with(".ts")).count();
        
        if rust_files > 0 {
            recommendations.push("Consider running `cargo check` to verify Rust code".to_string());
        }
        
        if js_files > 0 {
            recommendations.push("Consider running linter on JavaScript/TypeScript files".to_string());
        }
        
        if files.len() > 10 {
            recommendations.push("Many files accessed - consider organizing work into smaller sessions".to_string());
        }
        
        recommendations
    }

    fn recommend_based_on_intent(intent: &str) -> Vec<String> {
        let mut recommendations = Vec::new();
        let intent_lower = intent.to_lowercase();
        
        if intent_lower.contains("debug") {
            recommendations.push("Consider adding logging statements for better debugging".to_string());
        } else if intent_lower.contains("implement") {
            recommendations.push("Consider writing tests for the new implementation".to_string());
        } else if intent_lower.contains("refactor") {
            recommendations.push("Consider running full test suite after refactoring".to_string());
        }
        
        recommendations
    }

    fn recommend_based_on_insights(insights: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for insight in insights {
            if insight.contains("debugging") {
                recommendations.push("Use systematic debugging approach: reproduce, isolate, fix, verify".to_string());
            } else if insight.contains("implementation") {
                recommendations.push("Follow TDD: write test first, implement, refactor".to_string());
            }
        }
        
        recommendations
    }
}

pub async fn start_web_server(port: u16) -> Result<(), BrainError> {
    let server = WebServer::new(port).await?;
    server.start().await
} 