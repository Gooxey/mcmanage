#![cfg(test)]

use tokio::io::AsyncReadExt;

use super::*;
use crate::{
    generated_files::paths::{
        CONFIG_DIR,
        MCSERVER_TYPES_FILE,
    },
    test_functions::{
        cleanup,
        start_test,
    },
};

#[tokio::test]
async fn replace_with_valid_file_no_file_there() {
    start_test();

    replace_with_valid_file(&MCSERVER_TYPES_FILE).await;

    let mut file = OpenOptions::new()
        .read(true)
        .open(MCSERVER_TYPES_FILE.as_path())
        .await
        .unwrap();
    let mut buf = "".to_string();

    file.read_to_string(&mut buf).await.unwrap();

    assert_eq!(buf, get_valid_content(&MCSERVER_TYPES_FILE));

    cleanup();
}
#[tokio::test]
async fn replace_with_valid_file_one_file_there() {
    start_test();

    fs::create_dir(CONFIG_DIR.as_path()).await.unwrap();
    let mut invalid_server_types_file_1 = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(MCSERVER_TYPES_FILE.as_path())
        .await
        .unwrap();
    io::copy(
        &mut "Invalid content 1".as_bytes(),
        &mut invalid_server_types_file_1,
    )
    .await
    .unwrap();

    replace_with_valid_file(&MCSERVER_TYPES_FILE).await;

    let mut file_0 = OpenOptions::new()
        .read(true)
        .open(MCSERVER_TYPES_FILE.as_path())
        .await
        .unwrap();
    let mut file_1 = OpenOptions::new()
        .read(true)
        .open(CONFIG_DIR.join("invalid_server_types.toml"))
        .await
        .unwrap();

    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();

    file_0.read_to_string(&mut buf_0).await.unwrap();
    file_1.read_to_string(&mut buf_1).await.unwrap();

    assert_eq!(buf_0, get_valid_content(&MCSERVER_TYPES_FILE));
    assert_eq!(buf_1, "Invalid content 1");

    cleanup();
}
#[tokio::test]
async fn replace_with_valid_file_two_files_there() {
    start_test();

    fs::create_dir(CONFIG_DIR.as_path()).await.unwrap();
    let mut invalid_server_types_file_1 = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(MCSERVER_TYPES_FILE.as_path())
        .await
        .unwrap();
    let mut invalid_server_types_file_2 = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(CONFIG_DIR.join("invalid_server_types.toml"))
        .await
        .unwrap();
    io::copy(
        &mut "Invalid content 1".as_bytes(),
        &mut invalid_server_types_file_1,
    )
    .await
    .unwrap();
    io::copy(
        &mut "Invalid content 2".as_bytes(),
        &mut invalid_server_types_file_2,
    )
    .await
    .unwrap();

    replace_with_valid_file(&MCSERVER_TYPES_FILE).await;

    let mut file_0 = OpenOptions::new()
        .read(true)
        .open(MCSERVER_TYPES_FILE.as_path())
        .await
        .unwrap();
    let mut file_1 = OpenOptions::new()
        .read(true)
        .open(CONFIG_DIR.join("invalid_server_types.toml"))
        .await
        .unwrap();
    let mut file_2 = OpenOptions::new()
        .read(true)
        .open(CONFIG_DIR.join("invalid_server_types(1).toml"))
        .await
        .unwrap();

    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();
    let mut buf_2 = "".to_string();

    file_0.read_to_string(&mut buf_0).await.unwrap();
    file_1.read_to_string(&mut buf_1).await.unwrap();
    file_2.read_to_string(&mut buf_2).await.unwrap();

    assert_eq!(buf_0, get_valid_content(&MCSERVER_TYPES_FILE));
    assert_eq!(buf_1, "Invalid content 2");
    assert_eq!(buf_2, "Invalid content 1");

    cleanup();
}
