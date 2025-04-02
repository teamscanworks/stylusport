//! Domain model for Anchor programs
//!
//! This module defines the core data structures used to represent Anchor programs,
//! including programs, instructions, and account structures.

// Declare submodules
pub mod account;
pub mod instruction;
pub mod program;

// Re-export all types from submodules for easier access
pub use account::{Account, AccountField, Constraint, RawAccount, RawAccountField};
pub use instruction::{Instruction, Parameter};
pub use program::{Program, ProgramModule};

#[cfg(all(test, feature = "unit_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_model_integration() {
        // Create a program
        let mut program = Program::new();
        
        // Add a program module
        let mut module = ProgramModule::new("MyProgram", "pub");
        
        // Create an instruction
        let mut instruction = Instruction::new("initialize", "pub");
        
        // Add a context parameter
        instruction.add_parameter(Parameter::new_context("ctx", "Initialize"));
        
        // Add a regular parameter
        instruction.add_parameter(Parameter::new("amount", "u64", false));
        
        // Set return type
        instruction.set_return_type("Result<()>");
        instruction.set_context_type("Initialize");
        
        // Add instruction to module
        module.add_instruction(instruction);
        
        // Add module to program
        program.add_program_module(module);
        
        // Create an account struct
        let mut account = Account::new("Initialize", "pub");
        
        // Create account fields
        let mut owner_field = AccountField::new("owner", "Pubkey");
        owner_field.add_constraint(Constraint::without_value("signer"));
        
        let mut vault_field = AccountField::new("vault", "Account<'info, TokenAccount>");
        vault_field.add_constraint(Constraint::with_value("init", "true"));
        vault_field.add_constraint(Constraint::with_value("payer", "owner"));
        
        // Add fields to account
        account.add_field(owner_field);
        account.add_field(vault_field);
        
        // Add account to program
        program.add_account_struct(account);
        
        // Create a raw account
        let mut raw_account = RawAccount::new("VaultState", "pub");
        
        // Add fields to raw account
        raw_account.add_field(RawAccountField::new("owner", "Pubkey", "pub"));
        raw_account.add_field(RawAccountField::new("amount", "u64", "pub"));
        
        // Add raw account to program
        program.add_raw_account(raw_account);
        
        // Verify program structure
        assert_eq!(program.program_modules.len(), 1);
        assert_eq!(program.account_structs.len(), 1);
        assert_eq!(program.raw_accounts.len(), 1);
    }
}