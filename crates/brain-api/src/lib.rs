//! Brain API - Visualization and Web Interface Layer
//!
//! This crate provides web-based visualization capabilities for the Brain AI system,
//! including interactive concept graph exploration, memory timeline visualization,
//! and simulation results dashboards.

pub mod visualization;
pub mod web_server;

pub use visualization::*;
pub use web_server::*;
