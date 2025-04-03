//! Account normalization logic
//!
//! Handles normalization of Anchor account structures

use crate::error::Result;
use crate::model::account::{
    NormalizedAccountField, NormalizedAccountStruct, NormalizedConstraint, NormalizedRawAccount,
    NormalizedRawField,
};
use anchor_parser::model::account::{
    Account, AccountField, Constraint, RawAccount, RawAccountField,
};

/// Normalize an Anchor account struct
///
/// Transforms a parsed account struct into a normalized form with enhanced semantics.
///
/// # Arguments
///
/// * `account` - The parsed account struct
///
/// # Returns
///
/// A normalized account struct or an error if normalization fails
pub fn normalize_account_struct(account: &Account) -> Result<NormalizedAccountStruct> {
    let mut normalized =
        NormalizedAccountStruct::new(account.name.clone(), account.visibility.clone());

    // Normalize fields
    for field in &account.fields {
        normalized.add_field(normalize_account_field(field)?);
    }

    Ok(normalized)
}

/// Normalize an account field
fn normalize_account_field(field: &AccountField) -> Result<NormalizedAccountField> {
    let mut normalized = NormalizedAccountField::new(field.name.clone(), field.ty.clone());

    // Normalize constraints
    for constraint in &field.constraints {
        normalized.add_constraint(normalize_constraint(constraint)?);
    }

    Ok(normalized)
}

/// Normalize a constraint
fn normalize_constraint(constraint: &Constraint) -> Result<NormalizedConstraint> {
    Ok(NormalizedConstraint::new(
        constraint.constraint_type.clone(),
        constraint.value.clone(),
        false, // Not inferred
    ))
}

/// Normalize a raw account
pub fn normalize_raw_account(account: &RawAccount) -> Result<NormalizedRawAccount> {
    let mut normalized =
        NormalizedRawAccount::new(account.name.clone(), account.visibility.clone()); // Normalize fields
    for field in &account.fields {
        normalized.add_field(normalize_raw_field(field)?);
    }

    Ok(normalized)
}

/// Normalize a raw account field
fn normalize_raw_field(field: &RawAccountField) -> Result<NormalizedRawField> {
    Ok(NormalizedRawField::new(
        field.name.clone(),
        field.ty.clone(),
        field.visibility.clone(),
    ))
}
