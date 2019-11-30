#[macro_use]
extern crate lazy_static;
extern crate nature;
extern crate nature_common;
extern crate nature_db;

use std::collections::{HashMap, HashSet};
use std::env;

use reqwest::Client;

use nature::controller::IncomeController;
use nature_common::*;
use nature_db::*;

use crate::common::{CONN_STR, sleep, test_init};

pub mod common;

lazy_static! {
    pub static ref WEB_CLIENT: Client = Client::new();
}

#[test]
fn convert_is_empty() {
    env::set_var("DATABASE_URL", CONN_STR);
    // prepare input para
    let instance = new_with_type("/dynamic/converter/is/empty", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![],
    };
    let rtn = IncomeController::self_route(instance);
    assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
}

#[test]
fn target_is_null() {
    test_init();
    // prepare input para
    let instance = new_with_type("/dynamic/target/is/null", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: None,
            fun: Executor::for_local(""),
            use_upstream_id: false,
            delay: 0,
        }],
    };
    sleep(3000);
    let rtn = query(instance);
    assert_eq!(rtn, 338344869912127221082806885982185864543);
    // check input
    let written = InstanceDaoImpl::get_by_id(&ParaForQueryByID {
        id: 338344869912127221082806885982185864543,
        meta: "/D/dynamic/target/is/null:1".to_string(),
        state_version_from: 0,
        limit: 1,
    }).unwrap().unwrap();
    assert_eq!("/D/dynamic/target/is/null:1", written.data.meta);
}


#[test]
fn write_one_target_to_db() {
    test_init();
    // prepare input para
    let instance = new_with_type("/dynamic/write/one", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: Some("/dynamic/one_target".to_string()),
            fun: Executor::for_local(r#"nature_integrate_test_converter.dll:rtn_one"#),
            use_upstream_id: false,
            delay: 0,
        }],
    };
    sleep(5000);
    let rtn = query(instance);
    assert_eq!(rtn, 82729090071796891440669834987910301379);

    // query target
    sleep(3000);
    let ins_db = InstanceDaoImpl::get_by_id(&ParaForQueryByID {
        id: 303195405634045585338298858306929603801,
        meta: "/D/dynamic/one_target:1".to_string(),
        state_version_from: 0,
        limit: 1,
    }).unwrap().unwrap();
    assert_eq!("/D/dynamic/one_target:1", ins_db.meta);
}

#[test]
fn write_two_target_to_db() {
    test_init();
    // prepare input para
    let instance = new_with_type("/dynamic/write/two", MetaType::Dynamic).unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![
            DynamicConverter {
                to: Some("/dynamic/two_of_1".to_string()),
                fun: Executor::for_local(r#"nature_integrate_test_converter.dll:rtn_one"#),
                use_upstream_id: false,
                delay: 0,
            },
            DynamicConverter {
                to: Some("/dynamic/two_of_2".to_string()),
                fun: Executor::for_local(r#"nature_integrate_test_converter.dll:rtn_one"#),
                use_upstream_id: false,
                delay: 0,
            }],
    };
    sleep(5000);

    let rtn = query(instance);
    assert_eq!(rtn, 302121533483322119686559454437223883973);

    // query target
    sleep(2500);
    let ins_db = InstanceDaoImpl::get_by_id(&ParaForQueryByID {
        id: 251184288685302246237493378684975241377,
        meta: "/D/dynamic/two_of_1:1".to_string(),
        state_version_from: 0,
        limit: 1,
    }).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_1:1", ins_db.meta);
    let ins_db = InstanceDaoImpl::get_by_id(&ParaForQueryByID {
        id: 280748872477529468003584044421765998976,
        meta: "/D/dynamic/two_of_2:1".to_string(),
        state_version_from: 0,
        limit: 1,
    }).unwrap().unwrap();
    assert_eq!("/D/dynamic/two_of_2:1", ins_db.meta);
}


fn query(instance: SelfRouteInstance) -> u128 {
    let req = WEB_CLIENT.post("http://localhost:8080/self_route").json(&instance).build().unwrap();
    let mut rtn = WEB_CLIENT.execute(req).unwrap();
    let rtn = rtn.json::<Result<u128>>();
    let rtn = rtn.unwrap().unwrap();
    rtn
}

pub fn new_with_type(key: &str, meta: MetaType) -> Result<Instance> {
    if key.is_empty() {
        return Err(NatureError::VerifyError("key can not be empty".to_string()));
    }
    let key = Meta::key_standardize(key)?;
    Ok(Instance {
        id: 0,
        data: BizObject {
            meta: format!("{}{}:1", meta.get_prefix(), key),
            content: "".to_string(),
            context: HashMap::new(),
            states: HashSet::new(),
            state_version: 0,
            from: None,
            para: String::new(),
        },
        execute_time: 0,
        create_time: 0,
    })
}


