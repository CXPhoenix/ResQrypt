//! AES-256-GCM encryption and decryption
//!
//! Provides authenticated encryption using AES-256-GCM (AEAD).

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use rand::Rng;

use crate::aes_params;
use crate::error::{ResqryptError, Result};

/// Generate a random nonce for AES-GCM
pub fn generate_nonce() -> [u8; 12] {
    rand::rng().random()
}

/// Encrypt data using AES-256-GCM
///
/// # Arguments
/// * `key` - 32-byte encryption key (from KDF)
/// * `nonce` - 12-byte nonce (use `generate_nonce()` for new encryptions)
/// * `plaintext` - Data to encrypt
///
/// # Returns
/// Ciphertext with authentication tag appended (plaintext.len() + 16 bytes)
pub fn encrypt_data(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| ResqryptError::CryptoError(format!("Failed to create cipher: {}", e)))?;

    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| ResqryptError::CryptoError(format!("Encryption failed: {}", e)))
}

/// Decrypt data using AES-256-GCM
///
/// # Arguments
/// * `key` - 32-byte encryption key (from KDF)
/// * `nonce` - 12-byte nonce (must match the nonce used for encryption)
/// * `ciphertext` - Encrypted data with authentication tag
///
/// # Returns
/// Decrypted plaintext
///
/// # Errors
/// Returns an error if authentication fails (wrong password or tampered data)
pub fn decrypt_data(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if ciphertext.len() < aes_params::TAG_LEN {
        return Err(ResqryptError::CryptoError("Ciphertext too short".to_string()));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| ResqryptError::CryptoError(format!("Failed to create cipher: {}", e)))?;

    let nonce = Nonce::from_slice(nonce);

    cipher.decrypt(nonce, ciphertext).map_err(|_| {
        ResqryptError::PasswordError(
            "Decryption failed: wrong password or corrupted data".to_string(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0u8; 32];
        let nonce = generate_nonce();
        let plaintext = b"Hello, World!";

        let ciphertext = encrypt_data(&key, &nonce, plaintext).unwrap();
        let decrypted = decrypt_data(&key, &nonce, &ciphertext).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_ciphertext_longer_than_plaintext() {
        let key = [0u8; 32];
        let nonce = generate_nonce();
        let plaintext = b"Hello, World!";

        let ciphertext = encrypt_data(&key, &nonce, plaintext).unwrap();

        // Ciphertext should be plaintext + 16 byte tag
        assert_eq!(ciphertext.len(), plaintext.len() + 16);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = [0u8; 32];
        let key2 = [1u8; 32];
        let nonce = generate_nonce();
        let plaintext = b"Secret data";

        let ciphertext = encrypt_data(&key1, &nonce, plaintext).unwrap();
        let result = decrypt_data(&key2, &nonce, &ciphertext);

        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_nonce_fails() {
        let key = [0u8; 32];
        let nonce1 = [0u8; 12];
        let nonce2 = [1u8; 12];
        let plaintext = b"Secret data";

        let ciphertext = encrypt_data(&key, &nonce1, plaintext).unwrap();
        let result = decrypt_data(&key, &nonce2, &ciphertext);

        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_ciphertext_fails() {
        let key = [0u8; 32];
        let nonce = generate_nonce();
        let plaintext = b"Secret data";

        let mut ciphertext = encrypt_data(&key, &nonce, plaintext).unwrap();
        // Tamper with the ciphertext
        ciphertext[0] ^= 0xFF;

        let result = decrypt_data(&key, &nonce, &ciphertext);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_plaintext() {
        let key = [0u8; 32];
        let nonce = generate_nonce();
        let plaintext = b"";

        let ciphertext = encrypt_data(&key, &nonce, plaintext).unwrap();
        let decrypted = decrypt_data(&key, &nonce, &ciphertext).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_large_plaintext() {
        let key = [0u8; 32];
        let nonce = generate_nonce();
        let plaintext = vec![0xAB; 1024 * 1024]; // 1 MB

        let ciphertext = encrypt_data(&key, &nonce, &plaintext).unwrap();
        let decrypted = decrypt_data(&key, &nonce, &ciphertext).unwrap();

        assert_eq!(plaintext, decrypted);
    }
}
