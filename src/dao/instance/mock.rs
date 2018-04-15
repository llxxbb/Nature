use super::*;

pub struct InstanceDaoService;

impl InstanceDao for InstanceDaoService {
    fn insert(_instance: &Instance)-> Result<UuidBytes> {
        unimplemented!()
    }
}