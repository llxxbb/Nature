use actix_web::web::Data;

use crate::db::RawTask;
use crate::nature_lib::task::TaskForConvert;
use crate::util::channels::Channel;

pub struct WebContext {
    pub chanel: Channel<(TaskForConvert, RawTask, Data<WebContext>)>,
}

impl WebContext {
    pub fn new() -> Self {
        WebContext {
            chanel: Channel::new()
        }
    }
}