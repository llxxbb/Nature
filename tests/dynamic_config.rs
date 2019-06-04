extern crate nature;
extern crate nature_common;
extern crate nature_db;

use std::{env, thread, time};

use actix_web::http::Method;
use actix_web::HttpMessage;
use actix_web::test::TestServer;
use futures::future::Future;

use nature::task::IncomeController;
use nature_common::*;
use nature_db::*;

use crate::common::{CONN_STR, test_init};

pub mod common;

#[test]
fn convert_is_empty() {
    env::set_var("DATABASE_URL", CONN_STR);
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/converter/is/empty", ThingType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![],
    };
    let rtn = IncomeController::self_route(instance);
    assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
}

#[test]
fn target_is_null() {
    let mut svc = test_init();
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/target/is/null", ThingType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: None,
            fun: Executor::for_local(""),
        }],
    };
    let rtn = query(&mut svc, instance);
    assert_eq!(67155089214163907246089937900589001447, rtn);
    // check input
    let written = InstanceDaoImpl::get_by_id(67155089214163907246089937900589001447).unwrap().unwrap();
    assert_eq!("/D/dynamic/target/is/null", written.data.thing.get_full_key());
}

fn query(svc: &mut TestServer, instance: SelfRouteInstance) -> u128 {
    let req = svc.client(Method::POST, "self_route")
        .json(instance).unwrap();
    svc.execute(req.send()).unwrap().json::<Result<u128>>().wait().unwrap().unwrap()
}

#[test]
fn write_one_target_to_db() {
    let mut svc = test_init();
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/write/one", ThingType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: Some("/dynamic/one_target".to_string()),
            fun: Executor::for_local(r#"../nature_integrate_test_converter.dll:rtn_one"#),
        }],
    };
    let rtn = query(&mut svc, instance);
    assert_eq!(230241203652394260574473500578835056831, rtn);

    // query target
    thread::sleep(time::Duration::from_millis(10000));
    let ins_db = InstanceDaoImpl::get_by_id(64608961992354323405453802188093613020).unwrap().unwrap();
    assert_eq!("/D/dynamic/one_target", ins_db.thing.get_full_key());
}

#[test]
fn write_two_target_to_db() {
    let mut svc = test_init();
    // prepare input para
    let instance = Instance::new_with_type("/dynamic/write/two", ThingType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![
            DynamicConverter {
                to: Some("/dynamic/two_of_1".to_string()),
                fun: Executor::for_local(r#"../nature_integrate_test_converter.dll:rtn_one"#),
            },
            DynamicConverter {
                to: Some("/dynamic/two_of_2".to_string()),
                fun: Executor::for_local(r#"../nature_integrate_test_converter.dll:rtn_one"#),
            }],
    };
    let rtn = query(&mut svc, instance);
    assert_eq!(226464870279356952826561520522393294145, rtn);

    // query target
    thread::sleep(time::Duration::from_millis(2000));
    let ins_db = InstanceDaoImpl::get_by_id(229131768420092721318239706157158451568).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_1", ins_db.thing.get_full_key());
    let ins_db = InstanceDaoImpl::get_by_id(217550842272210133848994195869177780408).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_2", ins_db.thing.get_full_key());
}
