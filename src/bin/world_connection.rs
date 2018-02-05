extern crate hyper;

#[macro_use]
extern crate log;
extern crate fern;
extern crate world_connection;
extern crate chrono;

use world_connection::server::Server;
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
    world_connection::rpc::web_rpc(config.get("port").unwrap());
}



