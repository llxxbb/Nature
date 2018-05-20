use serde::Serialize;
use super::*;
use task::CarryError;

pub struct CarrierDaoService;

impl CarrierDao for CarrierDaoService {
    fn insert<T: Sized + Serialize>(_carrier: &Carrier<T>) -> Result<[u8; 16]> {
        unimplemented!()
    }

    fn delete(_id: &[u8; 16]) -> Result<()> {
        unimplemented!()
    }

    fn move_to_error<T: Sized + Serialize>(_err: CarryError<T>) -> Result<()> {
        unimplemented!()
    }

    fn update_execute_time(_id: [u8; 16], _new_time: i64) -> Result<()> {
        unimplemented!()
    }

    fn get<T: Sized + Serialize>(_id: [u8; 16]) -> Result<Carrier<T>> {
        unimplemented!()
    }
}
