use actix::prelude::*;

use nature_db::RawTask;

use crate::task::{InnerController, TaskForConvert};

pub struct MsgForConvert(pub TaskForConvert, pub RawTask);

impl Message for MsgForConvert {
    type Result = ();
}

pub struct ConvertActor;

impl Actor for ConvertActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("ConvertActor is started");
    }
}

impl Handler<MsgForConvert> for ConvertActor {
    type Result = ();

    fn handle(&mut self, msg: MsgForConvert, _ctx: &mut Self::Context) -> Self::Result {
        let _ = InnerController::channel_convert(msg.0, msg.1);
    }
}
