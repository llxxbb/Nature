#[allow(unused_imports)]
use nature::common::{ConverterReturned, Instance};
use nature::common::NatureError;

#[test]
fn string_parse_i32() {
    assert_eq!(0, "0".parse::<i32>().unwrap());
}

