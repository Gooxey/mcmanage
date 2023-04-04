//! This module provides various useful functions for tests.

#![allow(unreachable_code)]
#![cfg(test)]

use crate::generated_files::paths::ROOT_DIR;

/// This function will call the cleanup function and setup a logger to print log messages to the console.
///
/// # Panics
///
/// This method will panic when called outside of the test configuration.
pub fn start_test() {
    fn setup_logger() {
        let colors = fern::colors::ColoredLevelConfig::new()
            .debug(fern::colors::Color::Blue)
            .error(fern::colors::Color::Red)
            .info(fern::colors::Color::Green)
            .trace(fern::colors::Color::White)
            .warn(fern::colors::Color::Yellow);

        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{} | {:16.16} | {:5} | {}",
                    chrono::Local::now()
                        .format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"),
                    record.target(),
                    colors.color(record.level()),
                    message
                ))
            })
            .chain(fern::Output::call(|record| println!("{}", record.args())))
            .level(log::LevelFilter::Trace)
            .apply()
            .unwrap();
    }

    cleanup();
    setup_logger();
}

/// This method will delete everything inside [`struct@ROOT_DIR`](crate::generated_files::paths::ROOT_DIR).
///
/// # Panics
///
/// This method will panic when called outside of the test configuration.
pub fn cleanup() {
    fn cleanup_dir<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();

            if entry.file_type()?.is_dir() {
                cleanup_dir(&path)?;
                if let Err(erro) = std::fs::remove_dir(&path) {
                    match erro.kind() {
                        std::io::ErrorKind::NotFound => {}
                        _ => {
                            return Err(erro);
                        }
                    }
                }
            } else {
                std::fs::remove_file(path)?;
            }
        }
        Ok(())
    }
    cleanup_dir(ROOT_DIR.as_path())
        .unwrap_or_else(|erro| panic!("Failed to remove the testing directory. Error: {erro}"));
}
