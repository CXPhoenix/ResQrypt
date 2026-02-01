//! zstd compression and decompression
//!
//! Provides high-level compression/decompression functions.

use crate::error::{ResqryptError, Result};

/// Default compression level (3 is a good balance of speed and ratio)
const DEFAULT_LEVEL: i32 = 3;

/// Compress data using zstd
///
/// Uses the default compression level (3) which provides a good balance
/// between compression ratio and speed.
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    zstd::encode_all(data, DEFAULT_LEVEL)
        .map_err(|e| ResqryptError::CompressionError(format!("Compression failed: {}", e)))
}

/// Decompress zstd-compressed data
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    zstd::decode_all(data)
        .map_err(|e| ResqryptError::CompressionError(format!("Decompression failed: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_roundtrip() {
        let original = b"Hello, World! This is some test data for compression.";

        let compressed = compress(original).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(original.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_compression_reduces_size() {
        // Highly compressible data
        let original = vec![b'A'; 10000];

        let compressed = compress(&original).unwrap();

        assert!(compressed.len() < original.len());
    }

    #[test]
    fn test_empty_data() {
        let original: &[u8] = b"";

        let compressed = compress(original).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(original, decompressed.as_slice());
    }

    #[test]
    fn test_large_data() {
        let original = vec![0xAB; 1024 * 1024]; // 1 MB

        let compressed = compress(&original).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_invalid_zstd_data() {
        let invalid = b"This is not zstd compressed data";

        let result = decompress(invalid);
        assert!(result.is_err());
    }
}
