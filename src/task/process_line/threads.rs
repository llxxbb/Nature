use std::sync::mpsc::Sender;
use super::*;

pub fn send_carrier<T>(sender: Sender<Carrier<T>>, carrier: Carrier<T>)
    where T: 'static + Sized + Serialize + Sync + Send {
    thread::spawn(move || {
        sender.send(carrier).unwrap();
    });
}


pub fn start_receive_threads() {
    start_thread(&CHANNEL_ROUTE.receiver, ProcessLine::route);
    start_thread(&CHANNEL_DISPATCH.receiver, ProcessLine::dispatch);
    start_thread(&CHANNEL_CONVERT.receiver, ProcessLine::convert);
    start_thread(&CHANNEL_STORE.receiver, ProcessLine::store_for_receive);
    start_thread(&CHANNEL_PARALLEL.receiver, ProcessLine::parallel);
    start_thread(&CHANNEL_SERIAL.receiver, ProcessLine::serial);
}

fn start_thread<T, F>(receiver: &'static Mutex<Receiver<Carrier<T>>>, f: F)
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
