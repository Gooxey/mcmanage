//! This module provides the [`Config struct`](Config), which is used all over the [`MCManage network`](https://github.com/Gooxey/MCManage.git) as the application's config.


use std::time::Duration;

use proc_macros::add_convert;
use tokio::sync::Mutex;

use crate::mcmanage_error::MCManageError;
use crate::qol::load_toml_file::{load_toml_file, replace_with_valid_file};

use self::config_default::CONFIG_DEFAULT;


mod config_default;


/// This struct represents the config of applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).
#[add_convert]
pub struct Config {
    /// Sets whether or not all EULAs for the Minecraft servers get accepted automatically. \
    /// The following line is copied from the vanilla Minecraft server's EULA. \
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    agree_to_eula: bool,
    /// The size of the buffers created by this application. (If set too low, it can cause many different kinds of information to only be partially transmitted.)
    buffsize: usize,
    /// The port the communicator should use.
    communicator_port: u16,
    /// Sets how long the application wait to give other tasks a chance to execute.
    cooldown: Duration,
    /// The maximum number of times an operation gets retried.
    max_tries: u64,
    /// The amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). \
    /// If the value is 0, no restarts will be performed.
    mcserver_restart_time: Duration,
    /// If no player is playing on any server for that duration, the computer running this application gets shut down. \
    /// If the value is 0, no shutdowns will be performed.
    shutdown_time: Duration,
    /// The port the website should use.
    website_port: u16,
}
impl Config {
    /// Create a new [`Config`] instance.
    pub fn new() -> Mutex<Self> {
        Config::load_config().into()
    }
    /// Load the applications config from the `config.toml` file
    fn load_config() -> Self {
        let config_toml;
        if let Ok(toml) = load_toml_file(
            "Main",
            "config",
            "config",
            CONFIG_DEFAULT,
            true
        ) {
            config_toml = toml.to_string();
        } else {
            config_toml = load_toml_file(
                "Main",
                "config",
                "config",
                CONFIG_DEFAULT,
                true
            ).unwrap_or_else(|_| panic!("The first call to load_toml_file should have fixed the config.toml file"))
            .to_string();
        }
        
        if let Ok(config) = toml::from_str(&config_toml) {
            config
        } else {
            replace_with_valid_file(CONFIG_DEFAULT, "config", "config");
            toml::from_str(&config_toml).unwrap_or_else(|_| panic!("The default config content should be valid."))
        }
    }
    /// Return whether or not all EULAs for the Minecraft servers get accepted automatically. \
    /// The following line is copied from the vanilla Minecraft server's EULA. \
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    pub fn agree_to_eula(&mut self) -> &bool {
        *self = Config::load_config();
        &self.agree_to_eula
    }
    /// Return the size of the buffers created by this application. (If set too low, it can cause many different kinds of information to only be partially transmitted.)
    pub fn buffsize(&mut self) -> &usize {
        *self = Config::load_config();
        &self.buffsize
    }
    /// Return the port the communicator should use.
    pub fn communicator_port(&mut self) -> &u16 {
        *self = Config::load_config();
        &self.communicator_port
    }
    /// Return how long the application waits to give other tasks a chance to execute
    pub fn cooldown(&mut self) -> &Duration {
        *self = Config::load_config();
        &self.cooldown
    }
    /// Return the maximum number of times an operation gets retried.
    pub fn max_tries(&mut self) -> &u64 {
        *self = Config::load_config();
        &self.max_tries
    }
    /// Return the amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). \
    /// If the value is 0, no restarts will be performed.
    pub fn mcserver_restart_time(&mut self) -> &Duration {
        *self = Config::load_config();
        &self.mcserver_restart_time
    }
    /// If no player is playing on any server for that duration, the computer running this application gets shut down. \
    /// If the value is 0, no shutdowns will be performed.
    pub fn shutdown_time(&mut self) -> &Duration {
        *self = Config::load_config();
        &self.shutdown_time
    }
    /// The port the website should use.
    pub fn website_port(&mut self) -> &u16 {
        *self = Config::load_config();
        &self.website_port
    }
}