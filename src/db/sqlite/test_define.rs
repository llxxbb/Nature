use chrono::prelude::*;
use super::*;

#[test]
fn insert_define() {
    let define = ThingDefine {
        key: "/test".to_string(),
        description: Some("description".to_string()),
        version: 100,
        states: Some("status".to_string()),
        fields: Some("fields".to_string()),
        create_time: Local::now().naive_local(),
    };
    let thing = Thing {
        key: "/test".to_string(),
        version: 100,
    };
    if let Ok(Some(_)) = TableThingDefine::get(&thing) {
        let _ = TableThingDefine::delete(&thing);
    }
    let rtn = TableThingDefine::insert(&define);
    println!("insert result : {:?}", rtn);
    let row = TableThingDefine::get(&thing).unwrap().unwrap();
    assert_eq!(row, define);
    TableThingDefine::delete(&thing).unwrap();
}