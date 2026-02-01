//! Resqrypt - Secure file and directory encryption CLI
//!
//! Entry point for the resqrypt command-line application.

use anyhow::Result;
use clap::Parser;

use resqrypt::cli::{Cli, Commands};
use resqrypt::commands;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Encrypt(args) => commands::encrypt(args),
        Commands::Decrypt(args) => commands::decrypt(args),
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
