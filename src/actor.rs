use actix::{Addr, SyncArbiter};

use nature_common::{TaskForParallel, TaskForSerial};
use nature_db::RawTask;

use crate::actor::store::*;
use crate::system::*;
use crate::task::TaskForConvert;

pub struct MsgForConvert(TaskForConvert, RawTask);

pub struct MsgForParallel(TaskForParallel, RawTask);

pub struct MsgForSerial(TaskForSerial, RawTask);

pub mod store;

lazy_static!{
    pub static ref ACT_STORE: Addr<StoreActor> = SyncArbiter::start(*THREAD_NUM_FOR_STORE_ACTOR, || StoreActor {});
    pub static ref ACT_STORED: Addr<StoredActor> = SyncArbiter::start(*THREAD_NUM_FOR_STORED_ACTOR, || StoredActor {});
}

