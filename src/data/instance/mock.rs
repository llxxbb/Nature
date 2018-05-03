use super::*;
use std::sync::*;

#[derive(Debug)]
pub enum InstanceVerifyMode {
    Ok,
    Err,
}

lazy_static! {
    pub static ref INSTANCE_VERIFY_MODE: Mutex<InstanceVerifyMode> = Mutex::new(InstanceVerifyMode::Ok);
}



pub struct InstanceImpl;

impl InstanceImpl{
    pub fn verify(_instance: &mut Instance, _root: Root) -> Result<UuidBytes> {
        let mode = INSTANCE_VERIFY_MODE.lock().unwrap();
        let mode = &*mode;
        println!("INSTANCE_VERIFY_MODE {:?}", mode);
        match mode {
            InstanceVerifyMode::Ok => Ok([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            InstanceVerifyMode::Err => Err(NatureError::VerifyError("some error".to_string()))
        }
    }
}

