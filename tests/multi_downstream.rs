#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate log;
extern crate nature;
extern crate nature_common;
extern crate nature_db;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;


use common::*;
use nature::system::*;
use nature_common::Instance;
use self::nature::flow::*;
use self::nature_db::*;
use self::nature_db::service::*;
use std::thread;
use std::time;


mod common;


#[test]
fn local_converter() {
    let _ = sys_init();
    println!("------------------ insert thing define -----------------");
    let from = "/multi_downstream/from";
    let to_a = "/multi_downstream/toA";
    let to_b = "/multi_downstream/toB";
    new_thing_define(from);
    new_thing_define(to_a);
    new_thing_define(to_b);
    let url = format!("local://multi_downstream");
    new_one_step_flow(from, to_a, &url, "LocalRust");
    new_one_step_flow(from, to_b, &url, "LocalRust");
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = from.to_string();
    let id = SVC_INSTANCE.verify(&mut instance).unwrap();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let will_del = instance.clone();
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        println!("deleted : {} row", x);
    }
    println!("------------------ submit new instance -----------------");
    let rtn = Controller::input(instance);
    println!("saved instance id : {}", rtn.unwrap());
    thread::sleep(time::Duration::from_millis(500));
    println!("------------------ verify -----------------");
    // TODO
    // get instance which is saved to db
    let _ins_db = InstanceDaoImpl::get_by_id(id).unwrap().unwrap();
}

