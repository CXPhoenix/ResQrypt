//! Resqrypt error types
//!
//! This module defines all error types used throughout the application.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for resqrypt operations
#[derive(Error, Debug)]
pub enum ResqryptError {
    /// I/O operation failed
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid file format (not a resqrypt file)
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    /// Encryption or decryption failed
    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),

    /// Password-related error
    #[error("Password error: {0}")]
    PasswordError(String),

    /// Compression or decompression failed
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Archive operation failed
    #[error("Archive error: {0}")]
    ArchiveError(String),

    /// File or directory not found
    #[error("Not found: {}", .0.display())]
    NotFound(PathBuf),

    /// File already exists
    #[error("File already exists: {}", .0.display())]
    AlreadyExists(PathBuf),

    /// Invalid argument provided
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

/// Result type alias for resqrypt operations
pub type Result<T> = std::result::Result<T, ResqryptError>;
