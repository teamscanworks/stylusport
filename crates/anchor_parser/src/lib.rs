//! anchor_parser: A parser for Solana Anchor programs
//!
//! This crate provides functionality to parse Anchor program source code
//! and convert it into a semantic model.

pub mod error;
pub mod model;
pub mod parser;

pub use error::{ParseError, Result};
pub use model::program::Program;

// Legacy name for compatibility with existing tests
pub mod ast {
    pub use crate::model::program::Program as AnchorAst;
}

// Functions to parse programs
pub use parser::{parse_file, parse_str};