use actix::{Addr, SyncArbiter};

use crate::actor::store_actor::StoreActor;
use crate::system::THREAD_NUM_FOR_STORE_ACTOR;

pub struct State {
    pub act_store: Addr<StoreActor>,
}

impl State {
    pub fn new() -> Self {
        State {
            act_store: SyncArbiter::start(*THREAD_NUM_FOR_STORE_ACTOR, || StoreActor {}),
        }
    }
}