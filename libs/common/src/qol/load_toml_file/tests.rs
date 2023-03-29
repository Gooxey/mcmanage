#![cfg(test)]


use tokio::io::AsyncReadExt;

use crate::{
    mcserver_manager::mcserver::mcserver_type::mcserver_types_default::MCSERVER_TYPES_DEFAULT,
    test_functions::cleanup
};

use super::*;


#[tokio::test]
async fn replace_with_valid_file_no_file_there() {
    cleanup();

    replace_with_valid_file("config", "mcserver_types", MCSERVER_TYPES_DEFAULT).await;

    let mut file = OpenOptions::new().read(true).open("config/mcserver_types.toml").await.unwrap();
    let mut buf = "".to_string();

    file.read_to_string(&mut buf).await.unwrap();

    assert_eq!(buf, MCSERVER_TYPES_DEFAULT);

    cleanup();
}
#[tokio::test]
async fn replace_with_valid_file_one_file_there() {
    cleanup();

    fs::create_dir("config").await.unwrap();
    let mut invalid_mcserver_types_file_1 = OpenOptions::new().write(true).create_new(true).open("config/mcserver_types.toml").await.unwrap();
    io::copy(&mut "Invalid content 1".as_bytes(), &mut invalid_mcserver_types_file_1).await.unwrap();

    replace_with_valid_file("config", "mcserver_types", MCSERVER_TYPES_DEFAULT).await;

    let mut file_0 = OpenOptions::new().read(true).open("config/mcserver_types.toml").await.unwrap();
    let mut file_1 = OpenOptions::new().read(true).open("config/invalid_mcserver_types.toml").await.unwrap();

    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();

    file_0.read_to_string(&mut buf_0).await.unwrap();
    file_1.read_to_string(&mut buf_1).await.unwrap();

    assert_eq!(buf_0, MCSERVER_TYPES_DEFAULT);
    assert_eq!(buf_1, "Invalid content 1");

    cleanup();
}
#[tokio::test]
async fn replace_with_valid_file_two_files_there() {
    cleanup();

    fs::create_dir("config").await.unwrap();
    let mut invalid_mcserver_types_file_1 = OpenOptions::new().write(true).create_new(true).open("config/mcserver_types.toml").await.unwrap();
    let mut invalid_mcserver_types_file_2 = OpenOptions::new().write(true).create_new(true).open("config/invalid_mcserver_types.toml").await.unwrap();
    io::copy(&mut "Invalid content 1".as_bytes(), &mut invalid_mcserver_types_file_1).await.unwrap();
    io::copy(&mut "Invalid content 2".as_bytes(), &mut invalid_mcserver_types_file_2).await.unwrap();

    replace_with_valid_file("config", "mcserver_types", MCSERVER_TYPES_DEFAULT).await;

    let mut file_0 = OpenOptions::new().read(true).open("config/mcserver_types.toml").await.unwrap();
    let mut file_1 = OpenOptions::new().read(true).open("config/invalid_mcserver_types.toml").await.unwrap();
    let mut file_2 = OpenOptions::new().read(true).open("config/invalid_mcserver_types(1).toml").await.unwrap();

    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();
    let mut buf_2 = "".to_string();

    file_0.read_to_string(&mut buf_0).await.unwrap();
    file_1.read_to_string(&mut buf_1).await.unwrap();
    file_2.read_to_string(&mut buf_2).await.unwrap();

    assert_eq!(buf_0, MCSERVER_TYPES_DEFAULT);
    assert_eq!(buf_1, "Invalid content 2");
    assert_eq!(buf_2, "Invalid content 1");

    cleanup();
}