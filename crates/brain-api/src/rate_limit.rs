//! Rate Limiting Module
//!
//! This module provides comprehensive rate limiting functionality for the Brain AI API
//! including per-user, per-IP, and per-endpoint rate limiting with configurable policies.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use brain_types::BrainError;
use crate::auth::UserRole;

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Default rate limit per minute for unauthenticated requests
    pub default_rate_limit: u32,
    /// Rate limit window in seconds
    pub window_seconds: u64,
    /// Whether to use sliding window or fixed window
    pub sliding_window: bool,
    /// Rate limits by user role
    pub role_limits: HashMap<UserRole, u32>,
    /// Rate limits by IP (requests per minute)
    pub ip_limit: u32,
    /// Rate limits by endpoint
    pub endpoint_limits: HashMap<String, u32>,
    /// Burst allowance (additional requests allowed in short bursts)
    pub burst_allowance: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        let mut role_limits = HashMap::new();
        role_limits.insert(UserRole::Admin, 1000);
        role_limits.insert(UserRole::Developer, 500);
        role_limits.insert(UserRole::Analyst, 300);
        role_limits.insert(UserRole::Viewer, 100);
        
        let mut endpoint_limits = HashMap::new();
        endpoint_limits.insert("admin_endpoint".to_string(), 1000);
        endpoint_limits.insert("dev_endpoint".to_string(), 500);
        endpoint_limits.insert("guest_endpoint".to_string(), 100);
        
        Self {
            default_rate_limit: 100,
            window_seconds: 60,
            sliding_window: true,
            role_limits,
            ip_limit: 200,
            endpoint_limits,
            burst_allowance: 10,
        }
    }
}

/// Request context for rate limiting
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub user_id: Option<String>,
    pub user_role: Option<UserRole>,
    pub ip_address: IpAddr,
    pub endpoint: String,
    pub timestamp: Instant,
}

/// Create a request context for rate limiting
pub fn create_request_context(
    user_id: Option<String>,
    user_role: Option<UserRole>,
    ip_address: IpAddr,
    endpoint: String,
) -> RequestContext {
    RequestContext {
        user_id,
        user_role,
        ip_address,
        endpoint,
        timestamp: Instant::now(),
    }
}

/// Rate limit check result
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: u32,
    pub reset_time: Instant,
    pub limit: u32,
    pub reason: Option<String>,
}

/// Rate limiting statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStats {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub blocked_requests: u64,
    pub requests_by_user: HashMap<String, u64>,
    pub requests_by_ip: HashMap<String, u64>,
    pub requests_by_endpoint: HashMap<String, u64>,
}

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    capacity: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate: f64) -> Self {
        Self {
            tokens: capacity as f64,
            capacity: capacity as f64,
            refill_rate,
            last_refill: Instant::now(),
        }
    }
    
    fn try_consume(&mut self, tokens: f64) -> bool {
        self.refill();
        
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
    
    fn refill(&mut self) {
        let now = Instant::now();
        let time_passed = now.duration_since(self.last_refill).as_secs_f64();
        
        self.tokens = (self.tokens + time_passed * self.refill_rate).min(self.capacity);
        self.last_refill = now;
    }
    
    fn available_tokens(&mut self) -> u32 {
        self.refill();
        self.tokens as u32
    }
}

/// Main rate limiting manager
pub struct RateLimitManager {
    config: RateLimitConfig,
    user_buckets: Arc<Mutex<HashMap<String, TokenBucket>>>,
    ip_buckets: Arc<Mutex<HashMap<IpAddr, TokenBucket>>>,
    endpoint_buckets: Arc<Mutex<HashMap<String, TokenBucket>>>,
    stats: Arc<Mutex<RateLimitStats>>,
}

impl RateLimitManager {
    /// Create a new rate limit manager
    pub fn new(config: RateLimitConfig) -> Result<Self> {
        Ok(Self {
            config,
            user_buckets: Arc::new(Mutex::new(HashMap::new())),
            ip_buckets: Arc::new(Mutex::new(HashMap::new())),
            endpoint_buckets: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(RateLimitStats {
                total_requests: 0,
                allowed_requests: 0,
                blocked_requests: 0,
                requests_by_user: HashMap::new(),
                requests_by_ip: HashMap::new(),
                requests_by_endpoint: HashMap::new(),
            })),
        })
    }
    
    /// Check if a request should be rate limited
    pub fn check_rate_limit(&self, context: &RequestContext) -> Result<RateLimitResult> {
        // Update statistics
        self.update_stats(context);
        
        // Check user-based rate limiting
        if let Some(user_id) = &context.user_id {
            if let Some(user_role) = &context.user_role {
                let limit = self.config.role_limits.get(user_role)
                    .copied()
                    .unwrap_or(self.config.default_rate_limit);
                
                let result = self.check_user_rate_limit(user_id, limit)?;
                if !result.allowed {
                    self.record_blocked_request();
                    return Ok(result);
                }
            }
        }
        
        // Check IP-based rate limiting
        let ip_result = self.check_ip_rate_limit(context.ip_address)?;
        if !ip_result.allowed {
            self.record_blocked_request();
            return Ok(ip_result);
        }
        
        // Check endpoint-based rate limiting
        let endpoint_limit = self.config.endpoint_limits.get(&context.endpoint)
            .copied()
            .unwrap_or(self.config.default_rate_limit);
        
        let endpoint_result = self.check_endpoint_rate_limit(&context.endpoint, endpoint_limit)?;
        if !endpoint_result.allowed {
            self.record_blocked_request();
            return Ok(endpoint_result);
        }
        
        self.record_allowed_request();
        
        // Return the most restrictive limit
        let remaining = [ip_result.remaining, endpoint_result.remaining]
            .into_iter()
            .min()
            .unwrap_or(0);
            
        Ok(RateLimitResult {
            allowed: true,
            remaining,
            reset_time: Instant::now() + Duration::from_secs(self.config.window_seconds),
            limit: endpoint_limit,
            reason: None,
        })
    }
    
    fn check_user_rate_limit(&self, user_id: &str, limit: u32) -> Result<RateLimitResult> {
        let mut buckets = self.user_buckets.lock()
            .map_err(|_| BrainError::InternalError("Failed to acquire user buckets lock".to_string()))?;
        
        let bucket = buckets.entry(user_id.to_string()).or_insert_with(|| {
            TokenBucket::new(limit + self.config.burst_allowance, limit as f64 / 60.0)
        });
        
        let allowed = bucket.try_consume(1.0);
        let remaining = bucket.available_tokens();
        
        Ok(RateLimitResult {
            allowed,
            remaining,
            reset_time: Instant::now() + Duration::from_secs(self.config.window_seconds),
            limit,
            reason: if !allowed { Some("User rate limit exceeded".to_string()) } else { None },
        })
    }
    
    fn check_ip_rate_limit(&self, ip: IpAddr) -> Result<RateLimitResult> {
        let mut buckets = self.ip_buckets.lock()
            .map_err(|_| BrainError::InternalError("Failed to acquire IP buckets lock".to_string()))?;
        
        let bucket = buckets.entry(ip).or_insert_with(|| {
            TokenBucket::new(self.config.ip_limit + self.config.burst_allowance, self.config.ip_limit as f64 / 60.0)
        });
        
        let allowed = bucket.try_consume(1.0);
        let remaining = bucket.available_tokens();
        
        Ok(RateLimitResult {
            allowed,
            remaining,
            reset_time: Instant::now() + Duration::from_secs(self.config.window_seconds),
            limit: self.config.ip_limit,
            reason: if !allowed { Some("IP rate limit exceeded".to_string()) } else { None },
        })
    }
    
    fn check_endpoint_rate_limit(&self, endpoint: &str, limit: u32) -> Result<RateLimitResult> {
        let mut buckets = self.endpoint_buckets.lock()
            .map_err(|_| BrainError::InternalError("Failed to acquire endpoint buckets lock".to_string()))?;
        
        let bucket = buckets.entry(endpoint.to_string()).or_insert_with(|| {
            TokenBucket::new(limit + self.config.burst_allowance, limit as f64 / 60.0)
        });
        
        let allowed = bucket.try_consume(1.0);
        let remaining = bucket.available_tokens();
        
        Ok(RateLimitResult {
            allowed,
            remaining,
            reset_time: Instant::now() + Duration::from_secs(self.config.window_seconds),
            limit,
            reason: if !allowed { Some("Endpoint rate limit exceeded".to_string()) } else { None },
        })
    }
    
    fn update_stats(&self, context: &RequestContext) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_requests += 1;
            
            if let Some(user_id) = &context.user_id {
                *stats.requests_by_user.entry(user_id.clone()).or_insert(0) += 1;
            }
            
            *stats.requests_by_ip.entry(context.ip_address.to_string()).or_insert(0) += 1;
            *stats.requests_by_endpoint.entry(context.endpoint.clone()).or_insert(0) += 1;
        }
    }
    
    fn record_allowed_request(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.allowed_requests += 1;
        }
    }
    
    fn record_blocked_request(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.blocked_requests += 1;
        }
    }
    
    /// Get rate limiting statistics
    pub fn get_stats(&self) -> Result<RateLimitStats> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| BrainError::InternalError("Failed to acquire stats lock".to_string()).into())
    }
    
    /// Reset rate limits for a specific user
    pub fn reset_user_limits(&self, user_id: &str) -> Result<()> {
        if let Ok(mut buckets) = self.user_buckets.lock() {
            buckets.remove(user_id);
        }
        Ok(())
    }
    
    /// Reset rate limits for a specific IP
    pub fn reset_ip_limits(&self, ip: IpAddr) -> Result<()> {
        if let Ok(mut buckets) = self.ip_buckets.lock() {
            buckets.remove(&ip);
        }
        Ok(())
    }
    
    /// Get current rate limit status for a user
    pub fn get_user_status(&self, user_id: &str) -> Result<Option<u32>> {
        if let Ok(mut buckets) = self.user_buckets.lock() {
            if let Some(bucket) = buckets.get_mut(user_id) {
                return Ok(Some(bucket.available_tokens()));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_token_bucket() {
        let mut bucket = TokenBucket::new(10, 1.0); // 10 tokens, refill 1 per second
        
        // Should be able to consume initial tokens
        assert!(bucket.try_consume(5.0));
        assert_eq!(bucket.available_tokens(), 5);
        
        // Should not be able to consume more than available
        assert!(!bucket.try_consume(10.0));
    }

    #[tokio::test]
    async fn test_rate_limit_manager() {
        let config = RateLimitConfig::default();
        let manager = RateLimitManager::new(config).unwrap();
        
        let context = create_request_context(
            Some("test_user".to_string()),
            Some(UserRole::Developer),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            "test_endpoint".to_string(),
        );
        
        // First request should be allowed
        let result = manager.check_rate_limit(&context).unwrap();
        assert!(result.allowed);
        
        // Statistics should be updated
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.allowed_requests, 1);
    }

    #[test]
    fn test_request_context_creation() {
        let context = create_request_context(
            Some("user123".to_string()),
            Some(UserRole::Admin),
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            "api/test".to_string(),
        );
        
        assert_eq!(context.user_id, Some("user123".to_string()));
        assert_eq!(context.user_role, Some(UserRole::Admin));
        assert_eq!(context.endpoint, "api/test");
    }
} 