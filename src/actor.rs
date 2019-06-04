use actix::{Actor, Addr};

use nature_common::{TaskForParallel, TaskForSerial};
use nature_db::RawTask;

pub use self::convert::*;
pub use self::store::*;

pub struct MsgForParallel(TaskForParallel, RawTask);

pub struct MsgForSerial(TaskForSerial, RawTask);

mod store;
mod convert;

lazy_static! {
    pub static ref ACT_STORE: Addr<StoreActor> = StoreActor{}.start();
    pub static ref ACT_STORED: Addr<StoredActor> = StoredActor{}.start();
    pub static ref ACT_CONVERT: Addr<ConvertActor> = ConvertActor{}.start();
//    pub static ref ACT_STORE: Addr<StoreActor> = SyncArbiter::start(*THREAD_NUM_FOR_STORE_ACTOR, || StoreActor {});
//    pub static ref ACT_STORED: Addr<StoredActor> = SyncArbiter::start(*THREAD_NUM_FOR_STORED_ACTOR, || StoredActor {});
//    pub static ref ACT_CONVERT: Addr<ConvertActor> = SyncArbiter::start(*THREAD_NUM_FOR_CONVERT_ACTOR, || ConvertActor {});
}

pub fn init_actors() {
    // force to init
    let _ = ACT_STORE.clone();
    let _ = ACT_STORED.clone();
    let _ = ACT_CONVERT.clone();
}
