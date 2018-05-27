use data::*;
use db::*;
use global::*;
use std::any::Any;
use std::thread;
use std::time;
use super::super::delivery::*;
use super::super::store::Store;
use super::super::threads::*;
use task::struct_define::StoreInfo;
use util::*;
use uuid::UuidBytes;

#[test]
fn submit_and_delivery_error() {
    let _lock = lock_and_set_mock_value(&DELIVERY_LOCK, &DELIVERY_VALUE, Value::Err);
    let instance = Instance::default();
    match Store::submit_single(instance) {
        Err(NatureError::SystemError(sss)) => assert_eq!("delivery mock error", sss),
        Err(err) => {
            println!("{:?}", err);
            panic!("should match this arm!");
        }
        Ok(x) => {
            println!("{:?}", x);
            panic!("should match this arm!")
        }
    }
}
