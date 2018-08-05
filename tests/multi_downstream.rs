#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate log;
extern crate nature;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use common::*;
use nature::global::*;
use self::nature::data::Instance;
use self::nature::db::*;
use self::nature::fg_service::*;
use std::thread;
use std::time;

mod common;


#[test]
fn multi_downstream() {
    let threads = sys_init();
    println!("created threads: {:?}", threads);
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
    // TODO
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = from.to_string();
    let _ = serde_json::to_string(&(instance)).unwrap();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let mut will_del = instance.clone();
    will_del.id = 290732852190148059426348090381887592931;
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        assert_eq!(x, 1);
    }
    println!("------------------ submit new instance -----------------");
    let _ = StoreService::input(instance);
    thread::sleep(time::Duration::from_millis(500));
    println!("------------------ verify -----------------");
    // get instance which is saved to db
    let _ins_db = InstanceDaoImpl::get_by_id(290732852190148059426348090381887592931).unwrap().unwrap();
}

