use std::collections::HashMap;
use std::sync::Arc;
use std::path::PathBuf;
use serde::Deserialize;
use brain_types::error::BrainError;
use crate::meta::MetaMemoryRepository;
use crate::conversation::ConversationService;
use crate::agents::traits::{
    AgentOutput, ProjectContext, CognitivePreferenceProfile, 
    InteractionMode, DetailLevel, AutonomyLevel, BrainResult
};

/// Builder for creating cognitive context
pub struct CognitiveContextBuilder {
    meta_memory: Option<Arc<dyn MetaMemoryRepository>>,
    conversation_service: Option<Arc<dyn ConversationService>>,
    project_context: Option<ProjectContext>,
    cognitive_profile: Option<CognitivePreferenceProfile>,
    session_history: Vec<AgentOutput>,
    config: HashMap<String, serde_json::Value>,
    working_directory: Option<PathBuf>,
}

impl CognitiveContextBuilder {
    /// Create a new context builder
    pub fn new() -> Self {
        Self {
            meta_memory: None,
            conversation_service: None,
            project_context: None,
            cognitive_profile: None,
            session_history: Vec::new(),
            config: HashMap::new(),
            working_directory: None,
        }
    }
    
    /// Set the meta-memory repository
    pub fn with_meta_memory(mut self, meta_memory: Arc<dyn MetaMemoryRepository>) -> Self {
        self.meta_memory = Some(meta_memory);
        self
    }
    
    /// Set the conversation service
    pub fn with_conversation_service(mut self, service: Arc<dyn ConversationService>) -> Self {
        self.conversation_service = Some(service);
        self
    }
    
    /// Set the project context
    pub fn with_project_context(mut self, context: ProjectContext) -> Self {
        self.project_context = Some(context);
        self
    }
    
    /// Set the cognitive preference profile
    pub fn with_cognitive_profile(mut self, profile: CognitivePreferenceProfile) -> Self {
        self.cognitive_profile = Some(profile);
        self
    }
    
    /// Set the session history
    pub fn with_session_history(mut self, history: Vec<AgentOutput>) -> Self {
        self.session_history = history;
        self
    }
    
    /// Add a configuration value
    pub fn with_config(mut self, key: String, value: serde_json::Value) -> Self {
        self.config.insert(key, value);
        self
    }
    
    /// Set the working directory
    pub fn with_working_directory(mut self, dir: PathBuf) -> Self {
        self.working_directory = Some(dir);
        self
    }
    
    /// Build the cognitive context
    pub fn build(self) -> BrainResult<CognitiveContext> {
        let meta_memory = self.meta_memory
            .ok_or_else(|| BrainError::ConfigError("Meta-memory repository is required".to_string()))?;
        
        let conversation_service = self.conversation_service
            .ok_or_else(|| BrainError::ConfigError("Conversation service is required".to_string()))?;
        
        let project_context = self.project_context
            .unwrap_or_else(|| ProjectContext::default());
        
        let cognitive_profile = self.cognitive_profile
            .unwrap_or_else(|| CognitivePreferenceProfile::default());
        
        let working_directory = self.working_directory
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        
        Ok(CognitiveContext {
            meta_memory,
            conversation_service,
            project_context,
            cognitive_profile,
            session_history: self.session_history,
            config: self.config,
            working_directory,
        })
    }
}

/// Shared context for agent execution
pub struct CognitiveContext {
    /// Access to meta-memory system
    pub meta_memory: Arc<dyn MetaMemoryRepository>,
    
    /// Conversation service for RAG and context
    pub conversation_service: Arc<dyn ConversationService>,
    
    /// Current project state and file system context
    pub project_context: ProjectContext,
    
    /// User's cognitive preference profile
    pub cognitive_profile: CognitivePreferenceProfile,
    
    /// Session tracking and agent interaction history
    pub session_history: Vec<AgentOutput>,
    
    /// Global configuration and settings
    pub config: HashMap<String, serde_json::Value>,
    
    /// Current working directory
    pub working_directory: PathBuf,
}

impl CognitiveContext {
    /// Create a new context builder
    pub fn builder() -> CognitiveContextBuilder {
        CognitiveContextBuilder::new()
    }
    
    /// Get a configuration value
    pub fn get_config<T>(&self, key: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.config.get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    /// Set a configuration value
    pub fn set_config(&mut self, key: String, value: serde_json::Value) {
        self.config.insert(key, value);
    }
    
    /// Add an agent output to the session history
    pub fn add_to_history(&mut self, output: AgentOutput) {
        self.session_history.push(output);
    }
    
    /// Get the last N outputs from session history
    pub fn get_recent_history(&self, n: usize) -> &[AgentOutput] {
        let start = if self.session_history.len() > n {
            self.session_history.len() - n
        } else {
            0
        };
        &self.session_history[start..]
    }
    
    /// Clear session history
    pub fn clear_history(&mut self) {
        self.session_history.clear();
    }
    
    /// Update the cognitive profile
    pub fn update_cognitive_profile(&mut self, profile: CognitivePreferenceProfile) {
        self.cognitive_profile = profile;
    }
    
    /// Get the preferred interaction mode
    pub fn interaction_mode(&self) -> &InteractionMode {
        &self.cognitive_profile.interaction_mode
    }
    
    /// Get the preferred detail level
    pub fn detail_level(&self) -> &DetailLevel {
        &self.cognitive_profile.detail_level
    }
    
    /// Get the autonomy level
    pub fn autonomy_level(&self) -> &AutonomyLevel {
        &self.cognitive_profile.autonomy_level
    }
    
    /// Check if the user prefers detailed responses
    pub fn prefers_detailed_responses(&self) -> bool {
        matches!(self.cognitive_profile.detail_level, DetailLevel::Detailed | DetailLevel::Comprehensive)
    }
    
    /// Check if the user prefers autonomous operation
    pub fn prefers_autonomous_operation(&self) -> bool {
        matches!(self.cognitive_profile.autonomy_level, AutonomyLevel::SemiAuto | AutonomyLevel::FullAuto)
    }
    
    /// Get the maximum items per chunk based on cognitive load settings
    pub fn max_items_per_chunk(&self) -> usize {
        self.cognitive_profile.cognitive_load_settings.max_items_per_chunk as usize
    }
    
    /// Check if progressive disclosure is enabled
    pub fn uses_progressive_disclosure(&self) -> bool {
        self.cognitive_profile.cognitive_load_settings.progressive_disclosure
    }
    
    /// Update project context
    pub fn update_project_context(&mut self, context: ProjectContext) {
        self.project_context = context;
    }
    
    /// Get project name
    pub fn project_name(&self) -> &str {
        &self.project_context.project_name
    }
    
    /// Get current git branch
    pub fn current_branch(&self) -> Option<&str> {
        self.project_context.git_branch.as_deref()
    }
    
    /// Get technology stack
    pub fn tech_stack(&self) -> &[String] {
        &self.project_context.tech_stack
    }
    
    /// Check if a technology is in the stack
    pub fn uses_technology(&self, tech: &str) -> bool {
        self.project_context.tech_stack.iter().any(|t| t.eq_ignore_ascii_case(tech))
    }
    
    /// Get active files
    pub fn active_files(&self) -> &[String] {
        &self.project_context.active_files
    }
    
    /// Add an active file
    pub fn add_active_file(&mut self, file_path: String) {
        if !self.project_context.active_files.contains(&file_path) {
            self.project_context.active_files.push(file_path);
        }
    }
    
    /// Remove an active file
    pub fn remove_active_file(&mut self, file_path: &str) {
        self.project_context.active_files.retain(|f| f != file_path);
    }
    
    /// Get recent changes
    pub fn recent_changes(&self) -> &[String] {
        &self.project_context.recent_changes
    }
    
    /// Add a recent change
    pub fn add_recent_change(&mut self, change: String) {
        self.project_context.recent_changes.push(change);
        
        // Keep only the last 50 changes
        if self.project_context.recent_changes.len() > 50 {
            self.project_context.recent_changes.remove(0);
        }
    }
    
    /// Get directory structure
    pub fn directory_structure(&self) -> &HashMap<String, Vec<String>> {
        &self.project_context.directory_structure
    }
    
    /// Update directory structure
    pub fn update_directory_structure(&mut self, structure: HashMap<String, Vec<String>>) {
        self.project_context.directory_structure = structure;
    }
    
    /// Get working directory
    pub fn working_directory(&self) -> &PathBuf {
        &self.working_directory
    }
    
    /// Set working directory
    pub fn set_working_directory(&mut self, dir: PathBuf) {
        self.working_directory = dir;
    }
}

impl Default for ProjectContext {
    fn default() -> Self {
        Self {
            project_name: "Unknown Project".to_string(),
            project_version: "0.1.0".to_string(),
            project_description: None,
            tech_stack: Vec::new(),
            git_branch: None,
            git_commit: None,
            active_files: Vec::new(),
            recent_changes: Vec::new(),
            directory_structure: HashMap::new(),
        }
    }
}

impl Default for CognitiveContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for creating common project contexts
impl ProjectContext {
    /// Create a project context for a Rust project
    pub fn rust_project(name: String, version: String) -> Self {
        Self {
            project_name: name,
            project_version: version,
            project_description: None,
            tech_stack: vec!["Rust".to_string(), "Cargo".to_string()],
            git_branch: None,
            git_commit: None,
            active_files: Vec::new(),
            recent_changes: Vec::new(),
            directory_structure: HashMap::new(),
        }
    }
    
    /// Create a project context for a JavaScript/Node.js project
    pub fn javascript_project(name: String, version: String) -> Self {
        Self {
            project_name: name,
            project_version: version,
            project_description: None,
            tech_stack: vec!["JavaScript".to_string(), "Node.js".to_string(), "npm".to_string()],
            git_branch: None,
            git_commit: None,
            active_files: Vec::new(),
            recent_changes: Vec::new(),
            directory_structure: HashMap::new(),
        }
    }
    
    /// Create a project context for a Python project
    pub fn python_project(name: String, version: String) -> Self {
        Self {
            project_name: name,
            project_version: version,
            project_description: None,
            tech_stack: vec!["Python".to_string(), "pip".to_string()],
            git_branch: None,
            git_commit: None,
            active_files: Vec::new(),
            recent_changes: Vec::new(),
            directory_structure: HashMap::new(),
        }
    }
    
    /// Add a technology to the stack
    pub fn with_technology(mut self, tech: String) -> Self {
        if !self.tech_stack.contains(&tech) {
            self.tech_stack.push(tech);
        }
        self
    }
    
    /// Set git information
    pub fn with_git(mut self, branch: Option<String>, commit: Option<String>) -> Self {
        self.git_branch = branch;
        self.git_commit = commit;
        self
    }
    
    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.project_description = Some(description);
        self
    }
} 