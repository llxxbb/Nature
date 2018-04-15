use global::*;
use super::*;


pub struct ThingDefineDaoService;

impl ThingDefineDao for ThingDefineDaoService {
    fn get(thing: &Thing) -> Result<ThingDefine> {
        match thing.key.as_ref() {
            "/B/ok" => Ok(ThingDefine::default()),
            "/B/err" => Err(NatureError::VerifyError("not defined".to_string())),
            _ => Err(NatureError::VerifyError("unknown".to_string())),
        }
    }
}
