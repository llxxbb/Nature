extern crate fern;
extern crate chrono;
extern crate log;
extern crate config;

use std;
use std::collections::HashMap;

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
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

pub fn get_settings() -> HashMap<String, String> {
    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("settings")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP")).unwrap();
    // Print out our settings (as a HashMap)
    let content = settings.try_into::<HashMap<String, String>>().unwrap();
    info!("#### The settings.toml information are : \r\n  {:?}", content);
    content
}