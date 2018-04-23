use dao::*;
use rpc::rocket::*;
pub use self::convert::*;
pub use self::dispatch::*;
pub use self::route::*;
pub use self::threads::*;
use serde::Serialize;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    /// born an instance which is the beginning of the changes.
    pub fn single_input(instance: Instance) -> Result<UuidBytes> {
        let task = StoreTask(instance);
        let carrier = Carrier::new(task)?;
        let _ = CarrierDaoService::insert(&carrier)?;
        Self::store(carrier, Root::Business)
    }

    fn route(carrier: Carrier<StoreTask>) { do_route(carrier); }

    fn dispatch(carrier: Carrier<RouteInfo>) { do_dispatch(carrier); }

    fn convert(carrier: Carrier<ConverterTask>) { do_convert(carrier); }

    fn store(carrier: Carrier<StoreTask>, root: Root) -> Result<UuidBytes> {
        let mut carrier = carrier;
        let uuid = InstanceImpl::verify(&mut carrier.data.0, root)?;
        InstanceDaoService::insert(&carrier.data.0)?;
        send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
        Ok(uuid)
    }

    fn store_for_receive(carrier: Carrier<StoreTask>) {
        let cp = carrier.clone();
        if let Err(err) = Self::store(carrier, Root::Business) {
            Self::move_to_err(err, cp)
        };
    }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize {
        let _ = CarrierDaoService::move_to_error(CarryError { err, carrier });
    }
}

mod route;

mod dispatch;

mod convert;

mod threads;

#[cfg(test)]
mod test_store;
