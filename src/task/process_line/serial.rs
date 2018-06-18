use chrono::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::ops::Deref;
use super::*;

pub struct Queue<T> {
    phantom: PhantomData<T>
}

impl<T: DeliveryTrait> Queue<T> {
    pub fn submit_serial(batch: SerialBatchInstance) -> Result<()> {
        match T::create_carrier(batch, "".to_string(), DataType::QueueBatch as u8) {
            Ok(carrier) => {
                // to process asynchronous
                send_carrier(CHANNEL_SERIAL.sender.lock().unwrap().clone(), carrier);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn do_serial(carrier: Carrier<SerialBatchInstance>) {
        let sf = Self::store_batch_items(DATA_INSTANCE.clone().deref(), &carrier);
        if sf.is_err() {
            // retry if environment error occurs,
            // item error will not break the process and insert into error list of `SerialFinished`
            return;
        }

        let instance = match Self::new_virtual_instance(&carrier, sf.unwrap()) {
            Err(err) => {
                T::move_to_err(err, carrier);
                return;
            }
            Ok(ins) => ins,
        };

        let si = StoreInfo { instance, converter: None };
        if let Ok(route) = T::create_and_finish_carrier(si, carrier, si.instance.data.thing.key.clone(), DataType::QueueBatch as u8) {
            send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), route);
        }
    }

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
        where
            F: InstanceTrait
    {
        let mut errors: Vec<String> = Vec::new();
        let mut succeeded_id: Vec<u128> = Vec::new();
        for mut instance in carrier.content.data.instances.clone() {
            if let Err(err) = F::verify(&mut instance, Root::Business) {
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
