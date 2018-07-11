extern crate chrono;
extern crate nature;
extern crate rocket;
extern crate serde_json;

use chrono::prelude::*;
use self::nature::data::{Instance, ThingDefine};
use self::nature::db::*;
use self::rocket::http::ContentType;
use server_starter::*;

mod server_starter;

#[test]
fn key_defined() {
    // insert defined
    let key = "/key/defined".to_string();
    let define = ThingDefine {
        key: key.clone(),
        description: None,
        version: 0,
        states: None,
        fields: None,
        create_time: Local::now().naive_local(),
    };
    let result = ThingDefineDaoImpl::insert(&define);
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = key;
    let json = serde_json::to_string(&(
        instance)).unwrap();

    // call service
    let client = get_client();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    // verify returned
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Err":{"ThingNotDefined":"/B/key/undefined not defined"}}"#);
}

