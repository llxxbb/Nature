extern crate fern;
extern crate log;
extern crate chrono;

use std::io::stdout;

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("tokio_core", log::LevelFilter::Error)
        .level_for("hyper", log::LevelFilter::Error)
        .chain(stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
