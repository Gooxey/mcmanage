//! This module provides the [`MCServer struct`](MCServer) which represents an API for one Minecraft server, which got assigned with the initiation of this struct.


use std::{
    io::ErrorKind,
    path::Path,
    process::Stdio,
    sync::Arc,
    time::Instant
};

use futures_util::StreamExt;
use async_recursion::async_recursion;
use proc_macros::ConcurrentClass;
use tokio::{
    fs::{
        File,
        self, OpenOptions
    },
    io::{
        AsyncBufReadExt,
        AsyncWriteExt,
        BufReader, self, AsyncReadExt
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
    config,
    error,
    info,
    mcmanage_error::MCManageError,
    qol::load_toml_file::load_toml,
    status::Status,
    types::ThreadJoinHandle,
    warn
};

use self::mcserver_type::MCServerType;

use super::{server_item::ServerItem, server_list_example_default::SERVER_LIST_EXAMPLE_DEFAULT};


pub mod mcserver_type;
mod tests;

// TODO cancel download jar if stop or reset gets called


/// This struct represents an API for one Minecraft server, which got assigned with the initiation of this struct. \
/// 
/// # Features
/// 
/// - The log of the Minecraft server running gets saved to 'logs/servers/<name of this server>.txt'.
/// - Lines of text can be sent to the Minecraft server.
/// - The names of the players currently on the Minecraft server get saved.
/// - The [`status`](Status) of the Minecraft server gets saved. ( Starting, Stopping, ... )
/// - Automatically agrees to the EULA if activated in the [`config`](config::agree_to_eula).
/// - Downloads the server jar if needed from a link specified in the 'config/server_list.toml'
#[derive(ConcurrentClass)]
pub struct MCServer {
    /// This struct's name
    name: String,
    /// The main thread of this struct
    main_thread: Arc<Mutex<Option<ThreadJoinHandle>>>,
    /// The [`Status`] of this struct
    status: Mutex<Status>,

    /// The arguments which should be passed to the Minecraft server
    args: Mutex<Vec<String>>,
    /// The url to download the minecraft server jar from
    download_from: Mutex<String>,
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
    pub async fn new(name: &str, server_item: ServerItem) -> Arc<Self> {
        Self {
            name: name.to_owned(),
            main_thread: Arc::new(None.into()),
            status: Status::Stopped.into(),

            args: Mutex::new(server_item.args.split(' ').map(String::from).collect()),
            download_from: server_item.download_from.into(),
            mcserver_type: MCServerType::new(&server_item.mcserver_type, name),
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
        self.download_jar().await;

        *self.minecraft_server.lock().await = Some(
            Command::new("java")
                .current_dir(&self.path)
                .args(&*self.args.lock().await)
                .stderr(Stdio::inherit())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap_or_else(|erro| panic!("An error occurred while starting the Minecraft Server {}. Error: {erro}", self.name))
        );

        let rx = self.start_main_thread().await;
        self.recv_start_result(rx, restart).await;
        if !restart {
            *self.status.lock().await = Status::Started;
        }

        if !restart { info!(self.name, "Started in {:.3} secs!", start_time.elapsed().as_secs_f64()); }
        Ok(())
    }
    /// This method will check if a server jar exists. If no jar file was detected, the one from the configured link will be downloaded. In the event that neither a jar file
    /// nor a download lind exists, this method will block until one is set.
    async fn download_jar(self: &Arc<Self>) {
        /// This function returns the jar name from the args of a Minecraft server.
        fn get_jar_name(args: &Vec<String>) -> String {
            let mut jar_name = "".to_string();
            let args_len = args.len();
            for (i, arg) in args.iter().enumerate() {
                if arg == "-jar" {
                    if i+1 == args_len {
                        return "".to_string();
                    }
                    jar_name = args[i+1].clone();
                    if !jar_name.contains(".jar") {
                        return "".to_string();
                    }
                    break;
                }
            }
            jar_name
        }
        /// This function will read the `config/mcserver_list.toml` file and return the current [`ServerItem`].
        async fn get_current_server_item(mcserver: &Arc<MCServer>) -> Option<ServerItem> {
            let mcserver_list_toml;
            if let Ok(toml) = load_toml(
                "config",
                "server_list",
                SERVER_LIST_EXAMPLE_DEFAULT,
                &mcserver.name,
                false
            ).await {
                mcserver_list_toml = toml;
            } else {
                return None;
            }

            // create a list of MCServers and return it
            let mut server_item: Option<ServerItem> = None;
            for key in mcserver_list_toml.keys() {
                if key == &mcserver.name {
                    if let Some(server) = mcserver_list_toml.get(key) {
                        if let Ok(server) = server.clone().try_into() {
                            server_item = Some(server)
                        }
                    }
                    break;
                }
            }
            server_item
        }
        /// This function will wait for a file or download link to be set.
        async fn wait_for_jar_content(mcserver: &Arc<MCServer>, jar_path: &String) {
            loop {
                // Scan for a jar file
                if Path::exists(Path::new(jar_path)) {
                    info!(mcserver.name, "Registered a jar file. This MCServer will now continue its starting procedure.");
                    return;
                }
                // Scan for a download link
                if let Some(server_item) = get_current_server_item(mcserver).await {
                    if server_item.download_from != *"" {
                        info!(mcserver.name, "Registered a download link. This MCServer will now continue its starting procedure.");
                        *mcserver.download_from.lock().await = server_item.download_from;
                        break;
                    }
                }

                sleep(config::cooldown().await).await;
            }
        }


        let mut jar_name = get_jar_name(&*self.args.lock().await);
        if jar_name == *"" {
            error!(self.name, "No Minecraft server jar name has been defined for this MCServer. Please configure one in the 'config/server_list.toml' file.");
            error!(self.name, "This MCServer will now wait for a jar name to be set.");

            loop {
                if let Some(server_item) = get_current_server_item(self).await {
                    let args = server_item.args.split(' ').map(String::from).collect::<Vec<String>>();
                    if *self.args.lock().await != args {
                        jar_name = get_jar_name(&args);

                        if jar_name != *"" {
                            info!(self.name, "Registered a jar name. This MCServer will now continue its starting procedure.");
                            *self.args.lock().await = args;
                            break;
                        }
                    }
                }

                sleep(config::cooldown().await).await;
            }
        }

        let jar_path = format!("{}/{}", self.path, jar_name);
        if !Path::exists(Path::new(&jar_path)) {
            if *self.download_from.lock().await == *"" {
                error!(self.name, "Could not find a jar file or a link to download the jar file from.");
                error!(self.name, "Please copy a valid jar file to '{jar_path}' or set a download link for this server in 'config/server_list.toml'.");
                error!(self.name, "This MCServer will now wait for a file or download link to be set.");

                wait_for_jar_content(self, &jar_path).await;
                if Path::exists(Path::new(&jar_path)) {
                    return;
                }
            } else {
                info!(self.name, "No jar file could be found. Downloading a new one...");
            }

            let mut server_jar_option = None;
            let max_tries = config::max_tries().await;
            for i in 0..max_tries {
                match reqwest::get(&*self.download_from.lock().await).await {
                    Ok(file) => {
                        server_jar_option = Some(file.bytes_stream());
                        break;
                    }
                    Err(erro) => {
                        if erro.is_request() {
                            error!(self.name, "Something seems to be wrong with the download link given. Error: {erro}");
                            error!(self.name, "This MCServer will now wait for a file or download link to be set.");

                            wait_for_jar_content(self, &jar_path).await;
                            if Path::exists(Path::new(&jar_path)) {
                                return;
                            }
                        } else {
                            warn!(self.name, "Failed to download the server jar. Error: {erro}");
                            warn!(self.name, "This was attempt {i} out of {max_tries}.");

                            if i == max_tries {
                                panic!("The server jar could not be downloaded after {max_tries} attempts.");
                            }
                        }
                    }
                }
            }
            let mut server_jar = server_jar_option
                .unwrap_or_else(|| panic!("The server jar could not be downloaded after {max_tries} attempts."));

            fs::create_dir_all(&self.path).await
                .unwrap_or_else(|erro| panic!("Failed to create the path '{}'. Error: {erro}", self.path));
            let mut jar_file = File::create(&jar_path).await
                .unwrap_or_else(|erro| panic!("Failed to create the file '{jar_path}'. Error: {erro}"));

            while let Some(item) = server_jar.next().await {
                io::copy(
                    &mut item.unwrap_or_else(|erro| panic!("An error occurred while coping the downloaded jar to a file. Error: {erro}")).as_ref(),
                    &mut jar_file
                ).await.unwrap_or_else(|erro| panic!("An error occurred while coping the downloaded jar to a file. Error: {erro}"));
            }
        }
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
            let send_stop_result = minecraft_server
                .stdin.as_mut()
                .unwrap_or_else(|| panic!("The Minecraft server process of {} should have a stdin pipe.", self.name))
                .write_all("stop\n".as_bytes()).await;
            self.save_output(">> stop").await;

            if let Err(erro) = send_stop_result {
                if !restart { warn!(self.name, "An error occurred while writing the input `stop` to the Minecraft server. The process will be kill forcefully. Error: {erro}"); }
                if (minecraft_server.kill().await).is_err() {}
            }
            if minecraft_server.wait().await.is_err() {}
        } else {
            panic!("The MCServer {} should hold a Minecraft server process since it is in a started state.", self.name)
        }

        self.stop_main_thread().await;
        if !restart {
            *self.status.lock().await = Status::Stopped;
        }

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
        loop {
            if let Status::Started = *self.status.lock().await {
                break;
            }
        }

        let send_input_result = self.minecraft_server.lock().await.as_mut()
            .unwrap_or_else(|| panic!("The MCServer {} should hold a Minecraft server process since it is in a started state.", self.name))
            .stdin.as_mut()
            .unwrap_or_else(|| panic!("The Minecraft server process of {} should have a stdin pipe.", self.name))
            .write_all(format!("{input}\n").as_bytes()).await;

        if let Err(erro) = send_input_result {
            warn!(self.name, "An error occurred while writing the input `{input}` to the Minecraft server. This MCServer will be restarted. Error: {erro}");
            while let Err(MCManageError::NotReady) = self.clone().impl_restart().await {
                sleep(config::cooldown().await).await;
            }
            self.clone().send_input(input).await;
        }
        self.save_output(&format!(">> {input}")).await;
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
        let stdout = BufReader::new(self.get_stdout_pipe().await);

        let mut lines = stdout.lines();
        loop {
            let line;
            if let Some(content) = lines.next_line().await
                .unwrap_or_else(|erro| panic!("An error occurred while reading the output of {}. Error: {erro}", self.name))
            {
                line = content;
            } else {
                // It will only be None returned if the Child process got killed
                return Ok(())
            }

            self.save_output(&line).await;

            if !agreed_to_eula {
                self.agree_to_eula().await;
                agreed_to_eula = true;
            }

            if let Some(bootup_result_inner) = bootup_result {
                bootup_result = self.check_started(&line, bootup_result_inner).await;
            }

            self.check_player_activity(&line).await;
        }
    }
    /// Save a given line to a log file saved under ' logs/{MCServer.name}.txt '.
    async fn save_output(self: &Arc<Self>, line: &str) {
        fs::create_dir_all("logs/servers").await
            .unwrap_or_else(|erro| panic!("An error occurred while creating the dir `logs/servers`. Error: {erro}"));

        let destination = &format!("logs/servers/{}.log", self.name);
        let mut log_file;
        match OpenOptions::new().append(true).create(true).open(destination).await {
            Ok(file) => {
                log_file = file
            }
            Err(erro) if ErrorKind::NotFound == erro.kind() => {
                fs::create_dir("logs").await.expect("The error ErrorKind::NotFound only gets returned if the logs dir is missing.");

                log_file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(destination).await
                    .unwrap_or_else(|erro| panic!("Could not write to the log file {destination}. Error: {erro}"));
            }
            Err(erro) => {
                panic!("Could not write to the log file {destination}. Error: {erro}")
            }
        }

        log_file
            .write_all(format!("{line}\n").as_bytes()).await
            .unwrap_or_else(|erro| panic!("An error occurred while writing a log message to the file {destination}. Error: {erro}"));
    }
    /// Get the stdout pipe of the Minecraft server.
    async fn get_stdout_pipe(self: &Arc<Self>) -> ChildStdout {
        self.minecraft_server.lock().await.as_mut()
            .expect("This method should only be called once the Minecraft server process got set.")
            .stdout.take()
            .expect("The stdout pipe of this server only gets taken once.")
    }
    /// Check if the Minecraft server has started.
    async fn check_started(self: &Arc<Self>, line: &str, bootup_result: oneshot::Sender<()>) -> Option<oneshot::Sender<()>> {
        for item in self.mcserver_type.get_started().await {
            if !line.contains(&item) {
                return Some(bootup_result);
            }
        }
        self.send_start_result(&mut Some(bootup_result)).await;
        *self.status.lock().await = Status::Started;
        None
    }
    /// Check for player activity ( connecting/disconnecting ) and save the name of the player who joined or delete the one who left.
    async fn check_player_activity(self: &Arc<Self>, line: &str) {
        // check if anyone joined / left
        let mut player_joined = true;
        for item in self.mcserver_type.get_player_joined().await {
            if !line.contains(&item) {
                player_joined = false;
                break;
            }
        }
        let mut player_left = true;
        if !player_joined {
            for item in self.mcserver_type.get_player_left().await {
                if !line.contains(&item) {
                    player_left = false;
                    break;
                }
            }
        }

        // save the detected state to this MCServer
        let mut players = self.players.lock().await;
        if player_joined {
            players.push(
                self.mcserver_type.get_player_name_joined(line).await.unwrap_or_else(|_| panic!("It has already been checked whether or not a player joined."))
            );
        } else if player_left {
            let player_name = self.mcserver_type.get_player_name_left(line).await
                .unwrap_or_else(|_| panic!("It has already been checked whether or not a player left."));
            if let Ok(index) = players.binary_search(&player_name) {
                players.remove(index);
            } else {
                error!(self.name, "The player {player_name} left without ever joining this server. This MCServer will restart.");
                self.restart();
            }
        }
    }
    /// Automatically agree to the EULA if activated in the config. \
    /// If this setting is deactivated by the user, this function will send a message informing the user of the situation and then shut down the [`MCServer`] calling this
    /// function.
    async fn agree_to_eula(self: &Arc<Self>) {
        // check if the EULA has been accepted
        if Path::new(&(self.path.clone() + "/eula.txt")).exists() {
            let mut eula_txt = "".to_string();
            if File::open(self.path.clone() + "/eula.txt").await
                .unwrap_or_else(|_| panic!("It was already checked whether or not the {} file exists.", self.path.clone() + "/eula.txt"))
                .read_to_string(&mut eula_txt).await
                .is_err()
            {}

            if eula_txt.contains("eula=true") {
                return;
            }
        }
        warn!(self.name, "The EULA has to be accepted to use this MCServer.");

        // agree to the EULA if configured
        if config::agree_to_eula().await {
            File::create(self.path.clone() + "/eula.txt").await
                .unwrap_or_else(|erro| panic!("Failed to open the EULA file of {}. Error: {erro}", self.name))
                .write_all(b"eula=true").await
                .unwrap_or_else(|erro| panic!("Failed to accept the EULA of {}. Error: {erro}", self.name));

            info!(self.name, "#########################################################################################################################");
            info!(self.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            info!(self.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            info!(self.name, "# The EULA has been automatically accepted.                                                                             #");
            info!(self.name, "# To deactivate this behavior, change the ' agree_to_eula ' variable in the ' config/config.toml ' file to false.       #");
            info!(self.name, "#########################################################################################################################");
        } else {
            error!(self.name, "#########################################################################################################################");
            error!(self.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            error!(self.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            error!(self.name, "# The EULA has not yet been accepted. Please accept it to continue using this server.                                   #");
            error!(self.name, "# To automatically accept all EULAs in the future, change the ' agree_to_eula ' variable in the ' config/config.toml '  #");
            error!(self.name, "# file to true.                                                                                                         #");
            error!(self.name, "#                                                                                                                       #");
            error!(self.name, "# This MCServer will now shut down.                                                                                     #");
            error!(self.name, "#########################################################################################################################");

            self.stop();
        }
    }
}