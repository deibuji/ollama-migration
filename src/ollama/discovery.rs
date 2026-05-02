use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::platform::Platform;

use super::blob::BlobRef;
use super::manifest::Manifest;

#[derive(Debug, Clone)]
pub struct OllamaModel {
    pub name: String,
    pub manifest_path: PathBuf,
    pub total_size: u64,
    pub model_blob: BlobRef,
    pub config_blob: Option<BlobRef>,
}

#[derive(Debug, Clone)]
pub struct OllamaInstallation {
    pub models_dir: PathBuf,
    pub models: Vec<OllamaModel>,
}

impl OllamaInstallation {
    pub fn discover() -> Result<Self> {
        let models_dir = Platform::current().ollama_default_dir()?;
        Self::discover_at(&models_dir)
    }

    pub fn discover_at(models_dir: &Path) -> Result<Self> {
        let manifests_dir = models_dir.join("manifests");
        let blobs_dir = models_dir.join("blobs");

        let mut models = Vec::new();

        if !manifests_dir.exists() {
            return Ok(OllamaInstallation {
                models_dir: models_dir.to_path_buf(),
                models,
            });
        }

        Self::walk_manifests(&manifests_dir, &manifests_dir, &blobs_dir, &mut models)?;

        Ok(OllamaInstallation {
            models_dir: models_dir.to_path_buf(),
            models,
        })
    }

    fn walk_manifests(
        current_dir: &Path,
        manifests_root: &Path,
        blobs_dir: &Path,
        models: &mut Vec<OllamaModel>,
    ) -> Result<()> {
        for entry in std::fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::walk_manifests(&path, manifests_root, blobs_dir, models)?;
            } else if path.is_file()
                && let Some(model) = Self::parse_model(&path, manifests_root, blobs_dir)
            {
                models.push(model);
            }
        }
        Ok(())
    }

    fn parse_model(
        manifest_path: &Path,
        manifests_dir: &Path,
        blobs_dir: &Path,
    ) -> Option<OllamaModel> {
        let manifest = Manifest::from_file(manifest_path).ok()?;

        let name = Self::build_model_name(manifest_path, manifests_dir);

        let model_layer = manifest.get_model_layer()?;
        let model_blob = BlobRef::from_digest(&model_layer.digest, blobs_dir).ok()?;

        let config_blob = manifest
            .get_config_layer()
            .and_then(|l| BlobRef::from_digest(&l.digest, blobs_dir).ok());

        let total_size = model_layer.size + config_blob.as_ref().map(|b| b.size).unwrap_or(0);

        Some(OllamaModel {
            name,
            manifest_path: manifest_path.to_path_buf(),
            total_size,
            model_blob,
            config_blob,
        })
    }

    fn build_model_name(manifest_path: &Path, manifests_dir: &Path) -> String {
        let relative = manifest_path
            .strip_prefix(manifests_dir)
            .unwrap_or(manifest_path);
        let components: Vec<_> = relative
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .filter(|s| !s.is_empty())
            .collect();

        if components.len() >= 3 {
            format!(
                "{}/{}",
                components[components.len() - 2],
                components.last().unwrap()
            )
        } else if components.len() == 2 {
            format!("{}/{}", components[0], components[1])
        } else {
            relative.to_string_lossy().to_string()
        }
    }

    pub fn find_model(&self, name: &str) -> Option<&OllamaModel> {
        self.models.iter().find(|m| m.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn setup_mock_ollama(temp: &std::path::Path) -> PathBuf {
        let models_dir = temp.join(".ollama").join("models");
        let manifests = models_dir
            .join("manifests")
            .join("registry.ollama.ai")
            .join("library")
            .join("llama3.2");
        let blobs = models_dir.join("blobs");

        std::fs::create_dir_all(&manifests).unwrap();
        std::fs::create_dir_all(&blobs).unwrap();

        let manifest = r#"{
            "schemaVersion": 2,
            "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
            "config": {
                "mediaType": "application/vnd.ollama.image.config",
                "digest": "sha256:cfg000",
                "size": 100
            },
            "layers": [{
                "mediaType": "application/vnd.ollama.image.model",
                "digest": "sha256:mdl000",
                "size": 1000
            }]
        }"#;

        let mut f = std::fs::File::create(manifests.join("latest")).unwrap();
        f.write_all(manifest.as_bytes()).unwrap();

        std::fs::File::create(blobs.join("sha256-cfg000")).unwrap();
        std::fs::File::create(blobs.join("sha256-mdl000")).unwrap();

        models_dir
    }

    #[test]
    fn test_discover_at() {
        let temp = std::env::temp_dir().join("ollama_test");
        let _ = std::fs::remove_dir_all(&temp);
        let models_dir = setup_mock_ollama(&temp);

        let install = OllamaInstallation::discover_at(&models_dir).unwrap();
        assert_eq!(install.models.len(), 1);

        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_find_model() {
        let temp = std::env::temp_dir().join("ollama_test2");
        let _ = std::fs::remove_dir_all(&temp);
        let models_dir = setup_mock_ollama(&temp);

        let install = OllamaInstallation::discover_at(&models_dir).unwrap();
        assert!(install.find_model("llama3.2/latest").is_some());

        let _ = std::fs::remove_dir_all(&temp);
    }
}
