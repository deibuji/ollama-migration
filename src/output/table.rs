use crate::ollama::OllamaModel;
use crate::gguf::GGUFMetadata;
use crate::gguf::metadata::format_size;

/// Table output formatter
pub struct TableOutput;

impl TableOutput {
    pub fn print_model_list(&self, models: &[ OllamaModel]) {
        println!("{:<30} {:<15} {:<12}", "NAME", "SIZE", "BLOB");
        println!("{}", "-".repeat(60));
        
        for model in models {
            let size_str = format_size(model.total_size);
            let blob_short = if model.model_blob.hash.len() > 12 {
                &model.model_blob.hash[..12]
            } else {
                &model.model_blob.hash
            };
            println!("{:<30} {:<15} {:<12}", model.name, size_str, blob_short);
        }
        
        println!("\n{} model(s) found", models.len());
    }

    pub fn print_model_info(&self, model: &OllamaModel) {
        println!("Name:       {}", model.name);
        println!("Size:       {}", format_size(model.total_size));
        println!("Digest:     {}", model.model_blob.digest);
        println!("Manifest:   {}", model.manifest_path.display());
        
        if let Some(ref config) = model.config_blob {
            println!("Config:     {} ({} bytes)", config.digest, config.size);
        }
    }

    pub fn print_gguf_info(&self, meta: &GGUFMetadata) {
        println!("GGUF Version:        {}", meta.version);
        println!("Tensor Count:        {}", meta.tensor_count);
        println!("Metadata KV Count:   {}", meta.metadata_kv_count);
        
        if let Some(ref arch) = meta.architecture {
            println!("Architecture:      {}", arch);
        }
        if let Some(ref quant) = meta.quantization {
            println!("Quantization:        {}", quant);
        }
        if let Some(params) = meta.parameters {
            println!("Parameters:          {}", params);
        }
        if let Some(ctx) = meta.context_length {
            println!("Context Length:      {}", ctx);
        }
    }

    pub fn print_export_summary(&self, success: usize, failed: usize) {
        println!("\nExport Summary: {}/{} succeeded", success, success + failed);
    }
}
