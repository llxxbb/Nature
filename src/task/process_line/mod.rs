pub use self::convert::*;
pub use self::delivery::*;
pub use self::dispatch::*;
pub use self::parallel::*;
pub use self::route::*;
pub use self::serial::*;
pub use self::store::*;
pub use self::threads::*;
use serde::Serialize;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    fn route(carrier: Carrier<StoreInfo>) { do_route(carrier); }

    fn dispatch(carrier: Carrier<RouteInfo>) { do_dispatch(carrier); }

    fn convert(carrier: Carrier<ConverterInfo>) { do_convert(carrier); }

    pub fn store(carrier: Carrier<StoreInfo>, root: Root) -> Result<UuidBytes> { do_store(carrier, root) }

    fn store_for_receive(carrier: Carrier<StoreInfo>) {
        if let Err(err) = Self::store(carrier.clone(), Root::Business) {
            Self::move_to_err(err, carrier)
        };
    }

    fn parallel(carrier: Carrier<ParallelBatchInstance>) { do_parallel(carrier) }
    fn serial(carrier: Carrier<SerialBatchInstance>) { do_serial(carrier) }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize {
        let _ = CarrierDaoService::move_to_error(CarryError { err, carrier });
    }
}

mod parallel;

mod serial;

mod route;

mod dispatch;

mod convert;

mod store;

mod threads;

mod delivery;
