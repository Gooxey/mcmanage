#![cfg(test)]


use tokio::{time::sleep, fs};

use crate::test_functions::*;

use super::*;


#[tokio::test]
async fn new_config() {
    cleanup();
    setup_logger();

    replace_with_valid_file("config", "config", CONFIG_DEFAULT).await;

    let config = Config::new().await.lock().await.clone();
    let config_default: Config = toml::from_str(CONFIG_DEFAULT).unwrap();

    assert_eq!(config, config_default);

    cleanup();
}
#[tokio::test]
async fn field_gets_changed() {
    cleanup();
    setup_logger();

    replace_with_valid_file("config", "config", CONFIG_DEFAULT).await;

    let config = Config::new().await;
    let mut config_default: Config = toml::from_str(CONFIG_DEFAULT).unwrap();
    config_default.agree_to_eula = false;


    sleep(Duration::new(1, 0)).await;

    let mut config_toml = load_toml_replace("config", "config", CONFIG_DEFAULT, "test", true).await;
    *config_toml.get_mut("agree_to_eula").unwrap() = toml::Value::from(false);

    fs::write("config/config.toml", toml::to_string(&config_toml).unwrap().as_bytes()).await.unwrap();

    sleep(Duration::new(1, 0)).await;

    assert_eq!(*config.lock().await, config_default);

    cleanup();
}
#[tokio::test]
async fn file_gets_invalid() {
    cleanup();
    setup_logger();

    replace_with_valid_file("config", "config", CONFIG_DEFAULT).await;

    let config = Config::new().await;
    let config_default: Config = toml::from_str(CONFIG_DEFAULT).unwrap();


    sleep(Duration::new(1, 0)).await;

    let mut config_toml = load_toml_replace("config", "config", CONFIG_DEFAULT, "test", true).await;
    *config_toml.get_mut("agree_to_eula").unwrap() = toml::Value::from("should be a bool");

    fs::write("config/config.toml", toml::to_string(&config_toml).unwrap().as_bytes()).await.unwrap();

    sleep(Duration::new(1, 0)).await;

    assert_eq!(*config.lock().await, config_default);

    cleanup();
}
#[tokio::test]
async fn file_is_missing() {
    cleanup();
    setup_logger();

    let config = Config::new().await.lock().await.clone();
    let config_default: Config = toml::from_str(CONFIG_DEFAULT).unwrap();

    assert_eq!(config, config_default);

    cleanup();
}