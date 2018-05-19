use std::collections::HashMap;
use std::collections::HashSet;
use super::*;

#[test]
fn instance_common_test() {
    println!("----------------- instance_common_test --------------------");
    // prepare data to insert
    let instance = Instance {
        id: UuidBytes::default(),
        data: InstanceNoID {
            thing: Thing {
                key: "/instance/common".to_string(),
                version: 100,
            },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: HashMap::new(),
            status: HashSet::new(),
            status_version: 123,
            from: None,
        },
    };
    // delete if it exists
    if let Ok(true) = TableInstance::is_exists(&instance) {
        let _ = TableInstance::delete(&instance);
    }
    // insert one
    assert_eq!(Ok(1), TableInstance::insert(&instance));
    // exists
    assert_eq!(true, TableInstance::is_exists(&instance).unwrap());
    // delete it
    assert_eq!(1, TableInstance::delete(&instance).unwrap());
}

#[test]
fn get_last_status() {
    println!("----------------- get_last_status --------------------");
    // prepare data to insert
    let mut instance = Instance {
        id: UuidBytes::default(),
        data: InstanceNoID {
            thing: Thing {
                key: "/instance/getLast".to_string(),
                version: 100,
            },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: HashMap::new(),
            status: HashSet::new(),
            status_version: 123,
            from: None,
        },
    };
    // delete old if exists
    if let Ok(true) = TableInstance::is_exists(&instance) {
        let _ = TableInstance::delete(&instance);
    }
    instance.data.status_version = 111;
    if let Ok(true) = TableInstance::is_exists(&instance) {
        let _ = TableInstance::delete(&instance);
    }
    // insert one
    instance.data.status_version = 123;
    assert_eq!(Ok(1), TableInstance::insert(&instance));
    // insert two
    instance.data.status_version = 111;
    assert_eq!(Ok(1), TableInstance::insert(&instance));
    // get last
    if let Ok(Some(x)) = TableInstance::get_last_status_by_id(&instance.id){
        assert_eq!(123, x.status_version);
    }else
    {
        panic!("shouldn't get error");
    }
    // delete after test
    let _ = TableInstance::delete(&instance);
    instance.data.status_version = 123;
    let _ = TableInstance::delete(&instance);
}