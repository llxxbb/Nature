use actix_web::web::Data;
use async_channel::Receiver;
use tokio::task::JoinHandle;

use crate::db::RawTask;
use crate::nature_lib::dispatcher::channel_convert;
use crate::nature_lib::task::TaskForConvert;
use crate::util::web_context::WebContext;

pub fn start_receive_threads(receiver: Receiver<(TaskForConvert, RawTask, Data<WebContext>)>) -> Vec<JoinHandle<()>> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    threads.push(start_thread(receiver, channel_convert));
    info!("--------------------nature threads initialized---------------------");
    threads
}


fn start_thread<F>(rx: Receiver<(TaskForConvert, RawTask, Data<WebContext>)>, f: F) -> JoinHandle<()>
    where
        F: Fn((TaskForConvert, RawTask, Data<WebContext>)) + Send + Sync + 'static
{
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(r) => f(r),
                Err(e) => error!("{}", e)
            }
        }
    })
}
