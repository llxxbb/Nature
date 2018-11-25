use std::convert::TryFrom;

use super::*;
use serde::Deserialize;

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub fn input(instance: Instance) -> Result<u128> {
        SVC_NATURE.store_svc.input(instance)
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
