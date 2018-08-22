extern crate nature;
extern crate nature_common;
extern crate rocket;
extern crate serde_json;

use nature::flow::*;
use nature::global::*;
use nature_common::*;

mod common;

#[test]
fn instance_key_is_null() {
    let rtn = StoreService::input(Instance::default());
    match rtn.err().unwrap().err {
        NatureError::VerifyError(ss) => assert_eq!(ss, "key length can\'t be zero"),
        err => {
            println!("{:?}", err);
            panic!("un match")
        }
    }
}

#[test]
fn instance_key_undefined() {
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = "/key/undefined".to_string();
    let rtn = StoreService::input(instance);
    match rtn.err().unwrap().err {
        NatureError::ThingNotDefined(ss) => assert_eq!(ss, "/key/undefined not defined"),
        err => {
            println!("{:?}", err);
            panic!("un match")
        }
    }
}
