use actix::prelude::*;

use crate::actor::MsgForTask;
use crate::controller::channel_convert;
use crate::task::TaskForConvert;

pub struct ConvertActor;

impl Actor for ConvertActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("ConvertActor is started");
    }
}

impl Handler<MsgForTask<TaskForConvert>> for ConvertActor {
    type Result = ();

    fn handle(&mut self, msg: MsgForTask<TaskForConvert>, _ctx: &mut Self::Context) -> Self::Result {
        let _ = channel_convert(msg.0, msg.1);
    }
}
