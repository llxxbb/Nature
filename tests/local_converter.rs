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
use self::nature::data::*;
use self::nature::flow::*;
use std::thread;
use std::time;

mod common;


#[test]
fn local_converter() {
    let _ = sys_init();
    println!("------------------ insert thing define -----------------");
    let from = "/local_converter/from";
    let to = "/local_converter/to";
    new_thing_define(from);
    new_thing_define(to);
    let url = r#"../../../Nature-integrate-test-converter/target/debug/nature_integrate_test_converter.dll:rtn_one"#;
    new_one_step_flow(from, to, &url, "LocalRust");
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = from.to_string();
    let _id = InstanceServiceImpl::verify(&mut instance).unwrap();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let will_del = instance.clone();
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        println!("deleted : {} row", x);
    }
    println!("------------------ submit new instance -----------------");
    let _rtn = StoreService::input(instance);
    thread::sleep(time::Duration::from_millis(1000));
    println!("------------------ verify -----------------");
    // get instance which is saved to db
    let _ins_db = InstanceDaoImpl::get_by_key("/local_converter/to",217789594388339757346716979317903552035).unwrap().unwrap();
}

