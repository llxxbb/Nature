use std::ops::Deref;
use std::sync::*;
use super::*;

lazy_static! {
    pub static ref DELIVERY_LOCK: Mutex<u8> = Mutex::new(0);
    pub static ref DELIVERY_VALUE: Mutex<Value> = Mutex::new(Value::Err);
    pub static ref DELIVERY_FINISH_CARRIER_VALUE: Mutex<Value> = Mutex::new(Value::Err);
}

#[derive(Clone)]
pub enum Value {
    Ok,
    Err,
}

pub struct Delivery;

impl DeliveryTrait for Delivery {
    fn create_carrier<T>(valuable: T) -> Result<Carrier<T>> where T: Sized + Serialize {
        let x: Value = DELIVERY_VALUE.lock().unwrap().deref().clone();
        match x {
            Value::Ok => Carrier::new(valuable),
            Value::Err => Err(NatureError::SystemError("delivery mock error".to_string()))
        }
    }

    fn create_and_finish_carrier<T, U>(_valuable: T, _old: Carrier<U>) -> Result<Carrier<T>> where T: Sized + Serialize, U: Sized + Serialize {
        unimplemented!()
    }

    fn create_batch_and_finish_carrier<T, U>(_valuables: Vec<T>, _old: Carrier<U>) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize, U: Sized + Serialize {
        unimplemented!()
    }

    fn finish_carrier(_id: &[u8; 16]) -> Result<()> {
        let x: Value = DELIVERY_FINISH_CARRIER_VALUE.lock().unwrap().deref().clone();
        match x {
            Value::Ok => Ok(()),
            Value::Err => Err(NatureError::DaoEnvironmentError("delivery mock finish_carrier".to_string()))
        }
    }

    fn move_to_err<T>(_err: NatureError, _carrier: Carrier<T>) where T: Sized + Serialize { () }
}