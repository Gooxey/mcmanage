//! This module provides the [`Config struct`](Config), which is used all over the [`MCManage network`](https://github.com/Gooxey/MCManage.git) as the application's config.


#![allow(clippy::missing_docs_in_private_items)]


use std::time::Duration;

use proc_macros::add_toml_convert;

use crate::mcmanage_error::MCManageError;
use crate::qol::load_toml_file::{
    load_toml_replace,
    replace_with_valid_file
};

use self::config_default::CONFIG_DEFAULT;


mod config_default;


/// This struct represents the config of applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). \
/// See the fields functions for more information on them.
#[add_toml_convert]
struct Config {
    agree_to_eula: bool,
    buffsize: usize,
    communicator_port: u16,
    cooldown: Duration,
    max_tries: u64,
    shutdown_time: Duration,
    website_port: u16,
}


/// Load the application's [`Config`] from the `config.toml` file
async fn load_config() -> Config {
    let config_toml = load_toml_replace(
        "config",
        "config",
        CONFIG_DEFAULT,
        "Config",
        true
    ).await
    .to_string();

    if let Ok(config) = toml::from_str(&config_toml) {
        config
    } else {
        replace_with_valid_file("config", "config", CONFIG_DEFAULT).await;
        let config_toml = load_toml_replace(
            "config",
            "config",
            CONFIG_DEFAULT,
            "Config",
            true
        ).await
        .to_string();

        toml::from_str(&config_toml).unwrap_or_else(|_| panic!("The default config content should be valid."))
    }
}

/// Return whether or not all EULAs for the Minecraft servers get accepted automatically. \
/// The following line is copied from the vanilla Minecraft server's EULA. \
/// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
/// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
pub async fn agree_to_eula() -> bool {
    load_config().await
        .agree_to_eula
}
/// Return the size of the buffers created by this application. (If set too low, it can cause many different kinds of information to only be partially transmitted.)
pub async fn buffsize() -> usize {
    load_config().await
        .buffsize
}
/// Return the port the communicator should use.
pub async fn communicator_port() -> u16 {
    load_config().await
        .communicator_port
}
/// Return how long the application waits to give other tasks a chance to execute
pub async fn cooldown() -> Duration {
    load_config().await
        .cooldown
}
/// Return the maximum number of times an operation gets retried.
pub async fn max_tries() -> u64 {
    load_config().await
        .max_tries
}
/// If no player is playing on any server for that duration, the computer running this application gets shut down. \
/// If the value is 0, no shutdowns will be performed.
pub async fn shutdown_time() -> Duration {
    load_config().await
        .shutdown_time
}
/// The port the website should use.
pub async fn website_port() -> u16 {
    load_config().await
        .website_port
}