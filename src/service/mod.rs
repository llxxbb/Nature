///! World Connection Service provider
extern crate uuid;

use carrier::*;
use dao::*;
use diesel::sqlite::SqliteConnection;
use processor::*;
#[cfg(not(test))]
use r2d2::Pool;
#[cfg(not(test))]
use r2d2_diesel::ConnectionManager;
#[cfg(test)]
pub use self::test::*;
use std::sync::*;
use task::*;

pub type CONN = SqliteConnection;

#[cfg(not(test))]
lazy_static! {
        pub static ref  POOL :Pool<ConnectionManager<CONN>> = create_pool::<CONN>();
        pub static ref  DEFINE_DAO : Mutex<ThingDefineDaoService>  =  Mutex::new(ThingDefineDaoService::new());
        pub static ref  INSTANCE_DAO : InstanceDaoService  =  InstanceDaoService{};
        pub static ref  PROCESSOR_ROUTE : Processor<Carrier<StoreTask>>  =  Processor::new();
    }


#[cfg(test)]
pub mod test;