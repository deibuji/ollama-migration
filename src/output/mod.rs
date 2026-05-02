//! Output formatting module.

use crate::gguf::GGUFMetadata;
use crate::ollama::OllamaModel;

pub mod json;
pub mod table;

pub use json::JsonOutput;
pub use table::TableOutput;

/// Output trait for formatting CLI output
pub trait Output {
    fn write_model_list(&self, models: &[OllamaModel]);
    fn write_model_info(&self, model: &OllamaModel);
    fn write_gguf_info(&self, meta: &GGUFMetadata);
    fn write_export_summary(&self, success: usize, failed: usize);
}

impl Output for TableOutput {
    fn write_model_list(&self, models: &[OllamaModel]) {
        self.print_model_list(models);
    }

    fn write_model_info(&self, model: &OllamaModel) {
        self.print_model_info(model);
    }

    fn write_gguf_info(&self, meta: &GGUFMetadata) {
        self.print_gguf_info(meta);
    }

    fn write_export_summary(&self, success: usize, failed: usize) {
        self.print_export_summary(success, failed);
    }
}

impl Output for JsonOutput {
    fn write_model_list(&self, models: &[OllamaModel]) {
        self.print_model_list(models);
    }

    fn write_model_info(&self, model: &OllamaModel) {
        self.print_model_info(model);
    }

    fn write_gguf_info(&self, meta: &GGUFMetadata) {
        self.print_gguf_info(meta);
    }

    fn write_export_summary(&self, success: usize, failed: usize) {
        self.print_export_summary(success, failed);
    }
}
