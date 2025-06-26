//! Brain AI Core Domain Logic
//! 
//! This crate contains pure business logic with no I/O dependencies.
//! All infrastructure concerns are abstracted behind traits.

pub mod memory;
pub mod concepts;
pub mod segmentation;
pub mod insights;
pub mod neural;

// Re-export core traits and types
pub use memory::*;
pub use concepts::*;
pub use segmentation::*;
pub use insights::*;
pub use neural::*;
