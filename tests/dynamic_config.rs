extern crate nature;
extern crate nature_common;
extern crate nature_db;

use std::env;

use nature::flow::IncomeController;
use nature_common::*;
use nature_db::*;

mod common;

#[test]
fn convert_is_empty() {
    env::set_var("DATABASE_URL", "nature.sqlite");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing = Thing::new("/dynamic/converter/is/empty").unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![],
    };
    let rtn = IncomeController::self_route(instance);
    assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
}


#[test]
fn has_one_converter() {
    env::set_var("DATABASE_URL", "nature.sqlite");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing = Thing::new("/dynamic/target/is/null").unwrap();
    let instance = SelfRouteInstance {
        instance,
        converter: vec![DynamicConverter {
            to: None,
            fun: Executor {
                protocol: Protocol::LocalRust,
                url: r#"../../../../Nature-integrate-test-converter/target/debug/nature_integrate_test_converter.dll:rtn_one"#.to_string(),
                group: "".to_string(),
                proportion: 0,
            },
        }],
    };
    let rtn = IncomeController::self_route(instance);
    assert_eq!(67155089214163907246089937900589001447, rtn.unwrap());
    // check input
    let dao = InstanceDaoImpl {};
    let written = dao.get_by_id(67155089214163907246089937900589001447).unwrap().unwrap();
    assert_eq!("/D/dynamic/target/is/null", written.data.thing.get_full_key());
    assert_eq!(ThingType::Dynamic, written.data.thing.get_thing_type());
}

// #[test]
// fn write_one_target_to_db() {
//     env::set_var("DATABASE_URL", "nature.sqlite");
//     // prepare input para
//     let mut instance = Instance::default();
//     instance.data.thing.key = "/dynamic/write/one".to_string();
//     let instance = SelfRouteInstance {
//         instance,
//         converter: vec![DynamicConverter {
//             to: Some("/dynamic/one_target".to_string()),
//             fun: Executor {
//                 protocol: Protocol::LocalRust,
//                 url: r#"../../../../Nature-integrate-test-converter/target/debug/nature_integrate_test_converter.dll:rtn_one"#.to_string(),
//                 group: "".to_string(),
//                 proportion: 0,
//             },
//         }],
//     };
//     let rtn = IncomeController::self_route(instance);
//     assert_eq!(137820585092527411925203784740727265435, rtn.unwrap());
//     // query target
// //    assert_eq!()
// }
//
//fn multi_converter() {
//    // TODO
//}

