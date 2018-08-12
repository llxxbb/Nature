extern crate chrono;
extern crate log;
extern crate nature;
extern crate rocket;
extern crate serde_json;
extern crate nature_common;

use common::*;
use nature::global::sys_init;
use nature_common::Instance;
use self::nature::db::*;
use self::rocket::http::ContentType;
use std::thread;
use std::time;

mod common;

#[test]
fn web_post_and_save_one() {
    let threads = sys_init();
    println!("created threads: {:?}", threads);
    println!("------------------ insert thing define -----------------");
    let key = "/save_one".to_string();
    new_thing_define(&key);
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = key;
    let json = serde_json::to_string(&(
        instance)).unwrap();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let mut will_del = instance.clone();
    will_del.id = 327082908364944575799907940044792342105;
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        println!("delete {} rows", x);
    }
    println!("------------------ submit new instance -----------------");
    let client = get_test_client();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    thread::sleep(time::Duration::from_millis(500));
    println!("------------------ verify -----------------");
    // check return result
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":327082908364944575799907940044792342105}"#);
    // get instance which is saved to db
    let _ins_db = InstanceDaoImpl::get_by_id(327082908364944575799907940044792342105).unwrap().unwrap();
}

