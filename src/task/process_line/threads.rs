use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread;
use super::*;
use util::*;

lazy_static! {
    pub static ref CHANNEL_ROUTE : Channel<Carrier<StoreInfo>> = Channel::new();
    pub static ref CHANNEL_DISPATCH : Channel<Carrier<RouteInfo>> = Channel::new();
    pub static ref CHANNEL_CONVERT : Channel<Carrier<ConverterInfo>> = Channel::new();
    pub static ref CHANNEL_STORE : Channel<Carrier<StoreInfo>> = Channel::new();
    pub static ref CHANNEL_PARALLEL : Channel<Carrier<ParallelBatchInstance>> = Channel::new();
    pub static ref CHANNEL_SERIAL : Channel<Carrier<SerialBatchInstance>> = Channel::new();
}

pub fn send_carrier<T>(sender: Sender<Carrier<T>>, carrier: Carrier<T>)
    where T: 'static + Sized + Serialize + Sync + Send {
    thread::spawn(move || {
        sender.send(carrier).unwrap();
    });
}

pub fn start_receive_threads() {
    start_thread(&CHANNEL_ROUTE.receiver, do_route);
    start_thread(&CHANNEL_DISPATCH.receiver, do_dispatch);
    start_thread(&CHANNEL_CONVERT.receiver, do_convert);
    start_thread(&CHANNEL_STORE.receiver, Store::store_for_receive);
    start_thread(&CHANNEL_PARALLEL.receiver, do_parallel);
    start_thread(&CHANNEL_SERIAL.receiver, do_serial);
}

fn start_thread<T, F>(receiver: &'static Mutex<Receiver<Carrier<T>>>, f: F)
    where
        T: Serialize + Send,
        F: 'static + Fn(Carrier<T>) + Send
{
    thread::spawn(move || {
        let receiver = receiver.lock().unwrap();
        let mut iter = receiver.iter();
        while let Some(next) = iter.next() {
            f(next);
        }
    });
}
