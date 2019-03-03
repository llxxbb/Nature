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
        self.do_task(&task, &carrier)?;
        Ok(uuid)
    }

    fn self_route(&self, instance: SelfRouteInstance) -> Result<u128> {
        if instance.converter.is_empty() {
            return Err(NatureError::VerifyError("converter must not empty!".to_string()));
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