//! This module provides the [`Message`] struct, which is used all over the [`MCManage network`](https://github.com/Gooxey/MCManage.git) to transmit commands or information.


use std::sync::Arc;

use proc_macros::add_convert;

use crate::mcmanage_error::MCManageError;

use self::{
    command::{
        Command,
        error::ErrorArgs
    },
    message_type::MessageType
};

use super::{
    client_type::ClientType,
    CommunicatorTrait
};

pub mod command;
pub mod message_type;


/// This struct represents the standard message, which is used to send commands or information between different applications in the
/// [`MCManage network`](https://github.com/Gooxey/MCManage.git).
#[add_convert]
pub struct Message {
    /// The [`Command`] of this [`Message`]
    command: Command,
    /// The [`type`](MessageType) of this [`Message`]
    message_type: MessageType,
    /// The id of the machine that should receive this [`Message`]
    receiver: u64,
    /// The id of the machine sending this [`Message`]
    sender: u64,
}
impl Message {
    /// Create a new [`Message`].
    pub fn new(command: Command, message_type: MessageType, receiver: u64, sender: u64) -> Self {
        Self {
            command,
            message_type,
            receiver,
            sender
        }
    }
    /// Execute the [`Command`] contained inside this [`Message`]. \
    /// This method will not block the thread calling it. \
    /// If the client lacks the permission to execute a given command, this method will return an error of kind [`MCManageError::MissingPermission`].
    pub async fn execute<C: CommunicatorTrait>(&self, client_type: &ClientType, communicator: &Arc<C>) {
        if let MessageType::Request = self.message_type {
            if let Err(erro) = self.command.execute(client_type) {
                match erro {
                    MCManageError::MissingPermission => {
                        communicator.send_message(
                            Message::new(
                                Command::Error(ErrorArgs{error: "MissingPermission".to_string()}),
                                MessageType::Error,
                                self.sender,
                                0
                            )
                        ).await;
                    }
                    _ => {
                        unimplemented!("All expected errors have been handled.")
                    }
                }
            }
        } else {
            todo!("Implement the behavior of responses and errors")
        }
    }

    /// Get the [`Command`] of this [`Message`].
    pub fn command(&self) -> &Command {
        &self.command
    }
    /// Get the [`MessageType`] of this [`Message`].
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    /// Get the receiver id of this [`Message`].
    pub fn receiver(&self) -> u64 {
        self.receiver
    }
    /// Get the sender id of this [`Message`].
    pub fn sender(&self) -> u64 {
        self.sender
    }
}