extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;
extern crate nature;

use nature::fg_service::*;
use nature::global::*;
use nature::rpc::*;

fn main() {
    let _ = nature::util::setup_logger();

    // read config
//    let config = world_connection::util::get_settings();

    start_receive_threads();

    start_rocket_server().launch();
}

fn start_receive_threads() {
    info!("to start receive threads");
    start_thread(&CHANNEL_DISPATCH.receiver, DispatchService::do_dispatch_task);
    start_thread(&CHANNEL_CONVERT.receiver, ConvertService::do_convert_task);
    start_thread(&CHANNEL_STORE.receiver, StoreService::do_store_task);
    start_thread(&CHANNEL_PARALLEL.receiver, ParallelService::do_parallel_task);
    start_thread(&CHANNEL_SERIAL.receiver, SequentialService::do_serial_task);
}

