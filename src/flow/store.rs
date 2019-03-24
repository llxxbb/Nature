use std::rc::Rc;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreTaskInfo {
    pub instance: Instance,
    /// save outside has non converter info.
    pub upstream: Option<ConverterInfo>,
    pub mission: Option<Vec<Mission>>,
}

pub trait StoreServiceTrait {
    fn input(&self, instance: Instance) -> Result<u128>;
    fn self_route(&self, instance: SelfRouteInstance) -> Result<u128>;
    fn generate_store_task(&self, instance: &Instance) -> Result<StoreTaskInfo>;
    fn do_task(&self, task: &StoreTaskInfo, carrier: &RawTask) -> Result<()>;
}

pub struct StoreServiceImpl {
    pub instance_dao: Rc<InstanceDaoTrait>,
    pub route: Rc<RouteServiceTrait>,
    pub task_svc: Rc<TaskServiceTrait>,
    pub task_dao: Rc<TaskDaoTrait>,
    pub svc_instance: Rc<InstanceServiceTrait>,
}

impl StoreServiceTrait for StoreServiceImpl {
    fn input(&self, mut instance: Instance) -> Result<u128> {
        instance.data.thing.thing_type = ThingType::Business;
        let uuid = self.svc_instance.verify(&mut instance)?;
        let task = self.generate_store_task(&instance)?;
        let carrier = RawTask::new(&task, &instance.thing.key, TaskType::Store as i16)?;
        self.task_dao.insert(&carrier)?;
        self.do_task(&task, &carrier)?;
        Ok(uuid)
    }

    fn self_route(&self, instance: SelfRouteInstance) -> Result<u128> {
        if instance.converter.is_empty() {
            return Err(NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
        }
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = Instance {
            id: 0,
            data: instance.instance.data,
        };
        ins.data.thing.thing_type = ThingType::Dynamic;
        let uuid = self.svc_instance.id_generate_if_not_set(&mut ins)?;
        let task = self.generate_self_route_task(&ins, instance.converter)?;
        let carrier = RawTask::new(&task, &ins.thing.key, TaskType::Store as i16)?;
        self.do_task(&task, &carrier)?;
        Ok(uuid)
    }


    /// generate `StoreTaskInfo` include route information.
    /// `Err` on environment error
    fn generate_store_task(&self, instance: &Instance) -> Result<StoreTaskInfo> {
        let target = self.route.get_mission(instance)?;
        // save to task to make it can redo
        let task = StoreTaskInfo {
            instance: instance.clone(),
            upstream: None,
            mission: target,
        };
        Ok(task)
    }

    fn do_task(&self, task: &StoreTaskInfo, carrier: &RawTask) -> Result<()> {
        debug!("------------------do_store_task------------------------");
        if let Err(err) = self.save(&task.instance) {
            let _ = self.task_dao.raw_to_error(&err, carrier);
            Err(err)
        } else {
            let _ = CHANNEL_STORED.sender.lock().unwrap().send((task.to_owned(), carrier.to_owned()));
            Ok(())
        }
    }
}

impl StoreServiceImpl {
    /// save to db and handle duplicated data
    fn save(&self, instance: &Instance) -> Result<usize> {
        debug!("save instance for `Thing` {:?}, id : {:?}", instance.thing.key, instance.id);
        let result = self.instance_dao.insert(instance);
        match result {
            Ok(num) => Ok(num),
            Err(err) => match err {
                NatureError::DaoDuplicated(_) => Ok(0),
                _ => Err(err)
            }
        }
    }

    /// generate `StoreTaskInfo` include route information.
/// `Err` on environment error
    fn generate_self_route_task(&self, instance: &Instance, dynamic: Vec<DynamicConverter>) -> Result<StoreTaskInfo> {
        let target = self.route.get_dynamic_mission(dynamic)?;
        // save to task to make it can redo
        let task = StoreTaskInfo {
            instance: instance.clone(),
            upstream: None,
            mission: Some(target),
        };
        Ok(task)
    }
}

#[cfg(test)]
mod test {
    use mockers::matchers::{ANY, check};

    use test_util::*;

    use super::*;

    #[test]
    fn input_thing_type_must_be_business() {
        // prepare
        let mocks = MyMocks::new();
        let store_svc = init_store_svc(&mocks);
        // expect
        mocks.s.expect(mocks.s_instance.verify_call(check(|x: &&mut Instance| x.thing.thing_type == ThingType::Business))
            .and_return(Err(NatureError::VerifyError("deliberate".to_string()))));
        let mut instance = Instance::default();
        instance.data.thing.thing_type = ThingType::Dynamic;
        // run
        let _rtn = store_svc.input(instance);
    }

    #[test]
    fn thing_must_be_defined() {
        // prepare
        let mocks = MyMocks::new();
        let store_svc = init_store_svc(&mocks);
        // expect
        mocks.s.expect(mocks.s_instance.verify_call(ANY)
            .and_return(Err(NatureError::VerifyError("Thing must be defined".to_string()))));
        let mut instance = Instance::default();
        instance.data.thing.thing_type = ThingType::Dynamic;
        // run
        let rtn = store_svc.input(instance);
        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("Thing must be defined".to_string()))
    }

    #[test]
    fn generate_store_tasks_for_error() {
        // prepare
        let mocks = MyMocks::new();
        let store_svc = init_store_svc(&mocks);
        // expect
        let mut instance = Instance::default();
        instance.data.thing.key = "123".to_string();
        mocks.s.expect(mocks.s_instance.verify_call(ANY)
            .and_return(generate_id(&instance)));
        mocks.s.expect(mocks.s_route.get_mission_call(check(|x: &&Instance| x.thing.key == "123"))
            .and_return(Err(NatureError::VerifyError("get task error".to_string()))));
        // run
        let rtn = store_svc.input(instance);
        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("get task error".to_string()))
    }

    #[test]
    fn insert_task_error() {
        // prepare
        let mocks = MyMocks::new();
        let store_svc = init_store_svc(&mocks);
        // expect
        let mut instance = Instance::default();
        instance.data.thing.key = "123".to_string();
        mocks.s.expect(mocks.s_instance.verify_call(ANY)
            .and_return(generate_id(&instance)));
        mocks.s.expect(mocks.s_route.get_mission_call(ANY)
            .and_return(Ok(Some(vec![Mission::default()]))));
        mocks.s.expect(mocks.d_task.insert_call(check(|x: &&RawTask| x.thing == "123"))
            .and_return(Err(NatureError::VerifyError("insert task error".to_string()))));
        // run
        let rtn = store_svc.input(instance);
        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("insert task error".to_string()))
    }

    #[test]
    fn insert_instance_error() {
        // prepare
        let mocks = MyMocks::new();
        let store_svc = init_store_svc(&mocks);
        // expect
        let mut instance = Instance::default();
        instance.data.thing.key = "123".to_string();
        mocks.s.expect(mocks.s_instance.verify_call(ANY)
            .and_return(generate_id(&instance)));
        mocks.s.expect(mocks.s_route.get_mission_call(ANY)
            .and_return(Ok(Some(vec![Mission::default()]))));
        mocks.s.expect(mocks.d_task.insert_call(ANY)
            .and_return(Ok(1)));
        mocks.s.expect(mocks.d_instance.insert_call(check(|x: &&Instance| x.thing.key == "123"))
            .and_return(Err(NatureError::VerifyError("insert instance error".to_string()))));
        mocks.s.expect(mocks.d_task.raw_to_error_call(ANY, check(|x: &&RawTask| x.thing == "123"))
            .and_return(Ok(1)));
        // run
        let rtn = store_svc.input(instance);
        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("insert instance error".to_string()))
    }

    #[test]
    fn insert_instance_ok() {
        // prepare
        let mocks = MyMocks::new();
        let store_svc = init_store_svc(&mocks);
        // expect
        let mut instance = Instance::default();
        instance.data.thing.key = "123".to_string();
        mocks.s.expect(mocks.s_instance.verify_call(ANY)
            .and_return(generate_id(&instance)));
        mocks.s.expect(mocks.s_route.get_mission_call(ANY)
            .and_return(Ok(Some(vec![Mission::default()]))));
        mocks.s.expect(mocks.d_task.insert_call(ANY)
            .and_return(Ok(1)));
        mocks.s.expect(mocks.d_instance.insert_call(check(|x: &&Instance| x.thing.key == "123"))
            .and_return(Ok(1)));
        // run
        let rtn = store_svc.input(instance);
        assert_eq!(rtn.unwrap(), 157623018616273791007256710966552945966)
    }

    fn init_store_svc(mockers: &MyMocks) -> StoreServiceImpl {
        StoreServiceImpl {
            instance_dao: mockers.d_instance.clone(),
            route: mockers.s_route.clone(),
            task_svc: mockers.s_task.clone(),
            task_dao: mockers.d_task.clone(),
            svc_instance: mockers.s_instance.clone(),
        }
    }
}