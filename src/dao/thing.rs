use define::*;
use lru_time_cache::LruCache;
use std::time::Duration;

pub trait ThingDao {
    fn get(&mut self, key: &str) -> Result<&ThingExtended>;
}

pub struct ThingDaoService {
    cache: Option<LruCache<String, ThingExtended>>,
}

impl ThingDaoService {
    fn get_cache(&mut self) -> &mut LruCache<String, ThingExtended> {
//        if self.cache.is_none() {
//            self.cache = Some(LruCache::<String, ThingExtended>::with_expiry_duration(Duration::from_secs(3600)))
//        }
        // TODO config cache time
        self.cache.get_or_insert(LruCache::<String, ThingExtended>::with_expiry_duration(Duration::from_secs(3600)))
    }
}

impl ThingDao for ThingDaoService {
    fn get(&mut self, key: &str) -> Result<&ThingExtended> {
        let mut cache = self.get_cache();
        match cache.get(key) {
            None => {
                // TODO load from dao
                Err(NatureError::ThingNotDefined(format!("{} not defined", key)))
            }
            Some(x) => Ok(x)
        }
    }
}

pub static THING_DAO_SERVICE: ThingDaoService = ThingDaoService { cache: None };