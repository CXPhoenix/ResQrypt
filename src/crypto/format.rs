//! Encrypted file format handling
//!
//! Handles reading and writing the resqrypt file format header.

use std::io::{Read, Write};

use crate::error::{ResqryptError, Result};
use crate::{FORMAT_VERSION, MAGIC_BYTES, aes_params, flags, kdf_defaults};

/// File header for encrypted files
#[derive(Debug, Clone)]
pub struct FileHeader {
    /// Format version
    pub version: u8,
    /// Flags indicating compression and archive type
    pub flags: u8,
    /// Salt for key derivation
    pub salt: [u8; 32],
    /// Nonce for AES-GCM
    pub nonce: [u8; 12],
}

impl FileHeader {
    /// Header size in bytes: 8 (magic) + 1 (version) + 1 (flags) + 32 (salt) + 12 (nonce) = 54
    pub const SIZE: usize = 8 + 1 + 1 + kdf_defaults::SALT_LEN + aes_params::NONCE_LEN;

    /// Create a new header for encryption
    pub fn new(flags: u8, salt: [u8; 32], nonce: [u8; 12]) -> Self {
        Self { version: FORMAT_VERSION, flags, salt, nonce }
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

    // Read salt
    let mut salt = [0u8; 32];
    reader.read_exact(&mut salt)?;

    // Read nonce
    let mut nonce = [0u8; 12];
    reader.read_exact(&mut nonce)?;

    Ok(FileHeader { version, flags, salt, nonce })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_roundtrip() {
        let header = FileHeader::new(0, [1u8; 32], [2u8; 12]);

        let mut buffer = Vec::new();
        write_header(&mut buffer, &header).unwrap();

        assert_eq!(buffer.len(), FileHeader::SIZE);

        let mut cursor = Cursor::new(buffer);
        let read_header = read_header(&mut cursor).unwrap();

        assert_eq!(read_header.version, FORMAT_VERSION);
        assert_eq!(read_header.flags, 0);
        assert_eq!(read_header.salt, [1u8; 32]);
        assert_eq!(read_header.nonce, [2u8; 12]);
    }

    #[test]
    fn test_header_with_flags() {
        let header =
            FileHeader::new(flags::ALREADY_ZSTD | flags::IS_DIRECTORY, [0u8; 32], [0u8; 12]);

        assert!(header.is_already_zstd());
        assert!(header.is_directory());

        let header2 = FileHeader::new(0, [0u8; 32], [0u8; 12]);
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
        buffer.extend_from_slice(&[0u8; 32]); // salt
        buffer.extend_from_slice(&[0u8; 12]); // nonce

        let mut cursor = Cursor::new(buffer);
        let result = read_header(&mut cursor);

        assert!(result.is_err());
    }

    #[test]
    fn test_header_size() {
        assert_eq!(FileHeader::SIZE, 54);
    }
}
