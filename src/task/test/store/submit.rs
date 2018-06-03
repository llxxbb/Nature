use global::*;
use task::*;
use util::*;

#[test]
fn delivery_error() {
    let _lock_delivery = lock_and_set_mock_value(&TASK_DELIVERY_LOCK, &TASK_DELIVERY_VALUE, Value::Err);
    let instance = Instance::default();
    match StoreTaskImpl::submit_single(
        &MockDeliveryTrait,
        &MockInstanceTrait,
        &MockTableInstance,
        &MockThingDefineCache,
        instance) {
        Err(NatureError::SystemError(sss)) => assert_eq!("create_carrier mock error", sss),
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
fn delivery_ok_but_store_error() {
    let _lock_delivery = lock_and_set_mock_value(&TASK_DELIVERY_LOCK, &TASK_DELIVERY_VALUE, Value::Ok);
    let _lock_instance = lock_and_set_mock_value(&DATA_INSTANCE_LOCK, &DATA_INSTANCE_RESULT, Err(NatureError::VerifyError("instance mock verify error".to_string())));
    let instance = Instance::default();
    match StoreTaskImpl::submit_single(
        &MockDeliveryTrait,
        &MockInstanceTrait,
        &MockTableInstance,
        &MockThingDefineCache,
        instance) {
        Err(NatureError::VerifyError(sss)) => assert_eq!("instance mock verify error", sss),
        _ => panic!("should match this arm!"),
    }
}
