extern crate nature;
extern crate nature_common;
extern crate rocket;
extern crate serde_json;
extern crate dotenv;

use nature::flow::*;
use nature_common::*;

mod common;

#[test]
fn instance_key_is_null() {
    let rtn = Controller::input(Instance::default());
    match rtn.err().unwrap() {
        NatureError::VerifyError(ss) => assert_eq!(ss, "key length can\'t be zero"),
        err => {
            println!("{:?}", err);
            panic!("un match")
        }
    }
}

#[test]
fn instance_key_undefined() {
    dotenv::dotenv().ok();
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = "/key/undefined".to_string();
    let rtn = Controller::input(instance);
    match rtn.err().unwrap() {
        NatureError::ThingNotDefined(ss) => assert_eq!(ss, "/key/undefined not defined"),
        err => {
            println!("{:?}", err);
            panic!("un match")
        }
    }
}
