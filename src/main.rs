//! This is the hearth of the [`MCManage network`](https://github.com/Gooxey/MCManage.git). It is the only application required to be run 24/7 because this is the place where the
//! other two applications connect to or get started from. Therefore all data will be stored here.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]

use std::sync::{
    atomic::{
        AtomicBool,
        Ordering,
    },
    Arc,
};

use common::{
    config::Config,
    generated_files::paths::LOGS_DIR,
    info,
    mcmanage_error::MCManageError,
    mcserver_manager::MCServerManager,
};
use fern::colors::{
    Color,
    ColoredLevelConfig,
};
use tokio::spawn;

use crate::serve_frontend::{
    load_website,
    serve_website,
};

mod serve_frontend;

// TODO Save all files downloaded from the internet to the downloads folder; Save paths to files with their origin; on every use or start refresh this list in case any file got deleted
// TODO Create trace logs

/// This function will start the [`Logger`](fern::Dispatch) of this application
fn setup_logger() -> Result<(), fern::InitError> {
    std::fs::create_dir_all(LOGS_DIR.as_path()).unwrap_or_else(|erro| {
        panic!(
            "An error occurred while creating the directory '{}'. Error: {erro}",
            LOGS_DIR.display()
        )
    });

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
                .chain(fern::log_file(LOGS_DIR.join("mcmanage.log"))?),
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} | {:16.16} | {:5} | {}",
                        chrono::Local::now().format(
                            "\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"
                        ),
                        record.target(),
                        colors.color(record.level()),
                        message
                    ))
                })
                .level(log::LevelFilter::Info)
                .chain(std::io::stdout()),
        )
        .apply()?;
    Ok(())
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
    })
    .expect("Error setting Ctrl-C handler");
}

#[tokio::main]
async fn main() {
    let alive = Arc::new(AtomicBool::new(true));
    setup_exit_handlers(alive.clone());
    setup_logger().unwrap_or_else(|erro| panic!("Failed to setup the logger. Error: {erro}"));

    info!("Main", "Starting...");

    info!("Main", "Exporting the website...");
    load_website();

    let config = Config::new().await;
    let mcserver_manager = MCServerManager::new(&config).await;

    spawn(serve_website(config.clone()));

    mcserver_manager.start();

    while alive.load(Ordering::SeqCst) {}
    info!(
        "Main",
        "A Ctrl+C got registered. This application will now shut down..."
    );

    if let Err(MCManageError::NotReady) = mcserver_manager.clone().impl_stop(false, false).await {
        mcserver_manager.reset().await;
    }
}
