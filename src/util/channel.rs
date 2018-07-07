extern crate multiqueue;

use self::multiqueue::*;
use std::sync::Mutex;

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