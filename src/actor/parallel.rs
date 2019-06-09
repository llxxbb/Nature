use actix::{Actor, Context, Handler};

use nature_common::TaskForParallel;

use crate::actor::MsgForTask;
use crate::task::InnerController;

pub struct ParallelActor;

impl Actor for ParallelActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("ParallelActor is started");
    }
}

impl Handler<MsgForTask<TaskForParallel>> for ParallelActor {
    type Result = ();

    fn handle(&mut self, msg: MsgForTask<TaskForParallel>, _ctx: &mut Self::Context) -> Self::Result {
        let _ = InnerController::channel_parallel(msg);
    }
}
