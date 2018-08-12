extern crate nature;
extern crate nature_common;
extern crate rocket;
extern crate serde_json;

use common::*;
use nature_common::Instance;
use self::rocket::http::ContentType;

mod common;


#[test]
fn must_input_key() {
    println!("must input_key---------------");
    // prepare input para
    let json = serde_json::to_string(&(
        Instance::default())).unwrap();

    // call service
    let client = get_test_client();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    // verify returned
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Err":{"VerifyError":"key length can't be zero"}}"#);
}

#[test]
fn key_undefined() {
    println!("must input_key---------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = "/key/undefined".to_string();
    let json = serde_json::to_string(&(
        instance)).unwrap();

    // call service
    let client = get_test_client();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    // verify returned
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Err":{"ThingNotDefined":"/key/undefined not defined"}}"#);
}

