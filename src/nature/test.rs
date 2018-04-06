use define::error::NatureError::VerifyError;
use super::*;
//use define::error::NatureError;

#[test]
fn flow_verified_failed() {
    let svc = NatureService;
    let instance = Instance::default();
    let rtn = svc.flow(instance);
    match rtn {
        Err(VerifyError(x)) => assert_eq!(x, "[biz] must not be empty!"),
        _ => panic!("should got error!"),
    }
}

#[test]
fn create_carrier_error(){
    let svc = NatureService;
    let mut instance = Instance::default();
    instance.data.thing.key = "/key/is/ok".to_string();;
    let rtn = svc.flow(instance);
    match rtn {
        Err(VerifyError(x)) => assert_eq!(x, "[biz] must not be empty!"),
        _ => panic!("should got error!"),
    }

}



