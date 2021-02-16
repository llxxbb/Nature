use std::collections::HashMap;

use crate::send_with_context;

#[ignore]
#[test]
fn multi_warehouse() {
    #[derive(Serialize)]
    struct Order(String);

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("self".to_string(), "self".to_string());
    let _id = send_with_context("order", &Order("A".to_string()), &map).unwrap();

    map.clear();
    map.insert("third".to_string(), "third".to_string());
    let _id = send_with_context("order", &Order("B".to_string()), &map).unwrap();

    map.clear();
    map.insert("self".to_string(), "self".to_string());
    map.insert("third".to_string(), "third".to_string());
    let _id = send_with_context("order", &Order("C".to_string()), &map).unwrap();

    map.clear();
    let _id = send_with_context("order", &Order("D".to_string()), &map).unwrap();
}
