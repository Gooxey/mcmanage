//! This library provides derive and attribute macros for structs and enums in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]


use proc_macro::{
    TokenStream,
    self
};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;


mod match_command;
mod concurrent_class;
mod convert;
mod toml_convert;


/// This derive macro generates a function called `execute` which can be used to execute an asynchronous function associated with the variant of a given enum called
/// `execute_<variant name>`. \
/// \
/// Note: This macro MUST be used inside the `common library` and run inside a `tokio runtime` since the methods of each variant are asynchronous.
/// 
/// # Example
/// 
/// ```compile_fail
/// use proc_macros::MatchCommand;
/// use crate::{
///     mcmanage_error::MCManageError,
///     communicator::message::command::permission::Permission
/// };
/// 
/// // A Args struct can either be empty...
/// struct StartArgs;
/// 
/// // ...or it can be filled with fields as long as it implements the Convert trait from the common library
/// struct StopArgs {
///     delay_in_secs: i32
/// }
/// 
/// #[derive(MatchCommand, Clone)]  // the clone trait must be implemented
/// enum Command {
///     Start(StartArgs),   // only one parameter is allowed, and it has to be a struct called `<variant name>Args`
///     Stop(StopArgs)
/// }
/// impl Command {  // here we have to create asynchronous methods called `execute<variant name>` for every variant
///     pub async fn execute_Start(self, args: StartArgs) {
///         // some code which will be executed when `Command::Start(StartArgs).execute()` gets executed
///     }
///     pub async fn execute_Stop(self, args: StopArgs) {
///         // some code which will be executed when `Command::Stop(StopArgs{delay_in_secs:3}).execute()` gets executed
///     }
/// }
/// ```
#[proc_macro_derive(MatchCommand)]
pub fn derive_match_command(input: TokenStream) -> TokenStream {
    match_command::match_command(input)
}

/// This trait provides standard functions used by every concurrent struct in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). \
/// 
/// # Example
/// 
/// ```compile_fail
/// use std::{
///     sync::Arc,
///     time::Instant
/// };
/// 
/// use proc_macros::ConcurrentClass;
/// use tokio::{
///     sync::{
///         Mutex,
///         oneshot
///     },
///     task::JoinHandle
/// };
/// 
/// use crate::{    // if you use this derive macro inside another library, replace `crate` with `common`
///     config,
///     error,
///     info,
///     mcmanage_error::MCManageError,
///     status::Status,
///     types::ThreadJoinHandle
/// };
/// 
/// 
/// #[derive(ConcurrentClass)]
/// struct MyConcurrentStruct {
///     /// This struct's name
///     name: String,
///     /// The main thread of this struct
///     main_thread: Arc<Mutex<Option<ThreadJoinHandle>>>,
///     /// The [`Status`] of this struct
///     status: Mutex<Status>
/// }
/// // The following methods HAVE TO be implemented, otherwise the application will panic
/// impl MyConcurrentStruct {
///     /// Create a new [`MyConcurrentStruct`] instance.
///     pub async fn new() -> Arc<Self> {
///         Self {
///             name: "MyConcurrentStruct".to_string(),
///             main_thread: Arc::new(None.into()),
///             status: Status::Stopped.into()
///         }
///         .into()
///     }
///     /// This is the blocking implementation to start a given struct. \
///     /// For a non-blocking mode use the [`start method`](Self::start). \
///     /// \
///     /// The `restart` parameter will be used by the [`restart method`](Self::impl_restart) to deactivate all non-fatal error messages of this method and to enable
///     /// this method to be executed during a restart.
///     pub async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError> {
///         self.check_allowed_start(restart).await?;
///
///         if !restart { info!(self.name, "Starting..."); }
///         let start_time = Instant::now();
///
///         let rx = self.start_main_thread().await?;
///         self.recv_start_result(rx).await?;
///         *self.status.lock().await = Status::Started;
///
///         if !restart { info!(self.name, "Started in {:.3} secs!", start_time.elapsed().as_secs_f64()); }
///         Ok(())
///     }
///     /// This is the blocking implementation to stop a given struct. \
///     /// For a non-blocking mode use the [`stop method`](Self::stop). \
///     /// \
///     /// The `restart` parameter will be used by the [`restart method`](Self::impl_restart) to deactivate all non-fatal error messages of this method and to enable
///     /// this method to be executed during a restart. \
///     /// \
///     /// The `forced` parameter is used to wait for a given struct to start / stop to ensure a stop attempt.
///     pub async fn impl_stop(self: Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError> {
///         self.check_allowed_stop(restart, forced).await?;
///
///         if !restart { info!(self.name, "Shutting down..."); }
///         let stop_time = Instant::now();
///
///         self.stop_main_thread().await?;
///         *self.status.lock().await = Status::Stopped;
///
///         if !restart { info!(self.name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }
///         Ok(())
///     }
///     /// Reset a given struct to its starting values.
///     async fn reset(self: &Arc<Self>) {
///         if let Some(thread) = self.main_thread.lock().await.take() {thread.abort();}
///         *self.status.lock().await = Status::Stopped;
///     }
///     /// This represents the main loop of a given struct.
///     async fn main(self: Arc<Self>, mut bootup_result: Option<oneshot::Sender<()>>) -> Result<(), MCManageError> {
///         self.send_start_result(&mut bootup_result).await?;
/// 
///         loop {
///             todo!()
///         }
/// 
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_derive(ConcurrentClass)]
pub fn derive_concurrent_class(input: TokenStream) -> TokenStream {
    concurrent_class::concurrent_class(input)
}

/// This derive macro allows a struct or enum to be converted from and into [`json-objects`](serde_json::Value), `strings` and `byte-strings` using the `try_from()` and
/// `try_into()` methods. \
/// \
/// Note: Using the [`add_convert`](macro@add_convert) proc attribute significantly reduces the amount of boilerplate code.
/// 
/// # Example
/// 
/// Cargo.toml:
/// ```toml
/// serde = { version = "1.0.155", features = ["derive"] }
/// # A Dependency to this crate is also required
/// ```
/// 
/// Rust code:
/// ```compile_fail
/// use proc_macros::Convert;
/// use serde::{
///     Deserialize,
///     Serialize
/// };
/// use crate::mcmanage_error::MCManageError;   // if you use this derive macro inside another library, replace `crate` with `common`
/// 
/// 
/// #[derive(Convert, Deserialize, Serialize)]
/// struct MyConvertibleStruct {
///     text: String,
///     number: i64
/// }
/// ```
#[proc_macro_derive(Convert)]
pub fn derive_convert(input: TokenStream) -> TokenStream {
    convert::convert(input)
}

/// This attribute allows a struct or enum to be converted from and into [`json-objects`](serde_json::Value), `strings` and `byte-strings` using the `try_from()` and
/// `try_into()` methods.
/// 
/// # Example
/// 
/// Cargo.toml:
/// ```toml
/// serde = { version = "1.0.155", features = ["derive"] }
/// # A Dependency to this crate is also required
/// ```
/// 
/// Rust code:
/// ```compile_fail
/// use proc_macros::add_convert;
/// use crate::mcmanage_error::MCManageError;   // if you use this derive macro inside another library, replace `crate` with `common`
/// 
/// 
/// #[add_convert]
/// struct MyConvertibleStruct {
///     text: String,
///     number: i64
/// }
/// ```
#[proc_macro_attribute]
pub fn add_convert(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();

    quote! {
        #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, proc_macros::Convert)]
        #input
    }
    .into()
}

/// This derive macro allows a struct or enum to be converted from and into [`toml-objects`](toml::Value), `strings` and `byte-strings` using the `try_from()` and
/// `try_into()` methods. \
/// \
/// Note: Using the [`add_convert`](macro@add_convert) proc attribute significantly reduces the amount of boilerplate code.
/// 
/// # Example
/// 
/// Cargo.toml:
/// ```toml
/// serde = { version = "1.0.155", features = ["derive"] }
/// # A Dependency to this crate is also required
/// ```
/// 
/// Rust code:
/// ```compile_fail
/// use proc_macros::TomlConvert;
/// use serde::{
///     Deserialize,
///     Serialize
/// };
/// use crate::mcmanage_error::MCManageError;   // if you use this derive macro inside another library, replace `crate` with `common`
/// 
/// 
/// #[derive(TomlConvert, Deserialize, Serialize)]
/// struct MyConvertibleStruct {
///     text: String,
///     number: i64
/// }
/// ```
#[proc_macro_derive(TomlConvert)]
pub fn derive_toml_convert(input: TokenStream) -> TokenStream {
    toml_convert::toml_convert(input)
}

/// This attribute allows a struct or enum to be converted from and into [`toml-objects`](toml::Value), `strings` and `byte-strings` using the `try_from()` and
/// `try_into()` methods.
/// 
/// # Example
/// 
/// Cargo.toml:
/// ```toml
/// serde = { version = "1.0.155", features = ["derive"] }
/// # A Dependency to this crate is also required
/// ```
/// 
/// Rust code:
/// ```compile_fail
/// use proc_macros::add_toml_convert;
/// use crate::mcmanage_error::MCManageError;   // if you use this derive macro inside another library, replace `crate` with `common`
/// 
/// 
/// #[add_toml_convert]
/// struct MyConvertibleStruct {
///     text: String,
///     number: i64
/// }
/// ```
#[proc_macro_attribute]
pub fn add_toml_convert(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();

    quote! {
        #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, proc_macros::TomlConvert)]
        #input
    }
    .into()
}