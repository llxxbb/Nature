extern crate nature;
extern crate nature_common;

use std::env;

use nature::flow::IncomeController;
use nature_common::*;

mod common;

#[test]
fn convert_is_empty() {
    env::set_var("DATABASE_URL", "nature.sqlite");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = "/dynamec".to_string();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![],
    };
    let rtn = IncomeController::self_route(instance);
    assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
}

//fn target_is_null() {
//    // TODO
//}
//
//fn target_is_normal() {
//    // TODO
//}
//
//fn multi_converter() {
//    // TODO
//}

