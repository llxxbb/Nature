#[macro_use]
extern crate lazy_static;
extern crate nature;
extern crate nature_common;
extern crate nature_db;

use std::collections::{HashMap, HashSet};
use std::env;

use futures::executor::block_on;
use reqwest::blocking::Client;
use tokio::runtime::Runtime;

use nature::controller::IncomeController;
use nature_common::*;
use nature_db::InstanceDaoImpl;

use crate::common::{CONN_STR, sleep, test_init};

pub mod common;

lazy_static! {
    pub static ref WEB_CLIENT: Client = Client::new();
}

#[test]
fn convert_is_empty() {
    env::set_var("DATABASE_URL", CONN_STR);
    // prepare input para
    let instance = new_with_type("/dynamic/executor/is/empty", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![],
    };
    let rtn = block_on(IncomeController::self_route(instance));
    assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("executor must not empty for dynamic convert!".to_string()));
}


fn query(instance: SelfRouteInstance) -> ID {
    let req = WEB_CLIENT.post("http://localhost:8080/self_route").json(&instance).send().unwrap();
    let rtn = req.json::<Result<ID>>();
    let rtn = rtn.unwrap().unwrap();
    rtn
}

pub fn new_with_type(key: &str, meta: MetaType) -> Result<Instance> {
    if key.is_empty() {
        return Err(NatureError::VerifyError("key can not be empty".to_string()));
    }
    let key = Meta::key_standardize(key)?;
    Ok(Instance {
        id: 0,
        data: BizObject {
            meta: format!("{}:{}:1", meta.get_prefix(), key),
            content: "".to_string(),
            context: HashMap::new(),
            sys_context: HashMap::new(),
            states: HashSet::new(),
            state_version: 0,
            from: None,
            para: String::new(),
        },
        create_time: 0,
    })
}

// ----------- the following test need a db connection and renew for every time-------------------------

// #[test]
#[allow(dead_code)]
fn target_is_null() {
    test_init();
    // prepare input para
    let instance = new_with_type("/dynamic/target/is/null", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: None,
            fun: Executor::for_local(""),
            use_upstream_id: false,
            delay: 0,
        }],
    };
    sleep(4000);
    let rtn = query(instance);
    assert_eq!(rtn, 12);
    // check input
    let written = Runtime::new().unwrap().block_on(InstanceDaoImpl::get_by_id(KeyCondition {
        id: format!("{:x}", 3 as ID),
        meta: "/D/dynamic/target/is/null:1".to_string(),
        key_gt: "".to_string(),
        key_ge: "".to_string(),
        key_lt: "".to_string(),
        key_le: "".to_string(),
        para: "".to_string(),
        limit: 1,
        state_version: 0,
        time_ge: None,
        time_lt: None,
    })).unwrap().unwrap();
    assert_eq!("/D/dynamic/target/is/null:1", written.data.meta);
}

// #[test]
#[allow(dead_code)]
fn write_one_target_to_db() {
    test_init();
    // prepare input para
    let instance = new_with_type("/dynamic/write/one", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: Some("/dynamic/one_target".to_string()),
            fun: Executor::for_local(r#"nature_integrate_test_executor:rtn_one"#),
            use_upstream_id: false,
            delay: 0,
        }],
    };
    sleep(5000);
    let rtn = query(instance);
    assert_eq!(rtn, 12);

    // query target
    sleep(3000);
    let ins_db = Runtime::new().unwrap().block_on(InstanceDaoImpl::get_by_id(KeyCondition {
        id: format!("{:x}", 31 as ID),
        meta: "/D/dynamic/one_target:1".to_string(),
        key_gt: "".to_string(),
        key_ge: "".to_string(),
        key_lt: "".to_string(),
        key_le: "".to_string(),
        para: "".to_string(),
        limit: 1,
        state_version: 0,
        time_ge: None,
        time_lt: None,
    })).unwrap().unwrap();
    assert_eq!("/D/dynamic/one_target:1", ins_db.meta);
}

// #[test]
#[allow(dead_code)]
fn write_two_target_to_db() {
    test_init();
    // prepare input para
    let instance = new_with_type("/dynamic/write/two", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![
            DynamicConverter {
                to: Some("/dynamic/two_of_1".to_string()),
                fun: Executor::for_local(r#"nature_integrate_test_executor:rtn_one"#),
                use_upstream_id: false,
                delay: 0,
            },
            DynamicConverter {
                to: Some("/dynamic/two_of_2".to_string()),
                fun: Executor::for_local(r#"nature_integrate_test_executor:rtn_one"#),
                use_upstream_id: false,
                delay: 0,
            }],
    };
    sleep(5000);

    let rtn = query(instance);
    assert_eq!(rtn, 123);

    // query target
    sleep(2500);
    let ins_db = Runtime::new().unwrap().block_on(InstanceDaoImpl::get_by_id(KeyCondition {
        id: format!("{:x}", 12 as ID),
        meta: "/D/dynamic/two_of_1:1".to_string(),
        key_gt: "".to_string(),
        key_ge: "".to_string(),
        key_lt: "".to_string(),
        key_le: "".to_string(),
        para: "".to_string(),
        limit: 1,
        state_version: 0,
        time_ge: None,
        time_lt: None,
    })).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_1:1", ins_db.meta);
    let ins_db = Runtime::new().unwrap().block_on(InstanceDaoImpl::get_by_id(KeyCondition {
        id: format!("{:x}", 12 as ID),
        meta: "/D/dynamic/two_of_2:1".to_string(),
        key_gt: "".to_string(),
        key_ge: "".to_string(),
        key_lt: "".to_string(),
        key_le: "".to_string(),
        para: "".to_string(),
        limit: 1,
        state_version: 0,
        time_ge: None,
        time_lt: None,
    })).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_2:1", ins_db.meta);
}