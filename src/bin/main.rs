extern crate chrono;
extern crate fern;
extern crate nature;

use nature::rpc::*;
use nature::task::*;

fn main() {
    nature::util::setup_logger().unwrap();

    // read config
//    let config = world_connection::util::get_settings();

    start_receive_threads();

    start_rocket_server().launch();
}


