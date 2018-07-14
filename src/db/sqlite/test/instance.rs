use db::trait_define::*;
use std::collections::HashMap;
use std::collections::HashSet;
use super::*;

#[test]
fn instance_common_test() {
    println!("----------------- instance_common_test --------------------");
    // prepare data to insert
    let instance = Instance {
        id: 0,
        data: InstanceNoID {
            thing: Thing {
                key: "/instance/common".to_string(),
                version: 100,
                thing_type: ThingType::Business,
            },
            event_time: 0,
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
    if let Ok(true) = InstanceDaoImpl::is_exists(&instance) {
        let _ = InstanceDaoImpl::delete(&instance);
    }
    // insert one
    assert_eq!(Ok(1), InstanceDaoImpl::insert(&instance));
    // exists
    assert_eq!(true, InstanceDaoImpl::is_exists(&instance).unwrap());
    // delete it
    assert_eq!(1, InstanceDaoImpl::delete(&instance).unwrap());
}

#[test]
fn get_last_status() {
    println!("----------------- get_last_status --------------------");
    // prepare data to insert
    let mut instance = Instance {
        id: 0,
        data: InstanceNoID {
            thing: Thing {
                key: "/instance/getLast".to_string(),
                version: 100,
                thing_type: ThingType::Business,
            },
            event_time: 0,
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
    if let Ok(true) = InstanceDaoImpl::is_exists(&instance) {
        let _ = InstanceDaoImpl::delete(&instance);
    }
    instance.data.status_version = 111;
    if let Ok(true) = InstanceDaoImpl::is_exists(&instance) {
        let _ = InstanceDaoImpl::delete(&instance);
    }
    // insert one
    instance.data.status_version = 123;
    assert_eq!(Ok(1), InstanceDaoImpl::insert(&instance));
    // insert two
    instance.data.status_version = 111;
    assert_eq!(Ok(1), InstanceDaoImpl::insert(&instance));
    // get last
    if let Ok(Some(x)) = InstanceDaoImpl::get_last_status_by_id(&instance.id) {
        assert_eq!(123, x.status_version);
    } else {
        panic!("shouldn't get error");
    }
    // delete after test
    let _ = InstanceDaoImpl::delete(&instance);
    instance.data.status_version = 123;
    let _ = InstanceDaoImpl::delete(&instance);
}