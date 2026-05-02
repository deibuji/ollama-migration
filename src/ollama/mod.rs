//! Ollama integration module.
//!
//! Provides functionality to discover, parse, and extract models from Ollama's
//! content-addressable storage format.

pub mod blob;
pub mod discovery;
pub mod manifest;

pub use blob::BlobRef;
pub use discovery::{OllamaInstallation, OllamaModel};
pub use manifest::Manifest;
