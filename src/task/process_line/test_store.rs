use super::*;
use util::*;

#[test]
fn born_verified_failed() {
    let instance = Instance::default();
    let rtn = ProcessLine::store(instance, Root::Business);
    match rtn {
        Err(x) => println!("{:?}", x),
        Ok(x) => {
            println!("{:?}", x);
            panic!("should got error!");
        }
    }
}

/// verified ok
#[test]
fn born_carrier_error() {
    //set mode to error
    change_mode(&CARRIER_DAO_MODE, CarrierDaoMode::Err);

    let mut instance = Instance::default();
    instance.data.thing.key = "ok".to_string();
    let rtn = ProcessLine::store(instance, Root::Business);
    match rtn {
        Err(x) => println!("{:?}", x),
        Ok(x) => {
            println!("{:?}", x);
            panic!("should got error!");
        }
    }
}

#[test]
fn store_task_error() {
    change_mode(&CARRIER_DAO_MODE, CarrierDaoMode::Ok);

    change_mode(&STORE_TASK_MODE, StoreTaskMode::Err);

    let mut instance = Instance::default();
    instance.data.thing.key = "ok".to_string();
    let rtn = ProcessLine::store(instance, Root::Business);
    match rtn {
        Err(x) => println!("{:?}", x),
        Ok(x) => {
            println!("{:?}", x);
            panic!("should got error!");
        }
    }
}

#[test]
fn received_instance() {
    pub fn start_route(receiver: &'static Mutex<Receiver<Instance>>) {
        thread::spawn(move || {
            println!("Create Receiver Thread");
            let receiver = receiver.lock().unwrap();
            let mut iter = receiver.iter();
            while let Some(next) = iter.next()
                {
                    println!("Break receive fro {:?}", next);
                    panic!();
                }
        });
    }

    start_route(&CHANNEL_ROUTE.receiver);

    change_mode(&CARRIER_DAO_MODE, CarrierDaoMode::Ok);

    change_mode(&STORE_TASK_MODE, StoreTaskMode::Ok);

    let mut instance = Instance::default();
    instance.data.thing.key = "ok".to_string();
    let rtn = ProcessLine::store(instance, Root::Business);
    match rtn {
        Err(x) => {
            println!("my err : {:?}", x);
            panic!("should not got error!");
        }
        Ok(x) => {
            println!("{:?}", x);
        }
    }
}