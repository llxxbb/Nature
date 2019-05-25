use actix::prelude::*;
use crate::task::{TaskForStore, InnerController};
use nature_db::RawTask;

pub struct MsgForStore(TaskForStore, RawTask);

impl Message for MsgForStore{
    type Result = ();
}

pub struct StoreActor;

impl Actor for StoreActor {
    type Context = Context<Self>;
}

impl Handler<MsgForStore> for StoreActor{
    type Result = ();

    fn handle(&mut self, msg: MsgForStore, _ctx: &mut Self::Context) -> Self::Result {
        let _ = InnerController::save_instance(msg.0, msg.1);
    }
}