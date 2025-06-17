//! Authentication Module - Task 7.3
//!
//! This module provides comprehensive authentication capabilities for the Brain AI system,
//! including JWT token support, API key authentication, and role-based permission management.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// User roles with different permission levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    /// Administrative access - full system control
    Admin,
    /// Developer access - query, export, learn capabilities
    Developer,
    /// Analyst access - query and export only
    Analyst,
    /// Basic user - limited query access only
    User,
    /// Guest access - read-only basic queries
    Guest,
}

impl UserRole {
    /// Get rate limit per minute for this role
    pub fn rate_limit_per_minute(&self) -> u32 {
        match self {
            UserRole::Admin => 1000,
            UserRole::Developer => 500,
            UserRole::Analyst => 200,
            UserRole::User => 100,
            UserRole::Guest => 20,
        }
    }

    /// Check if role has permission for specific operations
    pub fn has_permission(&self, permission: &Permission) -> bool {
        match permission {
            Permission::QueryMemory => true, // All roles can query
            Permission::SegmentText => matches!(self, UserRole::Admin | UserRole::Developer | UserRole::Analyst),
            Permission::LearnInformation => matches!(self, UserRole::Admin | UserRole::Developer),
            Permission::RunSimulation => matches!(self, UserRole::Admin | UserRole::Developer),
            Permission::ExportData => matches!(self, UserRole::Admin | UserRole::Developer | UserRole::Analyst),
            Permission::ManageUsers => matches!(self, UserRole::Admin),
            Permission::ViewMetrics => matches!(self, UserRole::Admin | UserRole::Developer),
            Permission::SystemConfig => matches!(self, UserRole::Admin),
        }
    }
}

/// System permissions that can be granted to users
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    /// Query memory system
    QueryMemory,
    /// Segment text input
    SegmentText,
    /// Learn new information
    LearnInformation,
    /// Run simulations
    RunSimulation,
    /// Export system data
    ExportData,
    /// Manage other users
    ManageUsers,
    /// View system metrics
    ViewMetrics,
    /// Modify system configuration
    SystemConfig,
}

/// JWT token claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    /// User identifier
    pub sub: String,
    /// User role
    pub role: UserRole,
    /// Token issued at (timestamp)
    pub iat: i64,
    /// Token expires at (timestamp)
    pub exp: i64,
    /// Token issuer
    pub iss: String,
    /// Additional custom claims
    pub custom: HashMap<String, String>,
}

/// API key structure for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Unique identifier for the key
    pub id: String,
    /// User identifier this key belongs to
    pub user_id: String,
    /// User role for permissions
    pub role: UserRole,
    /// Hashed key value (SHA256)
    pub key_hash: String,
    /// Key description/name
    pub description: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<Utc>,
    /// Optional expiration timestamp
    pub expires_at: Option<chrono::DateTime<Utc>>,
    /// Whether the key is active
    pub active: bool,
    /// Usage statistics
    pub usage_count: u64,
    pub last_used: Option<chrono::DateTime<Utc>>,
}

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: String,
    /// User display name
    pub name: String,
    /// User email address
    pub email: String,
    /// User role and permissions
    pub role: UserRole,
    /// Account creation timestamp
    pub created_at: chrono::DateTime<Utc>,
    /// Last login timestamp
    pub last_login: Option<chrono::DateTime<Utc>>,
    /// Whether account is active
    pub active: bool,
    /// Account metadata
    pub metadata: HashMap<String, String>,
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// JWT secret key for signing tokens
    pub jwt_secret: String,
    /// Token expiration duration in hours
    pub token_expiration_hours: i64,
    /// Token issuer string
    pub token_issuer: String,
    /// Whether to require authentication (can be disabled for development)
    pub require_auth: bool,
    /// Default role for unauthenticated users when auth is disabled
    pub default_role: UserRole,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "your-secret-key-change-in-production".to_string(),
            token_expiration_hours: 24,
            token_issuer: "brain-ai-system".to_string(),
            require_auth: true,
            default_role: UserRole::Guest,
        }
    }
}

/// Main authentication manager
pub struct AuthManager {
    config: AuthConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    users: HashMap<String, User>,
    api_keys: HashMap<String, ApiKey>,
}

impl AuthManager {
    /// Create new authentication manager with configuration
    pub fn new(config: AuthConfig) -> Result<Self> {
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());

        let mut manager = Self {
            config,
            encoding_key,
            decoding_key,
            users: HashMap::new(),
            api_keys: HashMap::new(),
        };

        // Create default admin user if none exists
        manager.initialize_default_users()?;

        Ok(manager)
    }

    /// Initialize default users for system setup
    fn initialize_default_users(&mut self) -> Result<()> {
        // Create default admin user
        let admin_user = User {
            id: "admin".to_string(),
            name: "System Administrator".to_string(),
            email: "admin@brain-ai.local".to_string(),
            role: UserRole::Admin,
            created_at: Utc::now(),
            last_login: None,
            active: true,
            metadata: HashMap::new(),
        };

        self.users.insert(admin_user.id.clone(), admin_user);

        // Create a default API key for admin
        let api_key = self.generate_api_key("admin", UserRole::Admin, "Default admin key")?;
        info!("Created default admin API key: {}", api_key);

        Ok(())
    }

    /// Generate JWT token for user
    pub fn generate_token(&self, user_id: &str, role: UserRole) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.token_expiration_hours);

        let claims = TokenClaims {
            sub: user_id.to_string(),
            role,
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: self.config.token_issuer.clone(),
            custom: HashMap::new(),
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to generate token: {}", e))?;

        debug!("Generated JWT token for user: {}", user_id);
        Ok(token)
    }

    /// Validate JWT token and extract claims
    pub fn validate_token(&self, token: &str) -> Result<TokenClaims> {
        let validation = Validation::default();
        
        let token_data = decode::<TokenClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| anyhow!("Invalid token: {}", e))?;

        // Check if user exists and is active
        if let Some(user) = self.users.get(&token_data.claims.sub) {
            if !user.active {
                return Err(anyhow!("User account is disabled"));
            }
        } else {
            return Err(anyhow!("User not found"));
        }

        debug!("Validated JWT token for user: {}", token_data.claims.sub);
        Ok(token_data.claims)
    }

    /// Generate new API key for user
    pub fn generate_api_key(&mut self, user_id: &str, role: UserRole, description: &str) -> Result<String> {
        // Generate random key
        let key_bytes: [u8; 32] = rand::random();
        let key = STANDARD.encode(key_bytes);
        
        // Hash the key for storage
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());

        let api_key = ApiKey {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            role,
            key_hash: key_hash.clone(),
            description: description.to_string(),
            created_at: Utc::now(),
            expires_at: None,
            active: true,
            usage_count: 0,
            last_used: None,
        };

        let key_id = api_key.id.clone();
        self.api_keys.insert(key_hash, api_key);

        info!("Generated API key {} for user: {}", key_id, user_id);
        Ok(key)
    }

    /// Validate API key and return user role
    pub fn validate_api_key(&mut self, key: &str) -> Result<(String, UserRole)> {
        // Hash the provided key
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());

        if let Some(api_key) = self.api_keys.get_mut(&key_hash) {
            if !api_key.active {
                return Err(anyhow!("API key is disabled"));
            }

            // Check expiration
            if let Some(expires_at) = api_key.expires_at {
                if Utc::now() > expires_at {
                    return Err(anyhow!("API key has expired"));
                }
            }

            // Update usage statistics
            api_key.usage_count += 1;
            api_key.last_used = Some(Utc::now());

            debug!("Validated API key for user: {}", api_key.user_id);
            Ok((api_key.user_id.clone(), api_key.role.clone()))
        } else {
            warn!("Invalid API key attempted");
            Err(anyhow!("Invalid API key"))
        }
    }

    /// Add new user to the system
    pub fn add_user(&mut self, user: User) -> Result<()> {
        if self.users.contains_key(&user.id) {
            return Err(anyhow!("User already exists: {}", user.id));
        }

        info!("Adding new user: {} ({:?})", user.name, user.role);
        self.users.insert(user.id.clone(), user);
        Ok(())
    }

    /// Get user by ID
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    /// Update user information
    pub fn update_user(&mut self, user_id: &str, updater: impl FnOnce(&mut User)) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            updater(user);
            info!("Updated user: {}", user_id);
            Ok(())
        } else {
            Err(anyhow!("User not found: {}", user_id))
        }
    }

    /// Disable user account
    pub fn disable_user(&mut self, user_id: &str) -> Result<()> {
        self.update_user(user_id, |user| user.active = false)
    }

    /// Enable user account
    pub fn enable_user(&mut self, user_id: &str) -> Result<()> {
        self.update_user(user_id, |user| user.active = true)
    }

    /// List all users (admin only)
    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    /// Revoke API key
    pub fn revoke_api_key(&mut self, key_hash: &str) -> Result<()> {
        if let Some(api_key) = self.api_keys.get_mut(key_hash) {
            api_key.active = false;
            info!("Revoked API key: {}", api_key.id);
            Ok(())
        } else {
            Err(anyhow!("API key not found"))
        }
    }

    /// List API keys for user
    pub fn list_user_api_keys(&self, user_id: &str) -> Vec<&ApiKey> {
        self.api_keys
            .values()
            .filter(|key| key.user_id == user_id)
            .collect()
    }

    /// Check if authentication is required
    pub fn requires_auth(&self) -> bool {
        self.config.require_auth
    }

    /// Get default role for unauthenticated users
    pub fn default_role(&self) -> UserRole {
        self.config.default_role.clone()
    }

    /// Get authentication statistics
    pub fn get_auth_stats(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("total_users".to_string(), self.users.len() as u64);
        stats.insert("active_users".to_string(), 
                    self.users.values().filter(|u| u.active).count() as u64);
        stats.insert("total_api_keys".to_string(), self.api_keys.len() as u64);
        stats.insert("active_api_keys".to_string(), 
                    self.api_keys.values().filter(|k| k.active).count() as u64);
        stats
    }
}

/// Authentication result containing user info and permissions
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub user_id: String,
    pub role: UserRole,
    pub permissions: Vec<Permission>,
    pub rate_limit: u32,
}

impl AuthResult {
    pub fn new(user_id: String, role: UserRole) -> Self {
        let permissions = vec![
            Permission::QueryMemory,
            Permission::SegmentText,
            Permission::LearnInformation,
            Permission::RunSimulation,
            Permission::ExportData,
            Permission::ManageUsers,
            Permission::ViewMetrics,
            Permission::SystemConfig,
        ]
        .into_iter()
        .filter(|p| role.has_permission(p))
        .collect();

        let rate_limit = role.rate_limit_per_minute();

        Self {
            user_id,
            role,
            permissions,
            rate_limit,
        }
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_permissions() {
        assert!(UserRole::Admin.has_permission(&Permission::SystemConfig));
        assert!(UserRole::Developer.has_permission(&Permission::LearnInformation));
        assert!(!UserRole::Guest.has_permission(&Permission::LearnInformation));
        assert!(UserRole::User.has_permission(&Permission::QueryMemory));
    }

    #[test]
    fn test_auth_manager_creation() {
        let config = AuthConfig::default();
        let auth_manager = AuthManager::new(config).unwrap();
        
        // Should have created default admin user
        assert!(auth_manager.get_user("admin").is_some());
        assert_eq!(auth_manager.get_user("admin").unwrap().role, UserRole::Admin);
    }

    #[test]
    fn test_jwt_token_lifecycle() {
        let config = AuthConfig::default();
        let auth_manager = AuthManager::new(config).unwrap();
        
        // Generate token
        let token = auth_manager.generate_token("admin", UserRole::Admin).unwrap();
        assert!(!token.is_empty());
        
        // Validate token
        let claims = auth_manager.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "admin");
        assert_eq!(claims.role, UserRole::Admin);
    }

    #[test]
    fn test_api_key_lifecycle() {
        let config = AuthConfig::default();
        let mut auth_manager = AuthManager::new(config).unwrap();
        
        // Generate API key
        let api_key = auth_manager.generate_api_key("admin", UserRole::Admin, "Test key").unwrap();
        assert!(!api_key.is_empty());
        
        // Validate API key
        let (user_id, role) = auth_manager.validate_api_key(&api_key).unwrap();
        assert_eq!(user_id, "admin");
        assert_eq!(role, UserRole::Admin);
        
        // Check usage was tracked
        let keys = auth_manager.list_user_api_keys("admin");
        assert_eq!(keys.len(), 2); // Default + test key
        assert!(keys.iter().any(|k| k.usage_count > 0));
    }

    #[test]
    fn test_auth_result_permissions() {
        let admin_auth = AuthResult::new("admin".to_string(), UserRole::Admin);
        assert!(admin_auth.has_permission(&Permission::SystemConfig));
        assert_eq!(admin_auth.rate_limit, 1000);
        
        let guest_auth = AuthResult::new("guest".to_string(), UserRole::Guest);
        assert!(!guest_auth.has_permission(&Permission::LearnInformation));
        assert_eq!(guest_auth.rate_limit, 20);
    }
} 