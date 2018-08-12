#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate log;
extern crate nature;
extern crate nature_common;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use common::*;
use nature::global::*;
use nature_common::Instance;
use self::nature::db::*;
use self::nature::fg_service::*;
use std::thread;
use std::time;

mod common;


#[test]
fn multi_downstream() {
    let _ = sys_init();
    println!("------------------ insert thing define -----------------");
    let from = "/multi_downstream/from";
    let to_a = "/multi_downstream/toA";
    let to_b = "/multi_downstream/toB";
    new_thing_define(from);
    new_thing_define(to_a);
    new_thing_define(to_b);
    let url = format!("local://multi_downstream");
    new_one_step_flow(from, to_a, &url);
    new_one_step_flow(from, to_b, &url);
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = from.to_string();
    let id = InstanceServiceImpl::verify(&mut instance).unwrap();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let will_del = instance.clone();
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        println!("deleted : {} row", x);
    }
    println!("------------------ submit new instance -----------------");
    let rtn = StoreService::input(instance);
    println!("saved instance id : {}", rtn.unwrap());
    thread::sleep(time::Duration::from_millis(500));
    println!("------------------ verify -----------------");
    // TODO
    // get instance which is saved to db
    let _ins_db = InstanceDaoImpl::get_by_id(id).unwrap().unwrap();
}

