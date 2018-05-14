use super::*;

#[test]
fn thing_key_is_empty() {
    println!("----------------- thing_key_is_empty --------------------");
    let thing = Thing::default();
    match ThingDefineCache::get(&thing) {
        Err(NatureError::VerifyError(err)) => assert_eq!("[biz] must not be empty!", err),
        _ => panic!("should get error")
    }
}

#[test]
fn not_exists_in_db() {
    println!("----------------- not_exists_in_db --------------------");
    let _ = THING_DEFINE_LOCK.lock().unwrap();
    let thing = Thing {
        key: "test/not/exists".to_string(),
        version: 200,
    };
    match ThingDefineCache::get(&thing) {
        Err(NatureError::ThingNotDefined(err)) => assert_eq!("test/not/exists not defined", err),
        x => panic!("should not get {:?}", x)
    }
}

#[test]
fn get_from_cache() {
    // to void parallel
    let _ = THING_DEFINE_LOCK.lock().unwrap();
    use chrono::prelude::*;
    let thing = Thing {
        key: "test".to_string(),
        version: 200,
    };
    let def = ThingDefine {
        key: "from_cache".to_string(),
        description: None,
        version: 100,
        states: None,
        fields: None,
        create_time: Local::now().naive_local(),
    };
    // insert to cache
    let mut cache = CACHE_THING_DEFINE.lock().unwrap();
    cache.insert(thing.clone(), def);
    drop(cache);
    // verify
    match ThingDefineCache::get(&thing) {
        Ok(def) => assert_eq!(def.key, "from_cache"),
        x => panic!("should not get {:?}", x)
    }
    // again
    match ThingDefineCache::get(&thing) {
        Ok(def) => assert_eq!(def.key, "from_cache"),
        x => panic!("should not get {:?}", x)
    }
}

#[test]
fn load_from_db() {
    println!("----------------- load_from_db --------------------");
    let _ = THING_DEFINE_LOCK.lock().unwrap();
    use chrono::prelude::*;
    let key = "/define/cache/load_from_db".to_string();
    let thing = Thing {
        key: key.clone(),
        version: 200,
    };
    let def = ThingDefine {
        key: key.clone(),
        description: None,
        version: 100,
        states: None,
        fields: None,
        create_time: Local::now().naive_local(),
    };
    // insert to db
    let mut rtn = THING_DEFINE_VALUE.lock().unwrap();
    *rtn = Ok(Some(def));
    drop(rtn);
    println!("----------------halted!!!!!!---------------");
    // verify
    match ThingDefineCache::get(&thing) {
        Ok(def) => assert_eq!(def.key, key),
        x => panic!("should not get {:?}", x)
    }
    println!("----------------halted two!!!!---------------");
    // again
    match ThingDefineCache::get(&thing) {
        Ok(def) => assert_eq!(def.key, key),
        x => panic!("should not get {:?}", x)
    }
    // clean
    let mut rtn = THING_DEFINE_VALUE.lock().unwrap();
    *rtn = Ok(None);
}
