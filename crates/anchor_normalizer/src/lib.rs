// In lib.rs
pub mod error;
pub mod model; // This makes the model module public
pub mod normalization;

use crate::error::Result;
pub use error::NormalizeError;

use crate::normalization::normalize_program;
use anchor_parser::model::Program;

/// Normalize an Anchor program
///
/// This is the main entry point for the normalizer and transforms the
/// parsed AST into a semantically rich model suitable for IR generation.
///
/// # Arguments
///
/// * `program` - The parsed Anchor program
///
/// # Returns
///
/// A normalized program model or an error if normalization fails
pub fn normalize(program: &Program) -> Result<model::NormalizedProgram> {
    normalize_program(program)
}

// Re-export all relevant types for convenience
pub use crate::model::{
    BasicOperation, InstructionBody, NormalizedAccountField, NormalizedAccountStruct,
    NormalizedConstraint, NormalizedInstruction, NormalizedModule, NormalizedProgram,
    NormalizedRawAccount,
};
