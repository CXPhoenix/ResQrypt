//! CLI definition using clap
//!
//! Defines the command-line interface for resqrypt.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::kdf_defaults;

/// Resqrypt - Secure file and directory encryption
#[derive(Parser, Debug)]
#[command(name = "resqrypt")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encrypt a file or directory
    Encrypt(EncryptArgs),
    /// Decrypt a file or directory
    Decrypt(DecryptArgs),
}

/// Arguments for the encrypt command
#[derive(Parser, Debug)]
pub struct EncryptArgs {
    /// Input file or directory path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output encrypted file path (.resqrypt)
    #[arg(short, long)]
    pub output: PathBuf,

    /// Encryption password (will prompt if not provided)
    #[arg(short, long, env = "RESQRYPT_PASSWORD")]
    pub password: Option<String>,

    /// Argon2id memory cost in MB
    #[arg(long, default_value_t = kdf_defaults::MEMORY_COST / 1024)]
    pub argon2_memory: u32,

    /// Argon2id iteration count
    #[arg(long, default_value_t = kdf_defaults::TIME_COST)]
    pub argon2_iterations: u32,

    /// Argon2id parallelism degree
    #[arg(long, default_value_t = kdf_defaults::PARALLELISM)]
    pub argon2_parallelism: u32,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

/// Arguments for the decrypt command
#[derive(Parser, Debug)]
pub struct DecryptArgs {
    /// Input encrypted file path (.resqrypt)
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output file or directory path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Decryption password (will prompt if not provided)
    #[arg(short, long, env = "RESQRYPT_PASSWORD")]
    pub password: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
