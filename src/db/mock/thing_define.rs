use std::ops::Deref;
use std::sync::Mutex;
use super::*;

lazy_static! {
    pub static ref THING_DEFINE_LOCK:Mutex<u8> = Mutex::new(1);
    pub static ref THING_DEFINE_GET_VALUE:Mutex<Result<Option<ThingDefine>>> = Mutex::new(Ok(None));
}

pub struct TableThingDefine;

impl TableThingDefine {
    pub fn get(_thing: &Thing) -> Result<Option<ThingDefine>> {
        println!("---------------- TableThingDefine mock get ----------------------");
        THING_DEFINE_GET_VALUE.lock().unwrap().deref().clone()
    }

    pub fn insert(_define: &ThingDefine) -> Result<()> {
        println!("---------------- TableThingDefine mock insert ----------------------");
        Ok(())
    }

    pub fn delete(_thing: &Thing) -> Result<()> {
        println!("---------------- TableThingDefine mock delete ----------------------");
        Ok(())
    }
}

pub struct ThingDefineCache;

impl ThingDefineDao for ThingDefineCache {
    fn get(thing: &Thing) -> Result<ThingDefine> {
        match thing.key.as_ref() {
            "/B/ok" => Ok(ThingDefine::default()),
            "/B/err" => Err(NatureError::VerifyError("ThingDefineCache mock : not defined".to_string())),
            _ => Err(NatureError::VerifyError("ThingDefineCache mock : unknown".to_string())),
        }
    }
    fn insert(_define: &ThingDefine) -> Result<()> {
        unimplemented!()
    }
}