use axum::response::IntoResponse;

/// Get the log of a specified server. /
/// The log returned will be starting from the specified time. To get the full log, set the time to 0
pub async fn get_log() -> impl IntoResponse {
    todo!();
}