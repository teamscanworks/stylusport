// In normalization/inference.rs
use crate::error::Result;
use crate::model::{
    instruction::{BasicOperation, InstructionBody},
    NormalizedAccountStruct, NormalizedConstraint, NormalizedInstruction, NormalizedProgram,
};

/// Infer missing semantic information in the normalized program
///
/// This function adds semantics that aren't explicitly present in the
/// parsed AST but can be inferred from known patterns and conventions.
///
/// # Arguments
///
/// * `program` - The normalized program to enhance
///
/// # Returns
///
/// Success or an error if inference fails
pub fn infer_missing_semantics(program: &mut NormalizedProgram) -> Result<()> {
    // Infer operation semantics for instructions
    infer_instruction_operations(program)?;

    // Infer field constraints not explicitly provided
    infer_field_constraints(program)?;

    // Infer relationships between accounts
    infer_account_relationships(program)?;

    Ok(())
}

/// Infer basic operations for instructions based on their name and accounts
fn infer_instruction_operations(program: &mut NormalizedProgram) -> Result<()> {
    // First, collect the operations for each instruction
    let mut instruction_operations = Vec::new();

    for module_idx in 0..program.modules.len() {
        for instr_idx in 0..program.modules[module_idx].instructions.len() {
            let instruction = &program.modules[module_idx].instructions[instr_idx];

            // Skip if already has detailed body
            if let Some(InstructionBody::Basic(_)) = &instruction.body {
                continue;
            }

            // Try to infer operations
            if let Some(account_name) = &instruction.account_struct_name {
                if let Some(account) = program.find_account_struct(account_name) {
                    let operations = infer_operations_from_account(instruction, account);
                    if !operations.is_empty() {
                        instruction_operations.push((module_idx, instr_idx, operations));
                    }
                }
            }
        }
    }

    // Now update the instructions with the inferred operations
    for (module_idx, instr_idx, operations) in instruction_operations {
        program.modules[module_idx].instructions[instr_idx].body =
            Some(InstructionBody::Basic(operations));
    }

    Ok(())
}

/// Infer operations based on instruction name and account struct
fn infer_operations_from_account(
    instruction: &NormalizedInstruction,
    account: &NormalizedAccountStruct,
) -> Vec<BasicOperation> {
    let mut operations = Vec::new();

    // Check for init operations based on account constraints
    for field in &account.fields {
        if field
            .constraints
            .iter()
            .any(|c| c.constraint_type == "init")
        {
            // Find payer if specified
            let payer = field
                .constraints
                .iter()
                .find(|c| c.constraint_type == "payer")
                .and_then(|c| c.value.clone())
                .unwrap_or_else(|| "payer".to_string());

            operations.push(BasicOperation::Initialize {
                target: field.name.clone(),
                payer,
            });
        }
    }

    // Add more operations based on instruction name
    match instruction.name.as_str() {
        "initialize" | "init" | "create" => {
            // Already handled by init constraint check
        }
        "transfer" | "send" => {
            if let (Some(from), Some(to)) = (account.find_field("from"), account.find_field("to")) {
                operations.push(BasicOperation::Transfer {
                    from: from.name.clone(),
                    to: to.name.clone(),
                });
            }
        }
        "close" => {
            if let Some(close_field) = account
                .fields
                .iter()
                .find(|f| f.constraints.iter().any(|c| c.constraint_type == "close"))
            {
                // Find destination for lamports
                let refund_to = close_field
                    .constraints
                    .iter()
                    .find(|c| c.constraint_type == "close")
                    .and_then(|c| c.value.clone())
                    .unwrap_or_else(|| "authority".to_string());

                operations.push(BasicOperation::Close {
                    target: close_field.name.clone(),
                    refund_to,
                });
            }
        }
        _ => {
            // No operations inferred for other instruction types
        }
    }

    operations
}

/// Infer constraints that aren't explicitly specified
fn infer_field_constraints(program: &mut NormalizedProgram) -> Result<()> {
    // Collect the constraints to add
    let mut constraints_to_add = Vec::new();

    for account_idx in 0..program.account_structs.len() {
        for field_idx in 0..program.account_structs[account_idx].fields.len() {
            let field = &program.account_structs[account_idx].fields[field_idx];

            // Infer signer constraint for fields named "authority"
            if (field.name == "authority" || field.name == "owner" || field.name == "admin")
                && !field
                    .constraints
                    .iter()
                    .any(|c| c.constraint_type == "signer")
                && field.ty.contains("Signer")
            {
                constraints_to_add.push((
                    account_idx,
                    field_idx,
                    NormalizedConstraint::without_value("signer", true),
                ));
            }

            // Infer mut constraint for fields that have init
            if field
                .constraints
                .iter()
                .any(|c| c.constraint_type == "init")
                && !field.constraints.iter().any(|c| c.constraint_type == "mut")
            {
                constraints_to_add.push((
                    account_idx,
                    field_idx,
                    NormalizedConstraint::without_value("mut", true),
                ));
            }
        }
    }

    // Add the constraints
    for (account_idx, field_idx, constraint) in constraints_to_add {
        program.account_structs[account_idx].fields[field_idx].add_constraint(constraint);
    }

    Ok(())
}

/// Infer relationships between accounts
fn infer_account_relationships(program: &mut NormalizedProgram) -> Result<()> {
    // Collect the relationships to add
    let mut relationships_to_add = Vec::new();

    // For each account struct, look for related fields
    for account_idx in 0..program.account_structs.len() {
        let account = &program.account_structs[account_idx];

        // Look for fields that might be related to other fields
        for i in 0..account.fields.len() {
            let field_name = account.fields[i].name.clone();
            // Remove unused variable warning by prefixing with underscore
            let _field_type = account.fields[i].ty.clone();

            // Search for other fields that might reference this one
            for j in 0..account.fields.len() {
                if i == j {
                    continue;
                }

                // Check constraints that might relate fields
                for constraint in &account.fields[j].constraints {
                    if constraint.constraint_type == "has_one"
                        || constraint.constraint_type == "belongs_to"
                    {
                        if let Some(value) = &constraint.value {
                            if value == &field_name {
                                // Add relationship to update later
                                relationships_to_add.push((account_idx, j, field_name.clone()));
                            }
                        }
                    }
                }
            }
        }
    }

    // Update the relationships
    for (account_idx, field_idx, related_field) in relationships_to_add {
        program.account_structs[account_idx].fields[field_idx]
            .inferred_info
            .related_account = Some(related_field);
    }

    Ok(())
}
