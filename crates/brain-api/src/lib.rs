//! Brain API - Visualization and Web Interface Layer
//!
//! This crate provides web-based visualization capabilities for the Brain AI system,
//! including interactive concept graph exploration, memory timeline visualization,
//! and simulation results dashboards.

#![recursion_limit = "1024"]

pub mod visualization;
pub mod web_server;
pub mod auth;
pub mod rate_limit;
pub mod logging;
pub mod agents;
pub mod websocket;

pub use visualization::*;
pub use web_server::*;
pub use auth::{AuthManager, AuthConfig, User, UserRole, Permission, AuthResult};
pub use rate_limit::{RateLimitManager, RateLimitConfig, RequestContext, create_request_context};
pub use logging::{LoggingManager, LoggingConfig, ErrorCategory, ErrorSeverity};
pub use visualization::{VisualizationManager, VisualizationConfig};

// Re-export agent API types
pub use agents::{
    AgentApiManager, AgentExecutionRequest, AgentExecutionResponse,
    AgentListResponse, AgentStatusResponse, WorkflowExecutionRequest,
    WorkflowExecutionResponse, CreateProfileRequest, CreateProfileResponse,
    ProfileListResponse, ExecutionContext, ProjectContext, AgentInfo, AgentStatus,
};

// Re-export WebSocket types
pub use websocket::{
    WebSocketManager, WebSocketMessage, WebSocketClient, SubscriptionRequest,
};
