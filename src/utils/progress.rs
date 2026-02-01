//! Progress reporting
//!
//! Provides progress feedback during encryption/decryption operations.

use indicatif::{ProgressBar, ProgressStyle};

/// Progress reporter for CLI operations
pub struct ProgressReporter {
    bar: ProgressBar,
    verbose: bool,
}

impl ProgressReporter {
    /// Create a new progress reporter
    pub fn new(verbose: bool) -> Self {
        let bar = if verbose {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap(),
            );
            pb
        } else {
            ProgressBar::hidden()
        };

        Self { bar, verbose }
    }

    /// Set the current operation message
    pub fn set_message(&self, msg: impl Into<String>) {
        if self.verbose {
            self.bar.set_message(msg.into());
            self.bar.tick();
        }
    }

    /// Mark operation as complete
    pub fn finish(&self, msg: impl Into<String>) {
        if self.verbose {
            self.bar.finish_with_message(msg.into());
        }
    }

    /// Print a message (always shown, not just in verbose mode)
    pub fn println(&self, msg: impl AsRef<str>) {
        println!("{}", msg.as_ref());
    }
}

impl Default for ProgressReporter {
    fn default() -> Self {
        Self::new(false)
    }
}
