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
    fn store(&self, task: &StoreTaskInfo, carrier: &RawDelivery);
    fn generate_store_task(&self, instance: &Instance) -> Result<StoreTaskInfo>;
}

pub struct StoreServiceImpl {
    instance_dao: Rc<InstanceDaoTrait>,
    route: Rc<RouteServiceTrait>,
    delivery_svc: Rc<DeliveryServiceTrait>,
    delivery_dao: Rc<DeliveryDaoTrait>,
    svc_instance: Rc<InstanceServiceTrait>,
}

impl StoreServiceTrait for StoreServiceImpl {
    fn input(&self, mut instance: Instance) -> Result<u128> {
        instance.data.thing.thing_type = ThingType::Business;
        let uuid = self.svc_instance.verify(&mut instance)?;
        let task = self.generate_store_task(&instance)?;
        let carrier = RawDelivery::new(&task, &instance.thing.key, DataType::Store as i16)?;
        self.do_task(&task, &carrier)?;
        Ok(uuid)
    }

    fn store(&self, task: &StoreTaskInfo, carrier: &RawDelivery) {
        let _ = self.do_task(task, carrier);
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

    fn do_task(&self, task: &StoreTaskInfo, carrier: &RawDelivery) -> Result<()> {
        debug!("------------------do_store_task------------------------");
        if let Err(err) = self.save(&task.instance) {
            self.delivery_dao.raw_to_error(&err, carrier);
            Err(err)
        } else {
            CHANNEL_STORED.sender.lock().unwrap().send((task.to_owned(), carrier.to_owned()));
            Ok(())
        }
    }
}