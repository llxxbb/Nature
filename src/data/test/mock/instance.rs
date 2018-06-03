use data::instance::Instance;
use data::InstanceTrait;
use data::thing::Root;
use std::ops::Deref;
use super::*;
use uuid::UuidBytes;

lazy_static! {
    pub static ref DATA_INSTANCE_LOCK: Mutex<u8> = Mutex::new(1);
    pub static ref DATA_INSTANCE_RESULT: Mutex<Result<UuidBytes>> = Mutex::new(Ok(UuidBytes::default()));
}

pub struct MockInstanceTrait;

impl InstanceTrait for MockInstanceTrait {
    fn verify(_instance: &mut Instance, _root: Root) -> Result<UuidBytes> {
        DATA_INSTANCE_RESULT.lock().unwrap().deref().clone()
    }
}

