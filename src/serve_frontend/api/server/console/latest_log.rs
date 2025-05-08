use axum::response::IntoResponse;

/// Return the time the latest line of log of a specified Minecraft Server got saved.
pub async fn latest_log() -> impl IntoResponse {
    todo!();
}