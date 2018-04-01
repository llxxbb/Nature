use define::Instance;

pub trait InstanceDao {
    fn insert(&self, instance: Instance);
}

pub struct InstanceDaoService;

impl InstanceDao for InstanceDaoService {
    fn insert(&self, instance: Instance) {
        unimplemented!()
    }
}

