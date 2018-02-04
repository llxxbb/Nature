extern crate hyper;

#[macro_use]
extern crate log;
extern crate fern;
extern crate world_connection;
extern crate chrono;

use world_connection::server::Server;
use world_connection::rpc::WebServer;
use hyper::server::{Http};


fn main() {
    setup_logger();
    // create server
    let server = Server {};
    info!("##### Server created ---------------------------");
    // create rpc server
    web_rpc();
}

fn web_rpc() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let web_server = Http::new().bind(&addr, || Ok(WebServer)).unwrap();
    info!("##### Web service created ---------------------------");
    web_server.run().unwrap();
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
        .level_for("tokio_core", log::LevelFilter::Error)
        .level_for("hyper", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}