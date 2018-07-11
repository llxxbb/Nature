use chrono::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::ops::Deref;
use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialFinished {
    pub succeeded_id: Vec<u128>,
    pub errors: Vec<String>,
}

pub trait SequentialTrait {
    fn submit_serial(batch: SerialBatchInstance) -> Result<()>;
    fn do_serial_task(carrier: Carrier<SerialBatchInstance>);
}

pub struct SequentialServiceImpl<SD, SS> {
    delivery: PhantomData<SD>,
    store: PhantomData<SS>,
}

impl<SD, SS> SequentialTrait for SequentialServiceImpl<SD, SS>
    where SD: DeliveryServiceTrait, SS: StoreServiceTrait
{
    fn submit_serial(batch: SerialBatchInstance) -> Result<()> {
        match SD::create_carrier(batch, "".to_string(), DataType::QueueBatch as u8) {
            Ok(carrier) => {
                // to process asynchronous
                SD::send_carrier(&CHANNEL_SERIAL.sender, carrier);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_serial_task(carrier: Carrier<SerialBatchInstance>) {
        let sf = Self::store_batch_items(DATA_INSTANCE.clone().deref(), &carrier);
        if sf.is_err() {
            // retry if environment error occurs,
            // item error will not break the process and insert into error list of `SerialFinished`
            return;
        }

        let instance = match Self::new_virtual_instance(&carrier, sf.unwrap()) {
            Err(err) => {
                SD::move_to_err(err, carrier);
                return;
            }
            Ok(ins) => ins,
        };

        let si = SS::generate_store_task(instance);
        if si.is_err() {
            return;
        }
        let si = si.unwrap();
        let biz = si.instance.data.thing.key.clone();
        if let Ok(route) = SD::create_and_finish_carrier(si, carrier, biz, DataType::QueueBatch as u8) {
            SD::send_carrier(&CHANNEL_DISPATCH.sender, route);
        }
    }
}

impl<SD, SS> SequentialServiceImpl<SD, SS>
    where SD: DeliveryServiceTrait, SS: StoreServiceTrait
{
    fn new_virtual_instance(carrier: &Carrier<SerialBatchInstance>, sf: SerialFinished) -> Result<Instance> {
        let json = serde_json::to_string(&sf)?;
        let mut context: HashMap<String, String> = HashMap::new();
        context.insert(carrier.content.data.context_for_finish.clone(), json);
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

    fn store_batch_items<F>(_: &F, carrier: &Carrier<SerialBatchInstance>) -> Result<SerialFinished>
        where F: InstanceServiceTrait
    {
        let mut errors: Vec<String> = Vec::new();
        let mut succeeded_id: Vec<u128> = Vec::new();
        for mut instance in carrier.content.data.instances.clone() {
            instance.data.thing.thing_type = ThingType::Business;
            if let Err(err) = F::verify(&mut instance) {
                errors.push(format!("{:?}", err));
                continue;
            }
            match TableInstance::insert(&instance) {
                Ok(_) => succeeded_id.push(instance.id),
                Err(NatureError::DaoEnvironmentError(err)) => return Err(NatureError::DaoEnvironmentError(err)),
                Err(NatureError::DaoDuplicated) => succeeded_id.push(instance.id),
                Err(err) => {
                    errors.push(format!("{:?}", err));
                    continue;
                }
            }
        }
        Ok(SerialFinished { succeeded_id, errors })
    }
}

pub type SequentialTask = SequentialServiceImpl<DeliveryService, StoreService>;
