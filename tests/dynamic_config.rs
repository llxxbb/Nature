#[macro_use]
extern crate lazy_static;
extern crate nature;
extern crate nature_common;
extern crate nature_db;

use std::env;

use reqwest::Client;

use nature::task::IncomeController;
use nature_common::*;
use nature_db::*;

use crate::common::{CONN_STR, sleep, test_init};

pub mod common;

lazy_static! {
    pub static ref WEB_CLIENT: Client = Client::new();
}

#[test]
fn convert_is_empty() {
    env::set_var("DATABASE_URL", CONN_STR);
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/converter/is/empty", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![],
    };
    let rtn = IncomeController::self_route(instance);
    assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
}

#[test]
fn target_is_null() {
    test_init();
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/target/is/null", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: None,
            fun: Executor::for_local(""),
        }],
    };
    sleep(3000);
    let rtn = query(instance);
    assert_eq!(67155089214163907246089937900589001447, rtn);
    // check input
    let written = InstanceDaoImpl::get_by_id(67155089214163907246089937900589001447).unwrap().unwrap();
    assert_eq!("/D/dynamic/target/is/null", written.data.meta.get_full_key());
}


#[test]
fn write_one_target_to_db() {
    test_init();
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/write/one", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: Some("/dynamic/one_target".to_string()),
            fun: Executor::for_local(r#"nature_integrate_test_converter.dll:rtn_one"#),
        }],
    };
    sleep(5000);
    let rtn = query(instance);
    assert_eq!(230241203652394260574473500578835056831, rtn);

    // query target
    sleep(3000);
    let ins_db = InstanceDaoImpl::get_by_id(64608961992354323405453802188093613020).unwrap().unwrap();
    assert_eq!("/D/dynamic/one_target", ins_db.meta.get_full_key());
}

#[test]
fn write_two_target_to_db() {
    test_init();
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/write/two", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![
            DynamicConverter {
                to: Some("/dynamic/two_of_1".to_string()),
                fun: Executor::for_local(r#"nature_integrate_test_converter.dll:rtn_one"#),
            },
            DynamicConverter {
                to: Some("/dynamic/two_of_2".to_string()),
                fun: Executor::for_local(r#"nature_integrate_test_converter.dll:rtn_one"#),
            }],
    };
    sleep(5000);

    let rtn = query(instance);
    assert_eq!(226464870279356952826561520522393294145, rtn);

    // query target
    sleep(2000);
    let ins_db = InstanceDaoImpl::get_by_id(229131768420092721318239706157158451568).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_1", ins_db.meta.get_full_key());
    let ins_db = InstanceDaoImpl::get_by_id(217550842272210133848994195869177780408).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_2", ins_db.meta.get_full_key());
}


fn query(instance: SelfRouteInstance) -> u128 {
    let req = WEB_CLIENT.post("http://localhost:8080/self_route").json(&instance).build().unwrap();
    let mut rtn = WEB_CLIENT.execute(req).unwrap();
    let rtn = rtn.json::<Result<u128>>();
    let rtn = rtn.unwrap().unwrap();
    rtn
}


