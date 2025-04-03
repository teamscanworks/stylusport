use thiserror::Error;

/// Errors that can occur during Anchor program normalization
#[derive(Error, Debug)]
pub enum NormalizationError {
    /// Error extracting information from AST
    #[error("AST extraction error: {0}")]
    AstExtraction(String),

    /// Error validating program structure
    #[error("Validation error: {0}")]
    Validation(String),

    /// Error during semantic inference
    #[error("Inference error: {0}")]
    Inference(String),

    /// Missing required information
    #[error("Missing information: {0}")]
    MissingInfo(String),

    /// Other error
    #[error("Normalization error: {0}")]
    Other(String),
}

/// Result type for normalization operations
pub type Result<T> = std::result::Result<T, NormalizationError>;
