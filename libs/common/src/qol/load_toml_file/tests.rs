#![cfg(test)]


use std::io::Read;

use crate::{
    mcserver_manager::mcserver::mcserver_type::mcserver_types_default::MCSERVER_TYPES_DEFAULT,
    test_functions::cleanup
};

use super::*;


#[test]
fn replace_with_valid_file_no_file_there() {
    cleanup();

    replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");

    let mut file = File::options().read(true).open("config/mcserver_types.toml").unwrap();
    let mut buf = "".to_string();

    file.read_to_string(&mut buf).unwrap();

    assert_eq!(buf, MCSERVER_TYPES_DEFAULT);

    cleanup();
}
#[test]
fn replace_with_valid_file_one_file_there() {
    cleanup();
    
    fs::create_dir("config").unwrap();
    let mut invalid_mcserver_types_file_1 = File::options().write(true).create_new(true).open("config/mcserver_types.toml").unwrap();
    io::copy(&mut "Invalid content 1".as_bytes(), &mut invalid_mcserver_types_file_1).unwrap();

    replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");

    let mut file_0 = File::options().read(true).open("config/mcserver_types.toml").unwrap();
    let mut file_1 = File::options().read(true).open("config/invalid_mcserver_types.toml").unwrap();
    
    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();

    file_0.read_to_string(&mut buf_0).unwrap();
    file_1.read_to_string(&mut buf_1).unwrap();

    assert_eq!(buf_0, MCSERVER_TYPES_DEFAULT);
    assert_eq!(buf_1, "Invalid content 1");

    cleanup();
}
#[test]
fn replace_with_valid_file_two_files_there() {
    cleanup();

    fs::create_dir("config").unwrap();
    let mut invalid_mcserver_types_file_1 = File::options().write(true).create_new(true).open("config/mcserver_types.toml").unwrap();
    let mut invalid_mcserver_types_file_2 = File::options().write(true).create_new(true).open("config/invalid_mcserver_types.toml").unwrap();
    io::copy(&mut "Invalid content 1".as_bytes(), &mut invalid_mcserver_types_file_1).unwrap();
    io::copy(&mut "Invalid content 2".as_bytes(), &mut invalid_mcserver_types_file_2).unwrap();

    replace_with_valid_file(MCSERVER_TYPES_DEFAULT, "config", "mcserver_types");

    let mut file_0 = File::options().read(true).open("config/mcserver_types.toml").unwrap();
    let mut file_1 = File::options().read(true).open("config/invalid_mcserver_types.toml").unwrap();
    let mut file_2 = File::options().read(true).open("config/invalid_mcserver_types(1).toml").unwrap();
    
    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();
    let mut buf_2 = "".to_string();

    file_0.read_to_string(&mut buf_0).unwrap();
    file_1.read_to_string(&mut buf_1).unwrap();
    file_2.read_to_string(&mut buf_2).unwrap();

    assert_eq!(buf_0, MCSERVER_TYPES_DEFAULT);
    assert_eq!(buf_1, "Invalid content 2");
    assert_eq!(buf_2, "Invalid content 1");
    
    cleanup();
}