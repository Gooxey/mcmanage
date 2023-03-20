//! This module provides the [`ClientType`] enum which is used by the Communicator to identify a client connected. This will greatly influence further operations.


use proc_macros::add_convert;
use crate::mcmanage_error::MCManageError;


/// This enum is used by the Communicator to identify a connected client. With this information and the
/// [`Permission`](super::message::command::permission::Permission) enum, the [`Command`](super::message::command::Command) enum is able to determine if a given command can be
/// executed by that client. Additionally, this will influence how a client gets registered.
#[add_convert]
pub enum ClientType {
    /// The client is a worker application
    Worker,
    /// The client is a user from the website
    User
}