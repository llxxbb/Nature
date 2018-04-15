use data::instance::*;
use global::*;
pub use self::store_task::*;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;

pub trait Task {
    fn take_it_over(&self) -> Result<()>;
}

pub fn start_task_route(receiver: &'static Mutex<Receiver<Instance>>) {
    thread::spawn(move || {
        let receiver = receiver.lock().unwrap();
        let mut iter = receiver.iter();
        while let Some(next) = iter.next()
            {
                println!("{:?}", next);
                // TODO
            }
    });
}


pub mod store_task;