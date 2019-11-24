use std::thread;

use actix::{Actor, Addr, Message, System};

use nature_db::RawTask;

pub use self::convert::*;
pub use self::parallel::*;
pub use self::serial::*;
pub use self::store::*;

mod store;
mod convert;
mod parallel;
mod serial;

lazy_static! {
    pub static ref ACT_STORE: Addr<StoreActor> = StoreActor{}.start();
    pub static ref ACT_STORED: Addr<StoredActor> = StoredActor{}.start();
    pub static ref ACT_CONVERT: Addr<ConvertActor> = ConvertActor{}.start();
    pub static ref ACT_PARALLEL: Addr<ParallelActor> = ParallelActor{}.start();
    pub static ref ACT_SERIAL: Addr<SerialActor> = SerialActor{}.start();
}

pub fn init_actors() {
    thread::spawn(|| {
        let sys = System::new("other_actors");
        // force to init
        lazy_static::initialize(&ACT_STORE);
        lazy_static::initialize(&ACT_STORED);
        lazy_static::initialize(&ACT_CONVERT);
        lazy_static::initialize(&ACT_PARALLEL);
        lazy_static::initialize(&ACT_SERIAL);
        let _ = sys.run();
    });
}

pub struct MsgForTask<T>(pub T, pub RawTask);

impl<T> Message for MsgForTask<T> {
    type Result = ();
}
