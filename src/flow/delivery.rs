use chrono::prelude::*;
use global::*;
use nature_common::util::id_tool::generate_id;
use serde::Serialize;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use super::*;


pub trait DeliveryServiceTrait {
    fn create_carrier<T>(valuable: T, thing: &str, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Send + Debug;
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Debug, U: Sized + Serialize + Debug;
    fn create_batch_and_finish_carrier<T, U>(news: &Vec<Carrier<T>>, old: &Carrier<U>) -> Result<()> where T: Sized + Serialize + Send + Debug, U: Sized + Serialize + Debug;
    fn finish_carrier(id: u128) -> Result<()>;
    fn move_to_err<T>(err: NatureError, carrier: &Carrier<T>) where T: Sized + Serialize + Debug;
    fn new_carrier<T>(task: T, thing: &str, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Debug;
    fn send_carrier<T>(sender: &Mutex<Sender<Carrier<T>>>, carrier: Carrier<T>)
        where T: 'static + Sized + Serialize + Sync + Send + Debug;
    fn update_execute_time(_id: u128, _new_time: i64) -> Result<()>;
    fn get<T: Sized + Serialize + Debug>(_id: u128) -> Result<Carrier<T>>;
}


pub struct DeliveryServiceImpl<TD> {
    table_delivery: PhantomData<TD>,
}

impl<TD: DeliveryDaoTrait> DeliveryServiceTrait for DeliveryServiceImpl<TD> {
    fn create_carrier<T>(valuable: T, thing: &str, data_type: u8) -> Result<Carrier<T>>
        where T: Sized + Serialize + Send + Debug
    {
        let carrier = Self::new_carrier(valuable, &thing, data_type)?;
        let _ = TD::insert(&carrier)?;
        Ok(carrier)
    }

    /// by performance reason, for one-to-one carry we can reuse the beginning carry to finish all flows.
    /// That way we need not to communicate with DB for create new and delete old carrier.
    /// But for failure we must redo from beginning. but I think it has small chance.
    /// Another disadvantage is the failure information will be attached to the beginning.
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>, thing: String, data_type: u8) -> Result<Carrier<T>>
        where T: Sized + Serialize + Debug, U: Sized + Serialize + Debug,
    {
        let mut carrier = match Self::new_carrier(valuable, &thing, data_type) {
            Ok(new) => new,
            Err(err) => {
                DeliveryServiceImpl::<DeliveryDaoImpl>::move_to_err(err.clone(), &old);
                return Err(err);
            }
        };
        carrier.id = old.id; // the id is used for final finished
        Ok(carrier)
    }

    fn create_batch_and_finish_carrier<T, U>(news: &Vec<Carrier<T>>, old: &Carrier<U>) -> Result<()>
        where T: Sized + Serialize + Send + Debug, U: Sized + Serialize + Debug,
    {
        for v in news {
            TD::insert(v)?;
        }
        TD::delete(old.id)?;
        Ok(())
    }

    fn finish_carrier(id: u128) -> Result<()> {
        debug!("finished carrier for id: {:?}", id);
        TD::delete(id)
    }

    fn move_to_err<T>(err: NatureError, carrier: &Carrier<T>) where T: Sized + Serialize + Debug {
        let _ = TD::move_to_error(CarryError { err, carrier });
    }

    fn new_carrier<T>(task: T, thing: &str, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Debug {
        // this can avoid regenerate same content with different id
        let new_id = generate_id(&task)?;
        Ok(Carrier {
            content: CarrierContent {
                data: task,
                thing: thing.to_string(),
                data_type,
            },
            id: new_id,
            create_time: Local::now().timestamp_millis(),
            execute_time: Local::now().timestamp_millis(),
        })
    }
    fn send_carrier<T>(sender: &Mutex<Sender<Carrier<T>>>, carrier: Carrier<T>)
        where T: 'static + Sized + Serialize + Sync + Send + Debug {
        let _send_status = sender.lock().unwrap().send(carrier);
    }

    fn update_execute_time(id: u128, new_time: i64) -> Result<()> {
        TD::update_execute_time(id, new_time)
    }

    fn get<T: Sized + Serialize + Debug>(id: u128) -> Result<Carrier<T>> {
        TD::get(id)
    }
}
