extern crate r2d2;

use define::*;
use lru_time_cache::LruCache;
use std::time::Duration;
use thing::*;


pub trait ThingDefineDao {
    fn get(&mut self, thing: &Thing) -> Result<&ThingDefine>;
}

pub struct ThingDefineDaoService {
    cache: LruCache<Thing, ThingDefine>,
}

impl ThingDefineDaoService {
    pub fn new() -> ThingDefineDaoService {
        ThingDefineDaoService {
            // TODO config cache time
            cache: LruCache::<Thing, ThingDefine>::with_expiry_duration(Duration::from_secs(3600))
        }
    }
}

impl ThingDefineDao for ThingDefineDaoService {
    fn get(&mut self, thing: &Thing) -> Result<&ThingDefine> {
        if thing.key.is_empty() {
            return Err(NatureError::VerifyError("[biz] must not be empty!".to_string()));
        }
        match self.cache.get(thing) {
            None => {
                // TODO load from dao
                Err(NatureError::ThingNotDefined(format!("{} not defined", thing.key)))
            }
            Some(x) => Ok(x)
        }
    }
}

