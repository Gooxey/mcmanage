#![cfg(test)]


use std::fs;
use std::fs::File;
use std::io;
use std::time::Duration;
use reqwest;
use tokio::sync::oneshot::channel;
use tokio::task::spawn_blocking;

use crate::test_functions::cleanup;

use super::*;


async fn new_mcserver() -> Arc<MCServer> {
    cleanup();
    spawn_blocking(|| copy_minecraft_server()).await.unwrap();

    MCServer::new(
        "myMinecraftServer",
        &Config::new(),
        "-jar server.jar nogui",
        MCServerType::new("vanilla", "myMinecraftServer")
    )
}
fn new_mcserver_no_download() -> Arc<MCServer> {
    cleanup();
    
    MCServer::new(
        "myMinecraftServer",
        &Config::new(),
        "-jar server.jar nogui",
        MCServerType::new("vanilla", "myMinecraftServer")
    )
}
fn download_minecraft_server() {
    if Path::exists(Path::new("test_files/server.jar")) {
        return;
    } else {
        fs::create_dir_all("test_files").expect("An error occurred while creating the test_files dir");
        
        // https://api.purpurmc.org/v2/purpur/1.19.3/1876/download
        let mut data = reqwest::blocking::get("https://piston-data.mojang.com/v1/objects/c9df48efed58511cdd0213c56b9013a7b5c9ac1f/server.jar").expect("An error occurred while downloading the Minecraft server");
        let mut file = fs::File::create("test_files/server.jar").expect("failed to create file `server.jar`");
        io::copy(&mut data, &mut file).expect("failed to copy content to file");
    }
}
fn copy_minecraft_server() {
    download_minecraft_server();

    fs::create_dir_all("servers/myMinecraftServer").expect("An error occurred while creating the servers dir");
    fs::copy("test_files/server.jar", "servers/myMinecraftServer/server.jar").expect("Failed to copy the Minecraft server");
}


#[tokio::test]
async fn reset() {
    let mcserver = new_mcserver_no_download();

    *mcserver.status.lock().await = Status::Started;
    *mcserver.players.lock().await = vec!["hello".to_owned()];

    mcserver.reset().await;

    assert_eq!(*mcserver.status.lock().await, Status::Stopped);
    assert_eq!(mcserver.players.lock().await.len(), 0);
    cleanup();
}

#[tokio::test]
async fn start() {
    let mcserver = new_mcserver().await;

    mcserver.clone().impl_start(false).await.unwrap();
    if let None = *mcserver.minecraft_server.lock().await {
        assert!(false, "Expected minecraft_server field to be filled.");
    }
    if let None = *mcserver.main_thread.lock().await {
        assert!(false, "Expected main_thread field to be filled.");
    }
    loop {
        if let Status::Started = *mcserver.status.lock().await {
            break;
        } else if let Status::Stopped = *mcserver.status.lock().await {
            assert!(false, "The MCServer canceled its startup because the EULA was not accepted.");
        }
    }
    mcserver.impl_stop(false, false).await.unwrap();
    cleanup();
}
#[tokio::test]
async fn stop() {
    let mcserver = new_mcserver().await;

    mcserver.clone().impl_start(false).await.unwrap();
    loop {
        if let Err(_) = mcserver.clone().impl_stop(false, false).await {
        }
        else {
            break;
        }
    }
    assert_eq!(*mcserver.status.lock().await, Status::Stopped);
    if let Some(_) = *mcserver.minecraft_server.lock().await {
        assert!(false, "Expected minecraft_server field to be empty.");
    }
    if let Some(_) = *mcserver.main_thread.lock().await {
        assert!(false, "Expected main_thread field to be empty.");
    }
    cleanup();
}
#[tokio::test]
async fn restart() {
    let mcserver = new_mcserver().await;

    mcserver.clone().impl_start(false).await.unwrap();
    loop {
        if let Status::Started = *mcserver.status.lock().await {
            break;
        }
    }
    loop {
        if let Err(_) = mcserver.clone().impl_restart().await {
        }
        else {
            break;
        }
    }
    if let None = *mcserver.minecraft_server.lock().await {
        assert!(false, "Expected minecraft_server field to be filled.");
    }
    if let None = *mcserver.main_thread.lock().await {
        assert!(false, "Expected main_thread field to be filled.");
    }
    if let Status::Started = *mcserver.status.lock().await {
    } else {
        assert!(false, "Expected status field to be Status::Started.");
    };
    mcserver.impl_stop(false, false).await.unwrap();
    cleanup();
}

#[tokio::test]
async fn send_input() {
    let mcserver = new_mcserver().await;
    let expected_string = "] [Server thread/INFO]: Unknown or incomplete command, see below for error";

    mcserver.clone().impl_start(false).await.unwrap();
    loop {
        if let Status::Started = *mcserver.status.lock().await {
            break;
        }
    }

    mcserver.clone().send_input("invalid command").await;

    sleep(Duration::new(1, 0)).await;

    let mut out = "".to_string();
    if let Err(_) = File::options().read(true).open("./logs/myMinecraftServer.txt").unwrap().read_to_string(&mut out) {}

    if !out.contains(expected_string) {
        assert!(false, "Expected `{expected_string}` in log. Found: {out}")
    }
    mcserver.impl_stop(false, false).await.unwrap();
    cleanup();
}
#[tokio::test]
async fn save_output() {
    let mcserver = new_mcserver_no_download();

    mcserver.save_output("Test line").await;

    let mut out = "".to_string();
    if let Err(_) = File::options().read(true).open("./logs/myMinecraftServer.txt").unwrap().read_to_string(&mut out) {}

    assert_eq!(out, "Test line\n")
}
#[tokio::test]
async fn check_started() {
    let mcserver = new_mcserver_no_download();

    let (tx, _rx) = channel();

    if let Some(_) = mcserver.check_started("[18:21:56] [Server thread/INFO]: Done (16.756s)! For help, type \"help\"", tx).await.unwrap() {
        assert!(false, "Expected function to detect a 'start'");
    }
    if let Status::Started = *mcserver.status.lock().await {
    } else {
        assert!(false, "Expected status field to be Status::Started.");
    };
    cleanup();
}
#[tokio::test]
async fn check_player_activity_connect() {
    let mcserver = new_mcserver_no_download();

    mcserver.check_player_activity("[13:53:51 INFO]: Gooxey joined the game").await.unwrap();
    assert_eq!(*mcserver.players.lock().await, vec!["Gooxey".to_owned()], "Expected Gooxey to be in the players list.");
    cleanup();
}
#[tokio::test]
async fn check_player_activity_disconnect() {
    let mcserver = new_mcserver_no_download();
    mcserver.check_player_activity("[13:53:51 INFO]: Gooxey joined the game").await.unwrap();

    mcserver.check_player_activity("[13:53:51 INFO]: Gooxey left the game").await.unwrap();
    let vec: Vec<String> = vec![];
    assert_eq!(*mcserver.players.lock().await, vec, "Expected no one to be in the players list.");
    cleanup();
}
#[tokio::test]
async fn agree_to_eula_already_accepted() {
    let mcserver = new_mcserver_no_download();

    fs::create_dir_all("./servers/myMinecraftServer").unwrap();
    let mut file = File::options().write(true).create_new(true).open("./servers/myMinecraftServer/eula.txt").unwrap();
    let text = "eula=true";
    io::copy(&mut text.as_bytes(), &mut file).unwrap();

    mcserver.agree_to_eula().await.unwrap();

    let mut eula_txt = "".to_string();
    if let Err(_) = File::options().read(true).open(mcserver.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text has been changed")
    }
    cleanup();
}
#[tokio::test]
async fn agree_to_eula_already_not_accepted() {
    let mcserver = new_mcserver_no_download();

    fs::create_dir_all("./servers/myMinecraftServer").unwrap();
    let mut file = File::options().write(true).create_new(true).open("./servers/myMinecraftServer/eula.txt").unwrap();
    let text = "eula=false";
    io::copy(&mut text.as_bytes(), &mut file).unwrap();

    mcserver.agree_to_eula().await.unwrap();

    let mut eula_txt = "".to_string();
    if let Err(_) = File::options().read(true).open(mcserver.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text is still false")
    }
    cleanup();
}
#[tokio::test]
async fn agree_to_eula_not_existing() {
    let mcserver = new_mcserver_no_download();

    fs::create_dir_all("./servers/myMinecraftServer").unwrap();

    mcserver.agree_to_eula().await.unwrap();

    let mut eula_txt = "".to_string();
    if let Err(_) = File::options().read(true).open(mcserver.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text is still false")
    }
    cleanup();
}