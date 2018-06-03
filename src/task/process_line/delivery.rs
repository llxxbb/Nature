use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread;
use super::*;

pub trait DeliveryTrait {
    fn create_carrier<T>(valuable: T) -> Result<Carrier<T>> where T: Sized + Serialize;
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>) -> Result<Carrier<T>> where T: Sized + Serialize, U: Sized + Serialize;
    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize, U: Sized + Serialize;
    fn finish_carrier(id: &UuidBytes) -> Result<()>;
    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize;
}

pub fn send_carrier<T>(sender: Sender<Carrier<T>>, carrier: Carrier<T>)
    where T: 'static + Sized + Serialize + Sync + Send {
    thread::spawn(move || {
        sender.send(carrier).unwrap();
    });
}

pub fn start_thread<T, F>(receiver: &'static Mutex<Receiver<Carrier<T>>>, f: F)
    where
        T: Serialize + Send,
        F: 'static + Fn(Carrier<T>) + Send
{
    thread::spawn(move || {
        let receiver = receiver.lock().unwrap();
        let mut iter = receiver.iter();
        while let Some(next) = iter.next() {
            f(next);
        }
    });
}

pub struct DeliveryImpl;

impl DeliveryTrait for DeliveryImpl {
    fn create_carrier<T>(valuable: T) -> Result<Carrier<T>>
        where T: Sized + Serialize
    {
        let carrier = Carrier::new(valuable)?;
        let _ = CarrierDaoService::insert(&carrier)?;
        Ok(carrier)
    }

    /// by performance reason, for one-to-one carry we can reuse the beginning carry to finish all flows.
    /// That way we need not to communicate with DB for create new and delete old carrier.
    /// But for failure we must redo from beginning. but I think it has small chance.
    /// Another disadvantage is the failure information will be attached to the beginning.
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>) -> Result<Carrier<T>>
        where T: Sized + Serialize, U: Sized + Serialize,
    {
        let mut carrier = match Carrier::new(valuable) {
            Ok(new) => new,
            Err(err) => {
                DeliveryImpl::move_to_err(err.clone(), old);
                return Err(err);
            }
        };
        carrier.id = old.id; // the id is used for final finished
        Ok(carrier)
    }

    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>) -> Result<Vec<Carrier<T>>>
        where T: Sized + Serialize, U: Sized + Serialize,
    {
        let mut rtn: Vec<Carrier<T>> = Vec::new();
        for v in valuables {
            let _ = match Carrier::new(v) {
                Ok(new) => {
                    CarrierDaoService::insert(&new)?;
                    rtn.push(new);
                }
                Err(err) => {
                    DeliveryImpl::move_to_err(err.clone(), old);
                    return Err(err);
                }
            };
        }
        CarrierDaoService::delete(&old.id)?;
        Ok(rtn)
    }

    fn finish_carrier(id: &UuidBytes) -> Result<()> {
        CarrierDaoService::delete(&id)
    }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize {
        let _ = CarrierDaoService::move_to_error(CarryError { err, carrier });
    }
}

lazy_static! {
    pub static ref TASK_DELIVERY : Arc<DeliveryImpl> = Arc::new(DeliveryImpl);
}