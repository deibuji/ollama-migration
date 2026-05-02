use crate::error::Result;

use super::header::GGUFHeader;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct GGUFMetadata {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata_kv_count: u64,
    pub architecture: Option<String>,
    pub quantization: Option<String>,
    pub parameters: Option<u64>,
    pub context_length: Option<u32>,
}

impl TryFrom<GGUFHeader> for GGUFMetadata {
    type Error = crate::error::MigrationError;

    fn try_from(header: GGUFHeader) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            version: header.version,
            tensor_count: header.tensor_count,
            metadata_kv_count: header.metadata_kv_count,
            ..Default::default()
        })
    }
}

pub fn extract_header_info(data: &[u8]) -> Result<GGUFMetadata> {
    let header = GGUFHeader::from_bytes(data)?;
    let mut meta: GGUFMetadata = header.try_into()?;

    // Attempt to parse common metadata keys from GGUF kv array
    // Full metadata parsing requires full GGUF spec implementation
    // This is a simplified version that extracts header-level info

    Ok(meta)
}

pub fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_idx])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512.00 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(5 * 1024 * 1024 * 1024), "5.00 GB");
    }
}
