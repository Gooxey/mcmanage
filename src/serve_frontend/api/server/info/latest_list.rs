use axum::{response::IntoResponse, http::{Response, StatusCode}};

use crate::SERVER_MANAGER;

/// Get the time the list of Minecraft servers got updated.
pub async fn latest_list() -> impl IntoResponse {
    let latest_list = SERVER_MANAGER.get_latest().await;
    let latest_list_json = serde_json::to_string(&latest_list).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .body(latest_list_json)
        .expect("Failed to create a response body.")
}