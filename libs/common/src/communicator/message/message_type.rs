//! This module provides the [`MessageType`] enum, which is used to determine what kind of [`Message`](super::Message) was sent.


use proc_macros::add_convert;


/// This enum describes the type of message holding this enum. There are three possible states. The first one is [`request`](MessageType::Request). It is used when the
/// [`Message`](super::Message) was sent to receive or execute something. The second option, [`response`](MessageType::Response), is used when the [`Message`](super::Message)
/// returns the requested data or the result of an operation. The last variant gets used in case an error occurs while executing a command.
/// 
/// ## Variants
/// 
/// | Variant                             | Description                                                                                                         |
/// |-------------------------------------|---------------------------------------------------------------------------------------------------------------------|
/// | [`Request`](MessageType::Request)   | The [`Message`](super::Message) sent requests an operation to be executed or a value to be returned.                |
/// | [`Response`](MessageType::Response) | The [`Message`](super::Message) sent returns the result of an operation or the value requested.                     |
/// | [`Error`](MessageType::Error)       | The [`Message`](super::Message) sent indicates that an error occurred during the execution of the provided request. |
#[add_convert]
pub enum MessageType {
    /// The [`Message`](super::Message) sent indicates that an error occurred during the execution of the provided request.
    Error,
    /// The [`Message`](super::Message) sent requests an operation to be executed or a value to be returned.
    Request,
    /// The [`Message`](super::Message) sent returns the result of an operation or the value requested.
    Response
}