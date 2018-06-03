use db::trait_define::ThingDefineCacheTrait;
use db::trait_define::ThingDefineDao;
use std::ops::Deref;
use std::sync::Mutex;
use super::*;

lazy_static! {
    pub static ref CACHE_THING_DEFINE_LOCK:Mutex<u8> = Mutex::new(1);
    pub static ref CACHE_THING_DEFINE_VALUE:Mutex<Result<ThingDefine>> = Mutex::new(Err(NatureError::VerifyError("ThingDefineCache mock : not defined".to_string())));
}

pub struct MockTableThingDefine;

pub struct MockThingDefineCache;

impl ThingDefineCacheTrait for MockThingDefineCache {
    fn get(_thing: &Thing) -> Result<ThingDefine> {
        println!("---------------- ThingDefineCache mock get ----------------------");
        CACHE_THING_DEFINE_VALUE.lock().unwrap().deref().clone()
    }
}