use std::rc::Rc;

use nature_db::task_type::TaskType;

use super::*;

pub trait StoreServiceTrait {
    fn self_route(&self, instance: SelfRouteInstance) -> Result<u128>;
    fn do_task(&self, task: &StoreTaskInfo, carrier: &RawTask) -> Result<()>;
}

pub struct StoreServiceImpl {
    pub route: Rc<RouteServiceTrait>,
    pub task_svc: Rc<TaskServiceTrait>,
}

impl StoreServiceTrait for StoreServiceImpl {
    fn self_route(&self, instance: SelfRouteInstance) -> Result<u128> {
        if instance.converter.is_empty() {
            return Err(NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
        }
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = Instance {
            id: 0,
            data: instance.instance.data,
        };
        ins.data.thing.set_thing_type(ThingType::Dynamic);
        let uuid = ins.fix_id()?.id;
        let task = self.generate_self_route_task(&ins, instance.converter)?;
        let carrier = RawTask::new(&task, &ins.thing.get_full_key(), TaskType::Store as i16)?;
        self.do_task(&task, &carrier)?;
        Ok(uuid)
    }


    fn do_task(&self, task: &StoreTaskInfo, carrier: &RawTask) -> Result<()> {
        debug!("------------------do_store_task------------------------");
        if let Err(err) = self.save(&task.instance) {
            let _ = TaskDaoImpl::raw_to_error(&err, carrier);
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
        debug!("save instance for `Thing` {:?}, id : {:?}", instance.thing.get_full_key(), instance.id);
        let result = InstanceDaoImpl::insert(instance);
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
