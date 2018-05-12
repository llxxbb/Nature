use chrono::prelude::*;
use super::*;

#[test]
fn insert_define() {
    let define = ThingDefine {
        key: "/test".to_string(),
        description: Some("for test".to_string()),
        version: 1,
        states: None,
        fields: None,
        create_time: Local::now().naive_local(),
    };
    let thing = Thing {
        key: "/test".to_string(),
        version: 1,
    };
    if let Ok(Some(_)) = TableThingDefine::get(&thing) {
        let _ = TableThingDefine::delete(&thing);
    }
    let _ = TableThingDefine::insert(&define);
    let _ = TableThingDefine::get(&thing).unwrap().unwrap();
    TableThingDefine::delete(&thing).unwrap();
}