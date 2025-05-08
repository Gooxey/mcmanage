//! Functions, structs, and more used by applications in the [`MCManage Network`](https://github.com/Gooxey/MCManage.git).

#![warn(
    missing_docs,
    clippy::missing_panics_doc,
    clippy::missing_docs_in_private_items
)]


// please use the `goolog::fatal!` macro instead of `unwrap()`, `expect()` or `panic!`
// this will provide more user friendly crash messages
#![warn(clippy::unwrap_used)]   // unwrap is not denied entirely to allow easier prototyping
                                // but in the end you should also use the `goolog::fatal!` macro
#![deny(
    clippy::panic,
    clippy::expect_used,
)]

#[cfg(not(feature = "frontend"))]
pub mod config;
#[cfg(not(feature = "frontend"))]
pub mod generated_files;
#[cfg(not(feature = "frontend"))]
pub mod server_manager;
#[cfg(not(feature = "frontend"))]
pub mod test_functions;
#[cfg(not(feature = "frontend"))]
pub mod types;

pub mod rest_api;
pub mod mcmanage_error;
pub mod status;
#[cfg(not(feature = "frontend"))]
pub mod concurrent_class;

#[cfg(not(feature = "frontend"))]
pub mod database_server;