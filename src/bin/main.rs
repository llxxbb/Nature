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
    start_thread(&CHANNEL_ROUTE.receiver, RouteTask::do_route_task);
    start_thread(&CHANNEL_DISPATCH.receiver, DispatchTask::do_dispatch_task);
    start_thread(&CHANNEL_CONVERT.receiver, ConvertTaskImpl::do_convert_task);
    start_thread(&CHANNEL_STORE.receiver, StoreTask::do_store_task);
    start_thread(&CHANNEL_PARALLEL.receiver, ParallelTask::do_parallel_task);
    start_thread(&CHANNEL_SERIAL.receiver, QueueTask::do_serial_task);
}

