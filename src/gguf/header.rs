use std::io::Read;
use std::path::Path;

use crate::error::{MigrationError, Result};

/// GGUF file header magic number (big-endian: "GGUF")
pub const GGUF_MAGIC: u32 = 0x46554747;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GGUFHeader {
    pub magic: u32,
    pub version: u32,
    pub tensor_count: u64,
    pub metadata_kv_count: u64,
}

impl GGUFHeader {
    /// Minimum header size: 4 (magic) + 4 (version) + 8 (tensors) + 8 (metadata) = 24 bytes
    pub const MIN_SIZE: usize = 24;

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < Self::MIN_SIZE {
            return Err(MigrationError::InvalidGGUF("Header too small".into()));
        }

        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != GGUF_MAGIC {
            let magic_be = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
            return Err(MigrationError::InvalidGGUF(format!(
                "Invalid magic: expected 0x{:08x}, got 0x{:08x}",
                GGUF_MAGIC, magic_be
            )));
        }

        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if version != 2 && version != 3 {
            return Err(MigrationError::InvalidGGUF(format!(
                "Unsupported GGUF version: {}",
                version
            )));
        }

        let tensor_count = u64::from_le_bytes([
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
        ]);

        let metadata_kv_count = u64::from_le_bytes([
            data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23],
        ]);

        Ok(Self {
            magic,
            version,
            tensor_count,
            metadata_kv_count,
        })
    }

    pub fn from_reader(reader: &mut impl Read) -> Result<Self> {
        let mut buf = [0u8; Self::MIN_SIZE];
        reader.read_exact(&mut buf)?;
        Self::from_bytes(&buf)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        Self::from_reader(&mut file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gguf_v2_header() {
        let mut header = Vec::new();
        header.extend_from_slice(&GGUF_MAGIC.to_le_bytes());
        header.extend_from_slice(&2u32.to_le_bytes());
        header.extend_from_slice(&100u64.to_le_bytes());
        header.extend_from_slice(&200u64.to_le_bytes());

        let parsed = GGUFHeader::from_bytes(&header).unwrap();
        assert_eq!(parsed.magic, GGUF_MAGIC);
        assert_eq!(parsed.version, 2);
        assert_eq!(parsed.tensor_count, 100);
        assert_eq!(parsed.metadata_kv_count, 200);
    }

    #[test]
    fn test_valid_gguf_v3_header() {
        let mut header = Vec::new();
        header.extend_from_slice(&GGUF_MAGIC.to_le_bytes());
        header.extend_from_slice(&3u32.to_le_bytes());
        header.extend_from_slice(&100u64.to_le_bytes());
        header.extend_from_slice(&200u64.to_le_bytes());

        let parsed = GGUFHeader::from_bytes(&header).unwrap();
        assert_eq!(parsed.version, 3);
    }

    #[test]
    fn test_invalid_magic() {
        let header = vec![0x00; 24];
        assert!(GGUFHeader::from_bytes(&header).is_err());
    }

    #[test]
    fn test_header_too_small() {
        let header = vec![0x00; 10];
        assert!(GGUFHeader::from_bytes(&header).is_err());
    }
}
