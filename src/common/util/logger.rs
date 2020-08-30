extern crate chrono;
extern crate fern;

use std::env;
use std::io::stdout;
use std::str::FromStr;

use log::LevelFilter;

use crate::common::{NatureError, Result};

pub fn setup_logger() -> Result<()> {
    // get log level from env
    let level = match env::var("LOG_LEVEL").or::<String>(Ok("info".to_string())) {
        Ok(rtn) => rtn,
        Err(e) => return Err(NatureError::SystemError(e.to_string()))
    };
    let level_filter = match LevelFilter::from_str(&level) {
        Ok(rtn) => rtn,
        Err(e) => return Err(NatureError::SystemError(e.to_string()))
    };

    match fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level_filter)
        .level_for("tokio_core", log::LevelFilter::Error)
        .level_for("tokio_reactor", log::LevelFilter::Error)
        .level_for("hyper", log::LevelFilter::Error)
        .chain(stdout())
        .chain(fern::log_file("output.log")?)
        .apply() {
        Ok(_) => (),
        Err(e) => return Err(NatureError::SystemError(e.to_string()))
    };
    info!("--------------------logger initialized for level : {}---------------------", level);
    Ok(())
}
