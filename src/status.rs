use actix::{Addr, SyncArbiter};

use crate::actor::store::*;
use crate::system::*;

pub struct State {
    pub act_store: Addr<StoreActor>,
    pub act_stored: Addr<StoredActor>,
}

impl State {
    pub fn new() -> Self {
        State {
            act_store: SyncArbiter::start(*THREAD_NUM_FOR_STORE_ACTOR, || StoreActor {}),
            act_stored: SyncArbiter::start(*THREAD_NUM_FOR_STORED_ACTOR, || StoredActor {}),
        }
    }
}