use nature_common::{TaskForParallel, TaskForSerial};
use nature_db::RawTask;

use crate::task::TaskForConvert;

pub struct MsgForConvert(TaskForConvert, RawTask);

pub struct MsgForParallel(TaskForParallel, RawTask);

pub struct MsgForSerial(TaskForSerial, RawTask);

pub mod store_actor;

