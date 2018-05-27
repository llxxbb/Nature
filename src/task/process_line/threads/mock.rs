use std::any::Any;
use std::sync::Mutex;

lazy_static! {
    pub static ref RECEIVE_VALUE_MODIFY_LOCK : Mutex<bool> = Mutex::new(true);
    pub static ref RECEIVED_VALUES : Mutex<Vec<Box<Any + Send + Sync>>> = Mutex::new(Vec::new());
}




