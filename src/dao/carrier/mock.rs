use std::sync::Mutex;
///! Mock object of outer test
use super::*;

#[derive(Debug)]
pub enum CarrierDaoMode {
    Ok,
    Err,
}

lazy_static! {
    pub static ref CARRIER_DAO_MODE: Mutex<CarrierDaoMode> = Mutex::new(CarrierDaoMode::Ok);
}

pub struct CarrierDaoService;

impl CarrierDao for CarrierDaoService {
    fn insert<T: Sized + Serialize>(_carrier: &Carrier<T>) -> Result<UuidBytes> {
        let mode = CARRIER_DAO_MODE.lock().unwrap();
        let mode = &*mode;
        println!("CarrierDao calling {:?}", mode);
        match mode {
            CarrierDaoMode::Ok => Ok([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            CarrierDaoMode::Err => Err(NatureError::DaoEnvironmentError("some error".to_string()))
        }
    }
    fn delete(_id: &UuidBytes) -> Result<()> {
        unimplemented!()
    }
    fn move_to_error<T: Sized + Serialize>(_err: CarryError<T>) -> Result<()> {
        unimplemented!()
    }
    fn update_execute_time(_id: UuidBytes, _new_time: i64) -> Result<()> {
        unimplemented!()
    }
    fn get<T: Sized + Serialize>(_id: UuidBytes) -> Result<Carrier<T>>{
        unimplemented!()
    }
}