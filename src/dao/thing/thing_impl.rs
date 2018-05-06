extern crate r2d2;

use db::DBPool;
use db::thing_defines::dsl::*;
use diesel::prelude::*;
use lru_time_cache::LruCache;
use std::ops::Deref;
use std::sync::Mutex;
use std::time::Duration;
use super::*;

lazy_static! {
    pub static ref CACHE_THING_DEFINE: Mutex<LruCache<Thing, ThingDefine>> = Mutex::new(LruCache::<Thing, ThingDefine>::with_expiry_duration(Duration::from_secs(3600)));
}

pub struct ThingDefineDaoService;

impl ThingDefineDao for ThingDefineDaoService {
    fn get(thing: &Thing) -> Result<ThingDefine> {
        if thing.key.is_empty() {
            return Err(NatureError::VerifyError("[biz] must not be empty!".to_string()));
        }
        let mut cache = CACHE_THING_DEFINE.lock().unwrap();
        let rtn = cache.get(thing);
        if let Some(x) = rtn {
            return Ok(x.clone());
        }
        drop(rtn);
        drop(cache);
        let conn = DBPool::get_connection()?;
        let def = thing_defines.filter(key.eq(&thing.key))
            .filter(version.eq(thing.version))
            .load::<ThingDefine>(conn.deref())?;
        if def.len() == 0 {
            return Err(NatureError::ThingNotDefined(format!("{} not defined", thing.key)));
        }
        let mut cache = CACHE_THING_DEFINE.lock().unwrap();
        cache.insert(thing.clone(), def[0].clone());
        Ok(def[0].clone())
    }
}
