extern crate nature_common;

use nature_common::*;

#[test]
fn string_parse_i32() {
    let t = Thing {
        key: "lxb".to_string(),
        version: "0".parse().unwrap(),
        thing_type: ThingType::Business,
    };
    println!("{:?}", t);
}