use global::*;
use super::*;

pub struct TableInstance;

impl InstanceDao for TableInstance {
    fn insert(_instance: &Instance) -> Result<()> {
        unimplemented!()
    }
    fn get_last_status_by_id(_id: &UuidBytes) -> Result<Option<Instance>> {
        unimplemented!()
    }
    /// check whether source stored earlier
    fn source_stored(_instance: &Instance) -> Result<bool> {
        unimplemented!()
    }
}