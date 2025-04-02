use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parse(#[from] anchor_parser::ParseError),
    
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("Missing required argument: {0}")]
    MissingArgument(String),
    
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

// Implement conversions from other error types as needed
impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}