use flow::*;
use nature_common::*;
use nature_db::*;
use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

/// `CHANNEL_PARALLEL` & `CHANNEL_SERIAL` are used to short caller response time
lazy_static! {
    pub static ref CHANNEL_STORE : Channel<(StoreTaskInfo,RawTask)> = Channel::new();
    pub static ref CHANNEL_STORED : Channel<(StoreTaskInfo,RawTask)> = Channel::new();
    pub static ref CHANNEL_CONVERT : Channel<(ConverterInfo,RawTask)> = Channel::new();
    pub static ref CHANNEL_CONVERTED : Channel<(ConverterInfo,Converted)> = Channel::new();
    pub static ref CHANNEL_PARALLEL : Channel<(ParallelBatchInstance,RawTask)> = Channel::new();
    pub static ref CHANNEL_SERIAL : Channel<(SerialBatchInstance,RawTask)> = Channel::new();
}

pub fn start_receive_threads() -> Vec<JoinHandle<()>> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    info!("to start receive threads");
//    threads.push(start_thread(&CHANNEL_STORE.receiver, StoreServiceImpl::store_test));
    threads.push(start_thread(&CHANNEL_STORE.receiver, ControllerImpl::channel_store));
    threads.push(start_thread(&CHANNEL_STORED.receiver, ControllerImpl::channel_stored));
    threads.push(start_thread(&CHANNEL_CONVERT.receiver, ControllerImpl::channel_convert));
    threads.push(start_thread(&CHANNEL_CONVERTED.receiver, ControllerImpl::channel_converted));
    // used to improve caller response time
    threads.push(start_thread(&CHANNEL_SERIAL.receiver, ControllerImpl::channel_serial));
    threads.push(start_thread(&CHANNEL_PARALLEL.receiver, ControllerImpl::channel_parallel));
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