///! World Connection Service provider
extern crate uuid;

use data::*;
use lru_time_cache::LruCache;
pub use self::error::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::product::*;
use std;
use std::sync::*;
use std::time::Duration;
use task::*;
use util::*;

pub type Result<T> = std::result::Result<T, NatureError>;

// for product and mock
lazy_static! {
    pub static ref CHANNEL_ROUTE : Channel<Carrier<StoreInfo>> = Channel::new();
    pub static ref CHANNEL_DISPATCH : Channel<Carrier<RouteInfo>> = Channel::new();
    pub static ref CHANNEL_CONVERT : Channel<Carrier<ConverterInfo>> = Channel::new();
    pub static ref CHANNEL_STORE : Channel<Carrier<StoreInfo>> = Channel::new();

    pub static ref THING_DEFINE_CACHE: Mutex<LruCache<Thing, ThingDefine>> = Mutex::new(LruCache::<Thing, ThingDefine>::with_expiry_duration(Duration::from_secs(3600)));

    pub static ref SYS_KEY_BATCH_SERIAL : String = "/sys/batch/serial".to_string();
    pub static ref SYS_KEY_BATCH_PARALLEL : String = "/sys/batch/parallel".to_string();
}

#[cfg(not(test))]
pub mod product;

#[cfg(test)]
pub mod mock;

pub mod error;

