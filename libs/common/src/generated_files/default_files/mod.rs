//! This module provides the [`get_example_content`] and [`get_valid_content`] functions which can be used to get wither the `example file` or `valid file` content of a given
//! [`static's`](crate::generated_files::paths) file.

use std::path::{
    Path,
    PathBuf,
};

use self::{
    example_server_list::EXAMPLE_SERVER_LIST,
    valid_config::VALID_CONFIG,
    valid_server_types::VALID_MCSERVER_TYPES,
};
use super::paths::CONFIG_DIR;
use crate::generated_files::paths::{
    CONFIG_FILE,
    MCSERVER_TYPES_FILE,
    SERVER_LIST_FILE,
};

pub mod example_server_list;
pub mod valid_config;
pub mod valid_server_types;

/// This function will return a tuple with:
///     1. The path to the given [`static's`](crate::generated_files::paths) `example file`.
///     2. The `example file` content of a given [`static's`](crate::generated_files::paths) file.
/// \
/// Use the [`get_valid_content`] function to get the `valid file` content of a given [`static's`](crate::generated_files::paths) file.
///
/// # Panics
///
/// This function will panic if the given [`static`](crate::generated_files::paths) has no `example file` content assigned.
pub fn get_example_content(file_path: &Path) -> (PathBuf, &str) {
    if file_path == SERVER_LIST_FILE.as_path() {
        (
            CONFIG_DIR.join("example_server_list.toml"),
            EXAMPLE_SERVER_LIST,
        )
    } else {
        panic!("The given static's file should not have an 'example file' content.")
    }
}
/// This function will return the `valid file` content of a given [`static's`](crate::generated_files::paths) file.
/// \
/// Use the [`get_example_content`] function to get the `example file` content of a given [`static's`](crate::generated_files::paths) file.
///
/// # Panics
///
/// This function will panic if the given [`static`](crate::generated_files::paths) has no `valid file` content assigned.
pub fn get_valid_content(file_path: &Path) -> &str {
    if file_path == CONFIG_FILE.as_path() {
        VALID_CONFIG
    } else if file_path == MCSERVER_TYPES_FILE.as_path() {
        VALID_MCSERVER_TYPES
    } else {
        panic!("The given static's file should not have a 'valid file' content.")
    }
}
