use define::*;
use instance::Instance;
use serde_json;
use uuid::*;

pub trait Nature {
    fn flow(&self, thing: Instance) -> Result<UuidBytes>;
}

pub struct NatureService;

impl Nature for NatureService {
    fn flow(&self, instance: Instance) -> Result<UuidBytes> {
        let mut instance = instance;
        instance.store()
    }
}

