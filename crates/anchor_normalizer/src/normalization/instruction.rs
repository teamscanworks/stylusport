//! Instruction normalization logic
//!
//! Handles normalization of Anchor instruction definitions

use crate::error::Result;
use crate::model::instruction::{InstructionBody, NormalizedInstruction, NormalizedParameter};
use anchor_parser::model::instruction::{Instruction, Parameter};

/// Normalize an Anchor instruction
///
/// Transforms a parsed instruction into a normalized form with enhanced semantics.
///
/// # Arguments
///
/// * `instruction` - The parsed instruction
///
/// # Returns
///
/// A normalized instruction or an error if normalization fails
pub fn normalize_instruction(instruction: &Instruction) -> Result<NormalizedInstruction> {
    let mut normalized =
        NormalizedInstruction::new(instruction.name.clone(), instruction.visibility.clone());

    // Set return type if available
    if let Some(ret_type) = &instruction.return_type {
        normalized = normalized.with_return_type(ret_type);
    }

    // Set account struct name if available in the parsed instruction
    if let Some(ctx_type) = &instruction.context_type {
        normalized = normalized.with_account_struct(ctx_type);
    }

    // Normalize parameters
    for param in &instruction.parameters {
        normalized.add_parameter(normalize_parameter(param)?);
    }

    // Set instruction body (unknown for now)
    normalized = normalized.with_body(InstructionBody::Unknown);

    Ok(normalized)
}

/// Normalize an instruction parameter
fn normalize_parameter(param: &Parameter) -> Result<NormalizedParameter> {
    Ok(NormalizedParameter::new(
        param.name.clone(),
        param.ty.clone(),
        param.is_context,
    ))
}

/// Extract context type from a parameter type string
pub fn extract_context_type(ty: &str) -> Option<String> {
    // Handle Context<T> pattern
    if ty.starts_with("Context<") && ty.ends_with('>') {
        let inner = &ty["Context<".len()..ty.len() - 1];
        return Some(inner.trim().to_string());
    }
    None
}
