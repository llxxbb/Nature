extern crate chrono;
extern crate log;
extern crate nature;
extern crate rocket;
extern crate serde_json;

use chrono::prelude::*;
use nature::util::setup_logger;
use self::nature::data::{Instance, ThingDefine};
use self::nature::db::*;
use self::rocket::http::ContentType;
use server_starter::*;

mod server_starter;

#[test]
fn key_defined() {
    let _ = setup_logger();
    println!("------------------ insert thing define -----------------");
    let key = "/key/defined".to_string();
    let define = ThingDefine {
        key: key.clone(),
        description: None,
        version: 0,
        states: None,
        fields: None,
        create_time: Local::now().naive_local(),
    };
    let _ = ThingDefineDaoImpl::insert(&define);
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = key;
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
    let client = get_client();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    println!("------------------ verify -----------------");

    // TODO saved to db
    // TODO delivery removed
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":290732852190148059426348090381887592931}"#);
}

