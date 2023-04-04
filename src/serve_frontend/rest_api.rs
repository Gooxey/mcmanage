//! This module implements a REST API for this application

use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
    routing::get,
    Router,
};

/// This function will set all [`routes`](Router::route) for the REST API.
pub fn impl_rest_api(router: Router) -> Router {
    router.route("/hello", get(hello))
}

/// This function is used in the '/hello' [`route`](Router::route). \
/// This is just used for testing the REST API.
pub async fn hello() -> impl IntoResponse {
    println!("Hello from client");

    Response::builder()
        .status(StatusCode::OK)
        .body("Hello got send successfully.".to_string())
        .expect("Failed to create a response body.")
}
