use data::Carrier;
use std::ops::Deref;
use super::*;
use task::DeliveryTrait;


lazy_static! {
    pub static ref TASK_DELIVERY_LOCK: Mutex<u8> = Mutex::new(0);
    pub static ref TASK_DELIVERY_VALUE: Mutex<Value> = Mutex::new(Value::Err);
}

#[derive(Clone)]
pub enum Value {
    Ok,
    Err,
}

#[derive(Default)]
pub struct MockDeliveryTrait;

impl DeliveryTrait for MockDeliveryTrait {
    fn create_carrier<T>(valuable: T) -> Result<Carrier<T>> where T: Sized + Serialize {
        print!("    MockDeliveryTrait : create_carrier");
        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
        match x {
            Value::Ok => Carrier::new(valuable),
            Value::Err => Err(NatureError::SystemError("create_carrier mock error".to_string()))
        }
    }

    fn create_and_finish_carrier<T, U>(valuable: T, _old: Carrier<U>) -> Result<Carrier<T>> where T: Sized + Serialize, U: Sized + Serialize {
        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
        match x {
            Value::Ok => Carrier::new(valuable),
            Value::Err => Err(NatureError::SystemError("create_and_finish_carrier mock error".to_string()))
        }
    }

    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, _old: Carrier<U>) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize, U: Sized + Serialize {
        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
        match x {
            Value::Ok => {
                let mut rtn: Vec<Carrier<T>> = Vec::new();
                for x in valuables {
                    rtn.push(Carrier::new(x)?);
                }
                Ok(rtn)
            }
            Value::Err => Err(NatureError::SystemError("create_batch_and_finish_carrier mock error".to_string()))
        }
    }

    fn finish_carrier(_id: &[u8; 16]) -> Result<()> {
        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
        match x {
            Value::Ok => Ok(()),
            Value::Err => Err(NatureError::DaoEnvironmentError("finish_carrier mock error".to_string()))
        }
    }

    fn move_to_err<T>(_err: NatureError, _carrier: Carrier<T>) where T: Sized + Serialize {
        ()
    }
}

