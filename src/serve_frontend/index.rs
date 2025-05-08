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
    response::IntoResponse
};
use goolog::*;

use super::get_file;

pub async fn index() -> impl IntoResponse {
    let full_path = FRONTEND_DIR.join("index.html");

    if let Some(index_file) = get_file("index.html").await {
        let mime_type = mime_guess::from_path(&full_path).first_or_text_plain();

        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap_or_else(|error|
                    fatal!("HttpServer", "Failed to convert the MIME of '{}' to a str. Error: {error}", full_path.display())
                ),
            )
            .body(body::boxed(Full::from(index_file)))
            .unwrap_or_else(|error| {
                fatal!("HttpServer", "Failed to create a response. Error: {error}")
            })
    } else {
        format!("Could not find the file `{}`.", full_path.display()).into_response()
    }
}