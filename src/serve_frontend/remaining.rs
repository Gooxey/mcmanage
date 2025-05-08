use common::generated_files::paths::FRONTEND_DIR;
use goohttp::axum::{
    http::{
        StatusCode,
        Response,
        header,
        HeaderValue
    },
    body::{
        Full,
        self
    },
    response::IntoResponse,
    extract::Path,
    response::Redirect
};
use goolog::*;

use super::get_file;

pub async fn remaining(Path(file_path): Path<String>) -> impl IntoResponse {
    let full_path = FRONTEND_DIR.join(&file_path);

    if let Some(file) = get_file(&file_path).await {
        let mime_type = mime_guess::from_path(&file_path).first_or_text_plain();

        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap_or_else(|error|
                    fatal!("HttpServer", "Failed to convert the MIME of '{}' to a str. Error: {error}", full_path.display())
                ),
            )
            .body(body::boxed(Full::from(file)))
            .unwrap_or_else(|error| {
                fatal!("HttpServer", "Failed to create a response. Error: {error}")
            })
    } else {
        Redirect::to("/").into_response()
    }
}