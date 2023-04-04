#![cfg(test)]

use tokio::{
    fs,
    time::sleep,
};

use super::*;
use crate::{
    generated_files::default_files::get_valid_content,
    test_functions::*,
};

#[tokio::test]
async fn new_config() {
    start_test();

    replace_with_valid_file(&CONFIG_FILE).await;

    let config = Config::new().await.lock().await.clone();
    let config_default: Config = toml::from_str(get_valid_content(&CONFIG_FILE)).unwrap();

    assert_eq!(config, config_default);

    cleanup();
}
#[tokio::test]
async fn field_gets_changed() {
    start_test();

    replace_with_valid_file(&CONFIG_FILE).await;

    let config = Config::new().await;
    let mut config_default: Config = toml::from_str(get_valid_content(&CONFIG_FILE)).unwrap();
    config_default.agree_to_eula = false;

    sleep(Duration::new(1, 0)).await;

    let mut config_toml = load_toml_replace(&CONFIG_FILE, "test", true).await;
    *config_toml.get_mut("agree_to_eula").unwrap() = toml::Value::from(false);

    fs::write(
        CONFIG_FILE.as_path(),
        toml::to_string(&config_toml).unwrap().as_bytes(),
    )
    .await
    .unwrap();

    sleep(Duration::new(1, 0)).await;

    assert_eq!(*config.lock().await, config_default);

    cleanup();
}
#[tokio::test]
async fn file_gets_invalid() {
    start_test();

    replace_with_valid_file(&CONFIG_FILE).await;

    let config = Config::new().await;
    let config_default: Config = toml::from_str(get_valid_content(&CONFIG_FILE)).unwrap();

    sleep(Duration::new(1, 0)).await;

    let mut config_toml = load_toml_replace(&CONFIG_FILE, "test", true).await;
    *config_toml.get_mut("agree_to_eula").unwrap() = toml::Value::from("should be a bool");

    fs::write(
        CONFIG_FILE.as_path(),
        toml::to_string(&config_toml).unwrap().as_bytes(),
    )
    .await
    .unwrap();

    sleep(Duration::new(1, 0)).await;

    assert_eq!(*config.lock().await, config_default);

    cleanup();
}
#[tokio::test]
async fn file_is_missing() {
    start_test();

    let config = Config::new().await.lock().await.clone();
    let config_default: Config = toml::from_str(get_valid_content(&CONFIG_FILE)).unwrap();

    assert_eq!(config, config_default);

    cleanup();
}
