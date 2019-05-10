use std::convert::TryFrom;

use serde::Deserialize;

use super::*;

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub fn input(mut instance: Instance) -> Result<u128> {
        instance.mut_biz(ThingType::Business);
        let _ = instance.thing.check(|x| ThingDefineCacheImpl.get(x));
        let _ = instance.fix_id()?;
        let task = IncomeController::gen_store_task(&instance)?;
        let carrier = RawTask::new(&task, &instance.thing.get_full_key(), TaskType::Store as i16)?;
        TaskDaoImpl::insert(&carrier)?;
        // do_task -> make a reusable method
        if let Err(err) = InstanceDaoImpl::save(&task.instance) {
            let _ = TaskDaoImpl::raw_to_error(&err, &carrier);
            return Err(err);
        } else {
            let _ = CHANNEL_STORED.sender.lock().unwrap().send((task.to_owned(), carrier.to_owned()));
        }
        Ok(instance.id)
    }

    pub fn gen_store_task(instance: &Instance) -> Result<StoreTaskInfo> {
        let steps = match OneStepFlowCacheImpl::get(&instance.thing)? {
            Some(steps) => {
                Mission::filter_relations(&instance, steps)
            }
            None => None
        };
        let task = StoreTaskInfo::make_task(&instance, steps);
        Ok(task)
    }

    /// born an instance which is the beginning of the changes.
    pub fn self_route(instance: SelfRouteInstance) -> Result<u128> {
        SVC_NATURE.store_svc.self_route(instance)
    }

    pub fn callback(delayed: DelayedInstances) -> Result<()> {
        SVC_NATURE.converter_svc.callback(delayed)
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
