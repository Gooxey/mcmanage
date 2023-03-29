//! This is the hearth of the [`MCManage network`](https://github.com/Gooxey/MCManage.git). It is the only application required to be run 24/7 because this is the place where the
//! other two applications connect to or get started from. Therefore all data will be stored here.


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
use fern::colors::{
    ColoredLevelConfig,
    Color
};
use include_dir::{
    Dir,
    include_dir
};
use tokio::spawn;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use common::{
    config,
    info,
    mcmanage_error::MCManageError,
    mcserver_manager::MCServerManager
};

use crate::communicator::Communicator;


mod communicator;
mod test_functions;


// TODO Save all files downloaded from the internet to the downloads folder; Save paths to files with their origin; on every use or start refresh this list in case any file got deleted
// TODO Create trace logs


/// This is necessary for compiling the website in the 'dist/' directory into the executable
const DIST_FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist");

/// This function will start the [`Logger`](Dispatch) of this application
fn setup_logger() -> Result<(), fern::InitError> {
    std::fs::create_dir_all("logs").unwrap_or_else(|erro| panic!("An error occurred while creating the directory 'logs'. Error: {erro}"));

    let colors = ColoredLevelConfig::new()
        .debug(Color::Blue)
        .error(Color::Red)
        .info(Color::Green)
        .trace(Color::White)
        .warn(Color::Yellow);

    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} | {:16.16} | {:5} | {}",
                        chrono::Local::now().format("%d.%m.%Y | %H:%M:%S"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .level(log::LevelFilter::Info)
                .chain(fern::log_file("logs/mcmanage.log")?)
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} | {:16.16} | {:5} | {}",
                        chrono::Local::now().format("%d.%m.%Y | %H:%M:%S"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .level(log::LevelFilter::Trace)
                .chain(fern::log_file("logs/mcmanage_detail.log")?)
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} | {:16.16} | {:5} | {}",
                        chrono::Local::now().format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"),
                        record.target(),
                        colors.color(record.level()),
                        message
                    ))
                })
                .level(log::LevelFilter::Info)
                .chain(std::io::stdout())
        )
        .apply()?;
    Ok(())
}

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

/// This function will setup all handlers for exiting the application. \
/// In particular, it will set handlers for:
///     - Ctrl+C events
///     - Panics
fn setup_exit_handlers(alive: Arc<AtomicBool>) {
    // make the application exit on any panic
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);
        std::process::exit(1);
    }));

    // exit the application in case of an ctrl+c
    ctrlc::set_handler(move || {
        alive.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

#[tokio::main]
async fn main() {
    let alive = Arc::new(AtomicBool::new(true));
    setup_exit_handlers(alive.clone());
    setup_logger().unwrap_or_else(|erro| panic!("Failed to setup the logger. Error: {erro}"));


    info!("Main", "Starting...");

    info!("Main", "Exporting the website...");
    load_website();

    let addr = SocketAddr::from((
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        config::website_port().await,
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


    let communicator = Communicator::new().await;
    let mcserver_manager = MCServerManager::new().await;

    communicator.start();
    mcserver_manager.start();

    while alive.load(Ordering::SeqCst) {}
    info!("Main", "A Ctrl+C got registered. This application will now shut down...");

    if let Err(MCManageError::NotReady) = mcserver_manager.clone().impl_stop(false, false).await {
        mcserver_manager.reset().await;
    }
    if (communicator.impl_stop(false, true).await).is_err() {}
}