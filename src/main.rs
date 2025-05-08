//! This is the hearth of the [`MCManage network`](https://github.com/Gooxey/MCManage.git). It is the only application required to be run 24/7 because this is the place where the
//! other two applications connect to or get started from. Therefore all data will be stored here.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]

use std::sync::{
    atomic::{
        AtomicBool,
        Ordering,
    },
    Arc, OnceLock,
};

use common::{
    config::Config,
    generated_files::paths::LOGS_DIR,
    mcmanage_error::MCManageError,
    server_manager::ServerManager,
};
use goolog::*;
use lazy_static::lazy_static;
use tokio::{
    runtime::Handle,
    spawn,
    sync::Mutex,
};

use crate::serve_frontend::{
    serve_frontend,
};

mod serve_frontend;


lazy_static! {
    /// The [`ServerManager`] of this application.
    pub static ref SERVER_MANAGER: Arc<ServerManager> = {
        let handle = Handle::current();
        let _handle_lock = handle.enter();
        futures::executor::block_on(ServerManager::init())
    };
}

// TODO Save all files downloaded from the internet to the downloads folder; Save paths to files with their origin; on every use or start refresh this list in case any file got deleted
// TODO Create trace logs
// TODO Add a force shutdown on second Ctrl+C

/// This function will setup all handlers for exiting the application. \
/// In particular, it will set handlers for:
///     - Ctrl+C events
///     - Panics
fn setup_exit_handlers(alive: Arc<AtomicBool>) {
    // make the application exit on any panic
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);
        std::process::exit(1);
    }));

    // exit the application in case of an ctrl+c
    ctrlc::set_handler(move || {
        alive.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
}

#[tokio::main]
async fn main() {
    let alive = Arc::new(AtomicBool::new(true));
    setup_exit_handlers(alive.clone());
    init_logger(None, None, Some(LOGS_DIR.join("mcmanage.log")));
    Config::init().await;

    spawn(serve_frontend());

    SERVER_MANAGER.start();

    while alive.load(Ordering::SeqCst) {}
    info!(
        "Main",
        "A Ctrl+C got registered. This application will now shut down..."
    );

    if let Err(MCManageError::NotReady) = SERVER_MANAGER.clone().impl_stop(false, false).await {
        SERVER_MANAGER.reset().await;
    }
}
