use lru_time_cache::LruCache;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Mutex;
use std::time::Duration;
use super::*;

lazy_static! {
    pub static ref CACHE_MAPPING: Mutex<LruCache<Thing, (Vec<Mapping>, HashMap<Thing, Range<f32>>)>> = Mutex::new(LruCache::<Thing, (Vec<Mapping>, HashMap<Thing, Range<f32>>)>::with_expiry_duration(Duration::from_secs(3600)));

}
