extern crate nature_common;
extern crate nature;
mod common;

use nature::flow::IncomeController;
use nature_common::*;
use std::env;

#[test]
fn convert_is_empty(){
    env::set_var("DATABASE_URL", "nature.sqlite");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = "/dynamec".to_string();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![]
    };
    let rtn = IncomeController::self_route(instance);
    assert_eq!(rtn.err().unwrap(),Err(NatureError::VerifyError("hhe".to_string())));
}

fn target_is_null(){
    // TODO
}
fn target_is_normal(){
    // TODO
}
fn multi_converter(){
    // TODO
}

