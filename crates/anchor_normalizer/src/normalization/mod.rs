//! Normalization logic for Anchor programs
//!
//! This module contains the logic for transforming parsed AST into
//! a semantically rich normalized model.

pub mod account;
pub mod inference;
pub mod instruction;
pub mod program;
pub mod validation;

// Re-export the main normalization function
pub use program::normalize_program;
