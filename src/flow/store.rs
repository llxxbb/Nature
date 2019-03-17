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
    use mockers::Scenario;
    use mockers_derive::mock;

    use super::*;

    mock! {
        InstanceDaoTraitMock,
        test,
        trait InstanceDaoTrait{
            fn insert(&self, instance: &Instance) -> Result<usize>;
            /// check whether source stored earlier
            fn is_exists(&self, instance: &Instance) -> Result<bool>;
            fn get_by_id(&self, id: u128) -> Result<Option<Instance>>;
            fn get_by_key(&self, key: &str, id: u128) -> Result<Option<Instance>>;
        }
    }
    mock! {
        TaskDaoTraitMock,
        test,
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
        test,
        trait InstanceServiceTrait {
            fn verify(&self, instance: &mut Instance) -> Result<u128>;
            /// gegerate by Hash.
            fn id_generate_if_not_set(&self, instance: &mut Instance) -> Result<u128>;
        }
    }

    #[test]
    fn test_input() {
        let scenario = Scenario::new();
        let mut mock_instance_dao = scenario.create_mock::<InstanceDaoTraitMock>();
        let mut mock_route_svc = scenario.create_mock_for::<RouteServiceTrait>();
        let mut mock_task_svc = scenario.create_mock_for::<TaskServiceTrait>();
        let mut mock_task_dao = scenario.create_mock::<TaskDaoTraitMock>();
        let mut mock_instance_svc = scenario.create_mock::<InstanceServiceTraitMock>();

        let store_svc = StoreServiceImpl {
            instance_dao: Rc::new(mock_instance_dao),
            route: Rc::new(mock_route_svc),
            task_svc: Rc::new(mock_task_svc),
            task_dao: Rc::new(mock_task_dao),
            svc_instance: Rc::new(mock_instance_svc),
        };
//        scenario.expect(cond.make_hotter_call(4).and_return(()));
//
        let instance = Instance::default();
        scenario.expect(cond.get_temperature_call().and_return(16));
        let rrn = store_svc.input(instance);
    }
}