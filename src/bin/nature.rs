extern crate chrono;
extern crate fern;
extern crate nature;

use nature::rpc::*;
use nature::service::NatureService;

fn main() {
    // init logger
    nature::util::setup_logger().unwrap();

    // read config
//    let config = world_connection::util::get_settings();

    start_rocket_server(&NatureService).launch();
}


