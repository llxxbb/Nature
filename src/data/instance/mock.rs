use super::*;
use std::sync::*;

#[derive(Debug)]
pub enum InstanceVerifyMode {
    Ok,
    Err,
}

lazy_static! {
    pub static ref INSTANCE_VERIFY_MODE: Mutex<InstanceVerifyMode> = Mutex::new(InstanceVerifyMode::Ok);
}



pub struct InstanceImpl;

impl InstanceImpl{
    pub fn verify(_instance: &mut Instance) -> Result<UuidBytes> {
        let mode = INSTANCE_VERIFY_MODE.lock().unwrap();
        let mode = &*mode;
        println!("INSTANCE_VERIFY_MODE {:?}", mode);
        match mode {
            InstanceVerifyMode::Ok => Ok([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            InstanceVerifyMode::Err => Err(NatureError::VerifyError("some error".to_string()))
        }
    }
}

impl InstanceTrait for InstanceImpl {
    fn born(_instance: Instance) -> Result<[u8; 16]> {
        Ok([11, 172, 237, 228, 64, 20, 63, 157, 183, 32, 23, 63, 104, 161, 201, 51])
    }
    fn serial(_batch: SerialBatchInstance) -> Result<()> {
        unimplemented!()
    }

    fn parallel(_batch: ParallelBatchInstance) -> Result<()> {
        unimplemented!()
    }
}

