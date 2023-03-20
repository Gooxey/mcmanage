//! This module implements the [`GetType`](Command::GetType) command.


use proc_macros::add_convert;

use crate::{
    communicator::client_type::ClientType,
    mcmanage_error::MCManageError
};

use super::Command;


/// These are the arguments for the [`GetType`](Command::GetType) command.
#[add_convert]
pub struct GetTypeArgs {
    /// The [`type`](ClientType) of the client
    pub client_type: Option<ClientType>
}

impl Command {
    /// Execute the [`GetType`](Command::GetType) command.
    pub async fn execute_gettype(self, _args: GetTypeArgs) {

    }
}