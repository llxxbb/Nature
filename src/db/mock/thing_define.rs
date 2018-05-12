use super::*;


pub struct TableThingDefine;

impl ThingDefineDao for TableThingDefine {
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

pub struct ThingDefineCache;

impl ThingDefineDao for ThingDefineCache {
    fn get(_thing: &Thing) -> Result<ThingDefine> {
        unimplemented!()
    }

    fn insert(_define: &ThingDefine) -> Result<()> {
        unimplemented!()
    }
}