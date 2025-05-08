//! This module provides the [`ServerManager`] struct, which is responsible for managing all [`Servers`](Server). ( starting, stopping, ... )

use std::{
    sync::{Arc, Once},
    time::{
        Duration,
        Instant,
    },
};

use proc_macros::ConcurrentClass;
use tokio::{
    sync::{
        oneshot::{self, Sender},
        Mutex, OnceCell,
    },
    time::sleep, spawn, task::JoinHandle,
};
use toml::Table;
use goolog::*;

use self::{
    server::Server,
    server_item::ServerItem,
};
use crate::{
    config::Config,
    generated_files::{
        load_toml_file::load_toml,
        paths::SERVER_LIST_FILE,
    },
    mcmanage_error::MCManageError,
    status::Status,
    types::ThreadJoinHandle, concurrent_class::check_allowed::check_allowed_start, server_manager::server_list::ServerList,
};
use chrono::prelude::*;

pub mod server;
mod server_item;
mod tests;
mod server_list;

const GOOLOG_CALLER: &str = "ServerManager";
static SERVER_MANAGER: OnceCell<ServerManager> = OnceCell::const_new();


// FIXME When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.toml` file, the Minecraft server can fail to start. (only when starting them via the ServerManager)
// TODO constantly update Server list
// FIXME Register errors, like "java.net.BindException: Address already in use: bind" from the Minecraft server and print them to the console (do not crash the application -> instead, stop Minecraft server)
// TODO Make the server.properties file editable
// TODO Make tests able to run concurrently

/// This struct is responsible for managing all [`Servers`](Server). ( starting, stopping, ... ) \
/// In more detail, it creates [`Server`] structs accordingly to the `servers/server_list.toml` file. Additionally it will also start a thread which:
///     - If set, will shut down the computer that is running this application.
///     - If enabled, will restart Minecraft servers automatically.
///
/// # Warning
/// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.toml` file, the Minecraft server can fail to start.
// #[derive(ConcurrentClass)]
pub struct ServerManager {
    main_thread: JoinHandle<Result<(), MCManageError>>,
}
impl ServerManager {
    /// Get the [`ServerManager`].
    ///
    /// # Panics
    ///
    /// This function will panic if the [`ServerManager`] is not yet initialized.
    fn server_manager<'a>() -> &'a ServerManager {
        SERVER_MANAGER
            .get()
            .unwrap_or_else(|| {
                fatal!("You must first initialize the ServerManager with the `ServerManager::init()` function before doing anything.")
            })
    }

    // TODO doc
    // TODO move method used only in here to here
    pub async fn init() {
        ServerList::init();

        let main_thread = spawn(Self::main());

        if SERVER_MANAGER.set(Self {
            main_thread,
        }).is_err() {
            fatal!("Already initialized.")
        }

        info!("Initialized!");
    }

    /// # Panics
    ///
    /// This function will panic if the [`ServerManager`] has not yet been initialized via the [`ServerManager::init()`] function.
    pub async fn stop(forced: bool) -> Result<(), MCManageError> {
        let server_manager = Self::server_manager();

        info!("Shutting down...");
        let stop_time = Instant::now();

        ServerList::stop().await;

        server_manager
            .main_thread
            // .lock()
            // .await
            // .take()
            // .unwrap_or_else(|| {
            //     fatal!("The main thread should have been set by now.")
            // })
            .abort();

        info!(
            "Stopped in {:.3} secs!",
            stop_time.elapsed().as_secs_f64()
        );

        Ok(())
    }


    // /// Create the Servers according to the `servers/server_list.toml` file. \
    // /// If any problem is detected in the `servers/server_list.toml` file, this file will be renamed to `servers/invalid_server_list.toml` and an example file will be
    // /// generated under `servers/server_list_example.toml`.
    // ///
    // /// # Warning
    // /// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.toml` file, the Minecraft server can fail to start.
    // async fn load_server_list() {
    //     /// This function will read the 'config/server_list.toml' file and return a [`toml table`](Table) once the [`load_toml`] function returns one.
    //     async fn wait_for_valid_server_list() -> Table {
    //         if let Ok(server_list_toml) =
    //             load_toml(&SERVER_LIST_FILE, GOOLOG_CALLER, true).await
    //         {
    //             server_list_toml
    //         } else {
    //             error!(
    //                 "The ServerManager will now wait for a valid 'config/server_list.toml' file."
    //             );
    //             loop {
    //                 if let Ok(server_list_toml) =
    //                     load_toml(&SERVER_LIST_FILE, GOOLOG_CALLER, false).await
    //                 {
    //                     info!("A valid 'config/server_list.toml' has been registered. The starting process will now proceed.");
    //                     return server_list_toml;
    //                 } else {
    //                     sleep(Config::cooldown().await).await;
    //                 }
    //             }
    //         }
    //     }

    //     let mut server_list_toml = wait_for_valid_server_list().await;

    //     // create a list of Servers and return it
    //     let mut warned_about_empty_list = false;
    //     let mut warned_invalid_server = false;
    //     let mut server_list: Vec<Arc<Server>> = vec![];
    //     let mut restart_times: Vec<Duration> = vec![];
    //     loop {
    //         if server_list_toml.is_empty() {
    //             if !warned_about_empty_list {
    //                 error!("The 'config/server_list.toml' file did not contain any servers. See the example file for a valid style.");
    //                 warned_about_empty_list = true;
    //             }
    //             server_list_toml = wait_for_valid_server_list().await;
    //             sleep(Config::cooldown().await).await;
    //             continue;
    //         }

    //         let mut finished_reading_list = true;
    //         for key in server_list_toml.clone().keys() {
    //             if let Some(server) = server_list_toml.get(key) {
    //                 let server_item: ServerItem;
    //                 if let Ok(server) = server.clone().try_into() {
    //                     server_item = server
    //                 } else {
    //                     if !warned_invalid_server {
    //                         error!("The server {key} is invalid. See the 'servers/server_list_example.toml' file for a valid write style.");
    //                         warned_invalid_server = true;
    //                     }
    //                     warned_about_empty_list = false;
    //                     finished_reading_list = false;
    //                     server_list_toml = wait_for_valid_server_list().await;
    //                     sleep(Config::cooldown().await).await;
    //                     break;
    //                 }

    //                 let name = key;
    //                 let restart_time = server_item.restart_time;

    //                 server_list.push(Server::new(name, server_item).await);
    //                 restart_times.push(restart_time);
    //             }
    //         }

    //         if finished_reading_list {
    //             let server_manager = server_manager();
    //             let mut server_list_lock = server_manager.server_list.lock().await;

    //             server_list_lock.0 = server_list;
    //             server_list_lock.1 = Utc::now();
    //             *server_manager.restart_times.lock().await = restart_times;
    //             return;
    //         }
    //     }
    // }

    /// This represents the main loop of a given struct.
    async fn main() -> Result<(), MCManageError> {
        // let server_manager = Self::server_manager();

        // let mut offline_counter: Option<Instant> = None;
        // let mut last_restart = vec![];
        // for _ in 0..ServerList::server_count().await {
        //     last_restart.push(Instant::now());
        // }

        // loop {
        //     // check if any player is online
        //     let mut player_online = false;
        //     for server in ServerList::servers().await.iter() {
        //         if server.used().await {
        //             player_online = true;
        //             break;
        //         }
        //     }

        //     // shut down the computer running this application if configured
        //     // FIXME shutdown time should be stored as option
        //     if let Some(shutdown_time) = Config::shutdown_time().await {
        //         if let Some(offline_counter) = offline_counter {
        //             if Instant::now() - offline_counter >= shutdown_time {
        //                 info!(
        //                     "No player was active for {:?}. This machine will now shut down.",
        //                     shutdown_time
        //                 );
        //                 system_shutdown::shutdown().unwrap_or_else(|error| {
        //                     fatal!("Failed to shutdown this machine. Error: {error}")
        //                 });
        //             }
        //         }

        //         if player_online {
        //             offline_counter = None;
        //         } else if offline_counter.is_none() {
        //             offline_counter = Some(Instant::now());
        //         }
        //     }

        //     // restart the Servers automatically every configured amount of time
        //     let servers =  ServerList::servers().await;
        //     // for (i, restart_time) in server_manager.restart_times.lock().await.iter().enumerate() {
        //     //     if *restart_time > Duration::new(0, 0)
        //     //         && Instant::now() - last_restart[i] >= *restart_time
        //     //     {
        //     //         info!(
        //     //             "The automatic restart time of {:?} has been reached. {} will now restart.",
        //     //             restart_time,
        //     //             servers[i].name()
        //     //         );
        //     //         servers[i].restart();
        //     //         last_restart[i] = Instant::now();
        //     //     }
        //     // }

        //     sleep(Config::cooldown().await).await;
        // }

        Ok(())
    }
}
