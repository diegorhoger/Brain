//! Performance Monitoring and Optimization Infrastructure
//! 
//! This module provides comprehensive performance monitoring, profiling, and optimization
//! capabilities for the Brain AI system, including real-time metrics collection,
//! bottleneck identification, resource usage tracking, and performance alerting.

use brain_types::{Result, BrainError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::interval;
use serde::{Deserialize, Serialize};
use log::{info, warn, debug};
use uuid::Uuid;
use sysinfo::System;

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Metrics collection interval in milliseconds
    pub collection_interval_ms: u64,
    /// Enable real-time profiling
    pub enable_profiling: bool,
    /// Enable CPU profiling
    pub enable_cpu_profiling: bool,
    /// Enable memory profiling
    pub enable_memory_profiling: bool,
    /// Enable I/O monitoring
    pub enable_io_monitoring: bool,
    /// Performance alert thresholds
    pub alert_thresholds: AlertThresholds,
    /// Maximum metrics history to retain
    pub max_history_entries: usize,
    /// Enable performance dashboards
    pub enable_dashboards: bool,
    /// Export metrics to Prometheus
    pub export_prometheus: bool,
    /// Prometheus export port
    pub prometheus_port: u16,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval_ms: 1000, // 1 second
            enable_profiling: true,
            enable_cpu_profiling: true,
            enable_memory_profiling: true,
            enable_io_monitoring: true,
            alert_thresholds: AlertThresholds::default(),
            max_history_entries: 3600, // 1 hour at 1-second intervals
            enable_dashboards: true,
            export_prometheus: true,
            prometheus_port: 9090,
        }
    }
}

/// Alert thresholds for performance monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU usage percentage threshold
    pub cpu_usage_percent: f64,
    /// Memory usage percentage threshold
    pub memory_usage_percent: f64,
    /// Response time threshold in milliseconds
    pub response_time_ms: f64,
    /// Error rate percentage threshold
    pub error_rate_percent: f64,
    /// Disk usage percentage threshold
    pub disk_usage_percent: f64,
    /// Network latency threshold in milliseconds
    pub network_latency_ms: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 80.0,
            memory_usage_percent: 85.0,
            response_time_ms: 1000.0,
            error_rate_percent: 5.0,
            disk_usage_percent: 90.0,
            network_latency_ms: 500.0,
        }
    }
}

/// System metrics collector using sysinfo
#[derive(Debug)]
pub struct SystemMetricsCollector {
    system: System,
}

impl SystemMetricsCollector {
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();
        Ok(Self { system })
    }
    
    pub fn collect_metrics(&mut self) -> Result<SystemMetrics> {
        self.system.refresh_all();
        
        let cpu_usage_percent = self.system.global_cpu_info().cpu_usage() as f64;
        let memory_total_bytes = self.system.total_memory();
        let memory_used_bytes = self.system.used_memory();
        let memory_available_bytes = memory_total_bytes - memory_used_bytes;
        let disk_total_bytes = self.get_total_disk_space();
        let disk_used_bytes = self.get_used_disk_space();
        let network_rx_bytes = self.get_network_rx_bytes();
        let network_tx_bytes = self.get_network_tx_bytes();
        let load_average = self.get_load_average();
        let process_count = self.system.processes().len();
        let uptime_seconds = System::uptime();
        
        Ok(SystemMetrics {
            cpu_usage_percent,
            memory_total_bytes,
            memory_used_bytes,
            memory_available_bytes,
            disk_total_bytes,
            disk_used_bytes,
            network_rx_bytes,
            network_tx_bytes,
            load_average,
            process_count,
            uptime_seconds,
        })
    }
    
    pub fn get_current_metrics(&mut self) -> Result<SystemMetrics> {
        self.collect_metrics()
    }
    
    fn get_total_disk_space(&self) -> u64 {
        // Placeholder implementation - in real usage would enumerate disks
        1_000_000_000_000 // 1TB default
    }
    
    fn get_used_disk_space(&self) -> u64 {
        // Placeholder implementation - in real usage would calculate used space
        500_000_000_000 // 500GB default
    }
    
    fn get_network_rx_bytes(&self) -> u64 {
        // Placeholder implementation - in real usage would sum network interfaces
        0
    }
    
    fn get_network_tx_bytes(&self) -> u64 {
        // Placeholder implementation - in real usage would sum network interfaces
        0
    }
    
    fn get_load_average(&self) -> f64 {
        System::load_average().one
    }
}

/// Component performance tracking
#[derive(Debug)]
pub struct ComponentPerformanceTracker {
    component_metrics: HashMap<String, ComponentPerformanceMetrics>,
}

impl ComponentPerformanceTracker {
    pub fn new() -> Self {
        Self {
            component_metrics: HashMap::new(),
        }
    }
    
    pub fn record_operation(
        &mut self,
        component_name: &str,
        operation: &str,
        duration: Duration,
        success: bool,
    ) -> Result<()> {
        let metrics = self.component_metrics
            .entry(component_name.to_string())
            .or_insert_with(ComponentPerformanceMetrics::new);
        
        metrics.record_operation(operation, duration, success);
        Ok(())
    }
    
    pub fn get_component_metrics(&self, component_name: &str) -> Option<&ComponentPerformanceMetrics> {
        self.component_metrics.get(component_name)
    }
    
    pub fn get_all_metrics(&self) -> HashMap<String, ComponentPerformanceMetrics> {
        self.component_metrics.clone()
    }
}

/// Performance metrics for a specific component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentPerformanceMetrics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_duration_ms: f64,
    pub min_duration_ms: f64,
    pub max_duration_ms: f64,
    pub operations_per_second: f64,
    pub error_rate_percent: f64,
    pub operation_breakdown: HashMap<String, OperationMetrics>,
    pub last_updated: u64,
}

impl ComponentPerformanceMetrics {
    pub fn new() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            average_duration_ms: 0.0,
            min_duration_ms: f64::MAX,
            max_duration_ms: 0.0,
            operations_per_second: 0.0,
            error_rate_percent: 0.0,
            operation_breakdown: HashMap::new(),
            last_updated: current_timestamp(),
        }
    }
    
    pub fn record_operation(&mut self, operation: &str, duration: Duration, success: bool) {
        let duration_ms = duration.as_millis() as f64;
        
        self.total_operations += 1;
        if success {
            self.successful_operations += 1;
        } else {
            self.failed_operations += 1;
        }
        
        // Update duration statistics
        self.average_duration_ms = (self.average_duration_ms * (self.total_operations - 1) as f64 + duration_ms) / self.total_operations as f64;
        self.min_duration_ms = self.min_duration_ms.min(duration_ms);
        self.max_duration_ms = self.max_duration_ms.max(duration_ms);
        
        // Update error rate
        self.error_rate_percent = (self.failed_operations as f64 / self.total_operations as f64) * 100.0;
        
        // Update operation breakdown
        let op_metrics = self.operation_breakdown
            .entry(operation.to_string())
            .or_insert_with(OperationMetrics::new);
        op_metrics.record_operation(duration, success);
        
        self.last_updated = current_timestamp();
    }
}

/// Metrics for a specific operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetrics {
    pub count: u64,
    pub successful_count: u64,
    pub failed_count: u64,
    pub average_duration_ms: f64,
    pub min_duration_ms: f64,
    pub max_duration_ms: f64,
}

impl OperationMetrics {
    pub fn new() -> Self {
        Self {
            count: 0,
            successful_count: 0,
            failed_count: 0,
            average_duration_ms: 0.0,
            min_duration_ms: f64::MAX,
            max_duration_ms: 0.0,
        }
    }
    
    pub fn record_operation(&mut self, duration: Duration, success: bool) {
        let duration_ms = duration.as_millis() as f64;
        
        self.count += 1;
        if success {
            self.successful_count += 1;
        } else {
            self.failed_count += 1;
        }
        
        self.average_duration_ms = (self.average_duration_ms * (self.count - 1) as f64 + duration_ms) / self.count as f64;
        self.min_duration_ms = self.min_duration_ms.min(duration_ms);
        self.max_duration_ms = self.max_duration_ms.max(duration_ms);
    }
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f64,
    pub memory_total_bytes: u64,
    pub memory_used_bytes: u64,
    pub memory_available_bytes: u64,
    pub disk_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub load_average: f64,
    pub process_count: usize,
    pub uptime_seconds: u64,
}

/// Performance profiler for detailed analysis
#[derive(Debug)]
pub struct PerformanceProfiler {
    config: PerformanceConfig,
    cpu_profiler_active: bool,
    memory_profiler_active: bool,
    profiling_start_time: Option<Instant>,
}

impl PerformanceProfiler {
    pub fn new(config: &PerformanceConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            cpu_profiler_active: false,
            memory_profiler_active: false,
            profiling_start_time: None,
        })
    }
    
    pub fn start_profiling(&mut self) -> Result<()> {
        info!("Starting performance profiling");
        
        if self.config.enable_cpu_profiling {
            self.start_cpu_profiling()?;
        }
        
        if self.config.enable_memory_profiling {
            self.start_memory_profiling()?;
        }
        
        self.profiling_start_time = Some(Instant::now());
        Ok(())
    }
    
    pub fn stop_profiling(&mut self) -> Result<()> {
        info!("Stopping performance profiling");
        
        if self.cpu_profiler_active {
            self.stop_cpu_profiling()?;
        }
        
        if self.memory_profiler_active {
            self.stop_memory_profiling()?;
        }
        
        self.profiling_start_time = None;
        Ok(())
    }
    
    pub fn get_current_profile(&self) -> Result<ProfilerData> {
        let profiling_duration = self.profiling_start_time
            .map(|start| start.elapsed())
            .unwrap_or_default();
        
        Ok(ProfilerData {
            cpu_profile_available: self.cpu_profiler_active,
            memory_profile_available: self.memory_profiler_active,
            profiling_duration,
        })
    }
    
    fn start_cpu_profiling(&mut self) -> Result<()> {
        // Placeholder for CPU profiling implementation
        self.cpu_profiler_active = true;
        debug!("CPU profiling started");
        Ok(())
    }
    
    fn stop_cpu_profiling(&mut self) -> Result<()> {
        // Placeholder for CPU profiling stop
        self.cpu_profiler_active = false;
        debug!("CPU profiling stopped");
        Ok(())
    }
    
    fn start_memory_profiling(&mut self) -> Result<()> {
        // Placeholder for memory profiling implementation
        self.memory_profiler_active = true;
        debug!("Memory profiling started");
        Ok(())
    }
    
    fn stop_memory_profiling(&mut self) -> Result<()> {
        // Placeholder for memory profiling stop
        self.memory_profiler_active = false;
        debug!("Memory profiling stopped");
        Ok(())
    }
}

/// Profiler data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerData {
    pub cpu_profile_available: bool,
    pub memory_profile_available: bool,
    pub profiling_duration: Duration,
}

/// Alert management system
#[derive(Debug)]
pub struct AlertManager {
    thresholds: AlertThresholds,
    active_alerts: Vec<PerformanceAlert>,
    alert_history: Vec<PerformanceAlert>,
}

impl AlertManager {
    pub fn new(thresholds: AlertThresholds) -> Self {
        Self {
            thresholds,
            active_alerts: Vec::new(),
            alert_history: Vec::new(),
        }
    }
    
    pub fn check_system_metrics(&mut self, metrics: &SystemMetrics) -> Result<()> {
        // Check CPU usage
        if metrics.cpu_usage_percent > self.thresholds.cpu_usage_percent {
            self.trigger_alert(
                AlertType::HighCpuUsage,
                format!("CPU usage is {}%, exceeding threshold of {}%", 
                       metrics.cpu_usage_percent, self.thresholds.cpu_usage_percent),
            )?;
        }
        
        // Check memory usage
        let memory_usage_percent = (metrics.memory_used_bytes as f64 / metrics.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > self.thresholds.memory_usage_percent {
            self.trigger_alert(
                AlertType::HighMemoryUsage,
                format!("Memory usage is {:.1}%, exceeding threshold of {}%", 
                       memory_usage_percent, self.thresholds.memory_usage_percent),
            )?;
        }
        
        // Check disk usage
        let disk_usage_percent = (metrics.disk_used_bytes as f64 / metrics.disk_total_bytes as f64) * 100.0;
        if disk_usage_percent > self.thresholds.disk_usage_percent {
            self.trigger_alert(
                AlertType::HighDiskUsage,
                format!("Disk usage is {:.1}%, exceeding threshold of {}%", 
                       disk_usage_percent, self.thresholds.disk_usage_percent),
            )?;
        }
        
        Ok(())
    }
    
    pub fn check_operation_performance(
        &mut self,
        component: &str,
        operation: &str,
        duration: Duration,
        success: bool,
    ) -> Result<()> {
        let duration_ms = duration.as_millis() as f64;
        
        // Check response time
        if duration_ms > self.thresholds.response_time_ms {
            self.trigger_alert(
                AlertType::SlowResponse,
                format!("Operation {}::{} took {:.1}ms, exceeding threshold of {}ms", 
                       component, operation, duration_ms, self.thresholds.response_time_ms),
            )?;
        }
        
        // Check operation failure
        if !success {
            self.trigger_alert(
                AlertType::OperationFailure,
                format!("Operation {}::{} failed", component, operation),
            )?;
        }
        
        Ok(())
    }
    
    pub fn get_active_alerts(&self) -> Vec<PerformanceAlert> {
        self.active_alerts.clone()
    }
    
    fn trigger_alert(&mut self, alert_type: AlertType, message: String) -> Result<()> {
        let severity = match alert_type {
            AlertType::HighCpuUsage | AlertType::HighMemoryUsage => AlertSeverity::Warning,
            AlertType::HighDiskUsage | AlertType::SystemOverload => AlertSeverity::Critical,
            AlertType::SlowResponse | AlertType::NetworkLatency => AlertSeverity::Warning,
            AlertType::OperationFailure => AlertSeverity::Info,
        };
        
        let alert = PerformanceAlert {
            id: Uuid::new_v4(),
            alert_type,
            message: message.clone(),
            timestamp: current_timestamp(),
            severity,
            resolved: false,
        };
        
        warn!("Performance alert triggered: {}", message);
        
        self.active_alerts.push(alert.clone());
        self.alert_history.push(alert);
        
        // Limit active alerts to prevent memory growth
        if self.active_alerts.len() > 100 {
            self.active_alerts.drain(0..50);
        }
        
        Ok(())
    }
}

/// Performance alert structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub id: Uuid,
    pub alert_type: AlertType,
    pub message: String,
    pub timestamp: u64,
    pub severity: AlertSeverity,
    pub resolved: bool,
}

/// Types of performance alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    HighCpuUsage,
    HighMemoryUsage,
    HighDiskUsage,
    SlowResponse,
    OperationFailure,
    NetworkLatency,
    SystemOverload,
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::HighCpuUsage => write!(f, "High CPU Usage"),
            AlertType::HighMemoryUsage => write!(f, "High Memory Usage"),
            AlertType::HighDiskUsage => write!(f, "High Disk Usage"),
            AlertType::SlowResponse => write!(f, "Slow Response"),
            AlertType::OperationFailure => write!(f, "Operation Failure"),
            AlertType::NetworkLatency => write!(f, "Network Latency"),
            AlertType::SystemOverload => write!(f, "System Overload"),
        }
    }
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Performance snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub timestamp: u64,
    pub system_metrics: SystemMetrics,
    pub component_metrics: HashMap<String, ComponentPerformanceMetrics>,
    pub profiler_data: Option<ProfilerData>,
    pub alerts: Vec<PerformanceAlert>,
}

/// Performance optimization engine
#[derive(Debug)]
pub struct PerformanceOptimizer {
    optimization_rules: Vec<OptimizationRule>,
}

impl PerformanceOptimizer {
    pub fn new() -> Self {
        Self {
            optimization_rules: Self::create_default_rules(),
        }
    }
    
    pub fn identify_bottlenecks(&self, snapshot: &PerformanceSnapshot) -> Result<Vec<PerformanceBottleneck>> {
        let mut bottlenecks = Vec::new();
        
        // Check system-level bottlenecks
        let system_metrics = &snapshot.system_metrics;
        
        // CPU bottleneck
        if system_metrics.cpu_usage_percent > 80.0 {
            bottlenecks.push(PerformanceBottleneck {
                bottleneck_type: BottleneckType::HighCpuUsage,
                severity: if system_metrics.cpu_usage_percent > 95.0 {
                    BottleneckSeverity::Critical
                } else if system_metrics.cpu_usage_percent > 90.0 {
                    BottleneckSeverity::High
                } else {
                    BottleneckSeverity::Medium
                },
                description: format!("CPU usage at {:.1}%", system_metrics.cpu_usage_percent),
                component: None,
                operation: None,
                impact_score: system_metrics.cpu_usage_percent / 100.0,
            });
        }
        
        // Memory bottleneck
        let memory_usage_percent = (system_metrics.memory_used_bytes as f64 / system_metrics.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > 85.0 {
            bottlenecks.push(PerformanceBottleneck {
                bottleneck_type: BottleneckType::HighMemoryUsage,
                severity: if memory_usage_percent > 95.0 {
                    BottleneckSeverity::Critical
                } else if memory_usage_percent > 90.0 {
                    BottleneckSeverity::High
                } else {
                    BottleneckSeverity::Medium
                },
                description: format!("Memory usage at {:.1}%", memory_usage_percent),
                component: None,
                operation: None,
                impact_score: memory_usage_percent / 100.0,
            });
        }
        
        // Check component-level bottlenecks
        for (component_name, metrics) in &snapshot.component_metrics {
            if metrics.error_rate_percent > 5.0 {
                bottlenecks.push(PerformanceBottleneck {
                    bottleneck_type: BottleneckType::HighErrorRate,
                    severity: if metrics.error_rate_percent > 20.0 {
                        BottleneckSeverity::Critical
                    } else if metrics.error_rate_percent > 10.0 {
                        BottleneckSeverity::High
                    } else {
                        BottleneckSeverity::Medium
                    },
                    description: format!("Error rate at {:.1}%", metrics.error_rate_percent),
                    component: Some(component_name.clone()),
                    operation: None,
                    impact_score: metrics.error_rate_percent / 100.0,
                });
            }
            
            if metrics.average_duration_ms > 1000.0 {
                bottlenecks.push(PerformanceBottleneck {
                    bottleneck_type: BottleneckType::SlowOperations,
                    severity: if metrics.average_duration_ms > 5000.0 {
                        BottleneckSeverity::Critical
                    } else if metrics.average_duration_ms > 2000.0 {
                        BottleneckSeverity::High
                    } else {
                        BottleneckSeverity::Medium
                    },
                    description: format!("Average operation time {:.1}ms", metrics.average_duration_ms),
                    component: Some(component_name.clone()),
                    operation: None,
                    impact_score: (metrics.average_duration_ms / 5000.0).min(1.0),
                });
            }
        }
        
        Ok(bottlenecks)
    }
    
    pub fn generate_recommendations(&self, snapshot: &PerformanceSnapshot) -> Result<Vec<OptimizationRecommendation>> {
        let bottlenecks = self.identify_bottlenecks(snapshot)?;
        let mut recommendations = Vec::new();
        
        for bottleneck in &bottlenecks {
            for rule in &self.optimization_rules {
                if rule.applies_to_bottleneck(bottleneck) {
                    recommendations.extend(rule.generate_recommendations(bottleneck));
                }
            }
        }
        
        Ok(recommendations)
    }
    
    fn create_default_rules() -> Vec<OptimizationRule> {
        vec![
            OptimizationRule::new(
                "CPU Optimization",
                BottleneckType::HighCpuUsage,
                vec![
                    "Implement CPU-intensive operation caching",
                    "Consider parallel processing for CPU-bound tasks",
                    "Profile and optimize hot code paths",
                    "Use more efficient algorithms",
                ],
            ),
            OptimizationRule::new(
                "Memory Optimization",
                BottleneckType::HighMemoryUsage,
                vec![
                    "Implement memory pooling",
                    "Optimize data structures for memory efficiency",
                    "Add garbage collection tuning",
                    "Consider streaming for large data processing",
                ],
            ),
            OptimizationRule::new(
                "Error Rate Optimization",
                BottleneckType::HighErrorRate,
                vec![
                    "Implement retry mechanisms with exponential backoff",
                    "Add circuit breaker patterns",
                    "Improve error handling and recovery",
                    "Add input validation and sanitization",
                ],
            ),
            OptimizationRule::new(
                "Response Time Optimization",
                BottleneckType::SlowOperations,
                vec![
                    "Add caching layers",
                    "Implement asynchronous processing",
                    "Optimize database queries",
                    "Consider connection pooling",
                ],
            ),
        ]
    }
}

/// Performance bottleneck identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub bottleneck_type: BottleneckType,
    pub severity: BottleneckSeverity,
    pub description: String,
    pub component: Option<String>,
    pub operation: Option<String>,
    pub impact_score: f64,
}

/// Types of performance bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    HighCpuUsage,
    HighMemoryUsage,
    HighDiskUsage,
    SlowOperations,
    HighErrorRate,
    NetworkLatency,
    DatabaseSlowness,
    ConcurrencyIssues,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub estimated_impact: f64,
    pub implementation_effort: ImplementationEffort,
    pub component: Option<String>,
}

/// Types of optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    AlgorithmOptimization,
    CachingStrategy,
    DatabaseOptimization,
    MemoryManagement,
    ConcurrencyImprovement,
    ConfigurationTuning,
    ArchitecturalChange,
}

/// Recommendation priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Implementation effort estimates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Low,      // < 1 day
    Medium,   // 1-3 days
    High,     // 1-2 weeks
    VeryHigh, // > 2 weeks
}

/// Optimization rule for generating recommendations
#[derive(Debug, Clone)]
pub struct OptimizationRule {
    pub name: String,
    pub target_bottleneck: BottleneckType,
    pub recommendations: Vec<String>,
}

impl OptimizationRule {
    pub fn new(name: &str, target_bottleneck: BottleneckType, recommendations: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            target_bottleneck,
            recommendations: recommendations.iter().map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn applies_to_bottleneck(&self, bottleneck: &PerformanceBottleneck) -> bool {
        std::mem::discriminant(&self.target_bottleneck) == std::mem::discriminant(&bottleneck.bottleneck_type)
    }
    
    pub fn generate_recommendations(&self, bottleneck: &PerformanceBottleneck) -> Vec<OptimizationRecommendation> {
        self.recommendations.iter().map(|rec| {
            OptimizationRecommendation {
                recommendation_type: self.get_recommendation_type(&bottleneck.bottleneck_type),
                title: rec.clone(),
                description: format!("{} - {}", rec, bottleneck.description),
                priority: self.get_priority_from_severity(&bottleneck.severity),
                estimated_impact: bottleneck.impact_score,
                implementation_effort: ImplementationEffort::Medium,
                component: bottleneck.component.clone(),
            }
        }).collect()
    }
    
    fn get_recommendation_type(&self, bottleneck_type: &BottleneckType) -> RecommendationType {
        match bottleneck_type {
            BottleneckType::HighCpuUsage => RecommendationType::AlgorithmOptimization,
            BottleneckType::HighMemoryUsage => RecommendationType::MemoryManagement,
            BottleneckType::SlowOperations => RecommendationType::CachingStrategy,
            _ => RecommendationType::ConfigurationTuning,
        }
    }
    
    fn get_priority_from_severity(&self, severity: &BottleneckSeverity) -> RecommendationPriority {
        match severity {
            BottleneckSeverity::Low => RecommendationPriority::Low,
            BottleneckSeverity::Medium => RecommendationPriority::Medium,
            BottleneckSeverity::High => RecommendationPriority::High,
            BottleneckSeverity::Critical => RecommendationPriority::Critical,
        }
    }
}

/// Complete performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub timestamp: u64,
    pub snapshot: PerformanceSnapshot,
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub recommendations: Vec<OptimizationRecommendation>,
}

/// Report export formats
#[derive(Debug, Clone)]
pub enum ReportFormat {
    Json,
    Csv,
    Html,
}

/// Get current timestamp in seconds since UNIX epoch
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert!(config.enabled);
        assert_eq!(config.collection_interval_ms, 1000);
        assert!(config.enable_profiling);
    }

    #[test]
    fn test_component_performance_metrics() {
        let mut metrics = ComponentPerformanceMetrics::new();
        let duration = Duration::from_millis(100);
        
        metrics.record_operation("test_operation", duration, true);
        
        assert_eq!(metrics.total_operations, 1);
        assert_eq!(metrics.successful_operations, 1);
        assert_eq!(metrics.failed_operations, 0);
        assert_eq!(metrics.average_duration_ms, 100.0);
        assert_eq!(metrics.error_rate_percent, 0.0);
        
        // Test failed operation
        metrics.record_operation("test_operation", duration, false);
        assert_eq!(metrics.total_operations, 2);
        assert_eq!(metrics.failed_operations, 1);
        assert_eq!(metrics.error_rate_percent, 50.0);
    }

    #[tokio::test]
    async fn test_performance_monitor_creation() {
        let config = PerformanceConfig::default();
        let monitor = PerformanceMonitor::new(config);
        assert!(monitor.is_ok());
    }

    #[test]
    fn test_alert_manager() {
        let thresholds = AlertThresholds::default();
        let mut alert_manager = AlertManager::new(thresholds);
        
        let metrics = SystemMetrics {
            cpu_usage_percent: 90.0, // Above threshold
            memory_total_bytes: 1000,
            memory_used_bytes: 900,   // 90% usage, above threshold
            memory_available_bytes: 100,
            disk_total_bytes: 1000,
            disk_used_bytes: 500,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            load_average: 1.0,
            process_count: 10,
            uptime_seconds: 3600,
        };
        
        let result = alert_manager.check_system_metrics(&metrics);
        assert!(result.is_ok());
        
        let alerts = alert_manager.get_active_alerts();
        assert!(!alerts.is_empty());
    }

    #[test]
    fn test_performance_optimizer() {
        let optimizer = PerformanceOptimizer::new();
        
        let snapshot = PerformanceSnapshot {
            timestamp: current_timestamp(),
            system_metrics: SystemMetrics {
                cpu_usage_percent: 90.0,
                memory_total_bytes: 1000,
                memory_used_bytes: 950,
                memory_available_bytes: 50,
                disk_total_bytes: 1000,
                disk_used_bytes: 500,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
                load_average: 2.0,
                process_count: 50,
                uptime_seconds: 3600,
            },
            component_metrics: HashMap::new(),
            profiler_data: None,
            alerts: vec![],
        };
        
        let bottlenecks = optimizer.identify_bottlenecks(&snapshot);
        assert!(bottlenecks.is_ok());
        
        let recommendations = optimizer.generate_recommendations(&snapshot);
        assert!(recommendations.is_ok());
    }
}

/// Main performance monitoring system
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Configuration
    config: PerformanceConfig,
    /// System metrics collector
    system_metrics: Arc<RwLock<SystemMetricsCollector>>,
    /// Component performance tracker
    component_tracker: Arc<RwLock<ComponentPerformanceTracker>>,
    /// Performance profiler
    profiler: Arc<Mutex<PerformanceProfiler>>,
    /// Alert manager
    alert_manager: Arc<Mutex<AlertManager>>,
    /// Metrics history
    metrics_history: Arc<RwLock<Vec<PerformanceSnapshot>>>,
    /// Performance optimizer
    optimizer: Arc<Mutex<PerformanceOptimizer>>,
    /// Running status
    is_running: Arc<Mutex<bool>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(config: PerformanceConfig) -> Result<Self> {
        let system_metrics = Arc::new(RwLock::new(SystemMetricsCollector::new()?));
        let component_tracker = Arc::new(RwLock::new(ComponentPerformanceTracker::new()));
        let profiler = Arc::new(Mutex::new(PerformanceProfiler::new(&config)?));
        let alert_manager = Arc::new(Mutex::new(AlertManager::new(config.alert_thresholds.clone())));
        let metrics_history = Arc::new(RwLock::new(Vec::new()));
        let optimizer = Arc::new(Mutex::new(PerformanceOptimizer::new()));
        
        Ok(Self {
            config,
            system_metrics,
            component_tracker,
            profiler,
            alert_manager,
            metrics_history,
            optimizer,
            is_running: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Start performance monitoring
    pub async fn start(&self) -> Result<()> {
        {
            let mut running = self.is_running.lock().unwrap();
            if *running {
                return Ok(());
            }
            *running = true;
        }
        
        info!("Starting performance monitoring system");
        
        // Start system metrics collection
        self.start_system_metrics_collection().await?;
        
        // Start profiling if enabled
        if self.config.enable_profiling {
            self.start_profiling().await?;
        }
        
        // Start Prometheus exporter if enabled
        if self.config.export_prometheus {
            self.start_prometheus_exporter().await?;
        }
        
        info!("Performance monitoring system started successfully");
        Ok(())
    }
    
    /// Stop performance monitoring
    pub async fn stop(&self) -> Result<()> {
        {
            let mut running = self.is_running.lock().unwrap();
            if !*running {
                return Ok(());
            }
            *running = false;
        }
        
        info!("Stopping performance monitoring system");
        
        // Stop profiling
        if let Ok(mut profiler) = self.profiler.lock() {
            profiler.stop_profiling()?;
        }
        
        info!("Performance monitoring system stopped");
        Ok(())
    }
    
    /// Record component operation
    pub fn record_operation(
        &self,
        component_name: &str,
        operation: &str,
        duration: Duration,
        success: bool,
    ) -> Result<()> {
        if let Ok(mut tracker) = self.component_tracker.write() {
            tracker.record_operation(component_name, operation, duration, success)?;
        }
        
        // Check for performance alerts
        if let Ok(mut alert_manager) = self.alert_manager.lock() {
            alert_manager.check_operation_performance(component_name, operation, duration, success)?;
        }
        
        Ok(())
    }
    
    /// Get current performance snapshot
    pub fn get_current_snapshot(&self) -> Result<PerformanceSnapshot> {
        let timestamp = current_timestamp();
        
        let system_metrics = {
            let mut collector = self.system_metrics.write().unwrap();
            collector.collect_metrics()?
        };
        
        let component_metrics = {
            let tracker = self.component_tracker.read().unwrap();
            tracker.get_all_metrics()
        };
        
        let profiler_data = {
            let profiler = self.profiler.lock().unwrap();
            profiler.get_current_profile().ok()
        };
        
        let alerts = {
            let alert_manager = self.alert_manager.lock().unwrap();
            alert_manager.get_active_alerts()
        };
        
        Ok(PerformanceSnapshot {
            timestamp,
            system_metrics,
            component_metrics,
            profiler_data,
            alerts,
        })
    }
    
    /// Get performance history for a specific duration
    pub fn get_performance_history(&self, duration: Duration) -> Result<Vec<PerformanceSnapshot>> {
        let history = self.metrics_history.read().unwrap();
        let cutoff_time = current_timestamp() - duration.as_secs();
        
        Ok(history
            .iter()
            .filter(|snapshot| snapshot.timestamp >= cutoff_time)
            .cloned()
            .collect())
    }
    
    /// Identify performance bottlenecks
    pub fn identify_bottlenecks(&self) -> Result<Vec<PerformanceBottleneck>> {
        let snapshot = self.get_current_snapshot()?;
        let optimizer = self.optimizer.lock().unwrap();
        optimizer.identify_bottlenecks(&snapshot)
    }
    
    /// Get optimization recommendations
    pub fn get_optimization_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        let snapshot = self.get_current_snapshot()?;
        let optimizer = self.optimizer.lock().unwrap();
        optimizer.generate_recommendations(&snapshot)
    }
    
    /// Export performance report
    pub fn export_performance_report(&self, format: ReportFormat) -> Result<String> {
        let snapshot = self.get_current_snapshot()?;
        let bottlenecks = self.identify_bottlenecks()?;
        let recommendations = self.get_optimization_recommendations()?;
        
        let report = PerformanceReport {
            timestamp: current_timestamp(),
            snapshot,
            bottlenecks,
            recommendations,
        };
        
        match format {
            ReportFormat::Json => Ok(serde_json::to_string_pretty(&report)
                .map_err(|e| BrainError::Serialization { 
                    source: Box::new(e)
                })?),
            ReportFormat::Csv => self.export_csv_report(&report),
            ReportFormat::Html => self.export_html_report(&report),
        }
    }
    
    /// Start system metrics collection loop
    async fn start_system_metrics_collection(&self) -> Result<()> {
        let system_metrics = self.system_metrics.clone();
        let alert_manager = self.alert_manager.clone();
        let metrics_history = self.metrics_history.clone();
        let is_running = self.is_running.clone();
        let interval_ms = self.config.collection_interval_ms;
        let max_history = self.config.max_history_entries;
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(interval_ms));
            
            loop {
                interval.tick().await;
                
                // Check if we should continue running
                {
                    let running = is_running.lock().unwrap();
                    if !*running {
                        break;
                    }
                }
                
                // Collect metrics
                if let Ok(mut collector) = system_metrics.write() {
                    if let Ok(metrics) = collector.collect_metrics() {
                        // Check for alerts
                        if let Ok(mut alert_mgr) = alert_manager.lock() {
                            let _ = alert_mgr.check_system_metrics(&metrics);
                        }
                        
                        // Store in history
                        if let Ok(mut history) = metrics_history.write() {
                            let snapshot = PerformanceSnapshot {
                                timestamp: current_timestamp(),
                                system_metrics: metrics,
                                component_metrics: HashMap::new(),
                                profiler_data: None,
                                alerts: vec![],
                            };
                            
                            history.push(snapshot);
                            
                            // Trim history if too large
                            if history.len() > max_history {
                                let excess = history.len() - max_history;
                                history.drain(0..excess);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Start profiling
    async fn start_profiling(&self) -> Result<()> {
        let mut profiler = self.profiler.lock().unwrap();
        profiler.start_profiling()
    }
    
    /// Start Prometheus exporter
    async fn start_prometheus_exporter(&self) -> Result<()> {
        // Placeholder for Prometheus exporter implementation
        Ok(())
    }
    
    fn export_csv_report(&self, _report: &PerformanceReport) -> Result<String> {
        // Placeholder for CSV export
        Ok("CSV export not implemented".to_string())
    }
    
    fn export_html_report(&self, _report: &PerformanceReport) -> Result<String> {
        // Placeholder for HTML export
        Ok("<html><body><h1>Performance Report</h1><p>HTML export not implemented</p></body></html>".to_string())
    }
} 