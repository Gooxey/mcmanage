use goohttp::axum::{
    extract::Path,
    response::IntoResponse
};

pub async fn get_version(Path(server): Path<String>) -> impl IntoResponse {
    "TODO".into_response()
}