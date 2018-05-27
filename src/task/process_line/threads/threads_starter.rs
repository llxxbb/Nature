use super::*;

pub fn start_receive_threads() {
    start_thread(&CHANNEL_ROUTE.receiver, do_route);
    start_thread(&CHANNEL_DISPATCH.receiver, do_dispatch);
    start_thread(&CHANNEL_CONVERT.receiver, do_convert);
    start_thread(&CHANNEL_STORE.receiver, Store::store_for_receive);
    start_thread(&CHANNEL_PARALLEL.receiver, do_parallel);
    start_thread(&CHANNEL_SERIAL.receiver, do_serial);
}

