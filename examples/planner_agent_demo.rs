use std::sync::Arc;
use std::collections::HashMap;
use brain_cognitive::{
    agents::{traits::*, development::PlannerAgent},
    meta::*,
    conversation::*,
};

// Simple mock implementations for testing
#[derive(Clone)]
struct MockMetaMemoryRepository;

#[async_trait::async_trait]
impl MetaMemoryRepository for MockMetaMemoryRepository {
    async fn store_item(&mut self, _item: MetaMemoryItem) -> MetaMemoryResult<uuid::Uuid> {
        Ok(uuid::Uuid::new_v4())
    }
    
    async fn get_item(&self, _id: uuid::Uuid) -> MetaMemoryResult<Option<MetaMemoryItem>> {
        Ok(None)
    }
    
    async fn get_item_by_component(&self, _component_id: uuid::Uuid) -> MetaMemoryResult<Option<MetaMemoryItem>> {
        Ok(None)
    }
    
    async fn query_items(&self, _query: &MetaMemoryQuery) -> MetaMemoryResult<Vec<MetaMemoryItem>> {
        Ok(vec![])
    }
    
    async fn remove_item(&mut self, _id: uuid::Uuid) -> MetaMemoryResult<bool> {
        Ok(false)
    }
    
    async fn batch_update(&mut self, _items: Vec<MetaMemoryItem>) -> MetaMemoryResult<Vec<uuid::Uuid>> {
        Ok(vec![])
    }
    
    async fn count_items(&self) -> MetaMemoryResult<usize> {
        Ok(0)
    }
    
    async fn clear_all(&mut self) -> MetaMemoryResult<usize> {
        Ok(0)
    }
}

#[derive(Clone)]
struct MockConversationService;

#[async_trait::async_trait]
impl ConversationService for MockConversationService {
    async fn process_conversation(
        &mut self,
        _request: RagRequest,
        _memory_repo: &mut dyn brain_core::memory::WorkingMemoryRepository,
        _concept_repo: &mut dyn brain_core::concepts::ConceptRepository,
        _insight_repo: &mut dyn brain_core::insights::InsightRepository,
    ) -> Result<RagResponse, brain_types::BrainError> {
        Ok(RagResponse {
            response: "Mock response".to_string(),
            conversation_id: "mock-id".to_string(),
            context_used: vec![],
            confidence_score: 0.8,
            response_quality: response_quality::ResponseQuality::default(),
        })
    }

    fn get_conversation_stats(&self) -> HashMap<String, usize> {
        HashMap::new()
    }

    fn clear_conversation(&mut self, _conversation_id: &str) -> bool {
        true
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 Brain AI - PlannerAgent Demo");
    println!("=================================\n");

    // Create project context
    let project_context = ProjectContext {
        project_name: "Task Management App".to_string(),
        project_version: "1.0.0".to_string(),
        project_description: Some("A comprehensive task management application".to_string()),
        tech_stack: vec!["React".to_string(), "Node.js".to_string(), "PostgreSQL".to_string()],
        git_branch: Some("main".to_string()),
        git_commit: None,
        active_files: vec![],
        recent_changes: vec![],
        directory_structure: HashMap::new(),
    };

    // Create cognitive profile
    let cognitive_profile = CognitivePreferenceProfile {
        interaction_mode: InteractionMode::Collaborative,
        detail_level: DetailLevel::Comprehensive,
        emotional_sensitivity: EmotionalSensitivity::Medium,
        autonomy_level: AutonomyLevel::SemiAuto,
        communication_style: brain_cognitive::agents::traits::CommunicationStyle::Technical,
        cognitive_load_settings: CognitiveLoadSettings {
            max_items_per_chunk: 8,
            pacing_preference: PacingPreference::Medium,
            progressive_disclosure: true,
        },
    };

    // Create simple cognitive context
    let meta_memory: Arc<dyn MetaMemoryRepository> = Arc::new(MockMetaMemoryRepository);
    let conversation_service: Arc<dyn ConversationService> = Arc::new(MockConversationService);
    
    let context = CognitiveContext {
        meta_memory,
        conversation_service,
        project_context,
        cognitive_profile,
        session_history: Vec::new(),
        config: HashMap::new(),
        working_directory: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
    };

    println!("✅ Cognitive context initialized");
    println!("   Project: {}", context.project_context.project_name);
    println!("   Tech Stack: {:?}", context.project_context.tech_stack);
    println!("   Interaction Mode: {:?}", context.cognitive_profile.interaction_mode);
    println!("   Detail Level: {:?}", context.cognitive_profile.detail_level);
    println!();

    // Create and test PlannerAgent
    let planner = PlannerAgent::new();
    
    println!("🎯 PlannerAgent Metadata:");
    println!("   Name: {}", planner.metadata().name);
    println!("   Persona: {}", planner.metadata().persona);
    println!("   Capabilities: {:?}", planner.metadata().capabilities);
    println!("   Supported Inputs: {:?}", planner.metadata().supported_input_types);
    println!("   Base Confidence: {:.2}", planner.metadata().base_confidence);
    println!();

    // Test project idea planning
    let project_idea = r#"
    Create a modern task management web application that allows teams to collaborate effectively.
    
    Requirements:
    - Users must be able to create, edit, and delete tasks
    - Tasks should have priorities, due dates, and assignees  
    - The system must support real-time collaboration
    - Users should receive notifications for task updates
    - The app must work on both desktop and mobile devices
    - Data should be stored securely with user authentication
    - The system should generate progress reports and analytics
    "#;

    let input = AgentInput::new(
        "project_idea".to_string(),
        project_idea.to_string(),
        "demo-session".to_string(),
    );

    println!("📋 Testing PlannerAgent with project idea...");
    
    // Assess confidence first
    let confidence = planner.assess_confidence(&input, &context).await?;
    println!("   Agent confidence: {:.2}", confidence);
    
    if confidence >= planner.confidence_threshold() {
        // Execute the planning
        let output = planner.execute(input, &context).await?;
        
        println!("✅ Planning completed successfully!");
        println!("   Output Type: {}", output.output_type);
        println!("   Confidence: {:.2}", output.confidence);
        println!("   Execution Time: {}ms", output.execution_metadata.execution_time_ms);
        
        if let Some(reasoning) = &output.reasoning {
            println!("   Reasoning: {}", reasoning);
        }
        
        println!("   Next Actions: {:?}", output.next_actions);
        println!();
        
        // Parse and display the structured plan
        if let Ok(plan) = serde_json::from_str::<serde_json::Value>(&output.content) {
            println!("📊 Generated Project Plan Summary:");
            
            if let Some(overview) = plan.get("project_overview") {
                println!("   Analysis Confidence: {:.2}", 
                         overview.get("analysis_confidence").and_then(|v| v.as_f64()).unwrap_or(0.0));
            }
            
            if let Some(task_breakdown) = plan.get("task_breakdown") {
                if let Some(total_hours) = task_breakdown.get("total_estimated_hours") {
                    println!("   Total Estimated Hours: {}", total_hours);
                }
                
                if let Some(phases) = task_breakdown.get("phases") {
                    println!("   Development Phases: {:?}", phases);
                }
                
                if let Some(tasks) = task_breakdown.get("tasks").and_then(|t| t.as_array()) {
                    println!("   Total Tasks Generated: {}", tasks.len());
                    for (i, task) in tasks.iter().enumerate() {
                        if let (Some(title), Some(phase)) = (
                            task.get("title").and_then(|t| t.as_str()),
                            task.get("phase").and_then(|p| p.as_str())
                        ) {
                            println!("     {}. {} [{}]", i + 1, title, phase);
                        }
                    }
                }
            }
            
            if let Some(roadmap) = plan.get("project_roadmap") {
                if let Some(timeline) = roadmap.get("timeline") {
                    if let Some(weeks) = timeline.get("estimated_duration_weeks") {
                        println!("   Estimated Duration: {} weeks", weeks);
                    }
                }
                
                if let Some(milestones) = roadmap.get("milestones").and_then(|m| m.as_array()) {
                    println!("   Key Milestones: {}", milestones.len());
                }
                
                if let Some(risks) = roadmap.get("risks").and_then(|r| r.as_array()) {
                    println!("   Identified Risks: {}", risks.len());
                }
            }
            
            if let Some(recommendations) = plan.get("recommendations").and_then(|r| r.as_array()) {
                println!("\n💡 Key Recommendations:");
                for (i, rec) in recommendations.iter().enumerate() {
                    if let Some(rec_str) = rec.as_str() {
                        println!("   {}. {}", i + 1, rec_str);
                    }
                }
            }
        }
        
    } else {
        println!("❌ Agent confidence ({:.2}) below threshold ({:.2})", 
                 confidence, planner.confidence_threshold());
    }

    // Test additional input types
    println!("\n📝 Testing user story breakdown...");
    let user_story = "As a project manager, I want to create and assign tasks to team members so that I can track project progress effectively.";
    
    let story_input = AgentInput::new(
        "user_story".to_string(),
        user_story.to_string(),
        "demo-session".to_string(),
    );
    
    let story_output = planner.execute(story_input, &context).await?;
    println!("   Story breakdown confidence: {:.2}", story_output.confidence);
    
    if let Ok(story_plan) = serde_json::from_str::<serde_json::Value>(&story_output.content) {
        if let Some(breakdown) = story_plan.get("story_breakdown") {
            if let Some(effort) = breakdown.get("estimated_effort") {
                println!("   Estimated Effort: {}", effort);
            }
            if let Some(complexity) = breakdown.get("complexity") {
                println!("   Complexity: {}", complexity);
            }
        }
    }

    println!("\n🎉 PlannerAgent demo completed!");
    Ok(())
} 