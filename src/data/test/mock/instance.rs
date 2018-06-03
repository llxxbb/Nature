use data::instance::Instance;
use data::InstanceTrait;
use data::thing::Root;
use uuid::UuidBytes;
use std::ops::Deref;
use super::*;

lazy_static! {
    pub static ref INSTANCE_LOCK: Mutex<u8> = Mutex::new(1);
    pub static ref INSTANCE_RESULT: Mutex<Result<UuidBytes>> = Mutex::new(Ok(UuidBytes::default()));
}

pub struct MockInstanceTrait;

impl InstanceTrait for MockInstanceTrait {
    fn verify(_instance: &mut Instance, _root: Root) -> Result<UuidBytes> {
        INSTANCE_RESULT.lock().unwrap().deref().clone()
    }
}

