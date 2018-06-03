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

fn start_receive_threads() {
    start_thread(&CHANNEL_ROUTE.receiver, do_route);
    start_thread(&CHANNEL_DISPATCH.receiver, do_dispatch);
    start_thread(&CHANNEL_CONVERT.receiver, ConvertTaskImpl::do_convert);
    start_thread(&CHANNEL_STORE.receiver, StoreTaskImpl::do_store);
    start_thread(&CHANNEL_PARALLEL.receiver, do_parallel);
    start_thread(&CHANNEL_SERIAL.receiver, do_serial);
}

