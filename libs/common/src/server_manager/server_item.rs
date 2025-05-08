//! This module provides the [`ServerItem`] struct which represents a [`Server`](super::server::Server) defined in the `config/server_list.toml` file.

use std::time::Duration;

use proc_macros::add_toml_convert;

use crate::mcmanage_error::MCManageError;

/// This struct represents a [`Server`](super::server::Server) defined in the `config/server_list.toml` file.
#[add_toml_convert]
pub struct ServerItem {
    /// These are the args passed to the 'java' command.
    /// That means that this Minecraft server will be started using the command 'java -jar purpur-1.19.3-1876.jar nogui'
    ///
    /// Note: When specifying a ram limit like '-Xmx=4G', the Minecraft server will likely fail to start.
    pub args: String,
    /// This is a link from which the Minecraft server should be downloaded if none can be found.
    /// A download can be avoided by leaving this field empty. (For example: download_from = "")
    pub download_from: String,
    /// This is the type of the Minecraft server. Depending on what value got set,
    /// the application will register events like the joining of a player based on different log messages.
    /// See the 'config/server_types.toml' file for all available types.
    pub server_type: String,
    /// This is the amount of time the application should wait between restarts of this Minecraft server.
    /// If both the secs and nanos values are 0, no restarts will be performed.
    pub restart_time: Duration,
}
