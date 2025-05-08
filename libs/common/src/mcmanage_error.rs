//! This module provides the MCManageError which is used anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).

use std::{
    io,
    str::Utf8Error,
};

use thiserror::Error;

/// This error type provides errors used anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).
#[derive(Error, Debug)]
pub enum MCManageError {
    /// The function encountered an invalid file. See the function description for more information.
    #[error("The function encountered an invalid file. See the function description for more information.")]
    InvalidFile,
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
    /// The api request has invalid arguments.
    #[error("{0}")]
    InvalidRequest(String),
    /// An error of kind IOError occurred.
    #[error(transparent)]
    IOError(#[from] io::Error),
    /// An error of kind Utf8Error occurred.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// An error of kind toml::de::Error occurred.
    #[error(transparent)]
    TomlDeserializeError(#[from] toml::de::Error),
    /// An error of kind toml::de::Error occurred.
    #[error(transparent)]
    TomlSerializeError(#[from] toml::ser::Error),
    /// An error of kind SerdeJsonError occurred.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
