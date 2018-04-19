use data::*;
use global::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::process_line::*;
pub use self::store_task::*;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;

pub trait Task {
    fn take_it_over(&self) -> Result<()>;
}

pub fn start_route_thread(receiver: &'static Mutex<Receiver<Carrier<StoreTask>>>) {
    thread::spawn(move || {
        let receiver = receiver.lock().unwrap();
        let mut iter = receiver.iter();
        while let Some(next) = iter.next() {
            ProcessLine::route(next);
        }
    });
}


pub mod store_task;

pub mod dispatch_task;

pub mod process_line;

#[cfg(test)]
mod mock;