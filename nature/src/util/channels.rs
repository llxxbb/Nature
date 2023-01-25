use actix_web::web::Data;
use async_channel::Receiver;

use crate::db::RawTask;
use crate::nature_lib::dispatcher::channel_convert;
use crate::nature_lib::task::TaskForConvert;
use crate::util::web_context::WebContext;

pub async fn loop_receiver(receiver: Receiver<(TaskForConvert, RawTask, Data<WebContext>)>) {
    info!("start receive thread");
    tokio::spawn(async move {
        loop {
            match receiver.recv().await {
                Ok(r) => channel_convert(r).await,
                Err(e) => error!("{}", e)
            }
        }
    });
}
