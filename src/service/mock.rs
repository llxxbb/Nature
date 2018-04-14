use chrono::prelude::*;
use define::*;
use super::*;
use data::thing::*;

lazy_static! {
        pub static ref  DEFINE_DAO : Mutex<ThingDefineDaoMock>  =  Mutex::new(ThingDefineDaoMock::new());
    }


pub struct ThingDefineDaoMock(ThingDefine);

impl ThingDefineDaoMock {
    pub fn new() -> ThingDefineDaoMock {
        ThingDefineDaoMock(
            ThingDefine {
                key: String::new(),
                description: String::new(),
                version: 0,
                have_states: false,
                states: None,
                fields: None,
                create_time: Local::now(),
            }
        )
    }
}

impl ThingDefineDao for ThingDefineDaoMock {
    fn get(&mut self, thing: &Thing) -> Result<&ThingDefine> {
        match thing.key.as_ref() {
            "ok" => Ok(&self.0),
            "err" => Err(NatureError::VerifyError("not defined".to_string())),
            _ => Err(NatureError::VerifyError("unknown".to_string())),
        }
    }
}
