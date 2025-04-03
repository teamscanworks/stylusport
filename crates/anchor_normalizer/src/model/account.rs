//! Account model definitions
//!
//! Defines normalized account structures and related types

use serde::{Deserialize, Serialize};

/// Normalized account structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedAccountStruct {
    /// Account structure name
    pub name: String,

    /// Account structure visibility
    pub visibility: String,

    /// Account fields with their constraints
    pub fields: Vec<NormalizedAccountField>,

    /// Account structure documentation
    pub documentation: Option<String>,
}

/// Normalized account field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedAccountField {
    /// Field name
    pub name: String,

    /// Field type
    pub ty: String,

    /// Normalized constraints
    pub constraints: Vec<NormalizedConstraint>,

    /// Field documentation
    pub documentation: Option<String>,

    /// Inferred semantic information
    pub inferred_info: InferredFieldInfo,
}

/// Normalized constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedConstraint {
    /// Constraint type
    pub constraint_type: String,

    /// Constraint value (if any)
    pub value: Option<String>,

    /// Whether this constraint was inferred (not in source)
    pub is_inferred: bool,
}

/// Inferred semantic information for fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferredFieldInfo {
    /// Whether field must be mutable
    pub requires_mut: bool,

    /// Whether field must be a signer
    pub requires_signer: bool,

    /// Whether field is initialized by this instruction
    pub is_initialized: bool,

    /// Related account (if any)
    pub related_account: Option<String>,
}

/// Normalized raw account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedRawAccount {
    /// Account name
    pub name: String,

    /// Account visibility
    pub visibility: String,

    /// Account fields
    pub fields: Vec<NormalizedRawField>,

    /// Account documentation
    pub documentation: Option<String>,
}

/// Normalized raw account field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedRawField {
    /// Field name
    pub name: String,

    /// Field type
    pub ty: String,

    /// Field visibility
    pub visibility: String,

    /// Field documentation
    pub documentation: Option<String>,
}

impl NormalizedAccountStruct {
    /// Create a new account struct
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            fields: Vec::new(),
            documentation: None,
        }
    }

    /// Add a field to the account struct
    pub fn add_field(&mut self, field: NormalizedAccountField) {
        self.fields.push(field);
    }

    /// Find a field by name
    pub fn find_field(&self, name: &str) -> Option<&NormalizedAccountField> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Set the documentation
    pub fn with_documentation(mut self, docs: impl Into<String>) -> Self {
        self.documentation = Some(docs.into());
        self
    }
}

impl NormalizedAccountField {
    /// Create a new account field
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            constraints: Vec::new(),
            documentation: None,
            inferred_info: InferredFieldInfo {
                requires_mut: false,
                requires_signer: false,
                is_initialized: false,
                related_account: None,
            },
        }
    }

    /// Add a constraint to the field
    pub fn add_constraint(&mut self, constraint: NormalizedConstraint) {
        // Update inferred info based on constraint
        match constraint.constraint_type.as_str() {
            "mut" => self.inferred_info.requires_mut = true,
            "signer" => self.inferred_info.requires_signer = true,
            "init" => self.inferred_info.is_initialized = true,
            "payer" => {
                if let Some(value) = &constraint.value {
                    self.inferred_info.related_account = Some(value.clone());
                }
            }
            _ => {}
        }

        self.constraints.push(constraint);
    }

    /// Find a constraint by type
    pub fn find_constraint(&self, constraint_type: &str) -> Option<&NormalizedConstraint> {
        self.constraints
            .iter()
            .find(|c| c.constraint_type == constraint_type)
    }

    /// Set the documentation
    pub fn with_documentation(mut self, docs: impl Into<String>) -> Self {
        self.documentation = Some(docs.into());
        self
    }
}

impl NormalizedConstraint {
    /// Create a new constraint
    pub fn new(
        constraint_type: impl Into<String>,
        value: Option<impl Into<String>>,
        is_inferred: bool,
    ) -> Self {
        Self {
            constraint_type: constraint_type.into(),
            value: value.map(|v| v.into()),
            is_inferred,
        }
    }

    /// Create a new constraint with no value
    pub fn without_value(constraint_type: impl Into<String>, is_inferred: bool) -> Self {
        Self {
            constraint_type: constraint_type.into(),
            value: None,
            is_inferred,
        }
    }

    /// Create a new constraint with a value
    pub fn with_value(
        constraint_type: impl Into<String>,
        value: impl Into<String>,
        is_inferred: bool,
    ) -> Self {
        Self {
            constraint_type: constraint_type.into(),
            value: Some(value.into()),
            is_inferred,
        }
    }
}

impl InferredFieldInfo {
    /// Create new inferred field info
    pub fn new() -> Self {
        Self {
            requires_mut: false,
            requires_signer: false,
            is_initialized: false,
            related_account: None,
        }
    }
}

impl NormalizedRawAccount {
    /// Create a new raw account
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            fields: Vec::new(),
            documentation: None,
        }
    }

    /// Add a field to the raw account
    pub fn add_field(&mut self, field: NormalizedRawField) {
        self.fields.push(field);
    }

    /// Find a field by name
    pub fn find_field(&self, name: &str) -> Option<&NormalizedRawField> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Set the documentation
    pub fn with_documentation(mut self, docs: impl Into<String>) -> Self {
        self.documentation = Some(docs.into());
        self
    }
}

impl NormalizedRawField {
    /// Create a new raw field
    pub fn new(
        name: impl Into<String>,
        ty: impl Into<String>,
        visibility: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            visibility: visibility.into(),
            documentation: None,
        }
    }

    /// Set the documentation
    pub fn with_documentation(mut self, docs: impl Into<String>) -> Self {
        self.documentation = Some(docs.into());
        self
    }
}
