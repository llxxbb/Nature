use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use super::*;

/// `CHANNEL_PARALLEL` & `CHANNEL_SERIAL` are used to short caller response time
lazy_static! {
    pub static ref CHANNEL_STORE : Channel<Carrier<StoreTaskInfo>> = Channel::new();
    pub static ref CHANNEL_STORED : Channel<Carrier<StoreTaskInfo>> = Channel::new();
    pub static ref CHANNEL_CONVERT : Channel<Carrier<ConverterInfo>> = Channel::new();
    pub static ref CHANNEL_CONVERTED : Channel<Converted> = Channel::new();
    pub static ref CHANNEL_PARALLEL : Channel<Carrier<ParallelBatchInstance>> = Channel::new();
    pub static ref CHANNEL_SERIAL : Channel<Carrier<SerialBatchInstance>> = Channel::new();
}

pub fn start_receive_threads() -> Vec<JoinHandle<()>> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    info!("to start receive threads");
    threads.push(start_thread(&CHANNEL_STORE.receiver, StoreService::store));
    threads.push(start_thread(&CHANNEL_STORED.receiver, Controller::channel_stored));
    threads.push(start_thread(&CHANNEL_CONVERT.receiver, ConvertService::convert));
    threads.push(start_thread(&CHANNEL_CONVERTED.receiver, Controller::channel_converted));
    // used to improve caller response time
    threads.push(start_thread(&CHANNEL_PARALLEL.receiver, ParallelService::do_parallel_task));
    threads.push(start_thread(&CHANNEL_SERIAL.receiver, SequentialService::do_serial_task));
    threads
}


pub fn start_thread<T, F>(receiver: &'static Mutex<Receiver<T>>, f: F) -> JoinHandle<()>
    where
        T: Send,
        F: 'static + Fn(T) + Send
{
    use std::ops::Deref;
    thread::spawn(move || {
        let guard = receiver.lock().unwrap();
        let receiver = guard.deref();
        for next in receiver {
            f(next);
        }
    })
}


pub struct Channel<T> {
    pub sender: Mutex<Sender<T>>,
    pub receiver: Mutex<Receiver<T>>,
}

impl<T> Channel<T> {
    pub fn new() -> Channel<T> {
        let (sx, rx) = channel();
        Channel {
            sender: Mutex::new(sx),
            receiver: Mutex::new(rx),
        }
    }
}