use super::*;

pub struct InstanceDaoService;

impl InstanceDao for InstanceDaoService {
    fn insert(instance: &Instance)-> Result<UuidBytes> {
        // TODO
        Ok(instance.id)
    }
}