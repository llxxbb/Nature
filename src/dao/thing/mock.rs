use global::*;
use super::*;


pub struct ThingDefineServiceImpl;

impl ThingDefineService for ThingDefineServiceImpl {
    fn get(thing: &Thing) -> Result<ThingDefine> {
        match thing.key.as_ref() {
            "/B/ok" => Ok(ThingDefine::default()),
            "/B/err" => Err(NatureError::VerifyError("not defined".to_string())),
            _ => Err(NatureError::VerifyError("unknown".to_string())),
        }
    }
    fn insert(_define: &ThingDefine) -> Result<()> {
        unimplemented!()
    }
}
