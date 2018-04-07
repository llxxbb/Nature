use chrono::prelude::*;
use define::*;
use super::*;
use thing::*;

lazy_static! {
//        pub static ref  POOL :Pool<ConnectionManager<CONN>> = create_pool::<CONN>();
        pub static ref  DEFINE_DAO : Mutex<ThingDefineDaoMock>  =  Mutex::new(ThingDefineDaoMock::new());
//        pub static ref  CARRIER_DAO : Mutex<CarrierDaoService>  =  Mutex::new(CarrierDaoService(Mode::Ok));

//        pub static ref  INSTANCE_DAO : InstanceDaoService  =  InstanceDaoService{};
        pub static ref  PROCESSOR_ROUTE : Processor<Carrier<StoreTask>>  =  Processor::new();
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

//pub struct InstanceDaoMock;
//
//impl InstanceDao for InstanceDaoMock {
//    fn insert(&self, _instance: Instance) {
//        unimplemented!()
//    }
//}