use std::sync::mpsc::*;
use std::sync::Mutex;

pub struct Processor<T> {
    pub sender: Mutex<Sender<T>>,
    pub receiver: Mutex<Receiver<T>>,
}

impl<T> Processor<T> {
    pub fn new() -> Processor<T> {
        let (sx, rx) = channel::<T>();
        Processor {
            sender: Mutex::new(sx),
            receiver: Mutex::new(rx),
        }
    }
}