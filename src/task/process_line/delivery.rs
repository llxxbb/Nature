extern crate multiqueue;

use self::multiqueue::*;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use super::*;

pub trait DeliveryTrait {
    fn create_carrier<T>(valuable: T, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize + Send;
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>, thing: String, data_type: u8) -> Result<Carrier<T>> where T: Sized + Serialize, U: Sized + Serialize;
    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>, thing: String, data_type: u8) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize + Send, U: Sized + Serialize;
    fn finish_carrier(id: &u128) -> Result<()>;
    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize;
}

pub fn send_carrier<T>(sender: &Mutex<MPMCSender<Carrier<T>>>, carrier: Carrier<T>)
    where T: 'static + Sized + Serialize + Sync + Send {
    sender.lock().unwrap().try_send(carrier).unwrap();
}

pub fn start_thread<T, F>(receiver: &'static Mutex<MPMCReceiver<Carrier<T>>>, f: F)
    where
        T: Serialize + Send,
        F: 'static + Fn(Carrier<T>) + Send
{
    use std::ops::Deref;
    thread::spawn(move || {
        let guard = receiver.lock().unwrap();
        let receiver = guard.deref();
        for next in receiver {
            f(next);
        }
    });
}

pub struct DeliveryImpl<TD> {
    table_delivery: PhantomData<TD>,
}

impl<TD: DaoDelivery> DeliveryTrait for DeliveryImpl<TD> {
    fn create_carrier<T>(valuable: T, thing: String, data_type: u8) -> Result<Carrier<T>>
        where T: Sized + Serialize + Send
    {
        let carrier = Carrier::new(valuable, thing, data_type)?;
        let _ = TD::insert(&carrier)?;
        Ok(carrier)
    }

    /// by performance reason, for one-to-one carry we can reuse the beginning carry to finish all flows.
    /// That way we need not to communicate with DB for create new and delete old carrier.
    /// But for failure we must redo from beginning. but I think it has small chance.
    /// Another disadvantage is the failure information will be attached to the beginning.
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>, thing: String, data_type: u8) -> Result<Carrier<T>>
        where T: Sized + Serialize, U: Sized + Serialize,
    {
        let mut carrier = match Carrier::new(valuable, thing, data_type) {
            Ok(new) => new,
            Err(err) => {
                DeliveryImpl::<TableDelivery>::move_to_err(err.clone(), old);
                return Err(err);
            }
        };
        carrier.id = old.id; // the id is used for final finished
        Ok(carrier)
    }

    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>, thing: String, data_type: u8) -> Result<Vec<Carrier<T>>>
        where T: Sized + Serialize + Send, U: Sized + Serialize,
    {
        let mut rtn: Vec<Carrier<T>> = Vec::new();
        for v in valuables {
            let _ = match Carrier::new(v, thing.clone(), data_type) {
                Ok(new) => {
                    TableDelivery::insert(&new)?;
                    rtn.push(new);
                }
                Err(err) => {
                    DeliveryImpl::<TableDelivery>::move_to_err(err.clone(), old);
                    return Err(err);
                }
            };
        }
        TableDelivery::delete(&old.id)?;
        Ok(rtn)
    }

    fn finish_carrier(id: &u128) -> Result<()> {
        TableDelivery::delete(&id)
    }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize {
        let _ = TableDelivery::move_to_error(CarryError { err, carrier });
    }
}

pub type DeliveryService = DeliveryImpl<TableDelivery>;

lazy_static! {
    pub static ref TASK_DELIVERY : Arc<DeliveryService> = Arc::new(DeliveryImpl{table_delivery:PhantomData});
}