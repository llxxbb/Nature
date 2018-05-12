extern crate r2d2;

use db::*;
use lru_time_cache::LruCache;
use std::sync::Mutex;
use std::time::Duration;
use super::*;

lazy_static! {
    pub static ref CACHE_THING_DEFINE: Mutex<LruCache<Thing, ThingDefine>> = Mutex::new(LruCache::<Thing, ThingDefine>::with_expiry_duration(Duration::from_secs(3600)));
}

pub struct ThingDefineCache;

impl ThingDefineDao for ThingDefineCache {
    fn get(thing: &Thing) -> Result<ThingDefine> {
        if thing.key.is_empty() {
            return Err(NatureError::VerifyError("[biz] must not be empty!".to_string()));
        }
        let mut cache = CACHE_THING_DEFINE.lock().unwrap();
        let rtn = cache.get(thing);
        if let Some(x) = rtn {
            return Ok(x.clone());
        }
        match TableThingDefine::get(&thing)? {
            None => return Err(NatureError::ThingNotDefined(format!("{} not defined", thing.key))),
            Some(def) => {
                let mut cache = CACHE_THING_DEFINE.lock().unwrap();
                cache.insert(thing.clone(), def.clone());
                Ok(def)
            }
        }
    }
    fn insert(_define: &ThingDefine) -> Result<()> {
        unimplemented!()
    }
}
