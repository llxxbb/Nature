extern crate chrono;
extern crate fern;
extern crate world_connection;

use world_connection::rpc::*;
use world_connection::service::Service;

fn main() {
    // init logger
    world_connection::util::setup_logger().unwrap();

    // read config
//    let config = world_connection::util::get_settings();

    start_rocket_server(&Service).launch();
}



