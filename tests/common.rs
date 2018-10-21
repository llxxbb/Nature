extern crate nature;
extern crate nature_db;
extern crate rocket;

use nature::rpc::rocket_server;
use rocket::local::Client;
use self::nature_db::*;

#[allow(dead_code)]
pub fn get_test_client() -> Client {
    let rocket = rocket_server();
    Client::new(rocket).expect("valid rocket instance")
}

#[allow(dead_code)]
pub fn new_thing_define(key: &str) {
    let mut define = ThingDefine::default();
    define.key = key.to_string();
    let _ = ThingDefineDaoImpl::insert(&define);
}

#[allow(dead_code)]
pub fn new_one_step_flow(from: &str, to: &str, url: &str, protocol: &str) {
    let row = RawOneStepFlow {
        from_thing: from.to_string(),
        from_version: 0,
        to_thing: to.to_string(),
        to_version: 0,
        exe_protocol: protocol.to_string(),
        exe_url: url.to_string(),
        selector: None,
        group: None,
        weight: None,
    };
    let _ = OneStepFlowDaoImpl::insert(row);
}
