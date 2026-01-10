use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextFsmError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("State error: {0}")]
    StateError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, TextFsmError>;
