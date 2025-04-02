//! Program model for Anchor programs
//!
//! This module defines the core structures that represent an Anchor program,
//! including program modules, instructions, and account structures.

use serde::Serialize;

use crate::model::account::{Account, RawAccount};
use crate::model::instruction::Instruction;

/// Represents a program module with the #[program] attribute
///
/// In Anchor, a module marked with #[program] contains instruction handlers
/// that define the behavior of the Solana program.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ProgramModule {
    /// Name of the program module
    pub name: String,
    
    /// Visibility of the program module ("", "pub", "pub(crate)", etc.)
    pub visibility: String,
    
    /// Instructions defined in the program
    pub instructions: Vec<Instruction>,
}

/// Represents a complete Anchor program
///
/// A program contains program modules, account structures, and raw accounts.
/// It is the root object for representing an Anchor program's structure.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Program {
    /// Program modules (with #[program] attribute)
    pub program_modules: Vec<ProgramModule>,
    
    /// Account structs (with #[derive(Accounts)])
    pub account_structs: Vec<Account>,
    
    /// Raw account structs (with #[account])
    pub raw_accounts: Vec<RawAccount>,
    
    /// Source file path (if available)
    pub source_path: Option<String>,
}

impl Program {
    /// Create a new empty program
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a program module to the program
    pub fn add_program_module(&mut self, module: ProgramModule) {
        self.program_modules.push(module);
    }
    
    /// Add an account struct to the program
    pub fn add_account_struct(&mut self, account: Account) {
        self.account_structs.push(account);
    }
    
    /// Add a raw account to the program
    pub fn add_raw_account(&mut self, account: RawAccount) {
        self.raw_accounts.push(account);
    }
    
    /// Find a program module by name
    pub fn find_program_module(&self, name: &str) -> Option<&ProgramModule> {
        self.program_modules.iter().find(|m| m.name == name)
    }
    
    /// Find an account struct by name
    pub fn find_account_struct(&self, name: &str) -> Option<&Account> {
        self.account_structs.iter().find(|a| a.name == name)
    }
    
    /// Find a raw account by name
    pub fn find_raw_account(&self, name: &str) -> Option<&RawAccount> {
        self.raw_accounts.iter().find(|a| a.name == name)
    }

    /// Set the source path (builder pattern)
    pub fn with_source_path(mut self, path: impl Into<String>) -> Self {
        self.source_path = Some(path.into());
        self
    }
    
    /// Add a program module (builder pattern)
    pub fn with_program_module(mut self, module: ProgramModule) -> Self {
        self.add_program_module(module);
        self
    }
    
    /// Add an account struct (builder pattern)
    pub fn with_account_struct(mut self, account: Account) -> Self {
        self.add_account_struct(account);
        self
    }
    
    /// Add a raw account (builder pattern)
    pub fn with_raw_account(mut self, account: RawAccount) -> Self {
        self.add_raw_account(account);
        self
    }
}

impl ProgramModule {
    /// Create a new program module
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            instructions: Vec::new(),
        }
    }
    
    /// Add an instruction to the program module
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
    
    /// Find an instruction by name
    pub fn find_instruction(&self, name: &str) -> Option<&Instruction> {
        self.instructions.iter().find(|i| i.name == name)
    }
    
    /// Set instructions (builder pattern)
    pub fn with_instructions(mut self, instructions: Vec<Instruction>) -> Self {
        self.instructions = instructions;
        self
    }
    
    /// Add an instruction (builder pattern)
    pub fn with_instruction(mut self, instruction: Instruction) -> Self {
        self.add_instruction(instruction);
        self
    }
}

#[cfg(all(test, feature = "unit_test"))]
mod tests {
    use super::*;
    use crate::model::account::{Account, RawAccount};
    use crate::model::instruction::Instruction;

    #[test]
    fn test_program_default() {
        let program = Program::default();
        assert!(program.program_modules.is_empty());
        assert!(program.account_structs.is_empty());
        assert!(program.raw_accounts.is_empty());
        assert!(program.source_path.is_none());
    }

    #[test]
    fn test_program_new() {
        let program = Program::new();
        assert!(program.program_modules.is_empty());
        assert!(program.account_structs.is_empty());
        assert!(program.raw_accounts.is_empty());
        assert!(program.source_path.is_none());
    }

    #[test]
    fn test_program_add_components() {
        let mut program = Program::new();
        
        // Add program module
        let module = ProgramModule::new("MyModule", "pub");
        program.add_program_module(module);
        assert_eq!(program.program_modules.len(), 1);
        assert_eq!(program.program_modules[0].name, "MyModule");
        
        // Add account struct
        let account = Account::new("MyAccount", "pub");
        program.add_account_struct(account);
        assert_eq!(program.account_structs.len(), 1);
        assert_eq!(program.account_structs[0].name, "MyAccount");
        
        // Add raw account
        let raw_account = RawAccount::new("MyRawAccount", "pub");
        program.add_raw_account(raw_account);
        assert_eq!(program.raw_accounts.len(), 1);
        assert_eq!(program.raw_accounts[0].name, "MyRawAccount");
    }

    #[test]
    fn test_program_find_methods() {
        let mut program = Program::new();
        
        // Add program modules
        program.add_program_module(ProgramModule::new("Module1", "pub"));
        program.add_program_module(ProgramModule::new("Module2", ""));
        
        // Add account structs
        program.add_account_struct(Account::new("Account1", "pub"));
        program.add_account_struct(Account::new("Account2", ""));
        
        // Add raw accounts
        program.add_raw_account(RawAccount::new("Raw1", "pub"));
        program.add_raw_account(RawAccount::new("Raw2", ""));
        
        // Test find methods
        let found_module = program.find_program_module("Module1");
        assert!(found_module.is_some());
        assert_eq!(found_module.unwrap().name, "Module1");
        
        let found_module = program.find_program_module("ModuleX");
        assert!(found_module.is_none());
        
        let found_account = program.find_account_struct("Account1");
        assert!(found_account.is_some());
        assert_eq!(found_account.unwrap().name, "Account1");
        
        let found_account = program.find_account_struct("AccountX");
        assert!(found_account.is_none());
        
        let found_raw_account = program.find_raw_account("Raw1");
        assert!(found_raw_account.is_some());
        assert_eq!(found_raw_account.unwrap().name, "Raw1");
        
        let found_raw_account = program.find_raw_account("RawX");
        assert!(found_raw_account.is_none());
    }

    #[test]
    fn test_program_builder_methods() {
        // Test individual builder methods
        let program = Program::new()
            .with_source_path("path/to/file.rs");
        assert_eq!(program.source_path, Some("path/to/file.rs".to_string()));
        
        let program = Program::new()
            .with_program_module(ProgramModule::new("Module1", "pub"));
        assert_eq!(program.program_modules.len(), 1);
        assert_eq!(program.program_modules[0].name, "Module1");
        
        let program = Program::new()
            .with_account_struct(Account::new("Account1", "pub"));
        assert_eq!(program.account_structs.len(), 1);
        assert_eq!(program.account_structs[0].name, "Account1");
        
        let program = Program::new()
            .with_raw_account(RawAccount::new("Raw1", "pub"));
        assert_eq!(program.raw_accounts.len(), 1);
        assert_eq!(program.raw_accounts[0].name, "Raw1");
        
        // Test chained builder methods
        let program = Program::new()
            .with_program_module(ProgramModule::new("Module1", "pub"))
            .with_account_struct(Account::new("Account1", "pub"))
            .with_raw_account(RawAccount::new("Raw1", "pub"))
            .with_source_path("path/to/file.rs");
            
        assert_eq!(program.program_modules.len(), 1);
        assert_eq!(program.program_modules[0].name, "Module1");
        assert_eq!(program.account_structs.len(), 1);
        assert_eq!(program.account_structs[0].name, "Account1");
        assert_eq!(program.raw_accounts.len(), 1);
        assert_eq!(program.raw_accounts[0].name, "Raw1");
        assert_eq!(program.source_path, Some("path/to/file.rs".to_string()));
    }

    #[test]
    fn test_program_module_new() {
        // Test with String
        let module = ProgramModule::new(String::from("MyModule"), String::from("pub"));
        assert_eq!(module.name, "MyModule");
        assert_eq!(module.visibility, "pub");
        
        // Test with &str
        let module = ProgramModule::new("MyModule", "pub");
        assert_eq!(module.name, "MyModule");
        assert_eq!(module.visibility, "pub");
        
        // Test with mixed types
        let name = String::from("MyModule");
        let module = ProgramModule::new(&name, "pub");
        assert_eq!(module.name, "MyModule");
        assert_eq!(module.visibility, "pub");
    }

    #[test]
    fn test_program_module_add_instruction() {
        let mut module = ProgramModule::new("MyModule", "pub");
        let instruction = Instruction::new("initialize", "pub");
        module.add_instruction(instruction);
        assert_eq!(module.instructions.len(), 1);
        assert_eq!(module.instructions[0].name, "initialize");
    }
    
    #[test]
    fn test_program_module_find_instruction() {
        let mut module = ProgramModule::new("MyModule", "pub");
        module.add_instruction(Instruction::new("initialize", "pub"));
        module.add_instruction(Instruction::new("update", "pub"));
        
        let found = module.find_instruction("initialize");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "initialize");
        
        let found = module.find_instruction("unknown");
        assert!(found.is_none());
    }
    
    #[test]
    fn test_program_module_builder_methods() {
        // Test with_instructions
        let instructions = vec![
            Instruction::new("instr1", "pub"),
            Instruction::new("instr2", "pub"),
        ];
        
        let module = ProgramModule::new("MyModule", "pub")
            .with_instructions(instructions);
            
        assert_eq!(module.instructions.len(), 2);
        assert_eq!(module.instructions[0].name, "instr1");
        assert_eq!(module.instructions[1].name, "instr2");
        
        // Test with_instruction
        let module = ProgramModule::new("MyModule", "pub")
            .with_instruction(Instruction::new("instr1", "pub"))
            .with_instruction(Instruction::new("instr2", "pub"));
            
        assert_eq!(module.instructions.len(), 2);
        assert_eq!(module.instructions[0].name, "instr1");
        assert_eq!(module.instructions[1].name, "instr2");
    }

    #[test]
    fn test_string_conversion_flexibility() {
        // Test various string types for Program::with_source_path
        let program1 = Program::new().with_source_path("static string");
        let program2 = Program::new().with_source_path(String::from("owned string"));
        let path = String::from("reference to owned");
        let program3 = Program::new().with_source_path(&path);
        
        assert_eq!(program1.source_path, Some("static string".to_string()));
        assert_eq!(program2.source_path, Some("owned string".to_string()));
        assert_eq!(program3.source_path, Some("reference to owned".to_string()));
    }
}