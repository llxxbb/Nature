use define::*;
use uuid::UuidBytes;

pub trait Store {
    fn store(&self) -> Result<UuidBytes>;
}

impl Store for Instance {
    fn store(&self) -> Result<UuidBytes> {
        // TODO
        unimplemented!()
    }
}

