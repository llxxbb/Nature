#[allow(unused_imports)]
use nature::common::{ConverterReturned, Instance};

#[test]
fn string_parse_i32() {
    assert_eq!(0, "0".parse::<i32>().unwrap());
}

#[test]
#[allow(unused_mut)]
fn json_test() {
    // // instance
    // let mut instance = Instance::new("hello").unwrap();
    // instance.states.insert("a".to_string());
    // let result = serde_json::to_string(&instance).unwrap();
    // println!("{}",result);
}