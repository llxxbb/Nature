use db::*;
use diesel::prelude::*;
use serde::Serialize;
use std::fmt::Debug;
use super::*;
use util::*;

pub struct DeliveryDaoImpl;

impl DeliveryDaoTrait for DeliveryDaoImpl {
    fn insert<T: Sized + Serialize + Send + Debug>(carrier: &Carrier<T>) -> Result<u128> {
        debug!("insert carrier to db : {:?}", carrier);
        use self::schema::delivery;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let d = Delivery::new(carrier)?;
        let id = d.id.clone();
        let rtn = diesel::insert_into(delivery::table)
            .values(d)
            .execute(conn);
        match rtn {
            Ok(_) => Ok(vec_to_u128(&id)),
            Err(e) => Err(NatureError::from(e))
        }
    }

    fn delete(_id: &u128) -> Result<()> {
        unimplemented!()
    }

    fn move_to_error<T: Sized + Serialize + Debug>(_err: CarryError<T>) -> Result<()> {
        unimplemented!()
    }

    fn update_execute_time(_id: u128, _new_time: i64) -> Result<()> {
        unimplemented!()
    }

    fn get<T: Sized + Serialize + Debug>(_id: u128) -> Result<Carrier<T>> {
        unimplemented!()
    }
}
