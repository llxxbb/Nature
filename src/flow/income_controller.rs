use std::convert::TryFrom;

use serde::Deserialize;

use super::*;

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub fn input(mut instance: Instance) -> Result<u128> {
        instance.change_thing_type(ThingType::Business);
        let _ = instance.check_and_fix_id(ThingDefineCacheImpl::get);
        let task = StoreTaskInfo::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations)?;
        let carrier = RawTask::save(&task, &instance.thing.get_full_key(), TaskType::Store as i16, TaskDaoImpl::insert)?;
        let _ = instance.save(InstanceDaoImpl::save)?;
        let _ = task.send(&carrier, &CHANNEL_STORED.sender.lock().unwrap());
        Ok(instance.id)
    }

    /// born an instance which is the beginning of the changes.
    pub fn self_route(instance: SelfRouteInstance) -> Result<u128> {
        let _ = instance.verify()?;
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = instance.to_instance();
        ins.change_thing_type(ThingType::Dynamic);
        let uuid = ins.fix_id()?.id;
        let task = StoreTaskInfo::for_dynamic(&ins, instance.converter)?;
        let carrier = RawTask::save(&task, &ins.thing.get_full_key(), TaskType::Store as i16, TaskDaoImpl::insert)?;
        InnerController::save_instance(task, carrier)?;
        Ok(uuid)
    }

    pub fn callback(delayed: DelayedInstances) -> Result<()> {
        match TaskDaoImpl::get(&delayed.carrier_id) {
            Ok(raw) => {
                match raw {
                    None => Err(NatureError::VerifyError("task data missed, maybe it had done already.".to_string())),
                    Some(carrier) => match delayed.result {
                        CallbackResult::Err(err) => {
                            let err = NatureError::ConverterLogicalError(err);
                            let _ = TaskDaoImpl::raw_to_error(&err, &carrier);
                            Ok(())
                        }
                        CallbackResult::Instances(mut ins) => {
                            let task: ConverterInfo = serde_json::from_str(&carrier.data)?;
                            InnerController::received_instance(&task, &carrier, &mut ins)
                        }
                    }
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn redo_task(raw: RawTask) -> Result<()> {
        // TODO check busy first
        match TaskType::try_from(raw.data_type)? {
            TaskType::Store => Self::send_to_channel::<StoreTaskInfo>(&raw, &CHANNEL_STORED)?,
            TaskType::Convert => Self::send_to_channel::<ConverterInfo>(&raw, &CHANNEL_CONVERT)?,
            TaskType::ParallelBatch => Self::send_to_channel::<ParallelBatchInstance>(&raw, &CHANNEL_PARALLEL)?,
            TaskType::QueueBatch => Self::send_to_channel::<SerialBatchInstance>(&raw, &CHANNEL_SERIAL)?,
        }
        Ok(())
    }

    pub fn serial(batch: SerialBatchInstance) -> Result<()> {
        let raw = RawTask::save(&batch, &batch.thing.get_full_key(), TaskType::QueueBatch as i16, TaskDaoImpl::insert)?;
        let _ = CHANNEL_SERIAL.sender.lock().unwrap().send((batch.to_owned(), raw));
        Ok(())
    }

    pub fn parallel(batch: ParallelBatchInstance) -> Result<()> {
        let raw = RawTask::save(&batch, &batch.thing.get_full_key(), TaskType::ParallelBatch as i16, TaskDaoImpl::insert)?;
        let _ = CHANNEL_PARALLEL.sender.lock().unwrap().send((batch, raw));
        Ok(())
    }

    fn send_to_channel<'a, T: Deserialize<'a>>(raw: &'a RawTask, channel: &Channel<(T, RawTask)>) -> Result<()> {
        let task: T = serde_json::from_str(&raw.data)?;
        let _ = channel.sender.lock().unwrap().send((task, raw.clone()));
        Ok(())
    }
}
