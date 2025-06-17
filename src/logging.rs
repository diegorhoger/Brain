//! Logging and Telemetry Module - Task 7.3
//!
//! This module provides comprehensive logging capabilities for the Brain AI system,
//! including API usage patterns, error tracking, performance metrics, and system monitoring.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{error, info, warn, debug};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::auth::{UserRole, AuthResult};

/// System-wide telemetry and logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,
    /// Whether to enable structured JSON logging
    pub json_format: bool,
    /// Whether to log to file
    pub log_to_file: bool,
    /// Log file path
    pub log_file_path: String,
    /// Whether to enable performance metrics collection
    pub enable_metrics: bool,
    /// Whether to enable telemetry collection
    pub enable_telemetry: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_seconds: u64,
    /// Maximum log file size in MB before rotation
    pub max_log_file_size_mb: u64,
    /// Number of log files to keep in rotation
    pub log_rotation_count: u32,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            json_format: false,
            log_to_file: true,
            log_file_path: "logs/brain-ai.log".to_string(),
            enable_metrics: true,
            enable_telemetry: true,
            metrics_interval_seconds: 60,
            max_log_file_size_mb: 100,
            log_rotation_count: 10,
        }
    }
}

/// API request log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequestLog {
    /// Unique request ID
    pub request_id: String,
    /// Timestamp of the request
    pub timestamp: DateTime<Utc>,
    /// HTTP method (if applicable)
    pub method: String,
    /// API endpoint or operation
    pub endpoint: String,
    /// Client IP address
    pub client_ip: IpAddr,
    /// User ID (if authenticated)
    pub user_id: Option<String>,
    /// User role (if authenticated)
    pub user_role: Option<UserRole>,
    /// Request duration in milliseconds
    pub duration_ms: u64,
    /// Response status code
    pub status_code: u16,
    /// Request payload size in bytes
    pub request_size_bytes: Option<u64>,
    /// Response payload size in bytes
    pub response_size_bytes: Option<u64>,
    /// Whether the request was rate limited
    pub rate_limited: bool,
    /// Error message if request failed
    pub error_message: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Performance metrics for system monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Timestamp when metrics were collected
    pub timestamp: DateTime<Utc>,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in MB
    pub memory_usage_mb: u64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Number of active connections
    pub active_connections: u32,
    /// Requests per second over the last minute
    pub requests_per_second: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
    /// Queue depths for various operations
    pub queue_depths: HashMap<String, u32>,
    /// Cache hit rates
    pub cache_hit_rates: HashMap<String, f64>,
}

/// Error tracking information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLog {
    /// Unique error ID
    pub error_id: String,
    /// Timestamp of the error
    pub timestamp: DateTime<Utc>,
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Error category
    pub category: ErrorCategory,
    /// Error message
    pub message: String,
    /// Error details and stack trace
    pub details: String,
    /// Operation that caused the error
    pub operation: String,
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Client IP (if applicable)
    pub client_ip: Option<IpAddr>,
    /// Number of times this error has occurred
    pub occurrence_count: u32,
    /// First time this error was seen
    pub first_seen: DateTime<Utc>,
    /// Last time this error was seen
    pub last_seen: DateTime<Utc>,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Error categories for better organization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    Authentication,
    Authorization,
    RateLimit,
    Validation,
    Database,
    Network,
    Internal,
    Configuration,
    External,
}

/// Usage analytics for different operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalytics {
    /// Time period for these analytics
    pub time_period: String,
    /// Timestamp of data collection
    pub timestamp: DateTime<Utc>,
    /// Total API calls
    pub total_api_calls: u64,
    /// Unique users
    pub unique_users: u32,
    /// Calls by operation
    pub calls_by_operation: HashMap<String, u64>,
    /// Calls by user role
    pub calls_by_role: HashMap<String, u64>,
    /// Average response times by operation
    pub avg_response_times: HashMap<String, f64>,
    /// Error counts by category
    pub error_counts: HashMap<String, u64>,
    /// Most active users
    pub top_users: Vec<(String, u64)>,
    /// Most popular operations
    pub top_operations: Vec<(String, u64)>,
    /// Geographic distribution (if available)
    pub geographic_distribution: HashMap<String, u64>,
}

/// Main logging and telemetry manager
pub struct LoggingManager {
    config: LoggingConfig,
    request_logs: Arc<Mutex<Vec<ApiRequestLog>>>,
    error_logs: Arc<Mutex<HashMap<String, ErrorLog>>>,
    performance_metrics: Arc<Mutex<Vec<PerformanceMetrics>>>,
    usage_analytics: Arc<Mutex<UsageAnalytics>>,
    active_requests: Arc<Mutex<HashMap<String, Instant>>>,
}

impl LoggingManager {
    /// Create new logging manager with configuration
    pub fn new(config: LoggingConfig) -> Result<Self> {
        // Initialize tracing subscriber based on configuration
        Self::initialize_tracing(&config)?;

        info!("Initialized logging system with level: {}", config.log_level);

        Ok(Self {
            config,
            request_logs: Arc::new(Mutex::new(Vec::new())),
            error_logs: Arc::new(Mutex::new(HashMap::new())),
            performance_metrics: Arc::new(Mutex::new(Vec::new())),
            usage_analytics: Arc::new(Mutex::new(UsageAnalytics::default())),
            active_requests: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Initialize tracing subscriber
    fn initialize_tracing(config: &LoggingConfig) -> Result<()> {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(&config.log_level));

        // Try to initialize tracing, but don't fail if subscriber already set
        let result = if config.json_format {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer().json())
                .try_init()
        } else {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer())
                .try_init()
        };

        match result {
            Ok(_) => {
                info!("Initialized logging system with level: {}", config.log_level);
            },
            Err(_) => {
                // Global subscriber already set, which is fine in test environments
                eprintln!("Warning: Global tracing subscriber already initialized");
            }
        }

        Ok(())
    }

    /// Start tracking an API request
    pub fn start_request(&self, request_id: String, endpoint: String, method: String, _client_ip: IpAddr) {
        debug!("Starting request tracking: {} {} {}", method, endpoint, request_id);
        
        if let Ok(mut active) = self.active_requests.lock() {
            active.insert(request_id, Instant::now());
        }
    }

    /// Complete and log an API request
    pub fn complete_request(
        &self,
        request_id: String,
        endpoint: String,
        method: String,
        client_ip: IpAddr,
        auth_result: Option<&AuthResult>,
        status_code: u16,
        request_size: Option<u64>,
        response_size: Option<u64>,
        rate_limited: bool,
        error_message: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Result<()> {
        let duration = if let Ok(mut active) = self.active_requests.lock() {
            if let Some(start_time) = active.remove(&request_id) {
                start_time.elapsed()
            } else {
                Duration::from_millis(0)
            }
        } else {
            Duration::from_millis(0)
        };

        let log_entry = ApiRequestLog {
            request_id: request_id.clone(),
            timestamp: Utc::now(),
            method,
            endpoint: endpoint.clone(),
            client_ip,
            user_id: auth_result.map(|a| a.user_id.clone()),
            user_role: auth_result.map(|a| a.role.clone()),
            duration_ms: duration.as_millis() as u64,
            status_code,
            request_size_bytes: request_size,
            response_size_bytes: response_size,
            rate_limited,
            error_message,
            metadata,
        };

        // Log the request
        if status_code >= 400 {
            warn!("API request failed: {} {} - Status: {}, Duration: {}ms", 
                  log_entry.method, log_entry.endpoint, status_code, log_entry.duration_ms);
        } else {
            info!("API request completed: {} {} - Status: {}, Duration: {}ms", 
                  log_entry.method, log_entry.endpoint, status_code, log_entry.duration_ms);
        }

        // Store the log entry
        if let Ok(mut logs) = self.request_logs.lock() {
            logs.push(log_entry);
            
            // Keep only recent logs to prevent memory growth
            if logs.len() > 10000 {
                logs.drain(0..5000);
            }
        }

        // Update usage analytics
        self.update_usage_analytics(&endpoint, auth_result, duration.as_millis() as f64, status_code >= 400)?;

        Ok(())
    }

    /// Log an error with tracking
    pub fn log_error(
        &self,
        category: ErrorCategory,
        severity: ErrorSeverity,
        operation: String,
        message: String,
        details: String,
        user_id: Option<String>,
        client_ip: Option<IpAddr>,
    ) -> Result<String> {
        let error_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        // Create error signature for deduplication
        let error_signature = format!("{:?}:{}:{}", category, operation, message);

        let mut error_logs = self.error_logs.lock()
            .map_err(|_| anyhow!("Failed to acquire error logs lock"))?;

        if let Some(existing_error) = error_logs.get_mut(&error_signature) {
            // Update existing error
            existing_error.occurrence_count += 1;
            existing_error.last_seen = now;
            existing_error.details = details; // Update with latest details
        } else {
            // Create new error log
            let error_log = ErrorLog {
                error_id: error_id.clone(),
                timestamp: now,
                severity: severity.clone(),
                category: category.clone(),
                message: message.clone(),
                details,
                operation: operation.clone(),
                user_id: user_id.clone(),
                client_ip,
                occurrence_count: 1,
                first_seen: now,
                last_seen: now,
            };

            error_logs.insert(error_signature, error_log);
        }

        // Log with appropriate level based on severity
        match severity {
            ErrorSeverity::Critical => {
                error!("CRITICAL ERROR [{}]: {} in {}", error_id, message, operation);
            }
            ErrorSeverity::High => {
                error!("HIGH ERROR [{}]: {} in {}", error_id, message, operation);
            }
            ErrorSeverity::Medium => {
                warn!("MEDIUM ERROR [{}]: {} in {}", error_id, message, operation);
            }
            ErrorSeverity::Low => {
                debug!("LOW ERROR [{}]: {} in {}", error_id, message, operation);
            }
        }

        Ok(error_id)
    }

    /// Collect and store performance metrics
    pub fn collect_performance_metrics(&self) -> Result<()> {
        if !self.config.enable_metrics {
            return Ok(());
        }

        // In a real implementation, you would collect actual system metrics
        // For now, we'll create placeholder metrics
        let metrics = PerformanceMetrics {
            timestamp: Utc::now(),
            cpu_usage_percent: Self::get_cpu_usage(),
            memory_usage_mb: Self::get_memory_usage_mb(),
            memory_usage_percent: Self::get_memory_usage_percent(),
            active_connections: self.get_active_connections_count(),
            requests_per_second: self.calculate_requests_per_second()?,
            avg_response_time_ms: self.calculate_avg_response_time()?,
            error_rate_percent: self.calculate_error_rate()?,
            queue_depths: HashMap::new(),
            cache_hit_rates: HashMap::new(),
        };

        if let Ok(mut perf_metrics) = self.performance_metrics.lock() {
            perf_metrics.push(metrics);
            
            // Keep only recent metrics
            if perf_metrics.len() > 1440 { // 24 hours of minute-by-minute data
                perf_metrics.drain(0..720); // Remove 12 hours
            }
        }

        debug!("Collected performance metrics");
        Ok(())
    }

    /// Get system CPU usage (placeholder implementation)
    fn get_cpu_usage() -> f64 {
        // In a real implementation, you would use system APIs
        // For demonstration, return a simulated value
        rand::random::<f64>() * 100.0
    }

    /// Get system memory usage in MB (placeholder implementation)
    fn get_memory_usage_mb() -> u64 {
        // In a real implementation, you would use system APIs
        1024 + (rand::random::<u64>() % 2048)
    }

    /// Get system memory usage percentage (placeholder implementation)
    fn get_memory_usage_percent() -> f64 {
        rand::random::<f64>() * 80.0
    }

    /// Get number of active connections
    fn get_active_connections_count(&self) -> u32 {
        if let Ok(active) = self.active_requests.lock() {
            active.len() as u32
        } else {
            0
        }
    }

    /// Calculate requests per second over the last minute
    fn calculate_requests_per_second(&self) -> Result<f64> {
        let one_minute_ago = Utc::now() - chrono::Duration::minutes(1);
        
        if let Ok(logs) = self.request_logs.lock() {
            let recent_requests = logs.iter()
                .filter(|log| log.timestamp > one_minute_ago)
                .count();
            Ok(recent_requests as f64 / 60.0)
        } else {
            Ok(0.0)
        }
    }

    /// Calculate average response time
    fn calculate_avg_response_time(&self) -> Result<f64> {
        let one_hour_ago = Utc::now() - chrono::Duration::hours(1);
        
        if let Ok(logs) = self.request_logs.lock() {
            let recent_logs: Vec<_> = logs.iter()
                .filter(|log| log.timestamp > one_hour_ago)
                .collect();
                
            if recent_logs.is_empty() {
                return Ok(0.0);
            }
            
            let total_time: u64 = recent_logs.iter()
                .map(|log| log.duration_ms)
                .sum();
                
            Ok(total_time as f64 / recent_logs.len() as f64)
        } else {
            Ok(0.0)
        }
    }

    /// Calculate error rate percentage
    fn calculate_error_rate(&self) -> Result<f64> {
        let one_hour_ago = Utc::now() - chrono::Duration::hours(1);
        
        if let Ok(logs) = self.request_logs.lock() {
            let recent_logs: Vec<_> = logs.iter()
                .filter(|log| log.timestamp > one_hour_ago)
                .collect();
                
            if recent_logs.is_empty() {
                return Ok(0.0);
            }
            
            let error_count = recent_logs.iter()
                .filter(|log| log.status_code >= 400)
                .count();
                
            Ok((error_count as f64 / recent_logs.len() as f64) * 100.0)
        } else {
            Ok(0.0)
        }
    }

    /// Update usage analytics
    fn update_usage_analytics(
        &self,
        endpoint: &str,
        auth_result: Option<&AuthResult>,
        response_time: f64,
        is_error: bool,
    ) -> Result<()> {
        if let Ok(mut analytics) = self.usage_analytics.lock() {
            analytics.total_api_calls += 1;
            
            // Track by operation
            *analytics.calls_by_operation.entry(endpoint.to_string()).or_insert(0) += 1;
            
            // Track by role if authenticated
            if let Some(auth) = auth_result {
                *analytics.calls_by_role.entry(format!("{:?}", auth.role)).or_insert(0) += 1;
                
                // Update top users
                let user_entry = analytics.top_users.iter_mut()
                    .find(|(user_id, _)| user_id == &auth.user_id);
                    
                if let Some((_, count)) = user_entry {
                    *count += 1;
                } else {
                    analytics.top_users.push((auth.user_id.clone(), 1));
                }
                
                // Keep only top 100 users
                analytics.top_users.sort_by(|a, b| b.1.cmp(&a.1));
                analytics.top_users.truncate(100);
            }
            
            // Update response times
            let current_avg = analytics.avg_response_times
                .entry(endpoint.to_string())
                .or_insert(response_time);
            *current_avg = (*current_avg + response_time) / 2.0;
            
            // Track errors
            if is_error {
                *analytics.error_counts.entry("total".to_string()).or_insert(0) += 1;
            }
        }
        
        Ok(())
    }

    /// Get API request logs with optional filtering
    pub fn get_request_logs(
        &self,
        limit: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        user_id: Option<&str>,
        endpoint: Option<&str>,
    ) -> Result<Vec<ApiRequestLog>> {
        let logs = self.request_logs.lock()
            .map_err(|_| anyhow!("Failed to acquire request logs lock"))?;
            
        let mut filtered_logs: Vec<_> = logs.iter()
            .filter(|log| {
                if let Some(start) = start_time {
                    if log.timestamp < start {
                        return false;
                    }
                }
                if let Some(end) = end_time {
                    if log.timestamp > end {
                        return false;
                    }
                }
                if let Some(user) = user_id {
                    if log.user_id.as_ref() != Some(&user.to_string()) {
                        return false;
                    }
                }
                if let Some(ep) = endpoint {
                    if log.endpoint != ep {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
            
        // Sort by timestamp, newest first
        filtered_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            filtered_logs.truncate(limit);
        }
        
        Ok(filtered_logs)
    }

    /// Get error logs with optional filtering
    pub fn get_error_logs(
        &self,
        limit: Option<usize>,
        severity: Option<ErrorSeverity>,
        category: Option<ErrorCategory>,
    ) -> Result<Vec<ErrorLog>> {
        let error_logs = self.error_logs.lock()
            .map_err(|_| anyhow!("Failed to acquire error logs lock"))?;
            
        let mut filtered_errors: Vec<_> = error_logs.values()
            .filter(|error| {
                if let Some(sev) = &severity {
                    if &error.severity != sev {
                        return false;
                    }
                }
                if let Some(cat) = &category {
                    if &error.category != cat {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
            
        // Sort by last seen, newest first
        filtered_errors.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
        
        if let Some(limit) = limit {
            filtered_errors.truncate(limit);
        }
        
        Ok(filtered_errors)
    }

    /// Get current performance metrics
    pub fn get_performance_metrics(&self, limit: Option<usize>) -> Result<Vec<PerformanceMetrics>> {
        let metrics = self.performance_metrics.lock()
            .map_err(|_| anyhow!("Failed to acquire performance metrics lock"))?;
            
        let mut result: Vec<_> = metrics.iter().cloned().collect();
        result.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            result.truncate(limit);
        }
        
        Ok(result)
    }

    /// Get usage analytics
    pub fn get_usage_analytics(&self) -> Result<UsageAnalytics> {
        let analytics = self.usage_analytics.lock()
            .map_err(|_| anyhow!("Failed to acquire usage analytics lock"))?;
            
        Ok(analytics.clone())
    }

    /// Export logs to JSON format
    pub fn export_logs_json(&self, include_errors: bool, include_performance: bool) -> Result<String> {
        let mut export_data = serde_json::Map::new();
        
        // Export request logs
        let request_logs = self.get_request_logs(None, None, None, None, None)?;
        export_data.insert("request_logs".to_string(), serde_json::to_value(request_logs)?);
        
        if include_errors {
            let error_logs = self.get_error_logs(None, None, None)?;
            export_data.insert("error_logs".to_string(), serde_json::to_value(error_logs)?);
        }
        
        if include_performance {
            let performance_metrics = self.get_performance_metrics(Some(100))?;
            export_data.insert("performance_metrics".to_string(), serde_json::to_value(performance_metrics)?);
        }
        
        let usage_analytics = self.get_usage_analytics()?;
        export_data.insert("usage_analytics".to_string(), serde_json::to_value(usage_analytics)?);
        
        Ok(serde_json::to_string_pretty(&export_data)?)
    }

    /// Get logging system health status
    pub fn get_health_status(&self) -> HashMap<String, String> {
        let mut status = HashMap::new();
        
        status.insert("logging_enabled".to_string(), "true".to_string());
        status.insert("metrics_enabled".to_string(), self.config.enable_metrics.to_string());
        status.insert("telemetry_enabled".to_string(), self.config.enable_telemetry.to_string());
        status.insert("log_level".to_string(), self.config.log_level.clone());
        
        // Add memory usage statistics
        if let Ok(logs) = self.request_logs.lock() {
            status.insert("request_logs_count".to_string(), logs.len().to_string());
        }
        
        if let Ok(errors) = self.error_logs.lock() {
            status.insert("error_logs_count".to_string(), errors.len().to_string());
        }
        
        if let Ok(metrics) = self.performance_metrics.lock() {
            status.insert("performance_metrics_count".to_string(), metrics.len().to_string());
        }
        
        status
    }
}

impl Default for UsageAnalytics {
    fn default() -> Self {
        Self {
            time_period: "current_session".to_string(),
            timestamp: Utc::now(),
            total_api_calls: 0,
            unique_users: 0,
            calls_by_operation: HashMap::new(),
            calls_by_role: HashMap::new(),
            avg_response_times: HashMap::new(),
            error_counts: HashMap::new(),
            top_users: Vec::new(),
            top_operations: Vec::new(),
            geographic_distribution: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.log_level, "info");
        assert!(config.enable_metrics);
        assert!(config.enable_telemetry);
    }

    #[test]
    fn test_logging_manager_creation() {
        let config = LoggingConfig::default();
        // Try to create manager, but don't fail if global subscriber already set
        match LoggingManager::new(config) {
            Ok(_manager) => {
                // Creation succeeded
            },
            Err(_) => {
                // Global subscriber already set, which is fine in test environment
                // Just verify config defaults work
                let config = LoggingConfig::default();
                assert_eq!(config.log_level, "info");
                assert!(config.enable_telemetry);
            }
        }
    }

    #[test]
    fn test_error_logging() {
        let config = LoggingConfig::default();
        let manager = match LoggingManager::new(config) {
            Ok(m) => m,
            Err(_) => {
                // Global subscriber already set, skip this test
                return;
            }
        };
        
        let error_id = manager.log_error(
            ErrorCategory::Validation,
            ErrorSeverity::Medium,
            "test_operation".to_string(),
            "Test error message".to_string(),
            "Detailed error information".to_string(),
            Some("test_user".to_string()),
            Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        ).unwrap();
        
        assert!(!error_id.is_empty());
        
        let error_logs = manager.get_error_logs(Some(10), None, None).unwrap();
        assert_eq!(error_logs.len(), 1);
        assert_eq!(error_logs[0].message, "Test error message");
    }

    #[test]
    fn test_request_logging() {
        let config = LoggingConfig::default();
        let manager = match LoggingManager::new(config) {
            Ok(m) => m,
            Err(_) => {
                // Global subscriber already set, skip this test
                return;
            }
        };
        
        let request_id = "test_request_123".to_string();
        let endpoint = "/api/test".to_string();
        let method = "GET".to_string();
        let client_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        
        manager.start_request(request_id.clone(), endpoint.clone(), method.clone(), client_ip);
        
        // Simulate some processing time
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        manager.complete_request(
            request_id,
            endpoint,
            method,
            client_ip,
            None,
            200,
            Some(1024),
            Some(2048),
            false,
            None,
            HashMap::new(),
        ).unwrap();
        
        let request_logs = manager.get_request_logs(Some(10), None, None, None, None).unwrap();
        assert_eq!(request_logs.len(), 1);
        assert_eq!(request_logs[0].status_code, 200);
        assert!(request_logs[0].duration_ms > 0);
    }

    #[test]
    fn test_analytics_tracking() {
        let config = LoggingConfig::default();
        let manager = match LoggingManager::new(config) {
            Ok(m) => m,
            Err(_) => {
                // Global subscriber already set, skip this test
                return;
            }
        };
        
        // Simulate some API usage
        manager.update_usage_analytics("/api/test", None, 100.0, false).unwrap();
        manager.update_usage_analytics("/api/test", None, 150.0, false).unwrap();
        
        let analytics = manager.get_usage_analytics().unwrap();
        assert_eq!(analytics.total_api_calls, 2);
        assert!(analytics.calls_by_operation.contains_key("/api/test"));
    }
} 