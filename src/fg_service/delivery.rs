use chrono::prelude::*;
use serde::Serialize;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use super::*;
use util::id_tool::generate_id;


/// carry every kinds of **Task Info** to process which stayed at `Ready` table
#[derive(Debug, Clone)]
pub struct Carrier<T> where T: Sized + Serialize + Debug {
    pub id: u128,
    pub create_time: i64,
    pub execute_time: i64,
    pub content: CarrierContent<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CarrierContent<T> {
    pub data: T,
    pub thing: String,
    pub data_type: u8,
}

#[derive(Debug)]
pub struct CarryError<T> where T: Sized + Serialize + Debug {
    pub err: NatureError,
    pub carrier: Carrier<T>,
}


impl<T> Carrier<T> where T: Sized + Serialize + Debug {}

impl<T> Deref for Carrier<T> where T: Sized + Serialize + Debug {
    type Target = T;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.content.data
    }
}


pub trait DeliveryServiceTrait {
    fn create_carrier<T>(valuable: T, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Send + Debug;
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Debug, U: Sized + Serialize + Debug;
    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>, thing: String, data_type: u8) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize + Send + Debug, U: Sized + Serialize + Debug;
    fn finish_carrier(id: u128) -> Result<()>;
    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize + Debug;
    fn new_carrier<T>(task: T, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Debug;
    fn send_carrier<T>(sender: &Mutex<Sender<Carrier<T>>>, carrier: Carrier<T>)
        where T: 'static + Sized + Serialize + Sync + Send + Debug;
}


pub struct DeliveryServiceImpl<TD> {
    table_delivery: PhantomData<TD>,
}

impl<TD: DeliveryDaoTrait> DeliveryServiceTrait for DeliveryServiceImpl<TD> {
    fn create_carrier<T>(valuable: T, thing: String, data_type: u8) -> Result<Carrier<T>>
        where T: Sized + Serialize + Send + Debug
    {
        let carrier = Self::new_carrier(valuable, thing, data_type)?;
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
        let mut carrier = match Self::new_carrier(valuable, thing, data_type) {
            Ok(new) => new,
            Err(err) => {
                DeliveryServiceImpl::<DeliveryDaoImpl>::move_to_err(err.clone(), old);
                return Err(err);
            }
        };
        carrier.id = old.id; // the id is used for final finished
        Ok(carrier)
    }

    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>, thing: String, data_type: u8) -> Result<Vec<Carrier<T>>>
        where T: Sized + Serialize + Send + Debug, U: Sized + Serialize + Debug,
    {
        let mut rtn: Vec<Carrier<T>> = Vec::new();
        for v in valuables {
            let _ = match Self::new_carrier(v, thing.clone(), data_type) {
                Ok(new) => {
                    TD::insert(&new)?;
                    rtn.push(new);
                }
                Err(err) => {
                    DeliveryServiceImpl::<DeliveryDaoImpl>::move_to_err(err.clone(), old);
                    return Err(err);
                }
            };
        }
        TD::delete(old.id)?;
        Ok(rtn)
    }

    fn finish_carrier(id: u128) -> Result<()> {
        debug!("finished carrier for id: {:?}", id);
        TD::delete(id)
    }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize + Debug {
        let _ = TD::move_to_error(CarryError { err, carrier });
    }

    fn new_carrier<T>(task: T, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Debug {
        // this can avoid regenerate same content with different id
        let new_id = generate_id(&task)?;
        Ok(Carrier {
            content: CarrierContent {
                data: task,
                thing,
                data_type,
            },
            id: new_id,
            create_time: Local::now().timestamp_millis(),
            execute_time: Local::now().timestamp_millis(),
        })
    }
    fn send_carrier<T>(sender: &Mutex<Sender<Carrier<T>>>, carrier: Carrier<T>)
        where T: 'static + Sized + Serialize + Sync + Send + Debug {
        debug!("send carrier for id: {:?}", carrier.id);
        let _send_status = sender.lock().unwrap().send(carrier);
    }
}


pub enum DataType {
    Store = 1,
    Dispatch = 2,
    Convert = 3,
    ParallelBatch = 11,
    QueueBatch = 12,
}
