use brain::{
    AuthManager, AuthConfig, UserRole, Permission, User,
    RateLimitManager, RateLimitConfig, create_request_context,
    LoggingManager, LoggingConfig, ErrorCategory, ErrorSeverity,
    AuthenticationResult,
};
use std::net::{IpAddr, Ipv4Addr};
use std::collections::HashMap;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 Brain AI - Authentication, Logging & Rate Limiting Demo");
    println!("=========================================================\n");

    // ================================
    // Phase 1: Authentication System
    // ================================
    println!("📋 Phase 1: Authentication System");
    println!("----------------------------------");

    let auth_config = AuthConfig::default();
    let mut auth_manager = AuthManager::new(auth_config)?;

    // Create users with different roles
    let admin_user = User {
        id: "admin_001".to_string(),
        name: "Admin User".to_string(),
        email: "admin@brain.ai".to_string(),
        role: UserRole::Admin,
        created_at: chrono::Utc::now(),
        last_login: None,
        active: true,
        metadata: HashMap::new(),
    };
    auth_manager.add_user(admin_user.clone())?;
    println!("✅ Created admin user: {}", admin_user.id);

    let developer_user = User {
        id: "dev_001".to_string(),
        name: "Developer User".to_string(),
        email: "dev@brain.ai".to_string(),
        role: UserRole::Developer,
        created_at: chrono::Utc::now(),
        last_login: None,
        active: true,
        metadata: HashMap::new(),
    };
    auth_manager.add_user(developer_user.clone())?;
    println!("✅ Created developer user: {}", developer_user.id);

    // Generate API keys
    let admin_api_key = auth_manager.generate_api_key(&admin_user.id, UserRole::Admin, "Demo admin key")?;
    let _dev_api_key = auth_manager.generate_api_key(&developer_user.id, UserRole::Developer, "Demo dev key")?;
    println!("🔑 Generated API keys for admin and developer");

    // Generate JWT tokens
    let _admin_token = auth_manager.generate_token(&admin_user.id, UserRole::Admin)?;
    let dev_token = auth_manager.generate_token(&developer_user.id, UserRole::Developer)?;
    println!("🎫 Generated JWT tokens for admin and developer");

    // Test authentication methods
    println!("\n🔍 Testing Authentication Methods:");
    
    // Test API key authentication
    let (api_user_id, api_role) = auth_manager.validate_api_key(&admin_api_key)?;
    println!("  ✅ API Key Auth: User {} (Role: {:?})", api_user_id, api_role);

    // Test JWT authentication
    let jwt_claims = auth_manager.validate_token(&dev_token)?;
    println!("  ✅ JWT Auth: User {} (Role: {:?})", jwt_claims.sub, jwt_claims.role);

    // Test permission checking
    let has_query_permission = UserRole::Admin.has_permission(&Permission::QueryMemory);
    let has_manage_permission = UserRole::Analyst.has_permission(&Permission::ManageUsers);
    println!("  ✅ Admin has query permission: {}", has_query_permission);
    println!("  ❌ Analyst has manage permission: {}", has_manage_permission);

    // ================================
    // Phase 2: Rate Limiting System
    // ================================
    println!("\n📊 Phase 2: Rate Limiting System");
    println!("--------------------------------");

    let rate_config = RateLimitConfig::default();
    let rate_manager = RateLimitManager::new(rate_config)?;

    // Test different rate limiting scenarios
    let client_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
    let admin_context = create_request_context(
        Some(admin_user.id.clone()), 
        Some(UserRole::Admin),
        client_ip, 
        "admin_endpoint".to_string()
    );

    println!("🚦 Testing Rate Limits by User Role:");

    // Admin user (1000 req/min limit)
    for i in 1..=5 {
        let result = rate_manager.check_rate_limit(&admin_context)?;
        println!("  Admin Request {}: {} (Remaining: {})", 
                 i, if result.allowed { "✅ ALLOWED" } else { "❌ BLOCKED" }, result.remaining);
    }

    // Test IP-based rate limiting
    println!("\n🌐 Testing IP-based Rate Limiting:");
    let ip_context = create_request_context(
        None,
        None,
        client_ip, 
        "guest_endpoint".to_string()
    );
    for i in 1..=3 {
        let result = rate_manager.check_rate_limit(&ip_context)?;
        println!("  IP Request {}: {} (Remaining: {})", 
                 i, if result.allowed { "✅ ALLOWED" } else { "❌ BLOCKED" }, result.remaining);
    }

    // Get rate limiting statistics
    let stats = rate_manager.get_stats()?;
    println!("\n📈 Rate Limiting Statistics:");
    println!("  Total Requests: {}", stats.total_requests);
    println!("  Allowed Requests: {}", stats.allowed_requests);
    println!("  Blocked Requests: {}", stats.blocked_requests);
    if stats.total_requests > 0 {
        println!("  Block Rate: {:.2}%", (stats.blocked_requests as f64 / stats.total_requests as f64) * 100.0);
    }

    // ================================
    // Phase 3: Logging System
    // ================================
    println!("\n📝 Phase 3: Logging System");
    println!("--------------------------");

    let logging_config = LoggingConfig::default();
    let logging_manager = LoggingManager::new(logging_config)?;

    // Start tracking a request
    let request_id = "req_001".to_string();
    logging_manager.start_request(
        request_id.clone(), 
        "/api/memory/query".to_string(), 
        "POST".to_string(), 
        client_ip
    );

    // Complete the request
    let auth_result = AuthenticationResult::new(api_user_id.clone(), api_role);
    let mut metadata = HashMap::new();
    metadata.insert("query_type".to_string(), "concept_search".to_string());
    metadata.insert("result_count".to_string(), "25".to_string());
    
    logging_manager.complete_request(
        request_id,
        200,
        Some(auth_result),
        metadata,
    );

    // Log some errors
    let mut error_context = HashMap::new();
    error_context.insert("query".to_string(), "SELECT * FROM concepts".to_string());
    
    logging_manager.log_error(
        ErrorCategory::Validation,
        ErrorSeverity::Medium,
        "Invalid query syntax".to_string(),
        Some("Missing WHERE clause".to_string()),
        error_context,
        Some("req_001".to_string()),
        Some(api_user_id.clone()),
    );

    logging_manager.log_error(
        ErrorCategory::Authentication,
        ErrorSeverity::High,
        "JWT token expired".to_string(),
        Some("Token issued too long ago".to_string()),
        HashMap::new(),
        None,
        Some(api_user_id),
    );

    // Log an audit event
    logging_manager.log_audit(
        "user_action".to_string(),
        admin_user.id.clone(),
        UserRole::Admin,
        "memory_query".to_string(),
        Some("concept_search".to_string()),
        client_ip,
        true,
        HashMap::new(),
    );

    // Get logging statistics
    let log_stats = logging_manager.get_stats()?;
    println!("\n📈 Logging Statistics:");
    println!("  Total Requests: {}", log_stats.total_requests);
    println!("  Successful Requests: {}", log_stats.successful_requests);
    println!("  Failed Requests: {}", log_stats.failed_requests);
    println!("  Average Response Time: {:.2}ms", log_stats.average_response_time_ms);

    // Get recent errors
    let recent_errors = logging_manager.get_recent_errors(5)?;
    println!("\n📋 Recent Errors:");
    for error in recent_errors {
        println!("  {} - {}: {} ({})", 
                 error.timestamp.format("%H:%M:%S"),
                 error.category,
                 error.message,
                 error.severity);
    }

    // ================================
    // Phase 4: Integration Demo
    // ================================
    println!("\n🔗 Phase 4: Integration Demo");
    println!("----------------------------");

    // Get authentication statistics
    let auth_stats = auth_manager.get_stats();
    println!("👥 Authentication Statistics:");
    println!("  Total Users: {}", auth_stats.total_users);
    println!("  Active Users: {}", auth_stats.active_users);
    println!("  Total API Keys: {}", auth_stats.total_api_keys);
    println!("  Active API Keys: {}", auth_stats.active_api_keys);

    println!("\n🎉 Brain AI Authentication & Logging Demo Complete!");
    println!("====================================================");
    println!("✅ Authentication: Users, API keys, JWT tokens");
    println!("✅ Rate Limiting: Role-based and IP-based limits");
    println!("✅ Logging: Request tracking, error logging, audit trails");
    println!("✅ Integration: All systems working together");

    Ok(())
} 