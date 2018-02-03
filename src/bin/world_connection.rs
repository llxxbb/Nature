#[macro_use]
extern crate log;
extern crate fern;
extern crate world_connection;
extern crate chrono;

use world_connection::server::{Server};

fn main() {
    setup_logger();
    // create server
    let server = Server{};
    info!("Server Created");
}

fn setup_logger() -> Result<(), fern::InitError> {
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
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}