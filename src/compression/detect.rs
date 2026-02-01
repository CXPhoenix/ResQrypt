//! zstd format detection
//!
//! Detects if data is already zstd compressed by checking magic bytes.

use crate::ZSTD_MAGIC;

/// Check if data is already zstd compressed
///
/// Detects the zstd magic bytes (0x28 0xB5 0x2F 0xFD) at the start of the data.
pub fn is_zstd_compressed(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    &data[..4] == ZSTD_MAGIC
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_zstd_magic() {
        // zstd magic bytes
        let zstd_data = [0x28, 0xB5, 0x2F, 0xFD, 0x00, 0x00];
        assert!(is_zstd_compressed(&zstd_data));
    }

    #[test]
    fn test_detect_non_zstd() {
        let plain_text = b"Hello, World!";
        assert!(!is_zstd_compressed(plain_text));
    }

    #[test]
    fn test_detect_too_short() {
        let short_data = [0x28, 0xB5, 0x2F]; // Only 3 bytes
        assert!(!is_zstd_compressed(&short_data));
    }

    #[test]
    fn test_detect_empty() {
        let empty: [u8; 0] = [];
        assert!(!is_zstd_compressed(&empty));
    }

    #[test]
    fn test_detect_actual_zstd() {
        // Compress some data and verify detection
        let original = b"Test data for compression";
        let compressed = zstd::encode_all(&original[..], 3).unwrap();
        assert!(is_zstd_compressed(&compressed));
    }
}
