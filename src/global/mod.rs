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
use std::collections::HashMap;
use std::ops::Range;
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
    pub static ref CHANNEL_PARALLEL : Channel<Carrier<ParallelBatchInstance>> = Channel::new();
    pub static ref CHANNEL_SERIAL : Channel<Carrier<SerialBatchInstance>> = Channel::new();

    pub static ref CACHE_MAPPING: Mutex<LruCache<Thing, (Vec<Mapping>, HashMap<Thing, Range<f32>>)>> = Mutex::new(LruCache::<Thing, (Vec<Mapping>, HashMap<Thing, Range<f32>>)>::with_expiry_duration(Duration::from_secs(3600)));

    // sys biz define
    pub static ref SYS_KEY_SERIAL : String = "/serial".to_string();

    // sys context define
    pub static ref CONTEXT_TARGET_INSTANCE_ID : String = "sys.target_instance_id".to_string();

}

#[cfg(not(test))]
pub mod product;

#[cfg(test)]
pub mod mock;

pub mod error;

