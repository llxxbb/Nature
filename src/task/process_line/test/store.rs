use data::*;
use global::*;
use super::super::store::Store;
use super::super::delivery::*;

#[test]
fn submit_and_delivery_error() {
    set_delivery_value(Value::Err);
    let instance = Instance::default();
    match Store::submit_single(instance) {
        Err(NatureError::SystemError(sss)) => assert_eq!("delivery mock error", sss),
        _ => panic!("should match this arm!"),
    }
}

fn submit_store_error(){
    // TODO
    set_delivery_value(Value::Ok);
    //
    let instance = Instance::default();
    match Store::submit_single(instance) {
        Err(NatureError::SystemError(sss)) => assert_eq!("delivery mock error", sss),
        _ => panic!("should match this arm!"),
    }}

fn set_delivery_value(val: Value) {
    let _ = DELIVERY_LOCK.lock().unwrap();
    let mut value = DELIVERY_VALUE.lock().unwrap();
    *value = val;
    drop(value);
}

