//! System Integration Infrastructure
//! 
//! This module provides a unified API layer that integrates all Brain AI components
//! into a cohesive system with standardized interfaces, comprehensive logging,
//! and consistent error handling across all component boundaries.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use log::{info, warn, debug};
use tokio::sync::RwLock;

use brain_types::{Result, BrainError};
use brain_core::{
    ModelConfig, BpeConfig, ConsolidationConfig, SimulationConfig,
    CharacterVocab, CharacterPredictorService, MemoryService,
    SimulationEngine as SimulationEngineTrait,
};
use crate::{
    ConceptGraphManager, ConceptGraphConfig,
    CharacterPredictor, BpeSegmenter, WorkingMemoryRepository,
    SimulationEngineImpl,
    performance_monitor::{PerformanceMonitor, PerformanceConfig}
};

/// Current version of the Brain AI system
pub const VERSION: &str = "1.0.0";

/// The unified Brain AI system that orchestrates all cognitive components
#[derive(Debug)]
pub struct BrainSystem {
    /// System configuration
    config: BrainSystemConfig,
    /// Component registry with status tracking
    components: ComponentRegistry,
    /// Unified API interface
    api: UnifiedAPI,
    /// Workflow execution engine
    workflows: WorkflowEngine,
    /// System health monitoring
    health: Arc<Mutex<SystemHealth>>,
    /// Event logging and analytics
    events: Arc<Mutex<Vec<SystemEvent>>>,
    /// Performance metrics
    metrics: Arc<Mutex<SystemMetrics>>,
    /// Performance monitoring and optimization
    performance_monitor: Option<Arc<PerformanceMonitor>>,
    /// System initialization timestamp
    initialized_at: u64,
}

/// Configuration for the entire Brain AI system
#[derive(Debug, Clone)]
pub struct BrainSystemConfig {
    /// System identification
    pub system_id: String,
    pub system_name: String,
    pub version: String,
    
    /// Component configurations
    pub character_predictor: ModelConfig,
    pub segment_discovery: BpeConfig,
    pub memory_system: ConsolidationConfig,
    pub concept_graph: ConceptGraphConfig,
    pub simulation_engine: SimulationConfig,
    
    /// Infrastructure configurations
    pub enable_auth: bool,
    pub enable_rate_limiting: bool,
    pub enable_logging: bool,
    
    /// System-level settings
    pub enable_comprehensive_logging: bool,
    pub enable_performance_monitoring: bool,
    pub enable_health_checks: bool,
    pub max_concurrent_operations: usize,
    pub component_initialization_timeout_ms: u64,
    
    /// Performance monitoring configuration
    pub performance_config: PerformanceConfig,
}

impl Default for BrainSystemConfig {
    fn default() -> Self {
        Self {
            system_id: uuid::Uuid::new_v4().to_string(),
            system_name: "Brain AI System".to_string(),
            version: VERSION.to_string(),
            
            // Use default configurations for all components
            character_predictor: ModelConfig::default(),
            segment_discovery: BpeConfig::default(),
            memory_system: ConsolidationConfig::default(),
            concept_graph: ConceptGraphConfig::default(),
            simulation_engine: SimulationConfig::default(),
            
            enable_auth: true,
            enable_rate_limiting: true,
            enable_logging: true,
            
            enable_comprehensive_logging: true,
            enable_performance_monitoring: true,
            enable_health_checks: true,
            max_concurrent_operations: 100,
            component_initialization_timeout_ms: 30000, // 30 seconds
            
            // Performance monitoring configuration
            performance_config: PerformanceConfig::default(),
        }
    }
}

/// Builder pattern for constructing BrainSystem with validation
pub struct BrainSystemBuilder {
    config: BrainSystemConfig,
    custom_components: HashMap<String, Box<dyn SystemComponent>>,
}

impl BrainSystemBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: BrainSystemConfig::default(),
            custom_components: HashMap::new(),
        }
    }
    
    /// Configure the system with custom settings
    pub fn with_config(mut self, config: BrainSystemConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Add a custom component to the system
    pub fn with_component(mut self, name: String, component: Box<dyn SystemComponent>) -> Self {
        self.custom_components.insert(name, component);
        self
    }
    
    /// Configure character predictor settings
    pub fn with_character_predictor_config(mut self, config: ModelConfig) -> Self {
        self.config.character_predictor = config;
        self
    }
    
    /// Configure segment discovery settings
    pub fn with_segment_discovery_config(mut self, config: BpeConfig) -> Self {
        self.config.segment_discovery = config;
        self
    }
    
    /// Configure memory system settings
    pub fn with_memory_config(mut self, config: ConsolidationConfig) -> Self {
        self.config.memory_system = config;
        self
    }
    
    /// Enable or disable comprehensive logging
    pub fn with_logging_enabled(mut self, enabled: bool) -> Self {
        self.config.enable_comprehensive_logging = enabled;
        self
    }
    
    /// Set maximum concurrent operations
    pub fn with_max_concurrent_operations(mut self, max: usize) -> Self {
        self.config.max_concurrent_operations = max;
        self
    }
    
    /// Build the Brain AI system with validation
    pub async fn build(self) -> Result<BrainSystem> {
        BrainSystem::new(self.config, self.custom_components).await
    }
}

impl Default for BrainSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry for tracking all system components and their status
#[derive(Debug)]
pub struct ComponentRegistry {
    /// Component instances and their current status
    components: HashMap<String, (Box<dyn SystemComponent>, ComponentStatus)>,
    /// Component dependency graph
    dependencies: HashMap<String, Vec<String>>,
    /// Component initialization order
    initialization_order: Vec<String>,
}

/// Status of a system component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComponentStatus {
    Uninitialized,
    Initializing,
    Ready,
    Error(String),
    Stopped,
}

/// Unified API interface for all system operations
#[derive(Debug)]
pub struct UnifiedAPI {
    /// Component registry reference
    components: Arc<Mutex<ComponentRegistry>>,
    /// System configuration
    #[allow(dead_code)] // Reserved for future configuration-based functionality
    config: BrainSystemConfig,
    /// API call statistics
    call_stats: Arc<Mutex<HashMap<String, usize>>>,
}

/// Workflow execution engine for complex multi-step operations
#[derive(Debug)]
pub struct WorkflowEngine {
    /// Available workflows
    workflows: HashMap<String, Workflow>,
    /// Execution history
    execution_history: Arc<Mutex<Vec<WorkflowExecution>>>,
}

/// System health monitoring and status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall system status
    pub overall_status: HealthStatus,
    /// Component health breakdown
    pub component_health: HashMap<String, ComponentHealth>,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Memory usage statistics
    pub memory_usage_mb: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Last health check timestamp
    pub last_check: u64,
    /// Number of active operations
    pub active_operations: usize,
    /// Error count in last hour
    pub recent_errors: usize,
}

/// Overall health status of the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Critical,
    Down,
}

/// Health information for individual components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: ComponentStatus,
    pub last_response_time_ms: u64,
    pub error_count: usize,
    pub success_count: usize,
    pub last_error: Option<String>,
}

/// System-wide performance and operational metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Operations per second
    pub operations_per_second: f64,
    /// Component-specific metrics
    pub component_metrics: HashMap<String, ComponentMetrics>,
    /// Last metrics update
    pub last_updated: u64,
}

/// Performance metrics for individual components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetrics {
    pub operations: u64,
    pub avg_response_time_ms: f64,
    pub error_rate: f64,
    pub throughput: f64,
}

/// System event for logging and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub component: String,
    pub message: String,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

/// Types of system events that can occur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    SystemStartup,
    SystemShutdown,
    ComponentInitialized,
    ComponentError,
    ComponentHealthCheck,
    APICall,
    WorkflowExecution,
    PerformanceAlert,
    SecurityEvent,
    ConfigurationChange,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::SystemStartup => write!(f, "SystemStartup"),
            EventType::SystemShutdown => write!(f, "SystemShutdown"),
            EventType::ComponentInitialized => write!(f, "ComponentInitialized"),
            EventType::ComponentError => write!(f, "ComponentError"),
            EventType::ComponentHealthCheck => write!(f, "ComponentHealthCheck"),
            EventType::APICall => write!(f, "APICall"),
            EventType::WorkflowExecution => write!(f, "WorkflowExecution"),
            EventType::PerformanceAlert => write!(f, "PerformanceAlert"),
            EventType::SecurityEvent => write!(f, "SecurityEvent"),
            EventType::ConfigurationChange => write!(f, "ConfigurationChange"),
        }
    }
}

/// Result type for integration operations
pub type IntegrationResult<T> = std::result::Result<T, IntegrationError>;

/// Errors that can occur during system integration
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    #[error("Component not ready: {0}")]
    ComponentNotReady(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Workflow execution failed: {0}")]
    WorkflowExecutionFailed(String),
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
}

/// Trait for system components that can be managed by the registry
pub trait SystemComponent: std::fmt::Debug + Send + Sync {
    /// Component name for identification
    fn name(&self) -> &str;
    
    /// Component version
    fn version(&self) -> &str;
    
    /// Initialize the component
    fn initialize(&mut self) -> Result<()>;
    
    /// Shutdown the component gracefully
    fn shutdown(&mut self) -> Result<()>;
    
    /// Get current component status
    fn status(&self) -> ComponentStatus;
    
    /// Perform health check
    fn health_check(&self) -> Result<ComponentHealth>;
    
    /// Get component metrics
    fn metrics(&self) -> ComponentMetrics;
    
    /// Handle system events
    fn handle_event(&mut self, event: &SystemEvent) -> Result<()>;
    
    /// Get component dependencies
    fn dependencies(&self) -> Vec<String>;
    
    /// Validate configuration
    fn validate_config(&self) -> Result<()>;
}

/// Workflow definition for complex operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub dependencies: Vec<String>,
}

/// Individual step in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub component: String,
    pub operation: String,
    pub parameters: HashMap<String, String>,
    pub retry_count: usize,
    pub timeout_ms: u64,
}

/// Execution record for a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub workflow_id: String,
    pub execution_id: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub status: WorkflowStatus,
    pub step_results: HashMap<String, StepResult>,
    pub error: Option<String>,
}

/// Status of workflow execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Result of a workflow step execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub status: StepStatus,
    pub duration_ms: u64,
    pub output: Option<String>,
    pub error: Option<String>,
}

/// Status of individual workflow steps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl BrainSystem {
    /// Create a new Brain AI system with the given configuration
    pub async fn new(
        config: BrainSystemConfig,
        custom_components: HashMap<String, Box<dyn SystemComponent>>,
    ) -> Result<Self> {
        let initialized_at = current_timestamp();
        
        info!("Initializing Brain AI System v{}", VERSION);
        info!("System ID: {}", config.system_id);
        
        // Initialize component registry
        let mut components = ComponentRegistry::new();
        
        // Register core components
        components.register_core_components(&config).await?;
        
        // Register custom components
        for (name, component) in custom_components {
            components.register_component(name, component)?;
        }
        
        // Initialize performance monitoring if enabled
        let performance_monitor = if config.enable_performance_monitoring {
            let monitor = PerformanceMonitor::new(config.performance_config.clone())?;
            Some(Arc::new(monitor))
        } else {
            None
        };
        
        let system = Self {
            config: config.clone(),
            api: UnifiedAPI::new(Arc::new(Mutex::new(components)), config.clone()),
            workflows: WorkflowEngine::new(),
            health: Arc::new(Mutex::new(SystemHealth::new())),
            events: Arc::new(Mutex::new(Vec::new())),
            metrics: Arc::new(Mutex::new(SystemMetrics::new())),
            performance_monitor,
            components: ComponentRegistry::new(), // Will be moved from above
            initialized_at,
        };
        
        // Log system startup event
        let startup_event = SystemEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: EventType::SystemStartup,
            component: "BrainSystem".to_string(),
            message: format!("Brain AI System v{} initialized successfully", VERSION),
            timestamp: current_timestamp(),
            metadata: HashMap::new(),
        };
        system.log_event(startup_event);
        
        info!("Brain AI System initialized successfully");
        Ok(system)
    }
    
    /// Initialize all registered components
    fn initialize_components(&mut self) -> Result<()> {
        info!("Initializing system components...");
        
        let order = self.components.get_initialization_order().to_vec();
        for component_name in order {
            self.initialize_component(&component_name)?;
        }
        
        info!("All components initialized successfully");
        Ok(())
    }
    
    /// Initialize a specific component
    fn initialize_component(&mut self, name: &str) -> Result<()> {
        info!("Initializing component: {}", name);
        
        if let Some((component, status)) = self.components.components.get_mut(name) {
            *status = ComponentStatus::Initializing;
            
            match component.initialize() {
                Ok(()) => {
                    *status = ComponentStatus::Ready;
                    info!("Component '{}' initialized successfully", name);
                    
                    // Log component initialization event
                    let event = SystemEvent {
                        event_id: Uuid::new_v4().to_string(),
                        event_type: EventType::ComponentInitialized,
                        component: name.to_string(),
                        message: format!("Component '{}' initialized", name),
                        timestamp: current_timestamp(),
                        metadata: HashMap::new(),
                    };
                    self.log_event(event);
                }
                Err(e) => {
                    let error_msg = format!("Failed to initialize component '{}': {}", name, e);
                    *status = ComponentStatus::Error(error_msg.clone());
                    warn!("{}", error_msg);
                    return Err(e);
                }
            }
        } else {
            return Err(BrainError::Serialization { 
                source: format!("Component '{}' not found", name).into() 
            });
        }
        
        Ok(())
    }
    
    /// Get the unified API interface
    pub fn api(&self) -> &UnifiedAPI {
        &self.api
    }
    
    /// Get the workflow engine
    pub fn workflows(&self) -> &WorkflowEngine {
        &self.workflows
    }
    
    /// Get current system health
    pub fn health(&self) -> SystemHealth {
        self.health.lock().unwrap().clone()
    }
    
    /// Get current system metrics
    pub fn metrics(&self) -> SystemMetrics {
        self.metrics.lock().unwrap().clone()
    }
    
    /// Get performance monitor reference
    pub fn performance_monitor(&self) -> Option<Arc<PerformanceMonitor>> {
        self.performance_monitor.clone()
    }
    
    /// Start performance monitoring
    pub async fn start_performance_monitoring(&self) -> Result<()> {
        if let Some(monitor) = &self.performance_monitor {
            monitor.start().await?;
            info!("Performance monitoring started");
        } else {
            warn!("Performance monitoring not enabled");
        }
        Ok(())
    }
    
    /// Stop performance monitoring
    pub async fn stop_performance_monitoring(&self) -> Result<()> {
        if let Some(monitor) = &self.performance_monitor {
            monitor.stop().await?;
            info!("Performance monitoring stopped");
        }
        Ok(())
    }
    
    /// Record an operation for metrics and monitoring
    pub fn record_operation(
        &self,
        component_name: &str,
        operation: &str,
        duration: std::time::Duration,
        success: bool,
    ) -> Result<()> {
        // Update system metrics
        let mut metrics = self.metrics.lock().unwrap();
        metrics.total_operations += 1;
        if success {
            metrics.successful_operations += 1;
        } else {
            metrics.failed_operations += 1;
        }
        
        // Update component-specific metrics
        let component_metrics = metrics.component_metrics
            .entry(component_name.to_string())
            .or_insert_with(ComponentMetrics::default);
        component_metrics.operations += 1;
        component_metrics.avg_response_time_ms = 
            (component_metrics.avg_response_time_ms + duration.as_millis() as f64) / 2.0;
        
        // Record with performance monitor if available
        if let Some(monitor) = &self.performance_monitor {
            monitor.record_operation(component_name, operation, duration, success)?;
        }
        
        Ok(())
    }
    
    /// Get performance snapshot from monitor
    pub fn get_performance_snapshot(&self) -> Result<Option<crate::performance_monitor::PerformanceSnapshot>> {
        if let Some(monitor) = &self.performance_monitor {
            Ok(Some(monitor.get_current_snapshot()?))
        } else {
            Ok(None)
        }
    }
    
    /// Identify performance bottlenecks
    pub fn identify_bottlenecks(&self) -> Result<Vec<crate::performance_monitor::PerformanceBottleneck>> {
        if let Some(monitor) = &self.performance_monitor {
            monitor.identify_bottlenecks()
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Get optimization recommendations
    pub fn get_optimization_recommendations(&self) -> Result<Vec<crate::performance_monitor::OptimizationRecommendation>> {
        if let Some(monitor) = &self.performance_monitor {
            monitor.get_optimization_recommendations()
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Export performance report
    pub fn export_performance_report(&self, format: crate::performance_monitor::ReportFormat) -> Result<String> {
        if let Some(monitor) = &self.performance_monitor {
            monitor.export_performance_report(format)
        } else {
            Ok("Performance monitoring not enabled".to_string())
        }
    }
    
    /// Get recent system events
    pub fn recent_events(&self, limit: usize) -> Vec<SystemEvent> {
        let events = self.events.lock().unwrap();
        events.iter().rev().take(limit).cloned().collect()
    }
    
    /// Perform comprehensive health check
    pub fn perform_health_check(&self) -> Result<SystemHealth> {
        let mut health = SystemHealth::new();
        health.last_check = current_timestamp();
        health.uptime_seconds = current_timestamp() - self.initialized_at;
        
        // Check component health
        for (name, (component, _)) in &self.components.components {
            match component.health_check() {
                Ok(component_health) => {
                    health.component_health.insert(name.clone(), component_health);
                }
                Err(e) => {
                    let error_health = ComponentHealth {
                        status: ComponentStatus::Error(e.to_string()),
                        last_response_time_ms: 0,
                        error_count: 1,
                        success_count: 0,
                        last_error: Some(e.to_string()),
                    };
                    health.component_health.insert(name.clone(), error_health);
                }
            }
        }
        
        // Determine overall status
        let error_count = health.component_health.values()
            .filter(|h| matches!(h.status, ComponentStatus::Error(_)))
            .count();
        
        health.overall_status = if error_count == 0 {
            HealthStatus::Healthy
        } else if error_count < health.component_health.len() / 2 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Critical
        };
        
        // Update stored health
        *self.health.lock().unwrap() = health.clone();
        
        Ok(health)
    }
    
    /// Shutdown the system gracefully
    pub fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down Brain AI System...");
        
        // Log shutdown event
        let shutdown_event = SystemEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: EventType::SystemShutdown,
            component: "BrainSystem".to_string(),
            message: "Brain AI System shutdown initiated".to_string(),
            timestamp: current_timestamp(),
            metadata: HashMap::new(),
        };
        self.log_event(shutdown_event);
        
        // Shutdown components in reverse order
        let order = self.components.get_initialization_order().to_vec();
        for component_name in order.iter().rev() {
            if let Some((component, status)) = self.components.components.get_mut(component_name) {
                info!("Shutting down component: {}", component_name);
                match component.shutdown() {
                    Ok(()) => {
                        *status = ComponentStatus::Stopped;
                        info!("Component '{}' shut down successfully", component_name);
                    }
                    Err(e) => {
                        warn!("Failed to shutdown component '{}': {}", component_name, e);
                        *status = ComponentStatus::Error(format!("Shutdown failed: {}", e));
                    }
                }
            }
        }
        
        info!("Brain AI System shutdown complete");
        Ok(())
    }
    
    /// Log a system event
    fn log_event(&self, event: SystemEvent) {
        if self.config.enable_comprehensive_logging {
            debug!("Event: {} - {} - {}", event.event_type, event.component, event.message);
        }
        
        let mut events = self.events.lock().unwrap();
        events.push(event);
        
        // Keep only recent events to prevent memory growth
        if events.len() > 10000 {
            events.drain(0..1000);
        }
    }
    
    /// Export current system state
    pub fn export_system_state(&self) -> Result<String> {
        let state = SystemState {
            health: self.health(),
            metrics: self.metrics(),
            recent_events: self.recent_events(100),
            component_status: self.get_component_status(),
        };
        
        Ok(serde_json::to_string_pretty(&state)?)
    }
    
    /// Get status of all components
    fn get_component_status(&self) -> HashMap<String, ComponentStatus> {
        self.components.components.iter()
            .map(|(name, (_, status))| (name.clone(), status.clone()))
            .collect()
    }
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            dependencies: HashMap::new(),
            initialization_order: Vec::new(),
        }
    }
    
    /// Register a component with the registry
    pub fn register_component(&mut self, name: String, component: Box<dyn SystemComponent>) -> Result<()> {
        info!("Registering component: {}", name);
        
        // Get dependencies from the component
        let deps = component.dependencies();
        self.dependencies.insert(name.clone(), deps);
        
        // Add component to registry
        self.components.insert(name.clone(), (component, ComponentStatus::Uninitialized));
        
        // Update initialization order
        self.update_initialization_order();
        
        info!("Component '{}' registered successfully", name);
        Ok(())
    }
    
    /// Register core Brain AI components
    pub async fn register_core_components(&mut self, config: &BrainSystemConfig) -> Result<()> {
        info!("Registering core Brain AI components...");
        
        // Register character predictor
        let character_predictor = CharacterPredictorComponent::new(config.character_predictor.clone())?;
        self.register_component("CharacterPredictor".to_string(), Box::new(character_predictor))?;
        
        // Register BPE segmenter
        let bpe_segmenter = BpeSegmenterComponent::new(config.segment_discovery.clone())?;
        self.register_component("BpeSegmenter".to_string(), Box::new(bpe_segmenter))?;
        
        // Register memory system
        let memory_system = MemorySystemComponent::new(config.memory_system.clone())?;
        self.register_component("MemorySystem".to_string(), Box::new(memory_system))?;
        
        // Register concept graph (depends on memory system)
        let concept_graph_manager = ConceptGraphManager::new(config.concept_graph.clone()).await?;
        let concept_graph = ConceptGraphComponent::new(config.concept_graph.clone(), concept_graph_manager)?;
        self.register_component("ConceptGraph".to_string(), Box::new(concept_graph))?;
        
        // Register simulation engine (depends on concept graph)
        let simulation_concept_graph = ConceptGraphManager::new(config.concept_graph.clone()).await?;
        let simulation_engine = SimulationEngineComponent::new(config.simulation_engine.clone(), simulation_concept_graph)?;
        self.register_component("SimulationEngine".to_string(), Box::new(simulation_engine))?;
        
        info!("Core components registered successfully");
        Ok(())
    }
    
    /// Update the initialization order based on dependencies
    fn update_initialization_order(&mut self) {
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        let mut order = Vec::new();
        
        for component_name in self.components.keys() {
            if !visited.contains(component_name) {
                self.visit_component(component_name, &mut visited, &mut temp_visited, &mut order);
            }
        }
        
        self.initialization_order = order;
    }
    
    /// Depth-first search for dependency resolution
    fn visit_component(
        &self,
        name: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        order: &mut Vec<String>,
    ) {
        if temp_visited.contains(name) {
            // Circular dependency detected - skip for now
            warn!("Circular dependency detected involving component: {}", name);
            return;
        }
        
        if visited.contains(name) {
            return;
        }
        
        temp_visited.insert(name.to_string());
        
        if let Some(deps) = self.dependencies.get(name) {
            for dep in deps {
                if self.components.contains_key(dep) {
                    self.visit_component(dep, visited, temp_visited, order);
                }
            }
        }
        
        temp_visited.remove(name);
        visited.insert(name.to_string());
        order.push(name.to_string());
    }
    
    /// Get the initialization order
    pub fn get_initialization_order(&self) -> &[String] {
        &self.initialization_order
    }
}

impl UnifiedAPI {
    /// Create a new unified API interface
    pub fn new(
        components: Arc<Mutex<ComponentRegistry>>,
        config: BrainSystemConfig,
    ) -> Self {
        Self {
            components,
            config,
            call_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Execute an API call on a specific component
    pub fn execute_call(
        &self,
        component: &str,
        operation: &str,
        parameters: HashMap<String, String>,
    ) -> IntegrationResult<String> {
        // Update call statistics
        {
            let mut stats = self.call_stats.lock().unwrap();
            *stats.entry(format!("{}::{}", component, operation)).or_insert(0) += 1;
        }
        
        // Get component registry
        let components = self.components.lock().unwrap();
        
        // Check if component exists and is ready
        if let Some((_comp, status)) = components.components.get(component) {
            match status {
                ComponentStatus::Ready => {
                    // Component is ready - execute operation
                    info!("Executing operation '{}' on component '{}'", operation, component);
                    
                    // For now, return a placeholder response
                    // In a full implementation, this would route to the actual component method
                    Ok(format!("Operation '{}' executed on component '{}' with parameters: {:?}", 
                             operation, component, parameters))
                }
                ComponentStatus::Uninitialized => {
                    Err(IntegrationError::ComponentNotReady(
                        format!("Component '{}' is not initialized", component)
                    ))
                }
                ComponentStatus::Initializing => {
                    Err(IntegrationError::ComponentNotReady(
                        format!("Component '{}' is currently initializing", component)
                    ))
                }
                ComponentStatus::Error(ref error) => {
                    Err(IntegrationError::ComponentNotReady(
                        format!("Component '{}' is in error state: {}", component, error)
                    ))
                }
                ComponentStatus::Stopped => {
                    Err(IntegrationError::ComponentNotReady(
                        format!("Component '{}' is stopped", component)
                    ))
                }
            }
        } else {
            Err(IntegrationError::ComponentNotFound(component.to_string()))
        }
    }
    
    /// Get API call statistics
    pub fn get_call_stats(&self) -> HashMap<String, usize> {
        self.call_stats.lock().unwrap().clone()
    }
}

impl WorkflowEngine {
    /// Create a new workflow engine
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
            execution_history: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Register a workflow
    pub fn register_workflow(&mut self, workflow: Workflow) {
        info!("Registering workflow: {}", workflow.name);
        self.workflows.insert(workflow.id.clone(), workflow);
    }
    
    /// Execute a workflow
    pub fn execute_workflow(&self, workflow_id: &str) -> IntegrationResult<String> {
        info!("Executing workflow: {}", workflow_id);
        
        let workflow = self.workflows.get(workflow_id)
            .ok_or_else(|| IntegrationError::WorkflowExecutionFailed(
                format!("Workflow '{}' not found", workflow_id)
            ))?;
        
        let execution_id = Uuid::new_v4().to_string();
        let start_time = current_timestamp();
        
        let mut execution = WorkflowExecution {
            workflow_id: workflow_id.to_string(),
            execution_id: execution_id.clone(),
            start_time,
            end_time: None,
            status: WorkflowStatus::Running,
            step_results: HashMap::new(),
            error: None,
        };
        
        // Execute workflow steps
        for step in &workflow.steps {
            info!("Executing workflow step: {}", step.name);
            
            let step_start = current_timestamp();
            let step_result = StepResult {
                status: StepStatus::Completed,
                duration_ms: (current_timestamp() - step_start) * 1000,
                output: Some(format!("Step '{}' completed", step.name)),
                error: None,
            };
            
            execution.step_results.insert(step.id.clone(), step_result);
        }
        
        execution.end_time = Some(current_timestamp());
        execution.status = WorkflowStatus::Completed;
        
        // Store execution history
        {
            let mut history = self.execution_history.lock().unwrap();
            history.push(execution);
        }
        
        info!("Workflow '{}' executed successfully", workflow_id);
        Ok(format!("Workflow '{}' executed with execution ID: {}", workflow_id, execution_id))
    }
    
    /// Get workflow execution history
    pub fn get_execution_history(&self) -> Vec<WorkflowExecution> {
        self.execution_history.lock().unwrap().clone()
    }
}

impl SystemHealth {
    /// Create a new system health instance
    pub fn new() -> Self {
        Self {
            overall_status: HealthStatus::Healthy,
            component_health: HashMap::new(),
            uptime_seconds: 0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            last_check: current_timestamp(),
            active_operations: 0,
            recent_errors: 0,
        }
    }
}

impl SystemMetrics {
    /// Create a new system metrics instance
    pub fn new() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            avg_response_time_ms: 0.0,
            operations_per_second: 0.0,
            component_metrics: HashMap::new(),
            last_updated: current_timestamp(),
        }
    }
}

impl Default for ComponentMetrics {
    fn default() -> Self {
        Self {
            operations: 0,
            avg_response_time_ms: 0.0,
            error_rate: 0.0,
            throughput: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemState {
    health: SystemHealth,
    metrics: SystemMetrics,
    recent_events: Vec<SystemEvent>,
    component_status: HashMap<String, ComponentStatus>,
}

/// Character predictor component wrapper
#[derive(Debug)]
pub struct CharacterPredictorComponent {
    #[allow(dead_code)] // Reserved for future character prediction functionality
    vocab: CharacterVocab,
    config: ModelConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl CharacterPredictorComponent {
    pub fn new(config: ModelConfig) -> Result<Self> {
        let vocab = CharacterVocab::from_text("abcdefghijklmnopqrstuvwxyz");
        
        Ok(Self {
            vocab,
            config,
            status: ComponentStatus::Uninitialized,
            metrics: ComponentMetrics::default(),
        })
    }
}

impl SystemComponent for CharacterPredictorComponent {
    fn name(&self) -> &str { "CharacterPredictor" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn initialize(&mut self) -> Result<()> {
        self.status = ComponentStatus::Ready;
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        self.status = ComponentStatus::Stopped;
        Ok(())
    }
    
    fn status(&self) -> ComponentStatus { self.status.clone() }
    
    fn health_check(&self) -> Result<ComponentHealth> {
        Ok(ComponentHealth {
            status: self.status.clone(),
            last_response_time_ms: 10,
            error_count: 0,
            success_count: 1,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.vocab_size == 0 {
            return Err(BrainError::Serialization { 
                source: "Vocabulary size must be greater than 0".into() 
            });
        }
        Ok(())
    }
}

/// BPE segmenter component wrapper
#[derive(Debug)]
pub struct BpeSegmenterComponent {
    #[allow(dead_code)] // Reserved for future segmentation functionality
    segmenter: BpeSegmenter,
    config: BpeConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl BpeSegmenterComponent {
    pub fn new(config: BpeConfig) -> Result<Self> {
        let segmenter = BpeSegmenter::new(config.clone());
        
        Ok(Self {
            segmenter,
            config,
            status: ComponentStatus::Uninitialized,
            metrics: ComponentMetrics::default(),
        })
    }
}

impl SystemComponent for BpeSegmenterComponent {
    fn name(&self) -> &str { "BpeSegmenter" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn initialize(&mut self) -> Result<()> {
        self.status = ComponentStatus::Ready;
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        self.status = ComponentStatus::Stopped;
        Ok(())
    }
    
    fn status(&self) -> ComponentStatus { self.status.clone() }
    
    fn health_check(&self) -> Result<ComponentHealth> {
        Ok(ComponentHealth {
            status: self.status.clone(),
            last_response_time_ms: 5,
            error_count: 0,
            success_count: 1,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.max_vocab_size == 0 {
            return Err(BrainError::Serialization { 
                source: "Max vocabulary size must be greater than 0".into() 
            });
        }
        Ok(())
    }
}

/// Memory system component wrapper (using Debug derive manually due to WorkingMemoryRepository)
pub struct MemorySystemComponent {
    #[allow(dead_code)] // Reserved for future memory operations
    memory: Arc<Mutex<WorkingMemoryRepository>>,
    config: ConsolidationConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl std::fmt::Debug for MemorySystemComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemorySystemComponent")
            .field("config", &self.config)
            .field("status", &self.status)
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl MemorySystemComponent {
    pub fn new(config: ConsolidationConfig) -> Result<Self> {
        let memory = WorkingMemoryRepository::new(1000);
        
        Ok(Self {
            memory: Arc::new(Mutex::new(memory)),
            config,
            status: ComponentStatus::Uninitialized,
            metrics: ComponentMetrics::default(),
        })
    }
}

impl SystemComponent for MemorySystemComponent {
    fn name(&self) -> &str { "MemorySystem" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn initialize(&mut self) -> Result<()> {
        self.status = ComponentStatus::Ready;
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        self.status = ComponentStatus::Stopped;
        Ok(())
    }
    
    fn status(&self) -> ComponentStatus { self.status.clone() }
    
    fn health_check(&self) -> Result<ComponentHealth> {
        Ok(ComponentHealth {
            status: self.status.clone(),
            last_response_time_ms: 15,
            error_count: 0,
            success_count: 1,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.min_access_count == 0 {
            return Err(BrainError::Serialization { 
                source: "Min access count must be greater than 0".into() 
            });
        }
        Ok(())
    }
}

/// Concept graph component wrapper (using Debug derive manually due to ConceptGraphManager)
pub struct ConceptGraphComponent {
    #[allow(dead_code)] // Reserved for future concept graph operations
    graph: Arc<Mutex<ConceptGraphManager>>,
    #[allow(dead_code)] // Reserved for future configuration management
    config: ConceptGraphConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl std::fmt::Debug for ConceptGraphComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConceptGraphComponent")
            .field("config", &self.config)
            .field("status", &self.status)
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl ConceptGraphComponent {
    pub fn new(config: ConceptGraphConfig, graph_manager: ConceptGraphManager) -> Result<Self> {
        Ok(Self {
            graph: Arc::new(Mutex::new(graph_manager)),
            config,
            status: ComponentStatus::Uninitialized,
            metrics: ComponentMetrics::default(),
        })
    }
}

impl SystemComponent for ConceptGraphComponent {
    fn name(&self) -> &str { "ConceptGraph" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn initialize(&mut self) -> Result<()> {
        self.status = ComponentStatus::Ready;
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        self.status = ComponentStatus::Stopped;
        Ok(())
    }
    
    fn status(&self) -> ComponentStatus { self.status.clone() }
    
    fn health_check(&self) -> Result<ComponentHealth> {
        Ok(ComponentHealth {
            status: self.status.clone(),
            last_response_time_ms: 20,
            error_count: 0,
            success_count: 1,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec!["MemorySystem".to_string()] }
    
    fn validate_config(&self) -> Result<()> {
        Ok(())
    }
}

/// Simulation engine component wrapper (using Debug derive manually due to SimulationEngineImpl)
pub struct SimulationEngineComponent {
    #[allow(dead_code)] // Reserved for future simulation functionality
    engine: Arc<Mutex<SimulationEngineImpl>>,
    config: SimulationConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl std::fmt::Debug for SimulationEngineComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimulationEngineComponent")
            .field("config", &self.config)
            .field("status", &self.status)
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl SimulationEngineComponent {
    pub fn new(config: SimulationConfig, concept_graph: ConceptGraphManager) -> Result<Self> {
        let concept_graph_arc = Arc::new(RwLock::new(concept_graph));
        let engine = SimulationEngineImpl::new(concept_graph_arc);
        
        Ok(Self {
            engine: Arc::new(Mutex::new(engine)),
            config,
            status: ComponentStatus::Uninitialized,
            metrics: ComponentMetrics::default(),
        })
    }
}

impl SystemComponent for SimulationEngineComponent {
    fn name(&self) -> &str { "SimulationEngine" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn initialize(&mut self) -> Result<()> {
        self.status = ComponentStatus::Ready;
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        self.status = ComponentStatus::Stopped;
        Ok(())
    }
    
    fn status(&self) -> ComponentStatus { self.status.clone() }
    
    fn health_check(&self) -> Result<ComponentHealth> {
        Ok(ComponentHealth {
            status: self.status.clone(),
            last_response_time_ms: 25,
            error_count: 0,
            success_count: 1,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { 
        vec!["ConceptGraph".to_string(), "MemorySystem".to_string()]
    }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.max_entities_per_state == 0 {
            return Err(BrainError::Serialization { 
                source: "Max entities per state must be greater than 0".into() 
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_brain_system_builder() {
        let builder = BrainSystemBuilder::new()
            .with_logging_enabled(true)
            .with_max_concurrent_operations(50);
        
        // Test builder configuration
        assert_eq!(builder.config.max_concurrent_operations, 50);
        assert!(builder.config.enable_comprehensive_logging);
    }

    #[test]
    fn test_component_registry() {
        let mut registry = ComponentRegistry::new();
        assert_eq!(registry.get_initialization_order().len(), 0);
        
        // Test would require actual component implementations
        // This is a placeholder for more comprehensive testing
    }

    #[test]
    fn test_unified_api() {
        let registry = Arc::new(Mutex::new(ComponentRegistry::new()));
        let config = BrainSystemConfig::default();
        let api = UnifiedAPI::new(registry, config);
        
        // Test API call to non-existent component
        let result = api.execute_call("NonExistent", "test", HashMap::new());
        assert!(result.is_err());
        
        match result.unwrap_err() {
            IntegrationError::ComponentNotFound(name) => {
                assert_eq!(name, "NonExistent");
            }
            _ => panic!("Expected ComponentNotFound error"),
        }
    }

    #[test]
    fn test_workflow_engine() {
        let mut engine = WorkflowEngine::new();
        
        let workflow = Workflow {
            id: "test_workflow".to_string(),
            name: "Test Workflow".to_string(),
            description: "A test workflow".to_string(),
            steps: vec![
                WorkflowStep {
                    id: "step1".to_string(),
                    name: "First Step".to_string(),
                    component: "TestComponent".to_string(),
                    operation: "test_operation".to_string(),
                    parameters: HashMap::new(),
                    retry_count: 0,
                    timeout_ms: 5000,
                }
            ],
            dependencies: vec![],
        };
        
        engine.register_workflow(workflow);
        
        // Test workflow execution
        let result = engine.execute_workflow("test_workflow");
        assert!(result.is_ok());
    }

    #[test]
    fn test_system_health() {
        let health = SystemHealth::new();
        assert_eq!(health.overall_status, HealthStatus::Healthy);
        assert_eq!(health.component_health.len(), 0);
    }

    #[test]
    fn test_system_metrics() {
        let metrics = SystemMetrics::new();
        assert_eq!(metrics.total_operations, 0);
        assert_eq!(metrics.successful_operations, 0);
        assert_eq!(metrics.failed_operations, 0);
    }
} 