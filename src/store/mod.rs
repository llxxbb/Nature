use dao::instance::InstanceDao;
use define::*;
use uuid::UuidBytes;

pub trait Store {
    fn store<T: InstanceDao>(&self, store_dao: &T) -> Result<UuidBytes>;
}


impl Store for Instance {
    fn store<T: InstanceDao>(&self, store_dao: &T) -> Result<UuidBytes> {
        // TODO
        unimplemented!()
    }
}

