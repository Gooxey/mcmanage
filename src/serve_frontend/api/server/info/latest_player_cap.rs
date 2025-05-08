use goohttp::axum::{
    extract::Path,
    response::IntoResponse
};

pub async fn latest_player_cap(Path(server): Path<String>) -> impl IntoResponse {

    "TODO".into_response()
}