use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub config: Config,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub digest: String,
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub digest: String,
    pub size: u64,
}

impl Manifest {
    pub const MODEL_MEDIA_TYPE: &'static str = "application/vnd.ollama.image.model";
    pub const CONFIG_MEDIA_TYPE: &'static str = "application/vnd.ollama.image.config";

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Manifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    pub fn get_model_layer(&self) -> Option<&Layer> {
        self.layers
            .iter()
            .find(|l| l.media_type == Self::MODEL_MEDIA_TYPE)
    }

    pub fn get_config_layer(&self) -> Option<&Layer> {
        self.layers
            .iter()
            .find(|l| l.media_type == Self::CONFIG_MEDIA_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MANIFEST: &str = r#"{
        "schemaVersion": 2,
        "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
        "config": {
            "mediaType": "application/vnd.ollama.image.config",
            "digest": "sha256:abc123",
            "size": 100
        },
        "layers": [
            {
                "mediaType": "application/vnd.ollama.image.model",
                "digest": "sha256:def456",
                "size": 5000000000
            }
        ]
    }"#;

    #[test]
    fn test_parse_manifest() {
        let manifest: Manifest = serde_json::from_str(TEST_MANIFEST).unwrap();
        assert_eq!(manifest.schema_version, 2);
        assert_eq!(manifest.config.size, 100);
    }

    #[test]
    fn test_get_model_layer() {
        let manifest: Manifest = serde_json::from_str(TEST_MANIFEST).unwrap();
        let layer = manifest.get_model_layer().unwrap();
        assert_eq!(layer.digest, "sha256:def456");
        assert_eq!(layer.size, 5000000000);
    }
}
