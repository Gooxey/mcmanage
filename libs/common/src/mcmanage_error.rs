//! This module provides the MCManageError which is used anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).


use std::{
    io,
    string::FromUtf8Error, str::Utf8Error
};
use thiserror::Error;


/// This error type provides errors used anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).
#[derive(Error, Debug)]
pub enum MCManageError {
    /// The function encountered a recoverable error.
    #[error("The function encountered a recoverable error.")]
    CriticalError,
    /// The function encountered a non-recoverable error.
    #[error("The function encountered a non-recoverable error.")]
    FatalError,
    /// The function encountered an invalid file. See the function description for more information.
    #[error("The function encountered an invalid file. See the function description for more information.")]
    InvalidFile,
    /// The client did not react as expected. This connection will be closed.
    #[error("The client did not react as expected. This connection will be closed.")]
    InvalidClient,
    /// The requested item could not be found.
    #[error("The requested item could not be found.")]
    NotFound,
    /// The function has already been executed.
    #[error("The method has already been executed.")]
    AlreadyExecuted,
    /// The function is currently being executed by another thread.
    #[error("The method is currently being executed by another thread.")]
    CurrentlyExecuting,
    /// The function is not ready to be executed. Please try again later.
    #[error("The function is not ready to be executed. Please try again later.")]
    NotReady,
    /// The struct needs to be started before executing anything. Please execute the start function first.
    #[error("The struct needs to be started before executing anything. Please execute the start function first.")]
    NotStarted,
    /// The client encountered an error. The connection will be closed.
    #[error("The client encountered an error. The connection will be closed.")]
    ClientError,
    /// The client lacks the permission to execute a given command.
    #[error("The client lacks the permission to execute a given command.")]
    MissingPermission,
    /// An error of kind IOError occurred.
    #[error("An error of kind IOError occurred.")]
    IOError(#[from] io::Error),
    /// An error of kind FromUtf8Error occurred.
    #[error("An error of kind FromUtf8Error occurred.")]
    FromUtf8Error(#[from] FromUtf8Error),
    /// An error of kind Utf8Error occurred.
    #[error("An error of kind Utf8Error occurred.")]
    Utf8Error(#[from] Utf8Error),
    /// An error of kind toml::de::Error occurred.
    #[error("An error of kind toml::de::Error occurred.")]
    TomlDeserializeError(#[from] toml::de::Error),
    /// An error of kind toml::de::Error occurred.
    #[error("An error of kind toml::ser::Error occurred.")]
    TomlSerializeError(#[from] toml::ser::Error)
}