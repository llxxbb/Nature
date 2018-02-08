extern crate hyper;

#[macro_use]
extern crate log;
extern crate fern;
extern crate world_connection;
extern crate chrono;

use hyper::server::Http;
use world_connection::server::Server;


fn main() {
    // init logger
    world_connection::util::setup_logger();

    // read config
    let config = world_connection::util::get_settings();

    // create server
    let server = Server {};
    info!("##### Server created ---------------------------");
    // create rpc server
    world_connection::rpc::web::web_rpc(config.get("port").unwrap());
}



