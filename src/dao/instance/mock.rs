use std::sync::*;
use super::*;

#[derive(Debug, Copy, Clone)]
pub enum InstanceDaoMode {
    Ok,
    Err,
}
lazy_static! {
    pub static ref INSTANCE_DAO_MODE: Mutex<InstanceDaoMode> = Mutex::new(InstanceDaoMode::Ok);
}


pub struct InstanceDaoService;

impl InstanceDao for InstanceDaoService {
    fn insert(_instance: &Instance) -> Result<()> {
        let mode = &*INSTANCE_DAO_MODE.lock().unwrap();
        println!("InstanceDao mode is {:?}", mode);
        match mode {
            InstanceDaoMode::Ok => Ok(()),
            InstanceDaoMode::Err => Err(NatureError::DaoEnvironmentError("some error".to_string()))
        }
    }
    fn get_last_status_by_id(_id: &UuidBytes) -> Result<Option<Instance>> {
        unimplemented!()
    }
    fn source_stored(_instance: &Instance) -> Result<bool> {
        unimplemented!()
    }
}