//! Account models for Anchor account structures
//!
//! This module defines structures representing Anchor accounts, including both
//! account validation structures (#[derive(Accounts)]) and raw account structures (#[account]).

/// Represents an account structure with #[derive(Accounts)]
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Account {
    /// Name of the account struct
    pub name: String,

    /// Visibility of the struct
    pub visibility: String,

    /// Fields in the account struct
    pub fields: Vec<AccountField>,
}

/// Represents a field in an account structure
#[derive(Debug, Clone, Default, Serialize)]
pub struct AccountField {
    /// Name of the field
    pub name: String,

    /// Type of the field
    pub ty: String,

    /// Constraints on the field (from #[account(...)])
    pub constraints: Vec<Constraint>,
}

/// Represents a constraint on an account field
#[derive(Debug, Clone, Serialize)]
pub struct Constraint {
    /// Type of constraint (init, payer, seeds, etc.)
    pub constraint_type: String,

    /// Value of the constraint (if any)
    pub value: Option<String>,
}

/// Represents a raw account with #[account]
#[derive(Debug, Clone, Default, Serialize)]
pub struct RawAccount {
    /// Name of the account struct
    pub name: String,

    /// Visibility of the struct
    pub visibility: String,

    /// Fields in the account struct
    pub fields: Vec<RawAccountField>,
}

/// Represents a field in a raw account
#[derive(Debug, Clone, Default, Serialize)]
pub struct RawAccountField {
    /// Name of the field
    pub name: String,

    /// Type of the field
    pub ty: String,

    /// Visibility of the field
    pub visibility: String,
}

impl Account {
    /// Create a new account struct with the given name and visibility
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            fields: Vec::new(),
        }
    }

    /// Add a field to the account struct
    pub fn add_field(&mut self, field: AccountField) {
        self.fields.push(field);
    }

    /// Find a field by name
    pub fn find_field(&self, name: &str) -> Option<&AccountField> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Builder method: add a field and return self
    pub fn with_field(mut self, field: AccountField) -> Self {
        self.add_field(field);
        self
    }

    /// Builder method: set multiple fields at once
    pub fn with_fields(mut self, fields: Vec<AccountField>) -> Self {
        self.fields = fields;
        self
    }
}

impl AccountField {
    /// Create a new account field
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            constraints: Vec::new(),
        }
    }

    /// Add a constraint to the field
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Find a constraint by type
    pub fn find_constraint(&self, constraint_type: &str) -> Option<&Constraint> {
        self.constraints
            .iter()
            .find(|c| c.constraint_type == constraint_type)
    }

    /// Builder method: add a constraint and return self
    pub fn with_constraint(mut self, constraint: Constraint) -> Self {
        self.add_constraint(constraint);
        self
    }

    /// Builder method: set multiple constraints at once
    pub fn with_constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }
}

impl Constraint {
    /// Create a new constraint
    pub fn new(constraint_type: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        Self {
            constraint_type: constraint_type.into(),
            value: value.map(|v| v.into()),
        }
    }

    /// Create a new constraint with a value
    pub fn with_value(constraint_type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            constraint_type: constraint_type.into(),
            value: Some(value.into()),
        }
    }

    /// Create a new constraint without a value
    pub fn without_value(constraint_type: impl Into<String>) -> Self {
        Self {
            constraint_type: constraint_type.into(),
            value: None,
        }
    }
}

impl RawAccount {
    /// Create a new raw account with the given name and visibility
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            fields: Vec::new(),
        }
    }

    /// Add a field to the raw account
    pub fn add_field(&mut self, field: RawAccountField) {
        self.fields.push(field);
    }

    /// Find a field by name
    pub fn find_field(&self, name: &str) -> Option<&RawAccountField> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Builder method: add a field and return self
    pub fn with_field(mut self, field: RawAccountField) -> Self {
        self.add_field(field);
        self
    }

    /// Builder method: set multiple fields at once
    pub fn with_fields(mut self, fields: Vec<RawAccountField>) -> Self {
        self.fields = fields;
        self
    }
}

impl RawAccountField {
    /// Create a new raw account field
    pub fn new(
        name: impl Into<String>,
        ty: impl Into<String>,
        visibility: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            visibility: visibility.into(),
        }
    }
}

#[cfg(all(test, feature = "unit_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_account_new() {
        let account = Account::new("MyAccount", "pub");
        assert_eq!(account.name, "MyAccount");
        assert_eq!(account.visibility, "pub");
        assert!(account.fields.is_empty());
    }

    #[test]
    fn test_account_add_field() {
        let mut account = Account::new("MyAccount", "pub");
        let field = AccountField::new("owner", "Pubkey");
        account.add_field(field);

        assert_eq!(account.fields.len(), 1);
        assert_eq!(account.fields[0].name, "owner");
        assert_eq!(account.fields[0].ty, "Pubkey");
    }

    #[test]
    fn test_account_find_field() {
        let mut account = Account::new("MyAccount", "pub");
        account.add_field(AccountField::new("owner", "Pubkey"));
        account.add_field(AccountField::new("amount", "u64"));

        let field = account.find_field("owner");
        assert!(field.is_some());
        assert_eq!(field.unwrap().ty, "Pubkey");

        let field = account.find_field("unknown");
        assert!(field.is_none());
    }

    #[test]
    fn test_account_builder_methods() {
        let field1 = AccountField::new("owner", "Pubkey");
        let field2 = AccountField::new("amount", "u64");

        // Test with_field
        let account = Account::new("MyAccount", "pub")
            .with_field(field1.clone())
            .with_field(field2.clone());

        assert_eq!(account.fields.len(), 2);
        assert_eq!(account.fields[0].name, "owner");
        assert_eq!(account.fields[1].name, "amount");

        // Test with_fields
        let account = Account::new("MyAccount", "pub").with_fields(vec![field1, field2]);

        assert_eq!(account.fields.len(), 2);
        assert_eq!(account.fields[0].name, "owner");
        assert_eq!(account.fields[1].name, "amount");
    }

    #[test]
    fn test_account_field_new() {
        let field = AccountField::new("owner", "Pubkey");
        assert_eq!(field.name, "owner");
        assert_eq!(field.ty, "Pubkey");
        assert!(field.constraints.is_empty());
    }

    #[test]
    fn test_account_field_add_constraint() {
        let mut field = AccountField::new("owner", "Pubkey");
        let constraint = Constraint::new("signer", None::<String>);
        field.add_constraint(constraint);

        assert_eq!(field.constraints.len(), 1);
        assert_eq!(field.constraints[0].constraint_type, "signer");
        assert!(field.constraints[0].value.is_none());
    }

    #[test]
    fn test_account_field_find_constraint() {
        let mut field = AccountField::new("owner", "Pubkey");
        field.add_constraint(Constraint::new("signer", None::<String>));
        field.add_constraint(Constraint::new("init", Some("true")));

        let constraint = field.find_constraint("signer");
        assert!(constraint.is_some());

        let constraint = field.find_constraint("unknown");
        assert!(constraint.is_none());
    }

    #[test]
    fn test_account_field_builder_methods() {
        let constraint1 = Constraint::new("signer", None::<String>);
        let constraint2 = Constraint::new("init", Some("true"));

        // Test with_constraint
        let field = AccountField::new("owner", "Pubkey")
            .with_constraint(constraint1.clone())
            .with_constraint(constraint2.clone());

        assert_eq!(field.constraints.len(), 2);
        assert_eq!(field.constraints[0].constraint_type, "signer");
        assert_eq!(field.constraints[1].constraint_type, "init");

        // Test with_constraints
        let field =
            AccountField::new("owner", "Pubkey").with_constraints(vec![constraint1, constraint2]);

        assert_eq!(field.constraints.len(), 2);
        assert_eq!(field.constraints[0].constraint_type, "signer");
        assert_eq!(field.constraints[1].constraint_type, "init");
    }

    #[test]
    fn test_constraint_new() {
        // Without value
        let constraint = Constraint::new("signer", None::<String>);
        assert_eq!(constraint.constraint_type, "signer");
        assert!(constraint.value.is_none());

        // With value
        let constraint = Constraint::new("init", Some("true"));
        assert_eq!(constraint.constraint_type, "init");
        assert_eq!(constraint.value, Some("true".to_string()));
    }

    #[test]
    fn test_constraint_with_value() {
        let constraint = Constraint::with_value("init", "true");
        assert_eq!(constraint.constraint_type, "init");
        assert_eq!(constraint.value, Some("true".to_string()));
    }

    #[test]
    fn test_constraint_without_value() {
        let constraint = Constraint::without_value("signer");
        assert_eq!(constraint.constraint_type, "signer");
        assert!(constraint.value.is_none());
    }

    #[test]
    fn test_raw_account_new() {
        let account = RawAccount::new("MyAccount", "pub");
        assert_eq!(account.name, "MyAccount");
        assert_eq!(account.visibility, "pub");
        assert!(account.fields.is_empty());
    }

    #[test]
    fn test_raw_account_add_field() {
        let mut account = RawAccount::new("MyAccount", "pub");
        let field = RawAccountField::new("owner", "Pubkey", "pub");
        account.add_field(field);

        assert_eq!(account.fields.len(), 1);
        assert_eq!(account.fields[0].name, "owner");
        assert_eq!(account.fields[0].ty, "Pubkey");
        assert_eq!(account.fields[0].visibility, "pub");
    }

    #[test]
    fn test_raw_account_find_field() {
        let mut account = RawAccount::new("MyAccount", "pub");
        account.add_field(RawAccountField::new("owner", "Pubkey", "pub"));
        account.add_field(RawAccountField::new("amount", "u64", ""));

        let field = account.find_field("owner");
        assert!(field.is_some());
        assert_eq!(field.unwrap().ty, "Pubkey");

        let field = account.find_field("unknown");
        assert!(field.is_none());
    }

    #[test]
    fn test_raw_account_builder_methods() {
        let field1 = RawAccountField::new("owner", "Pubkey", "pub");
        let field2 = RawAccountField::new("amount", "u64", "");

        // Test with_field
        let account = RawAccount::new("MyAccount", "pub")
            .with_field(field1.clone())
            .with_field(field2.clone());

        assert_eq!(account.fields.len(), 2);
        assert_eq!(account.fields[0].name, "owner");
        assert_eq!(account.fields[1].name, "amount");

        // Test with_fields
        let account = RawAccount::new("MyAccount", "pub").with_fields(vec![field1, field2]);

        assert_eq!(account.fields.len(), 2);
        assert_eq!(account.fields[0].name, "owner");
        assert_eq!(account.fields[1].name, "amount");
    }

    #[test]
    fn test_raw_account_field_new() {
        let field = RawAccountField::new("owner", "Pubkey", "pub");
        assert_eq!(field.name, "owner");
        assert_eq!(field.ty, "Pubkey");
        assert_eq!(field.visibility, "pub");
    }

    #[test]
    fn test_string_conversions() {
        // Test flexibility in Account::new
        let account1 = Account::new("static", "pub");
        let account2 = Account::new(String::from("owned"), "pub");
        let name = String::from("reference");
        let account3 = Account::new(&name, "pub");

        assert_eq!(account1.name, "static");
        assert_eq!(account2.name, "owned");
        assert_eq!(account3.name, "reference");
    }
}
