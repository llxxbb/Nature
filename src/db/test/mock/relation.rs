//use std::sync::Mutex;
//use super::*;
//
//pub struct Relation;
//
//lazy_static! {
//    pub static ref RELATION_LOCK:Mutex<bool> = Mutex::new(true);
//    pub static ref RELATION_VALUE:Mutex<Result<Vec<Mapping>>> = Mutex::new(Ok(Vec::new()));
//}
//
//impl Relation {
//    pub fn get(_from: &Thing) -> Result<Vec<Mapping>> {
//        RELATION_VALUE.lock().unwrap().clone()
//    }
//}
