use std::rc::Rc;

use mockers::Scenario;
use mockers_derive::mock;

use nature_common::*;
use nature_db::*;
use crate::support::*;

use crate::flow::*;

mock! {
    RouteServiceTraitMock,
    self,
    trait RouteServiceTrait {
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
        fn get(thing: &Thing) -> Result<Option<RawThingDefine>>;
        fn insert(define: &RawThingDefine) -> Result<usize>;
        fn delete(thing: &Thing) -> Result<usize>;
    }
}

mock! {
    ThingDefineCacheTraitMock,
    self,
    trait ThingDefineCacheTrait {
        fn get(&self, thing: &Thing) -> Result<RawThingDefine>;
    }
}

pub struct MyRouteServiceTraitMock { pub m: RouteServiceTraitMock }

unsafe impl Sync for MyRouteServiceTraitMock {}

pub struct MyMocks {
    pub s: Scenario,
    pub s_thing_define_cache: Rc<ThingDefineCacheTraitMock>,
    pub d_thing_define: Rc<ThingDefineDaoTraitMock>,
    pub s_route: Rc<RouteServiceTraitMock>,
    pub s_task: Rc<TaskServiceTraitMock>,
    pub call_out: Rc<CallOutTraitMock>,
}

impl MyMocks {
    pub fn new() -> MyMocks {
        let s = Scenario::new();
        let s_thing_define_cache = Rc::new(s.create_mock::<ThingDefineCacheTraitMock>());
        let d_thing_define = Rc::new(s.create_mock::<ThingDefineDaoTraitMock>());
        let s_route = Rc::new(s.create_mock::<RouteServiceTraitMock>());
        let s_task = Rc::new(s.create_mock::<TaskServiceTraitMock>());
        let call_out = Rc::new(s.create_mock::<CallOutTraitMock>());
        MyMocks {
            s,
            s_thing_define_cache,
            d_thing_define,
            s_route,
            s_task,
            call_out,
        }
    }
}
