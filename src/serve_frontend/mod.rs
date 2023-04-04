use std::{
    fs,
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
    },
    sync::Arc,
};

use axum::{
    http::{
        Method,
        Response,
        StatusCode,
    },
    routing::get_service,
    Router,
    Server,
};
use common::{
    config::Config,
    generated_files::paths::FRONTEND_DIR,
    info,
};
use include_dir::{
    include_dir,
    Dir,
};
use tokio::sync::Mutex;
use tower_http::{
    cors::{
        self,
        CorsLayer,
    },
    services::ServeDir,
};

use crate::serve_frontend::rest_api::impl_rest_api;

mod rest_api;

/// This is necessary for compiling the website in the 'share/frontend/' directory into the executable
const COMPILED_FRONTEND_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/share/frontend");

/// This function will extract the website's files into the folder at [`struct@FRONTEND_DIR`] if none could be found.
pub fn load_website() {
    if !FRONTEND_DIR.as_path().exists() {
        if let Err(erro) = fs::create_dir_all(FRONTEND_DIR.as_path()) {
            panic!(
                "Encountered an error while creating the folder '{}'. Error: {erro}",
                FRONTEND_DIR.display()
            )
        }
    }
    // extract the standard ui if none can be found
    if FRONTEND_DIR
        .read_dir()
        .unwrap_or_else(|erro| {
            panic!(
                "Failed to read the directory at '{}'. Error: {erro}",
                FRONTEND_DIR.display()
            )
        })
        .next()
        .is_none()
    {
        COMPILED_FRONTEND_DIR
            .extract(FRONTEND_DIR.as_path())
            .unwrap_or_else(|erro| panic!("An error occurred while attempting to extract the website's files onto the drive. Error: {erro}"));
    }
}
/// This function will:
///     1. serve this application's frontend located at [`struct@FRONTEND_DIR`] until an error occurs or the application exits.
///     2. serve this application's [`REST API`](rest_api).
pub async fn serve_website(config: Arc<Mutex<Config>>) {
    /// Implement all [`routes`](Router::route) needed for serving the frontend website itself
    fn impl_frontend_routes(router: Router) -> Router {
        let frontend_dir = get_service(ServeDir::new(FRONTEND_DIR.as_path())).handle_error(|erro| async move {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("Encountered an error while serving the directory at './share/frontend'. Error: {erro}"))
                .expect("Failed to create an error body.")
        });

        router
            .route("/", frontend_dir.clone())
            .route("/:file", frontend_dir)
    }

    let addr = SocketAddr::from((
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        Config::website_port(&config).await,
    ));

    info!("Main", "Starting the webserver at 'http://{addr}'...");

    let mut router = Router::new().layer(
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(cors::Any),
    );
    router = impl_frontend_routes(router);
    router = impl_rest_api(router);

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap_or_else(|erro| {
            panic!("Encountered an error while serving the website. Error: {erro}")
        });
}
