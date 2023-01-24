use actix_web::web::Data;
use async_channel::Sender;

use crate::db::RawTask;
use crate::nature_lib::task::TaskForConvert;

pub struct WebContext {
    pub sender: Sender<(TaskForConvert, RawTask, Data<WebContext>)>,
}
