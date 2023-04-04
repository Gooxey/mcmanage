#![cfg(test)]

use super::*;
use crate::test_functions::{
    self,
    cleanup,
};

async fn start_test() -> MCServerType {
    test_functions::start_test();
    MCServerType::new(&Config::new().await, "purpur", "MyMCServer")
}

#[tokio::test]
async fn new() {
    let my_mcserver_type = start_test().await;

    assert_eq!(my_mcserver_type.server_type, "purpur".to_string());

    cleanup();
}

// get_message and get_message_vector got both indirectly tested by the tests below

#[tokio::test]
async fn get_started() {
    let my_mcserver_type = start_test().await;

    assert_eq!(
        my_mcserver_type.get_started().await,
        [" INFO]: Done (", ")! For help, type \"help\""]
    );

    cleanup();
}
#[tokio::test]
async fn get_player_joined() {
    let my_mcserver_type = start_test().await;

    assert_eq!(
        my_mcserver_type.get_player_joined().await[0],
        " joined the game"
    );

    cleanup();
}
#[tokio::test]
async fn get_player_left() {
    let my_mcserver_type = start_test().await;

    assert_eq!(my_mcserver_type.get_player_left().await[0], "left the game");

    cleanup();
}

#[tokio::test]
async fn get_player_name_joined() {
    let my_mcserver_type = start_test().await;

    let name = my_mcserver_type
        .get_player_name_joined("[13:53:51 INFO]: Gooxey joined the game")
        .await
        .unwrap();

    assert_eq!(name, "Gooxey");

    cleanup();
}
#[tokio::test]
async fn get_player_name_left() {
    let my_mcserver_type = start_test().await;

    let name = my_mcserver_type
        .get_player_name_left("[13:53:51 INFO]: Gooxey left the game")
        .await
        .unwrap();

    assert_eq!(name, "Gooxey");

    cleanup();
}
