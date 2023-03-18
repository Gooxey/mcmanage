//! This module provides the [`MCServerManager`] struct, which is responsible for managing all [`MCServers`](MCServer). ( starting, stopping, ... )


use std::{
    sync::Arc,
    time::{
        Duration,
        Instant
    }
};

use proc_macros::ConcurrentClass;
use serde_json::Value;
use tokio::{
    sync::{
        Mutex,
        oneshot
    },
    time::sleep
};

use crate::{
    config::Config,
    erro,
    info,
    mcmanage_error::MCManageError,
    status::Status,
    types::ThreadJoinHandle,
    qol::load_json_file::{
        generate_valid_file,
        load_json_file
    }
};
use self::{
    mcserver::{
        MCServer,
        mcserver_type::MCServerType
    },
    server_list_example_default::SERVER_LIST_EXAMPLE_DEFAULT
};


mod tests;
pub mod mcserver;
pub mod server_list_example_default;


// FIXME When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start. (only when starting them via the MCServerManager)
// TODO Download MCServer specified in its file
// TODO Custom restart timer for MCServer

/// This struct is responsible for managing all [`MCServers`](MCServer). ( starting, stopping, ... ) \
/// In more detail, it creates [`MCServer`] structs accordingly to the `servers/server_list.json` file. Additionally it will also start a thread which:
///     - If set, will shut down the computer that is running this application.
///     - If enabled, will restart Minecraft servers automatically.
/// 
/// # Warning
/// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start.
#[derive(ConcurrentClass)]
pub struct MCServerManager {
    /// This struct's name
    name: String,
    /// The applications [`Config`]
    config: Arc<Config>,
    /// The main thread of this struct
    main_thread: Arc<Mutex<Option<ThreadJoinHandle>>>,
    /// The [`Status`] of this struct
    status: Mutex<Status>,
    
    /// The list of every MCServer
    mcserver_list: Mutex<Vec<Arc<MCServer>>>
}
impl MCServerManager {
    /// Create a new [`MCServerManager`] instance.
    pub fn new(config: &Arc<Config>) -> Arc<Self> {
        Self {
            name: "MCServerManager".to_string(),
            config: config.clone(),
            main_thread: Arc::new(None.into()),
            status: Status::Stopped.into(),

            mcserver_list: vec![].into(),
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

        self.load_mcserver_list().await?;
        for mcserver in &*self.mcserver_list.lock().await {
            mcserver.start();
        }

        let rx = self.start_main_thread().await?;
        self.recv_start_result(rx).await?;
        *self.status.lock().await = Status::Started;

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

        self.stop_main_thread().await?;
        *self.status.lock().await = Status::Stopped;

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
    /// Create the MCServers according to the `servers/server_list.json` file. \
    /// If any problem is detected in the `servers/server_list.json` file, this file will be renamed to `servers/invalid_server_list.json` and an example file will be
    /// generated under `servers/server_list_example.json`.
    /// 
    /// # Warning
    /// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start.
    async fn load_mcserver_list(self: &Arc<Self>) -> Result<(), MCManageError> {
        let mcserver_list_json = load_json_file(
            &self.name,
            "config",
            "server_list",
            SERVER_LIST_EXAMPLE_DEFAULT,
            false
        )?;

        // create a list of MCServers and return it
        let mut mcserver_list: Vec<Arc<MCServer>> = vec![];
        let mut i = 0;
        loop {
            if let Some(server) = mcserver_list_json.get(i.to_string()) {
                let name = &self.get_server_parameter(server, i, "name")?;
                let args = &self.get_server_parameter(server, i, "arg")?;
                let mcserver_type = &self.get_server_parameter(server, i, "type")?;

                mcserver_list.push(MCServer::new(name, &self.config.clone(), args, MCServerType::new(mcserver_type, name)));
            } else {
                if i == 0 {
                    erro!(self.name, "The 'config/server_list.json' file did not contain any servers. See the example file for a valid style.");
                    generate_valid_file(SERVER_LIST_EXAMPLE_DEFAULT, "config", "server_list");
                    return Err(MCManageError::InvalidFile);
                }
                *self.mcserver_list.lock().await = mcserver_list;
                return Ok(());
            }
            i+=1;
        }
    }
    /// Read a given parameter of a json object and return its value in the form of a string.
    fn get_server_parameter(self: &Arc<Self>, server_json: &Value, server_id: i32, parameter_name: &str) -> Result<String, MCManageError> {
        if let Some(value) = server_json.get(parameter_name) {
            if let Some(real_value) = value.as_str() {
                return Ok(real_value.to_string());
            } else {
                erro!(self.name, "The '{parameter_name}' parameter of server {server_id} should be a string. See the 'servers/server_list_example.json' file for a valid write style.");
            }
        } else {
            erro!(self.name, "The server {server_id} is missing a '{parameter_name}' parameter. See the 'servers/server_list_example.json' file for a valid write style."); 
        }
        generate_valid_file(SERVER_LIST_EXAMPLE_DEFAULT, "config", "server_list");
        Err(MCManageError::InvalidFile)
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
        self.send_start_result(&mut bootup_result).await?;

        let mut offline_counter: Option<Instant> = None;
        let mut last_restart = Instant::now();

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
            if *self.config.shutdown_time() > Duration::new(0, 0) {
                if let Some(offline_counter) = offline_counter {
                    if Instant::now() - offline_counter >= *self.config.shutdown_time() {
                        info!(self.name, "No player was active for {:?}. This machine will now shut down.", self.config.shutdown_time());
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
            if *self.config.mcserver_restart_time() > Duration::new(0, 0) && Instant::now() - last_restart >= *self.config.mcserver_restart_time() {
                info!(self.name, "The automatic restart time of {:?} has been reached. All MCServer's will now restart.", *self.config.mcserver_restart_time());
                for mcserver in &*self.mcserver_list.lock().await {
                    mcserver.restart();
                }
                last_restart = Instant::now();
            }

            sleep(*self.config.refresh_rate()).await;
        }
    }
}