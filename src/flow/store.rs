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
    fn generate_store_task(&self, instance: &Instance) -> Result<StoreTaskInfo>;
    fn do_task(&self, task: &StoreTaskInfo, carrier: &RawTask) -> Result<()>;
}

pub struct StoreServiceImpl {
    pub instance_dao: Rc<InstanceDaoTrait>,
    pub route: Rc<RouteServiceTrait>,
    pub delivery_svc: Rc<DeliveryServiceTrait>,
    pub delivery_dao: Rc<DeliveryDaoTrait>,
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


    /// generate `StoreTaskInfo` include route information.
    /// `Err` on environment error
    fn generate_store_task(&self, instance: &Instance) -> Result<StoreTaskInfo> {
//        let key = &instance.thing.key;
        let target = self.route.get_route(instance)?;
        // save to delivery to make it can redo
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
            let _ = self.delivery_dao.raw_to_error(&err, carrier);
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
        debug!("save instance for `Thing` {:?}", instance.thing.key);
        let result = self.instance_dao.insert(instance);
        match result {
            Ok(num) => Ok(num),
            Err(err) => match err {
                NatureError::DaoDuplicated(_) => Ok(0),
                _ => Err(err)
            }
        }
    }
}