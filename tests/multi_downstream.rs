#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate log;
extern crate nature;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use ::rocket::{ignite, Rocket};
use common::*;
use nature::global::sys_init;
use self::nature::data::Instance;
use self::nature::db::*;
use self::rocket::http::ContentType;
use self::rocket_contrib::Json;
use std::thread;
use std::time;

mod common;

#[test]
fn multi_downstream() {
    let port = "7001";
    start_rocket_server(port);
    let threads = sys_init();
    println!("created threads: {:?}", threads);
    println!("------------------ insert thing define -----------------");
    let from = "/multi_downstream/from";
    let to_a = "/multi_downstream/toA";
    let to_b = "/multi_downstream/toB";
    new_thing_define(from);
    new_thing_define(to_a);
    new_thing_define(to_b);
    let url = format!("localhost:{}/multi_downstream", port);
    new_one_step_flow(from, to_a, &url);
    new_one_step_flow(from, to_b, &url);
    // TODO
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = from.to_string();
    let json = serde_json::to_string(&(
        instance)).unwrap();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let mut will_del = instance.clone();
    will_del.id = 290732852190148059426348090381887592931;
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
    assert_eq!(rtn, r#"{"Ok":290732852190148059426348090381887592931}"#);
    // get instance which is saved to db
    let _ins_db = InstanceDaoImpl::get_by_id(290732852190148059426348090381887592931).unwrap().unwrap();
}

pub fn start_rocket_server(port: &str) -> Rocket {
    std::env::set_var("ROCKET_PORT", port);
    ignite()
        .mount("/", routes![web_multi_downstream])
}

/// **Note** This do not receive System `Thing`'s instances
#[post("/multi_downstream", format = "application/json", data = "<instance>")]
fn web_multi_downstream(instance: Json<Instance>) -> Json<Option<Vec<Instance>>> {
    Json(None)
}
