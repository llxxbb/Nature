use db::*;
use diesel::prelude::*;
use serde::Serialize;
use super::*;
use task::CarryError;
use util::*;

pub struct TableDelivery;

impl DaoDelivery for TableDelivery {
    fn insert<T: Sized + Serialize + Send>(carrier: &Carrier<T>) -> Result<u128> {
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

    fn move_to_error<T: Sized + Serialize>(_err: CarryError<T>) -> Result<()> {
        unimplemented!()
    }

    fn update_execute_time(_id: u128, _new_time: i64) -> Result<()> {
        unimplemented!()
    }

    fn get<T: Sized + Serialize>(_id: u128) -> Result<Carrier<T>> {
        unimplemented!()
    }
}
