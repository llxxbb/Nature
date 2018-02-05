extern crate hyper;

#[macro_use]
extern crate log;
extern crate fern;
extern crate world_connection;
extern crate chrono;

use world_connection::server::Server;
use world_connection::rpc::WebServer;
use hyper::server::Http;


fn main() {
    // init logger
    world_connection::util::setup_logger();

    // read config
    let config = world_connection::util::get_settings();

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

