mod common;

use ::nature::flow::IncomeController;
use ::nature_common::Instance;

#[test]
fn dynamec_config_test(){
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