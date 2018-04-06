///! World Connection Service provider
extern crate uuid;

use carrier::*;
use dao::*;
use diesel::sqlite::SqliteConnection;
use processor::*;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::sync::*;
use task::*;

pub type CONN = SqliteConnection;

lazy_static! {
        pub static ref  POOL :Pool<ConnectionManager<CONN>> = create_pool::<CONN>();
        pub static ref  DEFINE_DAO : Mutex<ThingDefineDaoService>  =  Mutex::new(ThingDefineDaoService::new());
        pub static ref  INSTANCE_DAO : InstanceDaoService  =  InstanceDaoService{};
        pub static ref  PROCESSOR_ROUTE : Processor<Carrier<StoreTask>>  =  Processor::new();
    }

