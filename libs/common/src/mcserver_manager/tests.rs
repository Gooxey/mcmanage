#![cfg(test)]


use std::{fs::{self, File}, io};

use tokio::task::spawn_blocking;

use crate::test_functions::*;
use super::*;


async fn test_start() -> Arc<MCServerManager> {
    generate_server_list();
    spawn_blocking(|| generate_mcserver_manager()).await.unwrap()
}
fn generate_server_list() {
    cleanup();
    let content = "{
        \"0\": {
            \"name\": \"myMinecraftServer\",
            \"arg\": \"-jar purpur-1.19.3-1876.jar nogui\",
            \"type\": \"purpur\" 
        }
    }";

    fs::create_dir("config").unwrap();
    let mut server_list_file = File::options().write(true).create_new(true).open("config/server_list.json").unwrap();
    io::copy(&mut content.as_bytes(), &mut server_list_file).unwrap();
}
fn generate_mcserver_manager() -> Arc<MCServerManager> {
    download_minecraft_server();

    MCServerManager::new(&Config::new())
}
fn download_minecraft_server() {
    let mut resp = reqwest::blocking::get("https://api.purpurmc.org/v2/purpur/1.19.3/1876/download").expect("An error occurred while downloading the Minecraft server");
    fs::create_dir_all("servers/myMinecraftServer").expect("An error occurred while creating the servers dir");
    let mut out = File::create("servers/myMinecraftServer/purpur-1.19.3-1876.jar").expect("failed to create file `purpur-1.19.3-1876.jar`");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

// the following two functions will also test `get_server_parameter` and `generate_valid_server_list_file`
#[tokio::test]
async fn load_mcserver_list_valid_file() {
    let mcserver_manager = test_start().await;

    mcserver_manager.load_mcserver_list().await.unwrap();

    assert_eq!(mcserver_manager.mcserver_list.lock().await.len(), 1, "The function should only have captured one server.");
    cleanup();
}
#[tokio::test]
async fn load_mcserver_list_invalid_file() {
    cleanup();
    let content = "{
        \"0\": {
            \"name\": \"myMinecraftServer\",
            \"arg\": \"-jar purpur-1.19.3-1876.jar -Xmx4G nogui\",
        }
    }";

    fs::create_dir("config").unwrap();
    let mut server_list_file = File::options().write(true).create_new(true).open("config/server_list.json").unwrap();
    io::copy(&mut content.as_bytes(), &mut server_list_file).unwrap();
    
    let mcserver_manager = Arc::new(MCServerManager {
        name: "MCServerManager".to_string(),
        config: Config::new(),
        main_thread: Arc::new(None.into()),
        status: Status::Stopped.into(),

        mcserver_list: vec![].into()
    });


    mcserver_manager.load_mcserver_list().await.unwrap_err();

    File::options().write(true).create_new(true).open("config/server_list.json").unwrap();
    File::options().write(true).create_new(true).open("config/invalid_server_list.json").unwrap_err();
    File::options().write(true).create_new(true).open("config/server_list_example.json").unwrap_err();
    cleanup();
}

#[tokio::test]
async fn get_mcserver() {
    let mcserver_manager = test_start().await;

    mcserver_manager.clone().impl_start(false).await.unwrap();

    let mcserver = mcserver_manager.get_mcserver("myMinecraftServer").await.unwrap();

    assert_eq!(mcserver.name(), "myMinecraftServer");
    mcserver_manager.impl_stop(false, false).await.unwrap();
    cleanup();
}
// set the `src/config::Config::shutdown_time` field to 1min to test the shutdown of the own machine
// the `src/config::AGREE_TO_EULA` const needs to be true
// the `src/config::Config::mcserver_restart_time` const needs to be 1min
#[tokio::test]
async fn main() { // this is a test for almost every function in the MCServerManager struct
    let mcserver_manager = test_start().await;

    mcserver_manager.clone().impl_start(false).await.unwrap();

    let mcserver = mcserver_manager.get_mcserver("myMinecraftServer").await.unwrap();
    let start_time = Instant::now();
    loop {
        if Status::Restarting == mcserver.status().await {
            break;
        }
        if Instant::now() - start_time > Duration::new(100, 0) {
            assert!(false, "The MCServerManager took to long to restart.");
        }
        sleep(Duration::new(1, 0)).await;
    }
    loop {
        if Status::Started == mcserver.status().await {
            break;
        }
        sleep(Duration::new(1, 0)).await;
    }
    mcserver_manager.impl_stop(false, false).await.unwrap();
    cleanup();
}