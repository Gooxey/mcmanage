//! This module provides functions used in the context of loading toml files.


use std::{
    io::ErrorKind,
    path::Path
};

use tokio::{
    fs::{
        OpenOptions,
        self
    },
    io
};
use toml::Table;

use crate::{
    error,
    mcmanage_error::MCManageError, warn
};


mod tests;

// TODO only rename the file if the latest rename is different


/// This function will load a [`toml table`](Table) from a given file. \
/// In the event that the given file is invalid, this function will generate an example file and rename the invalid file using the [`generate_example_file`] function.
/// 
/// # Parameters
/// 
/// | Parameter               | Description                                                                                      |
/// |-------------------------|--------------------------------------------------------------------------------------------------|
/// | `caller_name: &str`     | This is the name by which log messages get written.                                              |
/// | `path: &str`            | This is the path TO the file. ( The path for the file 'logs/server1/log.txt' is 'logs/server1' ) |
/// | `file_name: &str`       | This is the name of the file. ( The name for the file 'logs/server1/log.txt' is 'log' )          |
/// | `valid_data: &str`      | This is the string that will be written to the example/valid file.                               |
pub async fn load_toml(path: &str, file_name: &str, valid_data: &str, caller_name: &str, log: bool) -> Result<Table, MCManageError> {
    let destination = format!("{path}/{file_name}.toml");

    match fs::read_to_string(&destination).await {
        Ok(toml_string) => {
            if let Ok(toml) = toml::from_str(&toml_string) {
                return Ok(toml);
            }
        }
        Err(erro) if ErrorKind::NotFound == erro.kind() => {}
        Err(erro) => {
            panic!("An error occurred while opening the file at {destination}. Error: {erro}")
        }
    }
    generate_example_file(path, file_name, valid_data, caller_name, log).await;
    Err(MCManageError::InvalidFile)
}
/// This function will load a [`toml table`](Table) from a given file. \
/// In the event that the given file is invalid, this function will replace it with a valid one using the [`replace_with_valid_file`] function.
/// 
/// # Parameters
/// 
/// | Parameter               | Description                                                                                      |
/// |-------------------------|--------------------------------------------------------------------------------------------------|
/// | `path: &str`            | This is the path TO the file. ( The path for the file 'logs/server1/log.txt' is 'logs/server1' ) |
/// | `file_name: &str`       | This is the name of the file. ( The name for the file 'logs/server1/log.txt' is 'log' )          |
/// | `valid_data: &str`      | This is the string that will be written to the example/valid file.                               |
pub async fn load_toml_replace(path: &str, file_name: &str, valid_data: &str, caller_name: &str, log: bool) -> Table {
    loop {
        // Read the toml file
        if let Ok(toml_string) = fs::read_to_string(&format!("{path}/{file_name}.toml")).await {
            if let Ok(toml) = toml::from_str(&toml_string) {
                return toml;
            }
        }

        if log { warn!(caller_name, "The file '{path}/{file_name}.toml' is invalid. It will be replaced by the default file."); }
        replace_with_valid_file(path, file_name, valid_data).await;
    }
}
/// This function renames the invalid file and generates an example file.
pub async fn generate_example_file(path: &str, file_name: &str, valid_data: &str, caller_name: &str, log: bool) {
    let destination = format!("{path}/{file_name}.toml");
    let example_destination = format!("{path}/{file_name}_example.toml");

    fs::create_dir_all("config").await
        .unwrap_or_else(|erro| panic!("An error occurred while trying to create the folder at 'config'. Error: {erro}"));

    if Path::new(&example_destination).exists() {
        if log { error!(caller_name, "You need to configure the file at {destination}."); }
        if log { error!(caller_name, "For a valid write style, see the file at {example_destination}."); }
        return;
    } else if log {
        error!(caller_name, "The file at {destination} is invalid. A valid example will be generated under {example_destination}.");
    }

    let mut example_file = OpenOptions::new().write(true).create(true).open(&example_destination).await
        .unwrap_or_else(|erro| panic!("Could not open the file at {example_destination}. Error: {erro}"));

    io::copy(
        &mut valid_data.as_bytes(),
        &mut example_file
    ).await.unwrap_or_else(|erro| panic!("Failed to copy the 'valid_data' to the file at {example_destination}. Error: {erro}"));
}
/// This function renames the invalid file and generates a valid one.
pub async fn replace_with_valid_file(path: &str, file_name: &str, valid_data: &str) {
    let destination = format!("{path}/{file_name}.toml");
    let mut invalid_file_name;
    for i in 0.. {
        if i == 0 {
            invalid_file_name = format!("{path}/invalid_{file_name}.toml");
        } else {
            invalid_file_name = format!("{path}/invalid_{file_name}({}).toml", i);
        }
        if !Path::new(&invalid_file_name).exists() {
            if fs::rename(&destination, &invalid_file_name).await.is_err() {
                fs::create_dir_all(path).await
                    .unwrap_or_else(|erro| panic!("An error occurred while trying to create the folder at '{path}'. Error: {erro}"));
            }
            break;
        }
    }

    let mut valid_file;
    match OpenOptions::new().write(true).create(true).open(&destination).await {
        Ok(file) => {
            valid_file = file;
        }
        Err(erro) if ErrorKind::NotFound == erro.kind() => {
            if let Err(erro) = fs::create_dir_all(path).await {
                panic!("An error occurred while trying to create the folder at '{path}'. Error: {erro}")
            }
            valid_file = OpenOptions::new().write(true).create(true).open(&destination).await
                .unwrap_or_else(|erro| panic!("Could not open the file at '{destination}'. Error: {erro}"));
        }
        Err(erro) => {
            panic!("Could not open the file at '{destination}'. Error: {erro}")
        }
    }
    io::copy(
        &mut valid_data.as_bytes(),
        &mut valid_file
    ).await.unwrap_or_else(|_| panic!("Failed to copy the 'valid_data' to the file at '{destination}'."));
}