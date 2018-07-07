//extern crate multiqueue;
//use self::multiqueue::MPMCSender;
//use service::*;
//use std::ops::Deref;
//use super::*;
//
//
//lazy_static! {
//    pub static ref TASK_DELIVERY_LOCK: Mutex<u8> = Mutex::new(0);
//    pub static ref TASK_DELIVERY_VALUE: Mutex<Value> = Mutex::new(Value::Err);
//
//    // counter
//    pub static ref TASK_DELIVERY_CREATE_COUNTER: Mutex<usize> = Mutex::new(0);
//    pub static ref TASK_DELIVERY_CREATE_AND_FINISH_COUNTER: Mutex<usize> = Mutex::new(0);
//    pub static ref TASK_DELIVERY_BATCH_AND_FINISH_COUNTER: Mutex<usize> = Mutex::new(0);
//    pub static ref TASK_DELIVERY_FINISH_COUNTER: Mutex<usize> = Mutex::new(0);
//    pub static ref TASK_DELIVERY_ERROR_COUNTER: Mutex<usize> = Mutex::new(0);
//}
//
//#[derive(Clone)]
//pub enum Value {
//    Ok,
//    Err,
//}
//
//#[derive(Default)]
//pub struct MockDeliveryTrait;
//
//impl DeliveryServiceTrait for MockDeliveryTrait {
//    fn create_carrier<T>(valuable: T, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize {
//        print!("    MockDeliveryTrait : create_carrier");
//        let mut cnt = TASK_DELIVERY_CREATE_COUNTER.lock().unwrap();
//        *cnt = cnt.deref() + 1;
//        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
//        match x {
//            Value::Ok => Carrier::new(valuable, thing, data_type),
//            Value::Err => Err(NatureError::SystemError("create_carrier mock error".to_string()))
//        }
//    }
//
//    fn create_and_finish_carrier<T, U>(valuable: T, _old: Carrier<U>, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize, U: Sized + Serialize {
//        let mut cnt = TASK_DELIVERY_CREATE_AND_FINISH_COUNTER.lock().unwrap();
//        *cnt = cnt.deref() + 1;
//        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
//        match x {
//            Value::Ok => Carrier::new(valuable, thing, data_type),
//            Value::Err => Err(NatureError::SystemError("create_and_finish_carrier mock error".to_string()))
//        }
//    }
//
//    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, _old: Carrier<U>, thing: String, data_type: u8) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize, U: Sized + Serialize {
//        let mut cnt = TASK_DELIVERY_BATCH_AND_FINISH_COUNTER.lock().unwrap();
//        *cnt = cnt.deref() + 1;
//        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
//        match x {
//            Value::Ok => {
//                let mut rtn: Vec<Carrier<T>> = Vec::new();
//                for x in valuables {
//                    rtn.push(Carrier::new(x, thing.clone(), data_type)?);
//                }
//                Ok(rtn)
//            }
//            Value::Err => Err(NatureError::SystemError("create_batch_and_finish_carrier mock error".to_string()))
//        }
//    }
//
//    fn finish_carrier(_id: &u128) -> Result<()> {
//        let mut cnt = TASK_DELIVERY_FINISH_COUNTER.lock().unwrap();
//        *cnt = cnt.deref() + 1;
//        let x: Value = TASK_DELIVERY_VALUE.lock().unwrap().deref().clone();
//        match x {
//            Value::Ok => Ok(()),
//            Value::Err => Err(NatureError::DaoEnvironmentError("finish_carrier mock error".to_string()))
//        }
//    }
//
//    fn move_to_err<T>(_err: NatureError, _carrier: Carrier<T>) where T: Sized + Serialize {
//        let mut cnt = TASK_DELIVERY_ERROR_COUNTER.lock().unwrap();
//        *cnt = cnt.deref() + 1;
//        ()
//    }
//
//    fn new_carrier<T>(task: T, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize {
//        unimplemented!()
//    }
//
//    fn send_carrier<T>(sender: &Mutex<MPMCSender<Carrier<T>>>, carrier: Carrier<T>) where T: 'static + Sized + Serialize + Sync + Send {
//        unimplemented!()
//    }
//}
//
