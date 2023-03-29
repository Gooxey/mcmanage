//! This module provides various macros used to print and save messages to the console and log files.


/// This macro can be used to print and save an info to the console and the log files at `logs/mcmanage*.log`. \
/// Infos indicate important information that should be logged under normal conditions such as services starting.
/// 
/// # Parameters
/// 
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use std::time::Duration;
/// use common::info;
/// 
/// # fn main() {
/// let secs = Duration::new(1, 0);
/// info!("MyFirstMCServer", "Started in {:?} secs", secs);
/// 
/// // This is what this macro will expand to:
/// log::info!(target: &"MyFirstMCServer", "Started in {:?} secs", secs);
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($sender: expr, $( $argument: tt ) *) => {
        log::info!(target: &$sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save a warning for the user to the console and the log files at `logs/mcmanage*.log`. \
/// Warnings indicate a potential problem that may or may not require investigation. They should be used sparingly to avoid becoming meaningless.
/// 
/// # Parameters
/// 
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use common::{
///     mcmanage_error::MCManageError,
///     warn
/// };
/// 
/// # fn main() {
/// let err = MCManageError::FatalError;
/// warn!("MyFirstMCServer", "Accept the EULA to use this MCServer. Error: {}", err);
/// 
/// // This is what this macro will expand to:
/// log::warn!(target: &"MyFirstMCServer", "Accept the EULA to use this MCServer. Error: {}", err);
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($sender: expr, $( $argument: tt ) *) => {
        log::warn!(target: &$sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save an error to the console and the log files at `logs/mcmanage*.log`. \
/// Errors indicate a problem that needs to be investigated, but doesn't require immediate attention.
/// 
/// # Parameters
/// 
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use common::{
///     mcmanage_error::MCManageError,
///     erro
/// };
/// 
/// # fn main() {
/// let err = MCManageError::FatalError;
/// erro!("MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// 
/// // This is what this macro will expand to:
/// log::error!(target: &"MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($sender: expr, $( $argument: tt ) *) => {
        log::error!(target: &$sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save trace messages to the console and the log files at `logs/mcmanage*.log`. \
/// Trace messages indicate the steps leading up to errors and warnings, and should provide context to understand them.
/// 
/// # Parameters
/// 
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use common::{
///     mcmanage_error::MCManageError,
///     erro
/// };
/// 
/// # fn main() {
/// let err = MCManageError::FatalError;
/// erro!("MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// 
/// // This is what this macro will expand to:
/// log::trace!(target: &"MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    ($sender: expr, $( $argument: tt ) *) => {
        log::trace!(target: &$sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save debug messages to the console and the log files at `logs/mcmanage*.log`. \
/// Debug messages indicate debugging information that is compiled out of Release builds and is discouraged due to its tendency to create log noise. \
/// \
/// Note: Messages passed to this macro will only be printed during debug mode.
/// 
/// # Parameters
/// 
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use common::{
///     mcmanage_error::MCManageError,
///     erro
/// };
/// 
/// # fn main() {
/// let err = MCManageError::FatalError;
/// erro!("MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// 
/// // This is what this macro will expand to:
/// #[cfg(debug_assertions)]
/// log::debug!(target: &"MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    ($sender: expr, $( $argument: tt ) *) => {
        #[cfg(debug_assertions)]
        log::debug!(target: &$sender, $( $argument ) *);
    }
}