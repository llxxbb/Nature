use std::convert::TryFrom;

use nature_common::{Instance, MetaType, NatureError, Result, SelfRouteInstance, TaskForSerial};
use nature_db::{CallbackResult, DelayedInstances, InstanceDaoImpl, MetaCacheImpl, MetaDaoImpl, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, TaskDaoImpl, TaskType};

use crate::actor::*;
use crate::task::{InnerController, TaskForConvert, TaskForStore};

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub fn input(mut instance: Instance) -> Result<u128> {
        let _ = instance.check_and_revise(MetaCacheImpl::get, MetaDaoImpl::get)?;
        let relations = RelationCacheImpl::get(&instance.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get)?;
        let mission = Mission::get_by_instance(&instance, &relations);
        let task = TaskForStore::new(instance.clone(), mission);
        let raw = RawTask::new(&task, &instance.meta, TaskType::Store as i16)?;
        TaskDaoImpl::insert(&raw)?;
        InnerController::save_instance(task, raw)?;
        Ok(instance.id)
    }

    /// born an instance which is the beginning of the changes.
    pub fn self_route(instance: SelfRouteInstance) -> Result<u128> {
        let _ = instance.verify()?;
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = instance.to_instance();
        MetaType::check_type(&ins.meta, MetaType::Dynamic)?;
        let uuid = ins.revise()?.id;
        let task = TaskForStore::for_dynamic(&ins, instance.converter)?;
        let raw = RawTask::new(&task, &ins.meta, TaskType::Store as i16)?;
        let _ = TaskDaoImpl::insert(&raw)?;
        InnerController::save_instance(task, raw)?;
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
                            let last = match task.target.to.is_state() {
                                true => task.from.get_last_taget(&task.target.to.meta_string(), InstanceDaoImpl::get_by_id)?,
                                false => None
                            };
                            InnerController::after_converted(&task, &carrier, ins, &last)
                        }
                    }
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn redo_task(raw: RawTask) -> Result<()> {
        // TODO check busy first
        dbg!(&raw);
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
        let _ = Instance::meta_must_same(&batch.instances)?;
        let raw = RawTask::new(&batch, &batch.instances[0].meta, TaskType::QueueBatch as i16)?;
        let _ = TaskDaoImpl::insert(&raw)?;
        let _ = ACT_SERIAL.try_send(MsgForTask(batch.to_owned(), raw));
        Ok(())
    }

    pub fn parallel(batch: Vec<Instance>) -> Result<()> {
        let _ = Instance::meta_must_same(&batch)?;
        let raw = RawTask::new(&batch, &batch[0].meta, TaskType::ParallelBatch as i16)?;
        let _ = TaskDaoImpl::insert(&raw)?;
        let _ = ACT_PARALLEL.try_send(MsgForTask(batch, raw));
        Ok(())
    }
}
