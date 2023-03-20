//! This module provides the [`load_toml_file`] function which will load a [`toml object`](Value) from a given file.


use std::{
    fs::{
        File,
        self
    },
    io::{
        ErrorKind,
        self
    },
    path::Path
};

use toml::Table;

use crate::{
    erro,
    mcmanage_error::MCManageError,
    warn
};


mod tests;


/// This function will load a [`toml table`](Table) from a given file. \
/// \
/// In case the given file is invalid, this function can do one of the following two things:
///     1. A example file will be generated, and the invalid file will be renamed.
///     2. The invalid file will be replaced with a valid one.
/// 
/// # Parameters
/// 
/// | Parameter               | Description                                                                                      |
/// |-------------------------|--------------------------------------------------------------------------------------------------|
/// | `caller_name: &str`     | This is the name by which log messages get written.                                              |
/// | `path: &str`            | This is the path TO the file. ( The path for the file 'logs/server1/log.txt' is 'logs/server1' ) |
/// | `file_name: &str`       | This is the name of the file. ( The name for the file 'logs/server1/log.txt' is 'log' )          |
/// | `valid_data: &str`      | This is the string that will be written to the example/valid file.                               |
/// | `replace_invalid: bool` | Whenever this gets set to true, this function will go with the second option described above.    |
pub fn load_toml_file(caller_name: &str, path: &str, file_name: &str, valid_data: &str, replace_invalid: bool) -> Result<Table, MCManageError> {
    let destination = format!("{path}/{file_name}.toml");

    if replace_invalid {
        if let Ok(toml_string) = fs::read_to_string(&destination) {
            if let Ok(toml) = toml::from_str(&toml_string) {
                return Ok(toml);
            }
        }
        replace_with_valid_file(valid_data, path, file_name);
        load_toml_file(caller_name, path, file_name, valid_data, replace_invalid)
    } else {
        let example_destination = format!("{path}/{file_name}_example.toml");

        match fs::read_to_string(&destination) {
            Ok(toml_string) => {
                if let Ok(toml) = toml::from_str(&toml_string) {
                    Ok(toml)
                } else {
                    erro!(caller_name, "The file at {destination} is invalid. A valid example will be generated under {example_destination}.");
                    generate_valid_file(valid_data, path, file_name);
                    Err(MCManageError::InvalidFile)
                }
            }
            Err(erro) => {
                if let ErrorKind::NotFound = erro.kind() {
                    if Path::new(&example_destination).exists() {
                        warn!(caller_name, "You need to configure the file at {destination}.");
                        warn!(caller_name, "For a valid write style, see the file at {example_destination}.");
                        return Err(MCManageError::IOError(erro));
                    } else {
                        erro!(caller_name, "The file at {destination} could not be found. A valid example will be generated under {example_destination}.");
                    }
                } else {
                    erro!(caller_name, "An error occurred while opening the file at {destination}. A valid example will be generated under {example_destination}.");
                }
                generate_valid_file(valid_data, path, file_name);
                Err(MCManageError::IOError(erro))
            }
        }
    }
}
/// This function generates an example file and renames the invalid file.
pub fn generate_valid_file(valid_data: &str, path: &str, file_name: &str) {
    let mut invalid_file_name;
    let mut i = 0;
    loop {
        if i == 0 {
            invalid_file_name = format!("{path}/invalid_{file_name}.toml");
        } else {
            invalid_file_name = format!("{path}/invalid_{file_name}({}).toml", i);
        }
        if !Path::new(&invalid_file_name).exists() {
            if fs::rename(format!("{path}/{file_name}.toml"), &invalid_file_name).is_err() {
                if let Err(erro) = fs::create_dir(path) {
                    if let ErrorKind::AlreadyExists = erro.kind() {
                    } else {
                        panic!("An error occurred while trying to create the folder at {path}. Error: {erro}")
                    }
                }
            }
            break;
        } else {
            i += 1;
        }
    }

    let example_destination = format!("{path}/{file_name}_example.toml");
    let mut example_file;
    match File::options().write(true).create(true).open(&example_destination) {
        Ok(file) => {
            example_file = file;
        }
        Err(erro) => {
            panic!("Could not open the file at {example_destination}. Error: {erro}")
        }
    }
    io::copy(
        &mut valid_data.as_bytes(),
        &mut example_file
    ).unwrap_or_else(|_| panic!("Failed to copy the 'valid_data' to the file at {example_destination}."));
}
/// This function replaces the invalid file with a valid one.
pub fn replace_with_valid_file(valid_data: &str, path: &str, file_name: &str) {
    let destination = format!("{path}/{file_name}.toml");
    let mut invalid_file_name;
    let mut i = 0;
    loop {
        if i == 0 {
            invalid_file_name = format!("{path}/invalid_{file_name}.toml");
        } else {
            invalid_file_name = format!("{path}/invalid_{file_name}({}).toml", i);
        }
        if !Path::new(&invalid_file_name).exists() {
            if fs::rename(&destination, &invalid_file_name).is_err() {
                if let Err(erro) = fs::create_dir(path) {
                    if let ErrorKind::AlreadyExists = erro.kind() {
                    } else {
                        panic!("An error occurred while trying to create the folder at {path}. Error: {erro}")
                    }
                }
            }
            break;
        } else {
            i += 1;
        }
    }

    let mut valid_file;
    match File::options().write(true).create(true).open(&destination) {
        Ok(file) => {
            valid_file = file;
        }
        Err(erro) if ErrorKind::NotFound == erro.kind() => {
            if let Err(erro) = fs::create_dir(path) {
                if let ErrorKind::AlreadyExists = erro.kind() {
                } else {
                    panic!("An error occurred while trying to create the folder at {path}. Error: {erro}")
                }
            }
            return replace_with_valid_file(valid_data, path, file_name);
        }
        Err(erro) => {
            panic!("Could not open the file at {destination}. Error: {erro}")
        }
    }
    io::copy(
        &mut valid_data.as_bytes(),
        &mut valid_file
    ).unwrap_or_else(|_| panic!("Failed to copy the 'valid_data' to the file at {destination}."));
}