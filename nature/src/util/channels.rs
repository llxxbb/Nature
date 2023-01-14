use std::ops::Deref;
use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use actix_web::web::Data;

use crate::db::RawTask;
use crate::nature_lib::dispatcher::channel_convert;
use crate::nature_lib::task::TaskForConvert;
use crate::util::web_context::WebContext;

pub fn start_receive_threads(context: Data<WebContext>) -> Vec<JoinHandle<()>> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    threads.push(start_thread(context, channel_convert));
    info!("--------------------nature threads initialized---------------------");
    threads
}


fn start_thread<F>(ctx: Data<WebContext>, f: F) -> JoinHandle<()>
    where
        F: Fn((TaskForConvert, RawTask, Data<WebContext>)) + Send + 'static
{
    thread::spawn(move || {
        let guard = ctx.chanel.receiver.lock().unwrap();
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