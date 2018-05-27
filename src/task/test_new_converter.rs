use super::*;
use util::*;

#[test]
fn target_not_defined() {
    let _lock_define_cache = lock_and_set_mock_value(&THING_DEFINE_LOCK, &THING_DEFINE_CACHE_VALUE, Err(NatureError::VerifyError("ThingDefineCache mock : not defined".to_string())));
    match ConverterInfo::new(&Instance::default(), &Mapping::default()) {
        Err(NatureError::VerifyError(err)) => assert_eq!("ThingDefineCache mock : not defined", err),
        _ => panic!("should not go here"),
    };
}

#[test]
fn target_is_not_a_status_one() {
    let instance = Instance::default();
    let mut mapping = Mapping::default();
    mapping.to.key = "/B/ok".to_string();
    match ConverterInfo::new(&instance, &mapping) {
        Ok(x) => {
            assert_eq!(x.from, instance);
            assert_eq!(x.mapping, mapping);
            assert_eq!(x.last_status, None);
        }
        _ => panic!("should not go here"),
    };
}

#[test]
fn new_converter() {
    // if it is status set last instance by context
    // verify demand status
    // status include
    // status exclude
}
