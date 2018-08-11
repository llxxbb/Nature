extern crate r2d2;

use db::*;
use lru_time_cache::LruCache;
use std::sync::Mutex;
use std::time::Duration;
use super::*;

lazy_static! {
    static ref CACHE: Mutex<LruCache<Thing, ThingDefine>> = Mutex::new(LruCache::<Thing, ThingDefine>::with_expiry_duration(Duration::from_secs(3600)));
}

pub struct ThingDefineCacheImpl;

impl ThingDefineCacheTrait for ThingDefineCacheImpl {
    fn get(thing: &Thing) -> Result<ThingDefine> {
        debug!("get `ThingDefine` from cache for thing : {:?}", thing);
        if thing.key.is_empty() {
            return Err(Box::new(NatureError::VerifyError("[biz] must not be empty!".to_string())));
        }
        let mut cache = CACHE.lock().unwrap();
        {   // An explicit scope to avoid cache.insert error
            if let Some(x) = cache.get(thing) {
                return Ok(x.clone());
            };
        };
        match ThingDefineDaoImpl::get(&thing)? {
            None => return Err(Box::new(NatureError::ThingNotDefined(format!("{} not defined", thing.key)))),
            Some(def) => {
                cache.insert(thing.clone(), def.clone());
                Ok(def)
            }
        }
    }
}

