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
    start_thread(&CHANNEL_ROUTE.receiver, RouteTask::do_route);
    start_thread(&CHANNEL_DISPATCH.receiver, DispatchTask::do_dispatch);
    start_thread(&CHANNEL_CONVERT.receiver, ConvertTaskImpl::do_convert);
    start_thread(&CHANNEL_STORE.receiver, StoreTask::do_store);
    start_thread(&CHANNEL_PARALLEL.receiver, ParallelTask::do_parallel);
    start_thread(&CHANNEL_SERIAL.receiver, QueueTask::do_serial);
}

