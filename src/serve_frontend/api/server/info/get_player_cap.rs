use goohttp::axum::{
    extract::Path,
    response::IntoResponse
};

pub async fn get_player_cap(Path(server): Path<String>) -> impl IntoResponse {
    "TODO".into_response()
}