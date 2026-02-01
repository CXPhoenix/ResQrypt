//! Resqrypt - Secure file and directory encryption
//!
//! A command-line tool for encrypting files and directories using:
//! - **zstd** compression (with smart detection of already-compressed files)
//! - **AES-256-GCM** authenticated encryption
//! - **Argon2id** password-based key derivation
//!
//! # Example
//!
//! ```bash
//! # Encrypt a file
//! resqrypt encrypt -i secret.txt -o secret.txt.resqrypt
//!
//! # Decrypt a file
//! resqrypt decrypt -i secret.txt.resqrypt -o secret.txt
//! ```

pub mod error;

pub use error::{ResqryptError, Result};

/// File format magic bytes
pub const MAGIC_BYTES: &[u8; 8] = b"RESQRYPT";

/// Current file format version
pub const FORMAT_VERSION: u8 = 0x01;

/// Flags for the encrypted file format
pub mod flags {
    /// Bit 0: 0 = data was compressed, 1 = data was already zstd
    pub const ALREADY_ZSTD: u8 = 0b0000_0001;
    /// Bit 1: 0 = single file, 1 = directory (tar archive)
    pub const IS_DIRECTORY: u8 = 0b0000_0010;
}

/// zstd magic bytes for detection
pub const ZSTD_MAGIC: &[u8; 4] = &[0x28, 0xB5, 0x2F, 0xFD];

/// Default Argon2id parameters
pub mod kdf_defaults {
    /// Memory cost in KiB (64 MB)
    pub const MEMORY_COST: u32 = 64 * 1024;
    /// Number of iterations
    pub const TIME_COST: u32 = 3;
    /// Degree of parallelism
    pub const PARALLELISM: u32 = 4;
    /// Output key length in bytes
    pub const OUTPUT_LEN: usize = 32;
    /// Salt length in bytes
    pub const SALT_LEN: usize = 32;
}

/// AES-256-GCM parameters
pub mod aes_params {
    /// Nonce length in bytes
    pub const NONCE_LEN: usize = 12;
    /// Authentication tag length in bytes
    pub const TAG_LEN: usize = 16;
}
