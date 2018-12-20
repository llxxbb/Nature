extern crate nature;
extern crate nature_db;
extern crate rocket;

use rocket::local::Client;

use nature::rpc::rocket_server;

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
