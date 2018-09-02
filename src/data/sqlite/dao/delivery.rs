use diesel::result::*;
use serde::Serialize;
use std::fmt::Debug;
use super::*;
use util::*;

pub struct DeliveryDaoImpl;

impl DeliveryDaoTrait for DeliveryDaoImpl {
    fn insert<T: Sized + Serialize + Send + Debug>(carrier: &Carrier<T>) -> Result<u128> {
        use self::schema::delivery;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let d = RawDelivery::new(carrier)?;
        let id = d.id.clone();
        let rtn = diesel::insert_into(delivery::table).values(d).execute(conn);
        match rtn {
            Ok(_) => {
//                debug!("insert carrier to db for id: {:?} successful", carrier.id);
                Ok(vec_to_u128(&id))
            }
            Err(Error::DatabaseError(kind, info)) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    Ok(vec_to_u128(&id))
                }
                DatabaseErrorKind::__Unknown => {
                    Err(NatureErrorWrapper::from(NatureError::DaoEnvironmentError(format!("{:?}", info))))
                }
                _ => Err(NatureErrorWrapper::from(NatureError::DaoLogicalError(format!("{:?}", info)))),
            },
            Err(e) => {
                error!(
                    "insert carrier to db for id: {:?} occurred error",
                    carrier.id
                );
                Err(NatureErrorWrapper::from(e))
            }
        }
    }

    fn delete(carrier_id: u128) -> Result<()> {
        use self::schema::delivery::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let rtn = diesel::delete(delivery.filter(id.eq(u128_to_vec_u8(carrier_id)))).execute(conn);
        match rtn {
            Ok(_) => Ok(()),
            Err(err) => Err(NatureErrorWrapper::from(err))
        }
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
