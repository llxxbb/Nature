use actix::{Actor, Context, Handler};

use nature_common::TaskForSerial;

use crate::actor::MsgForTask;
use crate::controller::channel_serial;

pub struct SerialActor;

impl Actor for SerialActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("SerialActor is started");
    }
}

impl Handler<MsgForTask<TaskForSerial>> for SerialActor {
    type Result = ();

    fn handle(&mut self, msg: MsgForTask<TaskForSerial>, _ctx: &mut Self::Context) -> Self::Result {
        let _ = channel_serial(msg);
    }
}
