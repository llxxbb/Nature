#[allow(unused_imports)]
use nature::common::{ConverterReturned, Instance};
use nature::common::NatureError;

#[test]
fn string_parse_i32() {
    assert_eq!(0, "0".parse::<i32>().unwrap());
}

#[test]
fn json_result_ok_null() {
    let rtn: Result<(),NatureError> = Ok(());
    let s_rtn = serde_json::to_string(&rtn).unwrap();
    println!("{}", s_rtn)
}

#[test]
fn json_result_ok_option() {
    let rtn: Result<Option<String>,NatureError> = Ok(None);
    let s_rtn = serde_json::to_string(&rtn).unwrap();
    println!("{}", s_rtn)
}