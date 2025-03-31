//! Anchor Program Parser library
//!
//! This library provides utilities for parsing and displaying
//! the AST of Anchor programs.

pub mod display;
pub mod parser;

// Re-export commonly used items for convenience
pub use display::print_ast;
pub use parser::parse_file;
