use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use crate::dispatcher::channel_convert;
use crate::db::*;
use crate::task::*;

lazy_static! {
    pub static ref CHANNEL_CONVERT : Channel<(TaskForConvert,RawTask)> = Channel::new();
}

pub fn start_receive_threads() -> Vec<JoinHandle<()>> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    threads.push(start_thread(&CHANNEL_CONVERT.receiver, channel_convert));
    info!("--------------------nature threads initialized---------------------");
    threads
}


fn start_thread<T, F>(receiver: &'static Mutex<Receiver<T>>, f: F) -> JoinHandle<()>
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