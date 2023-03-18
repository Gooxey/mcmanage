//! This module provides the [`Config struct`](Config), which is used all over the [`MCManage network`](https://github.com/Gooxey/MCManage.git) as the application's config.


use std::sync::Arc;
use std::time::Duration;


/// The following line is copied from the Minecraft servers EULA
/// By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).
const AGREE_TO_EULA: bool = true;


/// This struct represents the config of applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).
pub struct Config {
    /// The port the communicator should use.
    communicator_port: u16,
    /// The port the webserver should run on.
    website_port: u16,
    /// The size of the buffers created by this application. (If set too low, it can cause many different kinds of information to only be partially transmitted.)
    buffsize: usize,
    /// The time the application waits between checks.
    refresh_rate: Duration,
    /// The maximum number of times an operation gets retried.
    max_tries: u64,
    /// Sets whether or not all EULAs for the Minecraft servers get accepted automatically. \
    /// The following line is copied from the vanilla Minecraft server's EULA. \
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    agree_to_eula: bool,
    /// If no player is playing on any server for that duration, the computer running this application gets shut down. \
    /// If the value is 0, no shutdowns will be performed.
    shutdown_time: Duration,
    /// The amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). \
    /// If the value is 0, no restarts will be performed.
    mcserver_restart_time: Duration,
    /// Sets how long a message gets attempted to be sent until it gets destroyed. \
    /// If the value is 0, the message will never be destroyed.
    message_max_lifetime: Duration,
    /// Sets how long the application wait to give other tasks a chance to execute.
    cooldown: Duration
}
impl Config {
    /// Create a new [`Config`] instance.
    pub fn new() -> Arc<Self> {
        Self {
            communicator_port: 25564,
            website_port: 8080,
            buffsize: 100000000,
            refresh_rate: Duration::new(0, 100000000),
            max_tries: 3,
            agree_to_eula: AGREE_TO_EULA,
            shutdown_time: Duration::new(0, 0),
            mcserver_restart_time: Duration::new(86400, 0),
            message_max_lifetime: Duration::new(60, 0),
            cooldown: Duration::new(0, 100000000)
        }.into()
    }
    /// Return the port the communicator should use.
    pub fn communicator_port(&self) -> &u16 {
        &self.communicator_port
    }
    /// Return the port the webserver should run on.
    pub fn website_port(&self) -> &u16 {
        &self.website_port
    }
    /// Return the size of the buffers created by this application. (If set too low, it can cause many different kinds of information to only be partially transmitted.)
    pub fn buffsize(&self) -> &usize {
        &self.buffsize
    }
    /// Return the time the application waits between checks.
    pub fn refresh_rate(&self) -> &Duration {
        &self.refresh_rate
    }
    /// Return the maximum number of times an operation gets retried.
    pub fn max_tries(&self) -> &u64 {
        &self.max_tries
    }
    /// Return whether or not all EULAs for the Minecraft servers get accepted automatically. \
    /// The following line is copied from the vanilla Minecraft server's EULA. \
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    pub fn agree_to_eula(&self) -> &bool {
        &self.agree_to_eula
    }
    /// If no player is playing on any server for that duration, the computer running this application gets shut down. \
    /// If the value is 0, no shutdowns will be performed.
    pub fn shutdown_time(&self) -> &Duration {
        &self.shutdown_time
    }
    /// Return the amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). \
    /// If the value is 0, no restarts will be performed.
    pub fn mcserver_restart_time(&self) -> &Duration {
        &self.mcserver_restart_time
    }
    /// Return how long a message gets attempted to be sent until it gets destroyed
    pub fn message_max_lifetime(&self) -> &Duration {
        &self.message_max_lifetime
    }
    /// Return how long the application wait to give other tasks a chance to execute
    pub fn cooldown(&self) -> &Duration {
        &self.cooldown
    }
}