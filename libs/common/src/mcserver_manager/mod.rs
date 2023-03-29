//! This module provides the [`MCServerManager`] struct, which is responsible for managing all [`MCServers`](MCServer). ( starting, stopping, ... )


use std::{
    sync::Arc,
    time::{
        Duration,
        Instant
    }
};

use proc_macros::ConcurrentClass;
use tokio::{
    sync::{
        Mutex,
        oneshot
    },
    time::sleep
};
use toml::Table;

use crate::{
    config,
    error,
    info,
    mcmanage_error::MCManageError,
    qol::load_toml_file::load_toml,
    status::Status,
    types::ThreadJoinHandle
};
use self::{
    mcserver::MCServer,
    server_list_example_default::SERVER_LIST_EXAMPLE_DEFAULT, server_item::ServerItem
};


mod server_item;
mod tests;
pub mod mcserver;
pub mod server_list_example_default;


// FIXME When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.toml` file, the Minecraft server can fail to start. (only when starting them via the MCServerManager)
// TODO constantly update MCServer list

/// This struct is responsible for managing all [`MCServers`](MCServer). ( starting, stopping, ... ) \
/// In more detail, it creates [`MCServer`] structs accordingly to the `servers/server_list.toml` file. Additionally it will also start a thread which:
///     - If set, will shut down the computer that is running this application.
///     - If enabled, will restart Minecraft servers automatically.
/// 
/// # Warning
/// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.toml` file, the Minecraft server can fail to start.
#[derive(ConcurrentClass)]
pub struct MCServerManager {
    /// This struct's name
    name: String,
    /// The main thread of this struct
    main_thread: Arc<Mutex<Option<ThreadJoinHandle>>>,
    /// The [`Status`] of this struct
    status: Mutex<Status>,

    /// The list of every MCServer
    mcserver_list: Mutex<Vec<Arc<MCServer>>>,
    /// The list of every restart_time for every MCServer
    restart_times: Mutex<Vec<Duration>>
}
impl MCServerManager {
    /// Create a new [`MCServerManager`] instance.
    pub async fn new() -> Arc<Self> {
        Self {
            name: "MCServerManager".to_string(),
            main_thread: Arc::new(None.into()),
            status: Status::Stopped.into(),

            mcserver_list: vec![].into(),
            restart_times: vec![].into()
        }
        .into()
    }
    /// This is the blocking implementation to start a given struct. \
    /// For a non-blocking mode use the [`start method`](Self::start). \
    /// \
    /// The `restart` parameter will be used by the [`restart method`](Self::impl_restart) to deactivate all non-fatal error messages of this method and to enable
    /// this method to be executed during a restart.
    pub async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError> {
        self.check_allowed_start(restart).await?;

        if !restart { info!(self.name, "Starting..."); }
        let start_time = Instant::now();

        self.load_mcserver_list().await;
        for mcserver in &*self.mcserver_list.lock().await {
            mcserver.start();
        }

        let rx = self.start_main_thread().await;
        self.recv_start_result(rx, restart).await;
        if !restart {
            *self.status.lock().await = Status::Started;
        }

        if !restart { info!(self.name, "Started in {:.3} secs!", start_time.elapsed().as_secs_f64()); }
        Ok(())
    }
    /// This is the blocking implementation to stop a given struct. \
    /// For a non-blocking mode use the [`stop method`](Self::stop). \
    /// \
    /// The `restart` parameter will be used by the [`restart method`](Self::impl_restart) to deactivate all non-fatal error messages of this method and to enable
    /// this method to be executed during a restart. \
    /// \
    /// The `forced` parameter is used to wait for a given struct to start / stop to ensure a stop attempt.
    pub async fn impl_stop(self: Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError> {
        self.check_allowed_stop(restart, forced).await?;

        if !restart { info!(self.name, "Shutting down..."); }
        let stop_time = Instant::now();

        for mcserver in &*self.mcserver_list.lock().await {
            if (mcserver.clone().impl_stop(false, true).await).is_err() {}
        }

        self.stop_main_thread().await;
        if !restart {
            *self.status.lock().await = Status::Stopped;
        }

        if !restart { info!(self.name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }
        Ok(())
    }
    /// Reset a given struct to its starting values.
    pub async fn reset(self: &Arc<Self>) {
        if let Some(thread) = self.main_thread.lock().await.take() {thread.abort();}
        *self.status.lock().await = Status::Stopped;

        for mcserver in &*self.mcserver_list.lock().await {
            mcserver.reset().await;
        }
        *self.mcserver_list.lock().await = vec![];
    }
    /// Create the MCServers according to the `servers/server_list.toml` file. \
    /// If any problem is detected in the `servers/server_list.toml` file, this file will be renamed to `servers/invalid_server_list.toml` and an example file will be
    /// generated under `servers/server_list_example.toml`.
    /// 
    /// # Warning
    /// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.toml` file, the Minecraft server can fail to start.
    async fn load_mcserver_list(self: &Arc<Self>) {
        /// This function will read the 'config/server_list.toml' file and return a [`toml table`](Table) once the [`load_toml`] function returns one.
        async fn wait_for_valid_server_list(mcserver_manager: &Arc<MCServerManager>) -> Table {
            if let Ok(mcserver_list_toml) = load_toml(
                "config",
                "server_list",
                SERVER_LIST_EXAMPLE_DEFAULT,
                &mcserver_manager.name,
                true
            ).await {
                mcserver_list_toml
            } else {
                error!(mcserver_manager.name, "The MCServerManager will now wait for a valid 'config/server_list.toml' file.");
                loop {
                    if let Ok(mcserver_list_toml) = load_toml(
                        "config",
                        "server_list",
                        SERVER_LIST_EXAMPLE_DEFAULT,
                        &mcserver_manager.name,
                        false
                    ).await {
                        info!(mcserver_manager.name, "A valid 'config/server_list.toml' has been registered. The starting process will now proceed.");
                        return mcserver_list_toml;
                    } else {
                        sleep(config::cooldown().await).await;
                    }
                }
            }
        }


        let mut mcserver_list_toml = wait_for_valid_server_list(self).await;

        // create a list of MCServers and return it
        let mut warned_about_empty_list = false;
        let mut warned_invalid_server = false;
        let mut mcserver_list: Vec<Arc<MCServer>> = vec![];
        let mut restart_times: Vec<Duration> = vec![];
        loop {
            if mcserver_list_toml.is_empty() {
                if !warned_about_empty_list {
                    error!(self.name, "The 'config/server_list.toml' file did not contain any servers. See the example file for a valid style.");
                    warned_about_empty_list = true;
                }
                mcserver_list_toml = wait_for_valid_server_list(self).await;
                sleep(config::cooldown().await).await;
                continue;
            }

            let mut finished_reading_list = true;
            for key in mcserver_list_toml.clone().keys() {
                if let Some(server) = mcserver_list_toml.get(key) {
                    let server_item: ServerItem;
                    if let Ok(server) = server.clone().try_into() {
                        server_item = server
                    } else {
                        if !warned_invalid_server {
                            error!(self.name, "The server {key} is invalid. See the 'servers/server_list_example.toml' file for a valid write style.");
                            warned_invalid_server = true;
                        }
                        warned_about_empty_list = false;
                        finished_reading_list = false;
                        mcserver_list_toml = wait_for_valid_server_list(self).await;
                        sleep(config::cooldown().await).await;
                        break;
                    }

                    let name = key;
                    let restart_time = server_item.restart_time;

                    mcserver_list.push(MCServer::new(name, server_item).await);
                    restart_times.push(restart_time);
                }
            }

            if finished_reading_list {
                *self.mcserver_list.lock().await = mcserver_list;
                *self.restart_times.lock().await = restart_times;
                return;
            }
        }

    }
    /// Return a list of every [`MCServer`].
    pub async fn get_all(self: &Arc<Self>) -> Result<Vec<Arc<MCServer>>, MCManageError> {
        return Ok(self.mcserver_list.lock().await.clone())
    }
    /// Search for a [`MCServer`] by its name and return it if found.
    pub async fn get_mcserver(self: &Arc<Self>, mcserver_name: &str) -> Result<Arc<MCServer>, MCManageError> {
        for mcserver in &*self.mcserver_list.lock().await {
            if mcserver.name() == mcserver_name {
                return Ok(mcserver.clone());
            }
        }

        Err(MCManageError::NotFound)
    }
    /// This represents the main loop of a given struct.
    async fn main(self: Arc<Self>, mut bootup_result: Option<oneshot::Sender<()>>) -> Result<(), MCManageError> {
        self.send_start_result(&mut bootup_result).await;

        let mut offline_counter: Option<Instant> = None;
        let mut last_restart = vec![];
        for _ in 0..self.restart_times.lock().await.len() {
            last_restart.push(Instant::now());
        }

        loop {
            // check if any player is online
            let mut player_online = false;
            for mcserver in &*self.mcserver_list.lock().await {
                let player_list = mcserver.players().await;

                if !player_list.is_empty() {
                    player_online = true;
                    break;
                }
            }

            // shut down the computer running this application if configured
            if config::shutdown_time().await > Duration::new(0, 0) {
                if let Some(offline_counter) = offline_counter {
                    let shutdown_time = config::shutdown_time().await;
                    if Instant::now() - offline_counter >= shutdown_time {
                        info!(self.name, "No player was active for {:?}. This machine will now shut down.", shutdown_time);
                        system_shutdown::shutdown().expect("Could not shutdown this machine.");
                    }
                }

                if player_online {
                    offline_counter = None;
                } else if offline_counter.is_none() {
                    offline_counter = Some(Instant::now());
                }
            }

            // restart the MCServers automatically every configured amount of time
            let mcserver_list = self.mcserver_list.lock().await;
            for (i, restart_time) in self.restart_times.lock().await.iter().enumerate() {
                if *restart_time > Duration::new(0, 0) && Instant::now() - last_restart[i] >= *restart_time {
                    info!(self.name, "The automatic restart time of {:?} has been reached. {} will now restart.", restart_time, mcserver_list[i].name());
                    mcserver_list[i].restart();
                    last_restart[i] = Instant::now();
                }
            }

            sleep(config::cooldown().await).await;
        }
    }
}