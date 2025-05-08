use goohttp::axum::{
    extract::Path,
    response::IntoResponse
};

pub async fn latest_server_type(Path(server): Path<String>) -> impl IntoResponse {
    "TODO".into_response()
}