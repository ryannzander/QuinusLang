//! Error types and reporting for Q++

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lexer error at {line}:{col}: {message}")]
    Lexer {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("Parse error at {line}:{col}: {message}")]
    Parse {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("{0}")]
    Semantic(Box<SemanticError>),

    #[error("Codegen error: {message}")]
    Codegen { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Package error: {message}")]
    Package { message: String },
}

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
    pub line: Option<usize>,
    pub col: Option<usize>,
    pub hint: Option<String>,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Semantic error")?;
        if let (Some(l), Some(c)) = (self.line, self.col) {
            write!(f, " at {}:{}", l, c)?;
        }
        write!(f, ": {}", self.message)?;
        if let Some(h) = &self.hint {
            write!(f, "\n  {}", h)?;
        }
        Ok(())
    }
}

pub fn semantic_err(message: impl Into<String>) -> Error {
    Error::Semantic(Box::new(SemanticError {
        message: message.into(),
        line: None,
        col: None,
        hint: None,
    }))
}

pub fn semantic_err_hint(message: impl Into<String>, hint: impl Into<String>) -> Error {
    Error::Semantic(Box::new(SemanticError {
        message: message.into(),
        line: None,
        col: None,
        hint: Some(hint.into()),
    }))
}

pub fn semantic_err_span(message: impl Into<String>, line: usize, col: usize) -> Error {
    Error::Semantic(Box::new(SemanticError {
        message: message.into(),
        line: Some(line),
        col: Some(col),
        hint: None,
    }))
}

pub fn semantic_err_span_hint(
    message: impl Into<String>,
    hint: impl Into<String>,
    line: usize,
    col: usize,
) -> Error {
    Error::Semantic(Box::new(SemanticError {
        message: message.into(),
        line: Some(line),
        col: Some(col),
        hint: Some(hint.into()),
    }))
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<inkwell::builder::BuilderError> for Error {
    fn from(e: inkwell::builder::BuilderError) -> Self {
        semantic_err(e.to_string())
    }
}

impl From<inkwell::support::LLVMString> for Error {
    fn from(e: inkwell::support::LLVMString) -> Self {
        semantic_err(e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        semantic_err(e)
    }
}
