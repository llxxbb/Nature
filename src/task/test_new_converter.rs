use super::*;

#[test]
fn target_not_initialized() {
    let instance = Instance::default();
    let mapping = Mapping::default();
    match ConverterInfo::new(&instance, &mapping) {
        Err(NatureError::VerifyError(err)) => assert_eq!("ThingDefineCache mock : unknown", err),
        _ => panic!("should not go here"),
    };
}

#[test]
fn target_not_defined() {
    let instance = Instance::default();
    let mut mapping = Mapping::default();
    mapping.to.key = "/B/err".to_string();
    match ConverterInfo::new(&instance, &mapping) {
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
        },
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
