//! Normalized model definitions for Anchor programs
//!
//! This module defines the data structures that represent a semantically normalized
//! Anchor program, ready for IR generation.

pub mod account;
pub mod instruction;
pub mod program;
pub mod validation;

// Re-export all model types for easier imports
pub use account::*;
pub use instruction::*;
pub use program::*;
pub use validation::*;
