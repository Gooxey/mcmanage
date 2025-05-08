#![cfg(test)]

use std::time::Duration;

use tokio::{
    fs::OpenOptions,
    spawn,
    sync::oneshot::channel,
};

use super::*;
use crate::{
    generated_files::{
        default_files::get_example_content,
        paths::{
            SERVERS_DIR,
            SERVER_LIST_FILE,
            SERVER_LOGS_DIR,
        },
    },
    test_functions::{
        cleanup,
        start_test,
    },
};

async fn new_server() -> Arc<Server> {
    start_test();

    Server::new(
        "myMinecraftServer",
        ServerItem {
            args: "-jar purpur-1.19.3-1933.jar nogui".to_string(),
            download_from: "https://api.purpurmc.org/v2/purpur/1.19.3/1933/download".to_string(),
            server_type: "purpur".to_string(),
            restart_time: Duration::new(0, 0),
        },
    )
    .await
}

#[tokio::test]
async fn reset() {
    let server = new_server().await;

    *server.status.lock().await = Status::Started;
    server.players.lock().await.0 = vec!["hello".to_owned()];
    server.players.lock().await.1 = Utc::now();

    server.reset().await;

    assert_eq!(*server.status.lock().await, Status::Stopped);
    assert_eq!(server.players.lock().await.0.len(), 0);
    cleanup();
}

#[tokio::test]
async fn start() {
    let server = new_server().await;

    server.clone().impl_start(false).await.unwrap();
    if let None = *server.minecraft_server.lock().await {
        assert!(false, "Expected minecraft_server field to be filled.");
    }
    if let None = *server.main_thread.lock().await {
        assert!(false, "Expected main_thread field to be filled.");
    }
    loop {
        if let Status::Started = *server.status.lock().await {
            break;
        } else if let Status::Stopped = *server.status.lock().await {
            assert!(
                false,
                "The Server canceled its startup because the EULA was not accepted."
            );
        }
    }
    server.impl_stop(false, false).await.unwrap();
    cleanup();
}
#[tokio::test]
async fn stop() {
    let server = new_server().await;

    server.clone().impl_start(false).await.unwrap();
    loop {
        if let Err(_) = server.clone().impl_stop(false, false).await {
        } else {
            break;
        }
    }
    assert_eq!(*server.status.lock().await, Status::Stopped);
    if let Some(_) = *server.minecraft_server.lock().await {
        assert!(false, "Expected minecraft_server field to be empty.");
    }
    if let Some(_) = *server.main_thread.lock().await {
        assert!(false, "Expected main_thread field to be empty.");
    }
    cleanup();
}
#[tokio::test]
async fn restart() {
    let server = new_server().await;

    server.clone().impl_start(false).await.unwrap();
    loop {
        if let Status::Started = *server.status.lock().await {
            break;
        }
    }
    loop {
        if let Err(_) = server.clone().impl_restart().await {
        } else {
            break;
        }
    }
    if let None = *server.minecraft_server.lock().await {
        assert!(false, "Expected minecraft_server field to be filled.");
    }
    if let None = *server.main_thread.lock().await {
        assert!(false, "Expected main_thread field to be filled.");
    }
    if let Status::Started = *server.status.lock().await {
    } else {
        assert!(false, "Expected status field to be Status::Started.");
    };
    server.impl_stop(false, false).await.unwrap();
    cleanup();
}

#[tokio::test]
async fn send_input() {
    let server = new_server().await;
    let expected_string = r#" INFO]: Unknown command. Type "/help" for help."#;

    server.clone().impl_start(false).await.unwrap();
    loop {
        if let Status::Started = *server.status.lock().await {
            break;
        }
    }

    server.clone().send_input("invalid command").await;

    sleep(Duration::new(1, 0)).await;

    let mut out = "".to_string();
    if File::open(SERVER_LOGS_DIR.join("myMinecraftServer.log"))
        .await
        .unwrap()
        .read_to_string(&mut out)
        .await
        .is_err()
    {}

    if !out.contains(expected_string) {
        assert!(false, "Expected `{expected_string}` in log. Found: {out}")
    }
    server.impl_stop(false, false).await.unwrap();
    cleanup();
}
#[tokio::test]
async fn save_output() {
    let server = new_server().await;

    server.save_output("Test line").await;

    let mut out = "".to_string();
    if let Err(_) = OpenOptions::new()
        .read(true)
        .open(SERVER_LOGS_DIR.join("myMinecraftServer.log"))
        .await
        .unwrap()
        .read_to_string(&mut out)
        .await
    {}

    assert_eq!(out, "Test line\n");
    cleanup();
}
#[tokio::test]
async fn check_started() {
    let server = new_server().await;

    let (tx, _rx) = channel();

    if let Some(_) = server
        .check_started(
            "[16:54:34 INFO]: Done (3.152s)! For help, type \"help\"",
            tx,
        )
        .await
    {
        assert!(false, "Expected function to detect a 'start'");
    }
    if let Status::Started = *server.status.lock().await {
    } else {
        assert!(false, "Expected status field to be Status::Started.");
    };
    cleanup();
}
#[tokio::test]
async fn check_player_activity_connect() {
    let server = new_server().await;

    server
        .check_player_activity("[13:53:51 INFO]: Gooxey joined the game")
        .await;
    assert_eq!(
        server.players.lock().await.0,
        vec!["Gooxey".to_owned()],
        "Expected Gooxey to be in the players list."
    );
    cleanup();
}
#[tokio::test]
async fn check_player_activity_disconnect() {
    let server = new_server().await;
    server
        .check_player_activity("[13:53:51 INFO]: Gooxey joined the game")
        .await;

    server
        .check_player_activity("[13:53:51 INFO]: Gooxey left the game")
        .await;
    let vec: Vec<String> = vec![];
    assert_eq!(
        server.players.lock().await.0,
        vec,
        "Expected no one to be in the players list."
    );
    cleanup();
}
#[tokio::test]
async fn agree_to_eula_already_accepted() {
    let server = new_server().await;

    fs::create_dir_all(SERVERS_DIR.join("myMinecraftServer"))
        .await
        .unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(SERVERS_DIR.join("myMinecraftServer").join("eula.txt"))
        .await
        .unwrap();
    let text = "eula=true";
    io::copy(&mut text.as_bytes(), &mut file).await.unwrap();

    server.agree_to_eula().await;

    let mut eula_txt = "".to_string();
    if OpenOptions::new()
        .read(true)
        .open(server.path.join("eula.txt"))
        .await
        .unwrap()
        .read_to_string(&mut eula_txt)
        .await
        .is_err()
    {}

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text has been changed")
    }
    cleanup();
}
#[tokio::test]
async fn agree_to_eula_already_not_accepted() {
    let server = new_server().await;

    fs::create_dir_all(SERVERS_DIR.join("myMinecraftServer"))
        .await
        .unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(SERVERS_DIR.join("myMinecraftServer").join("eula.txt"))
        .await
        .unwrap();
    let text = "eula=false";
    io::copy(&mut text.as_bytes(), &mut file).await.unwrap();

    server.agree_to_eula().await;

    let mut eula_txt = "".to_string();
    if OpenOptions::new()
        .read(true)
        .open(server.path.join("eula.txt"))
        .await
        .unwrap()
        .read_to_string(&mut eula_txt)
        .await
        .is_err()
    {}

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text is still false")
    }
    cleanup();
}
#[tokio::test]
async fn agree_to_eula_not_existing() {
    let server = new_server().await;

    fs::create_dir_all(SERVERS_DIR.join("myMinecraftServer"))
        .await
        .unwrap();

    server.agree_to_eula().await;

    let mut eula_txt = "".to_string();
    if OpenOptions::new()
        .read(true)
        .open(server.path.join("eula.txt"))
        .await
        .unwrap()
        .read_to_string(&mut eula_txt)
        .await
        .is_err()
    {}

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text is still false")
    }
    cleanup();
}
#[tokio::test]
async fn download_jar_jar_already_there() {
    // This test tests whether or not any panic occurs
    let server = new_server().await;

    let mut server_jar = reqwest::get("https://api.purpurmc.org/v2/purpur/1.19.3/1933/download")
        .await
        .unwrap()
        .bytes_stream();
    fs::create_dir_all(SERVERS_DIR.join("myMinecraftServer"))
        .await
        .unwrap();
    let mut jar_file = File::create(
        SERVERS_DIR
            .join("myMinecraftServer")
            .join("purpur-1.19.3-1933.jar"),
    )
    .await
    .unwrap();
    while let Some(item) = server_jar.next().await {
        io::copy(
            &mut item.unwrap_or_else(|error| panic!("An error occurred while coping the downloaded jar to a file. Error: {error}")).as_ref(),
            &mut jar_file
        ).await.unwrap_or_else(|error| panic!("An error occurred while coping the downloaded jar to a file. Error: {error}"));
    }

    server.download_jar().await;

    cleanup();
}
#[tokio::test]
async fn download_jar_jar_not_there() {
    let server = new_server().await;

    server.download_jar().await;

    assert!(
        SERVERS_DIR
            .join("myMinecraftServer")
            .join("purpur-1.19.3-1933.jar")
            .exists(),
        "A server should have been downloaded."
    );

    cleanup();
}
#[tokio::test]
async fn download_jar_no_jar_name_set() {
    start_test();
    let server = Server::new(
        "myFirstServer",
        ServerItem {
            args: "-jar nogui".to_string(),
            download_from: "https://api.purpurmc.org/v2/purpur/1.19.3/1933/download".to_string(),
            server_type: "purpur".to_string(),
            restart_time: Duration::new(0, 0),
        },
    )
    .await;

    spawn(async {
        sleep(Duration::new(1, 0)).await;

        let mut valid_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(SERVER_LIST_FILE.as_path())
            .await
            .unwrap();
        io::copy(
            &mut get_example_content(&SERVER_LIST_FILE).1.as_bytes(),
            &mut valid_file,
        )
        .await
        .unwrap();

        let mut server_list = load_toml(&SERVER_LIST_FILE, "test", true).await.unwrap();
        *server_list
            .get_mut("myFirstServer")
            .unwrap()
            .as_table_mut()
            .unwrap()
            .get_mut("args")
            .unwrap() = toml::Value::from("-jar purpur-1.19.3-1933.jar nogui");
        server_list.remove("mySecondServer");

        fs::write(
            SERVER_LIST_FILE.as_path(),
            toml::to_string(&server_list).unwrap().as_bytes(),
        )
        .await
        .unwrap();
    });

    server.download_jar().await;

    assert!(
        SERVERS_DIR
            .join("myFirstServer")
            .join("purpur-1.19.3-1933.jar")
            .exists(),
        "A server should have been downloaded."
    );

    cleanup();
}
#[tokio::test]
async fn download_jar_only_one_arg() {
    start_test();
    let server = Server::new(
        "myFirstServer",
        ServerItem {
            args: "-jar".to_string(),
            download_from: "https://api.purpurmc.org/v2/purpur/1.19.3/1933/download".to_string(),
            server_type: "purpur".to_string(),
            restart_time: Duration::new(0, 0),
        },
    )
    .await;

    spawn(async {
        sleep(Duration::new(1, 0)).await;

        let mut valid_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(SERVER_LIST_FILE.as_path())
            .await
            .unwrap();
        io::copy(
            &mut get_example_content(&SERVER_LIST_FILE).1.as_bytes(),
            &mut valid_file,
        )
        .await
        .unwrap();

        let mut server_list = load_toml(&SERVER_LIST_FILE, "test", true).await.unwrap();
        *server_list
            .get_mut("myFirstServer")
            .unwrap()
            .as_table_mut()
            .unwrap()
            .get_mut("args")
            .unwrap() = toml::Value::from("-jar purpur-1.19.3-1933.jar nogui");
        server_list.remove("mySecondServer");

        fs::write(
            SERVER_LIST_FILE.as_path(),
            toml::to_string(&server_list).unwrap().as_bytes(),
        )
        .await
        .unwrap();
    });

    server.download_jar().await;

    assert!(
        SERVERS_DIR
            .join("myFirstServer")
            .join("purpur-1.19.3-1933.jar")
            .exists(),
        "A server should have been downloaded."
    );

    cleanup();
}
#[tokio::test]
async fn download_jar_no_download_from_set() {
    start_test();
    let server = Server::new(
        "myFirstServer",
        ServerItem {
            args: "-jar purpur-1.19.3-1933.jar nogui".to_string(),
            download_from: "".to_string(),
            server_type: "purpur".to_string(),
            restart_time: Duration::new(0, 0),
        },
    )
    .await;

    spawn(async {
        sleep(Duration::new(1, 0)).await;

        let mut valid_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(SERVER_LIST_FILE.as_path())
            .await
            .unwrap();
        io::copy(
            &mut get_example_content(&SERVER_LIST_FILE).1.as_bytes(),
            &mut valid_file,
        )
        .await
        .unwrap();

        let mut server_list = load_toml(&SERVER_LIST_FILE, "test", true).await.unwrap();
        *server_list
            .get_mut("myFirstServer")
            .unwrap()
            .as_table_mut()
            .unwrap()
            .get_mut("download_from")
            .unwrap() =
            toml::Value::from("https://api.purpurmc.org/v2/purpur/1.19.3/1933/download");
        server_list.remove("mySecondServer");

        fs::write(
            SERVER_LIST_FILE.as_path(),
            toml::to_string(&server_list).unwrap().as_bytes(),
        )
        .await
        .unwrap();
    });

    server.download_jar().await;

    assert!(
        SERVERS_DIR
            .join("myFirstServer")
            .join("purpur-1.19.3-1933.jar")
            .exists(),
        "A server should have been downloaded."
    );

    cleanup();
}
