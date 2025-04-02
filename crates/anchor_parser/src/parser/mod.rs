

mod predicates;
pub mod convert;

use crate::error::{ParseError, Result};
use crate::model::program::Program;
use std::path::Path;
use std::fs;

/// Parse an Anchor program file into a Program model
pub fn parse_file(path: &Path) -> Result<Program> {
    let source = fs::read_to_string(path)?;
    parse_str(&source)
}

/// Parse Anchor program source code into a Program model
pub fn parse_str(source: &str) -> Result<Program> {
    // First, parse with syn
    let file = syn::parse_str::<syn::File>(source)
        .map_err(ParseError::Syntax)?;
    
    // Then convert to our model
    convert::convert_file(&file)
}

// Re-export for compatibility with existing code
pub use predicates::{is_anchor_program, is_anchor_instruction};