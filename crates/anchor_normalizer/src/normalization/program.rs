//! Program normalization logic
//!
//! Handles normalization of the top-level Program structure

use crate::error::{NormalizationError, Result};
use crate::model::{NormalizedModule, NormalizedProgram, SourceInfo};
use crate::normalization::{
    account::{normalize_account_struct, normalize_raw_account},
    inference::infer_missing_semantics,
    instruction::normalize_instruction,
    validation::validate_program,
};
use anchor_parser::model::{Program, ProgramModule};

/// Normalize an Anchor program into a semantically rich model
///
/// This is the main entry point for the normalizer and transforms the
/// parsed AST into a model suitable for IR generation.
///
/// # Arguments
///
/// * `program` - The parsed Anchor program
///
/// # Returns
///
/// A normalized program model or an error if normalization fails
pub fn normalize_program(program: &Program) -> Result<NormalizedProgram> {
    // Extract program name
    let name = extract_program_name(program)?;

    // Generate a program ID
    let id = generate_program_id(program);

    // Create the base normalized program
    let mut normalized = NormalizedProgram::new(id, name);

    // Extract source information if available
    if let Some(source_path) = &program.source_path {
        normalized.source_info = Some(SourceInfo::new(source_path));
    }

    // Normalize program modules
    for module in &program.program_modules {
        normalized.add_module(normalize_module(module)?);
    }

    // Normalize account structs
    for account in &program.account_structs {
        normalized.add_account_struct(normalize_account_struct(account)?);
    }

    // Normalize raw accounts
    for account in &program.raw_accounts {
        normalized.add_raw_account(normalize_raw_account(account)?);
    }

    // Establish relationships between instructions and account structs
    link_instructions_to_accounts(&mut normalized)?;

    // Infer missing semantic information
    infer_missing_semantics(&mut normalized)?;

    // Validate the normalized program
    validate_program(&mut normalized)?;

    Ok(normalized)
}

/// Normalize a program module
fn normalize_module(module: &ProgramModule) -> Result<NormalizedModule> {
    let mut normalized = NormalizedModule::new(module.name.clone(), module.visibility.clone());

    // Normalize instructions
    for instruction in &module.instructions {
        normalized.add_instruction(normalize_instruction(instruction)?);
    }

    Ok(normalized)
}

/// Extract the program name from the Program model
fn extract_program_name(program: &Program) -> Result<String> {
    // If there's only one program module, use its name
    if program.program_modules.len() == 1 {
        return Ok(program.program_modules[0].name.clone());
    }

    // If there are multiple program modules, use the first one
    // but add a validation warning
    if !program.program_modules.is_empty() {
        return Ok(program.program_modules[0].name.clone());
    }

    // If there are no program modules, try to infer from source path
    if let Some(source_path) = &program.source_path {
        // Try to extract the base filename without extension
        if let Some(file_name) = std::path::Path::new(source_path)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            return Ok(file_name.to_string());
        }
    }

    // If we can't determine a name, return an error
    Err(NormalizationError::MissingInfo(
        "Could not determine program name".to_string(),
    ))
}

/// Generate a program ID based on the program
fn generate_program_id(program: &Program) -> String {
    // Use source path if available
    if let Some(source_path) = &program.source_path {
        return format!("program:{}", source_path);
    }

    // Otherwise use the first module name
    if !program.program_modules.is_empty() {
        return format!("program:{}", program.program_modules[0].name);
    }

    // Fallback to a timestamp-based ID
    format!("program:{}", chrono::Utc::now().timestamp())
}

/// Link instructions to their account structures
fn link_instructions_to_accounts(program: &mut NormalizedProgram) -> Result<()> {
    for module in &mut program.modules {
        for instruction in &mut module.instructions {
            // Skip if already set
            if instruction.account_struct_name.is_some() {
                continue;
            }

            // Find the context parameter
            for param in &instruction.parameters {
                if param.is_context {
                    // Extract account name from Context<Name>
                    if let Some(ctx_type) = extract_context_type(&param.ty) {
                        instruction.account_struct_name = Some(ctx_type);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Extract the account name from a Context<Name> type
fn extract_context_type(ty: &str) -> Option<String> {
    // Simple string-based extraction for now
    // This will be improved when parser provides better type information
    let start = ty.find('<')? + 1;
    let end = ty.rfind('>')?;

    if start < end {
        Some(ty[start..end].trim().to_string())
    } else {
        None
    }
}
