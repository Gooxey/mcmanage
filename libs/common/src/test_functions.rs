#![cfg(test)]


use std::{
    fs,
    io::{
        ErrorKind,
        self
    },
    path::Path
};

use fern::colors::{ColoredLevelConfig, Color};


pub fn cleanup() {
    if let Err(_) = cleanup_dir("./servers/") {}
    if let Err(_) = cleanup_dir("./config/") {}
    if let Err(_) = cleanup_dir("./logs/") {}
}
pub fn cleanup_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            cleanup_dir(&path)?;
            if let Err(erro) = fs::remove_dir(&path) {
                match erro.kind() {
                    ErrorKind::NotFound => {}
                    _ => {
                        return Err(erro);
                    }
                }
            }
        } else {
            fs::remove_file(path)?;
        }
    }
    fs::remove_dir(path)?;
    Ok(())
}

pub fn setup_logger() {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Blue)
        .error(Color::Red)
        .info(Color::Green)
        .trace(Color::White)
        .warn(Color::Yellow);

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
        .chain(fern::Output::call(|record| println!("{}", record.args())))
        .level(log::LevelFilter::Trace)
        .apply().unwrap();
}