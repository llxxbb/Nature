extern crate chrono;
extern crate fern;
extern crate nature;

use nature::global::*;
use nature::rpc::*;
use nature::task::*;

fn main() {
    nature::util::setup_logger().unwrap();

    // read config
//    let config = world_connection::util::get_settings();

    start_task_route(&CHANNEL_ROUTE.receiver);

    start_rocket_server().launch();
}


