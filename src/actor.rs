use std::thread;

use actix::{Actor, Addr, System};

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
}

pub fn init_actors() {
    thread::spawn(|| {
        let sys = System::new("other_actors");
        // force to init
        let _ = ACT_STORE.clone();
        let _ = ACT_STORED.clone();
        let _ = ACT_CONVERT.clone();
        sys.run();
    });
}
