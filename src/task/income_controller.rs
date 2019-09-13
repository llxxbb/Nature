use std::convert::TryFrom;

use nature_common::{Instance, MetaType, NatureError, Result, SelfRouteInstance, TaskForParallel, TaskForSerial};
use nature_db::{CallbackResult, DelayedInstances, MetaCacheImpl, Mission, OneStepFlowCacheImpl, RawTask, TaskDaoImpl, TaskType};

use crate::actor::*;
use crate::task::{InnerController, TaskForConvert, TaskForStore};

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub fn input(mut instance: Instance) -> Result<u128> {
        instance.change_meta_type(MetaType::Business);
        let _ = instance.check_and_fix_id(MetaCacheImpl::get)?;
        let task = TaskForStore::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations)?;
        let carrier = RawTask::save(&task, &instance.meta.get_full_key(), TaskType::Store as i16, TaskDaoImpl::insert)?;
        InnerController::save_instance(task, carrier)?;
        Ok(instance.id)
    }

    /// born an instance which is the beginning of the changes.
    pub fn self_route(instance: SelfRouteInstance) -> Result<u128> {
        let _ = instance.verify()?;
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = instance.to_instance();
        ins.change_meta_type(MetaType::Dynamic);
        let uuid = ins.fix_id()?.id;
        let task = TaskForStore::for_dynamic(&ins, instance.converter)?;
        let carrier = RawTask::save(&task, &ins.meta.get_full_key(), TaskType::Store as i16, TaskDaoImpl::insert)?;
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
                        CallbackResult::Instances(ins) => {
                            let task: TaskForConvert = serde_json::from_str(&carrier.data)?;
                            InnerController::received_instance(&task, &carrier, ins)
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
            TaskType::Store => {
                let rtn = serde_json::from_str(&raw.data)?;
                ACT_STORED.do_send(MsgForTask(rtn, raw));
            }
            TaskType::Convert => {
                let rtn = serde_json::from_str(&raw.data)?;
                ACT_CONVERT.do_send(MsgForTask(rtn, raw));
            }
            TaskType::ParallelBatch => {
                let rtn = serde_json::from_str(&raw.data)?;
                ACT_PARALLEL.do_send(MsgForTask(rtn, raw));
            }
            TaskType::QueueBatch => {
                let rtn = serde_json::from_str(&raw.data)?;
                ACT_SERIAL.do_send(MsgForTask(rtn, raw));
            }
        }
        Ok(())
    }

    pub fn serial(batch: TaskForSerial) -> Result<()> {
        let raw = RawTask::save(&batch, &batch.meta.get_full_key(), TaskType::QueueBatch as i16, TaskDaoImpl::insert)?;
        let _ = ACT_SERIAL.try_send(MsgForTask(batch.to_owned(), raw));
        Ok(())
    }

    pub fn parallel(batch: TaskForParallel) -> Result<()> {
        let raw = RawTask::save(&batch, &batch.meta.get_full_key(), TaskType::ParallelBatch as i16, TaskDaoImpl::insert)?;
        let _ = ACT_PARALLEL.try_send(MsgForTask(batch, raw));
        Ok(())
    }
}
