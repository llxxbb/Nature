use global::*;
use super::*;

pub struct TableInstance;

impl InstanceDao for TableInstance {
    fn insert(instance: &Instance) -> Result<()> {
        unimplemented!()
    }
    fn get_last_status_by_id(id: &UuidBytes) -> Result<Option<Instance>> {
        unimplemented!()
    }
    /// check whether source stored earlier
    fn source_stored(instance: &Instance) -> Result<bool> {
        unimplemented!()
    }
}