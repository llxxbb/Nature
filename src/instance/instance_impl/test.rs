use super::*;

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
    let mut guard = MODE.lock().unwrap();
    *guard = Mode::Err;
    drop(guard);

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
