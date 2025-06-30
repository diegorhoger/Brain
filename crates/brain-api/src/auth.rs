//! Authentication and Authorization Module
//!
//! This module provides comprehensive authentication, authorization, and security
//! features for the Brain AI API including JWT tokens, API keys, role-based access
//! control, and user management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use anyhow::{Result, Context};
use brain_types::BrainError;

/// User roles defining different access levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum UserRole {
    Admin,
    Developer,
    Analyst,
    Viewer,
}

/// System permissions for fine-grained access control
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    // Memory permissions
    QueryMemory,
    WriteMemory,
    DeleteMemory,
    ManageMemory,
    
    // User management permissions
    CreateUser,
    ReadUser,
    UpdateUser,
    DeleteUser,
    ManageUsers,
    
    // System permissions
    SystemAdmin,
    ViewLogs,
    ManageLogs,
    SystemMetrics,
    
    // API permissions
    UseAPI,
    ManageAPI,
    RateLimitExempt,
}

impl UserRole {
    /// Check if this role has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        match self {
            UserRole::Admin => true, // Admins have all permissions
            UserRole::Developer => matches!(permission,
                Permission::QueryMemory | Permission::WriteMemory | Permission::DeleteMemory |
                Permission::ReadUser | Permission::UseAPI | Permission::ViewLogs |
                Permission::SystemMetrics
            ),
            UserRole::Analyst => matches!(permission,
                Permission::QueryMemory | Permission::ReadUser | Permission::UseAPI |
                Permission::ViewLogs | Permission::SystemMetrics
            ),
            UserRole::Viewer => matches!(permission,
                Permission::QueryMemory | Permission::ReadUser | Permission::UseAPI
            ),
        }
    }
    
    /// Get the default rate limit for this role (requests per minute)
    pub fn default_rate_limit(&self) -> u32 {
        match self {
            UserRole::Admin => 1000,
            UserRole::Developer => 500,
            UserRole::Analyst => 300,
            UserRole::Viewer => 100,
        }
    }
}

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub active: bool,
    pub metadata: HashMap<String, String>,
}

impl User {
    pub fn new(id: String, name: String, email: String, role: UserRole) -> Self {
        Self {
            id,
            name,
            email,
            role,
            created_at: Utc::now(),
            last_login: None,
            active: true,
            metadata: HashMap::new(),
        }
    }
    
    pub fn mark_login(&mut self) {
        self.last_login = Some(Utc::now());
    }
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub role: UserRole,
    pub exp: i64,     // Expiration timestamp
    pub iat: i64,     // Issued at timestamp
    pub iss: String,  // Issuer
}

/// API key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub user_id: String,
    pub role: UserRole,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub active: bool,
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub api_key_prefix: String,
    pub issuer: String,
    pub require_https: bool,
    pub session_timeout_minutes: i64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "brain_ai_jwt_secret_key_changeme_in_production".to_string(),
            jwt_expiration_hours: 24,
            api_key_prefix: "brain_".to_string(),
            issuer: "brain-ai".to_string(),
            require_https: false, // Development default
            session_timeout_minutes: 60,
        }
    }
}

/// Authentication result
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub user_id: String,
    pub role: UserRole,
    pub authenticated_at: DateTime<Utc>,
}

impl AuthResult {
    pub fn new(user_id: String, role: UserRole) -> Self {
        Self {
            user_id,
            role,
            authenticated_at: Utc::now(),
        }
    }
}

/// Main authentication manager
pub struct AuthManager {
    config: AuthConfig,
    users: HashMap<String, User>,
    api_keys: HashMap<String, ApiKey>,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(config: AuthConfig) -> Result<Self> {
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_ref());
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_ref());
        
        Ok(Self {
            config,
            users: HashMap::new(),
            api_keys: HashMap::new(),
            encoding_key,
            decoding_key,
        })
    }
    
    /// Add a new user to the system
    pub fn add_user(&mut self, user: User) -> Result<()> {
        if self.users.contains_key(&user.id) {
            return Err(BrainError::Conflict(format!("User {} already exists", user.id)).into());
        }
        
        self.users.insert(user.id.clone(), user);
        Ok(())
    }
    
    /// Get a user by ID
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }
    
    /// Update user login timestamp
    pub fn mark_user_login(&mut self, user_id: &str) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            user.mark_login();
            Ok(())
        } else {
            Err(BrainError::NotFound(format!("User {} not found", user_id)).into())
        }
    }
    
    /// Generate a new API key for a user
    pub fn generate_api_key(&mut self, user_id: &str, role: UserRole, description: &str) -> Result<String> {
        // Verify user exists
        if !self.users.contains_key(user_id) {
            return Err(BrainError::NotFound(format!("User {} not found", user_id)).into());
        }
        
        let key = format!("{}{}", self.config.api_key_prefix, Uuid::new_v4().to_string().replace("-", ""));
        
        let api_key = ApiKey {
            key: key.clone(),
            user_id: user_id.to_string(),
            role,
            description: description.to_string(),
            created_at: Utc::now(),
            last_used: None,
            active: true,
        };
        
        self.api_keys.insert(key.clone(), api_key);
        Ok(key)
    }
    
    /// Validate an API key and return user info
    pub fn validate_api_key(&mut self, key: &str) -> Result<(String, UserRole)> {
        if let Some(api_key) = self.api_keys.get_mut(key) {
            if !api_key.active {
                return Err(BrainError::Unauthorized("API key is disabled".to_string()).into());
            }
            
            // Update last used timestamp
            api_key.last_used = Some(Utc::now());
            
            let user_id = api_key.user_id.clone();
            let role = api_key.role.clone();
            
            // Mark user login
            self.mark_user_login(&user_id)?;
            
            Ok((user_id, role))
        } else {
            Err(BrainError::Unauthorized("Invalid API key".to_string()).into())
        }
    }
    
    /// Generate a JWT token for a user
    pub fn generate_token(&self, user_id: &str, role: UserRole) -> Result<String> {
        // Verify user exists
        if !self.users.contains_key(user_id) {
            return Err(BrainError::NotFound(format!("User {} not found", user_id)).into());
        }
        
        let now = Utc::now();
        let exp = now + chrono::Duration::hours(self.config.jwt_expiration_hours);
        
        let claims = Claims {
            sub: user_id.to_string(),
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            iss: self.config.issuer.clone(),
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .context("Failed to encode JWT token")
    }
    
    /// Validate a JWT token and return claims
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.config.issuer]);
        
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .context("Failed to decode JWT token")?;
            
        Ok(token_data.claims)
    }
    
    /// Revoke an API key
    pub fn revoke_api_key(&mut self, key: &str) -> Result<bool> {
        if let Some(api_key) = self.api_keys.get_mut(key) {
            api_key.active = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get authentication statistics
    pub fn get_stats(&self) -> AuthStats {
        let total_users = self.users.len();
        let active_users = self.users.values().filter(|u| u.active).count();
        let total_api_keys = self.api_keys.len();
        let active_api_keys = self.api_keys.values().filter(|k| k.active).count();
        
        let mut role_distribution = HashMap::new();
        for user in self.users.values() {
            *role_distribution.entry(user.role.clone()).or_insert(0) += 1;
        }
        
        AuthStats {
            total_users,
            active_users,
            total_api_keys,
            active_api_keys,
            role_distribution,
        }
    }
}

/// Authentication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStats {
    pub total_users: usize,
    pub active_users: usize,
    pub total_api_keys: usize,
    pub active_api_keys: usize,
    pub role_distribution: HashMap<UserRole, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "test_001".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
            UserRole::Developer,
        );
        
        assert_eq!(user.id, "test_001");
        assert_eq!(user.role, UserRole::Developer);
        assert!(user.active);
    }

    #[test]
    fn test_role_permissions() {
        assert!(UserRole::Admin.has_permission(&Permission::ManageUsers));
        assert!(UserRole::Developer.has_permission(&Permission::QueryMemory));
        assert!(!UserRole::Viewer.has_permission(&Permission::DeleteMemory));
    }

    #[tokio::test]
    async fn test_auth_manager() {
        let config = AuthConfig::default();
        let mut auth_manager = AuthManager::new(config).unwrap();
        
        let user = User::new(
            "test_001".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
            UserRole::Developer,
        );
        
        auth_manager.add_user(user).unwrap();
        
        let api_key = auth_manager.generate_api_key("test_001", UserRole::Developer, "Test key").unwrap();
        let (user_id, role) = auth_manager.validate_api_key(&api_key).unwrap();
        
        assert_eq!(user_id, "test_001");
        assert_eq!(role, UserRole::Developer);
    }
} 