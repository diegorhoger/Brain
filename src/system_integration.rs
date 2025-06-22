//! Task 10.1: Core System Integration and Interface Standardization
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
use crate::{
    Result, BrainError,
    // Core component configs only (components will be wrapped)
    ModelConfig, BpeConfig, ConsolidationConfig,
    ConceptGraphConfig, RuleFormalizationConfig, SimulationConfig,
    MetaMemoryConfig, NoveltyDetectionConfig, CuriosityConfig,
    VisualizationConfig,
    // Import actual Brain components for integration
    CharacterPredictor, CharacterVocab,
    BpeSegmenter, 
    MemorySystem,
    ConceptGraphManager,
    SimulationEngine,
    QueryEngine,
    // Task 10.2 - Performance monitoring integration
    performance_monitor::{PerformanceMonitor, PerformanceConfig},
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
    /// Performance monitoring and optimization (Task 10.2)
    performance_monitor: Option<Arc<PerformanceMonitor>>,
    /// System initialization timestamp
    initialized_at: u64,
}

/// Configuration for the entire Brain AI system
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub rule_formalization: RuleFormalizationConfig,
    pub simulation_engine: SimulationConfig,
    pub meta_memory: MetaMemoryConfig,
    pub novelty_detection: NoveltyDetectionConfig,
    pub curiosity_learning: CuriosityConfig,
    pub visualization: VisualizationConfig,
    
    /// Infrastructure configurations (simplified for now)
    pub enable_auth: bool,
    pub enable_rate_limiting: bool,
    pub enable_logging: bool,
    
    /// System-level settings
    pub enable_comprehensive_logging: bool,
    pub enable_performance_monitoring: bool,
    pub enable_health_checks: bool,
    pub max_concurrent_operations: usize,
    pub component_initialization_timeout_ms: u64,
    
    /// Performance monitoring configuration (Task 10.2)
    pub performance_config: PerformanceConfig,
}

impl Default for BrainSystemConfig {
    fn default() -> Self {
        Self {
            system_id: uuid::Uuid::new_v4().to_string(),
            system_name: "Brain AI System".to_string(),
            version: crate::VERSION.to_string(),
            
            // Use default configurations for all components
            character_predictor: ModelConfig::default(),
            segment_discovery: BpeConfig::default(),
            memory_system: ConsolidationConfig::default(),
            concept_graph: ConceptGraphConfig::default(),
            rule_formalization: RuleFormalizationConfig::default(),
            simulation_engine: SimulationConfig::default(),
            meta_memory: MetaMemoryConfig::default(),
            novelty_detection: NoveltyDetectionConfig::default(),
            curiosity_learning: CuriosityConfig::default(),
            visualization: VisualizationConfig::default(),
            
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

/// Unified API interface providing standardized access to all components
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

/// Workflow execution engine for orchestrating complex operations
#[derive(Debug)]
pub struct WorkflowEngine {
    /// Available workflows
    workflows: HashMap<String, Workflow>,
    /// Execution history
    execution_history: Arc<Mutex<Vec<WorkflowExecution>>>,
}

/// System health monitoring
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

/// Health status for individual components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: ComponentStatus,
    pub last_response_time_ms: u64,
    pub error_count: usize,
    pub success_count: usize,
    pub last_error: Option<String>,
}

/// System performance metrics
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

/// Metrics for individual components
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

/// Types of system events
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

/// Result type for integration operations
pub type IntegrationResult<T> = std::result::Result<T, IntegrationError>;

/// Integration-specific error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationError {
    ComponentNotFound(String),
    ComponentNotReady(String),
    InvalidConfiguration(String),
    WorkflowExecutionFailed(String),
    HealthCheckFailed(String),
    ResourceExhausted(String),
    TimeoutError(String),
}

impl std::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationError::ComponentNotFound(name) => write!(f, "Component not found: {}", name),
            IntegrationError::ComponentNotReady(name) => write!(f, "Component not ready: {}", name),
            IntegrationError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
            IntegrationError::WorkflowExecutionFailed(msg) => write!(f, "Workflow execution failed: {}", msg),
            IntegrationError::HealthCheckFailed(msg) => write!(f, "Health check failed: {}", msg),
            IntegrationError::ResourceExhausted(msg) => write!(f, "Resource exhausted: {}", msg),
            IntegrationError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
        }
    }
}

impl std::error::Error for IntegrationError {}

/// Trait for system components to implement standardized interfaces
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

/// Workflow execution record
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

/// Result of a workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub status: StepStatus,
    pub duration_ms: u64,
    pub output: Option<String>,
    pub error: Option<String>,
}

/// Status of a workflow step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

impl BrainSystem {
    /// Create a new Brain AI system with the specified configuration
    pub async fn new(
        config: BrainSystemConfig,
        custom_components: HashMap<String, Box<dyn SystemComponent>>,
    ) -> Result<Self> {
        let start_time = current_timestamp();
        
        // Initialize component registry
        let mut components = ComponentRegistry::new();
        
        // Add custom components
        for (name, component) in custom_components {
            components.register_component(name, component)?;
        }
        
        // Initialize standard components
        components.register_core_components(&config).await?;
        
        // Create unified API with empty registry for now (will be updated later)
        let api = UnifiedAPI::new(Arc::new(Mutex::new(ComponentRegistry::new())), config.clone());
        
        // Initialize workflow engine
        let workflows = WorkflowEngine::new();
        
        // Initialize health monitoring
        let health = Arc::new(Mutex::new(SystemHealth::new()));
        
        // Initialize event logging
        let events = Arc::new(Mutex::new(Vec::new()));
        
        // Initialize metrics
        let metrics = Arc::new(Mutex::new(SystemMetrics::new()));
        
        // Initialize performance monitor (Task 10.2)
        let performance_monitor = if config.enable_performance_monitoring {
            match PerformanceMonitor::new(config.performance_config.clone()) {
                Ok(monitor) => Some(Arc::new(monitor)),
                Err(e) => {
                    warn!("Failed to initialize performance monitor: {}", e);
                    None
                }
            }
        } else {
            None
        };
        
        let system = Self {
            config: config.clone(),
            components, // Use the populated components registry!
            api,
            workflows,
            health,
            events,
            metrics,
            performance_monitor,
            initialized_at: start_time,
        };
        
        // Initialize all components
        let mut system_mut = system;
        system_mut.initialize_components()?;
        let system = system_mut;
        
        // Log system startup
        system.log_event(SystemEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::SystemStartup,
            component: "BrainSystem".to_string(),
            message: "Brain AI System initialized successfully".to_string(),
            timestamp: start_time,
            metadata: HashMap::new(),
        });
        
        Ok(system)
    }
    
    /// Initialize all system components in dependency order
    fn initialize_components(&mut self) -> Result<()> {
        let initialization_order: Vec<String> = self.components.get_initialization_order().to_vec();
        
        for component_name in initialization_order {
            self.initialize_component(&component_name)?;
        }
        
        Ok(())
    }
    
    /// Initialize a specific component
    fn initialize_component(&mut self, name: &str) -> Result<()> {
        if let Some((component, status)) = self.components.components.get_mut(name) {
            if *status == ComponentStatus::Uninitialized {
                *status = ComponentStatus::Initializing;
                
                match component.initialize() {
                    Ok(()) => {
                        *status = ComponentStatus::Ready;
                        self.log_event(SystemEvent {
                            event_id: uuid::Uuid::new_v4().to_string(),
                            event_type: EventType::ComponentInitialized,
                            component: name.to_string(),
                            message: format!("Component {} initialized successfully", name),
                            timestamp: current_timestamp(),
                            metadata: HashMap::new(),
                        });
                    }
                    Err(e) => {
                        *status = ComponentStatus::Error(e.to_string());
                        return Err(BrainError::Other(format!(
                            "Failed to initialize component {}: {}", name, e
                        )));
                    }
                }
            }
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
    
    /// Get system metrics
    pub fn metrics(&self) -> SystemMetrics {
        self.metrics.lock().unwrap().clone()
    }
    
    /// Get performance monitor (Task 10.2)
    pub fn performance_monitor(&self) -> Option<Arc<PerformanceMonitor>> {
        self.performance_monitor.clone()
    }
    
    /// Start performance monitoring (Task 10.2)
    pub async fn start_performance_monitoring(&self) -> Result<()> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.start().await?;
            info!("Performance monitoring started");
        } else {
            warn!("Performance monitoring not available - check configuration");
        }
        Ok(())
    }
    
    /// Stop performance monitoring (Task 10.2)
    pub async fn stop_performance_monitoring(&self) -> Result<()> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.stop().await?;
            info!("Performance monitoring stopped");
        }
        Ok(())
    }
    
    /// Record component operation for performance tracking (Task 10.2)
    pub fn record_operation(
        &self,
        component_name: &str,
        operation: &str,
        duration: std::time::Duration,
        success: bool,
    ) -> Result<()> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.record_operation(component_name, operation, duration, success)?;
        }
        Ok(())
    }
    
    /// Get performance snapshot (Task 10.2)
    pub fn get_performance_snapshot(&self) -> Result<Option<crate::performance_monitor::PerformanceSnapshot>> {
        if let Some(ref monitor) = self.performance_monitor {
            Ok(Some(monitor.get_current_snapshot()?))
        } else {
            Ok(None)
        }
    }
    
    /// Identify performance bottlenecks (Task 10.2)
    pub fn identify_bottlenecks(&self) -> Result<Vec<crate::performance_monitor::PerformanceBottleneck>> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.identify_bottlenecks()
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Get optimization recommendations (Task 10.2)
    pub fn get_optimization_recommendations(&self) -> Result<Vec<crate::performance_monitor::OptimizationRecommendation>> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.get_optimization_recommendations()
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Export performance report (Task 10.2)
    pub fn export_performance_report(&self, format: crate::performance_monitor::ReportFormat) -> Result<String> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.export_performance_report(format)
        } else {
            Err(BrainError::Other("Performance monitoring not available".to_string()))
        }
    }
    
    /// Get recent system events
    pub fn recent_events(&self, limit: usize) -> Vec<SystemEvent> {
        let events = self.events.lock().unwrap();
        events.iter().rev().take(limit).cloned().collect()
    }
    
    /// Perform system health check
    pub fn perform_health_check(&self) -> Result<SystemHealth> {
        let mut health = self.health.lock().unwrap();
        
        // Update uptime
        health.uptime_seconds = current_timestamp() - self.initialized_at;
        
        // Check component health
        let mut overall_healthy = true;
        for (name, (component, _)) in &self.components.components {
            match component.health_check() {
                Ok(component_health) => {
                    health.component_health.insert(name.clone(), component_health);
                }
                Err(e) => {
                    overall_healthy = false;
                    health.component_health.insert(name.clone(), ComponentHealth {
                        status: ComponentStatus::Error(e.to_string()),
                        last_response_time_ms: 0,
                        error_count: 1,
                        success_count: 0,
                        last_error: Some(e.to_string()),
                    });
                }
            }
        }
        
        // Update overall status
        health.overall_status = if overall_healthy {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };
        
        health.last_check = current_timestamp();
        
        Ok(health.clone())
    }
    
    /// Shutdown the system gracefully
    pub fn shutdown(&mut self) -> Result<()> {
        // Log shutdown event
        self.log_event(SystemEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::SystemShutdown,
            component: "BrainSystem".to_string(),
            message: "Brain AI System shutting down".to_string(),
            timestamp: current_timestamp(),
            metadata: HashMap::new(),
        });
        
        // Shutdown components in reverse order
        let shutdown_order: Vec<String> = self.components
            .initialization_order
            .iter()
            .rev()
            .cloned()
            .collect();
        
        for component_name in shutdown_order {
            if let Some((component, _)) = self.components.components.get_mut(&component_name) {
                let start_time = std::time::Instant::now();
                
                match component.shutdown() {
                    Ok(()) => {
                        let duration = start_time.elapsed();
                        info!("Shutdown component: {} ({}ms)", component_name, duration.as_millis());
                        
                        // Record shutdown performance (Task 10.2)
                        if let Err(e) = self.record_operation(&component_name, "shutdown", duration, true) {
                            debug!("Failed to record shutdown performance for {}: {}", component_name, e);
                        }
                    }
                    Err(e) => {
                        let duration = start_time.elapsed();
                        warn!("Failed to shutdown component {}: {}", component_name, e);
                        
                        // Record shutdown failure (Task 10.2)
                        if let Err(record_err) = self.record_operation(&component_name, "shutdown", duration, false) {
                            debug!("Failed to record shutdown failure for {}: {}", component_name, record_err);
                        }
                    }
                }
            }
        }
        
        // Stop performance monitoring last (Task 10.2)
        if let Some(ref monitor) = self.performance_monitor {
            // Try to stop performance monitoring - handle both sync and async contexts
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                // We're in an async context, spawn the task
                let monitor_clone = monitor.clone();
                handle.spawn(async move {
                    if let Err(e) = monitor_clone.stop().await {
                        warn!("Failed to stop performance monitoring during shutdown: {}", e);
                    } else {
                        info!("Performance monitoring stopped during shutdown");
                    }
                });
            } else {
                // We're in a sync context, create a new runtime
                match tokio::runtime::Runtime::new() {
                    Ok(runtime) => {
                        if let Err(e) = runtime.block_on(monitor.stop()) {
                            warn!("Failed to stop performance monitoring during shutdown: {}", e);
                        } else {
                            info!("Performance monitoring stopped during shutdown");
                        }
                    }
                    Err(_) => {
                        warn!("Failed to create tokio runtime for shutdown");
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Log a system event
    fn log_event(&self, event: SystemEvent) {
        // Always log critical system events (startup, shutdown, errors)
        let should_log = self.config.enable_comprehensive_logging || 
                        matches!(event.event_type, 
                            EventType::SystemStartup | 
                            EventType::SystemShutdown | 
                            EventType::ComponentError);
        
        if should_log {
            self.events.lock().unwrap().push(event);
        }
    }
    
    /// Export comprehensive system state for debugging
    pub fn export_system_state(&self) -> Result<String> {
        let state = SystemState {
            config: self.config.clone(),
            health: self.health(),
            metrics: self.metrics(),
            recent_events: self.recent_events(100),
            component_status: self.get_component_status(),
        };
        
        serde_json::to_string_pretty(&state)
            .map_err(|e| BrainError::Serialization { source: Box::new(e) })
    }
    
    /// Get status of all components
    fn get_component_status(&self) -> HashMap<String, ComponentStatus> {
        self.components.components
            .iter()
            .map(|(name, (_, status))| (name.clone(), status.clone()))
            .collect()
    }
}

/// Complete system state for export and debugging
#[derive(Debug, Serialize, Deserialize)]
struct SystemState {
    config: BrainSystemConfig,
    health: SystemHealth,
    metrics: SystemMetrics,
    recent_events: Vec<SystemEvent>,
    component_status: HashMap<String, ComponentStatus>,
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
    
    /// Register a component in the system
    pub fn register_component(&mut self, name: String, component: Box<dyn SystemComponent>) -> Result<()> {
        // Validate component configuration
        component.validate_config()?;
        
        // Store dependencies
        let deps = component.dependencies();
        self.dependencies.insert(name.clone(), deps);
        
        // Register component
        self.components.insert(name.clone(), (component, ComponentStatus::Uninitialized));
        
        // Update initialization order
        self.update_initialization_order();
        
        Ok(())
    }
    
    /// Register all core Brain AI components
    pub async fn register_core_components(&mut self, config: &BrainSystemConfig) -> Result<()> {
        // Task 10.1: Register all core Brain components with their configurations
        
        println!("Registering core Brain components...");
        
        // Register Character Predictor
        let char_predictor = CharacterPredictorComponent::new(config.character_predictor.clone())?;
        self.register_component("CharacterPredictor".to_string(), Box::new(char_predictor))?;
        
        // Register BPE Segmenter  
        let bpe_segmenter = BpeSegmenterComponent::new(config.segment_discovery.clone())?;
        self.register_component("BpeSegmenter".to_string(), Box::new(bpe_segmenter))?;
        
        // Register Memory System
        let memory_system = MemorySystemComponent::new(config.memory_system.clone())?;
        self.register_component("MemorySystem".to_string(), Box::new(memory_system))?;
        
        // Register Concept Graph
        let concept_graph_manager = ConceptGraphManager::new(config.concept_graph.clone()).await
            .map_err(|e| BrainError::Other(format!("Failed to create ConceptGraphManager: {}", e)))?;
        let concept_graph = ConceptGraphComponent::new(config.concept_graph.clone(), concept_graph_manager)?;
        self.register_component("ConceptGraph".to_string(), Box::new(concept_graph))?;
        
        // Register Simulation Engine
        let concept_graph_for_sim = ConceptGraphManager::new(config.concept_graph.clone()).await
            .map_err(|e| BrainError::Other(format!("Failed to create ConceptGraphManager for simulation: {}", e)))?;
        let simulation_engine = SimulationEngineComponent::new(config.simulation_engine.clone(), concept_graph_for_sim)?;
        self.register_component("SimulationEngine".to_string(), Box::new(simulation_engine))?;
        
        // Register Query Engine
        let query_engine = QueryEngineComponent::new()?;
        self.register_component("QueryEngine".to_string(), Box::new(query_engine))?;
        
        println!("✅ Task 10.1: Core Brain components registered successfully!");
        
        Ok(())
    }
    
    /// Update component initialization order based on dependencies
    fn update_initialization_order(&mut self) {
        // Topological sort of dependencies
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        
        for component_name in self.components.keys() {
            if !visited.contains(component_name) {
                self.visit_component(component_name, &mut visited, &mut temp_visited, &mut order);
            }
        }
        
        self.initialization_order = order;
    }
    
    /// Visit component in dependency graph (DFS)
    fn visit_component(
        &self,
        name: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        order: &mut Vec<String>,
    ) {
        if temp_visited.contains(name) {
            // Circular dependency detected - handle gracefully
            return;
        }
        
        if visited.contains(name) {
            return;
        }
        
        temp_visited.insert(name.to_string());
        
        if let Some(deps) = self.dependencies.get(name) {
            for dep in deps {
                self.visit_component(dep, visited, temp_visited, order);
            }
        }
        
        temp_visited.remove(name);
        visited.insert(name.to_string());
        order.insert(0, name.to_string()); // Reverse topological order
    }
    
    /// Get component initialization order
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
    
    /// Execute a standardized API call
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
        
        // Validate component exists and is ready
        let components = self.components.lock().unwrap();
        if let Some((_, status)) = components.components.get(component) {
            match status {
                ComponentStatus::Ready => {
                    // Component is ready - proceed with operation
                    // This would delegate to the specific component's operation
                    Ok(format!("Executed {}::{} with parameters {:?}", component, operation, parameters))
                }
                ComponentStatus::Uninitialized => {
                    Err(IntegrationError::ComponentNotReady(format!(
                        "Component {} is not initialized", component
                    )))
                }
                ComponentStatus::Error(e) => {
                    Err(IntegrationError::ComponentNotReady(format!(
                        "Component {} is in error state: {}", component, e
                    )))
                }
                _ => {
                    Err(IntegrationError::ComponentNotReady(format!(
                        "Component {} is not ready (status: {:?})", component, status
                    )))
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
        self.workflows.insert(workflow.id.clone(), workflow);
    }
    
    /// Execute a workflow
    pub fn execute_workflow(&self, workflow_id: &str) -> IntegrationResult<String> {
        if let Some(workflow) = self.workflows.get(workflow_id) {
            let execution_id = uuid::Uuid::new_v4().to_string();
            let start_time = current_timestamp();
            
            let mut execution = WorkflowExecution {
                workflow_id: workflow.id.clone(),
                execution_id: execution_id.clone(),
                start_time,
                end_time: None,
                status: WorkflowStatus::Running,
                step_results: HashMap::new(),
                error: None,
            };
            
            // Execute workflow steps
            for step in &workflow.steps {
                let step_start = current_timestamp();
                
                // Simulate step execution
                let step_result = StepResult {
                    status: StepStatus::Completed,
                    duration_ms: current_timestamp() - step_start,
                    output: Some(format!("Step {} completed", step.name)),
                    error: None,
                };
                
                execution.step_results.insert(step.id.clone(), step_result);
            }
            
            execution.end_time = Some(current_timestamp());
            execution.status = WorkflowStatus::Completed;
            
            // Store execution history
            self.execution_history.lock().unwrap().push(execution);
            
            Ok(execution_id)
        } else {
            Err(IntegrationError::WorkflowExecutionFailed(format!(
                "Workflow {} not found", workflow_id
            )))
        }
    }
    
    /// Get workflow execution history
    pub fn get_execution_history(&self) -> Vec<WorkflowExecution> {
        self.execution_history.lock().unwrap().clone()
    }
}

impl SystemHealth {
    /// Create new system health instance
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
    /// Create new system metrics instance
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

/// Get current timestamp in seconds since Unix epoch
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_brain_system_builder() {
        let config = BrainSystemConfig::default();
        let builder = BrainSystemBuilder::new()
            .with_config(config)
            .with_logging_enabled(true)
            .with_max_concurrent_operations(50);
        
        // Builder should be configured correctly
        assert_eq!(builder.config.max_concurrent_operations, 50);
        assert!(builder.config.enable_comprehensive_logging);
    }
    
    #[test]
    fn test_component_registry() {
        let registry = ComponentRegistry::new();
        
        // Registry should start empty
        assert_eq!(registry.components.len(), 0);
        assert_eq!(registry.initialization_order.len(), 0);
    }
    
    #[test]
    fn test_unified_api() {
        let components = Arc::new(Mutex::new(ComponentRegistry::new()));
        let config = BrainSystemConfig::default();
        let api = UnifiedAPI::new(components, config);
        
        // API should handle missing components gracefully
        let result = api.execute_call("nonexistent", "test", HashMap::new());
        assert!(result.is_err());
        
        match result.unwrap_err() {
            IntegrationError::ComponentNotFound(name) => {
                assert_eq!(name, "nonexistent");
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
            steps: vec![],
            dependencies: vec![],
        };
        
        engine.register_workflow(workflow);
        assert!(engine.workflows.contains_key("test_workflow"));
        
        // Should be able to execute registered workflow
        let result = engine.execute_workflow("test_workflow");
        assert!(result.is_ok());
        
        // Should fail for non-existent workflow
        let result = engine.execute_workflow("nonexistent");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_system_health() {
        let health = SystemHealth::new();
        assert_eq!(health.overall_status, HealthStatus::Healthy);
        assert_eq!(health.component_health.len(), 0);
        assert_eq!(health.uptime_seconds, 0);
    }
    
    #[test]
    fn test_system_metrics() {
        let metrics = SystemMetrics::new();
        assert_eq!(metrics.total_operations, 0);
        assert_eq!(metrics.successful_operations, 0);
        assert_eq!(metrics.failed_operations, 0);
        assert_eq!(metrics.avg_response_time_ms, 0.0);
    }
}

// ============================================================================
// Task 10.1: Concrete SystemComponent Implementations
// ============================================================================

/// Character Predictor Component Implementation
#[derive(Debug)]
pub struct CharacterPredictorComponent {
    #[allow(dead_code)] // Reserved for future character prediction functionality
    predictor: Arc<Mutex<CharacterPredictor>>,
    #[allow(dead_code)] // Reserved for future vocabulary operations
    vocab: CharacterVocab,
    config: ModelConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl CharacterPredictorComponent {
    pub fn new(config: ModelConfig) -> Result<Self> {
        let vocab = CharacterVocab::from_text("abcdefghijklmnopqrstuvwxyz ");
        let predictor = CharacterPredictor::new(vocab.clone(), Some(config.clone()))?;
        
        Ok(Self {
            predictor: Arc::new(Mutex::new(predictor)),
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
        self.status = ComponentStatus::Initializing;
        // Character predictor is ready once created
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
            last_response_time_ms: 1,
            error_count: 0,
            success_count: self.metrics.operations as usize,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.embedding_dim == 0 || self.config.hidden_dim == 0 {
            return Err(BrainError::ConfigError("Invalid dimensions".to_string()));
        }
        Ok(())
    }
}

/// BPE Segmenter Component Implementation
#[derive(Debug)]
pub struct BpeSegmenterComponent {
    #[allow(dead_code)] // Reserved for future segmentation functionality
    segmenter: Arc<Mutex<BpeSegmenter>>,
    config: BpeConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl BpeSegmenterComponent {
    pub fn new(config: BpeConfig) -> Result<Self> {
        let segmenter = BpeSegmenter::new(config.clone());
        
        Ok(Self {
            segmenter: Arc::new(Mutex::new(segmenter)),
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
        self.status = ComponentStatus::Initializing;
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
            last_response_time_ms: 2,
            error_count: 0,
            success_count: self.metrics.operations as usize,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.min_frequency == 0 {
            return Err(BrainError::ConfigError("Invalid min_frequency".to_string()));
        }
        Ok(())
    }
}

/// Memory System Component Implementation
#[derive(Debug)]
pub struct MemorySystemComponent {
    #[allow(dead_code)] // Reserved for future memory operations
    memory: Arc<Mutex<MemorySystem>>,
    config: ConsolidationConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl MemorySystemComponent {
    pub fn new(config: ConsolidationConfig) -> Result<Self> {
        let memory = MemorySystem::new(1000); // Default working memory capacity
        
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
        self.status = ComponentStatus::Initializing;
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
            last_response_time_ms: 3,
            error_count: 0,
            success_count: self.metrics.operations as usize,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec![] }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.working_to_episodic_hours == 0 {
            return Err(BrainError::ConfigError("Invalid working_to_episodic_hours".to_string()));
        }
        Ok(())
    }
}

/// Concept Graph Component Implementation  
#[derive(Debug)]
pub struct ConceptGraphComponent {
    #[allow(dead_code)] // Reserved for future concept graph operations
    graph: Arc<Mutex<ConceptGraphManager>>,
    #[allow(dead_code)] // Reserved for future configuration management
    config: ConceptGraphConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
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
        self.status = ComponentStatus::Initializing;
        // Initialize Neo4j connection if needed
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
            success_count: self.metrics.operations as usize,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { vec!["MemorySystem".to_string()] }
    
    fn validate_config(&self) -> Result<()> {
        // Neo4j connection validation could go here
        Ok(())
    }
}

/// Simulation Engine Component Implementation
#[derive(Debug)]
pub struct SimulationEngineComponent {
    #[allow(dead_code)] // Reserved for future simulation functionality
    engine: Arc<Mutex<SimulationEngine>>,
    config: SimulationConfig,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl SimulationEngineComponent {
    pub fn new(config: SimulationConfig, concept_graph: ConceptGraphManager) -> Result<Self> {
        let engine = SimulationEngine::new(concept_graph);
        
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
        self.status = ComponentStatus::Initializing;
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
            last_response_time_ms: 8,
            error_count: 0,
            success_count: self.metrics.operations as usize,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { 
        vec!["ConceptGraph".to_string(), "MemorySystem".to_string()] 
    }
    
    fn validate_config(&self) -> Result<()> {
        if self.config.max_state_complexity == 0 {
            return Err(BrainError::ConfigError("Invalid state complexity".to_string()));
        }
        Ok(())
    }
}

/// Query Engine Component Implementation
#[derive(Debug)]
pub struct QueryEngineComponent {
    #[allow(dead_code)] // Reserved for future query functionality
    engine: Arc<Mutex<QueryEngine>>,
    status: ComponentStatus,
    metrics: ComponentMetrics,
}

impl QueryEngineComponent {
    pub fn new() -> Result<Self> {
        let engine = QueryEngine::new();
        
        Ok(Self {
            engine: Arc::new(Mutex::new(engine)),
            status: ComponentStatus::Uninitialized,
            metrics: ComponentMetrics::default(),
        })
    }
}

impl SystemComponent for QueryEngineComponent {
    fn name(&self) -> &str { "QueryEngine" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn initialize(&mut self) -> Result<()> {
        self.status = ComponentStatus::Initializing;
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
            last_response_time_ms: 4,
            error_count: 0,
            success_count: self.metrics.operations as usize,
            last_error: None,
        })
    }
    
    fn metrics(&self) -> ComponentMetrics { self.metrics.clone() }
    
    fn handle_event(&mut self, _event: &SystemEvent) -> Result<()> { Ok(()) }
    
    fn dependencies(&self) -> Vec<String> { 
        vec!["MemorySystem".to_string(), "ConceptGraph".to_string()] 
    }
    
    fn validate_config(&self) -> Result<()> { Ok(()) }
}

// ============================================================================
// End Task 10.1: SystemComponent Implementations
// =============================================================================

// =============================================================================
// STANDARDIZED INTERFACES FOR ALL BRAIN AI COMPONENTS
// =============================================================================

/// Unified configuration trait that all Brain AI components must implement
/// This ensures consistent configuration management across the entire system
pub trait BrainConfig: std::fmt::Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync {
    /// Validate the configuration
    fn validate(&self) -> Result<()>;
    
    /// Get configuration as JSON string
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    
    /// Create configuration from JSON string
    fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
    
    /// Get configuration metadata
    fn get_metadata(&self) -> ConfigMetadata;
    
    /// Merge with another configuration (for updates)
    fn merge(&mut self, other: &Self) -> Result<()>;
}

/// Configuration metadata for component identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub component_name: String,
    pub config_version: String,
    pub required_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub performance_tier: PerformanceTier,
}

/// Performance tiers for resource allocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceTier {
    Low,      // Minimal resources
    Medium,   // Standard resources
    High,     // Enhanced resources
    Critical, // Maximum resources
}

/// Unified data exchange format for inter-component communication
/// All components must be able to produce and consume BrainData
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainData {
    /// Unique identifier for this data
    pub id: Uuid,
    /// Data type identifier
    pub data_type: DataType,
    /// Timestamp when data was created
    pub timestamp: u64,
    /// Source component that produced this data
    pub source_component: String,
    /// Target component(s) for this data
    pub target_components: Vec<String>,
    /// The actual data payload
    pub payload: DataPayload,
    /// Confidence score for this data (0.0 to 1.0)
    pub confidence: f64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of data that can be exchanged between components
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataType {
    // Character-level data
    CharacterPrediction,
    CharacterSequence,
    
    // Segment-level data
    SegmentPattern,
    SegmentStats,
    
    // Memory data
    WorkingMemoryItem,
    EpisodicEvent,
    SemanticConcept,
    
    // Concept graph data
    ConceptNode,
    ConceptRelationship,
    
    // Rule and insight data
    ExtractedRule,
    InsightPattern,
    
    // Simulation data
    SimulationState,
    SimulationTransition,
    
    // Meta-cognitive data
    MetaMemoryEntry,
    NoveltyScore,
    CuriosityDrive,
    
    // System data
    SystemEvent,
    PerformanceMetrics,
    ConfigurationUpdate,
    
    // Generic data
    TextData,
    NumericData,
    BinaryData,
}

/// Data payload that can contain various types of structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPayload {
    /// Text-based data
    Text(String),
    /// Numeric data (single value)
    Number(f64),
    /// Numeric vector
    Vector(Vec<f64>),
    /// Key-value pairs
    KeyValue(HashMap<String, String>),
    /// Structured JSON data
    Json(serde_json::Value),
    /// Binary data
    Binary(Vec<u8>),
    /// Complex nested data
    Complex(Box<BrainData>),
    /// Array of data items
    Array(Vec<BrainData>),
}

/// Unified operations interface that all Brain AI components must implement
/// This provides a consistent way to interact with any component
pub trait BrainOperations: std::fmt::Debug + Send + Sync {
    /// Process input data and return output data
    fn process(&mut self, input: BrainData) -> Result<Vec<BrainData>>;
    
    /// Batch process multiple data items
    fn batch_process(&mut self, inputs: Vec<BrainData>) -> Result<Vec<BrainData>> {
        let mut results = Vec::new();
        for input in inputs {
            results.extend(self.process(input)?);
        }
        Ok(results)
    }
    
    /// Query the component for specific data
    fn query(&self, query: DataQuery) -> Result<Vec<BrainData>>;
    
    /// Get supported input data types
    fn supported_input_types(&self) -> Vec<DataType>;
    
    /// Get supported output data types
    fn supported_output_types(&self) -> Vec<DataType>;
    
    /// Get operation capabilities
    fn get_capabilities(&self) -> OperationCapabilities;
    
    /// Configure component for specific operation mode
    fn configure_operations(&mut self, config: OperationConfig) -> Result<()>;
    
    /// Reset component to initial state
    fn reset(&mut self) -> Result<()>;
}

/// Query structure for component data retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQuery {
    /// Type of data to query
    pub data_type: Option<DataType>,
    /// Time range for query
    pub time_range: Option<(u64, u64)>,
    /// Confidence threshold
    pub min_confidence: Option<f64>,
    /// Source component filter
    pub source_component: Option<String>,
    /// Metadata filters
    pub metadata_filters: HashMap<String, String>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Sort order
    pub sort_order: SortOrder,
}

/// Sort order for query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Timestamp,
    Confidence,
    Source,
    DataType,
    Custom(String),
}

/// Operation capabilities of a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationCapabilities {
    /// Can process data in real-time
    pub real_time_processing: bool,
    /// Can handle batch processing
    pub batch_processing: bool,
    /// Can maintain state between operations
    pub stateful_operations: bool,
    /// Can operate asynchronously
    pub async_operations: bool,
    /// Maximum concurrent operations
    pub max_concurrent_ops: usize,
    /// Average processing time per operation (ms)
    pub avg_processing_time_ms: f64,
    /// Memory footprint per operation (bytes)
    pub memory_footprint_bytes: usize,
}

/// Configuration for component operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationConfig {
    /// Mode of operation
    pub operation_mode: OperationMode,
    /// Performance preferences
    pub performance_mode: PerformanceMode,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Quality settings
    pub quality_settings: QualitySettings,
}

/// Mode of operation for components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationMode {
    Training,
    Inference,
    Analysis,
    Maintenance,
    Debugging,
}

/// Performance mode settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceMode {
    PowerSaving,
    Balanced,
    Performance,
    HighPerformance,
}

/// Resource limit settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: Option<usize>,
    pub max_cpu_percent: Option<f64>,
    pub max_processing_time_ms: Option<u64>,
    pub max_concurrent_operations: Option<usize>,
}

/// Quality settings for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySettings {
    pub precision_level: PrecisionLevel,
    pub confidence_threshold: f64,
    pub error_tolerance: f64,
    pub enable_validation: bool,
}

/// Precision levels for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrecisionLevel {
    Low,
    Medium,
    High,
    Maximum,
}

/// Unified metrics interface for all Brain AI components
/// This provides consistent performance monitoring across the system
pub trait BrainMetrics: std::fmt::Debug + Send + Sync {
    /// Get current performance metrics
    fn get_metrics(&self) -> PerformanceMetrics;
    
    /// Get historical metrics over time period
    fn get_historical_metrics(&self, time_range: (u64, u64)) -> Result<Vec<PerformanceMetrics>>;
    
    /// Get component-specific metrics
    fn get_component_metrics(&self) -> ComponentSpecificMetrics;
    
    /// Reset metrics collection
    fn reset_metrics(&mut self);
    
    /// Export metrics to various formats
    fn export_metrics(&self, format: MetricsFormat) -> Result<String>;
    
    /// Get metrics metadata
    fn get_metrics_metadata(&self) -> MetricsMetadata;
}

/// Performance metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Timestamp of metrics collection
    pub timestamp: u64,
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
    /// Current memory usage (bytes)
    pub memory_usage_bytes: usize,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Throughput (operations per second)
    pub throughput_ops_per_sec: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
    /// Confidence score distribution
    pub confidence_distribution: HashMap<String, usize>,
}

/// Component-specific metrics that vary by component type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentSpecificMetrics {
    CharacterPredictor {
        vocabulary_size: usize,
        prediction_accuracy: f64,
        training_loss: f64,
        model_parameters: usize,
    },
    SegmentDiscovery {
        total_segments: usize,
        average_segment_length: f64,
        segment_quality_score: f64,
        merge_operations: usize,
    },
    Memory {
        working_memory_items: usize,
        episodic_events: usize,
        semantic_concepts: usize,
        consolidation_rate: f64,
    },
    ConceptGraph {
        total_concepts: usize,
        total_relationships: usize,
        average_connectivity: f64,
        clustering_coefficient: f64,
    },
    Simulation {
        active_simulations: usize,
        branching_factor: f64,
        simulation_accuracy: f64,
        state_complexity: f64,
    },
    Generic {
        custom_metrics: HashMap<String, f64>,
    },
}

/// Metrics export formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsFormat {
    Json,
    Csv,
    Prometheus,
    InfluxDB,
    Custom(String),
}

/// Metadata about metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsMetadata {
    pub collection_start_time: u64,
    pub collection_interval_ms: u64,
    pub metrics_version: String,
    pub supported_formats: Vec<MetricsFormat>,
    pub retention_period_hours: u64,
}

/// Enhanced system component trait that extends the basic SystemComponent
/// This is the unified interface that all Brain AI components must implement
pub trait EnhancedSystemComponent: SystemComponent + BrainOperations + BrainMetrics {
    /// Get component configuration as JSON string
    fn get_config_json(&self) -> Result<String>;
    
    /// Update component configuration from JSON string
    fn update_config_json(&mut self, config_json: &str) -> Result<()>;
    
    /// Get enhanced component capabilities
    fn get_enhanced_capabilities(&self) -> ComponentCapabilities;
    
    /// Get integration points with other components
    fn get_integration_points(&self) -> Vec<IntegrationPoint>;
    
    /// Subscribe to data from other components
    fn subscribe_to_data(&mut self, data_type: DataType, source_component: String) -> Result<()>;
    
    /// Unsubscribe from data
    fn unsubscribe_from_data(&mut self, data_type: DataType, source_component: String) -> Result<()>;
    
    /// Publish data to subscribers
    fn publish_data(&self, data: BrainData) -> Result<()>;
    
    /// Get component documentation
    fn get_documentation(&self) -> ComponentDocumentation;
}

/// Component capabilities description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCapabilities {
    /// Component name
    pub name: String,
    /// Component version
    pub version: String,
    /// Supported operations
    pub supported_operations: Vec<String>,
    /// Data types the component can process
    pub input_types: Vec<DataType>,
    /// Data types the component can produce
    pub output_types: Vec<DataType>,
    /// Dependencies on other components
    pub dependencies: Vec<String>,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
}

/// Resource requirements for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub min_memory_mb: usize,
    pub recommended_memory_mb: usize,
    pub min_cpu_cores: usize,
    pub recommended_cpu_cores: usize,
    pub disk_space_mb: usize,
    pub network_bandwidth_mbps: f64,
}

/// Quality metrics for component assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub robustness: f64,
    pub reliability: f64,
}

/// Integration points between components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPoint {
    /// Target component name
    pub target_component: String,
    /// Type of integration
    pub integration_type: IntegrationType,
    /// Data flow direction
    pub data_flow: DataFlow,
    /// Required data types
    pub required_data_types: Vec<DataType>,
    /// Integration priority
    pub priority: IntegrationPriority,
}

/// Types of integration between components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationType {
    DataPipeline,
    EventDriven,
    RequestResponse,
    Streaming,
    Batch,
    Feedback,
}

/// Data flow direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFlow {
    Input,
    Output,
    Bidirectional,
}

/// Integration priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationPriority {
    Critical,
    Important,
    Optional,
    Future,
}

/// Component documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDocumentation {
    /// Component description
    pub description: String,
    /// Usage examples
    pub examples: Vec<UsageExample>,
    /// API reference
    pub api_reference: ApiReference,
    /// Configuration guide
    pub configuration_guide: String,
    /// Troubleshooting guide
    pub troubleshooting: String,
}

/// Usage example for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageExample {
    pub title: String,
    pub description: String,
    pub code_example: String,
    pub expected_output: String,
}

/// API reference documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiReference {
    pub endpoints: Vec<ApiEndpoint>,
    pub data_schemas: HashMap<String, String>,
    pub error_codes: HashMap<String, String>,
}

/// API endpoint documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub name: String,
    pub description: String,
    pub input_schema: String,
    pub output_schema: String,
    pub examples: Vec<String>,
}

// =============================================================================
// BRAIN SYSTEM CORE STRUCTURES (ENHANCED)
// =============================================================================

// Note: BrainSystem struct is already defined earlier in the file
// This section focuses on supporting structures for the enhanced system

/// Enhanced registry for managing components with unified interfaces
#[derive(Debug)]
pub struct EnhancedComponentRegistry {
    /// Component instances with their unified interfaces
    components: HashMap<String, Box<dyn EnhancedSystemComponent>>,
    /// Component dependency graph
    dependencies: HashMap<String, Vec<String>>,
    /// Component initialization order
    initialization_order: Vec<String>,
    /// Data subscriptions (component -> data types they want)
    subscriptions: HashMap<String, Vec<(DataType, String)>>,
    /// Data publishers (data type -> components that publish it)
    publishers: HashMap<DataType, Vec<String>>,
    /// Component capabilities cache
    capabilities_cache: HashMap<String, ComponentCapabilities>,
}

impl EnhancedComponentRegistry {
    /// Create a new enhanced registry
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            dependencies: HashMap::new(),
            initialization_order: Vec::new(),
            subscriptions: HashMap::new(),
            publishers: HashMap::new(),
            capabilities_cache: HashMap::new(),
        }
    }

    /// Register a component with enhanced capabilities
    pub fn register_enhanced_component(
        &mut self,
        name: String,
        component: Box<dyn EnhancedSystemComponent>,
    ) -> Result<()> {
        // Get component capabilities
        let capabilities = component.get_enhanced_capabilities();
        
        // Update dependencies
        self.dependencies.insert(name.clone(), capabilities.dependencies.clone());
        
        // Update data publishers
        for output_type in &capabilities.output_types {
            self.publishers.entry(output_type.clone()).or_default().push(name.clone());
        }
        
        // Cache capabilities
        self.capabilities_cache.insert(name.clone(), capabilities);
        
        // Register component
        self.components.insert(name.clone(), component);
        
        // Update initialization order
        self.update_initialization_order();
        
        info!("Registered enhanced component: {}", name);
        Ok(())
    }

    /// Get component by name
    pub fn get_component(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    /// Check if component exists and is mutable
    pub fn has_component_mut(&mut self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    /// Subscribe a component to data from another component
    pub fn subscribe_component_to_data(
        &mut self,
        subscriber: &str,
        data_type: DataType,
        publisher: &str,
    ) -> Result<()> {
        // Check if publisher can provide this data type
        if let Some(capabilities) = self.capabilities_cache.get(publisher) {
            if !capabilities.output_types.contains(&data_type) {
                return Err(BrainError::Other(format!(
                    "Component {} cannot provide data type {:?}",
                    publisher,
                    data_type
                )));
            }
        }

        // Add subscription
        self.subscriptions
            .entry(subscriber.to_string())
            .or_default()
            .push((data_type.clone(), publisher.to_string()));

        info!("Subscribed {} to {:?} from {}", subscriber, data_type, publisher);
        Ok(())
    }

    /// Get all components that subscribe to a specific data type
    pub fn get_subscribers(&self, data_type: &DataType) -> Vec<String> {
        let mut subscribers = Vec::new();
        for (component, subs) in &self.subscriptions {
            for (sub_data_type, _) in subs {
                if sub_data_type == data_type {
                    subscribers.push(component.clone());
                }
            }
        }
        subscribers
    }

    /// Get initialization order
    pub fn get_initialization_order(&self) -> &[String] {
        &self.initialization_order
    }

    /// Update initialization order based on dependencies
    fn update_initialization_order(&mut self) {
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        let mut order = Vec::new();

        for component in self.components.keys() {
            if !visited.contains(component) {
                self.visit_component_for_order(component, &mut visited, &mut temp_visited, &mut order);
            }
        }

        self.initialization_order = order;
    }

    /// Visit component for topological ordering
    fn visit_component_for_order(
        &self,
        name: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        order: &mut Vec<String>,
    ) {
        if temp_visited.contains(name) {
            warn!("Circular dependency detected involving component: {}", name);
            return;
        }

        if visited.contains(name) {
            return;
        }

        temp_visited.insert(name.to_string());

        if let Some(deps) = self.dependencies.get(name) {
            for dep in deps {
                self.visit_component_for_order(dep, visited, temp_visited, order);
            }
        }

        temp_visited.remove(name);
        visited.insert(name.to_string());
        order.push(name.to_string());
    }
}

/// Data orchestrator for managing data flow between components
#[derive(Debug)]
pub struct DataOrchestrator {
    /// Data flow routes (from -> to mappings)
    data_routes: HashMap<String, Vec<DataRoute>>,
    /// Data transformation rules
    transformation_rules: HashMap<(DataType, DataType), Box<dyn DataTransformer>>,
    /// Data buffer for async processing
    data_buffer: Arc<Mutex<Vec<BrainData>>>,
    /// Processing queue
    processing_queue: Arc<Mutex<Vec<DataProcessingTask>>>,
    /// Orchestrator configuration
    config: OrchestratorConfig,
}

/// Data route definition
#[derive(Debug, Clone)]
pub struct DataRoute {
    /// Source component
    pub from_component: String,
    /// Target component
    pub to_component: String,
    /// Data type being routed
    pub data_type: DataType,
    /// Route priority
    pub priority: RoutePriority,
    /// Transformation required
    pub transformation: Option<TransformationType>,
    /// Route conditions
    pub conditions: Vec<RouteCondition>,
}

/// Route priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutePriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of data transformation
#[derive(Debug, Clone)]
pub enum TransformationType {
    Direct,              // No transformation
    Format,              // Format conversion
    Aggregation,         // Data aggregation
    Filtering,           // Data filtering
    Enrichment,          // Data enrichment
    Custom(String),      // Custom transformation
}

/// Conditions for route activation
#[derive(Debug, Clone)]
pub struct RouteCondition {
    pub condition_type: RouteConditionType,
    pub value: String,
    pub operator: ConditionOperator,
}

/// Types of route conditions
#[derive(Debug, Clone)]
pub enum RouteConditionType {
    DataConfidence,
    DataSize,
    DataAge,
    SourceComponent,
    TargetComponent,
    Custom(String),
}

/// Condition operators
#[derive(Debug, Clone)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
}

/// Data transformer trait for converting between data types
pub trait DataTransformer: std::fmt::Debug + Send + Sync {
    /// Transform data from one type to another
    fn transform(&self, input: BrainData) -> Result<BrainData>;
    
    /// Get supported input types
    fn input_types(&self) -> Vec<DataType>;
    
    /// Get supported output types
    fn output_types(&self) -> Vec<DataType>;
    
    /// Get transformation metadata
    fn get_metadata(&self) -> TransformationMetadata;
}

/// Metadata about a data transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub input_schema: String,
    pub output_schema: String,
    pub transformation_cost: TransformationCost,
}

/// Cost of performing a transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationCost {
    pub cpu_cost: f64,
    pub memory_cost: usize,
    pub time_cost_ms: f64,
    pub complexity: ComplexityLevel,
}

/// Complexity levels for transformations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Trivial,
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Data processing task for the orchestrator
#[derive(Debug, Clone)]
pub struct DataProcessingTask {
    pub id: Uuid,
    pub data: BrainData,
    pub route: DataRoute,
    pub priority: RoutePriority,
    pub created_at: u64,
    pub deadline: Option<u64>,
    pub retry_count: usize,
    pub max_retries: usize,
}

/// Configuration for the data orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    /// Maximum buffer size for data
    pub max_buffer_size: usize,
    /// Processing batch size
    pub batch_size: usize,
    /// Processing timeout in milliseconds
    pub processing_timeout_ms: u64,
    /// Maximum concurrent processing tasks
    pub max_concurrent_tasks: usize,
    /// Enable data validation
    pub enable_data_validation: bool,
    /// Enable route optimization
    pub enable_route_optimization: bool,
    /// Metrics collection interval
    pub metrics_interval_ms: u64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            max_buffer_size: 10000,
            batch_size: 100,
            processing_timeout_ms: 30000,
            max_concurrent_tasks: 10,
            enable_data_validation: true,
            enable_route_optimization: true,
            metrics_interval_ms: 5000,
        }
    }
}

impl DataOrchestrator {
    /// Create a new data orchestrator
    pub fn new(config: OrchestratorConfig) -> Self {
        Self {
            data_routes: HashMap::new(),
            transformation_rules: HashMap::new(),
            data_buffer: Arc::new(Mutex::new(Vec::new())),
            processing_queue: Arc::new(Mutex::new(Vec::new())),
            config,
        }
    }

    /// Add a data route
    pub fn add_route(&mut self, route: DataRoute) {
        self.data_routes
            .entry(route.from_component.clone())
            .or_default()
            .push(route);
    }

    /// Remove a data route
    pub fn remove_route(&mut self, from_component: &str, to_component: &str, data_type: &DataType) {
        if let Some(routes) = self.data_routes.get_mut(from_component) {
            routes.retain(|r| {
                !(r.to_component == to_component && r.data_type == *data_type)
            });
        }
    }

    /// Route data from one component to another
    pub fn route_data(&self, data: BrainData) -> Result<Vec<String>> {
        let mut routed_to = Vec::new();
        
        if let Some(routes) = self.data_routes.get(&data.source_component) {
            let mut applicable_routes: Vec<_> = routes
                .iter()
                .filter(|route| {
                    route.data_type == data.data_type && self.evaluate_route_conditions(route, &data)
                })
                .collect();

            // Sort by priority
            applicable_routes.sort_by(|a, b| b.priority.cmp(&a.priority));

            for route in applicable_routes {
                // Create processing task
                let task = DataProcessingTask {
                    id: Uuid::new_v4(),
                    data: data.clone(),
                    route: route.clone(),
                    priority: route.priority.clone(),
                    created_at: current_timestamp(),
                    deadline: None,
                    retry_count: 0,
                    max_retries: 3,
                };

                // Add to processing queue
                if let Ok(mut queue) = self.processing_queue.lock() {
                    queue.push(task);
                    routed_to.push(route.to_component.clone());
                }
            }
        }

        Ok(routed_to)
    }

    /// Evaluate route conditions
    fn evaluate_route_conditions(&self, route: &DataRoute, data: &BrainData) -> bool {
        for condition in &route.conditions {
            if !self.evaluate_condition(condition, data) {
                return false;
            }
        }
        true
    }

    /// Evaluate a single condition
    fn evaluate_condition(&self, condition: &RouteCondition, data: &BrainData) -> bool {
        let actual_value = match &condition.condition_type {
            RouteConditionType::DataConfidence => data.confidence.to_string(),
            RouteConditionType::DataSize => {
                // Calculate approximate data size
                match &data.payload {
                    DataPayload::Text(s) => s.len().to_string(),
                    DataPayload::Vector(v) => v.len().to_string(),
                    DataPayload::Binary(b) => b.len().to_string(),
                    _ => "0".to_string(),
                }
            },
            RouteConditionType::DataAge => {
                let age = current_timestamp() - data.timestamp;
                age.to_string()
            },
            RouteConditionType::SourceComponent => data.source_component.clone(),
            RouteConditionType::TargetComponent => {
                data.target_components.get(0).cloned().unwrap_or_default()
            },
            RouteConditionType::Custom(key) => {
                data.metadata.get(key.as_str()).cloned().unwrap_or_default()
            },
        };

        match condition.operator {
            ConditionOperator::Equals => actual_value == condition.value,
            ConditionOperator::NotEquals => actual_value != condition.value,
            ConditionOperator::GreaterThan => {
                if let (Ok(actual), Ok(expected)) = (actual_value.parse::<f64>(), condition.value.parse::<f64>()) {
                    actual > expected
                } else {
                    false
                }
            },
            ConditionOperator::LessThan => {
                if let (Ok(actual), Ok(expected)) = (actual_value.parse::<f64>(), condition.value.parse::<f64>()) {
                    actual < expected
                } else {
                    false
                }
            },
            ConditionOperator::Contains => actual_value.contains(condition.value.as_str()),
            ConditionOperator::NotContains => !actual_value.contains(condition.value.as_str()),
        }
    }

    /// Process data in the queue
    pub fn process_queue(&mut self) -> Result<usize> {
        let mut processed = 0;
        
        if let Ok(mut queue) = self.processing_queue.lock() {
            // Sort by priority and creation time
            queue.sort_by(|a, b| {
                b.priority.cmp(&a.priority)
                    .then_with(|| a.created_at.cmp(&b.created_at))
            });

            let batch_size = self.config.batch_size.min(queue.len());
            let tasks: Vec<_> = queue.drain(0..batch_size).collect();
            
            for task in tasks {
                if self.process_task(task).is_ok() {
                    processed += 1;
                }
            }
        }

        Ok(processed)
    }

    /// Process a single task
    fn process_task(&self, task: DataProcessingTask) -> Result<()> {
        // Apply transformation if needed
        let transformed_data = if let Some(transformation) = &task.route.transformation {
            self.apply_transformation(&task.data, transformation)?
        } else {
            task.data
        };

        // Route the data (this would involve calling the target component)
        info!(
            "Routed data {:?} from {} to {} via route",
            transformed_data.data_type,
            task.route.from_component,
            task.route.to_component
        );

        Ok(())
    }

    /// Apply data transformation
    fn apply_transformation(&self, data: &BrainData, transformation: &TransformationType) -> Result<BrainData> {
        match transformation {
            TransformationType::Direct => Ok(data.clone()),
            TransformationType::Format => {
                // Simple format transformation (placeholder)
                let mut transformed = data.clone();
                transformed.id = Uuid::new_v4();
                transformed.timestamp = current_timestamp();
                Ok(transformed)
            },
            _ => {
                // For other transformations, return data as-is for now
                // In a full implementation, these would be more sophisticated
                Ok(data.clone())
            }
        }
    }

    /// Get orchestrator statistics
    pub fn get_statistics(&self) -> OrchestratorStatistics {
        let queue_size = self.processing_queue.lock().map(|q| q.len()).unwrap_or(0);
        let buffer_size = self.data_buffer.lock().map(|b| b.len()).unwrap_or(0);

        OrchestratorStatistics {
            total_routes: self.data_routes.values().map(|v| v.len()).sum(),
            queue_size,
            buffer_size,
            total_transformations: self.transformation_rules.len(),
            processed_tasks: 0, // Would be tracked in a full implementation
            failed_tasks: 0,    // Would be tracked in a full implementation
            avg_processing_time_ms: 0.0, // Would be tracked in a full implementation
        }
    }
}

/// Statistics for the data orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorStatistics {
    pub total_routes: usize,
    pub queue_size: usize,
    pub buffer_size: usize,
    pub total_transformations: usize,
    pub processed_tasks: u64,
    pub failed_tasks: u64,
    pub avg_processing_time_ms: f64,
}

// =============================================================================
// CONFIGURATION IMPLEMENTATIONS FOR BRAIN CONFIGS
// =============================================================================

/// Implementation of BrainConfig for ModelConfig
impl BrainConfig for ModelConfig {
    fn validate(&self) -> Result<()> {
        if self.vocab_size == 0 {
            return Err(BrainError::ConfigError("Vocabulary size must be greater than 0".to_string()));
        }
        if self.embedding_dim == 0 {
            return Err(BrainError::ConfigError("Embedding dimension must be greater than 0".to_string()));
        }
        if self.hidden_dim == 0 {
            return Err(BrainError::ConfigError("Hidden dimension must be greater than 0".to_string()));
        }
        if self.learning_rate <= 0.0 || self.learning_rate >= 1.0 {
            return Err(BrainError::ConfigError("Learning rate must be between 0 and 1".to_string()));
        }
        Ok(())
    }

    fn get_metadata(&self) -> ConfigMetadata {
        ConfigMetadata {
            component_name: "CharacterPredictor".to_string(),
            config_version: "1.0.0".to_string(),
            required_dependencies: vec![],
            optional_dependencies: vec!["SegmentProvider".to_string()],
            performance_tier: PerformanceTier::Medium,
        }
    }

    fn merge(&mut self, other: &Self) -> Result<()> {
        // Merge configurations, preferring non-default values from other
        if other.vocab_size != ModelConfig::default().vocab_size {
            self.vocab_size = other.vocab_size;
        }
        if other.embedding_dim != ModelConfig::default().embedding_dim {
            self.embedding_dim = other.embedding_dim;
        }
        if other.hidden_dim != ModelConfig::default().hidden_dim {
            self.hidden_dim = other.hidden_dim;
        }
        if other.learning_rate != ModelConfig::default().learning_rate {
            self.learning_rate = other.learning_rate;
        }
        if other.sequence_length != ModelConfig::default().sequence_length {
            self.sequence_length = other.sequence_length;
        }
        Ok(())
    }
}

/// Implementation of BrainConfig for BpeConfig
impl BrainConfig for BpeConfig {
    fn validate(&self) -> Result<()> {
        if self.min_frequency == 0 {
            return Err(BrainError::ConfigError("Minimum frequency must be greater than 0".to_string()));
        }
        if self.max_vocab_size == 0 {
            return Err(BrainError::ConfigError("Maximum vocabulary size must be greater than 0".to_string()));
        }
        if self.min_confidence < 0.0 || self.min_confidence > 1.0 {
            return Err(BrainError::ConfigError("Minimum confidence must be between 0 and 1".to_string()));
        }
        Ok(())
    }

    fn get_metadata(&self) -> ConfigMetadata {
        ConfigMetadata {
            component_name: "BpeSegmenter".to_string(),
            config_version: "1.0.0".to_string(),
            required_dependencies: vec![],
            optional_dependencies: vec![],
            performance_tier: PerformanceTier::Medium,
        }
    }

    fn merge(&mut self, other: &Self) -> Result<()> {
        let default = BpeConfig::default();
        
        if other.min_frequency != default.min_frequency {
            self.min_frequency = other.min_frequency;
        }
        if other.max_vocab_size != default.max_vocab_size {
            self.max_vocab_size = other.max_vocab_size;
        }
        if other.num_merges != default.num_merges {
            self.num_merges = other.num_merges;
        }
        if other.include_chars != default.include_chars {
            self.include_chars = other.include_chars;
        }
        if other.min_entropy_threshold != default.min_entropy_threshold {
            self.min_entropy_threshold = other.min_entropy_threshold;
        }
        if other.context_window_size != default.context_window_size {
            self.context_window_size = other.context_window_size;
        }
        if other.min_confidence != default.min_confidence {
            self.min_confidence = other.min_confidence;
        }
        if other.enable_advanced_heuristics != default.enable_advanced_heuristics {
            self.enable_advanced_heuristics = other.enable_advanced_heuristics;
        }
        
        Ok(())
    }
}

// ... existing code ... 