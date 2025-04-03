//! Helper functions for normalizer tests

use anchor_normalizer::{
    BasicOperation, NormalizedAccountField, NormalizedInstruction, NormalizedProgram,
};

/// Asserts the core structure of a normalized program
pub fn assert_program_structure(
    normalized: &NormalizedProgram,
    expected_name: &str,
    expected_modules: usize,
    expected_account_structs: usize,
    expected_raw_accounts: usize,
) {
    assert_eq!(normalized.name, expected_name, "Program name should match");
    assert_eq!(
        normalized.modules.len(),
        expected_modules,
        "Module count should match"
    );
    assert_eq!(
        normalized.account_structs.len(),
        expected_account_structs,
        "Account struct count should match"
    );
    assert_eq!(
        normalized.raw_accounts.len(),
        expected_raw_accounts,
        "Raw account count should match"
    );
}

/// Asserts the basic properties of an instruction
pub fn assert_instruction_basics(
    instruction: &NormalizedInstruction,
    expected_name: &str,
    expected_visibility: &str,
    expected_return_type: Option<&str>,
    expected_account_struct: Option<&str>,
) {
    assert_eq!(
        instruction.name, expected_name,
        "Instruction name should match"
    );
    assert_eq!(
        instruction.visibility, expected_visibility,
        "Instruction visibility should match"
    );
    assert_eq!(
        instruction.return_type,
        expected_return_type.map(String::from),
        "Instruction return type should match"
    );
    assert_eq!(
        instruction.account_struct_name,
        expected_account_struct.map(String::from),
        "Instruction account struct name should match"
    );
}

/// Asserts that an instruction has a specific operation
pub fn assert_has_operation<F>(instruction: &NormalizedInstruction, predicate: F, error_msg: &str)
where
    F: Fn(&BasicOperation) -> bool,
{
    if let Some(body) = &instruction.body {
        // Check if the body contains basic operations
        match body {
            anchor_normalizer::InstructionBody::Basic(ops) => {
                assert!(ops.iter().any(|op| predicate(op)), "{}", error_msg);
            }
            _ => panic!("Expected basic operations for instruction"),
        }
    } else {
        panic!("Instruction has no body");
    }
}

/// Asserts that an account field has a specific constraint
pub fn assert_has_constraint(
    field: &NormalizedAccountField,
    constraint_type: &str,
    expected_value: Option<&str>,
) {
    let found = field
        .constraints
        .iter()
        .any(|c| c.constraint_type == constraint_type && c.value.as_deref() == expected_value);
    assert!(
        found,
        "Field '{}' should have constraint '{}' with value '{:?}'",
        field.name, constraint_type, expected_value
    );
}

/// Asserts that the normalized program has a validation issue containing the specified text
pub fn assert_validation_issue(normalized: &NormalizedProgram, expected_text: &str) {
    let has_issue = normalized.validation_issues.iter().any(|issue| {
        issue
            .message
            .to_lowercase()
            .contains(&expected_text.to_lowercase())
    });

    if !has_issue {
        // Print all validation issues to help with debugging
        println!("All validation issues:");
        for (i, issue) in normalized.validation_issues.iter().enumerate() {
            println!("  {}: {} (element: {})", i, issue.message, issue.element);
        }
    }

    assert!(
        has_issue,
        "Expected validation issue containing '{}' but none found. Issues: {:?}",
        expected_text, normalized.validation_issues
    );
}
