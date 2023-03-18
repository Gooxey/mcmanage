//! This module provides the [`Command`] enum which contains all valid commands that can be used across the [`MCManage Network`](https://github.com/Gooxey/MCManage.git).


use proc_macros::{
    add_convert,
    MatchCommand
};

use crate::{
    communicator::client_type::ClientType,
    mcmanage_error::MCManageError
};
use self::{
    error::ErrorArgs,
    get_type::GetTypeArgs,
    permission::Permission,
    set_id::SetIdArgs
};

pub mod error;
pub mod get_type;
pub mod permission;
pub mod set_id;


// TODO check if the command can be send by the sender

/// This enum provides all valid commands that can be used across the MCManage Network.
#[derive(MatchCommand)]
#[add_convert]
pub enum Command {
    /// Encountered an error when executing the requested command.
    Error(ErrorArgs),
    /// Get the [`type`](super::super::client_type::ClientType) of the client.
    GetType(GetTypeArgs),
    /// Set the id of a client
    SetId(SetIdArgs),
}
impl Command {
    /// This function returns who can execute a given [`Command`].
    pub fn permission(&self) -> Permission {
        match self {
            Self::GetType(_) => Permission::Main,
            _ => Permission::All
        }
    }
}