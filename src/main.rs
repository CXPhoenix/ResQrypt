//! Resqrypt - Secure file and directory encryption CLI
//!
//! Entry point for the resqrypt command-line application.

use anyhow::Result;

fn main() -> Result<()> {
    println!("resqrypt v{}", env!("CARGO_PKG_VERSION"));
    println!("Secure file and directory encryption tool");
    println!();
    println!("Use --help for usage information.");

    Ok(())
}
