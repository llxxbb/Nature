use super::*;


pub struct TableInstance;

impl InstanceDao for TableInstance {
    fn insert(_instance: &Instance) -> Result<usize> {
        unimplemented!()
    }
    fn get_last_status_by_id(_id: &UuidBytes) -> Result<Option<Instance>> {
        unimplemented!()
    }
    fn is_exists(_instance: &Instance) -> Result<bool> {
        unimplemented!()
    }
}