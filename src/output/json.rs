use serde::Serialize;

use crate::ollama::OllamaModel;
use crate::gguf::GGUFMetadata;

/// JSON output formatter
pub struct JsonOutput;

impl JsonOutput {
    pub fn print_model_list(&self, models: &[ OllamaModel]) {
        let serializable: Vec<SerializableModel> = models.iter().map(Into::into).collect();
        println!("{}", serde_json::to_string_pretty(&serializable).unwrap());
    }

    pub fn print_model_info(&self, model: &OllamaModel) {
        let json: SerializableModel = model.into();
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }

    pub fn print_gguf_info(&self, meta: &GGUFMetadata) {
        println!("{}", serde_json::to_string_pretty(meta).unwrap());
    }

    pub fn print_export_summary(&self, success: usize, failed: usize) {
        let summary = serde_json::json!({
            "success": success,
            "failed": failed,
            "total": success + failed,
        });
        println!("{}", serde_json::to_string_pretty(&summary).unwrap());
    }
}

#[derive(Serialize)]
struct SerializableModel {
    name: String,
    size: u64,
    digest: String,
    manifest_path: String,
}

impl From<&OllamaModel> for SerializableModel {
    fn from(m: &OllamaModel) -> Self {
        Self {
            name: m.name.clone(),
            size: m.total_size,
            digest: m.model_blob.digest.clone(),
            manifest_path: m.manifest_path.display().to_string(),
        }
    }
}
