extern crate chrono;
extern crate fern;
extern crate nature;

use nature::global::*;
use nature::rpc::*;
use nature::service::*;
use nature::task::*;

fn main() {
    nature::util::setup_logger().unwrap();

    // read config
//    let config = world_connection::util::get_settings();

    start_receive_threads();

    start_rocket_server().launch();
}

fn start_receive_threads() {
    start_thread(&CHANNEL_DISPATCH.receiver, DispatchService::do_dispatch_task);
    start_thread(&CHANNEL_CONVERT.receiver, ConvertService::do_convert_task);
    start_thread(&CHANNEL_STORE.receiver, StoreService::receive_store_task);
    start_thread(&CHANNEL_PARALLEL.receiver, ParallelTask::do_parallel_task);
    start_thread(&CHANNEL_SERIAL.receiver, SequentialTask::do_serial_task);
}

