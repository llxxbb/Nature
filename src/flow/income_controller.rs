use std::convert::TryFrom;

use serde::Deserialize;

use super::*;

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub fn input(mut instance: Instance) -> Result<u128> {
        instance.mut_biz(ThingType::Business);
        let _ = instance.check_and_fix_id(ThingDefineCacheImpl::get);
        let task = StoreTaskInfo::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations)?;
        let carrier = RawTask::save(&task, &instance.thing.get_full_key(), TaskType::Store as i16, TaskDaoImpl::insert)?;
        let _ = instance.save(InstanceDaoImpl::save)?;
        let _ = task.send(&carrier, &CHANNEL_STORED.sender.lock().unwrap());
        Ok(instance.id)
    }

    /// born an instance which is the beginning of the changes.
    pub fn self_route(instance: SelfRouteInstance) -> Result<u128> {
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
        let task = StoreTaskInfo::for_dynamic(&ins, instance.converter)?;
        // TODO save raw task
        let carrier = RawTask::new(&task, &ins.thing.get_full_key(), TaskType::Store as i16)?;
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
        SVC_NATURE.batch_serial_svc.one_by_one(&batch)
    }

    pub fn parallel(batch: ParallelBatchInstance) -> Result<()> {
        SVC_NATURE.batch_parallel_svc.parallel(batch)
    }

    fn send_to_channel<'a, T: Deserialize<'a>>(raw: &'a RawTask, channel: &Channel<(T, RawTask)>) -> Result<()> {
        let task: T = serde_json::from_str(&raw.data)?;
        let _ = channel.sender.lock().unwrap().send((task, raw.clone()));
        Ok(())
    }
}
