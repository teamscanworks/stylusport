//! Validation model definitions
//!
//! Defines types for validation issues and related concerns

use serde::{Deserialize, Serialize};

/// Validation issue found during normalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Severity level
    pub severity: IssueSeverity,

    /// Issue message
    pub message: String,

    /// Related element
    pub element: String,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Informational message
    Info,

    /// Warning (doesn't prevent IR generation)
    Warning,

    /// Error (may prevent correct IR generation)
    Error,
}

impl ValidationIssue {
    /// Create a new validation issue
    pub fn new(
        severity: IssueSeverity,
        message: impl Into<String>,
        element: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            message: message.into(),
            element: element.into(),
        }
    }

    /// Create a new info issue
    pub fn info(message: impl Into<String>, element: impl Into<String>) -> Self {
        Self::new(IssueSeverity::Info, message, element)
    }

    /// Create a new warning issue
    pub fn warning(message: impl Into<String>, element: impl Into<String>) -> Self {
        Self::new(IssueSeverity::Warning, message, element)
    }

    /// Create a new error issue
    pub fn error(message: impl Into<String>, element: impl Into<String>) -> Self {
        Self::new(IssueSeverity::Error, message, element)
    }
}
