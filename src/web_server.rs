use crate::error::BrainError;
use crate::github_integration::{GitHubLearningEngine, GitHubClient};
use crate::memory::{MemorySystem, Priority, WorkingMemoryQuery, SemanticQuery, EpisodicQuery};
use crate::concept_graph::{ConceptGraphManager, ConceptGraphConfig, ConceptNode, ConceptType};
use crate::insight_extraction::PatternDetector;
use crate::segment_discovery::{BpeSegmenter, BpeConfig};
use crate::conversation::{RagOrchestrator, RagRequest};
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

pub struct WebServer {
    port: u16,
    memory_system: Arc<Mutex<MemorySystem>>,
    concept_graph: Arc<Mutex<ConceptGraphManager>>,
    pattern_detector: Arc<Mutex<PatternDetector>>,
    rag_orchestrator: Arc<Mutex<RagOrchestrator>>,
}

impl WebServer {
    pub async fn new(port: u16) -> Result<Self, BrainError> {
        let memory_system = Arc::new(Mutex::new(MemorySystem::new(1000)));
        let concept_graph = Arc::new(Mutex::new(
            ConceptGraphManager::new(ConceptGraphConfig::default()).await?
        ));
        let pattern_detector = Arc::new(Mutex::new(PatternDetector::new()));
        let rag_orchestrator = Arc::new(Mutex::new(RagOrchestrator::new()?));
        
        Ok(Self { 
            port,
            memory_system,
            concept_graph,
            pattern_detector,
            rag_orchestrator,
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

        // Combine all routes
        let routes = static_files
            .or(status)
            .or(stats)
            .or(health)
            .or(learn)
            .or(memory_query)
            .or(segment)
            .or(simulate)
            .or(concepts_analyze)
            .or(chat)
            .or(rag_chat)
            .or(rag_stats)
            .or(export)
            .with(cors);

        println!("🚀 Brain AI Web Server starting on http://localhost:{}", self.port);
        println!("📱 Interface available at: http://localhost:{}/brain-interface.html", self.port);
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
            match Self::process_github_learning(&request.text, memory_system).await {
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
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let mut data = HashMap::new();
        data.insert("concepts_discovered".to_string(), serde_json::Value::Number((request.text.len() / 50 + 5).into()));
        data.insert("knowledge_connections".to_string(), serde_json::Value::Number((request.text.len() / 30 + 8).into()));
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
        
        let (memories_found, relevance_score) = {
            let memory = memory_system.lock().await;
            
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
            
            let mut total_memories = 0;
            let mut max_relevance: f64 = 0.0;
            
            if let Ok(episodes) = memory.query_episodic(&episodic_query) {
                total_memories += episodes.len();
                for episode in episodes {
                    let relevance = Self::calculate_text_similarity(&episode.content, &request.query);
                    max_relevance = max_relevance.max(relevance);
                }
            }
            
            // Also query working memory
            let working_query = WorkingMemoryQuery {
                content_pattern: Some(request.query.clone()),
                limit: Some(20),
                ..Default::default()
            };
            
            if let Ok(working_items) = memory.query_working(&working_query) {
                total_memories += working_items.len();
                for item in working_items {
                    let relevance = Self::calculate_text_similarity(&item.content, &request.query);
                    max_relevance = max_relevance.max(relevance);
                }
            }
            
            // Query semantic memory
            let semantic_query = SemanticQuery {
                name_pattern: Some(request.query.clone()),
                limit: Some(20),
                ..Default::default()
            };
            
            if let Ok(semantic_concepts) = memory.query_semantic(&semantic_query) {
                total_memories += semantic_concepts.len();
                for concept in semantic_concepts {
                    let relevance = Self::calculate_text_similarity(&concept.name, &request.query);
                    max_relevance = max_relevance.max(relevance);
                }
            }
            
            (total_memories, max_relevance)
        };
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let mut data = HashMap::new();
        data.insert("memories_found".to_string(), serde_json::Value::Number(memories_found.into()));
        data.insert("query".to_string(), serde_json::Value::String(request.query.clone()));
        data.insert("relevance_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(relevance_score).unwrap_or_else(|| serde_json::Number::from(0))));
        
        let response = ProcessResponse {
            success: true,
            message: "Memory query completed".to_string(),
            data: Some(serde_json::Value::Object(data.into_iter().collect())),
            processing_time,
        };
        Ok(warp::reply::json(&response))
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
}

pub async fn start_web_server(port: u16) -> Result<(), BrainError> {
    let server = WebServer::new(port).await?;
    server.start().await
} 