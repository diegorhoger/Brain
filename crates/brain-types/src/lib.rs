//! Brain AI shared types and utilities
//! 
//! This crate provides common types, error definitions, and utilities
//! used across all Brain AI crates.

pub mod error;
pub mod common;
pub mod config;

// Re-export everything for easy access
pub use error::*;
pub use common::*;
pub use config::*;
