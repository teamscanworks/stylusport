//! Error types for the anchor_parser crate

use std::fmt;

/// Errors that can occur during parsing
#[derive(Debug)]
pub enum ParseError {
    /// I/O error
    Io(std::io::Error),

    /// Syntax error
    Syntax(syn::Error),

    /// Other parse error
    Parse(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(err) => write!(f, "I/O error: {}", err),
            ParseError::Syntax(err) => write!(f, "Syntax error: {}", err),
            ParseError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::Io(err) => Some(err),
            ParseError::Syntax(err) => Some(err),
            ParseError::Parse(_) => None,
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err)
    }
}

impl From<syn::Error> for ParseError {
    fn from(err: syn::Error) -> Self {
        ParseError::Syntax(err)
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
