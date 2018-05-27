#[cfg(not(test))]
pub use self::delivery_impl::*;
#[cfg(test)]
pub use self::mock::*;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread;
use super::*;

mod delivery_impl;

pub trait DeliveryTrait {
    fn create_carrier<T>(valuable: T) -> Result<Carrier<T>> where T: Sized + Serialize;
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>) -> Result<Carrier<T>> where T: Sized + Serialize, U: Sized + Serialize;
    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize, U: Sized + Serialize;
    fn finish_carrier(id: &UuidBytes) -> Result<()>;
    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize;
}

pub fn send_carrier<T>(sender: Sender<Carrier<T>>, carrier: Carrier<T>)
    where T: 'static + Sized + Serialize + Sync + Send {
    thread::spawn(move || {
        sender.send(carrier).unwrap();
    });
}

pub fn start_thread<T, F>(receiver: &'static Mutex<Receiver<Carrier<T>>>, f: F)
    where
        T: Serialize + Send,
        F: 'static + Fn(Carrier<T>) + Send
{
    thread::spawn(move || {
        let receiver = receiver.lock().unwrap();
        let mut iter = receiver.iter();
        while let Some(next) = iter.next() {
            f(next);
        }
    });
}


#[cfg(test)]
mod mock;
