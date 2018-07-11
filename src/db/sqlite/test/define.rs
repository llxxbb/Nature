use chrono::prelude::*;
use db::trait_define::ThingDefineDaoTrait;
use super::*;


#[test]
fn define_test() {
    println!("----------------- define_test --------------------");
    // prepare data to insert
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
        thing_type: ThingType::Business,
    };
    // delete if it exists 
    if let Ok(Some(_)) = ThingDefineDaoImpl::get(&thing) {
        let _ = ThingDefineDaoImpl::delete(&thing);
    }
    // insert
    let rtn = ThingDefineDaoImpl::insert(&define);
    assert_eq!(rtn.unwrap(), 1);
    // repeat insert
    let rtn = ThingDefineDaoImpl::insert(&define);
    assert_eq!(rtn.err().unwrap(), NatureError::DaoDuplicated);
    // find inserted
    let row = ThingDefineDaoImpl::get(&thing).unwrap().unwrap();
    assert_eq!(row, define);
    // delete it
    ThingDefineDaoImpl::delete(&thing).unwrap();
}