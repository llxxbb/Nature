use actix::{Actor, Addr};

use nature_common::{TaskForParallel, TaskForSerial};
use nature_db::RawTask;

use crate::actor::store_actor::StoreActor;
use crate::task::TaskForConvert;

pub struct MsgForConvert(TaskForConvert, RawTask);

pub struct MsgForParallel(TaskForParallel, RawTask);

pub struct MsgForSerial(TaskForSerial, RawTask);

pub mod store_actor;

pub static mut ACT_STORE: Option<Addr<StoreActor>> = None;

pub fn create_actors() {
    unsafe {
        ACT_STORE = Some(StoreActor {}.start());
    }
}
