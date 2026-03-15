//! Package manager for QuinusLang

pub mod fetch;
pub mod lockfile;
pub mod manifest;
pub mod resolve;

pub use manifest::Manifest;
pub use resolve::resolve_dependencies;
