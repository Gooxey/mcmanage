//! This module provides the [`MCServer struct`](MCServer) which represents an API for one Minecraft server, which got assigned with the initiation of this struct.


use std::{
    fs::File,
    io::{
        Read,
        Write
    },
    path::Path,
    process::Stdio,
    sync::Arc,
    time::Instant
};

use async_recursion::async_recursion;
use proc_macros::ConcurrentClass;
use tokio::{
    io::{
        AsyncBufReadExt,
        AsyncWriteExt,
        BufReader
    },
    process::{
        Child,
        ChildStdout,
        Command
    },
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
    qol::write_to_log_file::write_to_log_file,
    status::Status,
    types::ThreadJoinHandle,
    warn
};

use self::mcserver_type::MCServerType;


pub mod mcserver_type;
mod tests;


/// This struct represents an API for one Minecraft server, which got assigned with the initiation of this struct. \
/// 
/// 
/// # Features
/// 
/// - The log of the Minecraft server running gets saved to ' logs/{MCServer.name}.txt '.
/// - Lines of text can be sent to the Minecraft server.
/// - The names of the players currently on the Minecraft server get saved.
/// - The [`status`](Status) of the Minecraft server gets saved. ( Starting, Stopping, ... )
/// - Automatically agrees to the EULA if activated in the [`config`](Config).
#[derive(ConcurrentClass)]
pub struct MCServer {
    /// This struct's name
    name: String,
    /// The applications [`Config`]
    config: Arc<Config>,
    /// The main thread of this struct
    main_thread: Arc<Mutex<Option<ThreadJoinHandle>>>,
    /// The [`Status`] of this struct
    status: Mutex<Status>,
    
    /// The arguments which should be passed to the Minecraft server
    args: Vec<String>,
    /// The [`type`](MCServerType) of the Minecraft server
    mcserver_type: MCServerType,
    /// This holds the Minecraft server process
    minecraft_server: Mutex<Option<Child>>,
    /// The path to the Minecraft server
    path: String,
    /// A list of all players on the Minecraft server
    players: Mutex<Vec<String>>
}
impl MCServer {
    /// Create a new [`MCServer`] instance.
    pub fn new(name: &str, config: &Arc<Config>, args: &str, mcserver_type: MCServerType) -> Arc<Self> {
        Self {
            name: name.to_owned(),
            config: config.clone(),
            main_thread: Arc::new(None.into()),
            status: Status::Stopped.into(),
            
            args: args.split(' ').map(String::from).collect(),
            mcserver_type,
            minecraft_server: None.into(),
            path: format!("servers/{}", name),
            players: vec![].into()
        }
        .into()
    }
    /// Get the name of this [`MCServer`].
    pub fn name(self: &Arc<Self>) -> String {
        self.name.clone()
    }
    /// Get the status of this [`MCServer`].
    pub async fn status(self: &Arc<Self>) -> Status {
        *self.status.lock().await
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

        match Command::new("java")
            .current_dir(&self.path)
            .args(&self.args)
            .stderr(Stdio::inherit())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(minecraft_server) => {
                *self.minecraft_server.lock().await = Some(minecraft_server);
            }
            Err(err) => {
                erro!(self.name, "An error occurred while starting the Minecraft Server {}. Error: {err}", self.name);
                self.reset().await;
                return Err(MCManageError::FatalError)
            }
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

        if let Some(mut minecraft_server ) = self.minecraft_server.lock().await.take() {
            // send the stop command to the Minecraft server
            if let Some(stdin) = minecraft_server.stdin.as_mut() {
                if let Err(erro) = stdin.write_all("stop\n".to_string().as_bytes()).await {
                    if !restart { erro!(self.name, "An error occurred while writing the input `stop` to the Minecraft server. The process will be kill forcefully. Error: {erro}"); }
                    if (minecraft_server.kill().await).is_err() {}
                }
                self.save_output(">> stop").await;
            } else {
                if !restart { erro!(self.name, "The stdin pipe of this Minecraft server process does not exist. The process will be kill forcefully."); }
                if (minecraft_server.kill().await).is_err() {}
            }

            // wait for the Minecraft server to finish
            if let Err(erro) = minecraft_server.wait().await {
                erro!(self.name, "An error occurred while waiting on the Minecraft server to finish. Error: {erro}");
                self.reset().await;
                return Err(MCManageError::FatalError);
            }
        }

        self.stop_main_thread().await?;
        *self.status.lock().await = Status::Stopped;

        if !restart { info!(self.name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }
        Ok(())
    }
    
    /// Return a list of every player who is currently on this Minecraft server.
    pub async fn players(self: &Arc<Self>) -> Vec<String> {
        self.players.lock().await.clone()
    }
    /// Send a given string to the Minecraft server as an input. \
    /// It is guaranteed that the string given will be sent to the MCServer, but this can cause the blocking of the thread calling this function due to the MCServer restarting.
    #[async_recursion]
    pub async fn send_input(self: Arc<Self>, input: &str) {
        if let Some(child) = self.minecraft_server.lock().await.as_mut() {
            if let Some(stdin) = child.stdin.as_mut() {
                if let Err(erro) = stdin.write_all(format!("{input}\n").as_bytes()).await {
                    erro!(self.name, "An error occurred while writing the input `{input}` to the Minecraft server. This MCServer will be restarted. Error: {erro}");
                    while let Err(MCManageError::NotReady) = self.clone().impl_restart().await {
                        sleep(*self.config.refresh_rate()).await;
                    }
                    self.clone().send_input(input).await;
                }
                self.save_output(&format!(">> {input}")).await;
            } else {
                erro!(self.name, "The stdin pipe of this Minecraft server process does not exist. This MCServer will be restarted.");
                while let Err(MCManageError::NotReady) = self.clone().impl_restart().await {
                    sleep(*self.config.refresh_rate()).await;
                }
                self.clone().send_input(input).await;
            }
        } else {
            erro!(self.name, "The Minecraft server process could not be found. Please start the Minecraft server before sending input to it.");
        }
    }

    /// Reset a given struct to its starting values.
    pub(super) async fn reset(self: &Arc<Self>) {
        if let Some(thread) = self.main_thread.lock().await.take() {thread.abort();}
        *self.status.lock().await = Status::Stopped;
        if let Some(mut server) = self.minecraft_server.lock().await.take() {
            if (server.kill().await).is_err() {}
        }
        *self.players.lock().await = vec![];
    }
    /// This represents the main loop of a given struct.
    async fn main(self: Arc<Self>, mut bootup_result: Option<oneshot::Sender<()>>) -> Result<(), MCManageError> {
        let mut agreed_to_eula = false;
        let stdout = BufReader::new(self.get_stdout_pipe().await?);

        let mut lines = stdout.lines();
        loop {
            let line;
            match lines.next_line().await {
                Ok(content) => {
                    if let Some(content) = content {
                        line = content;
                    } else {
                        // It will only be None returned if the Child process got killed
                        return Ok(())
                    }
                }
                Err(erro) => {
                    unimplemented!("An error occurred while reading the output of {}. Error: {erro}", self.name)
                }
            }

            self.save_output(&line).await;
            
            if !agreed_to_eula {
                self.agree_to_eula().await?;
                agreed_to_eula = true;
            }
            
            if let Some(bootup_result_inner) = bootup_result {
                match self.check_started(&line, bootup_result_inner).await {
                    Ok(result) => bootup_result = result,
                    Err(erro) => {
                        match erro {
                            MCManageError::CriticalError => {
                                return Err(MCManageError::CriticalError);
                            }
                            // this will handle:
                            //      MCServerTypeError::InvalidFile
                            //      MCServerTypeError::FileNotFound
                            //      MCServerTypeError::NotFound
                            _ => {
                                // TODO: Handle this error <= Something went wrong with the server_types.json file
                                todo!("Something went wrong with the server_types.json file => The console needs to be implemented before deciding what to do here")
                            }
                        }
                    }
                }
            }

            self.check_player_activity(&line).await?;
        }
    }
    /// Save a given line to a log file saved under ' logs/{MCServer.name}.txt '.
    async fn save_output(self: &Arc<Self>, line: &str) {
        write_to_log_file(format!("{line}\n").as_bytes(), &self.name);
    }
    /// Get the stdout pipe of the Minecraft server. This function will not handle errors.
    async fn get_stdout_pipe(self: &Arc<Self>) -> Result<ChildStdout, MCManageError> {
        if let Some(child ) = self.minecraft_server.lock().await.as_mut() {
            if let Some(childstdout) = child.stdout.take() {
                return Ok(childstdout);
            } else {
                erro!(self.name, "The stdout pipe of this Minecraft server process does not exist. This MCServer will be restarted.");
            }
        } else {
            erro!(self.name, "The Minecraft server process could not be found.");
        }
        self.restart();
        Err(MCManageError::CriticalError)
    }
    /// Check if the Minecraft server has started.
    async fn check_started(self: &Arc<Self>, line: &str, bootup_result: oneshot::Sender<()>) -> Result<Option<oneshot::Sender<()>>, MCManageError> {
        for item in self.mcserver_type.get_started().await? {
            if !line.contains(&item) {
                return Ok(Some(bootup_result));
            }
        }
        self.send_start_result(&mut Some(bootup_result)).await?;
        *self.status.lock().await = Status::Started;
        Ok(None)
    }
    /// Check for player activity ( connecting/disconnecting ) and save the name of the player who joined or delete the one who left.
    async fn check_player_activity(self: &Arc<Self>, line: &str) -> Result<(), MCManageError> {
        // check if anyone joined / left
        let mut player_joined = true;
        for item in self.mcserver_type.get_player_joined().await? {
            if !line.contains(&item) {
                player_joined = false;
                break;
            }
        }
        let mut player_left = true;
        if !player_joined {
            for item in self.mcserver_type.get_player_left().await? {
                if !line.contains(&item) {
                    player_left = false;
                    break;
                }
            }
        }
        
        // save the detected state to this MCServer
        let mut players = self.players.lock().await;
        if player_joined {
            players.push(self.mcserver_type.get_player_name_joined(line).await?);
        } else if player_left {
            let player_name = self.mcserver_type.get_player_name_left(line).await?;
            if let Ok(index) = players.binary_search(&player_name) {
                players.remove(index);
            } else {
                erro!(self.name, "The player {player_name} left without ever joining this server.");

                self.restart();
                return Err(MCManageError::CriticalError);
            }
        }
        Ok(())
    }
    /// Automatically agree to the EULA if activated in the config. \
    /// If this setting is deactivated by the user, this function will send a message informing the user of the situation and then return an error and shut down the
    /// MCServer calling this function.
    async fn agree_to_eula(self: &Arc<Self>) -> Result<(), MCManageError> {
        // check if the EULA has been accepted
        if Path::new(&(self.path.clone() + "/eula.txt")).exists() {
            let mut eula_txt = "".to_string();
            if File::options().read(true).open(self.path.clone() + "/eula.txt")
                .unwrap_or_else(|_| panic!("It was already checked whether or not the {} file exists.", self.path.clone() + "/eula.txt"))
                .read_to_string(&mut eula_txt)
                .is_err()
            {}

            if eula_txt.contains("eula=true") {
                return Ok(());
            }
        }
        warn!(self.name, "The EULA has to be accepted to use this MCServer.");

        // agree to the EULA if configured
        if *self.config.agree_to_eula() {
            match File::create(self.path.clone() + "/eula.txt") {
                Ok(mut eula_file) => {
                    let failcounter = 0;
                    while eula_file.write(b"eula=true").is_err() {
                        if failcounter == *self.config.max_tries() {
                            erro!(self.name, "The maximum number of write attempts to the ' eula.txt ' file have been reached. The MCServer will no longer try to accept the EULA.");
                            self.stop();
                            return Err(MCManageError::FatalError);
                        } else {
                            erro!(self.name, "This was attempt number {} out of {}", failcounter, self.config.max_tries());
                        }
                        sleep(*self.config.refresh_rate()).await;
                    }
                }
                Err(erro) => {
                    erro!(self.name, "Failed to open the eula.txt file of this Minecraft server. Error: {erro}");
                    self.stop();
                    return Err(MCManageError::FatalError);
                }
            }
            
            info!(self.name, "#########################################################################################################################");
            info!(self.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            info!(self.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            info!(self.name, "# The EULA has been automatically accepted.                                                                             #");
            info!(self.name, "# To deactivate this behavior, change the ' agree_to_eula ' variable in the given config to false.                      #");
            info!(self.name, "#########################################################################################################################");
            
            Ok(())
        } else {
            warn!(self.name, "#########################################################################################################################");
            warn!(self.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            warn!(self.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            warn!(self.name, "# The EULA has not yet been accepted. Please accept it to continue using this server.                                   #");
            warn!(self.name, "# To automatically accept all EULAs in the future, change the ' agree_to_eula ' variable in the given config to true.   #");
            warn!(self.name, "#                                                                                                                       #");
            warn!(self.name, "# This MCServer will now shut down.                                                                                     #");
            warn!(self.name, "#########################################################################################################################");
            
            self.stop();
            Err(MCManageError::FatalError)
        }
    }
}