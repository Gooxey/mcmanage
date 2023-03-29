#![cfg(test)]

use fern::colors::{Color, ColoredLevelConfig};

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