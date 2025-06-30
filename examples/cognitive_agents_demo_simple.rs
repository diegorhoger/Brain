use std::sync::Arc;
use brain_cognitive::{
    agents::{
        traits::{
            BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitivePreferences,
            CognitivePreferenceProfile, ProjectContext, VerbosityLevel, InteractionMode,
            DetailLevel, EmotionalSensitivity, AutonomyLevel, CommunicationStyle,
            CognitiveLoadSettings, PacingPreference, ExecutionMetadata, ExecutionStatus,
            CognitiveContext
        },
        registry::{AgentRegistry, AgentQuery},
    },
    conversation::ConversationService,
    meta::MetaMemoryRepository,
    RagRequest, RagResponse,
};
use brain_infra::{
    config::BrainConfig,
    database::DatabaseConfig,
};
use brain::{WorkingMemoryRepository, ConceptRepository, InsightRepository};
use async_trait::async_trait;
use brain_types::error::BrainError;
use std::collections::HashMap;
use tokio;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Simple meta-memory repository implementation for demo
pub struct SimpleMetaMemoryRepository {
    items: Arc<RwLock<HashMap<Uuid, brain_cognitive::meta::MetaMemoryItem>>>,
    component_to_meta: Arc<RwLock<HashMap<Uuid, Uuid>>>,
}

impl SimpleMetaMemoryRepository {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            component_to_meta: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MetaMemoryRepository for SimpleMetaMemoryRepository {
    async fn store_item(&mut self, item: brain_cognitive::meta::MetaMemoryItem) -> brain_cognitive::meta::MetaMemoryResult<Uuid> {
        let mut items = self.items.write().await;
        let mut component_map = self.component_to_meta.write().await;
        
        let item_id = item.id;
        let component_id = item.component_id;
        
        items.insert(item_id, item);
        component_map.insert(component_id, item_id);
        
        Ok(item_id)
    }
    
    async fn get_item(&self, id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<Option<brain_cognitive::meta::MetaMemoryItem>> {
        let items = self.items.read().await;
        Ok(items.get(&id).cloned())
    }
    
    async fn get_item_by_component(&self, component_id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<Option<brain_cognitive::meta::MetaMemoryItem>> {
        let component_map = self.component_to_meta.read().await;
        if let Some(&meta_id) = component_map.get(&component_id) {
            self.get_item(meta_id).await
        } else {
            Ok(None)
        }
    }
    
    async fn query_items(&self, _query: &brain_cognitive::meta::MetaMemoryQuery) -> brain_cognitive::meta::MetaMemoryResult<Vec<brain_cognitive::meta::MetaMemoryItem>> {
        let items = self.items.read().await;
        Ok(items.values().cloned().collect())
    }
    
    async fn remove_item(&mut self, id: Uuid) -> brain_cognitive::meta::MetaMemoryResult<bool> {
        let mut items = self.items.write().await;
        Ok(items.remove(&id).is_some())
    }
    
    async fn batch_update(&mut self, items_to_update: Vec<brain_cognitive::meta::MetaMemoryItem>) -> brain_cognitive::meta::MetaMemoryResult<Vec<Uuid>> {
        let mut ids = Vec::new();
        for item in items_to_update {
            let id = self.store_item(item).await?;
            ids.push(id);
        }
        Ok(ids)
    }
    
    async fn count_items(&self) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        let items = self.items.read().await;
        Ok(items.len())
    }
    
    async fn clear_all(&mut self) -> brain_cognitive::meta::MetaMemoryResult<usize> {
        let mut items = self.items.write().await;
        let mut component_map = self.component_to_meta.write().await;
        let count = items.len();
        items.clear();
        component_map.clear();
        Ok(count)
    }
}

/// Example agent that processes code requests
#[derive(Clone)]
pub struct DemoCodeAgent {
    metadata: AgentMetadata,
    preferences: CognitivePreferences,
}

impl DemoCodeAgent {
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            id: "demo-code-agent".to_string(),
            name: "Demo Code Agent".to_string(),
            persona: "A helpful coding assistant that can analyze and generate code".to_string(),
            version: "1.0.0".to_string(),
            supported_input_types: vec!["code_request".to_string(), "code_analysis".to_string()],
            supported_output_types: vec!["code_response".to_string(), "analysis_report".to_string()],
            capabilities: vec![
                "code_generation".to_string(),
                "code_analysis".to_string(),
                "refactoring".to_string(),
            ],
            dependencies: vec![],
            tags: vec!["development".to_string(), "coding".to_string()],
            base_confidence: 0.8,
        };

        let preferences = CognitivePreferences {
            verbosity: VerbosityLevel::Standard,
            risk_tolerance: 0.7,
            collaboration_preference: 0.8,
            learning_enabled: true,
            adaptation_rate: 0.1,
        };

        Self { metadata, preferences }
    }
}

#[async_trait]
impl BrainAgent for DemoCodeAgent {
    async fn execute(
        &self,
        input: AgentInput,
        _context: &CognitiveContext,
    ) -> Result<AgentOutput, BrainError> {
        println!("🤖 Demo Code Agent executing with input: {}", input.input_type);
        
        let start_time = std::time::Instant::now();
        
        // Simulate some processing based on the input type
        let (content, confidence) = match input.input_type.as_str() {
            "code_request" => {
                let response = format!(
                    "Generated code for: {}\n\n```rust\nfn example() {{\n    println!(\"Hello, Brain AI!\");\n}}\n```",
                    input.content
                );
                (response, 0.85)
            }
            "code_analysis" => {
                let analysis = format!(
                    "Code analysis for: {}\n\nThe code appears to be well-structured with good practices.",
                    input.content
                );
                (analysis, 0.9)
            }
            _ => {
                return Err(BrainError::InvalidInput(format!(
                    "Unsupported input type: {}",
                    input.input_type
                )));
            }
        };

        let execution_time = start_time.elapsed().as_millis() as u64;

        let output = AgentOutput::new(
            self.metadata.id.clone(),
            "code_response".to_string(),
            content,
            confidence,
        )
        .with_reasoning("Processed code request using demo logic".to_string())
        .with_next_actions(vec!["test_code".to_string(), "review_code".to_string()]);

        // Update execution metadata
        let mut output = output;
        output.execution_metadata = ExecutionMetadata {
            execution_time_ms: execution_time,
            memory_usage_mb: 1.2,
            api_calls: 0,
            status: ExecutionStatus::Success,
            warnings: vec![],
        };

        println!("✅ Agent completed execution in {}ms", execution_time);
        Ok(output)
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn confidence_threshold(&self) -> f32 {
        0.6
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.preferences
    }

    async fn assess_confidence(
        &self,
        input: &AgentInput,
        _context: &CognitiveContext,
    ) -> Result<f32, BrainError> {
        // Simple confidence assessment based on input type
        let confidence = match input.input_type.as_str() {
            "code_request" => 0.85,
            "code_analysis" => 0.9,
            _ => 0.5,
        };
        Ok(confidence)
    }
}

/// Mock conversation service for demo
#[derive(Clone)]
pub struct MockConversationService;

#[async_trait]
impl ConversationService for MockConversationService {
    async fn process_conversation(
        &mut self,
        _request: RagRequest,
        _working_memory: &mut dyn WorkingMemoryRepository,
        _concept_repo: &mut dyn ConceptRepository,
        _insight_repo: &mut dyn InsightRepository,
    ) -> Result<RagResponse, BrainError> {
        Ok(RagResponse {
            response: "Mock conversation response".to_string(),
            conversation_id: "mock".to_string(),
            context_used: vec![],
            confidence_score: 0.8,
            response_quality: brain_cognitive::ResponseQuality {
                factual_grounding: 0.8,
                coherence: 0.85,
                relevance: 0.9,
                safety_score: 0.95,
                source_attribution: 0.7,
                consistency_score: 0.8,
                completeness: 0.8,
                clarity: 0.85,
                toxicity_score: 0.05,
                bias_score: 0.1,
                hallucination_risk: 0.1,
                confidence_calibration: 0.8,
            },
        })
    }

    fn get_conversation_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_conversations".to_string(), 1);
        stats
    }

    fn clear_conversation(&mut self, _conversation_id: &str) -> bool {
        true
    }
}

#[tokio::main]
async fn main() -> Result<(), BrainError> {
    println!("🧠 Brain AI Cognitive Agents Demo (Simplified)");
    println!("===============================================\n");

    // Initialize infrastructure components (simplified)
    let _config = BrainConfig::default();
    let _db_config = DatabaseConfig::default();
    let meta_memory: Arc<dyn MetaMemoryRepository> = 
        Arc::new(SimpleMetaMemoryRepository::new());
    let conversation_service: Arc<dyn ConversationService> = 
        Arc::new(MockConversationService);

    // Create project context
    let project_context = ProjectContext::rust_project(
        "Brain AI".to_string(),
        "0.8.0".to_string(),
    )
    .with_technology("Tokio".to_string())
    .with_technology("Brain AI".to_string())
    .with_description("Advanced cognitive AI system with agent-based architecture".to_string());

    // Create cognitive preference profile
    let cognitive_profile = CognitivePreferenceProfile {
        interaction_mode: InteractionMode::Collaborative,
        detail_level: DetailLevel::Standard,
        emotional_sensitivity: EmotionalSensitivity::Medium,
        autonomy_level: AutonomyLevel::SemiAuto,
        communication_style: CommunicationStyle::Technical,
        cognitive_load_settings: CognitiveLoadSettings {
            max_items_per_chunk: 5,
            pacing_preference: PacingPreference::Medium,
            progressive_disclosure: true,
        },
    };

    // Build cognitive context manually
    let mut config = HashMap::new();
    config.insert("demo_mode".to_string(), serde_json::Value::Bool(true));
    
    let context = CognitiveContext {
        meta_memory,
        conversation_service,
        project_context,
        cognitive_profile,
        session_history: Vec::new(),
        config,
        working_directory: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
    };

    println!("✅ Cognitive context initialized");
    println!("   Project: {}", context.project_context.project_name);
    println!("   Tech Stack: {:?}", context.project_context.tech_stack);
    println!("   Interaction Mode: {:?}", context.cognitive_profile.interaction_mode);
    println!("   Detail Level: {:?}", context.cognitive_profile.detail_level);
    println!();

    // Create agent registry
    let registry = AgentRegistry::new();
    
    // Create and register demo agent
    let demo_agent = Arc::new(DemoCodeAgent::new()) as Arc<dyn BrainAgent>;
    registry.register_agent(demo_agent.clone())?;
    
    println!("✅ Registered demo agent: {}", demo_agent.metadata().name);
    println!("   Capabilities: {:?}", demo_agent.metadata().capabilities);
    println!("   Confidence Threshold: {}", demo_agent.confidence_threshold());
    println!();

    // Demonstrate agent discovery
    println!("🔍 Agent Discovery Demo");
    println!("-----------------------");
    
    let query = AgentQuery::new()
        .with_input_type("code_request".to_string())
        .with_capability("code_generation".to_string());
    
    let discovered_agents = registry.discover_agents(&query)?;
    println!("Found {} agent(s) for code_request with code_generation capability", 
             discovered_agents.len());
    
    for agent in &discovered_agents {
        println!("  - {}: {}", agent.metadata().name, agent.metadata().persona);
    }
    println!();

    // Demonstrate agent execution
    println!("🚀 Agent Execution Demo");
    println!("-----------------------");
    
    let input = AgentInput::new(
        "code_request".to_string(),
        "Create a function to calculate fibonacci numbers".to_string(),
        "demo-session".to_string(),
    );
    
    println!("📝 Input: {} - {}", input.input_type, input.content);
    
    // Execute the agent
    if let Some(agent) = discovered_agents.first() {
        // Check confidence first
        let confidence = agent.assess_confidence(&input, &context).await?;
        println!("🎯 Agent confidence: {:.2}", confidence);
        
        if confidence >= agent.confidence_threshold() {
            let output = agent.execute(input.clone(), &context).await?;
            
            println!("📤 Output:");
            println!("   Type: {}", output.output_type);
            println!("   Confidence: {:.2}", output.confidence);
            println!("   Execution Time: {}ms", output.execution_metadata.execution_time_ms);
            println!("   Content Preview: {}", 
                     output.content.chars().take(100).collect::<String>());
            
            if let Some(reasoning) = &output.reasoning {
                println!("   Reasoning: {}", reasoning);
            }
            
            if !output.next_actions.is_empty() {
                println!("   Suggested Next Actions: {:?}", output.next_actions);
            }
        } else {
            println!("❌ Agent confidence ({:.2}) below threshold ({:.2})", 
                     confidence, agent.confidence_threshold());
        }
    }
    println!();

    // Demonstrate registry statistics
    println!("📊 Registry Statistics");
    println!("---------------------");
    let stats = registry.get_statistics()?;
    println!("Total Agents: {}", stats.total_agents);
    println!("Total Capabilities: {}", stats.total_capabilities);
    println!("Total Input Types: {}", stats.total_input_types);
    println!("Agents by Category: {:?}", stats.agents_by_category);
    println!();

    println!("🎉 Demo completed successfully!");
    println!("\n🚀 Phase 1 Complete: Core Agent Infrastructure");
    println!("==============================================");
    println!("✅ Agent trait system with async execution");
    println!("✅ Agent metadata and capability system");
    println!("✅ Cognitive preference profiles (CPP)");
    println!("✅ Agent registry with discovery");
    println!("✅ Cognitive context for shared execution environment");
    println!("✅ Integration with existing Brain AI infrastructure");
    
    println!("\n📋 Next Implementation Steps:");
    println!("1. Phase 2: Implement specialized agents (PlannerAgent, ArchitectAgent, etc.)");
    println!("2. Phase 3: Add agent orchestration with DAG execution engine");
    println!("3. Phase 4: Implement agent-specific memory and learning integration");
    println!("4. Phase 5: Add self-evolution and meta-agent capabilities");
    println!("5. Phase 6: Full cognitive preference adaptation and personalization");

    Ok(())
} 