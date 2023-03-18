//! This module provides macros and functions used by macros in the [`log module`](super).


pub extern crate chrono;


/// This function prints a log message to the console. \
/// 
/// # Parameters
/// 
/// 1. This represents the `variant` of the log. There are three states:
///     - warn => Use this one in case you want to warn the user about something.
///     - erro => In the event of an error, use this one. 
///     - info => This is the default, so no specific input is required.
/// 2. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 3. This is the `message` that will be printed after all the other information has been printed.
pub fn log_print(variant: &str, sender: String, message: &str) {
    print!("{} | ", chrono::Local::now().format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"));
    print!("{} | ", 
        match variant {
            "warn" => "\x1b[93m\x1b[1mWARN\x1b[0m",
            "erro" => "\x1b[91m\x1b[1mERRO\x1b[0m",
            _ => "\x1b[94m\x1b[1mINFO\x1b[0m" // the default is an info text
        }
    );
    print!("\x1b[97m\x1b[1m{:<16.16}\x1b[0m | ", sender);
    print!("{}", message);
    println!();
}

/// This macro can be used to print and save a given string to the console and a set file. \
/// \
/// Note: There are much simpler alternatives than using this macro: \
///     - [`info`](crate::info) and [`info_to`](crate::info_to) \
///     - [`warn`](crate::warn) and [`warn_to`](crate::warn_to) \
///     - [`erro`](crate::erro) and [`erro_to`](crate::erro_to) \
/// See their respective documentation for more information.
/// 
/// # Parameters
/// 
/// 1. This represents the `variant` of the log. There are three states:
///     - warn => Use this one in case you want to warn the user about something.
///     - erro => In the event of an error, use this one. 
///     - info => This is the default, so no specific input is required.
/// 2. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 3. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
/// 
/// # Example
/// 
/// ```rust
/// # use common::mcserver_manager::mcserver::mcserver_error::MCServerError;
/// # use common::log;
/// let err = MCServerError::FatalError;
/// 
/// log!("my_first_mcserver", "erro", "MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// ```
#[macro_export]
macro_rules! log {
    ($destination: expr, $variant: expr, $sender: expr, $( $arguments: tt ) *) => {
        $crate::qol::log::helper::log_print($variant, $sender.clone().to_string(), &format!($( $arguments ) *));

        let mut log: String = "".to_string();
        log += &format!("{} | ", $crate::qol::log::helper::chrono::Local::now().format("%d.%m.%Y | %H:%M:%S"));
        log += &format!("{} | ", 
            match $variant.into() {
                "warn" => "WARN",
                "erro" => "ERRO",
                _ => "INFO" // the default is an info text
            }
        );
        log += &format!("{:<16.16} | ", $sender);
        log += &format!($ ( $arguments ) *);
        log += &format!("\n");

        $crate::qol::write_to_log_file::write_to_log_file(
            log.as_bytes(),
            $destination
        );
    }
}