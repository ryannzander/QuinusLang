//! Error types and reporting for QuinusLang

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lexer error at {line}:{col}: {message}")]
    Lexer { line: usize, col: usize, message: String },

    #[error("Parse error at {line}:{col}: {message}")]
    Parse { line: usize, col: usize, message: String },

    #[error("Semantic error: {message}")]
    Semantic { message: String },

    #[error("Codegen error: {message}")]
    Codegen { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Package error: {message}")]
    Package { message: String },
}

pub type Result<T> = std::result::Result<T, Error>;
