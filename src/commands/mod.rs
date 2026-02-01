//! Command implementations
//!
//! Contains the encrypt and decrypt command logic.

pub mod decrypt;
pub mod encrypt;

pub use decrypt::execute as decrypt;
pub use encrypt::execute as encrypt;
