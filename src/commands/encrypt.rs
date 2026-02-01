//! Encrypt command implementation
//!
//! Handles the encryption workflow:
//! 1. Read input (file or directory)
//! 2. Archive if directory
//! 3. Compress (if not already zstd)
//! 4. Encrypt with AES-256-GCM
//! 5. Write output with header

use std::fs;
use std::io::Write;
use std::path::Path;

use rpassword::prompt_password;

use crate::archive::tar::{create_archive, read_file};
use crate::cli::EncryptArgs;
use crate::compression::{compress, is_zstd_compressed};
use crate::crypto::aes::{encrypt_data, generate_nonce};
use crate::crypto::format::{FileHeader, write_header};
use crate::crypto::kdf::{KdfParams, derive_key, generate_salt};
use crate::error::{ResqryptError, Result};
use crate::flags;
use crate::utils::ProgressReporter;

/// Execute the encrypt command
pub fn execute(args: EncryptArgs) -> Result<()> {
    let progress = ProgressReporter::new(args.verbose);

    // Validate input exists
    if !args.input.exists() {
        return Err(ResqryptError::NotFound(args.input.clone()));
    }

    // Check if output already exists
    if args.output.exists() {
        return Err(ResqryptError::AlreadyExists(args.output.clone()));
    }

    // Get password
    let password = get_password(&args.password)?;

    progress.set_message("Reading input...");

    // Read input data
    let (data, mut file_flags) = read_input(&args.input)?;
    let input_size = data.len();

    progress.set_message("Processing data...");

    // Check if already zstd compressed
    let data_to_encrypt = if is_zstd_compressed(&data) {
        progress.set_message("Detected zstd format, skipping compression...");
        file_flags |= flags::ALREADY_ZSTD;
        data
    } else {
        progress.set_message("Compressing...");
        compress(&data)?
    };

    progress.set_message("Deriving encryption key...");

    // Setup KDF parameters
    let kdf_params =
        KdfParams::new(args.argon2_memory, args.argon2_iterations, args.argon2_parallelism);

    // Generate salt and derive key
    let salt = generate_salt();
    let key = derive_key(password.as_bytes(), &salt, &kdf_params)?;

    progress.set_message("Encrypting...");

    // Generate nonce and encrypt
    let nonce = generate_nonce();
    let ciphertext = encrypt_data(&key, &nonce, &data_to_encrypt)?;

    progress.set_message("Writing output...");

    // Write output file
    write_encrypted_file(&args.output, file_flags, &kdf_params, &salt, &nonce, &ciphertext)?;

    progress.finish("Done!");
    progress.println(format!(
        "✅ Encrypted: {} -> {}",
        args.input.display(),
        args.output.display()
    ));

    if args.verbose {
        let output_size = ciphertext.len() + FileHeader::SIZE;
        let ratio = (output_size as f64 / input_size as f64) * 100.0;
        progress.println(format!(
            "   Input: {} bytes, Output: {} bytes ({:.1}%)",
            input_size, output_size, ratio
        ));
    }

    Ok(())
}

/// Get password from args or prompt
fn get_password(password_arg: &Option<String>) -> Result<String> {
    match password_arg {
        Some(p) => Ok(p.clone()),
        None => {
            let password = prompt_password("Enter encryption password: ").map_err(|e| {
                ResqryptError::PasswordError(format!("Failed to read password: {}", e))
            })?;

            if password.is_empty() {
                return Err(ResqryptError::PasswordError("Password cannot be empty".to_string()));
            }

            // Confirm password
            let confirm = prompt_password("Confirm password: ").map_err(|e| {
                ResqryptError::PasswordError(format!("Failed to read password: {}", e))
            })?;

            if password != confirm {
                return Err(ResqryptError::PasswordError("Passwords do not match".to_string()));
            }

            Ok(password)
        }
    }
}

/// Read input file or directory
fn read_input(path: &Path) -> Result<(Vec<u8>, u8)> {
    if path.is_dir() {
        // Create tar archive from directory
        let archive_data = create_archive(path)?;
        Ok((archive_data, flags::IS_DIRECTORY))
    } else {
        // Read file
        let file_data = read_file(path)?;
        Ok((file_data, 0))
    }
}

/// Write the encrypted output file
fn write_encrypted_file(
    path: &Path,
    flags: u8,
    kdf_params: &KdfParams,
    salt: &[u8; 32],
    nonce: &[u8; 12],
    ciphertext: &[u8],
) -> Result<()> {
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(path)?;

    // Write header
    let header = FileHeader::new(flags, kdf_params.clone(), *salt, *nonce);
    write_header(&mut file, &header)?;

    // Write ciphertext
    file.write_all(ciphertext)?;

    Ok(())
}
