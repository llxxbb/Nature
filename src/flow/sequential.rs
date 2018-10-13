use chrono::prelude::*;
use flow::store::StoreServiceTrait;
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use super::*;
use system::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialFinished {
    pub succeeded_id: Vec<u128>,
    pub errors: Vec<String>,
}

pub trait SequentialTrait {
    fn one_by_one(&self, batch: &SerialBatchInstance) -> Result<()>;
    fn do_serial_task(&self, task: SerialBatchInstance, carrier: &RawDelivery);
}

pub struct SequentialServiceImpl {
    pub svc_delivery: Rc<DeliveryServiceTrait>,
    pub dao_delivery: Rc<DeliveryDaoTrait>,
    pub store: Rc<StoreServiceTrait>,
    pub svc_instance: Rc<InstanceServiceTrait>,
    pub dao_instance: Rc<InstanceDaoTrait>,
}

impl SequentialTrait for SequentialServiceImpl {
    fn one_by_one(&self, batch: &SerialBatchInstance) -> Result<()> {
        let raw = RawDelivery::new(batch, &batch.thing.key, DataType::QueueBatch as i16)?;
        match self.dao_delivery.insert(&raw) {
            Ok(_carrier) => {
                // to process asynchronous
                let _ = CHANNEL_SERIAL.sender.lock().unwrap().send((batch.to_owned(), raw));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_serial_task(&self, task: SerialBatchInstance, carrier: &RawDelivery) {
        let finish = &task.context_for_finish.clone();
        if let (Ok(si)) = self.store_batch_items(task) {
            match Self::new_virtual_instance(finish, si) {
                Ok(instance) => {
                    if let (Ok(si)) = self.store.generate_store_task(&instance) {
                        match RawDelivery::new(&si, &instance.thing.key, DataType::QueueBatch as i16) {
                            Ok(new) => {
                                let mut new = new;
                                if let Ok(_route) = self.svc_delivery.create_and_finish_carrier(carrier, &mut new) {
                                    let _ = CHANNEL_STORED.sender.lock().unwrap().send((si, new));
                                }
                            }
                            Err(err) => {
                                let _ = self.dao_delivery.raw_to_error(&err, &carrier);
                            }
                        }
                    }
                }
                Err(err) => {
                    let _ = self.dao_delivery.raw_to_error(&err, &carrier);
                }
            };
        }
        // auto retry if environment error occurs,
        // item error will not break the process and insert into error list of `SerialFinished`
    }
}

impl SequentialServiceImpl {
    fn new_virtual_instance(context_for_finish: &str, sf: SerialFinished) -> Result<Instance> {
        let json = serde_json::to_string(&sf)?;
        let mut context: HashMap<String, String> = HashMap::new();
        context.insert(context_for_finish.to_string(), json);
        let time = Local::now().timestamp();
        Ok(Instance {
            id: 0,
            data: InstanceNoID {
                thing: Thing {
                    key: SYS_KEY_SERIAL.clone(),
                    version: 1,
                    thing_type: ThingType::System,
                },
                event_time: time,
                execute_time: time,
                create_time: time,
                content: String::new(),
                context,
                status: HashSet::new(),
                status_version: 0,
                from: None,
            },
        })
    }

    fn store_batch_items(&self, task: SerialBatchInstance) -> Result<SerialFinished>
    {
        let mut errors: Vec<String> = Vec::new();
        let mut succeeded_id: Vec<u128> = Vec::new();
        for mut instance in task.instances {
            instance.data.thing.thing_type = ThingType::Business;
            instance.data.thing = task.thing.clone();
            if let Err(err) = self.svc_instance.verify(&mut instance) {
                errors.push(format!("{:?}", err));
                continue;
            }
            match self.dao_instance.insert(&instance) {
                Ok(_) => succeeded_id.push(instance.id),
                Err(err) => match err {
                    NatureError::DaoEnvironmentError(_) => return Err(err),
                    NatureError::DaoDuplicated(_) => succeeded_id.push(instance.id),
                    _ => {
                        errors.push(format!("{:?}", err));
                        continue;
                    }
                }
            }
        }
        Ok(SerialFinished { succeeded_id, errors })
    }
}
