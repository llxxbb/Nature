extern crate dotenv;
extern crate nature;
extern crate nature_common;
extern crate rocket;
extern crate serde_json;

use nature::flow::*;
use nature_common::*;

mod common;


#[test]
fn instance_key_undefined() {
    dotenv::dotenv().ok();
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = "/key/undefined".to_string();
    let rtn = IncomeController::input(instance);
    match rtn.err().unwrap() {
        NatureError::ThingNotDefined(ss) => assert_eq!(ss, "/key/undefined not defined"),
        err => {
            println!("{:?}", err);
            panic!("un match")
        }
    }
}
