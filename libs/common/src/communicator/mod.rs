//! This module provides the [`CommunicatorTrait`] which allows [`Messages`](message::Message) to send a command to any Communicator given.


use std::sync::Arc;

use async_trait::async_trait;

use self::message::Message;


pub mod message;
pub mod client_type;


/// This trait allows [`Messages`](message::Message) to send a command to any Communicator given.
#[async_trait]
pub trait CommunicatorTrait {
    /// Send a given message. \
    /// The message will be stored inside the buffer until the client connected or until the message gets to old.
    async fn send_message(self: &Arc<Self>, message: Message);
}