extern crate r2d2;

use super::*;

pub struct ThingDefineDaoService;

impl ThingDefineDao for ThingDefineDaoService {
    fn get(thing: &Thing) -> Result<ThingDefine> {
        if thing.key.is_empty() {
            return Err(NatureError::VerifyError("[biz] must not be empty!".to_string()));
        }
        let mut cache = THING_DEFINE_CACHE.lock().unwrap();
        let rtn = cache.get(thing);
        match rtn {
            None => {
                // TODO load from dao
                Err(NatureError::ThingNotDefined(format!("{} not defined", thing.key)))
            }
            Some(x) => Ok(x.clone())
        }
    }
}
