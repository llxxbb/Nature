//use super::*;
//use util::*;
//
//#[test]
//fn store_verified_failed() {
//    println!("----------------- store_verified_failed --------------------");
//    let _ = THING_DEFINE_LOCK.lock().unwrap();
//
//    let mut value = THING_DEFINE_GET_VALUE.lock().unwrap();
//    *value = Err(NatureError::VerifyError("some err".to_string()));
//    drop(value);
//    let mut instance = Instance::default();
//    instance.data.thing.key = "/B/err".to_string();
//    let rtn = Teller::single_input(instance);
//    match rtn {
//        Err(x) => println!("{:?}", x),
//        Ok(x) => {
//            println!("{:?}", x);
//            panic!("should got error!");
//        }
//    }
//}
//
///// verified ok
//#[test]
//fn store_carrier_error() {
//    println!("----------------- store_carrier_error --------------------");
//    let _ = THING_DEFINE_LOCK.lock().unwrap();
//    change_mode(&INSTANCE_VERIFY_MODE, InstanceVerifyMode::Ok);
//    change_mode(&CARRIER_DAO_MODE, CarrierDaoMode::Err);
//
//    let mut instance = Instance::default();
//    instance.data.thing.key = "/B/ok".to_string();
//    let rtn = Teller::single_input(instance);
//    match rtn {
//        Err(x) => println!("{:?}", x),
//        Ok(x) => {
//            println!("{:?}", x);
//            panic!("should got error!");
//        }
//    }
//}
//
//#[test]
//fn store_persistence_error() {
//    println!("----------------- store_persistence_error --------------------");
//    let _ = THING_DEFINE_LOCK.lock().unwrap();
//    change_mode(&INSTANCE_VERIFY_MODE, InstanceVerifyMode::Ok);
//    change_mode(&CARRIER_DAO_MODE, CarrierDaoMode::Ok);
//    change_mode(&INSTANCE_DAO_MODE, InstanceDaoMode::Err);
//
//    let mut instance = Instance::default();
//    instance.data.thing.key = "/B/ok".to_string();
//    let rtn = Teller::single_input(instance);
//    match rtn {
//        Err(x) => println!("{:?}", x),
//        Ok(x) => {
//            println!("{:?}", x);
//            panic!("should got error!");
//        }
//    }
//}
//
//#[test]
//fn received_instance() {
//    println!("----------------- received_instance --------------------");
//    pub fn start_route(receiver: &'static Mutex<Receiver<Carrier<StoreInfo>>>) {
//        thread::spawn(move || {
//            println!("Create Receiver Thread");
//            let receiver = receiver.lock().unwrap();
//            let mut iter = receiver.iter();
//            while let Some(next) = iter.next() {
//                println!("Break receive for {:?}", next);
//                panic!();
//            }
//        });
//    }
//
//    start_route(&CHANNEL_ROUTE.receiver);
//
//    change_mode(&INSTANCE_VERIFY_MODE, InstanceVerifyMode::Ok);
//    change_mode(&CARRIER_DAO_MODE, CarrierDaoMode::Ok);
//    change_mode(&INSTANCE_DAO_MODE, InstanceDaoMode::Ok);
//
//    let mut instance = Instance::default();
//    instance.data.thing.key = "/B/ok".to_string();
//    let rtn = Teller::single_input(instance);
//    match rtn {
//        Err(x) => {
//            println!("my err : {:?}", x);
//            panic!("should not got error!");
//        }
//        Ok(x) => {
//            println!("{:?}", x);
//        }
//    }
//}
//
