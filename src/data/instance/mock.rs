use std::ops::Deref;
use std::sync::*;
use super::*;

lazy_static! {
    pub static ref INSTANCE_LOCK: Mutex<u8> = Mutex::new(1);
    pub static ref INSTANCE_RESULT: Mutex<Result<UuidBytes>> = Mutex::new(Ok(UuidBytes::default()));
}



pub struct InstanceImpl;

impl InstanceImpl {
    pub fn verify(_instance: &mut Instance, _root: Root) -> Result<UuidBytes> {
        INSTANCE_RESULT.lock().unwrap().deref().clone()
    }
}

