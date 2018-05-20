use serde::Serialize;
use super::*;
use task::CarryError;


pub struct CarrierDaoService;

impl CarrierDao for CarrierDaoService {
    fn insert<T: Sized + Serialize>(_carrier: &Carrier<T>) -> Result<UuidBytes> {
        Ok([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
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
    fn get<T: Sized + Serialize>(_id: UuidBytes) -> Result<Carrier<T>> {
        unimplemented!()
    }
}