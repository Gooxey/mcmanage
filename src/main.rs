//! This is the hearth of the [`MCManage network`](https://github.com/Gooxey/MCManage.git). It is the only application required to be run 24/7 because this is the place where the
//! other two applications connect to or get started from. Therefore all data will be stored here.


// TODO add descriptions to the config files


#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]


use std::{
    fs,
    io::ErrorKind,
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr
    },
    sync::{
        Arc,
        atomic::{
            Ordering,
            AtomicBool
        }
    }
};

use axum::{
    body::{
        Body,
        boxed
    },
    http::{
        Response,
        StatusCode
    },
    Router,
    routing::get, Server
};
use include_dir::{
    Dir,
    include_dir
};
use tokio::spawn;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use common::{
    config::Config,
    info,
    mcmanage_error::MCManageError,
    mcserver_manager::MCServerManager
};

use crate::communicator::Communicator;

// TODO Constantly check for changes in file system (config files get edited)

mod communicator;

/// This is necessary for compiling the website in the 'dist/' directory into the executable
const DIST_FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist");

/// This function will extract the website's files into the folder `website`. \
/// To ensure that the data is valid, the old data on the system will be wiped and then replaced with the one stored inside the executable.
fn load_website() {    
    match fs::remove_dir_all("website") {
        Ok(_) => {}
        Err(erro) if ErrorKind::NotFound == erro.kind() => {}
        Err(erro) => {
            panic!("Encountered an error while deleting the directory 'website'. Error: {erro}")
        }
    }
    match fs::create_dir("website") {
        Ok(_) => {}
        Err(erro) if ErrorKind::AlreadyExists == erro.kind() => {}
        Err(erro) => {
            panic!("Encountered an error while creating the folder 'website'. Error: {erro}")
        }
    }
    if let Err(erro) = DIST_FILES.extract("website") {
        panic!("An error occurred while attempting to extract the website's files onto the drive.  Error: {erro}")
    }
}
/// This task serves the website until an error occurs or the application exits.
async fn serve_website(addr: SocketAddr, app: Router) {
    if let Err(erro) = Server::bind(&addr).serve(app.into_make_service()).await {
        panic!("Encountered an error while serving the website. Error: {erro}");
    }
}

#[tokio::main]
async fn main() {
    info!("Main", "Starting...");

    let config = Config::new();
    let communicator = Communicator::new().await;
    let mcserver_manager = MCServerManager::new();
    
    info!("Main", "Exporting the website...");
    load_website();

    let addr = SocketAddr::from((
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        *config.lock().await.website_port(),
    ));
    info!("Main", "Starting the webserver at 'http://{addr}'...");

    let app = Router::new().fallback_service(get(
        |req| async move {
            match ServeDir::new("website").oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(boxed(Body::from(format!("error: {err}"))))
                        .expect("error response")
                }
            }
        }
    ));
    spawn(serve_website(addr, app));


    communicator.start();
    mcserver_manager.start();


    // exit the application in case of an ctrl+c
    let alive = Arc::new(AtomicBool::new(true));
    
    let alive_clone = alive.clone();
    ctrlc::set_handler(move || {
        alive_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    
    while alive.load(Ordering::SeqCst) {}
    info!("Main", "A Ctrl+C got registered. This application will now shut down...");


    if let Err(MCManageError::NotReady) = mcserver_manager.clone().impl_stop(false, false).await {
        mcserver_manager.reset().await; // FIXME Will not stop when: An error occurred while starting the Minecraft Server myFirstServer. Error: Der Verzeichnisname ist ungültig. (os error 267)
    }
    if (communicator.impl_stop(false, true).await).is_err() {}
}