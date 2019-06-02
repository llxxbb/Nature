use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use nature_common::*;
use nature_db::*;

use crate::task::*;

#[allow(unused_doc_comments)]
/// `CHANNEL_PARALLEL` & `CHANNEL_SERIAL` are used to short caller response time
lazy_static! {
    pub static ref CHANNEL_STORE : Channel<(TaskForStore,RawTask)> = Channel::new();
    pub static ref CHANNEL_PARALLEL : Channel<(TaskForParallel,RawTask)> = Channel::new();
    pub static ref CHANNEL_SERIAL : Channel<(TaskForSerial,RawTask)> = Channel::new();
}

pub fn start_receive_threads() -> Vec<JoinHandle<()>> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    threads.push(start_thread(&CHANNEL_STORE.receiver, InnerController::channel_store));
    // used to improve caller response time
    threads.push(start_thread(&CHANNEL_SERIAL.receiver, InnerController::channel_serial));
    threads.push(start_thread(&CHANNEL_PARALLEL.receiver, InnerController::channel_parallel));
    info!("--------------------nature threads initialized---------------------");
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

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}