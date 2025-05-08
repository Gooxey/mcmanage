//! This module provides the [`ServerData`] struct.

use proc_macros::add_convert;

use crate::{
    mcmanage_error::MCManageError,
    status::Status,
};

/// This struct is used to transmit some general data about an Minecraft server.
#[add_convert]
pub struct ServerData {
    /// The name of the Minecraft server.
    pub name: String,
    /// The version of the Minecraft server.
    pub version: String,
    /// The [`type`](ServerType) of the Minecraft server.
    pub server_type: String,
    /// The [`Status`] of the Minecraft server.
    pub status: Status,
    /// The number of players currently on the Minecraft server.
    pub player_count: u64,
    /// The maximum amount of players allowed on the Minecraft server.
    pub player_cap: u64,
}
