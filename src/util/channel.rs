use std::sync::mpsc::*;
use std::sync::Mutex;

pub struct Channel<T> {
    pub sender: Mutex<Sender<T>>,
    pub receiver: Mutex<Receiver<T>>,
}

impl<T> Channel<T> {
    pub fn new() -> Channel<T> {
        let (sx, rx) = channel::<T>();
        Channel {
            sender: Mutex::new(sx),
            receiver: Mutex::new(rx),
        }
    }
}