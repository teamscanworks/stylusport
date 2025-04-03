//! Instruction model definitions
//!
//! Defines normalized instruction structures and related types

use serde::{Deserialize, Serialize};

/// Normalized representation of an instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedInstruction {
    /// Instruction name
    pub name: String,

    /// Instruction visibility
    pub visibility: String,

    /// Parameter specifications
    pub parameters: Vec<NormalizedParameter>,

    /// Return type (if any)
    pub return_type: Option<String>,

    /// Associated account structure (by name)
    pub account_struct_name: Option<String>,

    /// Semantic model of the instruction body (if available)
    pub body: Option<InstructionBody>,

    /// Instruction-level documentation
    pub documentation: Option<String>,
}

/// Normalized parameter for an instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedParameter {
    /// Parameter name
    pub name: String,

    /// Parameter type
    pub ty: String,

    /// Whether this is a Context parameter
    pub is_context: bool,
}

/// Placeholder for instruction body semantics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstructionBody {
    /// Unknown implementation - will be completed when parser is enhanced
    Unknown,

    /// Basic operations inferred from context
    Basic(Vec<BasicOperation>),
}

/// Basic operation types that might be inferred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BasicOperation {
    /// Logs a message
    Log(String),

    /// Creates a new account
    Initialize { target: String, payer: String },

    /// Transfers funds between accounts
    Transfer { from: String, to: String },

    /// Closes an account
    Close { target: String, refund_to: String },
}

impl NormalizedInstruction {
    /// Create a new instruction
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            parameters: Vec::new(),
            return_type: None,
            account_struct_name: None,
            body: Some(InstructionBody::Unknown),
            documentation: None,
        }
    }

    /// Add a parameter to the instruction
    pub fn add_parameter(&mut self, parameter: NormalizedParameter) {
        self.parameters.push(parameter);
    }

    /// Set the return type
    pub fn with_return_type(mut self, ty: impl Into<String>) -> Self {
        self.return_type = Some(ty.into());
        self
    }

    /// Set the account struct name
    pub fn with_account_struct(mut self, account_struct: impl Into<String>) -> Self {
        self.account_struct_name = Some(account_struct.into());
        self
    }

    /// Set the body
    pub fn with_body(mut self, body: InstructionBody) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the documentation
    pub fn with_documentation(mut self, docs: impl Into<String>) -> Self {
        self.documentation = Some(docs.into());
        self
    }

    /// Check if this is a Context parameter
    pub fn has_context_parameter(&self) -> bool {
        self.parameters.iter().any(|p| p.is_context)
    }

    /// Get the context parameter
    pub fn get_context_parameter(&self) -> Option<&NormalizedParameter> {
        self.parameters.iter().find(|p| p.is_context)
    }
}

impl NormalizedParameter {
    /// Create a new parameter
    pub fn new(name: impl Into<String>, ty: impl Into<String>, is_context: bool) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            is_context,
        }
    }

    /// Create a new context parameter
    pub fn new_context(name: impl Into<String>, context_type: impl Into<String>) -> Self {
        let context_type = context_type.into();
        Self {
            name: name.into(),
            ty: format!("Context<{}>", context_type),
            is_context: true,
        }
    }
}
