use std::sync::Mutex;
///! Mock object of outer test
use super::*;

#[derive(Debug)]
pub enum Mode {
    Ok,
    Err,
}
lazy_static! {
    pub static ref MODE: Mutex<Mode> = Mutex::new(Mode::Ok);
}

pub struct CarrierDaoService;

impl CarrierDao for CarrierDaoService {
    fn insert<T>(_carrier: &Carrier<T>) -> Result<UuidBytes> {
        let mode = MODE.lock().unwrap();
        println!("CarrierDao calling {:?}", mode);
        let mode = &*mode;
        println!("CarrierDao calling {:?}", mode);
        match mode {
            Mode::Ok => Ok([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            Mode::Err => Err(NatureError::CarrierDaoError("some error".to_string()))
        }
    }
}