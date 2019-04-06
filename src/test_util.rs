use std::rc::Rc;

use mockers::Scenario;
use mockers_derive::mock;

use nature_common::*;
use nature_db::*;
use crate::support::*;

use crate::flow::*;

mock! {
    InstanceDaoTraitMock,
    nature_db,
    trait InstanceDaoTrait{
        fn insert(&self, instance: &Instance) -> Result<usize>;
        /// check whether source stored earlier
        fn is_exists(&self, instance: &Instance) -> Result<bool>;
        fn get_by_id(&self, id: u128) -> Result<Option<Instance>>;
        fn get_by_key(&self, key: &str, limit: i64) -> Result<Option<Vec<Instance>>>;
        fn get_by_full_key(&self, key: &str, limit: i64) -> Result<Option<Vec<Instance>>>;
    }
}
mock! {
    TaskDaoTraitMock,
    nature_db,
    trait TaskDaoTrait{
        fn insert(&self, raw: &RawTask) -> Result<usize>;
        fn delete(&self, record_id: &[u8]) -> Result<usize>;
        fn raw_to_error(&self, err: &NatureError, raw: &RawTask) -> Result<usize>;
        fn update_execute_time(&self, record_id: &[u8], delay: i64) -> Result<()>;
        fn increase_times_and_delay(&self, record_id: &[u8], delay: i32) -> Result<usize>;
        fn get(&self, record_id: &[u8]) -> Result<Option<RawTask>>;
        fn get_overdue(&self, seconds: &str) -> Result<Vec<RawTask>>;
    }
}

mock! {
    InstanceServiceTraitMock,
    nature_db,
    trait InstanceServiceTrait {
        fn verify(&self, instance: &mut Instance) -> Result<u128>;
        /// gegerate by Hash.
        fn id_generate_if_not_set(&self, instance: &mut Instance) -> Result<u128>;
    }
}

mock! {
    RouteServiceTraitMock,
    self,
    trait RouteServiceTrait {
        fn get_mission(&self, instance: &Instance) -> Result<Option<Vec<Mission>>>;
        fn get_dynamic_mission(&self, dynamic: Vec<DynamicConverter>) -> Result<Vec<Mission>>;
    }
}

mock! {
    TaskServiceTraitMock,
    self,
    trait TaskServiceTrait {
        fn create_and_finish_carrier(&self, old: &RawTask, new: &mut RawTask) -> Result<usize>;
        fn create_batch_and_finish_carrier(&self, news: &[RawTask], old_id: &[u8]) -> Result<()>;
    }
}

mock! {
    CallOutTraitMock,
    self,
    trait CallOutTrait {
        fn convert(&self, mission: &Mission, para: &CallOutParameter) -> Result<ConverterReturned>;
    }
}

mock! {
    ThingDefineDaoTraitMock,
    self,
    trait ThingDefineDaoTrait {
        fn get(thing: &Thing) -> Result<Option<ThingDefine>>;
        fn insert(define: &ThingDefine) -> Result<usize>;
        fn delete(thing: &Thing) -> Result<usize>;
    }
}

mock! {
    ThingDefineCacheTraitMock,
    self,
    trait ThingDefineCacheTrait {
        fn get(&self, thing: &Thing) -> Result<ThingDefine>;
    }
}

pub struct MyRouteServiceTraitMock { pub m: RouteServiceTraitMock }

unsafe impl Sync for MyRouteServiceTraitMock {}

pub struct MyMocks {
    pub s: Scenario,
    pub s_thing_define_cache: Rc<ThingDefineCacheTraitMock>,
    pub d_thing_define: Rc<ThingDefineDaoTraitMock>,
    pub s_instance: Rc<InstanceServiceTraitMock>,
    pub d_instance: Rc<InstanceDaoTraitMock>,
    pub s_route: Rc<RouteServiceTraitMock>,
    pub s_task: Rc<TaskServiceTraitMock>,
    pub d_task: Rc<TaskDaoTraitMock>,
    pub call_out: Rc<CallOutTraitMock>,
}

impl MyMocks {
    pub fn new() -> MyMocks {
        let s = Scenario::new();
        let s_thing_define_cache = Rc::new(s.create_mock::<ThingDefineCacheTraitMock>());
        let d_thing_define = Rc::new(s.create_mock::<ThingDefineDaoTraitMock>());
        let s_instance = Rc::new(s.create_mock::<InstanceServiceTraitMock>());
        let d_instance = Rc::new(s.create_mock::<InstanceDaoTraitMock>());
        let s_route = Rc::new(s.create_mock::<RouteServiceTraitMock>());
        let s_task = Rc::new(s.create_mock::<TaskServiceTraitMock>());
        let d_task = Rc::new(s.create_mock::<TaskDaoTraitMock>());
        let call_out = Rc::new(s.create_mock::<CallOutTraitMock>());
        MyMocks {
            s,
            s_thing_define_cache,
            d_thing_define,
            s_instance,
            d_instance,
            s_route,
            s_task,
            d_task,
            call_out,
        }
    }
}
