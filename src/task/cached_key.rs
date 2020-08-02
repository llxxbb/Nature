use std::sync::Mutex;
use std::time::Duration;

use lru_time_cache::LruCache;

use crate::system::CACHE_SAVED_TIME;

lazy_static! {
    static ref CACHE: Mutex<LruCache<String, u32>> = Mutex::new(LruCache::<String, u32>::with_expiry_duration(Duration::from_secs(*CACHE_SAVED_TIME)));
}

/// used to avoid save conflict
pub struct CachedKey;

impl CachedKey {
    pub fn get(key: &str) -> bool {
        let mut c = CACHE.lock().unwrap();
        match c.get(key) {
            Some(_) => {
                debug!("cached key: {}", key);
                true
            },
            None => false
        }
    }

    pub fn set(key: &str) {
        let mut c = CACHE.lock().unwrap();
        c.insert(key.to_string(), 1);
    }
}

#[cfg(test)]
mod test {
    use crate::task::cached_key::CachedKey;

    #[test]
    fn get_and_set_test() {
        assert_eq!(CachedKey::get("hello"), false);
        CachedKey::set("hello");
        assert_eq!(CachedKey::get("hello"), true);
    }
}