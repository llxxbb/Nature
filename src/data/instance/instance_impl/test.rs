use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use super::*;
use util::change_mode;

#[test]
fn born_verified_failed() {
    let instance = Instance::default();
    let rtn = InstanceImpl::born(instance);
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
    let rtn = InstanceImpl::born(instance);
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
    let rtn = InstanceImpl::born(instance);
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
    let rtn = InstanceImpl::born(instance);
    match rtn {
        Err(x) => {
            println!("my err : {:?}", x);
            panic!("should not got error!");
        },
        Ok(x) => {
            println!("{:?}", x);
        }
    }
}

#[test]
fn id_generate() {
    let mut instance = Instance {
        id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        data: InstanceNoID {
            thing: Thing { key: "hello".to_string(), version: 3 },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: String::new(),
        },
    };
    InstanceImpl::id_generate_if_not_set(&mut instance).unwrap();
    println!("{:?}", Uuid::from_bytes(&instance.id));
    assert_eq!(instance.id, [92, 134, 13, 161, 58, 84, 48, 67, 177, 110, 233, 201, 56, 64, 195, 240]);
}
