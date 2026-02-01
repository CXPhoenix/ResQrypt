//! Archive operations module
//!
//! Provides tar archive creation and extraction for directory encryption.

pub mod tar;

pub use tar::{create_archive, extract_archive};
