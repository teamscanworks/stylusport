// In normalization/validation.rs
use crate::error::Result;
use crate::model::{validation::ValidationIssue, NormalizedProgram};
use std::collections::HashSet;

/// Validate a normalized program
///
/// Checks the program structure for consistency and completeness.
///
/// # Arguments
///
/// * `program` - The normalized program to validate
///
/// # Returns
///
/// Success or an error if validation fails
pub fn validate_program(program: &mut NormalizedProgram) -> Result<()> {
    // Collect validation issues in a Vec
    let mut issues = Vec::new();

    // Check for unique account struct names
    validate_unique_account_names(program, &mut issues);

    // Validate instruction references to account structs
    validate_instruction_references(program, &mut issues);

    // Validate field types
    validate_field_types(program, &mut issues);

    // Check for consistent visibility
    validate_visibility(program, &mut issues);

    // Add all collected issues to the program
    for issue in issues {
        program.add_validation_issue(issue);
    }

    Ok(())
}

/// Validate that account struct names are unique
fn validate_unique_account_names(program: &NormalizedProgram, issues: &mut Vec<ValidationIssue>) {
    let mut names = HashSet::new();

    // Check account structs
    for account in &program.account_structs {
        if !names.insert(&account.name) {
            issues.push(ValidationIssue::error(
                format!("Duplicate account struct name: {}", account.name),
                account.name.clone(),
            ));
        }
    }

    // Check raw accounts
    for account in &program.raw_accounts {
        if !names.insert(&account.name) {
            issues.push(ValidationIssue::error(
                format!("Duplicate account name: {}", account.name),
                account.name.clone(),
            ));
        }
    }
}

/// Validate that instruction references to account structs are valid
fn validate_instruction_references(program: &NormalizedProgram, issues: &mut Vec<ValidationIssue>) {
    let account_names: HashSet<_> = program
        .account_structs
        .iter()
        .map(|a| a.name.clone())
        .collect();

    for module in &program.modules {
        for instruction in &module.instructions {
            if let Some(account_name) = &instruction.account_struct_name {
                if !account_names.contains(account_name) {
                    issues.push(ValidationIssue::warning(
                        format!(
                            "Instruction {} references undefined account struct {}",
                            instruction.name, account_name
                        ),
                        instruction.name.clone(),
                    ));
                }
            } else if instruction.has_context_parameter() {
                issues.push(ValidationIssue::warning(
                    format!(
                        "Instruction {} has Context parameter but no associated account struct",
                        instruction.name
                    ),
                    instruction.name.clone(),
                ));
            }
        }
    }
}

/// Validate field types
fn validate_field_types(program: &NormalizedProgram, issues: &mut Vec<ValidationIssue>) {
    // Check account struct fields
    for account in &program.account_structs {
        for field in &account.fields {
            if field.ty.is_empty() {
                issues.push(ValidationIssue::warning(
                    format!(
                        "Field {} in account {} has no type information",
                        field.name, account.name
                    ),
                    format!("{}.{}", account.name, field.name),
                ));
            }
        }
    }

    // Check raw account fields
    for account in &program.raw_accounts {
        for field in &account.fields {
            if field.ty.is_empty() {
                issues.push(ValidationIssue::warning(
                    format!(
                        "Field {} in raw account {} has no type information",
                        field.name, account.name
                    ),
                    format!("{}.{}", account.name, field.name),
                ));
            }
        }
    }
}

/// Validate visibility consistency
fn validate_visibility(program: &NormalizedProgram, issues: &mut Vec<ValidationIssue>) {
    // Check that exported instructions have public visibility
    for module in &program.modules {
        for instruction in &module.instructions {
            if instruction.visibility != "pub" {
                issues.push(ValidationIssue::info(
                    format!(
                        "Instruction {} has non-public visibility: {}",
                        instruction.name, instruction.visibility
                    ),
                    instruction.name.clone(),
                ));
            }
        }
    }
}
