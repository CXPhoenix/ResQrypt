//! Encrypted file format handling
//!
//! Handles reading and writing the resqrypt file format header.
//!
//! File format v1:
//! - Magic (8 bytes): "RESQRYPT"
//! - Version (1 byte): 0x01
//! - Flags (1 byte): compression/archive flags
//! - KDF memory cost (4 bytes, LE): Argon2id memory in KiB
//! - KDF time cost (4 bytes, LE): Argon2id iterations
//! - KDF parallelism (4 bytes, LE): Argon2id parallelism
//! - Salt (32 bytes): Argon2id salt
//! - Nonce (12 bytes): AES-GCM nonce
//! - Encrypted data: payload + 16-byte auth tag

use std::io::{Read, Write};

use crate::crypto::kdf::KdfParams;
use crate::error::{ResqryptError, Result};
use crate::{FORMAT_VERSION, MAGIC_BYTES, aes_params, flags, kdf_defaults};

/// File header for encrypted files
#[derive(Debug, Clone)]
pub struct FileHeader {
    /// Format version
    pub version: u8,
    /// Flags indicating compression and archive type
    pub flags: u8,
    /// KDF parameters used for encryption
    pub kdf_params: KdfParams,
    /// Salt for key derivation
    pub salt: [u8; 32],
    /// Nonce for AES-GCM
    pub nonce: [u8; 12],
}

impl FileHeader {
    /// Header size in bytes: 8 (magic) + 1 (version) + 1 (flags) + 12 (kdf params) + 32 (salt) + 12 (nonce) = 66
    pub const SIZE: usize = 8 + 1 + 1 + 12 + kdf_defaults::SALT_LEN + aes_params::NONCE_LEN;

    /// Create a new header for encryption
    pub fn new(flags: u8, kdf_params: KdfParams, salt: [u8; 32], nonce: [u8; 12]) -> Self {
        Self { version: FORMAT_VERSION, flags, kdf_params, salt, nonce }
    }

    /// Check if the source was already zstd compressed
    pub fn is_already_zstd(&self) -> bool {
        self.flags & flags::ALREADY_ZSTD != 0
    }

    /// Check if the source was a directory
    pub fn is_directory(&self) -> bool {
        self.flags & flags::IS_DIRECTORY != 0
    }
}

/// Write the file header to a writer
pub fn write_header<W: Write>(writer: &mut W, header: &FileHeader) -> Result<()> {
    writer.write_all(MAGIC_BYTES)?;
    writer.write_all(&[header.version])?;
    writer.write_all(&[header.flags])?;
    // Write KDF params as little-endian u32
    writer.write_all(&header.kdf_params.memory_cost.to_le_bytes())?;
    writer.write_all(&header.kdf_params.time_cost.to_le_bytes())?;
    writer.write_all(&header.kdf_params.parallelism.to_le_bytes())?;
    writer.write_all(&header.salt)?;
    writer.write_all(&header.nonce)?;
    Ok(())
}

/// Read and validate the file header from a reader
pub fn read_header<R: Read>(reader: &mut R) -> Result<FileHeader> {
    // Read magic bytes
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic)?;

    if &magic != MAGIC_BYTES {
        return Err(ResqryptError::InvalidFormat(
            "Not a valid resqrypt file (invalid magic bytes)".to_string(),
        ));
    }

    // Read version
    let mut version = [0u8; 1];
    reader.read_exact(&mut version)?;
    let version = version[0];

    if version != FORMAT_VERSION {
        return Err(ResqryptError::InvalidFormat(format!(
            "Unsupported file format version: {} (expected {})",
            version, FORMAT_VERSION
        )));
    }

    // Read flags
    let mut flags_buf = [0u8; 1];
    reader.read_exact(&mut flags_buf)?;
    let flags = flags_buf[0];

    // Read KDF params
    let mut memory_cost_buf = [0u8; 4];
    reader.read_exact(&mut memory_cost_buf)?;
    let memory_cost = u32::from_le_bytes(memory_cost_buf);

    let mut time_cost_buf = [0u8; 4];
    reader.read_exact(&mut time_cost_buf)?;
    let time_cost = u32::from_le_bytes(time_cost_buf);

    let mut parallelism_buf = [0u8; 4];
    reader.read_exact(&mut parallelism_buf)?;
    let parallelism = u32::from_le_bytes(parallelism_buf);

    let kdf_params = KdfParams { memory_cost, time_cost, parallelism };

    // Read salt
    let mut salt = [0u8; 32];
    reader.read_exact(&mut salt)?;

    // Read nonce
    let mut nonce = [0u8; 12];
    reader.read_exact(&mut nonce)?;

    Ok(FileHeader { version, flags, kdf_params, salt, nonce })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_roundtrip() {
        let kdf_params = KdfParams::default();
        let header = FileHeader::new(0, kdf_params.clone(), [1u8; 32], [2u8; 12]);

        let mut buffer = Vec::new();
        write_header(&mut buffer, &header).unwrap();

        assert_eq!(buffer.len(), FileHeader::SIZE);

        let mut cursor = Cursor::new(buffer);
        let read_header = read_header(&mut cursor).unwrap();

        assert_eq!(read_header.version, FORMAT_VERSION);
        assert_eq!(read_header.flags, 0);
        assert_eq!(read_header.kdf_params.memory_cost, kdf_params.memory_cost);
        assert_eq!(read_header.kdf_params.time_cost, kdf_params.time_cost);
        assert_eq!(read_header.kdf_params.parallelism, kdf_params.parallelism);
        assert_eq!(read_header.salt, [1u8; 32]);
        assert_eq!(read_header.nonce, [2u8; 12]);
    }

    #[test]
    fn test_header_with_custom_kdf() {
        let kdf_params = KdfParams { memory_cost: 32 * 1024, time_cost: 5, parallelism: 2 };
        let header = FileHeader::new(0, kdf_params.clone(), [0u8; 32], [0u8; 12]);

        let mut buffer = Vec::new();
        write_header(&mut buffer, &header).unwrap();

        let mut cursor = Cursor::new(buffer);
        let read_header = read_header(&mut cursor).unwrap();

        assert_eq!(read_header.kdf_params.memory_cost, 32 * 1024);
        assert_eq!(read_header.kdf_params.time_cost, 5);
        assert_eq!(read_header.kdf_params.parallelism, 2);
    }

    #[test]
    fn test_header_with_flags() {
        let header = FileHeader::new(
            flags::ALREADY_ZSTD | flags::IS_DIRECTORY,
            KdfParams::default(),
            [0u8; 32],
            [0u8; 12],
        );

        assert!(header.is_already_zstd());
        assert!(header.is_directory());

        let header2 = FileHeader::new(0, KdfParams::default(), [0u8; 32], [0u8; 12]);
        assert!(!header2.is_already_zstd());
        assert!(!header2.is_directory());
    }

    #[test]
    fn test_invalid_magic() {
        let mut buffer = vec![0u8; FileHeader::SIZE];
        buffer[..8].copy_from_slice(b"INVALID!");

        let mut cursor = Cursor::new(buffer);
        let result = read_header(&mut cursor);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ResqryptError::InvalidFormat(_)));
    }

    #[test]
    fn test_invalid_version() {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(MAGIC_BYTES);
        buffer.push(0xFF); // Invalid version
        buffer.push(0); // flags
        buffer.extend_from_slice(&[0u8; 12]); // kdf params
        buffer.extend_from_slice(&[0u8; 32]); // salt
        buffer.extend_from_slice(&[0u8; 12]); // nonce

        let mut cursor = Cursor::new(buffer);
        let result = read_header(&mut cursor);

        assert!(result.is_err());
    }

    #[test]
    fn test_header_size() {
        assert_eq!(FileHeader::SIZE, 66);
    }
}
