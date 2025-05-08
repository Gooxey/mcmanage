use axum::{response::IntoResponse, http::{Response, StatusCode}};
use serde_json::Value;

use crate::SERVER_MANAGER;

/// Get a list of every Minecraft server. /
/// Only their names will be returned. To get some more details see the other info commands.
pub async fn get_list() -> impl IntoResponse {
    let mut server_data_list: Vec<Value> = vec![];
    let server_list = SERVER_MANAGER.get_all().await;

    for server in server_list {
        server_data_list.push(server.name().into());
    }

    Response::builder()
        .status(StatusCode::OK)
        .body(Value::from(server_data_list).to_string())
        .expect("Failed to create a response body.")
}