extern crate multiqueue;

use data::*;
use self::multiqueue::*;
use serde::Serialize;
use fg_service::*;
use std::sync::Mutex;
use std::thread;
use std::fmt::Debug;

lazy_static! {
    pub static ref CHANNEL_DISPATCH : Channel<Carrier<StoreTaskInfo>> = Channel::new();
    pub static ref CHANNEL_CONVERT : Channel<Carrier<ConverterInfo>> = Channel::new();
    pub static ref CHANNEL_STORE : Channel<Carrier<StoreTaskInfo>> = Channel::new();
    pub static ref CHANNEL_PARALLEL : Channel<Carrier<ParallelBatchInstance>> = Channel::new();
    pub static ref CHANNEL_SERIAL : Channel<Carrier<SerialBatchInstance>> = Channel::new();
}

pub fn start_thread<T, F>(receiver: &'static Mutex<MPMCReceiver<Carrier<T>>>, f: F)
    where
        T: Serialize + Send + Debug,
        F: 'static + Fn(Carrier<T>) + Send
{
    use std::ops::Deref;
    thread::spawn(move || {
        let guard = receiver.lock().unwrap();
        let receiver = guard.deref();
        for next in receiver {
            f(next);
        }
    });
}

pub struct Channel<T> {
    pub sender: Mutex<MPMCSender<T>>,
    pub receiver: Mutex<MPMCReceiver<T>>,
}

impl<T> Channel<T> {
    pub fn new() -> Channel<T> {
        let (sx, rx) = mpmc_queue(10);
        Channel {
            sender: Mutex::new(sx),
            receiver: Mutex::new(rx),
        }
    }
}