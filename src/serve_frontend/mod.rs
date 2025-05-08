use std::{
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
    },
};

use goohttp::axum::{
    http::{
        Method,
    },
    routing::get,
    Server,
};
use common::{
    config::Config,
    generated_files::paths::FRONTEND_DIR,
};
use goolog::*;
use include_dir::{
    include_dir,
    Dir,
};
use tokio::fs::read;
use tower_http::{
    cors::{
        self,
        CorsLayer,
    },
};
use goohttp::*;

/// This is necessary for compiling the website in the 'share/frontend/' directory into the executable
const COMPILED_FRONTEND_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/share/frontend");

/// Get a file from the frontend dir [`FRONTEND_DIR`]. If it is empty the compiled frontend dir will be used. \
/// \
/// The path given should be relative to the [`FRONTEND_DIR`]. So for the file at `FRONTEND_DIR/index.html` the path would
/// be `index.html`.
async fn get_file(path: &str) -> Option<Vec<u8>> {
    if FRONTEND_DIR.read_dir().unwrap_or_else(|error| {
        fatal!("HttpServer", "The directory `{}` should exist. Error: {error}", FRONTEND_DIR.display())
    }).next().is_some() {
        let full_path = FRONTEND_DIR.join(path);
        match read(full_path.clone()).await {
            Ok(file) => {
                file.into()
            }
            Err(_) => {
                None
            }
        }
    } else {
        COMPILED_FRONTEND_DIR
            .get_file(path)
            .and_then(|file| {
                file.contents().to_vec().into()
            })
    }
}

router! {
    frontend_router {
        index, get;
        remaining, get;
        api
    }
}

// TODO fix doc
/// This function will:
///     1. serve this application's frontend located at [`struct@FRONTEND_DIR`] until an error occurs or the application exits.
///     2. serve this application's [`REST API`](rest_api).
pub async fn serve_frontend() {
    let addr = SocketAddr::from((
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        Config::website_port().await,
    ));

    info!("Main", "Starting the webserver at 'http://{addr}'...");

    let router = frontend_router()
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(cors::Any),
        );

    if let Err(error) = Server::bind(&addr).serve(router.into_make_service()).await {
        fatal!("HttpServer", "Encountered an error while serving the website. Error: {error}")
    }
}
