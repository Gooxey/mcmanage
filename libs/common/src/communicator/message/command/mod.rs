//! This module provides the [`Command`] enum which contains all valid commands that can be used across the [`MCManage Network`](https://github.com/Gooxey/MCManage.git).

use proc_macros::{
    add_convert,
    MatchCommand,
};

use self::{
    error::ErrorArgs,
    permission::Permission,
    set_id::SetIdArgs,
};
use crate::mcmanage_error::MCManageError;

pub mod error;
pub mod permission;
pub mod set_id;

// TODO check if the command can be send by the sender

/// This enum provides all valid commands that can be used across the MCManage Network.
#[derive(MatchCommand)]
#[add_convert]
pub enum Command {
    /// Encountered an error when executing the requested command.
    Error(ErrorArgs),
    /// Set the id of a client
    SetId(SetIdArgs),
}
impl Command {
    /// This function returns who can execute a given [`Command`].
    pub fn permission(&self) -> Permission {
        match self {
            _ => Permission::All,
        }
    }
}
