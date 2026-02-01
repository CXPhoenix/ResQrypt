//! Cryptographic operations module
//!
//! This module provides encryption and decryption functionality using:
//! - Argon2id for password-based key derivation
//! - AES-256-GCM for authenticated encryption

pub mod aes;
pub mod format;
pub mod kdf;

pub use aes::{decrypt_data, encrypt_data};
pub use format::{FileHeader, read_header, write_header};
pub use kdf::{KdfParams, derive_key};
