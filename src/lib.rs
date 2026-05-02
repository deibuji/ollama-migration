//! Ollama Model Migration Tool
//!
//! Extracts GGUF model files from Ollama's internal storage format
//! to standard GGUF files usable by vLLM, llama.cpp, and other LLM tools.

pub mod cli;
pub mod error;
pub mod export;
pub mod gguf;
pub mod ollama;
pub mod output;
pub mod paths;
pub mod platform;

pub use error::{MigrationError, Result};

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
