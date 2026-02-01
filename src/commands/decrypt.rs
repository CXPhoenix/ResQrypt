//! Decrypt command implementation
//!
//! Handles the decryption workflow:
//! 1. Read encrypted file
//! 2. Verify header and extract metadata
//! 3. Derive key from password
//! 4. Decrypt with AES-256-GCM
//! 5. Decompress (if was compressed)
//! 6. Extract archive (if was directory)
//! 7. Write output

use std::fs::File;
use std::io::Read;
use std::path::Path;

use rpassword::prompt_password;

use crate::archive::tar::{extract_archive, write_file};
use crate::cli::DecryptArgs;
use crate::compression::decompress;
use crate::crypto::aes::decrypt_data;
use crate::crypto::format::{FileHeader, read_header};
use crate::crypto::kdf::derive_key;
use crate::error::{ResqryptError, Result};
use crate::utils::ProgressReporter;

/// Execute the decrypt command
pub fn execute(args: DecryptArgs) -> Result<()> {
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

    progress.set_message("Reading encrypted file...");

    // Read and parse encrypted file
    let (header, ciphertext) = read_encrypted_file(&args.input)?;

    progress.set_message("Deriving decryption key...");

    // Derive key using params from file header
    let key = derive_key(password.as_bytes(), &header.salt, &header.kdf_params)?;

    progress.set_message("Decrypting...");

    // Decrypt
    let decrypted = decrypt_data(&key, &header.nonce, &ciphertext)?;

    progress.set_message("Processing decrypted data...");

    // Decompress if needed
    let output_data = if header.is_already_zstd() {
        progress.set_message("Original was zstd, preserving format...");
        decrypted
    } else {
        progress.set_message("Decompressing...");
        decompress(&decrypted)?
    };

    progress.set_message("Writing output...");

    // Write output
    if header.is_directory() {
        // Extract tar archive
        extract_archive(&output_data, &args.output)?;
    } else {
        // Write file
        write_file(&args.output, &output_data)?;
    }

    progress.finish("Done!");
    progress.println(format!(
        "✅ Decrypted: {} -> {}",
        args.input.display(),
        args.output.display()
    ));

    if args.verbose {
        let input_size = ciphertext.len() + FileHeader::SIZE;
        let output_size = output_data.len();
        progress.println(format!("   Input: {} bytes, Output: {} bytes", input_size, output_size));

        if header.is_directory() {
            progress.println("   Type: Directory (extracted from archive)");
        } else {
            progress.println("   Type: File");
        }
    }

    Ok(())
}

/// Get password from args or prompt
fn get_password(password_arg: &Option<String>) -> Result<String> {
    match password_arg {
        Some(p) => Ok(p.clone()),
        None => {
            let password = prompt_password("Enter decryption password: ").map_err(|e| {
                ResqryptError::PasswordError(format!("Failed to read password: {}", e))
            })?;

            if password.is_empty() {
                return Err(ResqryptError::PasswordError("Password cannot be empty".to_string()));
            }

            Ok(password)
        }
    }
}

/// Read encrypted file and parse header
fn read_encrypted_file(path: &Path) -> Result<(FileHeader, Vec<u8>)> {
    let mut file = File::open(path)?;

    // Read header
    let header = read_header(&mut file)?;

    // Read remaining ciphertext
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    Ok((header, ciphertext))
}
