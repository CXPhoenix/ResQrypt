//! Compression operations module
//!
//! Provides zstd compression/decompression and format detection.

pub mod detect;
pub mod zstd;

pub use detect::is_zstd_compressed;
pub use zstd::{compress, decompress};
