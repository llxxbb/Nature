extern crate r2d2;

use define::*;
use lru_time_cache::LruCache;
use self::r2d2::{ManageConnection, Pool};
use std::time::Duration;

pub trait ThingDao<CM: ManageConnection> {
    fn get(&mut self, thing: &Thing, pool: Pool<CM>) -> Result<&ThingDefine>;
}

pub struct ThingDaoService {
    cache: LruCache<Thing, ThingDefine>,
}

impl ThingDaoService {
    fn new() -> ThingDaoService {
        ThingDaoService {
            // TODO config cache time
            cache: LruCache::<Thing, ThingDefine>::with_expiry_duration(Duration::from_secs(3600))
        }
    }
}

impl<CM: ManageConnection> ThingDao<CM> for ThingDaoService {
    fn get(&mut self, thing: &Thing, pool: Pool<CM>) -> Result<&ThingDefine> {
        match self.cache.get(thing) {
            None => {
                // TODO load from dao
                Err(NatureError::ThingNotDefined(format!("{} not defined", thing.key)))
            }
            Some(x) => Ok(x)
        }
    }
}

