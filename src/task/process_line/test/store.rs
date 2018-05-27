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

#[test]
fn submit_store_error() {
    let _lock = lock_and_set_mock_value(&DELIVERY_LOCK, &DELIVERY_VALUE, Value::Ok);
    let _lock_instance = lock_and_set_mock_value(&INSTANCE_LOCK, &INSTANCE_RESULT, Err(NatureError::VerifyError("instance mock verify error".to_string())));
    let instance = Instance::default();
    match Store::submit_single(instance) {
        Err(NatureError::VerifyError(sss)) => assert_eq!("instance mock verify error", sss),
        _ => panic!("should match this arm!"),
    }
}

#[test]
fn do_store_insert_env_error() {
    let _lock_instance = lock_and_set_mock_value(&INSTANCE_LOCK, &INSTANCE_RESULT, Ok(UuidBytes::default()));
    let _lock_instance_table = lock_and_set_mock_value(&TABLE_INSTANCE_LOCK, &TABLE_INSTANCE_INSERT_VALUE, Err(NatureError::DaoEnvironmentError("instance dao mock insert error".to_string())));
    match Store::do_store(Carrier::new(StoreInfo::default()).unwrap(), Root::Business) {
        Err(NatureError::DaoEnvironmentError(sss)) => assert_eq!("instance dao mock insert error", sss),
        _ => panic!("should match this arm!"),
    }
}

#[test]
fn do_store_duplicated_handler_error() {
    let _lock_instance = lock_and_set_mock_value(&INSTANCE_LOCK, &INSTANCE_RESULT, Ok(UuidBytes::default()));
    let _lock_instance_table = lock_and_set_mock_value(&TABLE_INSTANCE_LOCK, &TABLE_INSTANCE_INSERT_VALUE, Err(NatureError::DaoDuplicated));
    let _lock_define_cache = lock_and_set_mock_value(&THING_DEFINE_LOCK, &THING_DEFINE_CACHE_VALUE, Err(NatureError::VerifyError("ThingDefineCache mock : not defined".to_string())));
    match Store::do_store(Carrier::new(StoreInfo::default()).unwrap(), Root::Business) {
        Err(NatureError::VerifyError(err)) => {
            assert_eq!("ThingDefineCache mock : not defined".to_string(), err)
        }
        _ => panic!("should not match this arm!"),
    }
}

#[test]
fn do_store_duplicated_not_status_finish_carrier_error() {
    let _lock_instance = lock_and_set_mock_value(&INSTANCE_LOCK, &INSTANCE_RESULT, Ok(UuidBytes::default()));
    let _lock_instance_table = lock_and_set_mock_value(&TABLE_INSTANCE_LOCK, &TABLE_INSTANCE_INSERT_VALUE, Err(NatureError::DaoDuplicated));
    let _lock_define_cache = lock_and_set_mock_value(&THING_DEFINE_LOCK, &THING_DEFINE_CACHE_VALUE, Ok(ThingDefine::default()));
    set_mock_value(&DELIVERY_FINISH_CARRIER_VALUE, Value::Err);
    match Store::do_store(Carrier::new(StoreInfo::default()).unwrap(), Root::Business) {
        Err(NatureError::DaoEnvironmentError(err)) => {
            assert_eq!("delivery mock finish_carrier".to_string(), err)
        }
        _ => panic!("should not match this arm!"),
    }
}

#[test]
fn do_store_duplicated_not_status_ok() {
    let _lock_instance = lock_and_set_mock_value(&INSTANCE_LOCK, &INSTANCE_RESULT, Ok(UuidBytes::default()));
    let _lock_instance_table = lock_and_set_mock_value(&TABLE_INSTANCE_LOCK, &TABLE_INSTANCE_INSERT_VALUE, Err(NatureError::DaoDuplicated));
    let _lock_define_cache = lock_and_set_mock_value(&THING_DEFINE_LOCK, &THING_DEFINE_CACHE_VALUE, Ok(ThingDefine::default()));
    set_mock_value(&DELIVERY_FINISH_CARRIER_VALUE, Value::Ok);
    match Store::do_store(Carrier::new(StoreInfo::default()).unwrap(), Root::Business) {
        Ok(x) => {
            assert_eq!(UuidBytes::default(), x)
        }
        _ => panic!("should not match this arm!"),
    }
}

#[test]
fn do_store_duplicated_status() {
    let _lock_instance = lock_and_set_mock_value(&INSTANCE_LOCK, &INSTANCE_RESULT, Ok(UuidBytes::default()));
    let _lock_instance_table = lock_and_set_mock_value(&TABLE_INSTANCE_LOCK, &TABLE_INSTANCE_INSERT_VALUE, Err(NatureError::DaoDuplicated));
    let mut define = ThingDefine::default();
    define.states = Some("A,B,C".to_string());
    let _lock_define_cache = lock_and_set_mock_value(&THING_DEFINE_LOCK, &THING_DEFINE_CACHE_VALUE, Ok(define));
    match Store::do_store(Carrier::new(StoreInfo::default()).unwrap(), Root::Business) {
        Err(NatureError::InstanceStatusVersionConflict) => (),
        _ => panic!("should not match this arm!"),
    }
}

#[test]
fn do_store_ok() {
    use std::ops::Deref;
    let _lock_instance = lock_and_set_mock_value(&INSTANCE_LOCK, &INSTANCE_RESULT, Ok(UuidBytes::default()));
    let _lock_instance_table = lock_and_set_mock_value(&TABLE_INSTANCE_LOCK, &TABLE_INSTANCE_INSERT_VALUE, Ok(1));
    let _lock_received = lock_and_set_mock_value(&RECEIVE_VALUE_MODIFY_LOCK, &RECEIVED_VALUES, Vec::new());

    start_thread(&CHANNEL_ROUTE.receiver, do_route);
    fn do_route(received: Carrier<StoreInfo>) {
        println!("------- mock do_route received {:?}", received);
        let mut val = RECEIVED_VALUES.lock().unwrap();
        val.push(Box::new(received));
    }

    match Store::do_store(Carrier::new(StoreInfo::default()).unwrap(), Root::Business) {
        Ok(x) => {
            assert_eq!(UuidBytes::default(), x)
        }
        _ => panic!("should not match this arm!"),
    }

    thread::sleep(time::Duration::from_millis(1));
    let received = RECEIVED_VALUES.lock().unwrap();
    let x: &Vec<Box<Any + Send + Sync>> = received.deref();
    assert_eq!(1, x.len());
}
