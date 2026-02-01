//! Argon2id key derivation
//!
//! Provides secure password-based key derivation using Argon2id,
//! which is resistant to GPU and ASIC attacks.

use argon2::{Algorithm, Argon2, Params, Version};
use rand::Rng;

use crate::error::{ResqryptError, Result};
use crate::kdf_defaults;

/// Parameters for Argon2id key derivation
#[derive(Debug, Clone)]
pub struct KdfParams {
    /// Memory cost in KiB
    pub memory_cost: u32,
    /// Number of iterations (time cost)
    pub time_cost: u32,
    /// Degree of parallelism
    pub parallelism: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        Self {
            memory_cost: kdf_defaults::MEMORY_COST,
            time_cost: kdf_defaults::TIME_COST,
            parallelism: kdf_defaults::PARALLELISM,
        }
    }
}

impl KdfParams {
    /// Create new KdfParams with custom values
    pub fn new(memory_mb: u32, iterations: u32, parallelism: u32) -> Self {
        Self {
            memory_cost: memory_mb * 1024, // Convert MB to KiB
            time_cost: iterations,
            parallelism,
        }
    }

    /// Build Argon2 instance with these parameters
    fn build_argon2(&self) -> Result<Argon2<'static>> {
        let params = Params::new(
            self.memory_cost,
            self.time_cost,
            self.parallelism,
            Some(kdf_defaults::OUTPUT_LEN),
        )
        .map_err(|e| ResqryptError::CryptoError(format!("Invalid Argon2 params: {}", e)))?;

        Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
    }
}

/// Generate a random salt for key derivation
pub fn generate_salt() -> [u8; 32] {
    rand::rng().random()
}

/// Derive an encryption key from a password using Argon2id
///
/// # Arguments
/// * `password` - The user's password
/// * `salt` - A random 32-byte salt (use `generate_salt()` for new encryptions)
/// * `params` - KDF parameters (use `KdfParams::default()` for standard security)
///
/// # Returns
/// A 32-byte key suitable for AES-256
pub fn derive_key(password: &[u8], salt: &[u8; 32], params: &KdfParams) -> Result<[u8; 32]> {
    let argon2 = params.build_argon2()?;

    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password, salt, &mut key)
        .map_err(|e| ResqryptError::CryptoError(format!("Key derivation failed: {}", e)))?;

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_basic() {
        let password = b"test-password";
        let salt = generate_salt();
        let params = KdfParams::default();

        let key = derive_key(password, &salt, &params).unwrap();

        assert_eq!(key.len(), 32);
        // Key should not be all zeros
        assert!(key.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_derive_key_deterministic() {
        let password = b"test-password";
        let salt = [0u8; 32]; // Fixed salt for testing
        let params = KdfParams::default();

        let key1 = derive_key(password, &salt, &params).unwrap();
        let key2 = derive_key(password, &salt, &params).unwrap();

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_different_passwords_different_keys() {
        let salt = [0u8; 32];
        let params = KdfParams::default();

        let key1 = derive_key(b"password1", &salt, &params).unwrap();
        let key2 = derive_key(b"password2", &salt, &params).unwrap();

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_different_salts_different_keys() {
        let password = b"same-password";
        let params = KdfParams::default();

        let salt1 = [1u8; 32];
        let salt2 = [2u8; 32];

        let key1 = derive_key(password, &salt1, &params).unwrap();
        let key2 = derive_key(password, &salt2, &params).unwrap();

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_custom_params() {
        let password = b"test";
        let salt = generate_salt();
        let params = KdfParams::new(32, 2, 2); // Lower params for faster testing

        let key = derive_key(password, &salt, &params).unwrap();
        assert_eq!(key.len(), 32);
    }
}
