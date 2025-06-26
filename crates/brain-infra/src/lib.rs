//! Brain AI Infrastructure Layer
//! 
//! This crate provides concrete implementations of repository traits
//! and handles all I/O operations including database access, file system,
//! HTTP clients, and external API integrations.

pub mod memory;
pub mod concepts;
pub mod segmentation;
pub mod insights;
pub mod neural;
pub mod database;
pub mod filesystem;
pub mod http;
pub mod config;

// Re-export key infrastructure components
pub use memory::*;
pub use concepts::InMemoryConceptRepository;
pub use segmentation::{InMemorySegmentRepository, cosine_similarity as segment_cosine_similarity};
pub use insights::*;
pub use neural::*;
pub use database::{DatabaseManager, DatabaseConfig as DbConfig};
pub use filesystem::*;
pub use http::*;
pub use config::{BrainConfig, DatabaseConfig as ConfigDbConfig};
