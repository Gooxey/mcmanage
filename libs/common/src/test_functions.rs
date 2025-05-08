//! This module provides various useful functions for tests.

#![allow(unreachable_code)]
#![cfg(test)]

use goolog::*;

use crate::{generated_files::paths::ROOT_DIR, config::Config};

/// This function will call the cleanup function and setup a logger to print log messages to the console.
///
/// # Panics
///
/// This method will panic when called outside of the test configuration.
pub fn start_test() {
    cleanup();
    init_logger(None, None, None);
    Config::init();
}

/// This method will delete everything inside [`struct@ROOT_DIR`](crate::generated_files::paths::ROOT_DIR).
///
/// # Panics
///
/// This method will panic when called outside of the test configuration.
pub fn cleanup() {
    fn cleanup_dir<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();

            if entry.file_type()?.is_dir() {
                cleanup_dir(&path)?;
                if let Err(error) = std::fs::remove_dir(&path) {
                    match error.kind() {
                        std::io::ErrorKind::NotFound => {}
                        _ => {
                            return Err(error);
                        }
                    }
                }
            } else {
                std::fs::remove_file(path)?;
            }
        }
        Ok(())
    }
    cleanup_dir(ROOT_DIR.as_path())
        .unwrap_or_else(|error| fatal!("Cleanup"; "Failed to remove the testing directory. Error: {error}"));
}
