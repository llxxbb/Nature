#![feature(plugin)]
extern crate chrono;
extern crate log;
extern crate nature;
extern crate nature_common;
extern crate nature_db;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;


use std::thread;
use std::time;

use common::*;
use nature::system::*;
use nature_common::Instance;

use self::nature::flow::*;
use self::nature_db::*;

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
    let _ = OneStepFlowDaoImpl::insert_by_biz(from, to_a, &url, "LocalRust");
    let _ = OneStepFlowDaoImpl::insert_by_biz(from, to_b, &url, "LocalRust");
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = from.to_string();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let will_del = instance.clone();
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        println!("deleted : {} row", x);
    }
    println!("------------------ submit new instance -----------------");
    let rtn = IncomeController::input(instance);
    println!("saved instance id : {}", rtn.clone().unwrap());
    thread::sleep(time::Duration::from_millis(500));
    println!("------------------ verify -----------------");
    // get instance which is saved to db
    let i_d = InstanceDaoImpl {};
    let _ins_db = i_d.get_by_id(rtn.unwrap()).unwrap().unwrap();
}

