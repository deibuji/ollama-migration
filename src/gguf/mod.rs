//! GGUF file format handling module.

pub mod header;
pub mod metadata;
pub mod validator;

pub use header::GGUFHeader;
pub use metadata::GGUFMetadata;
pub use validator::{validate_gguf, ValidationResult};
