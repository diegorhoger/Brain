//! Rate Limiting Module - Task 7.3
//!
//! This module provides rate limiting capabilities for the Brain AI system to prevent
//! API abuse and ensure fair resource usage across different user roles.

use anyhow::{anyhow, Result};
use governor::{
    clock::{Clock, QuantaClock},
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::auth::UserRole;

/// Rate limiting configuration for different user roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute for different user roles
    pub role_limits: HashMap<UserRole, u32>,
    /// Global rate limit (requests per minute) regardless of user
    pub global_limit: u32,
    /// IP-based rate limit for unauthenticated requests
    pub ip_limit: u32,
    /// Burst allowance (number of requests that can be made immediately)
    pub burst_allowance: u32,
    /// Whether to enable rate limiting (can be disabled for development)
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        let mut role_limits = HashMap::new();
        role_limits.insert(UserRole::Admin, 1000);
        role_limits.insert(UserRole::Developer, 500);
        role_limits.insert(UserRole::Analyst, 200);
        role_limits.insert(UserRole::User, 100);
        role_limits.insert(UserRole::Guest, 20);

        Self {
            role_limits,
            global_limit: 10000,
            ip_limit: 50,
            burst_allowance: 10,
            enabled: true,
        }
    }
}

/// Rate limit check result
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    /// Whether the request is allowed
    pub allowed: bool,
    /// Number of requests remaining in the current window
    pub remaining: u32,
    /// Time until the rate limit resets
    pub reset_time: Duration,
    /// The limit that was applied
    pub limit: u32,
    /// Which limiter was hit (if any)
    pub limiter_type: LimiterType,
}

/// Type of rate limiter that was applied
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimiterType {
    /// User-specific rate limit based on role
    User,
    /// IP-based rate limit for unauthenticated requests
    IpAddress,
    /// Global system-wide rate limit
    Global,
    /// No limit applied (rate limiting disabled)
    None,
}

/// Rate limiting statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStats {
    /// Total requests processed
    pub total_requests: u64,
    /// Total requests allowed
    pub allowed_requests: u64,
    /// Total requests blocked
    pub blocked_requests: u64,
    /// Breakdown by limiter type
    pub blocks_by_type: HashMap<String, u64>,
    /// Breakdown by user role
    pub requests_by_role: HashMap<String, u64>,
    /// Top IP addresses by request count
    pub top_ips: Vec<(String, u64)>,
}

impl Default for RateLimitStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            allowed_requests: 0,
            blocked_requests: 0,
            blocks_by_type: HashMap::new(),
            requests_by_role: HashMap::new(),
            top_ips: Vec::new(),
        }
    }
}

/// Request context for rate limiting
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// User ID (if authenticated)
    pub user_id: Option<String>,
    /// User role (if authenticated)
    pub user_role: Option<UserRole>,
    /// Client IP address
    pub ip_address: IpAddr,
    /// Request timestamp
    pub timestamp: std::time::Instant,
    /// Request endpoint/operation
    pub endpoint: String,
}

/// Main rate limiting manager
pub struct RateLimitManager {
    config: RateLimitConfig,
    clock: QuantaClock,
    
    // User-specific rate limiters (by user ID)
    user_limiters: Arc<Mutex<HashMap<String, RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware>>>>,
    
    // IP-based rate limiters
    ip_limiters: Arc<Mutex<HashMap<IpAddr, RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware>>>>,
    
    // Global rate limiter
    global_limiter: RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware>,
    
    // Statistics tracking
    stats: Arc<Mutex<RateLimitStats>>,
    
    // IP request tracking for statistics
    ip_requests: Arc<Mutex<HashMap<IpAddr, u64>>>,
}

impl RateLimitManager {
    /// Create new rate limiting manager with configuration
    pub fn new(config: RateLimitConfig) -> Result<Self> {
        let clock = QuantaClock::default();
        
        // Create global rate limiter
        let global_quota = Quota::per_minute(
            NonZeroU32::new(config.global_limit)
                .ok_or_else(|| anyhow!("Invalid global limit: {}", config.global_limit))?
        ).allow_burst(
            NonZeroU32::new(config.burst_allowance)
                .ok_or_else(|| anyhow!("Invalid burst allowance: {}", config.burst_allowance))?
        );
        
        let global_limiter = RateLimiter::direct_with_clock(global_quota, &clock);

        info!("Initialized rate limiting with global limit: {} req/min, burst: {}", 
              config.global_limit, config.burst_allowance);

        Ok(Self {
            config,
            clock,
            user_limiters: Arc::new(Mutex::new(HashMap::new())),
            ip_limiters: Arc::new(Mutex::new(HashMap::new())),
            global_limiter,
            stats: Arc::new(Mutex::new(RateLimitStats::default())),
            ip_requests: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Check if a request is allowed based on rate limits
    pub fn check_rate_limit(&self, context: &RequestContext) -> Result<RateLimitResult> {
        // Update statistics
        self.update_stats(context);

        // If rate limiting is disabled, allow all requests
        if !self.config.enabled {
            return Ok(RateLimitResult {
                allowed: true,
                remaining: u32::MAX,
                reset_time: Duration::from_secs(0),
                limit: u32::MAX,
                limiter_type: LimiterType::None,
            });
        }

        // Check global rate limit first
        let global_result = self.check_global_limit()?;
        if !global_result.allowed {
            self.record_block(&LimiterType::Global);
            return Ok(global_result);
        }

        // Check user-specific rate limit if authenticated
        if let (Some(user_id), Some(role)) = (&context.user_id, &context.user_role) {
            let user_result = self.check_user_limit(user_id, role)?;
            if !user_result.allowed {
                self.record_block(&LimiterType::User);
                return Ok(user_result);
            }
            debug!("Rate limit check passed for user: {} ({:?})", user_id, role);
            return Ok(user_result);
        }

        // Check IP-based rate limit for unauthenticated requests
        let ip_result = self.check_ip_limit(context.ip_address)?;
        if !ip_result.allowed {
            self.record_block(&LimiterType::IpAddress);
            warn!("Rate limit exceeded for IP: {}", context.ip_address);
        } else {
            debug!("Rate limit check passed for IP: {}", context.ip_address);
        }

        Ok(ip_result)
    }

    /// Check global rate limit
    fn check_global_limit(&self) -> Result<RateLimitResult> {
        match self.global_limiter.check() {
            Ok(_) => Ok(RateLimitResult {
                allowed: true,
                remaining: self.estimate_remaining(&self.global_limiter),
                reset_time: Duration::from_secs(60), // Quota resets every minute
                limit: self.config.global_limit,
                limiter_type: LimiterType::Global,
            }),
            Err(negative) => {
                let wait_time = negative.wait_time_from(self.clock.now());
                Ok(RateLimitResult {
                    allowed: false,
                    remaining: 0,
                    reset_time: wait_time,
                    limit: self.config.global_limit,
                    limiter_type: LimiterType::Global,
                })
            }
        }
    }

    /// Check user-specific rate limit
    fn check_user_limit(&self, user_id: &str, role: &UserRole) -> Result<RateLimitResult> {
        let limit = self.config.role_limits.get(role)
            .copied()
            .unwrap_or(20); // Default to guest limit if role not found

        let mut user_limiters = self.user_limiters.lock()
            .map_err(|_| anyhow!("Failed to acquire user limiters lock"))?;

        let limiter = user_limiters.entry(user_id.to_string())
            .or_insert_with(|| {
                let quota = Quota::per_minute(
                    NonZeroU32::new(limit).unwrap_or(NonZeroU32::new(1).unwrap())
                ).allow_burst(
                    NonZeroU32::new(self.config.burst_allowance).unwrap_or(NonZeroU32::new(1).unwrap())
                );
                RateLimiter::direct_with_clock(quota, &self.clock)
            });

        match limiter.check() {
            Ok(_) => Ok(RateLimitResult {
                allowed: true,
                remaining: self.estimate_remaining(limiter),
                reset_time: Duration::from_secs(60),
                limit,
                limiter_type: LimiterType::User,
            }),
            Err(negative) => {
                let wait_time = negative.wait_time_from(self.clock.now());
                Ok(RateLimitResult {
                    allowed: false,
                    remaining: 0,
                    reset_time: wait_time,
                    limit,
                    limiter_type: LimiterType::User,
                })
            }
        }
    }

    /// Check IP-based rate limit
    fn check_ip_limit(&self, ip: IpAddr) -> Result<RateLimitResult> {
        let limit = self.config.ip_limit;

        let mut ip_limiters = self.ip_limiters.lock()
            .map_err(|_| anyhow!("Failed to acquire IP limiters lock"))?;

        let limiter = ip_limiters.entry(ip)
            .or_insert_with(|| {
                let quota = Quota::per_minute(
                    NonZeroU32::new(limit).unwrap_or(NonZeroU32::new(1).unwrap())
                ).allow_burst(
                    NonZeroU32::new(self.config.burst_allowance).unwrap_or(NonZeroU32::new(1).unwrap())
                );
                RateLimiter::direct_with_clock(quota, &self.clock)
            });

        match limiter.check() {
            Ok(_) => Ok(RateLimitResult {
                allowed: true,
                remaining: self.estimate_remaining(limiter),
                reset_time: Duration::from_secs(60),
                limit,
                limiter_type: LimiterType::IpAddress,
            }),
            Err(negative) => {
                let wait_time = negative.wait_time_from(self.clock.now());
                Ok(RateLimitResult {
                    allowed: false,
                    remaining: 0,
                    reset_time: wait_time,
                    limit,
                    limiter_type: LimiterType::IpAddress,
                })
            }
        }
    }

    /// Estimate remaining requests in the current window
    fn estimate_remaining(&self, limiter: &RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware>) -> u32 {
        // This is an approximation since governor doesn't expose remaining count directly
        match limiter.check() {
            Ok(_) => {
                // If we can make a request, assume at least some remaining
                // This is conservative - real implementation would need governor internals
                10
            },
            Err(_) => 0,
        }
    }

    /// Update statistics for the request
    fn update_stats(&self, context: &RequestContext) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_requests += 1;
            
            // Track by role if authenticated
            if let Some(role) = &context.user_role {
                *stats.requests_by_role.entry(format!("{:?}", role)).or_insert(0) += 1;
            }
        }

        // Track IP requests
        if let Ok(mut ip_requests) = self.ip_requests.lock() {
            *ip_requests.entry(context.ip_address).or_insert(0) += 1;
        }
    }

    /// Record a rate limit block
    fn record_block(&self, limiter_type: &LimiterType) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.blocked_requests += 1;
            *stats.blocks_by_type.entry(format!("{:?}", limiter_type)).or_insert(0) += 1;
        }
    }

    /// Get current rate limiting statistics
    pub fn get_stats(&self) -> Result<RateLimitStats> {
        let mut stats = self.stats.lock()
            .map_err(|_| anyhow!("Failed to acquire stats lock"))?
            .clone();

        stats.allowed_requests = stats.total_requests - stats.blocked_requests;

        // Update top IPs
        if let Ok(ip_requests) = self.ip_requests.lock() {
            let mut ip_vec: Vec<_> = ip_requests.iter()
                .map(|(ip, count)| (ip.to_string(), *count))
                .collect();
            ip_vec.sort_by(|a, b| b.1.cmp(&a.1));
            stats.top_ips = ip_vec.into_iter().take(10).collect();
        }

        Ok(stats)
    }

    /// Clear old limiters to prevent memory leaks
    pub fn cleanup_old_limiters(&self) -> Result<()> {
        // Clean up user limiters (keep only recent ones)
        if let Ok(mut user_limiters) = self.user_limiters.lock() {
            // In a real implementation, you'd track last access time and remove old entries
            if user_limiters.len() > 10000 {
                user_limiters.clear();
                info!("Cleaned up user rate limiters");
            }
        }

        // Clean up IP limiters
        if let Ok(mut ip_limiters) = self.ip_limiters.lock() {
            if ip_limiters.len() > 50000 {
                ip_limiters.clear();
                info!("Cleaned up IP rate limiters");
            }
        }

        Ok(())
    }

    /// Update rate limiting configuration
    pub fn update_config(&mut self, new_config: RateLimitConfig) -> Result<()> {
        info!("Updating rate limiting configuration");
        self.config = new_config;

        // Clear existing limiters to apply new limits
        if let Ok(mut user_limiters) = self.user_limiters.lock() {
            user_limiters.clear();
        }
        if let Ok(mut ip_limiters) = self.ip_limiters.lock() {
            ip_limiters.clear();
        }

        info!("Rate limiting configuration updated successfully");
        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> &RateLimitConfig {
        &self.config
    }

    /// Check if rate limiting is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Enable or disable rate limiting
    pub fn set_enabled(&mut self, enabled: bool) {
        info!("Rate limiting {}", if enabled { "enabled" } else { "disabled" });
        self.config.enabled = enabled;
    }
}

/// Helper function to create request context
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
        timestamp: std::time::Instant::now(),
        endpoint,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;


    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.role_limits[&UserRole::Admin], 1000);
        assert_eq!(config.role_limits[&UserRole::Guest], 20);
        assert!(config.enabled);
    }

    #[test]
    fn test_rate_limit_manager_creation() {
        let config = RateLimitConfig::default();
        let manager = RateLimitManager::new(config).unwrap();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_user_rate_limiting() {
        let mut config = RateLimitConfig::default();
        config.role_limits.insert(UserRole::User, 2); // Very low limit for testing
        
        let manager = RateLimitManager::new(config).unwrap();
        
        let context = create_request_context(
            Some("test_user".to_string()),
            Some(UserRole::User),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            "test_endpoint".to_string(),
        );

        // First request should be allowed
        let result = manager.check_rate_limit(&context).unwrap();
        assert!(result.allowed);
        assert_eq!(result.limiter_type, LimiterType::User);

        // Second request should be allowed (within burst)
        let result = manager.check_rate_limit(&context).unwrap();
        assert!(result.allowed);

        // Third request might be blocked depending on burst allowance
        // This test demonstrates the rate limiting behavior
    }

    #[test]
    fn test_ip_rate_limiting() {
        let mut config = RateLimitConfig::default();
        config.ip_limit = 1; // Very low limit for testing
        
        let manager = RateLimitManager::new(config).unwrap();
        
        let context = create_request_context(
            None,
            None,
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            "test_endpoint".to_string(),
        );

        // First request should be allowed
        let result = manager.check_rate_limit(&context).unwrap();
        assert!(result.allowed);
        assert_eq!(result.limiter_type, LimiterType::IpAddress);
    }

    #[test]
    fn test_disabled_rate_limiting() {
        let mut config = RateLimitConfig::default();
        config.enabled = false;
        
        let manager = RateLimitManager::new(config).unwrap();
        
        let context = create_request_context(
            None,
            None,
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            "test_endpoint".to_string(),
        );

        let result = manager.check_rate_limit(&context).unwrap();
        assert!(result.allowed);
        assert_eq!(result.limiter_type, LimiterType::None);
    }

    #[test]
    fn test_statistics_tracking() {
        let config = RateLimitConfig::default();
        let manager = RateLimitManager::new(config).unwrap();
        
        let context = create_request_context(
            Some("stats_test_user".to_string()),
            Some(UserRole::Developer),
            IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1)),
            "stats_test".to_string(),
        );

        // Make a request to generate stats
        let _result = manager.check_rate_limit(&context).unwrap();
        
        let stats = manager.get_stats().unwrap();
        assert!(stats.total_requests > 0);
        assert!(stats.requests_by_role.contains_key("Developer"));
    }
} 