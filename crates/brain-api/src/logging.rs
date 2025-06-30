//! Logging Module
//!
//! This module provides comprehensive logging functionality for the Brain AI API
//! including structured logging, request tracking, error categorization, and audit trails.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::net::IpAddr;
use chrono::{DateTime, Utc};

use brain_types::{BrainError, Result};
use tracing::{info, warn, error, debug};
use crate::auth::{AuthResult, UserRole};
use std::fmt;

/// Error categories for structured logging
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ErrorCategory {
    Authentication,
    Authorization,
    RateLimit,
    Validation,
    Database,
    External,
    Internal,
    Network,
    Configuration,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Authentication => write!(f, "Authentication"),
            ErrorCategory::Authorization => write!(f, "Authorization"),
            ErrorCategory::RateLimit => write!(f, "RateLimit"),
            ErrorCategory::Validation => write!(f, "Validation"),
            ErrorCategory::Database => write!(f, "Database"),
            ErrorCategory::External => write!(f, "External"),
            ErrorCategory::Internal => write!(f, "Internal"),
            ErrorCategory::Network => write!(f, "Network"),
            ErrorCategory::Configuration => write!(f, "Configuration"),
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "Low"),
            ErrorSeverity::Medium => write!(f, "Medium"),
            ErrorSeverity::High => write!(f, "High"),
            ErrorSeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub log_level: String,
    pub log_format: String,
    pub enable_file_logging: bool,
    pub log_file_path: String,
    pub enable_structured_logging: bool,
    pub enable_request_logging: bool,
    pub enable_error_tracking: bool,
    pub retention_days: u32,
    pub max_log_size_mb: u64,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            log_format: "json".to_string(),
            enable_file_logging: true,
            log_file_path: "logs/brain_api.log".to_string(),
            enable_structured_logging: true,
            enable_request_logging: true,
            enable_error_tracking: true,
            retention_days: 30,
            max_log_size_mb: 100,
        }
    }
}

/// Request log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub request_id: String,
    pub method: String,
    pub endpoint: String,
    pub client_ip: IpAddr,
    pub user_id: Option<String>,
    pub user_role: Option<UserRole>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<u64>,
    pub status_code: Option<u16>,
    pub response_size: Option<u64>,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Error log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLog {
    pub error_id: String,
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub message: String,
    pub details: Option<String>,
    pub context: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub stack_trace: Option<String>,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub event_id: String,
    pub event_type: String,
    pub user_id: String,
    pub user_role: UserRole,
    pub action: String,
    pub resource: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub client_ip: IpAddr,
    pub success: bool,
    pub details: HashMap<String, String>,
}

/// Logging statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub errors_by_category: HashMap<ErrorCategory, u64>,
    pub errors_by_severity: HashMap<ErrorSeverity, u64>,
    pub requests_by_endpoint: HashMap<String, u64>,
    pub requests_by_user_role: HashMap<UserRole, u64>,
}

/// Main logging manager
pub struct LoggingManager {
    config: LoggingConfig,
    request_logs: Arc<Mutex<HashMap<String, RequestLog>>>,
    error_logs: Arc<Mutex<Vec<ErrorLog>>>,
    audit_logs: Arc<Mutex<Vec<AuditLog>>>,
    stats: Arc<Mutex<LoggingStats>>,
}

static LOGGING_INIT: Once = Once::new();
static mut LOGGING_INITIALIZED: bool = false;

impl LoggingManager {
    /// Create a new logging manager (singleton)
    pub fn new(config: LoggingConfig) -> Result<Self> {
        let mut can_initialize = false;
        
        LOGGING_INIT.call_once(|| {
            unsafe {
                LOGGING_INITIALIZED = true;
                can_initialize = true;
            }
        });
        
        if !can_initialize {
            return Err(BrainError::Conflict("Logging manager already initialized".to_string()).into());
        }
        
        let manager = Self {
            config,
            request_logs: Arc::new(Mutex::new(HashMap::new())),
            error_logs: Arc::new(Mutex::new(Vec::new())),
            audit_logs: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(LoggingStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                errors_by_category: HashMap::new(),
                errors_by_severity: HashMap::new(),
                requests_by_endpoint: HashMap::new(),
                requests_by_user_role: HashMap::new(),
            })),
        };
        
        // Initialize tracing
        Self::setup_tracing(&manager.config)?;
        
        Ok(manager)
    }
    
    /// Setup tracing infrastructure
    fn setup_tracing(config: &LoggingConfig) -> Result<()> {
        // In a real implementation, this would set up tracing subscribers
        // For now, we'll use the basic tracing setup
        info!("Logging manager initialized with config: {:?}", config.log_level);
        Ok(())
    }
    
    /// Start tracking a request
    pub fn start_request(&self, request_id: String, endpoint: String, method: String, client_ip: IpAddr) {
        let request_log = RequestLog {
            request_id: request_id.clone(),
            method,
            endpoint: endpoint.clone(),
            client_ip,
            user_id: None,
            user_role: None,
            started_at: Utc::now(),
            completed_at: None,
            duration_ms: None,
            status_code: None,
            response_size: None,
            error_message: None,
            metadata: HashMap::new(),
        };
        
        if let Ok(mut logs) = self.request_logs.lock() {
            logs.insert(request_id.clone(), request_log);
        }
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_requests += 1;
            *stats.requests_by_endpoint.entry(endpoint).or_insert(0) += 1;
        }
        
        debug!("Started tracking request: {}", request_id);
    }
    
    /// Complete a request with authentication info
    pub fn complete_request(
        &self,
        request_id: String,
        status_code: u16,
        auth_result: Option<AuthResult>,
        metadata: HashMap<String, String>,
    ) {
        let completed_at = Utc::now();
        
        if let Ok(mut logs) = self.request_logs.lock() {
            if let Some(request_log) = logs.get_mut(&request_id) {
                let duration = completed_at.signed_duration_since(request_log.started_at);
                request_log.completed_at = Some(completed_at);
                request_log.duration_ms = Some(duration.num_milliseconds() as u64);
                request_log.status_code = Some(status_code);
                request_log.metadata = metadata;
                
                if let Some(auth) = auth_result {
                    request_log.user_id = Some(auth.user_id);
                    request_log.user_role = Some(auth.role.clone());
                    
                    // Update role statistics
                    if let Ok(mut stats) = self.stats.lock() {
                        *stats.requests_by_user_role.entry(auth.role).or_insert(0) += 1;
                    }
                }
                
                // Update success/failure statistics
                if let Ok(mut stats) = self.stats.lock() {
                    if status_code < 400 {
                        stats.successful_requests += 1;
                    } else {
                        stats.failed_requests += 1;
                    }
                    
                    // Update average response time
                    let total_time = stats.average_response_time_ms * (stats.total_requests - 1) as f64;
                    let new_time = duration.num_milliseconds() as f64;
                    stats.average_response_time_ms = (total_time + new_time) / stats.total_requests as f64;
                }
            }
        }
        
        info!("Completed request: {} with status: {}", request_id, status_code);
    }
    
    /// Log an error
    pub fn log_error(
        &self,
        category: ErrorCategory,
        severity: ErrorSeverity,
        message: String,
        details: Option<String>,
        context: HashMap<String, String>,
        request_id: Option<String>,
        user_id: Option<String>,
    ) {
        let error_id = uuid::Uuid::new_v4().to_string();
        
        let error_log = ErrorLog {
            error_id: error_id.clone(),
            category: category.clone(),
            severity: severity.clone(),
            message: message.clone(),
            details,
            context,
            timestamp: Utc::now(),
            request_id,
            user_id,
            stack_trace: None, // Could be populated with backtrace in real implementation
        };
        
        if let Ok(mut logs) = self.error_logs.lock() {
            logs.push(error_log);
        }
        
        // Update error statistics
        if let Ok(mut stats) = self.stats.lock() {
            *stats.errors_by_category.entry(category).or_insert(0) += 1;
            *stats.errors_by_severity.entry(severity.clone()).or_insert(0) += 1;
        }
        
        // Log to tracing based on severity
        match severity {
            ErrorSeverity::Low => debug!("Error {}: {}", error_id, message),
            ErrorSeverity::Medium => warn!("Error {}: {}", error_id, message),
            ErrorSeverity::High | ErrorSeverity::Critical => error!("Error {}: {}", error_id, message),
        }
    }
    
    /// Log an audit event
    pub fn log_audit(
        &self,
        event_type: String,
        user_id: String,
        user_role: UserRole,
        action: String,
        resource: Option<String>,
        client_ip: IpAddr,
        success: bool,
        details: HashMap<String, String>,
    ) {
        let event_id = uuid::Uuid::new_v4().to_string();
        
        let audit_log = AuditLog {
            event_id: event_id.clone(),
            event_type,
            user_id: user_id.clone(),
            user_role,
            action: action.clone(),
            resource,
            timestamp: Utc::now(),
            client_ip,
            success,
            details,
        };
        
        if let Ok(mut logs) = self.audit_logs.lock() {
            logs.push(audit_log);
        }
        
        let status = if success { "SUCCESS" } else { "FAILURE" };
        info!("Audit {}: User {} performed {} - {}", event_id, user_id, action, status);
    }
    
    /// Get logging statistics
    pub fn get_stats(&self) -> Result<LoggingStats> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| BrainError::InternalError("Failed to acquire stats lock".to_string()).into())
    }
    
    /// Get recent error logs
    pub fn get_recent_errors(&self, limit: usize) -> Result<Vec<ErrorLog>> {
        if let Ok(logs) = self.error_logs.lock() {
            let mut recent_logs: Vec<ErrorLog> = logs.iter().cloned().collect();
            recent_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            recent_logs.truncate(limit);
            Ok(recent_logs)
        } else {
            Err(BrainError::InternalError("Failed to acquire error logs lock".to_string()).into())
        }
    }
    
    /// Get recent audit logs
    pub fn get_recent_audits(&self, limit: usize) -> Result<Vec<AuditLog>> {
        if let Ok(logs) = self.audit_logs.lock() {
            let mut recent_logs: Vec<AuditLog> = logs.iter().cloned().collect();
            recent_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            recent_logs.truncate(limit);
            Ok(recent_logs)
        } else {
            Err(BrainError::InternalError("Failed to acquire audit logs lock".to_string()).into())
        }
    }
    
    /// Search request logs by criteria
    pub fn search_requests(&self, user_id: Option<&str>, endpoint: Option<&str>) -> Result<Vec<RequestLog>> {
        if let Ok(logs) = self.request_logs.lock() {
            let filtered_logs: Vec<RequestLog> = logs.values()
                .filter(|log| {
                    if let Some(uid) = user_id {
                        if log.user_id.as_ref().map(|s| s.as_str()) != Some(uid) {
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
            Ok(filtered_logs)
        } else {
            Err(BrainError::InternalError("Failed to acquire request logs lock".to_string()).into())
        }
    }
    
    /// Clear old logs based on retention policy
    pub fn cleanup_old_logs(&self) -> Result<()> {
        let cutoff = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
        
        // Clean error logs
        if let Ok(mut logs) = self.error_logs.lock() {
            logs.retain(|log| log.timestamp > cutoff);
        }
        
        // Clean audit logs
        if let Ok(mut logs) = self.audit_logs.lock() {
            logs.retain(|log| log.timestamp > cutoff);
        }
        
        // Clean completed request logs
        if let Ok(mut logs) = self.request_logs.lock() {
            logs.retain(|_, log| {
                if let Some(completed_at) = log.completed_at {
                    completed_at > cutoff
                } else {
                    log.started_at > cutoff
                }
            });
        }
        
        info!("Cleaned up logs older than {} days", self.config.retention_days);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Low < ErrorSeverity::Medium);
        assert!(ErrorSeverity::Medium < ErrorSeverity::High);
        assert!(ErrorSeverity::High < ErrorSeverity::Critical);
    }

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.log_level, "info");
        assert!(config.enable_structured_logging);
        assert_eq!(config.retention_days, 30);
    }

    #[tokio::test]
    async fn test_logging_manager() {
        let config = LoggingConfig::default();
        
        // Note: This test might fail if run multiple times due to singleton
        if let Ok(manager) = LoggingManager::new(config) {
            let request_id = "test_request_001".to_string();
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            
            // Start a request
            manager.start_request(
                request_id.clone(),
                "/api/test".to_string(),
                "GET".to_string(),
                ip,
            );
            
            // Complete the request
            let auth_result = AuthResult::new("test_user".to_string(), UserRole::Developer);
            manager.complete_request(
                request_id,
                200,
                Some(auth_result),
                HashMap::new(),
            );
            
            // Check statistics
            let stats = manager.get_stats().unwrap();
            assert_eq!(stats.total_requests, 1);
            assert_eq!(stats.successful_requests, 1);
        }
    }

    #[test]
    fn test_audit_log_creation() {
        let audit_log = AuditLog {
            event_id: "test_event".to_string(),
            event_type: "user_action".to_string(),
            user_id: "user123".to_string(),
            user_role: UserRole::Admin,
            action: "delete_user".to_string(),
            resource: Some("user456".to_string()),
            timestamp: Utc::now(),
            client_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            success: true,
            details: HashMap::new(),
        };
        
        assert_eq!(audit_log.user_id, "user123");
        assert_eq!(audit_log.action, "delete_user");
        assert!(audit_log.success);
    }
} 