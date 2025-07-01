use std::sync::Arc;
use std::collections::HashMap;
use brain_cognitive::agents::{traits::*, development::ArchitectAgent};
use brain_cognitive::{
    meta::{MetaMemoryRepository, MetaMemoryItem, MetaMemoryQuery},
    conversation::{
        traits::ConversationService,
        RagRequest, RagResponse,
        ResponseQuality,
    },
};
use brain_core::{
    memory::WorkingMemoryRepository,
    concepts::ConceptRepository,
    insights::InsightRepository,
};
use brain_types::BrainError;
use async_trait::async_trait;
use uuid::Uuid;

// Mock implementation for MetaMemoryRepository
struct MockMetaMemoryRepository;

#[async_trait]
impl MetaMemoryRepository for MockMetaMemoryRepository {
    async fn store_item(&mut self, _item: MetaMemoryItem) -> Result<Uuid, brain_cognitive::meta::MetaMemoryError> {
        Ok(Uuid::new_v4())
    }
    
    async fn get_item(&self, _id: Uuid) -> Result<Option<MetaMemoryItem>, brain_cognitive::meta::MetaMemoryError> {
        Ok(None)
    }
    
    async fn get_item_by_component(&self, _component_id: Uuid) -> Result<Option<MetaMemoryItem>, brain_cognitive::meta::MetaMemoryError> {
        Ok(None)
    }
    
    async fn query_items(&self, _query: &MetaMemoryQuery) -> Result<Vec<MetaMemoryItem>, brain_cognitive::meta::MetaMemoryError> {
        Ok(Vec::new())
    }
    
    async fn remove_item(&mut self, _id: Uuid) -> Result<bool, brain_cognitive::meta::MetaMemoryError> {
        Ok(true)
    }
    
    async fn batch_update(&mut self, _items: Vec<MetaMemoryItem>) -> Result<Vec<Uuid>, brain_cognitive::meta::MetaMemoryError> {
        Ok(Vec::new())
    }
    
    async fn count_items(&self) -> Result<usize, brain_cognitive::meta::MetaMemoryError> {
        Ok(0)
    }
    
    async fn clear_all(&mut self) -> Result<usize, brain_cognitive::meta::MetaMemoryError> {
        Ok(0)
    }
}

// Mock implementation for ConversationService
struct MockConversationService;

#[async_trait]
impl ConversationService for MockConversationService {
    async fn process_conversation(
        &mut self,
        _request: RagRequest,
        _memory_repo: &mut dyn WorkingMemoryRepository,
        _concept_repo: &mut dyn ConceptRepository,
        _insight_repo: &mut dyn InsightRepository,
    ) -> Result<RagResponse, BrainError> {
        Ok(RagResponse {
            response: "Mock response".to_string(),
            conversation_id: "mock-conversation".to_string(),
            context_used: Vec::new(),
            confidence_score: 0.8,
            response_quality: ResponseQuality {
                factual_grounding: 0.8,
                coherence: 0.9,
                relevance: 0.8,
                safety_score: 1.0,
                source_attribution: 0.7,
                consistency_score: 0.8,
                completeness: 0.7,
                clarity: 0.9,
                toxicity_score: 0.0,
                bias_score: 0.0,
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️ ArchitectAgent Demo - System Architecture Design");
    println!("{}", "=".repeat(60));
    println!();

    // Initialize infrastructure components (simplified)
    let _config = brain_infra::config::BrainConfig::default();
    let _db_config = brain_infra::database::DatabaseConfig::default();

    // Create mock dependencies
    let meta_memory = Arc::new(MockMetaMemoryRepository);
    let conversation_service = Arc::new(MockConversationService);

    // Create project context
    let project_context = ProjectContext {
        project_name: "TaskFlow Pro".to_string(),
        project_version: "2.0.0".to_string(),
        project_description: Some("Advanced task management platform with real-time collaboration".to_string()),
        tech_stack: vec!["React".to_string(), "Node.js".to_string(), "PostgreSQL".to_string(), "Redis".to_string()],
        git_branch: Some("feature/architecture-redesign".to_string()),
        git_commit: Some("abc123def".to_string()),
        active_files: vec!["src/components/TaskBoard.tsx".to_string(), "src/api/tasks.ts".to_string()],
        recent_changes: vec!["Added real-time sync functionality".to_string()],
        directory_structure: {
            let mut map = HashMap::new();
            map.insert("src".to_string(), vec!["components".to_string(), "api".to_string(), "utils".to_string()]);
            map.insert("docs".to_string(), vec!["architecture.md".to_string(), "api.md".to_string()]);
            map
        },
    };

    // Create cognitive preference profile
    let cognitive_profile = CognitivePreferenceProfile {
        interaction_mode: InteractionMode::Collaborative,
        detail_level: DetailLevel::Detailed,
        emotional_sensitivity: EmotionalSensitivity::Medium,
        autonomy_level: AutonomyLevel::SemiAuto,
        communication_style: brain_cognitive::agents::traits::CommunicationStyle::Technical,
        cognitive_load_settings: CognitiveLoadSettings {
            max_items_per_chunk: 7,
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

    // Initialize ArchitectAgent
    let architect_agent = ArchitectAgent::new();
    println!("🏗️ Initializing ArchitectAgent...");
    println!("   Agent: {}", architect_agent.metadata().name);
    println!("   Persona: {}", architect_agent.metadata().persona);
    println!("   Capabilities: {:?}", architect_agent.metadata().capabilities);
    println!("   Base Confidence: {:.2}", architect_agent.metadata().base_confidence);
    println!();

    // Test Case 1: Project Requirements Analysis
    println!("📋 Test Case 1: Project Requirements Analysis");
    println!("{}", "-".repeat(50));
    
    let requirements_input = AgentInput::new(
        "project_plan".to_string(),
        r#"
        We need to design a scalable task management system that supports:
        - Real-time collaboration for teams of up to 100 users
        - Advanced project analytics and reporting
        - Integration with external tools (Slack, GitHub, Jira)
        - Mobile app support for iOS and Android
        - Enterprise-grade security and compliance
        - Multi-tenant architecture for SaaS deployment
        - Global deployment across multiple regions
        - 99.9% uptime guarantee
        "#.to_string(),
        "architect-demo-session".to_string(),
    );

    let confidence = architect_agent.assess_confidence(&requirements_input, &context).await?;
    println!("📊 Confidence Assessment: {:.2}", confidence);

    if confidence >= architect_agent.confidence_threshold() {
        println!("✅ Confidence threshold met, proceeding with architecture design...");
        let result = architect_agent.execute(requirements_input, &context).await?;
        
        println!("📐 Architecture Design Result:");
        println!("   Output Type: {}", result.output_type);
        println!("   Confidence: {:.2}", result.confidence);
        println!("   Execution Time: {}ms", result.execution_metadata.execution_time_ms);
        
        if let Some(reasoning) = &result.reasoning {
            println!("   Reasoning: {}", reasoning);
        }
        
        println!("   Next Actions: {:?}", result.next_actions);
        
        // Parse and display key architecture components
        if let Ok(arch_data) = serde_json::from_str::<serde_json::Value>(&result.content) {
            if let Some(system_arch) = arch_data.get("system_architecture") {
                if let Some(pattern) = system_arch.get("architecture_overview").and_then(|o| o.get("pattern")) {
                    println!("   🏗️ Recommended Pattern: {}", pattern.as_str().unwrap_or("N/A"));
                }
                if let Some(components) = system_arch.get("system_components") {
                    println!("   🧩 Key Components: {}", components.get("microservices").map(|v| v.to_string()).unwrap_or("N/A".to_string()));
                }
            }
        }
    } else {
        println!("❌ Confidence too low ({:.2}), skipping execution", confidence);
    }
    println!();

    // Test Case 2: Architecture Review
    println!("🔍 Test Case 2: Architecture Review");
    println!("{}", "-".repeat(50));
    
    let review_input = AgentInput::new(
        "architecture_review".to_string(),
        r#"
        Current architecture uses:
        - Monolithic Node.js application with Express
        - Single PostgreSQL database
        - Redis for session management
        - React frontend served from same server
        - Basic Docker deployment on single server
        
        Issues identified:
        - Performance bottlenecks under high load
        - Difficulty scaling individual components
        - Single point of failure
        - Manual deployment process
        "#.to_string(),
        "architect-demo-session".to_string(),
    );

    let review_result = architect_agent.execute(review_input, &context).await?;
    println!("🔍 Architecture Review Result:");
    println!("   Output Type: {}", review_result.output_type);
    println!("   Confidence: {:.2}", review_result.confidence);
    println!("   Execution Time: {}ms", review_result.execution_metadata.execution_time_ms);
    println!();

    // Test Case 3: Scalability Analysis
    println!("📈 Test Case 3: Scalability Requirements");
    println!("{}", "-".repeat(50));
    
    let scalability_input = AgentInput::new(
        "scalability_requirements".to_string(),
        r#"
        Expected growth:
        - 10,000 concurrent users within 6 months
        - 1M+ tasks processed daily
        - 100GB+ data storage requirements
        - Global user base requiring low latency
        - Peak loads during business hours (10x normal)
        "#.to_string(),
        "architect-demo-session".to_string(),
    );

    let scalability_result = architect_agent.execute(scalability_input, &context).await?;
    println!("📈 Scalability Analysis Result:");
    println!("   Output Type: {}", scalability_result.output_type);
    println!("   Confidence: {:.2}", scalability_result.confidence);
    println!("   Execution Time: {}ms", scalability_result.execution_metadata.execution_time_ms);
    println!();

    // Display agent capabilities summary
    println!("🎯 ArchitectAgent Capabilities Summary");
    println!("{}", "-".repeat(50));
    println!("✅ System architecture design and validation");
    println!("✅ Technology stack recommendations");
    println!("✅ Scalability and performance planning");
    println!("✅ Security architecture guidance");
    println!("✅ Deployment strategy design");
    println!("✅ API specification design");
    println!("✅ Data architecture planning");
    println!("✅ Component relationship modeling");
    println!("✅ Performance optimization strategies");
    println!("✅ Architecture pattern recommendations");
    println!();

    println!("🎉 ArchitectAgent Demo completed successfully!");
    Ok(())
} 