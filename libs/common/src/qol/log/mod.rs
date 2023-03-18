//! This module provides various macros used to print and save messages to the console and log files.


pub mod helper;


/// This macro can be used to print and save an info to the console and the `log.txt` file. \
/// Use the [`info_to`](crate::info_to) macro to save the log to a specified file.
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
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($sender: expr, $( $argument: tt ) *) => {       
        $crate::log!("log", "", $sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save an info to the console and a set file. \
/// Use the [`info`](crate::info) macro to save the log to the `log.txt` file.
/// 
/// # Parameters
/// 
/// 1. This is the `destination`, or the name, of the file in which the log message should be saved.
/// 2. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 3. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use std::time::Duration;
/// use common::info_to;
/// 
/// # fn main() {
/// let secs = Duration::new(1, 0);
/// info_to!("my_first_mcserver", "MyFirstMCServer", "Started in {:?} secs", secs);
/// 
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! info_to {
    ($destination: expr, $sender: expr, $( $argument: tt ) *) => {       
        $crate::log!($destination, "", $sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save a warning for the user to the console and the `log.txt` file. \
/// Use the [`warn_to`](crate::warn_to) macro to save the log to a specified file.
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
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($sender: expr, $( $argument: tt ) *) => {       
        $crate::log!("log", "warn", $sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save a warning for the user to the console and a set file. \
/// Use the [`warn`](crate::warn) macro to save the log to the `log.txt` file.
/// 
/// # Parameters
/// 
/// 1. This is the `destination`, or the name, of the file in which the log message should be saved.
/// 2. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 3. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use common::{
///     mcmanage_error::MCManageError,
///     warn_to
/// };
/// 
/// # fn main() {
/// let err = MCManageError::FatalError;
/// warn_to!("my_first_mcserver", "MyFirstMCServer", "Accept the EULA to use this MCServer. Error: {}", err);
/// 
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! warn_to {
    ($destination: expr, $sender: expr, $( $argument: tt ) *) => {       
        $crate::log!($destination, "warn", $sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save an error to the console and the `log.txt` file. \
/// Use the [`erro_to`](crate::erro_to) macro to save the log to a specified file.
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
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! erro {
    ($sender: expr, $( $argument: tt ) *) => {       
        $crate::log!("log", "erro", $sender, $( $argument ) *);
    }
}
/// This macro can be used to print and save an error to the console and a set file. \
/// Use the [`erro`](crate::erro) macro to save the log to the `log.txt` file.
/// 
/// # Parameters
/// 
/// 1. This is the `destination`, or the name, of the file in which the log message should be saved.
/// 2. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 3. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```
/// use common::{
///     mcmanage_error::MCManageError,
///     erro_to
/// };
/// 
/// # fn main() {
/// let err = MCManageError::FatalError;
/// erro_to!("my_first_mcserver", "MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// 
/// # std::fs::remove_dir_all("./logs").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! erro_to {
    ($destination: expr, $sender: expr, $( $argument: tt ) *) => {       
        $crate::log!($destination, "erro", $sender, $( $argument ) *);
    }
}