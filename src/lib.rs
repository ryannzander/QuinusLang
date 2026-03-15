//! QuinusLang compiler library

pub mod ast;
pub mod codegen;
pub mod error;
pub mod fmt;
pub mod lexer;
pub mod package;
pub mod parser;
pub mod semantic;

pub use error::{Error, Result};
pub use lexer::tokenize;
pub use parser::{parse, parse_from_stream, parse_with_imports};
pub use semantic::analyze;
