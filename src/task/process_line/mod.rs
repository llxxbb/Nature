use dao::*;
use global::*;
use rpc::rocket::*;
pub use self::convert::*;
pub use self::delivery::*;
pub use self::dispatch::*;
pub use self::route::*;
pub use self::store::*;
pub use self::threads::*;
use serde::Serialize;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    /// born an instance which is the beginning of the changes.
    pub fn single_input(instance: Instance) -> Result<UuidBytes> {
        let task = StoreInfo { instance, converter: None };
        let carrier = create_carrier(task)?;
        Self::store(carrier, Root::Business)
    }

    fn route(carrier: Carrier<StoreInfo>) { do_route(carrier); }

    fn dispatch(carrier: Carrier<RouteInfo>) { do_dispatch(carrier); }

    fn convert(carrier: Carrier<ConverterInfo>) { do_convert(carrier); }

    fn store(carrier: Carrier<StoreInfo>, root: Root) -> Result<UuidBytes> { do_store(carrier, root) }

    fn store_for_receive(carrier: Carrier<StoreInfo>) {
        if let Err(err) = Self::store(carrier.clone(), Root::Business) {
            Self::move_to_err(err, carrier)
        };
    }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize {
        let _ = CarrierDaoService::move_to_error(CarryError { err, carrier });
    }

    pub fn callback(delayed: DelayedInstances) { do_callback(delayed) }
}

mod route;

mod dispatch;

mod convert;

mod store;

mod threads;

mod delivery;

#[cfg(test)]
mod test_store;

