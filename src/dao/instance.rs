use define::Instance;

pub trait InstanceDao {
    fn insert(instance: Instance);
}

pub struct InstanceDaoService;

impl InstanceDao for InstanceDaoService{
    fn insert(instance: Instance) {
        unimplemented!()
    }
}

pub static INSTANCE_DAO: InstanceDaoService = InstanceDaoService;
