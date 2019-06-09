use actix::prelude::*;

use nature_db::RawTask;

use crate::task::{InnerController, TaskForStore};

pub struct MsgForStore(pub TaskForStore, pub RawTask);

impl Message for MsgForStore {
    type Result = ();
}

pub struct StoreActor;

impl Actor for StoreActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("StoreActor is started");
    }
}

impl Handler<MsgForStore> for StoreActor {
    type Result = ();

    fn handle(&mut self, msg: MsgForStore, _ctx: &mut Self::Context) -> Self::Result {
        let _ = InnerController::save_instance(msg.0, msg.1);
    }
}


pub struct StoredActor;

impl Actor for StoredActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("StoredActor is started");
    }
}

impl Handler<MsgForStore> for StoredActor {
    type Result = ();

    fn handle(&mut self, msg: MsgForStore, _ctx: &mut Self::Context) -> Self::Result {
        InnerController::channel_stored(msg.0, msg.1)
    }
}