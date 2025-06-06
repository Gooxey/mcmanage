//! This module provides the [`Config struct`](Config), which is used all over the [`MCManage network`](https://github.com/Gooxey/MCManage.git) as the application's config.

#![allow(clippy::missing_docs_in_private_items)]

use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use notify::{
    Event,
    ReadDirectoryChangesWatcher,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};
use proc_macros::add_toml_convert;
use tokio::{
    runtime::Runtime,
    sync::{Mutex, MutexGuard},
};
use goolog::*;

use crate::{
    generated_files::{
        load_toml_file::{
            load_toml_replace,
            replace_with_valid_file,
        },
        paths::CONFIG_FILE,
    },
    mcmanage_error::MCManageError,
};

mod tests;

const GOOLOG_CALLER: &str = "Config";
static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

/// This struct represents the config of applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). \
/// See the fields functions for more information on them.
#[add_toml_convert]
pub struct Config {
    agree_to_eula: bool,
    buffsize: usize,
    cooldown: Duration,
    max_tries: u64,
    shutdown_time: Duration,
    website_port: u16,
    database_port: i16
}
impl Config {
    /// This method will do two things:
    ///     1. Create a [`static`](crate::generated_files::paths) holding the application's [`Config`].
    ///     2. Create a [`static`](crate::generated_files::paths) holding the [`watcher`](RecommendedWatcher) responsible for updating every field of the [`Config`] on a change to the file at `config/config.toml`.
    pub async fn init() {
        /// Load the application's [`Config`] from the `config.toml` file
        async fn load_config() -> Config {
            let config_toml = load_toml_replace(&CONFIG_FILE, "Config", true)
                .await
                .to_string();

            if let Ok(config) = toml::from_str(&config_toml) {
                config
            } else {
                replace_with_valid_file(&CONFIG_FILE).await;
                let config_toml = load_toml_replace(&CONFIG_FILE, "Config", true)
                    .await
                    .to_string();

                toml::from_str(&config_toml)
                    .unwrap_or_else(|_| panic!("The default config content should be valid."))
            }
        }
        /// Reload the configs data.
        async fn hot_reload() {
            *Config::get_config().await = load_config().await;
        }

        info!(
            "Initializing..."
        );

        if !CONFIG_FILE.exists() {
            warn!(
                "Could not find a the file at '{}'. A default config file will be generated.",
                CONFIG_FILE.display()
            );
            replace_with_valid_file(&CONFIG_FILE).await;
        }

        if CONFIG.set(load_config().await.into()).is_err() {
            fatal!("The config has already been initiated.")
        }

        static CONFIG_WATCHER: OnceLock<Mutex<ReadDirectoryChangesWatcher>> = OnceLock::new();
        CONFIG_WATCHER
            .get_or_init(|| {
                RecommendedWatcher::new(
                    move |result: Result<Event, notify::Error>| {
                        let event = result.unwrap_or_else(|error| {
                            fatal!("An error occurred while watching the file at '{}'. Error: {error}", CONFIG_FILE.display())
                        });

                        if event.kind.is_modify() {
                            Runtime::new()
                                .unwrap_or_else(|error| {
                                    fatal!("The config failed to start a new tokio runtime. Error: {error}")
                                })
                                .block_on(hot_reload())
                        }
                    },
                    notify::Config::default()
                )
                .unwrap_or_else(|error| {
                    fatal!("An error occurred while creating a watcher for the file at '{}'. Error: {error}", CONFIG_FILE.display())
                })
                .into()
            })
            .lock()
            .await
            .watch(&CONFIG_FILE, RecursiveMode::NonRecursive)
            .unwrap_or_else(|error| {
                fatal!("An error occurred while starting a watcher for the file at '{}'. Error: {error}", CONFIG_FILE.display())
            });
    }

    async fn get_config<'a>() -> MutexGuard<'a, Config> {
        CONFIG
            .get()
            .unwrap_or_else(|| {
                fatal!("The config has to be initialized before reading anything from it.")
            })
            .lock()
            .await
    }

    /// Return whether or not all EULAs for the Minecraft servers get accepted automatically. \
    /// The following line is copied from the vanilla Minecraft server's EULA. \
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    pub async fn agree_to_eula() -> bool {
        Self::get_config().await.agree_to_eula
    }
    /// Return the size of the buffers created by this application. (If set too low, it can cause many different kinds of information to only be partially transmitted.)
    pub async fn buffsize() -> usize {
        Self::get_config().await.buffsize
    }
    /// Return how long the application waits to give other tasks a chance to execute
    pub async fn cooldown() -> Duration {
        Self::get_config().await.cooldown
    }
    /// Return the maximum number of times an operation gets retried.
    pub async fn max_tries() -> u64 {
        Self::get_config().await.max_tries
    }
    /// If no player is playing on any server for that duration, the computer running this application gets shut down. \
    /// If the value is 0, no shutdowns will be performed.
    pub async fn shutdown_time() -> Option<Duration> {
        Self::get_config().await.shutdown_time.into()
    }
    /// The port the website should use.
    pub async fn website_port() -> u16 {
        Self::get_config().await.website_port
    }
    /// The port the database server should use.
    pub async fn database_port() -> i16 {
        Self::get_config().await.database_port
    }
}
