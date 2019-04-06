#[test]
fn string_parse_i32() {
    assert_eq!(0, "0".parse().unwrap());
}