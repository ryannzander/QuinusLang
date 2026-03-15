//! QuinusLang compiler library

pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod package;
pub mod parser;
pub mod semantic;

pub use error::{Error, Result};
pub use lexer::tokenize;
pub use parser::{parse, parse_from_stream};
pub use semantic::analyze;
