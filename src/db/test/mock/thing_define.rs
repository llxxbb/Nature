//use db::dao_impl::ThingDefineDao;
//use std::ops::Deref;
//use std::sync::Mutex;
//use super::*;
//
//lazy_static! {
//    pub static ref THING_DEFINE_LOCK:Mutex<u8> = Mutex::new(1);
//    pub static ref THING_DEFINE_GET_VALUE:Mutex<Result<Option<ThingDefine>>> = Mutex::new(Ok(None));
//    pub static ref THING_DEFINE_CACHE_VALUE:Mutex<Result<ThingDefine>> = Mutex::new(Err(NatureError::VerifyError("ThingDefineCache mock : not defined".to_string())));
//}
//
//pub struct TableThingDefine;
//
//impl TableThingDefine {
//    pub fn get(_thing: &Thing) -> Result<Option<ThingDefine>> {
//        println!("---------------- TableThingDefine mock get ----------------------");
//        THING_DEFINE_GET_VALUE.lock().unwrap().deref().clone()
//    }
//
//    pub fn insert(_define: &ThingDefine) -> Result<()> {
//        println!("---------------- TableThingDefine mock insert ----------------------");
//        Ok(())
//    }
//
//    pub fn delete(_thing: &Thing) -> Result<()> {
//        println!("---------------- TableThingDefine mock delete ----------------------");
//        Ok(())
//    }
//}
//
//pub struct ThingDefineCacheMock;
//
//impl ThingDefineDao for ThingDefineCacheMock {
//    fn get(_thing: &Thing) -> Result<ThingDefine> {
//        println!("---------------- ThingDefineCache mock get ----------------------");
//        THING_DEFINE_CACHE_VALUE.lock().unwrap().deref().clone()
//    }
//    fn insert(_define: &ThingDefine) -> Result<()> {
//        unimplemented!()
//    }
//}