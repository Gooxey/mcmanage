#![cfg(test)]


use tokio::{fs::{OpenOptions, self}, io, spawn};

use crate::{test_functions::*, qol::load_toml_file::load_toml_replace};
use super::*;


async fn test_start() -> Arc<MCServerManager> {
    setup_logger();
    generate_server_list().await;
    MCServerManager::new().await
}
// TODO use 2 servers here as soon as the properties of a server become configurable
async fn generate_server_list() {
    cleanup();
    let content = r#"
        [myFirstServer]
        download_from = "https://api.purpurmc.org/v2/purpur/1.19.3/1933/download"
        args = "-jar purpur-1.19.3-1933.jar nogui"
        mcserver_type = "purpur"
        [myFirstServer.restart_time]
        secs = 60
        nanos = 0
    "#;

    fs::create_dir("config").await.unwrap();
    let mut server_list_file = OpenOptions::new().write(true).create_new(true).open("config/server_list.toml").await.unwrap();
    io::copy(&mut content.as_bytes(), &mut server_list_file).await.unwrap();
}

// the following two functions will also test `get_server_parameter` and `generate_valid_server_list_file`
#[tokio::test]
async fn load_mcserver_list_valid_file() {
    let mcserver_manager = test_start().await;

    mcserver_manager.load_mcserver_list().await;

    assert_eq!(mcserver_manager.mcserver_list.lock().await.len(), 1, "The function should have captured one server.");
    cleanup();
}
#[tokio::test]
async fn load_mcserver_list_invalid_file() {
    cleanup();
    let content = r#"
        [0]
        name = "myFirstServer"
        args = "-jar purpur-1.19.3-1876.jar nogui"
    }"#;

    fs::create_dir("config").await.unwrap();
    let mut server_list_file = OpenOptions::new().write(true).create_new(true).open("config/server_list.toml").await.unwrap();
    io::copy(&mut content.as_bytes(), &mut server_list_file).await.unwrap();

    let mcserver_manager = Arc::new(MCServerManager {
        name: "MCServerManager".to_string(),
        main_thread: Arc::new(None.into()),
        status: Status::Stopped.into(),

        mcserver_list: vec![].into(),
        restart_times: vec![].into()
    });

    spawn(async {
        sleep(Duration::new(1, 0)).await;

        let mut mcserver_list = load_toml_replace("config", "server_list", SERVER_LIST_EXAMPLE_DEFAULT, "Test", true).await;
        mcserver_list.remove("mySecondServer");

        fs::write("config/server_list.toml", toml::to_string(&mcserver_list).unwrap().as_bytes()).await.unwrap();
    });

    mcserver_manager.load_mcserver_list().await;

    OpenOptions::new().write(true).create_new(true).open("config/invalid_server_list.toml").await.unwrap_err();
    OpenOptions::new().write(true).create_new(true).open("config/server_list_example.toml").await.unwrap_err();
    cleanup();
}

#[tokio::test]
async fn get_mcserver() {
    let mcserver_manager = test_start().await;

    mcserver_manager.clone().impl_start(false).await.unwrap();

    let mcserver = mcserver_manager.get_mcserver("myFirstServer").await.unwrap();

    assert_eq!(mcserver.name(), "myFirstServer");

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

    let mcserver = mcserver_manager.get_mcserver("myFirstServer").await.unwrap();
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