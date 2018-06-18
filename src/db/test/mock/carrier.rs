//use db::trait_define::DeliveryDao;
//use serde::Serialize;
//use super::*;
//use task::CarryError;
//
//
//pub struct TableDeliveryMock;
//
//impl DeliveryDao for TableDeliveryMock {
//    fn insert<T: Sized + Serialize>(_carrier: &Carrier<T>) -> Result<u128> {
//        Ok(0)
//    }
//    fn delete(_id: &u128) -> Result<()> {
//        unimplemented!()
//    }
//    fn move_to_error<T: Sized + Serialize>(_err: CarryError<T>) -> Result<()> {
//        unimplemented!()
//    }
//    fn update_execute_time(_id: u128, _new_time: i64) -> Result<()> {
//        unimplemented!()
//    }
//    fn get<T: Sized + Serialize>(_id: u128) -> Result<Carrier<T>> {
//        unimplemented!()
//    }
//}