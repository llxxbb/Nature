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
    fn insert(_instance: &Instance) -> Result<UuidBytes> {
        let mode = &*INSTANCE_DAO_MODE.lock().unwrap();
        println!("InstanceDao mode is {:?}", mode);
        match mode {
            InstanceDaoMode::Ok => Ok([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            InstanceDaoMode::Err => Err(NatureError::InstanceDaoError("some error".to_string()))
        }
    }
}