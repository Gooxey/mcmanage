//! This module provides functions used in the context of loading toml files.

use std::{
    io::ErrorKind,
    path::Path,
};

use tokio::{
    fs::{
        self,
        OpenOptions,
    },
    io,
};
use toml::Table;

use goolog::*;

use super::default_files::{
    get_example_content,
    get_valid_content,
};
use crate::{
    generated_files::paths::{
        get_example_path,
        get_invalid_path,
    },
    mcmanage_error::MCManageError,
};

mod tests;

// TODO only rename the file if the latest rename is different

/// This function will load a [`toml table`](Table) from a given file. \
/// In the event that the given file is invalid, this function will generate an example file and rename the invalid file using the [`generate_example_file`] function.
pub async fn load_toml(
    file_path: &Path,
    caller_name: &str,
    log: bool,
) -> Result<Table, MCManageError> {
    match fs::read_to_string(file_path).await {
        Ok(toml_string) => {
            if let Ok(toml) = toml::from_str(&toml_string) {
                return Ok(toml);
            }
        }
        Err(error) if ErrorKind::NotFound == error.kind() => {}
        Err(error) => {
            panic!(
                "An error occurred while opening the file at {}. Error: {error}",
                file_path.display()
            )
        }
    }
    generate_example_file(file_path, caller_name, log).await;
    Err(MCManageError::InvalidFile)
}
/// This function will load a [`toml table`](Table) from a given file. \
/// In the event that the given file is invalid, this function will replace it with a valid one using the [`replace_with_valid_file`] function.
pub async fn load_toml_replace(file_path: &Path, caller_name: &str, log: bool) -> Table {
    loop {
        // Read the toml file
        if let Ok(toml_string) = fs::read_to_string(file_path).await {
            if let Ok(toml) = toml::from_str(&toml_string) {
                return toml;
            }
        }

        if log {
            warn!(
                caller_name;
                "The file '{}' is invalid. It will be replaced by the default file.",
                file_path.display()
            );
        }
        replace_with_valid_file(file_path).await;
    }
}
/// This function renames the invalid file and generates an example file.
pub async fn generate_example_file(file_path: &Path, caller_name: &str, log: bool) {
    let example_path = get_example_path(file_path).await;
    let example_content = get_example_content(file_path).1;

    if example_path.exists() {
        if log {
            error!(
                caller_name;
                "You need to configure the file at {}.",
                file_path.display()
            );
            error!(
                caller_name;
                "For a valid write style, see the file at {}.",
                example_path.display()
            );
        }
        return;
    } else if log {
        error!(
            caller_name;
            "The file at {} is invalid. A valid example will be generated under {}.",
            file_path.display(),
            example_path.display()
        );
    }

    let mut example_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&example_path)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "Could not open the file at {}. Error: {error}",
                example_path.display()
            )
        });

    io::copy(&mut example_content.as_bytes(), &mut example_file)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "Failed to copy the 'valid_content' to the file at {}. Error: {error}",
                example_path.display()
            )
        });
}
/// This function renames the invalid file and generates a valid one.
pub async fn replace_with_valid_file(file_path: &Path) {
    let invalid_file_path = get_invalid_path(file_path).await;
    let valid_content = get_valid_content(file_path);

    if file_path.exists() {
        fs::rename(&file_path, &invalid_file_path)
            .await
            .unwrap_or_else(|error| {
                panic!(
                    "An error occurred while renaming the file at '{}' to '{}'. Error: {error}",
                    file_path.display(),
                    invalid_file_path.display()
                )
            });
    }

    let mut valid_file;
    match OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)
        .await
    {
        Ok(file) => {
            valid_file = file;
        }
        Err(error) => {
            panic!(
                "Could not open the file at '{}'. Error: {error}",
                file_path.display()
            )
        }
    }
    io::copy(&mut valid_content.as_bytes(), &mut valid_file)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to copy the 'valid_content' to the file at '{}'.",
                file_path.display()
            )
        });
}
