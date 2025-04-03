//! Core program model definitions
//!
//! Defines the top-level normalized program structure

use serde::{Deserialize, Serialize};

use crate::model::{
    account::{NormalizedAccountStruct, NormalizedRawAccount},
    instruction::NormalizedInstruction,
    validation::ValidationIssue,
};

/// Normalized representation of an Anchor program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedProgram {
    /// Unique identifier for the program
    pub id: String,

    /// Program name(s) - the main module name
    pub name: String,

    /// Program modules with their instructions
    pub modules: Vec<NormalizedModule>,

    /// Account structures used by the program
    pub account_structs: Vec<NormalizedAccountStruct>,

    /// Raw account definitions
    pub raw_accounts: Vec<NormalizedRawAccount>,

    /// Program-level documentation extracted from comments
    pub documentation: Option<String>,

    /// Validation issues found during normalization
    pub validation_issues: Vec<ValidationIssue>,

    /// Source information (if available)
    pub source_info: Option<SourceInfo>,

    /// Schema version for future compatibility
    pub schema_version: String,
}

/// Normalized representation of a program module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedModule {
    /// Module name
    pub name: String,

    /// Module visibility
    pub visibility: String,

    /// Instructions defined in this module
    pub instructions: Vec<NormalizedInstruction>,

    /// Module-level documentation
    pub documentation: Option<String>,
}

/// Source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Source file path
    pub file_path: String,

    /// Line range in source
    pub line_range: Option<(usize, usize)>,
}

impl NormalizedProgram {
    /// Create a new, empty normalized program
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            modules: Vec::new(),
            account_structs: Vec::new(),
            raw_accounts: Vec::new(),
            documentation: None,
            validation_issues: Vec::new(),
            source_info: None,
            schema_version: "1.0".to_string(),
        }
    }

    /// Find an account struct by name
    pub fn find_account_struct(&self, name: &str) -> Option<&NormalizedAccountStruct> {
        self.account_structs.iter().find(|a| a.name == name)
    }

    /// Find a raw account by name
    pub fn find_raw_account(&self, name: &str) -> Option<&NormalizedRawAccount> {
        self.raw_accounts.iter().find(|a| a.name == name)
    }

    /// Find an instruction by name (searches all modules)
    pub fn find_instruction(&self, name: &str) -> Option<&NormalizedInstruction> {
        for module in &self.modules {
            if let Some(instr) = module.instructions.iter().find(|i| i.name == name) {
                return Some(instr);
            }
        }
        None
    }

    /// Add a validation issue
    pub fn add_validation_issue(&mut self, issue: ValidationIssue) {
        self.validation_issues.push(issue);
    }

    /// Set the source information
    pub fn with_source_info(mut self, source_info: SourceInfo) -> Self {
        self.source_info = Some(source_info);
        self
    }

    /// Add a module to the program
    pub fn add_module(&mut self, module: NormalizedModule) {
        self.modules.push(module);
    }

    /// Add an account struct to the program
    pub fn add_account_struct(&mut self, account: NormalizedAccountStruct) {
        self.account_structs.push(account);
    }

    /// Add a raw account to the program
    pub fn add_raw_account(&mut self, account: NormalizedRawAccount) {
        self.raw_accounts.push(account);
    }
}

impl NormalizedModule {
    /// Create a new module
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            instructions: Vec::new(),
            documentation: None,
        }
    }

    /// Add an instruction to the module
    pub fn add_instruction(&mut self, instruction: NormalizedInstruction) {
        self.instructions.push(instruction);
    }

    /// Find an instruction by name
    pub fn find_instruction(&self, name: &str) -> Option<&NormalizedInstruction> {
        self.instructions.iter().find(|i| i.name == name)
    }

    /// Set the documentation
    pub fn with_documentation(mut self, docs: impl Into<String>) -> Self {
        self.documentation = Some(docs.into());
        self
    }
}

impl SourceInfo {
    /// Create new source information
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            line_range: None,
        }
    }

    /// Set the line range
    pub fn with_line_range(mut self, start: usize, end: usize) -> Self {
        self.line_range = Some((start, end));
        self
    }
}
