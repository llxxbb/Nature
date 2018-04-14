///! World Connection Service provider
extern crate uuid;

use data::dao::*;
use data::instance::*;
use diesel::sqlite::SqliteConnection;
#[cfg(not(test))]
use r2d2::Pool;
#[cfg(not(test))]
use r2d2_diesel::ConnectionManager;
#[cfg(test)]
pub use self::mock::*;
use std::sync::*;
use util::*;

pub type CONN = SqliteConnection;

lazy_static! {
    pub static ref CHANNEL_ROUTE : Channel<Instance>  =  Channel::new();
    pub static ref SYS_KEY_BATCH_SERIAL : String = "/sys/batch/serial".to_string();
    pub static ref SYS_KEY_BATCH_PARALLEL : String = "/sys/batch/parallel".to_string();

}


#[cfg(not(test))]
lazy_static! {
        pub static ref  POOL :Pool<ConnectionManager<CONN>> = create_pool::<CONN>();
        pub static ref  DEFINE_DAO : Mutex<ThingDefineDaoService>  =  Mutex::new(ThingDefineDaoService::new());
        pub static ref  INSTANCE_DAO : InstanceDaoService  =  InstanceDaoService{};
    }


#[cfg(test)]
pub mod mock;