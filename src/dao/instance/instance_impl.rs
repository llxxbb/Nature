use super::*;

pub struct InstanceDaoService;

impl InstanceDao for InstanceDaoService {
    fn insert(_instance: &Instance)-> Result<()> {
        // TODO
        Ok(())
    }
    fn get_last_status_by_id(_id: &UuidBytes) -> Result<Option<Instance>> {
        unimplemented!()
    }
    fn source_stored(_instance: &Instance) -> Result<bool>{
        unimplemented!()
    }
}